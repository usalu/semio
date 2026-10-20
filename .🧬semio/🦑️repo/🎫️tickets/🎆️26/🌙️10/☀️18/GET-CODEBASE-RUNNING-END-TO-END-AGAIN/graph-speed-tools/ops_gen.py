#!/usr/bin/env python3
"""Builds staged/ops.json: every anchored edit of the graph-speed set, extracted from the LIVE tree (anchor = exact current text, each must occur exactly once), plus the base sha256 of every touched file and the new-file map.

usage: ops_gen.py [--root REPO_ROOT]          (re-run after any live drift; apply.py refuses drifted anchors)"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
if "--root" in sys.argv:
    ROOT = sys.argv[sys.argv.index("--root") + 1]


def E(cp):
    return chr(cp) + "️"


def parts(*codes):
    return [E(c) if isinstance(c, int) else c for c in codes]


L = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(L, E(0x1F3D7) + "build")
GRAPH = os.path.join(BUILD, E(0x1F578) + "graph")
RECORDS = os.path.join(L, E(0x26A1) + "caching", E(0x1F5C3) + "records")
PROCESS = os.path.join(E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process", E(0x1F9ED) + "routing", E(0x1F4E5) + "invocation", E(0x1F3C3) + "process")
JS, TS, JSON_, DMTS = E(0x1F7E8) + ".mjs", E(0x1F7E6) + ".ts", E(0x1F523) + ".json", E(0x1F7E8) + ".d.mts"
TESTS, SCHEMA = E(0x1F9EA) + "tests", E(0x1F9EC) + "schema"
LEAN = os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory")
PHASES = os.path.join(GRAPH, E(0x23F1) + "phases")
OWNER = os.path.join(GRAPH, E(0x1F9ED) + "file-owner")
SEALED_TESTS = os.path.join(GRAPH, TESTS, E(0x1F513) + "sealed-reuse")
RECORD_TESTS = os.path.join(RECORDS, TESTS, E(0x1F50C) + "descriptor-limit")
PROGRESS_TESTS = os.path.join(PROCESS, TESTS, E(0x1F4C8) + "graph-phase")

NEW = {
    "lean/module.mjs": os.path.join(LEAN, JS), "lean/module.d.mts": os.path.join(LEAN, DMTS),
    "lean/test.ts": os.path.join(LEAN, TESTS, TS), "lean/fixture.json": os.path.join(LEAN, TESTS, JSON_), "lean/fixture-schema.json": os.path.join(LEAN, TESTS, SCHEMA, JSON_),
    "phases/module.mjs": os.path.join(PHASES, JS), "phases/module.d.mts": os.path.join(PHASES, DMTS),
    "phases/test.ts": os.path.join(PHASES, TESTS, TS), "phases/fixture.json": os.path.join(PHASES, TESTS, JSON_), "phases/fixture-schema.json": os.path.join(PHASES, TESTS, SCHEMA, JSON_),
    "records/test.ts": os.path.join(RECORD_TESTS, TS), "records/fixture.json": os.path.join(RECORD_TESTS, JSON_), "records/fixture-schema.json": os.path.join(RECORD_TESTS, SCHEMA, JSON_),
    "owner/module.mjs": os.path.join(OWNER, JS), "owner/module.d.mts": os.path.join(OWNER, DMTS),
    "owner/test.ts": os.path.join(OWNER, TESTS, TS), "owner/fixture.json": os.path.join(OWNER, TESTS, JSON_), "owner/fixture-schema.json": os.path.join(OWNER, TESTS, SCHEMA, JSON_),
    "sealed/test.ts": os.path.join(SEALED_TESTS, TS), "sealed/fixture.json": os.path.join(SEALED_TESTS, JSON_), "sealed/fixture-schema.json": os.path.join(SEALED_TESTS, SCHEMA, JSON_),
    "sealed/oracle.py": os.path.join(SEALED_TESTS, E(0x1F40D) + "oracle.py"),
    "progress/test.ts": os.path.join(PROGRESS_TESTS, TS), "progress/fixture.json": os.path.join(PROGRESS_TESTS, JSON_), "progress/fixture-schema.json": os.path.join(PROGRESS_TESTS, SCHEMA, JSON_),
}


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def between(path, start, end, include_end=False):
    text = read(path)
    first = text.index(start)
    last = text.index(end, first)
    if text.count(start) != 1:
        raise SystemExit("start marker not unique in %s: %r" % (path, start[:60]))
    return text[first:last + (len(end) if include_end else 0)]


ops = []


def op(path, anchor, replacement):
    ops.append({"path": path, "anchor": anchor, "replacement": replacement})


# ---- 1. records: bounded process-wide descriptor set ---------------------------------------------------------------------------
R = os.path.join(RECORDS, TS)
op(R, "const PENDING_FLUSH_BYTES = 64 * 1024 * 1024;\n", """const PENDING_FLUSH_BYTES = 64 * 1024 * 1024;

