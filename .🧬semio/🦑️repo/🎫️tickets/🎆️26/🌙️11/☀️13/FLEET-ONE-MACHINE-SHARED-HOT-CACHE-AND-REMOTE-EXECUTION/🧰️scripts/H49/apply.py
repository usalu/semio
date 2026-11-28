import sys,os,hashlib,glob
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def find(rel):
    p=R+rel
    return p
UC=R+"⚡️caching/🦀️cargo/🗃️unit-cache/"
RC=R+"🏗️build/🌐️remote-cache/"
RE=R+"🏗️build/🌐️remote-execution/"
EX=R+"🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json"
check=len(sys.argv)>1 and sys.argv[1]=="check"
edits=[]
def rep(path,old,new,count=1):
    edits.append((path,old,new,count))

S=UC+"🔏️key/🔗️sources/🟦️.ts"
rep(S,"/** 📦️ Whether a directory is the unpacked source of one version of a registry crate",
"""/** 🌫️ A captured compiler stream (`stderr`, `stdout`) without the number rustc draws for the file that holds a type name too long to print (`long-type-<digits>.txt`, another number in every compilation of one unit): two compilations of one unit, on one machine or two, then give the same stream, and the stored result of a unit whose warning names such a file is no longer refused as diverged. Everything else stays byte for byte; equal compilations give equal text. */
export const canonicalStreamV1=(text:string):string=>text.replace(/long-type-[0-9]+\\.txt/gu,"long-type-0.txt");

/** 🏷️ The name under which the captured streams of a unit are converted for the fleet (see {@link canonicalOutputV1}); no output file has it. */
export const STREAM_NAME_V1="⟨stream⟩";

/** 🔀️ The text of one stored output of a unit as the fleet keeps it: a dependency file without what the compilation itself wrote ({@link canonicalDepInfoV1}), a captured stream without compiler noise ({@link canonicalStreamV1}), anything else unchanged. */
export function canonicalOutputV1(name:string,text:string):string{return name.endsWith(".d")?canonicalDepInfoV1(text):name===STREAM_NAME_V1?canonicalStreamV1(text):text;}

/** 📦️ Whether a directory is the unpacked source of one version of a registry crate""")

U=UC+"🛰️fleet/📤️uploader/🟦️.ts"
rep(U,'import {canonicalDepInfoV1} from "../../🔏️key/🔗️sources/🟦️.ts";','import {canonicalOutputV1,STREAM_NAME_V1} from "../../🔏️key/🔗️sources/🟦️.ts";')
rep(U,'portableTextV1(name.endsWith(".d")?canonicalDepInfoV1(decoder.decode(bytes)):decoder.decode(bytes),roots)','portableTextV1(canonicalOutputV1(name,decoder.decode(bytes)),roots)')
rep(U,'value==null?null:convert(value,true);','value==null?null:convert(value,true,STREAM_NAME_V1);')

K=UC+"🔏️key/🟦️.ts"
rep(K,"/** 📦️ The OUT_DIR of a build script as a key value:",
"""/** 🔢️ The front-end thread count a command line ends up with (`-Z threads=N` in either spelling, the last one wins), `undefined` when it names none; the unit event of a compilation tells it, so the policy of {@link deterministicFrontEndArgumentsV1} is visible in the telemetry. */
export function frontEndThreadsV1(args:readonly string[]):string|undefined{
 let found:string|undefined;
 for(let index=0;index<args.length;index++){const argument=args[index]!,value=/^-Zthreads=(.*)$/u.exec(argument)?.[1]??(args[index-1]==="-Z"?/^threads=(.*)$/u.exec(argument)?.[1]:undefined);if(value!==undefined)found=value;}
 return found;
}

/** 📦️ The OUT_DIR of a build script as a key value:""")

