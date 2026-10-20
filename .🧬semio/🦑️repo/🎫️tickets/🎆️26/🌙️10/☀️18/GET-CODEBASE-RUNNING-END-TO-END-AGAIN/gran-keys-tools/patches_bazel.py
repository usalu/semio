# 🪝 Anchored edits of the Bazel adapter: a lock marker declares only its crate manifest and stays in the task inputs, and the action fingerprint folds the crate's resolution identity (the digest nx hashes) instead of the nearest Cargo.lock file.
from patchlib import chain

def closure(text):
    lock_old = text[text.index("/** 🔒️ Finds the workspace lock"):] if "/** 🔒️ Finds the workspace lock" in text else None
    edits = [
        ("closure-fs", 'import {existsSync} from "node:fs";\n', ''),
        ("closure-path", 'import {join,posix} from "node:path";', 'import {join} from "node:path";'),
        ("closure-parse-doc", "a lock projection stands for its manifest and the workspace lock that covers it. */", "a lock projection stands for its crate manifest only: the locked resolution reaches the action key as the crate's resolution identity, not as the whole Cargo.lock file. */"),
        ("closure-parse-sig", "export function parseBazelClosureManifestV1(kind:BazelClosureKindV1,value:unknown,lockFiles:(manifest:string)=>readonly string[]):readonly string[]{", "export function parseBazelClosureManifestV1(kind:BazelClosureKindV1,value:unknown):readonly string[]{"),
        ("closure-parse-names", "[(value as {manifest:string}).manifest,...lockFiles((value as {manifest:string}).manifest)]", "[(value as {manifest:string}).manifest]"),
        ("closure-load-sig", "export async function loadBazelClosuresV1(markers:readonly BazelClosureMarkerV1[],fetch:BazelClosureFetchV1,lockFiles:(manifest:string)=>readonly string[]):Promise<BazelClosuresV1>{", "export async function loadBazelClosuresV1(markers:readonly BazelClosureMarkerV1[],fetch:BazelClosureFetchV1):Promise<BazelClosuresV1>{"),
        ("closure-load-call", "parseBazelClosureManifestV1(kind,value,lockFiles)", "parseBazelClosureManifestV1(kind,value)"),
    ]
    text, changed = chain(text, edits)
    start = text.find("\n/** 🔒️ Finds the workspace lock")
    if start >= 0:
        end = text.index("\n}\n", text.index("export function workspaceBazelLockFilesV1")) + 3
        text, changed = text[:start] + text[end:], True
    return text, changed

def closure_schema(text):
    return chain(text, [("closure-schema-lock", '"markerPattern": "^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$",\n        "lockFile": "Cargo.lock"', '"markerPattern": "^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$"')])