/** 🔌️ Packs keep at most this many read descriptors open across every store of the process: the least recently read one is closed first, so a graph that visits thousands of project stores never exhausts the descriptor table. */
export const RECORD_STORE_DESCRIPTOR_LIMIT = 32;
const openDescriptors = new Map<string, () => void>();

/** 📊️ Counts the pack descriptors this process currently keeps open (tests and diagnostics). */
export function openRecordStoreDescriptorCount(): number { return openDescriptors.size; }
""")
op(R, """  const descriptor = (pack: string): number | null => {
    if (!descriptors.has(pack)) { try { descriptors.set(pack, openSync(join(root, `${pack}.pack`), "r")); } catch { descriptors.set(pack, null); } }
    return descriptors.get(pack)!;
  };
""", """  const release = (pack: string): void => {
    const fd = descriptors.get(pack);
    openDescriptors.delete(`${root}\\0${pack}`);
    descriptors.delete(pack);
    if (typeof fd === "number") { try { closeSync(fd); } catch {} }
  };
  const descriptor = (pack: string): number | null => {
    const key = `${root}\\0${pack}`, held = descriptors.get(pack);
    if (typeof held === "number") { openDescriptors.delete(key); openDescriptors.set(key, () => release(pack)); return held; }
    if (held === null) return null;
    while (openDescriptors.size >= RECORD_STORE_DESCRIPTOR_LIMIT) { const [oldest, evict] = openDescriptors.entries().next().value!; openDescriptors.delete(oldest); evict(); }
    try { const fd = openSync(join(root, `${pack}.pack`), "r"); descriptors.set(pack, fd); openDescriptors.set(key, () => release(pack)); return fd; }
    catch { descriptors.set(pack, null); return null; }
  };
""")
op(R, 'const fd=descriptors.get(pack);if(typeof fd==="number")closeSync(fd);descriptors.delete(pack);for(const suffix of["idx","pack"])',
   'release(pack);for(const suffix of["idx","pack"])')
op(R, """      for (const fd of descriptors.values()) if (typeof fd === "number") closeSync(fd);
      descriptors.clear();}""", """      for (const pack of [...descriptors.keys()]) release(pack);
      descriptors.clear();}""")

# ---- 2. storage: a registered optional handle can be unregistered -------------------------------------------------------------------
ST = os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1F4BE) + "storage", JS)
op(ST, "export function registerBuildGraphOptionalHandleV1(close){const scope=writers.getStore();if(scope)scope.handles.add(close);}",
   "export function registerBuildGraphOptionalHandleV1(close){const scope=writers.getStore();if(!scope)return()=>{};scope.handles.add(close);return()=>{scope.handles.delete(close);};}")
op(os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1F4BE) + "storage", DMTS), "export function registerBuildGraphOptionalHandleV1(close:()=>void):void;",
   "export function registerBuildGraphOptionalHandleV1(close:()=>void):()=>void;")

# ---- 3. observation: a project's record store is released as soon as its inference is persisted ---------------------------------
OB = os.path.join(BUILD, E(0x1F441) + "observation", JS)
op(OB, "const recordStores = new Map();\n", "const recordStores = new Map();\nconst recordStoreHandles = new Map();\n")
old = between(OB, "function observedRecordStoreV1(path){", "\n}\n", True)
new = """function observedRecordStoreV1(path){
 let records=recordStores.get(path);if(!records){records=openRecordStore(path,{publication:(plan,write)=>withBuildGraphOptionalPublicationV1(join(path,"0-0-0000000000000000.pack"),plan,write)});recordStores.set(path,records);const original=records;recordStoreHandles.set(path,registerBuildGraphOptionalHandleV1(()=>{try{original.close();}finally{if(recordStores.get(path)===original){recordStores.delete(path);recordStoreHandles.delete(path);}}}));}return records;
}

