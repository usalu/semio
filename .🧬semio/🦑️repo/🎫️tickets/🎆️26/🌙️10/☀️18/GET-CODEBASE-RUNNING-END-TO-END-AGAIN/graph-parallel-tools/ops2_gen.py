#!/usr/bin/env python3
"""Builds staged/round2.json (the round 2 increment of the graph farm set) from the CURRENT text of a tree that already has round 1 applied
(anchors must occur exactly once). With --virtual-round1 the text is the live tree plus the operations of staged/ops.json applied in memory, so the live tree can be checked
before round 1 is written.
usage: ops2_gen.py [--root REPO_ROOT] [--virtual-round1]"""
import hashlib, json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
root = HERE
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
if "--root" in sys.argv:
    root = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]).resolve()
VIRTUAL = "--virtual-round1" in sys.argv
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
G = B + "/🕸️graph"
INF = G + "/📥️inference"
NODES = INF + "/📦️project-nodes"
edits, new_files, replace_files, delete_files = [], [], [], []
round1 = json.loads((HERE / "staged/ops.json").read_text(encoding="utf8")) if VIRTUAL else None
current = {}


def text(file):
    if file in current:
        return current[file]
    value = (root / file).read_text(encoding="utf8")
    if VIRTUAL:
        for op in round1["edits"]:
            if op["file"] == file and op["replacement"] not in value:
                if value.count(op["anchor"]) != 1:
                    raise SystemExit("round 1 anchor drift in %s: %r" % (file[-60:], op["anchor"][:80]))
                value = value.replace(op["anchor"], op["replacement"])
    current[file] = value
    return value


def edit(file, anchor, replacement):
    value = text(file)
    count = value.count(anchor)
    if count != 1:
        raise SystemExit("anchor occurs %d times in %s: %r" % (count, file[-60:], anchor[:90]))
    current[file] = value.replace(anchor, replacement)
    edits.append({"file": file, "anchor": anchor, "replacement": replacement})


def span(file, start, end):
    value = text(file)
    i = value.index(start)
    j = value.index(end, i) + len(end)
    return value[i:j]


def replace_file(live, source, round1_source):
    replace_files.append({"file": live, "from": "staged/" + source, "replaces": "staged/" + round1_source})


def new_file(live, source):
    new_files.append({"file": live, "from": "staged/" + source})


def delete_file(live, round1_source):
    delete_files.append({"file": live, "replaces": "staged/" + round1_source})