SC=UC+"📜️script.ts"
rep(SC,"withoutIncrementalV1,deterministicFrontEndArgumentsV1,unitKeyComponentsV1","withoutIncrementalV1,deterministicFrontEndArgumentsV1,frontEndThreadsV1,unitKeyComponentsV1")
rep(SC,"let shared=false,held:ClaimV1|undefined,ranMs=0,alone=false,hold:FleetHoldV1|undefined;","let shared=false,held:ClaimV1|undefined,ranMs=0,alone=false,hold:FleetHoldV1|undefined,threadsUsed:string|undefined;")
rep(SC,"try{return await run(settings.inner,[...(shared?deterministicFrontEndArgumentsV1","try{const final=[...(shared?deterministicFrontEndArgumentsV1")
rep(SC,"...await frontEndThreadsArgumentsV1(settings.root,crate,unit.target,args)],{...environment,SEMIO_RUSTC_UNIT_LOCAL_INPUT_KEY:localInputKey},capture);}finally{","...await frontEndThreadsArgumentsV1(settings.root,crate,unit.target,args)];threadsUsed=frontEndThreadsV1(final);return await run(settings.inner,final,{...environment,SEMIO_RUSTC_UNIT_LOCAL_INPUT_KEY:localInputKey},capture);}finally{")
rep(SC,'+(remote?" offloaded:"+remote.origin:""),remote?undefined:outcome.peakRssBytes);lookup("miss"','+(remote?" offloaded:"+remote.origin:"")+(remote||threadsUsed===undefined?"":" threads:"+threadsUsed),remote?undefined:outcome.peakRssBytes);lookup("miss"')

A=RE+"🚀️offload/📜️action/🟦️.ts"
rep(A,"/** 🪶️ The arguments of a `cdylib` final the way the build scheduler runs one locally: one front-end thread (`-Z threads=<n>` becomes `threads=1`).","/** 🪶️ The arguments of every offloaded unit the way a shared unit is compiled locally: one front-end thread (`-Z threads=<n>` becomes `threads=1`).")
rep(A,"Output bytes do not depend on the thread count. Every other unit is returned unchanged. */","A parallel front end orders the definitions it creates while lowering by whichever thread is first, so only one thread gives the same bytes on every machine (see `deterministicFrontEndArgumentsV1`), and the requester shares an offloaded unit through the fleet like a local one. */")
rep(A,' if(!offloadCrateTypesOfV1(args).includes("cdylib"))return args;\n const threads=',' const threads=')

T=RE+"🚀️offload/🧪️tests/🕸️wasm-cdylib/🟦️.ts"
rep(T,"a cdylib final is sent with one front-end thread and no back-end flag (the worker sizes the back end); every other unit is unchanged","every offloaded unit is sent with one front-end thread (a parallel front end gives other metadata bytes on other machines) and a cdylib final no back-end flag (the worker sizes the back end)")
rep(T,'expect(offloadFrontEndArgumentsV1(library)).toBe(library);','expect(offloadFrontEndArgumentsV1(library)).toEqual(["--crate-type","lib","-Z","threads=1","a.rs"]);\n expect(offloadFrontEndArgumentsV1(["--crate-type=proc-macro","-Zthreads=0","--cfg","threads=0"])).toEqual(["--crate-type=proc-macro","-Zthreads=1","--cfg","threads=0"]);')

EV=RC+"🧾️provenance/🗂️evidence/🟦️.ts"
rep(EV,"/** 📐️ What the evidence of divergences may hold: the newest 64 per class of result, 256 MiB in all (the oldest go first), 16 MiB per kept blob (a larger blob is named by its digest only). */\nexport const EVIDENCE_LIMITS_V1=Object.freeze({perClass:64,bytes:268435456,blob:16777216});",
"""/** 📐️ What the evidence of divergences may hold: the newest 64 per class of result and, for units, the newest 16 per kind of divergence (a flood of one kind never pushes out the evidence of the others), 256 MiB in all (the oldest go first), 16 MiB per kept blob (a larger blob is named by its digest only). */
export const EVIDENCE_LIMITS_V1=Object.freeze({perClass:64,perKind:16,bytes:268435456,blob:16777216});

/** 🏷️ The kind of a divergence from the output files that differ: `stream` (the captured stdout and stderr, named `⟨stdout⟩`, `⟨stderr⟩`), `rmeta`, `rlib`, `dep-info` (`.d`), `dylib`, `binary` (a name without extension), `other`; several kinds of one divergence are joined with `+` in sorted order, `none` when no file is named. */
export function divergenceKindV1(differing:readonly string[]):string{
 const kind=(path:string):string=>{
  if(path.startsWith("⟨"))return "stream";
  const name=path.split("/").at(-1)!;
  if(/\\.rmeta(\\.|$)/u.test(name))return "rmeta";
  if(name.endsWith(".rlib"))return "rlib";
  if(name.endsWith(".d"))return "dep-info";
  if(/\\.(dylib|so|dll)$/u.test(name))return "dylib";
  return name.includes(".")?"other":"binary";
 };
 return[...new Set(differing.map(kind))].sort().join("+")||"none";
}""")
rep(EV,"  for(const list of perClass.values())for(const row of list.slice(0,Math.max(0,list.length-EVIDENCE_LIMITS_V1.perClass)))await drop(row.id);",
"""  for(const list of perClass.values())for(const row of list.slice(0,Math.max(0,list.length-EVIDENCE_LIMITS_V1.perClass)))await drop(row.id);
  const perKind=new Map<string,EvidenceV1[]>();
  for(const row of [...index.values()].filter(entry=>entry.sort==="unit").sort((a,b)=>a.atMs-b.atMs)){const name=divergenceKindV1(row.differing);perKind.set(name,[...(perKind.get(name)??[]),row]);}
  for(const list of perKind.values())for(const row of list.slice(0,Math.max(0,list.length-EVIDENCE_LIMITS_V1.perKind)))await drop(row.id);""")

