#!/usr/bin/env python3
"""[rustfast-g] builds staged/rustfast (batch.json anchored edits, new.json + new/ flat files) from the LIVE tree and src/. usage: python -I build.py"""
import json, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def find_root(path):
    while path != os.path.dirname(path):
        if os.path.exists(path + "/AGENTS.md"): return path
        path = os.path.dirname(path)
ROOT = find_root(HERE) + "/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
P = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/"
U = L + "⚡️caching/🦀️cargo/🗃️unit-cache/"
T = L + "🏃️process/🦀️cargo/⏱️timing/"
def live(rel): return open(ROOT + rel, encoding="utf8").read()
def between(text, start, end):
    i = text.index(start); j = text.index(end, i); return text[i:j]
files = []
def edit(rel, *pairs):
    text = live(rel); edits = []
    for old, new in pairs:
        assert text.count(old) == 1, (rel[-40:], old[:70], text.count(old))
        edits.append({"old": old, "new": new})
    files.append({"file": rel, "edits": edits})

# 1. units: the planning digest is a function of the unit's identity, not of Cargo's numbering
edit(P + "🕸️units/🟦️.ts", ("planningDigest:digest(unit),",
 "planningDigest:digest({...unit,dependencies:unit.dependencies.map((value:unknown)=>{const{index,...facts}=object(value);return{...facts,unit:ids[index]};}).sort((left:any,right:any)=>left.unit.localeCompare(right.unit)||left.extern_crate_name.localeCompare(right.extern_crate_name))}),"))

# 2. capture: foreign crate inventory through a stamp memo, progress every 64 files
cap = P + "🕸️units/📥️capture/🟦️.ts"
text = live(cap)
old_source = between(text, "async sourceInputs(manifest,control){", "async compiler(environment,cwd,control){")
new_source = "async sourceInputs(manifest,control){let files=0;return foreignSourceInventoryV1(dirname(manifest),options.digests,{check:()=>checkResourceLeaseControl(control),tick:async()=>{if(++files%64===0){await control.publish({resource:\"cargo:foreign-source\",mode:\"shared\",elapsedMs:0});await control.yieldContinuation();}}});},"
new_defs = '''/** 🗂️ The no-follow stamp of one file: device, inode, size, mtime and ctime. */
export type FileStampV1=Readonly<{dev:string;ino:string;size:string;mtimeNs:string;ctimeNs:string}>;
/** 📚️ Remembers file digests across invocations by stamp; the memo alone decides which stamps are too young to trust. @see ../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/📚️foreign/🟦️.ts */
export interface FileDigestMemoV1 {open(directory:string,startedMs:number):Readonly<{recall(path:string,stamp:FileStampV1):string|undefined;remember(path:string,stamp:FileStampV1,digest:string):void;close():void}>}
const stampOfV1=(row:import("node:fs").BigIntStats):FileStampV1=>Object.freeze({dev:String(row.dev),ino:String(row.ino),size:String(row.size),mtimeNs:String(row.mtimeNs),ctimeNs:String(row.ctimeNs)});
/** 🧾️ Inventories every file of one foreign crate directory with its sha-256 and size: a file whose stamp the memo knows is not read, a file that changed while it was hashed is not remembered, a symlink refuses the walk. */
export async function foreignSourceInventoryV1(root:string,memo:FileDigestMemoV1|undefined,hooks:Readonly<{check():void;tick():Promise<void>}>,now:()=>number=Date.now):Promise<{path:string;digest:string;bytes:number}[]>{
 const book=memo?.open(root,now()),result:{path:string;digest:string;bytes:number}[]=[];
 try{
  const walk=async(directory:string):Promise<void>=>{hooks.check();for(const row of await readdir(requireMaterializedPathV1(directory),{withFileTypes:true})){
   if([".git","target","node_modules","dist","🗑️generated"].includes(row.name))continue;
   const path=requireMaterializedPathV1(join(directory,row.name));
   if(row.isSymbolicLink())throw Error("Foreign Cargo Source symlink cannot qualify immutable input");
   if(row.isDirectory()){await walk(path);continue;}
   if(!row.isFile())continue;
   const before=stampOfV1(await lstat(path,{bigint:true}));let digest=book?.recall(path,before),bytes=Number(before.size);
   if(digest===undefined){const file=await open(path,"r"),hash=createHash("sha256"),buffer=Buffer.alloc(65536);bytes=0;try{for(;;){hooks.check();const part=await file.read(buffer,0,buffer.length,null);if(!part.bytesRead)break;hash.update(buffer.subarray(0,part.bytesRead));bytes+=part.bytesRead;}}finally{await file.close();}digest=hash.digest("hex");const after=stampOfV1(await lstat(path,{bigint:true}));if(after.ino===before.ino&&after.size===before.size&&after.mtimeNs===before.mtimeNs&&after.ctimeNs===before.ctimeNs)book?.remember(path,before,digest);}
   result.push({path,digest,bytes});if(result.length>65536)throw Error("Foreign Cargo Source input inventory limit exceeded");await hooks.tick();
  }};
  await walk(root);
 }finally{book?.close();}
 return result.sort((left,right)=>left.path.localeCompare(right.path));
}

'''
anchor = "/** ⏺️ Plans the exact selected Cargo invocation without dispatching its compiler or test bodies. */"
edit(cap,
 ("import {readdir,open} from \"node:fs/promises\";", "import {readdir,open,lstat} from \"node:fs/promises\";"),
 ("export function actualCargoCompilerPlanningPortV1():CargoCompilerPlanningPortV1{return{", "export function actualCargoCompilerPlanningPortV1(options:Readonly<{digests?:FileDigestMemoV1}>={}):CargoCompilerPlanningPortV1{return{"),
 (old_source, new_source),
 (anchor, new_defs + anchor))