# ---- storage: worker writer scope, batch accounting, presence -----------------------------------------------------------------
S = B + "/📥️inputs/💾️storage/🟨️.mjs"
LEAVES = B + "/📥️inputs/💾️storage/🏊️leaves"
HOOK = "for(const name of outcome.accepted)recordBuildGraphOptionalMemberV1(scope.store.root,join(root,name));" if "function recordBuildGraphOptionalMemberV1(" in text(S) else ""
edit(S, 'import {openBuildGraphOptionalLedgerV1} from "./📒️ledger/🟨️.mjs";', 'import {openBuildGraphOptionalLedgerV1} from "./📒️ledger/🟨️.mjs";\nimport {createOptionalAccountingV1,createWorkerWriterV1} from "./🏊️leaves/🟨️.mjs";')
edit(S, "const stagings=new AsyncLocalStorage(),writers=new AsyncLocalStorage(),", "const workerWriters=new AsyncLocalStorage(),batches=new AsyncLocalStorage(),accountings=new WeakMap(),writers=new AsyncLocalStorage(),")
staging_block = span(S, "/** 🛰️ Runs a body whose graph fact directories are private staging directories", "return staged?join(staged.readRoot??staged.root,namespace):undefined;\n}\n")
edit(S, staging_block, '''/** 🛰️ Runs a body as one graph worker job: its fact directories are the shared graph directories the parent named and every optional leaf is written there directly, tmp then rename, within a byte grant; the parent accounts the leaves the writer reports and no generation is ever published from here. */
export function withBuildGraphWorkerWriterV1(spec,body){
 const writer=createWorkerWriterV1(spec);
 return workerWriters.run(writer,()=>body(writer));
}

/** 🧾️ Defers the refresh of a cargo resolution receipt to the end of the graph construction, where its parent produces every receipt once, in order and alone (a receipt is shared by every project of its closure); answers whether the body is a worker job or a batch that collects them. */
export function deferBuildGraphReceiptV1(manifest){
 const writer=workerWriters.getStore();if(writer!==undefined){writer.defer(manifest);return true;}
 const batch=batches.getStore();if(batch?.receipts!==undefined){batch.receipts.add(manifest);return true;}
 return false;
}

/** 🧮️ The accounting of optional leaves of one durable ledger, shared by every nested scope of the same graph writer. */
function optionalAccountingOfV1(scope){
 let accounting=accountings.get(scope.ledger);if(accounting===undefined){accounting=createOptionalAccountingV1({ledger:scope.ledger,quota:scope.optionalQuota});accountings.set(scope.ledger,accounting);}return accounting;
}

/** 🏁️ Pays the debt of deferred optional accounting to the durable ledger before it closes or measures the physical bytes. */
function flushBuildGraphOwedV1(ledger){
 const accounting=accountings.get(ledger);if(accounting!==undefined)accounting.flush(true);
}

/** 📦️ Runs a body whose optional leaves are written at once, admitted by the in-memory quota and owed to the durable ledger in batches instead of one durable transaction per leaf; `receipts` collects the cargo resolution manifests the body defers. */
export function withBuildGraphOptionalBatchV1(body,receipts){return batches.run({receipts},body);}

/** 🛰️ Accounts the optional leaves worker processes wrote straight into the graph directory under this writer's custody; a content addressed leaf is accounted once however many workers wrote it, a leaf the quota refuses is removed again unless this writer holds it as present, and every accepted leaf joins the member index of its group when that index exists. */
export function adoptBuildGraphWorkerLeavesV1(workspace,leaves){
 requireRepositoryBuildGraphWriterV1(workspace);const scope=writers.getStore(),root=repositoryBuildGraphDataDirectoryV1(workspace,"inference"),outcome=optionalAccountingOfV1(scope).adopt(leaves,name=>{const path=join(root,name);if(!scope.presence.has(path))rmSync(path,{force:true});});
 MEMBERHOOK
 return outcome;
}

/** 🎟️ Reserves the bytes one worker job may write, or answers undefined when the quota has none to spare; the returned function gives the reservation back. */
export function grantBuildGraphWorkerBytesV1(workspace,bytes){
 requireRepositoryBuildGraphWriterV1(workspace);return optionalAccountingOfV1(writers.getStore()).grant(bytes);
}

/** 🧊️ The blocks this graph writer (or worker job) has written or verified: under its custody a leaf cannot vanish, so presence is not probed again. */
export function buildGraphWriterPresenceV1(){return writers.getStore()?.presence??workerWriters.getStore()?.presence;}
'''.replace(" MEMBERHOOK\n", (" " + HOOK + "\n") if HOOK else ""))
edit(S, "const staged=stagings.getStore();if(staged){requireBuildGraphStagedPathV1(staged,path);write();return true;}", "const worker=workerWriters.getStore();if(worker)return worker.publish(path,plan,write);")
edit(S, "if(!scope.ledger.publish(plan,write))return decline();reservation.commit();", "if(!optionalAccountingOfV1(scope).publish(plan,write,batches.getStore()!==undefined,reservation))return decline();")
edit(S, 'Declared bounded graph fact namespace required");const staged=stagings.getStore();if(staged)return join(staged.root,namespace);', 'Declared bounded graph fact namespace required");const worker=workerWriters.getStore();if(worker)return worker.directory(namespace);')
edit(S, "export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){\n const staged=stagings.getStore();if(staged){const path=join(staged.root,namespace);mkdirSync(path,{recursive:true});return path;}", "export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){\n const worker=workerWriters.getStore();if(worker)return worker.directory(namespace);")
edit(S, "handles:new Set(),ledger,optionalQuota,namespace,phaseControl:actualControl};", "handles:new Set(),presence:new Set(),ledger,optionalQuota,namespace,phaseControl:actualControl};")
edit(S, "handles:new Set(),ledger,optionalQuota,phaseControl:control};", "handles:new Set(),presence:new Set(),ledger,optionalQuota,phaseControl:control};")
edit(S, "finally{try{ledger.close();}catch(error){if(!failed)throw error;}}", "finally{try{flushBuildGraphOwedV1(ledger);}catch(error){if(!failed)throw error;}try{ledger.close();}catch(error){if(!failed)throw error;}}")
edit(S, "try{ledger?.close();}catch(error){cleanupFailed=true;cleanupFailure=error;}", "try{if(ledger)flushBuildGraphOwedV1(ledger);}catch(error){cleanupFailed=true;cleanupFailure=error;}try{ledger?.close();}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}")
edit(S, "writer.ledger.retire(()=>rmSync(directory,{recursive:true}));", "flushBuildGraphOwedV1(writer.ledger);writer.presence.clear();writer.ledger.retire(()=>rmSync(directory,{recursive:true}));")