P=RC+"🧾️provenance/🟦️.ts"
rep(P,'import {openEvidenceV1,type EvidenceStoreV1} from "./🗂️evidence/🟦️.ts";','import {divergenceKindV1,openEvidenceV1,type EvidenceStoreV1} from "./🗂️evidence/🟦️.ts";')
rep(P,"/** 🔀️ What one producer-consumer pair of origins exchanged:","""/** 🕐️ The divergences of one clock hour (UTC, `hour` is its start in milliseconds): how many and of which kind of output (`stream`, `rmeta`, `rlib`, `rlib+rmeta`, … see `divergenceKindV1`), so a kind that keeps coming is seen per hour. */
export type DivergenceHourV1=Readonly<{hour:number;count:number;byKind:Readonly<Record<string,number>>}>;

/** 🔀️ What one producer-consumer pair of origins exchanged:""")
rep(P,"lastHour:DivergenceWindowV1;advisory:Readonly<Record<string,number>>;superseded","lastHour:DivergenceWindowV1;hourly:readonly DivergenceHourV1[];advisory:Readonly<Record<string,number>>;superseded")
rep(P,"const address=(namespace:string,kind:string,key:string):string=>","""const hourlyOf=(rows:readonly DivergenceV1[],now:number):DivergenceHourV1[]=>{
 const buckets=new Map<number,{count:number;byKind:Record<string,number>}>();
 for(const row of rows){if(row.atMs<now-24*HOUR)continue;const hour=Math.floor(row.atMs/HOUR)*HOUR,bucket=buckets.get(hour)??{count:0,byKind:{}},kind=divergenceKindV1(row.differing);bucket.count++;bucket.byKind[kind]=(bucket.byKind[kind]??0)+1;buckets.set(hour,bucket);}
 return[...buckets].sort(([a],[b])=>a-b).map(([hour,bucket])=>({hour,count:bucket.count,byKind:Object.fromEntries(Object.entries(bucket.byKind).sort(([a],[b])=>a<b?-1:1))}));
};
const address=(namespace:string,kind:string,key:string):string=>""")
rep(P,"lastHour:windowOf(divergences,clock()-HOUR),advisory:","lastHour:windowOf(divergences,clock()-HOUR),hourly:hourlyOf(divergences,clock()),advisory:")

SV=RC+"🖥️server/🟦️.ts"
rep(SV,"type DivergenceV1,type DivergenceWindowV1","type DivergenceHourV1,type DivergenceV1,type DivergenceWindowV1")
rep(SV,"lastHour?:DivergenceWindowV1;advisory?:","lastHour?:DivergenceWindowV1;hourly?:readonly DivergenceHourV1[];advisory?:")