/** 🔌️ Closes one project's pack store once its inference is persisted: its descriptors and location index are not kept for the remaining projects of the graph. */
function releaseObservedRecordStoreV1(path){
 const records=recordStores.get(path);if(!records)return;
 recordStores.delete(path);const unregister=recordStoreHandles.get(path);recordStoreHandles.delete(path);
 try{records.close();}finally{unregister?.();}
}
"""
op(OB, old, new)
op(OB, "  if (records.stats.records > 2 * records.stats.accessed) records.retainAccessed();\n  const rows = [...authority, ...observations.values()];",
   "  if (records.stats.records > 2 * records.stats.accessed) records.retainAccessed();\n  releaseObservedRecordStoreV1(`${cachePath}.records`);\n  const rows = [...authority, ...observations.values()];")

# ---- 4. reuse: lean content-addressed verification, per-leaf, remembered by identity -------------------------------------------------
RE = os.path.join(BUILD, E(0x1F441) + "observation", E(0x1F4E5) + "reuse", JS)
op(RE, 'import{withManagedGraphSourceReadBatchV1,readExactSourceInventoryV1}from"../../📥️inputs/🧮️digest/📥️read/🟨️.mjs";'.replace("️", "️"),
   'import{readInventoryRecordLeanV1,rememberInventoryRecordV1,inventoryRecordStillHoldsV1}from"../../📥️inputs/🪶️lean-inventory/🟨️.mjs";')
op(RE, "await verifyInventoryCohortsV1(root,actual,all,false,store.root);", "await verifyInventoryCohortsV1(root,actual,all,false,store.root,store);")
op(RE, "verifyInventoryCohortsV1(root,actual,names,true,store.root));", "verifyInventoryCohortsV1(root,actual,names,true,store.root,store));")
op(RE, "async function verifyInventoryCohortsV1(root,control,names,strict,storeRoot){", "async function verifyInventoryCohortsV1(root,control,names,strict,storeRoot,store){")
old = between(RE, " for(const members of cohorts){\n  checkResourceLeaseControl(control);\n  let holds=false;", "return true;\n}\n", True)
new = """ let done=0;const phase=async()=>{if(++done%128===0){await store.recheck();await control.yieldContinuation();checkResourceLeaseControl(control);}};
 for(const members of cohorts){
  checkResourceLeaseControl(control);await store.recheck();
  for(const name of members){
   const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),directory=resolve(storeRoot,source?"build-input-closures":"build-lock-closures"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length);
   try{
    if(!inventoryRecordStillHoldsV1(root,directory,key)){
     const file=readInventoryRecordLeanV1(directory,key,readPolicy.maximumClosureBytes);
     if(!inferenceInventoryRecordHoldsV1(root,name,file)){if(strict)return false;continue;}
     rememberInventoryRecordV1(root,directory,key,file.stamp);
    }
   }catch(error){if(strict)throw error;continue;}
   if(validInventoryRecords.size>262144)validInventoryRecords.clear();validInventoryRecords.set(root+"\\0"+name,Date.now());
   await phase();
  }
  checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();
 }
 checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();return true;
}
"""
op(RE, old, new)

# ---- 5. graph: lease scope, compact publication, lean expansion, phase timing ------------------------------------------------------
G = os.path.join(GRAPH, TS)
op(G, 'import{withManagedGraphSourceReadBatchV1,readExactSourceInventoryV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";'.replace("️", "️"),
   'import{readInventoryRecordLeanV1,rememberInventoryRecordV1}from"../📥️inputs/🪶️lean-inventory/🟨️.mjs";\nimport{createGraphPhaseClockV1,writeGraphPhaseRowV1}from"./⏱️phases/🟨️.mjs";\nimport{createProjectFileIndexV1}from"./🧭️file-owner/🟨️.mjs";'.replace("️", "️"))
op(G, 'import{captureInferenceInventoryReferencesV1,admitInferenceInventoryCohortV1,planInferenceInventoryCohortsV1}from"../👁️observation/📥️reuse/🟨️.mjs";'.replace("️", "️"),
   'import{captureInferenceInventoryReferencesV1}from"../👁️observation/📥️reuse/🟨️.mjs";'.replace("️", "️"))
op(G, 'import type {RepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";'.replace("️", "️"),
   'import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";'.replace("️", "️"))
old = between(G, "    await withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{\n      const extents", "    });\n    const copies=new WeakMap<object,unknown>();visited=0;")
new = """    await withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{
      const clock=createGraphPhaseClockV1({publish:resource=>actual.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
      let bytes=0,readRecords=0;const cadence=exactSourceReadPolicyV1.maximumInventoryRecordsPerYield;
      await store.recheck();checkResourceLeaseControl(actual);
      if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");
      await clock.begin("expansion",entries.length);
      for(const compact of entries){
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const directory=join(store.root,compact.directory),file=readInventoryRecordLeanV1(directory,compact.digest,exactSourceReadPolicyV1.maximumClosureBytes);
        if(file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
        bytes+=file.byteLength;if(bytes>exactSourceReadPolicyV1.maximumSourceBytes)throw Error("Build input closure bytes exceeded");
        expanded.set(compact.key,compact.directory==="build-input-closures"?captureSourceInputInventoryV1(file.value,root).value.files.map(file=>"{workspaceRoot}/"+file):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(file.value,root).value.manifest]);
        rememberInventoryRecordV1(root,directory,compact.digest,file.stamp);
        if(++readRecords%cadence===0){await store.recheck();await actual.yieldContinuation();checkScriptInvocation(invocation);checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");}
      }
      await store.recheck();checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);await clock.end("expansion",{records:readRecords,bytes});
"""
op(G, old, new)
op(G, "export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1):Promise<BuildGraphV1> {",
   "export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1,compact=false):Promise<BuildGraphV1> {")
op(G, "return readBuildGraphV1({version:1,projects:await portableInputs(root,neutral,invocation,phaseControl),dependencies,", "const compactGraph=({version:1,projects:neutral,dependencies,")
op(G, 'closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});\n}', 'closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});\n  return compact?compactGraph as unknown as BuildGraphV1:readBuildGraphV1({...compactGraph,projects:await portableInputs(root,neutral,invocation,phaseControl)});\n}')
op(G, "async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean}",
   "async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean;sealedOnly?:boolean}")
op(G, 'const profile=process.env.SEMIO_BUILD_PROFILE_INPUTS==="1",timing=(phase:string,started:number,details:Record<string,unknown>={})=>{if(profile){}};',
   'const phaseRows:{phase:string;milliseconds:number}[]=[],timing=(phase:string,started:number,details:Record<string,unknown>={})=>{const row={phase,milliseconds:Math.round(performance.now()-started),...details};phaseRows.push(row);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"||process.env.SEMIO_GRAPH_PHASES==="1")process.stderr.write("[graph-phase] "+JSON.stringify(row)+"\\n");};')
op(G, '  const observation=observationV1,[producer]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();\n',
   '  if(options.sealedOnly){if(selected)return undefined;try{if(!buildGraphProducerAuthorityV1(JSON.parse(await readFile(authorityPath,"utf8")),loadedGraphModuleAuthorityV1))return undefined;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||error instanceof SyntaxError)return undefined;throw error;}}\n  const observation=observationV1,[producer]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();\n')
op(G, "  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;", "  if(options.sealedOnly)return undefined;\n  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;")
op(G, "return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission);",
   "return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission,true);")
old = between(G, "  const graph=captured.value,source=JSON.stringify(graph),digest=", 'await mkdir(dirname(path),{recursive:true});', True)
new = old.replace("const graph=captured.value,source=JSON.stringify(graph),digest=", "const publishStarted=performance.now(),compact=captured.value as BuildGraphV1,source=JSON.stringify(compact),digest=")
new = new.replace('throw Error("Build graph cache exceeds its finite publication limit");', 'throw Error("Build graph cache exceeds its finite publication limit: "+Buffer.byteLength(source)+" graph bytes, "+Buffer.byteLength(authority)+" authority bytes, 536870912 allowed");')
new = new.replace(";if(Buffer.byteLength(source)>536870912", ";timing(\"graph-serialize\",publishStarted,{graphBytes:Buffer.byteLength(source),authorityBytes:Buffer.byteLength(authority)});if(Buffer.byteLength(source)>536870912")
new = new + 'const expandStarted=performance.now(),graph=readBuildGraphV1(await portableInputs(root,compact,invocation,phaseControl));timing("graph-expansion",expandStarted);'
op(G, old, new)
op(G, "await rename(temporary,path);await rename(receipt,authorityPath);}", 'timing("graph-publication",publishStarted);process.stderr.write("[graph-phase] "+JSON.stringify({summary:phaseRows.map(row=>[row.phase,row.milliseconds])})+"\\n");await rename(temporary,path);await rename(receipt,authorityPath);}')
op(G, "/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */",
   "/** 🔓️ Serves a current sealed graph to any number of shared readers: only a cache-store reader is held, the graph writer and its exclusive lease are never requested, so readers never queue behind a builder that is still inferring. */\n"
   "async function readSealedBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry:BuildOwnerRegistryV1):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {\n"
   "  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);\n"
   "  return withRepositoryCacheStoreV1(root,\"graph\",invocation,phaseControl,async(store,actualControl)=>{await store.recheck();checkResourceLeaseControl(actualControl);return loadBuildGraphAttemptV1(root,invocation,actualControl,{...options,sealedOnly:true},registry,undefined);});\n}\n\n"
   "/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */")
op(G, "  const ordered=Object.entries(projects).sort(([,a],[,b])=>b.root.length-a.root.length),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));\n  const withheld=[...unavailable.keys()];\n  for(const file of files){const match=ordered.find(([,project])=>project.root===\".\"||file.startsWith(project.root+\"/\"));if(match&&!withheld.some(root=>file.startsWith(root+\"/\")&&root.length>match[1].root.length)){const entries=projectFileMap[match[0]];if(!entries)throw Error(\"Discovered project file owner is missing\");entries.push({file});}}\n",
   "  const ownerOf=createProjectFileIndexV1(projects),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));\n  const withheld=[...unavailable.keys()];\n  for(const file of files){const owner=ownerOf(file);if(owner!==undefined&&!withheld.some(root=>file.startsWith(root+\"/\")&&root.length>projects[owner]!.root.length)){const entries=projectFileMap[owner];if(!entries)throw Error(\"Discovered project file owner is missing\");entries.push({file});}}\n")
op(G, "  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).values()].sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));\n",
   "  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).entries()].sort(([a],[b])=>a.localeCompare(b)).map(([,edge])=>edge);\n")
op(G, "    const graph=await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{",
   "    if(scope===undefined){const sealed=await readSealedBuildGraphV1(root,invocation,phaseControl,options,registry);if(sealed&&!isOwnerRegistryInvalidationV1(sealed)){checkScriptInvocation(invocation);return sealed;}}\n    const graph=await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{")

# ---- 6. project nodes: project counter and timing ------------------------------------------------------------------------------------------
PN = os.path.join(GRAPH, E(0x1F4E5) + "inference", E(0x1F4E6) + "project-nodes", JS)
op(PN, "  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length)await inferProject(jobs[cursor++]);});\n  await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});\n",
   "  const phases=context.invocation?createGraphPhaseClockV1({publish:async resource=>{context.invocation.control.signal.throwIfAborted();await context.invocation.control.publish({owner:context.invocation.policy.owner,command:\"build:projects\",stage:\"running\",activity:{resource,mode:\"shared\",elapsedMs:0}});},write:writeGraphPhaseRowV1}):undefined;let finished=0;\n  await phases?.begin(\"inference\",jobs.length);\n  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){await inferProject(jobs[cursor++]);await phases?.advance(\"inference\",++finished);}});\n  await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});\n  await phases?.end(\"inference\",{projects:jobs.length});\n")
pn_text = read(PN)
first_import = pn_text.index("\nlet inferenceCandidates")
op(PN, "\nlet inferenceCandidates", '\nimport { createGraphPhaseClockV1, writeGraphPhaseRowV1 } from "../../⏱️phases/🟨️.mjs";\nlet inferenceCandidates'.replace("️", "️"))

# ---- 7. progress: the graph phase counter family (EN/DE) --------------------------------------------------------------------------------------
PS = os.path.join(PROCESS, SCHEMA, E(0x1F4C8) + "progress.json")
labels = json.load(open(os.path.join(HERE, "staged", "new", "progress", "labels.json"), encoding="utf8"))
block = json.dumps(labels, ensure_ascii=False, indent=2).replace("\n", "\n      ")
op(PS, '    "cacheScan": {\n      "resourcePrefix": "cache-scan:",', '    "graphPhase": {\n      "resourcePrefix": "build:graph-phase:",\n      "maximumResourceCharacters": 128,\n      "maximumCommandCharacters": 256,\n      "commandPattern": "^[A-Za-z0-9:_/.-]+$",\n      "countPattern": "^(?:0|[1-9][0-9]{0,15})$",\n      "labels": ' + block + '\n    },\n    "cacheScan": {\n      "resourcePrefix": "cache-scan:",')
PF = os.path.join(PROCESS, TS)
op(PF, " const queue=progressSchema.const.queueWait;\n", """ const phase=progressSchema.const.graphPhase;
 if(activity.resource.startsWith(phase.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.mode!=="shared"||activity.resource.length>phase.maximumResourceCharacters||progress.command.length>phase.maximumCommandCharacters||!new RegExp(phase.commandPattern,"u").test(progress.command))return null;
  const parts=activity.resource.slice(phase.resourcePrefix.length).split(":"),name=parts[0]??"";if(parts.length!==3||!Object.hasOwn(phase.labels.en.phases,name)||!parts.slice(1).every(part=>new RegExp(phase.countPattern,"u").test(part)))return null;
  const done=Number(parts[1]),total=Number(parts[2]);if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||done>total)return null;
  const labels=phase.labels[progressLanguage(environment)],row=labels.phases[name as keyof typeof labels.phases],fields:Record<string,string>={command:progress.command,phase:row.label,units:row.units,done:String(done),total:String(total)};
  return labels.template.replace(/\\{([A-Za-z]+)\\}/gu,(_token,key:string)=>fields[key]!);
 }
 const queue=progressSchema.const.queueWait;
""")

# ---- 8. the existing portable expansion law follows the lean reader ----------------------------------------------------------------------------
RT = os.path.join(BUILD, E(0x1F441) + "observation", TESTS, E(0x1F4E5) + "reuse", TS)
op(RT, 'import corpus from"./' + JSON_ + '";\n', 'import corpus from"./' + JSON_ + '";\nimport{createGraphPhaseClockV1}from"../../../' + E(0x1F578) + 'graph/' + E(0x23F1) + 'phases/' + JS + '";\n')
old = between(RT, 'test("neutral portable expansion shares capped cohorts and retains refined control",async()=>{', "3+Math.floor(row.count/readPolicy.maximumInventoryRecordsPerYield):0);}\n});\n", True)
new = old.replace("shares capped cohorts and retains refined control", "reads each compact leaf once through the lean reader and retains refined control")
new = new.replace("const records=new Map<string,{value:unknown;sha256:string;byteLength:number}>()", "const records=new Map<string,{value:unknown;sha256:string;byteLength:number;stamp:string}>()")
new = new.replace("records.set(sha256,{value,sha256,byteLength:Buffer.byteLength(body)});", "records.set(sha256,{value,sha256,byteLength:Buffer.byteLength(body),stamp:\"s\"+index});")
new = new.replace("let outer=false,inner=false,batches=0,yields=0,rechecks=0;", "let outer=false,reads=0,remembered=0,yields=0,rechecks=0;const published:string[]=[];")
new = new.replace("actual={yieldContinuation:async()=>", "actual={publish:async(row:{resource:string})=>{published.push(row.resource);},yieldContinuation:async()=>")
new = new.replace("admitInferenceInventoryCohortV1:reuse.admitInferenceInventoryCohortV1,planInferenceInventoryCohortsV1:reuse.planInferenceInventoryCohortsV1,", "createGraphPhaseClockV1,writeGraphPhaseRowV1:()=>{},")
i = new.index("lstat:async(path:string)=>")
j = new.index("captureSourceInputInventoryV1:data.captureSourceInputInventoryV1")
new = new[:i] + 'readInventoryRecordLeanV1:(directory:string,key:string,cap:number)=>{expect(outer).toBe(true);expect(directory).toBe(join(root,"graph","build-input-closures"));const value=records.get(key);if(!value)throw Error("Finite inventory required");expect(value.byteLength<=cap).toBe(true);reads++;return value;},rememberInventoryRecordV1:()=>{remembered++;},' + new[j:]
new = new.replace("expect(batches).toBe(row.batches);expect(outer).toBe(false);expect(inner).toBe(false);expect(yields).toBe(2*Math.floor(row.count/readPolicy.maximumInventoryRecordsPerYield));expect(rechecks).toBe(row.count?3+Math.floor(row.count/readPolicy.maximumInventoryRecordsPerYield):0);",
                  "expect(reads).toBe(row.count);expect(remembered).toBe(row.count);expect(outer).toBe(false);expect(yields).toBe(Math.floor(row.count/readPolicy.maximumInventoryRecordsPerYield));expect(rechecks).toBe(row.count?2+Math.floor(row.count/readPolicy.maximumInventoryRecordsPerYield):0);expect(published.at(-1)).toBe(row.count?`build:graph-phase:expansion:${row.count}:${row.count}`:undefined);if(row.count)expect(published[0]).toBe(`build:graph-phase:expansion:0:${row.count}`);")
op(RT, old, new)

# ---- verify uniqueness and write ----------------------------------------------------------------------------------------------------------
bases, texts = {}, {}
for row in ops:
    texts.setdefault(row["path"], read(row["path"]))
for row in ops:
    count = texts[row["path"]].count(row["anchor"])
    if count != 1:
        raise SystemExit("anchor occurs %d times in %s: %r" % (count, row["path"], row["anchor"][:80]))
for path, text in texts.items():
    bases[path] = hashlib.sha256(text.encode("utf8")).hexdigest()
json.dump({"root": "repository", "ops": ops, "bases": bases, "new": NEW}, open(os.path.join(HERE, "staged", "ops.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("ops:", len(ops), "files:", len(bases), "new:", len(NEW))