SD = B + "/📥️inputs/💾️storage/🟨️.d.mts"
d_block = span(SD, "/** 🛰️ Runs a body whose graph fact directories are private staging directories adopted later", "export function repositoryBuildGraphStagedReadDirectoryV1(namespace:string):string|undefined;\n")
edit(SD, d_block, '''/** 🛰️ Runs a body as one graph worker job whose optional leaves go straight into the shared graph directories within a byte grant. */
export function withBuildGraphWorkerWriterV1<T>(spec:Readonly<{roots:Readonly<Record<string,string>>;grantBytes:number}>,body:(writer:Readonly<{report():Readonly<{leaves:{name:string;bytes:number}[];declined:string[];receipts:string[]}>}>)=>Promise<T>|T):Promise<T>|T;
/** 🧾️ Defers the refresh of a cargo resolution receipt to the end of the graph construction, and answers whether it was deferred. */
export function deferBuildGraphReceiptV1(manifest:string):boolean;
/** 📦️ Runs a body whose optional leaves are owed to the durable ledger in batches and which may defer cargo resolution receipts into the given set. */
export function withBuildGraphOptionalBatchV1<T>(body:()=>Promise<T>|T,receipts?:Set<string>):Promise<T>|T;
/** 🛰️ Accounts the optional leaves worker processes wrote into the graph directory. */
export function adoptBuildGraphWorkerLeavesV1(workspace:string,leaves:readonly Readonly<{name:string;bytes:number}>[]):Readonly<{accepted:string[];refused:string[]}>;
/** 🎟️ Reserves the bytes of one worker job, or answers undefined; the function gives them back. */
export function grantBuildGraphWorkerBytesV1(workspace:string,bytes:number):(()=>void)|undefined;
/** 🧊️ The set of block paths the live graph writer has written or verified. */
export function buildGraphWriterPresenceV1():Set<string>|undefined;
''')

# ---- inputs: worker roots, deferred receipts; the staged adoption is gone ----------------------------------------------------
I = B + "/📥️inputs/🟨️.mjs"
edit(I, 'import {repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1} from "./💾️storage/🟨️.mjs";', 'import {deferBuildGraphReceiptV1, repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1, withBuildGraphOptionalBatchV1} from "./💾️storage/🟨️.mjs";')
adopt_block = span(I, "/** 🛰️ Publishes the files a worker process staged under this process's graph writer custody", '  }else throw Error("Declared graph staging namespace required: "+name);\n }\n}\n')
edit(I, adopt_block, '''/** 🛰️ The graph directories a worker job writes into: the very directories this process publishes to, named once so the workers never resolve a store of their own. */
export function graphWorkerRootsV1(workspaceRoot){
 return {inference:repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"inference"),"build-input-closures":repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures"),"build-lock-closures":repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")};
}
''')
edit(I, "async function warmCargoResolutionV1(workspaceRoot, scripts, manifest, context) {\n  try {", '''async function warmCargoResolutionV1(workspaceRoot, scripts, manifest, context) {
  if (deferBuildGraphReceiptV1(manifest)) return;
  return warmCargoResolutionNowV1(workspaceRoot, scripts, manifest, context);
}

/** 🦀️ Refreshes the resolution receipts of the crates a graph construction deferred, in order and one after the other through one resolution run, so every receipt has exactly one writer and the receipts of a graph are the same whichever process inferred its projects. */
export async function refreshCargoReceiptsV1(workspaceRoot, manifests, context) {
  const key = {};
  await withBuildGraphOptionalBatchV1(async () => { for (const manifest of [...manifests].sort()) await warmCargoResolutionNowV1(workspaceRoot, key, manifest, context); });
}

/** 🦀️ Refreshes the resolution receipt of one crate at once. */
export async function warmCargoResolutionNowV1(workspaceRoot, scripts, manifest, context) {
  try {''')