CM=RC+"⌨️command/🟦️.ts"
rep(CM," const status={version:1,configured:true,contract,",""" const divergenceHours=(server?.divergences?.hourly??[]).map(row=>{
  const kinds=Object.entries(row.byKind).sort(([a],[b])=>a<b?-1:1).map(([name,count])=>`${name} ${count}`).join(", "),hour=new Date(row.hour).toISOString().slice(0,13)+":00Z";
  return {hour:row.hour,count:row.count,byKind:row.byKind,text:Object.fromEntries(LANGUAGES.map(language=>[language,remoteCacheTextV1(language,"status.divergences.hour",{hour,count:row.count,kinds})]))};
 });
 const status={version:1,configured:true,contract,""")
rep(CM,"probes,server,serverFault,store,","probes,server,serverFault,divergenceHours,store,")

TX=RC+"📖️texts/🔣️.json"
rep(TX,'    "status.off": {','''    "status.divergences.hour": {
      "en": "{hour}: {count} cache divergences by kind of output: {kinds}",
      "de": "{hour}: {count} Cache-Abweichungen nach Art der Ausgabe: {kinds}"
    },
    "status.off": {''')

rep(EX,'''            "advisory": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },''','''            "hourly": { "description": "The divergences per clock hour (UTC) of the last 24 hours that have any, oldest first, with the count per kind of output that differs (`stream`, `rmeta`, `rlib`, `rlib+rmeta`, …) / Die Abweichungen je voller Stunde (UTC) der letzten 24 Stunden, die welche haben, älteste zuerst, mit der Anzahl je Art der abweichenden Ausgabe.", "type": "array", "items": { "type": "object", "additionalProperties": false, "required": ["hour", "count", "byKind"], "properties": { "hour": { "type": "integer", "minimum": 0 }, "count": { "type": "integer", "minimum": 1 }, "byKind": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } } } } },
            "advisory": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },''')
rep(EX,'        "serverFault": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/ServerFault" }] },','        "serverFault": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/ServerFault" }] },\n        "divergenceHours": { "description": "The divergences of the running server per clock hour (UTC) and kind of output, each row with its sentence in both languages / Die Abweichungen des laufenden Servers je voller Stunde (UTC) und Art der Ausgabe, jede Zeile mit ihrem Satz in beiden Sprachen.", "type": "array", "items": { "type": "object", "additionalProperties": false, "required": ["hour", "count", "byKind", "text"], "properties": { "hour": { "type": "integer", "minimum": 0 }, "count": { "type": "integer", "minimum": 1 }, "byKind": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } }, "text": { "type": "object", "additionalProperties": false, "required": ["en", "de"], "properties": { "en": { "type": "string" }, "de": { "type": "string" } } } } } },')

# tests
PT=UC+"🧪️tests/🧳️portable/🟦️.ts"
rep(PT,"import {deterministicFrontEndArgumentsV1,keyEnvironmentV1,","import {deterministicFrontEndArgumentsV1,frontEndThreadsV1,keyEnvironmentV1,")
rep(PT,"import {canonicalDepInfoV1,compilerWrittenV1,keyGapV1,unitSourcesV1}","import {canonicalDepInfoV1,canonicalOutputV1,canonicalStreamV1,compilerWrittenV1,keyGapV1,unitSourcesV1}")
rep(PT,'test("a shared unit is compiled with one front-end thread in every spelling, and nothing else of the command line moves",()=>{','''test("a captured stream names the file of a long type name the same in every compilation (the digits are compiler noise), equal to an independent Python substitution",()=>{
 const stream=(digits:string)=>'{"message":"the full name for the type has been written to \\'@@semio-lane@@/z/c@@semio-extra@@.long-type-'+digits+'.txt\\'","rendered":"note: long-type-'+digits+'.txt"}\\n{"keep":"long-type-abc.txt short-type-12.txt"}\\n';
 const first=canonicalStreamV1(stream("10939469467408083722")),second=canonicalStreamV1(stream("11235794353410082362"));
 expect(first).toBe(second);expect(canonicalStreamV1(first)).toBe(first);expect(first).toContain("long-type-abc.txt short-type-12.txt");expect(first).not.toContain("10939469467408083722");
 expect(canonicalOutputV1("⟨stream⟩",stream("5"))).toBe(canonicalStreamV1(stream("5")));expect(canonicalOutputV1("out/x.rlib",stream("5"))).toBe(stream("5"));
 const python=spawnSync("python3",["-I","-c","import re,sys;sys.stdout.write(re.sub(r'long-type-[0-9]+\\\\.txt','long-type-0.txt',sys.stdin.read()))"],{input:stream("10939469467408083722"),encoding:"utf8"});
 if(python.status===0)expect(first).toBe(python.stdout);
});

test("the front-end thread count of a final command line is read in either spelling and the last one wins",()=>{
 expect(frontEndThreadsV1(["-Z","threads=0","-Zthreads=1"])).toBe("1");
 expect(frontEndThreadsV1(["-Z","threads=4","-C","opt-level=0","-Z","threads=2"])).toBe("2");
 expect(frontEndThreadsV1(["--cfg","threads=0"])).toBeUndefined();expect(frontEndThreadsV1([])).toBeUndefined();
});

test("a shared unit is compiled with one front-end thread in every spelling, and nothing else of the command line moves",()=>{''')