# 3. admission memo: a key over the facts that decide the admission, remembered under the key it was recalled with
mem = U + "📖️admission/🟦️.ts"
text = live(mem)
old_key = between(text, "/** 🔑️ Identity of one admission", "\n/** 📸️ Stamps a path")
new_key = '''/** 🧷️ The unit facts that decide an admission result. Run-specific facts (unit ids, dependency edges, planning and compilation digests, the configuration digest and the invoking process's paths) never enter the key, so two plans of one workspace share it. */
export const ADMISSION_KEYED_FIELDS_V1:readonly string[]=Object.freeze(["packageId","packageName","packageVersion","edition","manifest","sourceRoot","provenance","entry","name","kind","operation","platform","roles","profileName","compilerProfile","profileDigest","compilerFlags","compilerArguments","features","boundary","support","compiler"]);
/** 🌬️ The names a unit's compiler environment copies from the invoking process as paths; they are left out of the key and replaced by the current values when an admission is recalled. */
export const ADMISSION_VOLATILE_ENVIRONMENT_V1:readonly string[]=Object.freeze(["PATH","SystemRoot","WINDIR","PATHEXT","RUSTUP_HOME"]);
type EnvironmentV1=Readonly<Record<string,string|null>>|null;
/** 🔑️ Identity of one admission: the keyed unit facts, the unit's compiler environment without its volatile names, the policy and the toolchain. */
export function admissionMemoKeyV1(unit:AdmissionUnitV1,policy:unknown,toolchain:string):string{
 const row=unit as Record<string,unknown>,environment=(row.compilerEnvironment??null) as EnvironmentV1;
 const rest={...Object.fromEntries(ADMISSION_KEYED_FIELDS_V1.map(name=>[name,row[name]??null])),environment:environment&&Object.fromEntries(Object.entries(environment).filter(([name])=>!ADMISSION_VOLATILE_ENVIRONMENT_V1.includes(name)))};
 return sha256V1(JSON.stringify(canonical([UNIT_CACHE_POLICY_V1.namespace,toolchain,policy,rest])));
}
/** 🌿️ A remembered compiler environment with the current values of the volatile names. */
export function admissionEnvironmentV1(remembered:EnvironmentV1,unit:AdmissionUnitV1):EnvironmentV1{
 const now=(unit as {compilerEnvironment?:EnvironmentV1}).compilerEnvironment;if(remembered===null||!now)return remembered;
 const result:Record<string,string|null>={...remembered};for(const name of ADMISSION_VOLATILE_ENVIRONMENT_V1)if(name in now)result[name]=now[name]!;else delete result[name];return result;
}
'''
old_port = text[text.index("/** 📝️ The memo port the source admission calls once per unit. */"):]
new_port = '''/** 📝️ The memo port the source admission calls once per unit: a unit is remembered under the key it was recalled with, because its admission rewrites the very fields the key reads. */
export function unitAdmissionMemoV1(root:string,toolchain:string){
 const keys=new WeakMap<object,string>();
 return Object.freeze({
  recall(unit:AdmissionUnitV1,policy:unknown):Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"]}>|undefined{const key=admissionMemoKeyV1(unit,policy,toolchain);keys.set(unit,key);const memo=recallAdmissionV1(root,key);return memo&&{inputs:memo.inputs,cfg:memo.cfg,environment:admissionEnvironmentV1(memo.environment,unit)};},
  remember(unit:AdmissionUnitV1,policy:unknown,result:Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"];probed:readonly string[];startedMs:number}>):void{rememberAdmissionV1(root,keys.get(unit)??admissionMemoKeyV1(unit,policy,toolchain),result);},
 });
}
'''
edit(mem, (old_key, new_key), (old_port, new_port))