ID = B + "/📥️inputs/🟨️.d.mts"
edit(ID, "/** 🛰️ Publishes files a worker process staged, under the writer custody of this process. */\nexport function adoptStagedGraphFilesV1(workspaceRoot:string,files:readonly Readonly<{name:string;bytes:Uint8Array}>[]):void;", '''/** 🛰️ The graph directories a worker job writes into. */
export function graphWorkerRootsV1(workspaceRoot:string):Readonly<Record<"inference"|"build-input-closures"|"build-lock-closures",string>>;
/** 🦀️ Refreshes the resolution receipt of one crate at once. */
export function warmCargoResolutionNowV1(workspaceRoot:string,scripts:unknown,manifest:string,context:unknown):Promise<void>;
/** 🦀️ Refreshes the resolution receipts of the deferred crates in order. */
export function refreshCargoReceiptsV1(workspaceRoot:string,manifests:Iterable<string>,context:unknown):Promise<void>;''')

edit(I, "    await cargoResolutionPartsV1(run, manifest, { tolerant: true });\n    await flushCargoReceiptsV1(run);", "    try { await cargoResolutionPartsV1(run, manifest, { tolerant: true }); } finally { await flushCargoReceiptsV1(run); }")
RC = B + "/📥️inputs/🦀️resolution/💾️receipt/🟨️.mjs"
edit(RC, 'brotliCompressSync(Buffer.from(JSON.stringify(sealed),"utf8"),', 'brotliCompressSync(Buffer.from(canonicalJsonV1(sealed),"utf8"),')

RS = B + "/📥️inputs/🦀️resolution/🟦️.ts"
edit(RS, "  if (context.store?.write) for (const crate of [...context.dirty].sort(bytes)) { await context.store.write((await context.leaves.get(crate))!); written++; }\n  context.dirty.clear();", "  const crates = [...context.dirty].sort(bytes);\n  context.dirty.clear();\n  if (context.store?.write) for (const crate of crates) { await context.store.write((await context.leaves.get(crate))!); written++; }")

# ---- project assembly: receipts are read from the one graph directory again ---------------------------------------------------
A = INF + "/🧱️project-assembly/🟨️.mjs"
edit(A, 'import { repositoryBuildGraphStagedReadDirectoryV1, withBuildGraphOptionalPublicationV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";', 'import { withBuildGraphOptionalPublicationV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";')
read_block = span(A, 'read: (key) => { const name = `${createHash("sha256")', 'return undefined; },')
edit(A, read_block, 'read: (key) => { try { return brotliDecompressSync(rawReadFileSync(leaf(key))).toString("utf8"); } catch { return undefined; } },')

