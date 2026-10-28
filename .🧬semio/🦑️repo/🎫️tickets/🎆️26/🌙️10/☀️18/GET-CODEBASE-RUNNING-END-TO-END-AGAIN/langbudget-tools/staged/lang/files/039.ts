/** 🗓️ Contract: the scheduling policy is closed by its schema and refuses forged tables; every prefix is a real directory and every port a real file inside its own domain; a split keeps the entry and every output-producing file in the keyed part, moves exactly the files under the named prefixes into the overlay and leaves the keyed identity unchanged when only the overlay changes; the immutable loader merges the overlay beside a keyed snapshot, refuses every overlay it cannot prove (outside the declared domains, shadowing a keyed image, another root, hard-linked) and runs the edited scheduling code without the digest of the keyed snapshot ever moving. The cases live in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,linkSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {buildSourceClosureIdentityV1,buildSourceClosureLoaderV1,type BuildSourceClosureV1} from "../../🟦️.ts";
import {buildSchedulingDomainOfV1,buildSchedulingKeyedPathsV1,buildSchedulingExclusionPatternsV1,buildSchedulingExclusionsInputV1,buildSchedulingNativePrefixesV1,buildSchedulingOverlayNameV1,buildSchedulingPortsV1,buildSchedulingPrefixesV1,buildSchedulingV1,splitBuildSourceClosureV1,type BuildSchedulingV1} from "../🟦️.ts";
import declared from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";
import {closureRefusalCatalogPathV1} from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import sitesCatalog from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json";

const checkout=resolve(import.meta.dir,corpus.checkoutFromTests),sha=(value:string|Buffer)=>createHash("sha256").update(value).digest("hex");
const refusal=(table:unknown):unknown=>{try{buildSchedulingV1(table);}catch(error){return (error as {key?:string}).key;}return undefined;};
type Draft={version:1;domains:{id:string;contract:{en:string;de:string};prefixes:string[];nativeKeyed?:string[];ports:string[]}[];keyed:{path:string;reason:{en:string;de:string}}[]};
const copy=():Draft=>JSON.parse(JSON.stringify(declared));
const reason={en:"x",de:"y"},minimal=(prefixes:string[],keyed:string[]):Draft=>({version:1,domains:[{id:"only",contract:reason,prefixes,ports:[]}],keyed:keyed.map(path=>({path,reason}))});
const islandForgeries=():Record<string,()=>unknown>=>{
 const island=declared.keyed.find(row=>row.path.endsWith("/"))!.path,parent=island.slice(0,island.slice(0,-1).lastIndexOf("/")+1);
 return {
  "island-inside-prefix":()=>minimal([parent],[island]),
  "prefix-inside-island":()=>minimal([island+"inner/"],[island]),
  "island-duplicate":()=>minimal(["elsewhere/"],[island,island]),
  "island-parent-segment":()=>minimal(["elsewhere/"],["a/../b/"]),
  "keyed-missing":()=>{const table:Record<string,unknown>={...copy()};delete table.keyed;return table;}
 };
};

test("the corpus and the policy are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(schema,declared),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,{...declared,extra:1})).toBe(false);
 expect(ajv.validate(schema,{...declared,domains:[]})).toBe(false);
 for(const domain of declared.domains){expect(domain.contract.en.length,domain.id).toBeGreaterThan(0);expect(domain.contract.de.length,domain.id).toBeGreaterThan(0);}
});

test("every forged policy is refused and the declared one is admitted once",()=>{
 expect(buildSchedulingV1()).toBe(buildSchedulingV1());
 const forged:Record<string,()=>unknown>={
  "duplicate-domain-id":()=>{const table=copy();table.domains=[table.domains[0]!,{...table.domains[1]!,id:table.domains[0]!.id}];return table;},
  "nested-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first+"inner/"],ports:[]}];return table;},
  "duplicate-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first],ports:[]}];return table;},
  "parent-segment":()=>{const table=copy();table.domains=[{...table.domains[0]!,prefixes:["a/../b/"],ports:[]}];return table;},
  "port-outside-domain":()=>{const table=copy();table.domains=[{...table.domains[0]!,ports:["elsewhere/file.ts"]}];return table;},
  "port-is-directory":()=>{const table=copy();table.domains=[{...table.domains[0]!,ports:[table.domains[0]!.prefixes[0]!]}];return table;},
  "extra-key":()=>({...copy(),extra:true}),
  "no-domains":()=>({...copy(),domains:[]}),
  "native-keyed-outside-domain":()=>{const table=copy();table.domains=[{...table.domains[0]!,nativeKeyed:["elsewhere/"]}];return table;},
  "bad-id":()=>{const table=copy();table.domains=[{...table.domains[0]!,id:"Bad Id"}];return table;},
  ...Object.fromEntries(Object.entries(islandForgeries()).map(([name,make])=>[name,make]))
 };
 expect(Object.keys(forged).sort()).toEqual([...corpus.forgedPolicies].sort());
 for(const [name,make] of Object.entries(forged))expect(refusal(make()),name).toBe("build.scheduling.table-required");
});

test("every prefix is a real directory and every port a real file inside its own domain",()=>{
 for(const domain of buildSchedulingV1().domains){
  for(const prefix of domain.prefixes)expect(statSync(join(checkout,prefix)).isDirectory(),prefix).toBe(true);
  for(const port of domain.ports){expect(statSync(join(checkout,port)).isFile(),port).toBe(true);expect(buildSchedulingDomainOfV1(port),port).toBe(domain.id);}
 }
 expect(buildSchedulingPrefixesV1().length).toBe(new Set(buildSchedulingPrefixesV1()).size);
 expect(buildSchedulingPortsV1().size).toBeGreaterThan(0);
 expect(()=>buildSchedulingPrefixesV1(buildSchedulingV1(),["no-such-domain"])).toThrow();
 expect(buildSchedulingDomainOfV1("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts")).toBeUndefined();
});

const file=(path:string,text:string,imports:Record<string,string>={})=>({path,sha256:sha(text),base64:Buffer.from(text).toString("base64"),imports});

test("a split moves exactly the files under the named prefixes, keeps the entry keyed and leaves the identity to the keyed part",()=>{
 const [domain]=buildSchedulingV1().domains,prefix=domain!.prefixes[0]!;
 const snapshot:BuildSourceClosureV1={version:1,root:"/r",entry:"entry.ts",files:[file("entry.ts","a"),file("keyed.ts","b"),file(prefix+"x.ts","c"),file(prefix+"deep/y.json","{}")]};
 const {keyed,overlay}=splitBuildSourceClosureV1(snapshot,[domain!.id]);
 expect(keyed.files.map(row=>row.path)).toEqual(["entry.ts","keyed.ts"]);
 expect(overlay?.files.map(row=>row.path)).toEqual([prefix+"x.ts",prefix+"deep/y.json"]);
 expect(Object.keys(keyed)).toEqual(["version","root","entry","files"]);
 expect(Object.keys(overlay!)).toEqual(["version","root","entry","files"]);
 const edited={...snapshot,files:[...snapshot.files.slice(0,2),file(prefix+"x.ts","changed"),snapshot.files[3]!]};
 expect(buildSourceClosureIdentityV1(splitBuildSourceClosureV1(edited,[domain!.id]).keyed)).toBe(buildSourceClosureIdentityV1(keyed));
 const moved={...snapshot,files:[snapshot.files[0]!,file("keyed.ts","changed"),...snapshot.files.slice(2)]};
 expect(buildSourceClosureIdentityV1(splitBuildSourceClosureV1(moved,[domain!.id]).keyed)).not.toBe(buildSourceClosureIdentityV1(keyed));
 expect(splitBuildSourceClosureV1({...snapshot,files:snapshot.files.slice(0,2)},[domain!.id]).overlay).toBeUndefined();
 expect(splitBuildSourceClosureV1(snapshot,[]).overlay).toBeUndefined();
 expect(()=>splitBuildSourceClosureV1({...snapshot,entry:prefix+"x.ts"},[domain!.id])).toThrow();
 expect(buildSchedulingOverlayNameV1("implementation.executor-command.json")).toBe("implementation.executor-command.scheduling.json");
 expect(()=>buildSchedulingOverlayNameV1("implementation")).toThrow();
});

const run=(name:string)=>{
 const prefix=buildSchedulingPrefixesV1()[0]!,area=mkdtempSync(join(tmpdir(),"sch-")),root=join(tmpdir(),"sch-law-root");
 const keyed=[file("entry.ts",`import {k} from "./keyed.ts";import {s} from "./${prefix}probe.ts";export async function runCapturedBuildSourceV1(args:string[],proof:{sha256:string}){console.log(JSON.stringify([k,s,args,proof.sha256.length]));}`,{"./keyed.ts":"keyed.ts",[`./${prefix}probe.ts`]:prefix+"probe.ts"}),file("keyed.ts",'export const k="K1";'),file(closureRefusalCatalogPathV1,JSON.stringify(sitesCatalog))];
 const base={version:1,root,entry:"entry.ts",files:keyed},probe=(text:string)=>file(prefix+"probe.ts",text);
 const overlays:Record<string,object|undefined>={
  "merged":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";')]},
  "edited-overlay":{version:1,root,entry:"entry.ts",files:[probe('export const s="S2";')]},
  "missing-overlay":undefined,
  "outside-declared-domains":{version:1,root,entry:"entry.ts",files:[file("evil.ts",'export const s="E";')]},
  "shadows-keyed":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";'),file(prefix+"keyed.ts","x")]},
  "other-root":{version:1,root:root+"x",entry:"entry.ts",files:[probe('export const s="S1";')]},
  "writable-hardlink":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";')]}
 };
 if(name==="shadows-keyed")base.files=[...keyed,file(prefix+"keyed.ts","shadow")];
 const runner=join(area,"📜️script.ts");writeFileSync(runner,buildSourceClosureLoaderV1);
 const text=JSON.stringify(base),snapshot=join(area,"implementation.x.json");writeFileSync(snapshot,text);chmodSync(snapshot,0o400);
 const overlay=overlays[name];if(overlay){const target=join(area,buildSchedulingOverlayNameV1("implementation.x.json"));writeFileSync(target,JSON.stringify(overlay));chmodSync(target,0o400);if(name==="writable-hardlink")linkSync(target,join(area,"alias.json"));}
 const result=spawnSync("bun",[runner,snapshot,sha(text),"arg1"],{encoding:"utf8",cwd:area});rmSync(area,{recursive:true,force:true});
 return {status:result.status,stdout:result.stdout.trim(),stderr:result.stderr,digest:sha(text)};
};

for(const item of corpus.loaderCases)test(`the immutable loader ${item.outcome==="runs"?"merges":"refuses"} the scheduling overlay: ${item.case}`,()=>{
 const result=run(item.case);
 if(item.outcome==="runs"){expect(result.status,result.stderr).toBe(0);expect(result.stdout).toContain(item.case==="merged"?'"S1"':'"S2"');expect(result.stdout).toContain('"K1"');}
 else{expect(result.status).not.toBe(0);expect(result.stderr).toContain(item.message);}
},60000);

test("editing only the overlay never moves the digest of the keyed snapshot the action key is built from",()=>{
 const first=run("merged"),second=run("edited-overlay");
 expect(first.digest).toBe(second.digest);
 expect(first.stdout).not.toBe(second.stdout);
},60000);

test("the named input that removes scheduling code from declared inputs equals the policy in the build policy and in its Nx mirror",()=>{
 const expected=[...buildSchedulingExclusionPatternsV1()];
 for(const file of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","nx.json"]){
  const named=JSON.parse(readFileSync(join(checkout,file),"utf8")).namedInputs as Record<string,string[]>;
  expect(named[buildSchedulingExclusionsInputV1],file).toEqual(expected);
 }
 expect(expected.length).toBe(buildSchedulingPrefixesV1().length);
 for(const pattern of expected){expect(pattern.startsWith("!{workspaceRoot}/")).toBe(true);expect(pattern.endsWith("/**/*")).toBe(true);}
});

const oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");

test("the prefixes that leave the key of a Rust compile command equal the independent Python derivation and keep the cargo argument builders keyed",()=>{
 const run=spawnSync("python3",["-I",oracleFile],{input:JSON.stringify(declared),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const native=[...buildSchedulingNativePrefixesV1()];
 expect(native).toEqual(JSON.parse(run.stdout));
 for(const prefix of corpus.nativeFallback.leaves)expect(native.some(row=>prefix.startsWith(row)),prefix).toBe(true);
 for(const prefix of corpus.nativeFallback.stays)expect(native.some(row=>prefix.startsWith(row)||row.startsWith(prefix)),prefix).toBe(false);
 expect(native.length).toBeLessThan(buildSchedulingPrefixesV1().length);
});

test("the fallback command inputs of native targets leave the scheduling code out and keep the native-build modules, measured on the real cargo script closure",async()=>{
 const library=join(checkout,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/");
 const inference=await import(join(library,"🟨️.mjs")) as any;await inference.libraryBootstrap;
 const scripts=await import(join(library,"🧭️script-inputs/🟨️.mjs")) as any;
 const cache=inference.createScriptInputCache({workspaceRoot:checkout});
 const script=join(checkout,corpus.nativeFallback.cargoScript);
 const raw=(await scripts.relativeScriptInputs([script],checkout,inference.createScriptInputCache({workspaceRoot:checkout}))).filter((row:unknown)=>typeof row==="string").map((row:string)=>row.slice("{workspaceRoot}/".length));
 const groups=new Map<string,unknown[]>(await scripts.closureApiV1(inference.createScriptInputCache({workspaceRoot:checkout}),checkout).sharedClosureGroups("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers","🔣️.json"));
 const inputs=(await scripts.nativeCommandInputs(checkout,cache)).flatMap((row:any)=>typeof row==="string"?[row]:typeof row?.input==="string"&&groups.has(row.input)?groups.get(row.input)!.filter((item:unknown)=>typeof item==="string"):[]).map((row:string)=>row.slice("{workspaceRoot}/".length));
 const kept=new Set(inputs),native=buildSchedulingNativePrefixesV1();
 const expected=raw.filter((file:string)=>!native.some(prefix=>file.startsWith(prefix)));
 for(const file of expected)expect(kept.has(file),file).toBe(true);
 for(const file of raw.filter((file:string)=>native.some(prefix=>file.startsWith(prefix))))expect(kept.has(file),file).toBe(false);
 for(const prefix of corpus.nativeFallback.stays)expect(raw.some((file:string)=>file.startsWith(prefix)&&kept.has(file)),prefix).toBe(true);
 for(const prefix of corpus.nativeFallback.leaves){expect(raw.some((file:string)=>file.startsWith(prefix)),prefix).toBe(true);expect(inputs.some((file:string)=>file.startsWith(prefix)),prefix).toBe(false);}
 expect(expected.length).toBeLessThan(raw.length);
},120000);

const filesBelow=(path:string):string[]=>{const full=join(checkout,path);if(!statSync(full).isDirectory())return [path];return readdirSync(full,{withFileTypes:true}).filter(row=>row.name!=="node_modules").flatMap(row=>filesBelow(path+(path.endsWith("/")?"":"/")+row.name+(row.isDirectory()?"/":"")));};

test("Law: the content islands next to scheduling code are real, stay keyed down to every file, and equal the independent Python overlap check",()=>{
 const islands=buildSchedulingKeyedPathsV1();
 expect(islands.length).toBeGreaterThan(0);
 for(const island of islands){
  expect(statSync(join(checkout,island)).isDirectory()||statSync(join(checkout,island)).isFile(),island).toBe(true);
  for(const file of filesBelow(island))expect(buildSchedulingDomainOfV1(file),file).toBeUndefined();
 }
 const run=spawnSync("python3",["-I",oracleFile,"islands"],{input:JSON.stringify(declared),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout),"the declared table has no overlap").toEqual([]);
 for(const [name,make] of Object.entries(islandForgeries()).filter(([name])=>name!=="keyed-missing")){
  const table=make() as Draft,overlaps=JSON.parse(spawnSync("python3",["-I",oracleFile,"islands"],{input:JSON.stringify(table),encoding:"utf8"}).stdout) as string[][];
  if(name==="island-inside-prefix"||name==="prefix-inside-island")expect(overlaps.length,name).toBeGreaterThan(0);
  expect(refusal(table),name).toBe("build.scheduling.table-required");
 }
});

test("Negative control: under the table of before (the whole generation store as store custody) an edit of the fact codec moves no key, which is the defect the islands close",()=>{
 const generations=declared.keyed.find(row=>row.path.endsWith("🕰️generations/🟨️.mjs"))!.path.slice(0,-"🟨️.mjs".length),before=copy();
 before.domains.find(row=>row.id==="store-custody")!.prefixes.push(generations);
 const codec=generations+"🧮️fact-codec/🟨️.mjs",snapshot:BuildSourceClosureV1={version:1,root:"/r",entry:"entry.ts",files:[file("entry.ts","a"),file(codec,"k")]},ids=["store-custody"];
 const identity=(text:string)=>buildSourceClosureIdentityV1(splitBuildSourceClosureV1({...snapshot,files:[snapshot.files[0]!,file(codec,text)]},ids,before as unknown as BuildSchedulingV1).keyed);
 expect(identity("k"),"unrefused by the old table").toBe(identity("an edited codec"));
 const now=(text:string)=>buildSourceClosureIdentityV1(splitBuildSourceClosureV1({...snapshot,files:[snapshot.files[0]!,file(codec,text)]},ids).keyed);
 expect(now("k")).not.toBe(now("an edited codec"));
});

test("Law: whatever derives, encodes or digests a graph fact stays keyed: an edit of a generation codec, of the churn protocol or of the capture snapshot moves the keyed identity, an edit of a pin, a claim or a coverage stamp does not",()=>{
 const generations=declared.keyed.find(row=>row.path.endsWith("🕰️generations/🟨️.mjs"))!.path.slice(0,-"🟨️.mjs".length),churn=declared.keyed.find(row=>row.path.endsWith("🔁️churn/🟦️.ts"))!.path;
 const keyedFiles=[generations+"🟨️.mjs",generations+"🧮️fact-codec/🟨️.mjs",generations+"🌍️environment/🟨️.mjs",generations+"✂️cutoff/🟨️.mjs",generations+"🔌️load/🟨️.mjs",churn,churn.slice(0,-"🟦️.ts".length)+"📸️snapshot/🟦️.ts"],custodyFiles=[generations+"🧷️pin/🟨️.mjs",generations+"🔭️coverage/🟨️.mjs",generations+"🧩️claims/🟨️.mjs"];
 const ids=buildSchedulingV1().domains.map(row=>row.id),entry="entry.ts";
 const snapshot:BuildSourceClosureV1={version:1,root:"/r",entry,files:[file(entry,"a"),...keyedFiles.map(path=>file(path,"k")),...custodyFiles.map(path=>file(path,"c"))]};
 const identityOf=(edit?:string)=>buildSourceClosureIdentityV1(splitBuildSourceClosureV1({...snapshot,files:snapshot.files.map(row=>row.path===edit?file(row.path,"edited"):row)},ids).keyed);
 const before=identityOf();
 for(const path of keyedFiles){expect(statSync(join(checkout,path)).isFile(),path).toBe(true);expect(identityOf(path),path).not.toBe(before);}
 for(const path of custodyFiles){expect(statSync(join(checkout,path)).isFile(),path).toBe(true);expect(identityOf(path),path).toBe(before);}
 const wide=copy();wide.domains.find(row=>row.id==="store-custody")!.prefixes.push(generations);
 expect(refusal(wide),"widening the custody prefix back over the generation store is refused when the table is admitted").toBe("build.scheduling.table-required");
});