# 4. unit-cache schema: the foreign digest books
edit(U + "🧬️schema/🔣️.json", ('"stamps": "m", "admissions": "a",', '"stamps": "m", "admissions": "a", "foreign": "f",'))

# 5. key parts and the miss reason
edit(U + "🔏️key/🟦️.ts", ("function unescapeDepPath(text:string):string{",
 '''/** 🧩️ Names the parts of a first-level key by short digests (toolchain, arguments, source, externs, environment), so two compilations of one crate that disagree on the key are explained by the part that differs.
 * @see ./🧪️tests/🧩️parts/🟦️.ts */
export function unitKeyPartsV1(input:Parameters<typeof unitKeyV1>[0]):string{
 const part=(value:unknown)=>sha256V1(JSON.stringify(value)).slice(0,8);
 return["toolchain:"+part(input.toolchain),"args:"+part(input.normalized),"source:"+part(input.sourceSha256),"externs:"+part(input.externs.map(row=>[row.name,row.sha256])),"environment:"+part(input.environment)].join(" ");
}

function unescapeDepPath(text:string):string{'''))
script = live(U + "📜️script.ts")
key_expr = between(script, "key=unitKeyV1({toolchain:", "});\n")
key_expr_full = key_expr + "});"
inner = key_expr[len("key=unitKeyV1("):]
edit(U + "📜️script.ts",
 ("toolchainIdentityV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from \"./🔏️key/🟦️.ts\";", "toolchainIdentityV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from \"./🔏️key/🟦️.ts\";"),
 (" let key:string;\n", " let key:string,parts=\"\";\n"),
 (key_expr_full, "const keyInput=" + inner + "};key=unitKeyV1(keyInput);parts=unitKeyPartsV1(keyInput);"),
 ('event("miss",key,Date.now()-compiled,waited);', 'event("miss",key,Date.now()-compiled,waited,parts);'))

# 6. timing ledger
edit(T + "🟦️.ts",
 ('import schema from"./🧬️schema/🔣️.json" with { type: "json" };', 'import {appendFileSync,mkdirSync,renameSync,statSync} from "node:fs";\nimport {dirname} from "node:path";\nimport schema from"./🧬️schema/🔣️.json" with { type: "json" };'),
 ("export const cargoTimingPolicyV1=Object.freeze({...schema.definitions.Policy.const});",
  '''export const cargoTimingPolicyV1=Object.freeze({...schema.definitions.Policy.const});
/** 📒️ Formats one phase mark of a Cargo launch as a ledger line: a closed phase and point, finite times.
 * @see ./🧪️tests/📒️ledger/🟦️.ts */
export function cargoTimingLedgerRowV1(pid:number,command:string,phase:string,point:string,atMs:number,elapsedMs:number):string{
 cargoTimingEventV1(phase,point,atMs,elapsedMs,null);if(!Number.isSafeInteger(pid)||pid<1)throw Error("Actual process id required");
 return JSON.stringify({version:1,atMs,pid,command:command.slice(0,64),phase,point,elapsedMs})+"\\n";
}
/** 📥️ Appends ledger lines; a ledger at or above `maximumBytes` first moves to `<file>.1` (one generation), and a ledger that cannot be written costs nothing. */
export function appendCargoTimingLedgerV1(file:string,rows:string,maximumBytes:number=cargoTimingPolicyV1.maximumLogBytes):void{
 if(!rows)return;
 try{mkdirSync(dirname(file),{recursive:true});try{if(statSync(file).size>=maximumBytes)renameSync(file,file+".1");}catch{}appendFileSync(file,rows);}catch{}
}'''))
sch = T + "🧬️schema/🔣️.json"
edit(sch, ('"phase":{"enum":["store","inputs","graph","source","writer","profile","capacity","scheduler","child"]}', '"phase":{"enum":["store","inputs","graph","source","writer","profile","capacity","scheduler","child","consume"]}'),
 ('"Phase":{"enum":["store","inputs","graph","source","writer","profile","capacity","scheduler","child"]}', '"Phase":{"enum":["store","inputs","graph","source","writer","profile","capacity","scheduler","child","consume"]}'))