# ---- observation: blocks written by workers, presence under custody ------------------------------------------------------------
O = B + "/👁️observation/🟨️.mjs"
edit(O, "export function exportSourceAnalysisRecordsV1(){\n return [...sourceAnalysisImages].map(([key,row])=>[key,Buffer.from(JSON.stringify(row.result))]);\n}", """export function exportSourceAnalysisRecordsV1(maximumBytes=Infinity){
 const out=[];let total=0;
 for(const[key,row]of sourceAnalysisImages){const bytes=Buffer.from(JSON.stringify(row.result));if(total+bytes.byteLength>maximumBytes)break;total+=bytes.byteLength;out.push([key,bytes]);}
 return out;
}""")
edit(O, "/** 🛰️ Answers the parser records of every analysis this process still holds in memory, so a worker process starts with what this process already parsed instead of parsing it again. */", "/** 🛰️ Answers the parser records of the analyses this process still holds in memory, as many as fit in `maximumBytes`, so a worker process starts with what this process already parsed instead of parsing it again. */")
edit(O, "const sealing={digest},result", "const sealing={digest,directory:dirname(cachePath)},result")
edit(O, "export function currentProjectRecordDigestV1(){return recordScopes.getStore()?.sealing?.digest;}", '''export function currentProjectRecordDigestV1(){return recordScopes.getStore()?.sealing?.digest;}

/** 📂️ The directory the record of the project under observation will be written to, which names where a worker process writes the blocks it builds. */
export function currentProjectRecordDirectoryV1(){return recordScopes.getStore()?.sealing?.directory;}''')
old_persist = span(O, "/** 📦️ Persists the blocks a sealed record names: bytes the worker sent are written", "return true;\n}\n")
edit(O, old_persist, '''/** 📦️ Checks the blocks a sealed record names: a worker process wrote the ones this process lacked straight into the graph directory and the writer custody accounted them, so a block this writer has written or verified is not probed again, any other block is checked once, and a block that is not there makes the seal unusable. */
function persistSealedBlocksV1(directory,sealed){
 for(const digest of sealed.blockDigests)if(!persistInferenceBlockV1(directory,{digest,only:true}))return false;
 return true;
}

/** 📦️ Writes the blocks of a sealed record this process built straight into the graph directory, content addressed and atomic (tmp then rename), skipping the blocks that are already there, also the ones another writer finished between the check and the rename, which are never replaced or removed; answers the digests it created, or undefined when a block was declined by the grant of the worker writer. */
export function writeSealedBlocksV1(directory,sealed){
 const created=[];
 for(const block of sealed.blocks){
  const path=inferenceObservationBlockPathV1(directory,block.digest);
  if(fs.existsSync(path))continue;
  const temporary=path+"."+process.pid+"."+Date.now()+".tmp";
  if(!withBuildGraphOptionalPublicationV1(path,{peakBytes:block.packed.byteLength,retainedDeltaBytes:block.packed.byteLength},()=>{fs.mkdirSync(dirname(path),{recursive:true});fs.writeFileSync(temporary,block.packed,{flag:"wx"});let present=false;try{present=fs.statSync(path).size===block.packed.byteLength;}catch{}if(present){fs.rmSync(temporary,{force:true});return false;}fs.renameSync(temporary,path);}))return undefined;
  created.push(block.digest);
 }
 return created;
}
''')
edit(O, "const path=inferenceObservationBlockPathV1(directory,known.digest),present=verifiedBlockV1(path)?{absent:false,recheck:()=>{}}:readObservationBlockV1(directory,known.digest,true);if(present&&!present.absent){present.recheck();return true;}", "const path=inferenceObservationBlockPathV1(directory,known.digest),presence=buildGraphWriterPresenceV1();if(presence!==undefined&&presence.has(path))return true;const present=verifiedBlockV1(path)?{absent:false,recheck:()=>{}}:readObservationBlockV1(directory,known.digest,true);if(present&&!present.absent){present.recheck();presence?.add(path);return true;}")
edit(O, "const published=readObservationBlockV1(directory,known.digest);published.recheck();return true;", "const published=readObservationBlockV1(directory,known.digest);published.recheck();presence?.add(path);return true;")
edit(O, 'import {withBuildGraphOptionalPublicationV1,registerBuildGraphOptionalHandleV1,requireRepositoryBuildGraphObservationPathV1} from "../📥️inputs/💾️storage/🟨️.mjs";', 'import {buildGraphWriterPresenceV1,withBuildGraphOptionalPublicationV1,registerBuildGraphOptionalHandleV1,requireRepositoryBuildGraphObservationPathV1} from "../📥️inputs/💾️storage/🟨️.mjs";')
OD = B + "/👁️observation/🟨️.d.mts"
edit(OD, "export function exportSourceAnalysisRecordsV1():[string,Buffer][];", "export function exportSourceAnalysisRecordsV1(maximumBytes?:number):[string,Buffer][];")
edit(OD, "export function replayDetachedProjectInferenceV1(rows:readonly SourceObservationV1[],records:readonly (readonly [string,Uint8Array])[],sealed?:SealedInferenceRecordV1):void;", "export function replayDetachedProjectInferenceV1(rows:readonly SourceObservationV1[],records:readonly (readonly [string,Uint8Array])[],sealed?:Omit<SealedInferenceRecordV1,\"blocks\">):void;")
edit(OD, "/** 🪞️ Marks the JSON text of a project value computed elsewhere. */", '''/** 📂️ The directory the record of the project under observation will be written to. */
export function currentProjectRecordDirectoryV1():string|undefined;
/** 📦️ Writes the blocks of a sealed record straight into the graph directory and answers the digests it created, or undefined when a block was declined. */
export function writeSealedBlocksV1(directory:string,sealed:Pick<SealedInferenceRecordV1,"blocks">):string[]|undefined;
/** 🪞️ Marks the JSON text of a project value computed elsewhere. */''')

# ---- project nodes: batched accounting, the farm joins the pool -------------------------------------------------------------------
N = NODES + "/🟨️.mjs"
edit(N, 'import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";', 'import { withBuildGraphOptionalBatchV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";\nimport { openProjectFarmV1, startProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";')
edit(N, "  const inferProject=async({result,name,json})=>{", "  const receipts=new Set();\n  const inferProject=({result,name,json})=>withBuildGraphOptionalBatchV1(async()=>{")
edit(N, "    result.projects[name].targets={...inactive,...result.projects[name].targets};\n  };", "    result.projects[name].targets={...inactive,...result.projects[name].targets};\n  },receipts);")
edit(N, ".finally(()=>farm?.close());", ".finally(()=>farm?.close());\n  await refreshCargoReceiptsV1(workspaceRoot,receipts,context);")
edit(N, 'import { withBuildGraphOptionalBatchV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";', 'import { withBuildGraphOptionalBatchV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";\nimport { refreshCargoReceiptsV1 } from "../../../../🏗️build/📥️inputs/🟨️.mjs";')