PV=RC+"🧾️provenance/🧪️tests/🟦️.ts"
rep(PV,'import {openProvenanceV1,PROVENANCE_ADVISORY_OUTPUTS_V1','import {divergenceKindV1} from "../🗂️evidence/🟦️.ts";\nimport {openProvenanceV1,PROVENANCE_ADVISORY_OUTPUTS_V1')
rep(PV,'test("evidence keeps both sides of a divergence, replaces one per key and origin,','''test("the divergences are counted per clock hour and per kind of output, and a flood of one kind never pushes out the evidence of the others",async()=>{
 const hour=3_600_000,root=directory();let now=10*hour+5;
 const first=await openProvenanceV1(root,{now:()=>now});
 const row=(index:number,differing:string[])=>({namespace:"main",kind:"ac" as const,key:String(index).padStart(64,"0"),keptOrigin:"g1-m01",keptSha256:"a".repeat(64),refusedOrigin:"g1-m02",refusedSha256:"b".repeat(64),sort:"unit",differing});
 first.diverged(row(1,["⟨stderr⟩"]));first.diverged(row(2,["⟨stderr⟩"]));first.diverged(row(3,["out/liba@@semio-extra@@.rlib","out/liba@@semio-extra@@.rmeta","out/liba@@semio-extra@@.rmeta.semio-deps"]));
 now+=hour;first.diverged(row(4,["out/b@@semio-extra@@.d"]));first.diverged(row(5,["out/build_script_build-1a2b"]));
 expect(first.summary().hourly).toEqual([{hour:10*hour,count:3,byKind:{"rlib+rmeta":1,stream:2}},{hour:11*hour,count:2,byKind:{binary:1,"dep-info":1}}]);
 now+=25*hour;expect(first.summary().hourly).toEqual([]);
 expect(divergenceKindV1([])).toBe("none");expect(divergenceKindV1(["⟨stdout⟩","⟨stderr⟩"])).toBe("stream");expect(divergenceKindV1(["x.dylib","y.so"])).toBe("dylib");expect(divergenceKindV1(["a.json"])).toBe("other");
 const blobs=new Map<string,Uint8Array>([["h1",new Uint8Array([1])],["h2",new Uint8Array([2])]]);
 const input=(key:string,path:string)=>({outcome:"refused" as const,sort:"unit",namespace:"main",key,keptOrigin:"g1-m01",refusedOrigin:"g1-m02",differing:[path],kept:new Uint8Array([9]),refused:new Uint8Array([8]),keptRows:[{path,hash:"h1"}],refusedRows:[{path,hash:"h2"}],blob:async(hash:string)=>blobs.get(hash)});
 for(let index=0;index<3;index++){now+=1;await first.evidence.keep(input(String(100+index).padStart(64,"0"),"out/c.rlib"));}
 for(let index=0;index<40;index++){now+=1;await first.evidence.keep(input(String(index).padStart(64,"0"),"⟨stderr⟩"));}
 const kept=first.evidence.list().map(entry=>divergenceKindV1(entry.differing));
 expect(kept.filter(kind=>kind==="stream").length).toBe(16);expect(kept.filter(kind=>kind==="rlib").length).toBe(3);
});

test("evidence keeps both sides of a divergence, replaces one per key and origin,''')

TS=RE+"🛰️sidecar/🧬️schema/🔣️.json"