# 7. launcher
lau = L + "🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
ltext = live(lau)
old_mark = between(ltext, "const mark=(phase:string,point:string)=>{", "const reportResource=")
new_mark = "const began=performance.now(),pending:string[]=[];let ledger:string|undefined;const record=(phase:string,point:string)=>{try{const row=cargoTimingLedgerRowV1(process.pid,label,phase,point,Date.now(),Math.floor(performance.now()-began));if(ledger)appendCargoTimingLedgerV1(ledger,row);else pending.push(row);}catch{}};" + old_mark.replace("const mark=(phase:string,point:string)=>{", "const mark=(phase:string,point:string)=>{record(phase,point);", 1)
edit(lau,
 ('import{cargoTimingPolicyV1,cargoTimingEventV1,cargoResourceWaitEventV1}from"../⏱️timing/🟦️.ts";', 'import{cargoTimingPolicyV1,cargoTimingEventV1,cargoResourceWaitEventV1,cargoTimingLedgerRowV1,appendCargoTimingLedgerV1}from"../⏱️timing/🟦️.ts";'),
 ('import {unitAdmissionMemoV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";', 'import {unitAdmissionMemoV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";\nimport {foreignDigestMemoV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/📚️foreign/🟦️.ts";'),
 (old_mark, new_mark),
 ('mark("store","ready");', 'ledger=join(store.root,"timing","cargo-phases.jsonl");try{requireNativeStoragePathV1(store.root,ledger);appendCargoTimingLedgerV1(ledger,pending.splice(0).join(""));}catch{ledger=undefined;}mark("store","ready");'),
 ('checkResourceLeaseControl(control);await store.storage.recheck();const success=execution.kind==="capture"', 'mark("consume","begin");checkResourceLeaseControl(control);await store.storage.recheck();const success=execution.kind==="capture"'),
 ('if(execution.kind==="stream")return execution.status;if(options.output!=="capture")', 'if(execution.kind==="stream"){mark("consume","end");return execution.status;}if(options.output!=="capture")'),
 ('checkScriptInvocation(invocation);return value;', 'checkScriptInvocation(invocation);mark("consume","end");return value;'),
 ('actualCargoCompilerPlanningPortV1())', 'actualCargoCompilerPlanningPortV1({digests:foreignDigestMemoV1(unitCacheRootV1(store.root))}))'))

edit(U + "🧪️tests/🟦️.ts", ('for(const changed of[{...unit,kind:"binary"},{...unit,id:"b"}])', 'for(const changed of[{...unit,kind:"binary"},{...unit,packageName:"b"}])'))
# new files
new = {
 "n01-foreign-memo.ts": U + "📖️admission/📚️foreign/🟦️.ts",
 "n04-foreign-test.ts": U + "📖️admission/📚️foreign/🧪️tests/🟦️.ts",
 "n04-foreign.json": U + "📖️admission/📚️foreign/🧪️tests/🔣️.json",
 "n04-foreign.py": U + "📖️admission/📚️foreign/🧪️tests/🐍️.py",
 "t02-memo-key.ts": U + "📖️admission/🧪️tests/🔑️key-stability/🟦️.ts",
 "t02-memo-key.json": U + "📖️admission/🧪️tests/🔑️key-stability/🔣️.json",
 "t02-memo-key.py": U + "📖️admission/🧪️tests/🔑️key-stability/🐍️.py",
 "n03-key-parts-test.ts": U + "🔏️key/🧪️tests/🧩️parts/🟦️.ts",
 "n03-key-parts.json": U + "🔏️key/🧪️tests/🧩️parts/🔣️.json",
 "n03-key-parts.py": U + "🔏️key/🧪️tests/🧩️parts/🐍️.py",
 "n05-plan-test.ts": P + "🕸️units/🧪️tests/🔁️plan-stability/🟦️.ts",
 "n05-plan.json": P + "🕸️units/🧪️tests/🔁️plan-stability/🔣️.json",
 "n05-plan.py": P + "🕸️units/🧪️tests/🔁️plan-stability/🐍️.py",
 "n06-ledger-test.ts": T + "🧪️tests/📒️ledger/🟦️.ts",
 "n06-ledger.json": T + "🧪️tests/📒️ledger/🔣️.json",
 "n06-ledger.py": T + "🧪️tests/📒️ledger/🐍️.py",
}
out = HERE + "/staged/rustfast"
shutil.rmtree(out + "/new", ignore_errors=True); os.makedirs(out + "/new")
names = {}
for index, (src, rel) in enumerate(sorted(new.items()), 1):
    flat = "f%02d" % index; shutil.copy(HERE + "/src/" + src, out + "/new/" + flat); names[flat] = rel
    assert len(rel) < 256, rel
json.dump({"files": files}, open(out + "/batch.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
json.dump(names, open(out + "/new.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("edits:", sum(len(f["edits"]) for f in files), "in", len(files), "files; new files:", len(names))