def script(text):
    return chain(text, [
        ("script-import", "loadBazelClosuresV1,managedBazelClosureFetchV1,workspaceBazelLockFilesV1,type BazelClosureFetchV1,type BazelClosuresV1}", "loadBazelClosuresV1,managedBazelClosureFetchV1,type BazelClosureFetchV1,type BazelClosureMarkerV1,type BazelClosuresV1}"),
        ("script-lock-import", 'import {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1', 'import {selectedCargoResolutionDigests} from "../../📥️inputs/🔒️lock/📥️read/🟨️.mjs";\nimport {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1'),
        ("script-marker", "return[...exactClosures.files(closure)];}", 'return closure.kind==="lock"?[...exactClosures.files(closure),input]:[...exactClosures.files(closure)];}'),
        ("script-fingerprint-sig", "implementation:string,exactClosures:BazelClosuresV1):Promise<string>{", "implementation:string,exactClosures:BazelClosuresV1,resolutions:ReadonlyMap<string,string>):Promise<string>{"),
        ("script-fingerprint-env", 'else if(typeof row.env==="string")digest.update(JSON.stringify({env:row.env,value:original[row.env]??null}));}', 'else if(typeof row.env==="string"){const lock=bazelClosureMarkerV1(row);if(lock?.kind==="lock"){const resolution=resolutions.get(lock.key);if(resolution===undefined)throw new Error("Bazel Cargo resolution identity required: "+lock.key);digest.update(JSON.stringify({resolution}));}else digest.update(JSON.stringify({env:row.env,value:original[row.env]??null}));}}'),
        ("script-load", "exactClosures=await loadBazelClosuresV1(bazelClosureMarkersForPlanV1(graph,Object.values(plan.tasks)),closureFetch??managedBazelClosureFetchV1(root,phaseControl),workspaceBazelLockFilesV1(root)),", "markers=bazelClosureMarkersForPlanV1(graph,Object.values(plan.tasks)),exactClosures=await loadBazelClosuresV1(markers,closureFetch??managedBazelClosureFetchV1(root,phaseControl)),resolutions=await bazelCargoResolutionsV1(root,markers,exactClosures,phaseControl),"),
        ("script-call", "implementation.sha256+implementation.loaderSha256,exactClosures);", "implementation.sha256+implementation.loaderSha256,exactClosures,resolutions);"),
        ("script-helper", "/** 🧮️ Fingerprints only what Bazel cannot read from declared files:", '''/** 🦀️ The resolution identity of every crate a plan locks, keyed by its closure marker: the locked dependency slice, what the crate and the crates it compiles with inherit from their workspaces, the pinned toolchain and the Cargo configuration; the same digest the nx hasher computes, read once under one batch. */
export async function bazelCargoResolutionsV1(root:string,markers:readonly BazelClosureMarkerV1[],closures:BazelClosuresV1,control:ResourceLeaseControl):Promise<ReadonlyMap<string,string>>{
 const locks=markers.filter(row=>row.kind==="lock"),manifests=[...new Set(locks.map(row=>closures.files(row)[0]!))];
 if(!manifests.length)return new Map();
 const digests=await selectedCargoResolutionDigests(root,manifests,control);
 return new Map(locks.map(row=>[row.key,digests.get(closures.files(row)[0]!)!]));
}

/** 🧮️ Fingerprints only what Bazel cannot read from declared files:'''),
    ])

CL_TEST_OLD = 'closure.filesystemBazelClosureFetchV1(store(root)),closure.workspaceBazelLockFilesV1(workspace(root)));'
def closure_test(text):
    return chain(text, [
        ("test-load", "const load=async(root:string,rows:readonly {project:string;inputs:unknown[]}[]=corpus.cases)=>closure.loadBazelClosuresV1(closure.bazelClosureMarkersForPlanV1({...graph,dependencies:graph.dependencies},plan(rows)),closure.filesystemBazelClosureFetchV1(store(root)),closure.workspaceBazelLockFilesV1(workspace(root)));", "const load=async(root:string,rows:readonly {project:string;inputs:unknown[]}[]=corpus.cases)=>(workspace(root),closure.loadBazelClosuresV1(closure.bazelClosureMarkersForPlanV1({...graph,dependencies:graph.dependencies},plan(rows)),closure.filesystemBazelClosureFetchV1(store(root))));"),
        ("test-objects", "expect(got.objects,row.name).toEqual(row.passthrough);\n   expect(inputs.some(value=>closure.bazelClosureMarkerV1(value)),row.name).toBe(false);", "expect(got.objects.filter(value=>!closure.bazelClosureMarkerV1(value)),row.name).toEqual(row.passthrough);\n   expect(inputs.some(value=>closure.bazelClosureMarkerV1(value)?.kind===\"source\"),row.name).toBe(false);\n   expect(inputs.filter(value=>closure.bazelClosureMarkerV1(value)?.kind===\"lock\").length,row.name).toBe(row.expanded.includes(\"crate/Cargo.toml\")?1:0);\n   expect(got.strings,row.name+\": no lock file is declared\").not.toContain(\"Cargo.lock\");"),
        ("test-load-source", "closure.loadBazelClosuresV1([{kind:\"source\",key:\"0\".repeat(64)}],closure.filesystemBazelClosureFetchV1(directory),()=>[])", "closure.loadBazelClosuresV1([{kind:\"source\",key:\"0\".repeat(64)}],closure.filesystemBazelClosureFetchV1(directory))"),
        ("test-load-missing", "closure.loadBazelClosuresV1([{kind:\"source\",key:\"1\".repeat(64)}],async()=>new Map(),()=>[])", "closure.loadBazelClosuresV1([{kind:\"source\",key:\"1\".repeat(64)}],async()=>new Map())"),
        ("test-refusal", "closure.parseBazelClosureManifestV1(refusal.kind as \"source\"|\"lock\",refusal.value,()=>[])", "closure.parseBazelClosureManifestV1(refusal.kind as \"source\"|\"lock\",refusal.value)"),
        ("test-hole", 'expect(after).toContain("Cargo.lock");', 'expect(after).not.toContain("Cargo.lock");'),
    ])