AD="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/"
RS=AD+"🦀️.rs"
rep(RS,"    let mut actor_decoder = false;\n    let mut wasip2 = false;\n","    let mut actor_input = false;\n")
rep(RS,'        actor_decoder |= name == Some("semio_plugin_actor_input_decoder");\n        wasip2 |= target == Some("wasm32-wasip2");\n','        actor_input |= matches!(name, Some("semio_plugin_actor_input_decoder" | "semio_plugin_actor_input_wire"));\n')
rep(RS,"    let serial_unit = final_component || (actor_decoder && wasip2);","    let serial_unit = final_component || actor_input;")
rep(RS,"/// The concrete WASI actor decoder measured 1,173 MiB with the serial policy; wider codegen exceeded its library memory ceiling.","/// The actor input decoder and wire crates are compiled serially on every target (one front-end thread, no parallel back end): the WASI decoder measured 1,173 MiB serial against 2,280 MiB wide, and the host decoder and wire peaked at 1,836 and 1,749 MiB under the admitted parallelism (warden alarms of 2026-11-27, which the front-end thread count alone does not change: shared units already run one front-end thread).")
PY=AD+"🧪️tests/🧵️frontend-threads/🐍️.py"
rep(PY,'    bounded_decoder = (option("--crate-name"), option("--target")) == ("semio_plugin_actor_input_decoder", "wasm32-wasip2")','    bounded_decoder = option("--crate-name") in ("semio_plugin_actor_input_decoder", "semio_plugin_actor_input_wire")')
JS=AD+"🧪️tests/🧵️frontend-threads/🔣️.json"
rep(JS,'''   "id": "actor-decoder-host-keeps-grant",''','''   "id": "actor-decoder-host-serial",''')
rep(JS,'''   "note": "native actor decoder retains the admitted parallelism",
   "expected": [
    "--crate-name",
    "semio_plugin_actor_input_decoder",
    "--target=aarch64-apple-darwin",
    "--crate-type=lib",
    "-Zthreads=5"
   ]
  },''','''   "note": "the native actor decoder is compiled serially like the WASI one (1836 MB under the admitted parallelism, above its ceiling)",
   "expected": [
    "--crate-name",
    "semio_plugin_actor_input_decoder",
    "--target=aarch64-apple-darwin",
    "--crate-type=lib",
    "-Zthreads=1",
    "-Z",
    "no-parallel-backend"
   ]
  },
  {
   "id": "actor-wire-host-serial",
   "args": [
    "--crate-name",
    "semio_plugin_actor_input_wire",
    "--crate-type=lib",
    "-Zthreads=0"
   ],
   "tokens": 5,
   "note": "the actor wire crate peaked at 1749 MB (host) and 2269 MB (WASI) under the admitted parallelism: serial on every target",
   "expected": [
    "--crate-name",
    "semio_plugin_actor_input_wire",
    "--crate-type=lib",
    "-Zthreads=1",
    "-Z",
    "no-parallel-backend"
   ]
  },''')

files={}
for path,old,new,count in edits:
    files.setdefault(path,[]).append((old,new,count))
bad=False
out={}
for path,rows in files.items():
    text=open(path,encoding="utf8").read()
    before=hashlib.sha256(text.encode()).hexdigest()[:8]
    for old,new,count in rows:
        n=text.count(old)
        if n!=count:
            print("ANCHOR",n,"!=",count,path.split("/")[-2:],old[:70].replace("\n","\\n"));bad=True;continue
        text=text.replace(old,new)
    out[path]=(text,before)
if bad:sys.exit(1)
if check:
    print("all anchors unique:",len(edits),"edits in",len(files),"files");sys.exit(0)
stage=os.environ.get("H49_STAGE")
if stage:
    os.makedirs(stage,exist_ok=True)
    for index,(path,(text,before)) in enumerate(out.items()):
        open(os.path.join(stage,str(index)+"-"+os.path.basename(path).replace("🟦️","index").replace("📜️","script")),"w",encoding="utf8").write(text)
    print("staged",len(out),"files in",stage);sys.exit(0)
for path,(text,before) in out.items():
    tmp=path+".h49tmp"
    open(tmp,"w",encoding="utf8").write(text)
    os.replace(tmp,path)
    print(before,"->",hashlib.sha256(text.encode()).hexdigest()[:8],"/".join(path.split("/")[-3:]))