edit(N, "  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });\n  const catalogContributions=", "  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });\n  const started=options?.ownerAdmission?undefined:startProjectFarmV1({workspaceRoot,projects:configFiles.length,readRoot:workspaceDataDirectory(workspaceRoot),signal:context.invocation?.control.signal});\n  const catalogContributions=")
edit(N, "componentRoots,readRoot:workspaceDataDirectory(workspaceRoot)});", "componentRoots,readRoot:workspaceDataDirectory(workspaceRoot),started});")

# ---- revision: the leaves module joins the implementation identity -------------------------------------------------------------------
R = INF + "/♻️revision/🟨️.mjs"
edit(R, 'return ["🏗️build/🏊️farm/🟨️.mjs",', 'return ["🏗️build/📥️inputs/💾️storage/🏊️leaves/🟨️.mjs", "🏗️build/📥️inputs/💾️storage/🏊️leaves/🧬️schema/🔣️.json", "🏗️build/🏊️farm/🧰️pool/🟨️.mjs", "🏗️build/🏊️farm/🟨️.mjs",')

# ---- new and replaced files --------------------------------------------------------------------------------------------------------
new_file(LEAVES + "/🟨️.mjs", "new2/leaves/leaves.mjs")
new_file(LEAVES + "/🟨️.d.mts", "new2/leaves/leaves.d.mts")
new_file(LEAVES + "/🧬️schema/🔣️.json", "new2/leaves/schema.json")
new_file(LEAVES + "/🧪️tests/🟦️.ts", "new2/leaves/tests/law.ts")
new_file(LEAVES + "/🧪️tests/🔣️.json", "new2/leaves/tests/fixture.json")
new_file(LEAVES + "/🧪️tests/🐍️oracle.py", "new2/leaves/tests/oracle.py")
FARM = B + "/🏊️farm"
new_file(FARM + "/🧰️pool/🟨️.mjs", "new2/pool/pool.mjs")
new_file(FARM + "/🧰️pool/🟨️.d.mts", "new2/pool/pool.d.mts")
for live, source, before in [
    (FARM + "/🟨️.mjs", "new2/farm/engine.mjs", "new/farm/engine.mjs"),
    (FARM + "/🟨️.d.mts", "new2/farm/engine.d.mts", "new/farm/engine.d.mts"),
    (FARM + "/🧬️schema/🔣️.json", "new2/farm/schema.json", "new/farm/schema.json"),
    (FARM + "/👷️worker/🟨️.mjs", "new2/farm/worker.mjs", "new/farm/worker.mjs"),
    (NODES + "/🏊️project-farm/🟨️.mjs", "new2/project-farm/glue.mjs", "new/project-farm/glue.mjs"),
    (NODES + "/🏊️project-farm/👷️service/🟨️.mjs", "new2/project-farm/service.mjs", "new/project-farm/service.mjs"),
    (INF + "/🕸️dependency-edges/🏊️module-farm/🟨️.mjs", "new2/module-farm/glue.mjs", "new/module-farm/glue.mjs"),
    (INF + "/🕸️dependency-edges/🏊️module-farm/👷️service/🟨️.mjs", "new2/module-farm/service.mjs", "new/module-farm/service.mjs"),
]:
    replace_file(live, source, before)
delete_file(B + "/📥️inputs/💾️storage/🧪️tests/🛰️staging/🟦️.ts", "new/tests/staging.ts")
tests2 = HERE / "staged/new-tests2.json"
if tests2.exists():
    for item in json.loads(tests2.read_text(encoding="utf8")):
        if "replaces" in item:
            replace_file(item["file"], item["from"], item["replaces"])
        else:
            new_file(item["file"], item["from"])
spec = {"version": 1, "requires": "ops", "edits": edits, "newFiles": new_files, "replaceFiles": replace_files, "deleteFiles": delete_files}
(HERE / "staged/round2.json").write_text(json.dumps(spec, ensure_ascii=False, indent=1), encoding="utf8")
print("round 2: edits", len(edits), "new files", len(new_files), "replaced", len(replace_files), "deleted", len(delete_files), "root", root)