def closure_corpus_text(text):
    return chain(text, [("corpus-lock", '        "tools/unrelated.ts",\n        "Cargo.lock",\n        "crate/Cargo.toml"\n', '        "tools/unrelated.ts",\n        "crate/Cargo.toml"\n')])

FP_BIND_OLD = "bindings={createHash,randomUUID,bazelCargoClassV1:()=>undefined,"
FP_BIND_NEW = "bindings={createHash,randomUUID,bazelClosureMarkerV1:()=>undefined,bazelCargoClassV1:()=>undefined,"
FP_TAIL = '''

test("Bazel folds the crate resolution identity into the fingerprint, never a lock file, and refuses a lock marker without one",async()=>{
 const root=process.cwd(),key="a".repeat(64),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"}),task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"bun ./📜️script.ts",env:{}},inputs:[{env:"SEMIO_CARGO_LOCK_"+key}]}};
 const fingerprint=(resolutions:Map<string,string>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:fingerprint",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  const bindings={createHash,randomUUID,bazelClosureMarkerV1:(row:{env:string})=>row.env.startsWith("SEMIO_CARGO_LOCK_")?{kind:"lock",key:row.env.slice("SEMIO_CARGO_LOCK_".length)}:undefined,bazelCargoClassV1:()=>undefined,bazelCargoToolchainDigestV1:async()=>"",bazelGoClassV1:()=>false,bazelGoToolchainDigestV1:async()=>"",process:{...process,env:{...process.env}},bazelTaskInputsV1:(_graph:unknown,selected:typeof task)=>selected.definition.inputs,issueBuildTaskEnvironmentV1,buildExecEnvironmentV1,resolve,checkScriptInvocation,captureOwnedCommandV1:async()=>""},run=Function(...Object.keys(bindings),fingerprintCode())(...Object.values(bindings));
  return run(root,{},task,[],new Map(),original,createIssuedResourceControlV1(original,"bazel:fingerprint",0),true,"implementation",undefined,resolutions);
 });
 const first=await fingerprint(new Map([[key,"1".repeat(64)]])),same=await fingerprint(new Map([[key,"1".repeat(64)]])),moved=await fingerprint(new Map([[key,"2".repeat(64)]]));
 expect(same).toBe(first);expect(moved).not.toBe(first);
 await expect(fingerprint(new Map())).rejects.toThrow("Bazel Cargo resolution identity required");
},30000);
'''
def fingerprint_test(text):
    changed = False
    if FP_BIND_NEW not in text:
        if text.count(FP_BIND_OLD) != 2: raise SystemExit("anchor fp-bind: %d occurrence(s), expected 2" % text.count(FP_BIND_OLD))
        text, changed = text.replace(FP_BIND_OLD, FP_BIND_NEW), True
    if FP_TAIL.strip()[:60] in text: return text, changed
    return text.rstrip("\n") + "\n" + FP_TAIL, True
