#!/usr/bin/env python3
"""Builds staged/ops.json for the graph farm set from the CURRENT text of the target tree (anchors must occur exactly once).
usage: ops_gen.py [--root REPO_ROOT]   (default: the repository this file lives in)"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
root = HERE
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
if "--root" in sys.argv:
    root = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]).resolve()
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
G = B + "/🕸️graph"
INF = G + "/📥️inference"
NODES = INF + "/📦️project-nodes"
edits = []


def text(file):
    return (root / file).read_text(encoding="utf8")


def edit(file, anchor, replacement):
    count = text(file).count(anchor)
    if count != 1:
        raise SystemExit("anchor occurs %d times in %s: %r" % (count, file[-60:], anchor[:90]))
    edits.append({"file": file, "anchor": anchor, "replacement": replacement})


def span(file, start, end):
    t = text(file)
    i = t.index(start)
    j = t.index(end, i) + len(end)
    return t[i:j]


# ---- storage: private staging directories for a worker process ----------------------------------------------------------
S = B + "/📥️inputs/💾️storage/🟨️.mjs"
edit(S, "const writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),", "const stagings=new AsyncLocalStorage(),writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),")
edit(S, "export function withBuildGraphOptionalPublicationV1(path,plan,write){\n const scope=writers.getStore();", "export function withBuildGraphOptionalPublicationV1(path,plan,write){\n const staged=stagings.getStore();if(staged){requireBuildGraphStagedPathV1(staged,path);write();return true;}\n const scope=writers.getStore();")
edit(S, "export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){\n requireRepositoryBuildGraphWriterV1(workspace);", "export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){\n const staged=stagings.getStore();if(staged){const path=join(staged.root,namespace);mkdirSync(path,{recursive:true});return path;}\n requireRepositoryBuildGraphWriterV1(workspace);")
edit(S, 'export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){\n if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");', 'export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){\n if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");const staged=stagings.getStore();if(staged)return join(staged.root,namespace);')
edit(S, "/** 📏️ Observes bounded quota refusal DATA without granting publication capacity. */", '''/** 🛰️ Runs a body whose graph fact directories are private staging directories: a worker process publishes nothing into the graph backing, its parent adopts the files it staged under the original writer custody. */
export function withBuildGraphStagingV1(staging,body){
 if(typeof staging?.root!=="string"||!isAbsolute(staging.root))throw Error("Absolute private graph staging root required");
 return stagings.run(Object.freeze({root:resolve(staging.root),readRoot:staging.readRoot}),body);
}

/** 🧷️ Admits only paths inside the private staging root. */
function requireBuildGraphStagedPathV1(staged,path){
 const local=relative(staged.root,resolve(path));if(local===""||local.startsWith("..")||isAbsolute(local))throw Error("Private graph staging path required");
}

/** 📖️ The directory a staged body reads published optional leaves from: the graph data directory its parent named, else its private staging directory. */
export function repositoryBuildGraphStagedReadDirectoryV1(namespace){
 const staged=stagings.getStore();return staged?join(staged.readRoot??staged.root,namespace):undefined;
}

/** 📏️ Observes bounded quota refusal DATA without granting publication capacity. */''')
SD = B + "/📥️inputs/💾️storage/🟨️.d.mts"
edit(SD, "export function withBuildGraphOptionalPublicationV1(", '''/** 🛰️ Runs a body whose graph fact directories are private staging directories adopted later by the parent that holds the writer custody. */
export function withBuildGraphStagingV1<T>(staging:Readonly<{root:string;readRoot?:string}>,body:()=>Promise<T>|T):Promise<T>|T;
/** 📖️ The directory a staged body reads published optional leaves from, or undefined outside a staged body. */
export function repositoryBuildGraphStagedReadDirectoryV1(namespace:string):string|undefined;
export function withBuildGraphOptionalPublicationV1(''')

# ---- inputs: adopt what a worker staged -----------------------------------------------------------------------------------
I = B + "/📥️inputs/🟨️.mjs"
edit(I, "/** 📍️ Projects the project-root sentinel into a repository-relative Cargo manifest. */", '''/** 🛰️ Publishes the files a worker process staged under this process's graph writer custody: closure inventories are content addressed and mandatory, every other file is an optional cache leaf of the inference namespace. */
export function adoptStagedGraphFilesV1(workspaceRoot,files){
 for(const{name,bytes}of files){
  const [namespace,...rest]=name.split("/"),leaf=rest.join("/");
  if(!leaf||rest.some(part=>part===""||part===".."))throw Error("Relative staged graph file required: "+name);
  if(namespace==="build-input-closures"||namespace==="build-lock-closures"){
   const contract=Buffer.from(bytes).toString("utf8");
   if(!/^[a-f0-9]{64}\\.json$/u.test(leaf)||createHash("sha256").update(contract).digest("hex")!==leaf.slice(0,-5))throw Error("Closure content differs from its name: "+name);
   publishInputClosureV1(join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot,namespace),leaf),contract);
  }else if(namespace==="inference"){
   const target=join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"inference"),leaf),data=Buffer.from(bytes);let previous=0;try{previous=lstatSync(target).size;}catch{}
   withBuildGraphOptionalPublicationV1(target,{peakBytes:data.byteLength,retainedDeltaBytes:Math.min(data.byteLength,data.byteLength-previous)},()=>{const temporary=`${target}.${process.pid}.${Date.now()}.tmp`;mkdirSync(dirname(target),{recursive:true});writeFileSync(temporary,data,{flag:"wx"});renameSync(temporary,target);});
  }else throw Error("Declared graph staging namespace required: "+name);
 }
}

/** 📍️ Projects the project-root sentinel into a repository-relative Cargo manifest. */''')
ID = B + "/📥️inputs/🟨️.d.mts"
edit(ID, "/** 📍️ Projects the project-root sentinel to its exact Cargo manifest coordinate. */", '''/** 🛰️ Publishes files a worker process staged, under the writer custody of this process. */
export function adoptStagedGraphFilesV1(workspaceRoot:string,files:readonly Readonly<{name:string;bytes:Uint8Array}>[]):void;
/** 📍️ Projects the project-root sentinel to its exact Cargo manifest coordinate. */''')

# ---- project assembly: read published input precision receipts besides the staged ones --------------------------------------
A = INF + "/🧱️project-assembly/🟨️.mjs"
edit(A, 'import { withBuildGraphOptionalPublicationV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";', 'import { repositoryBuildGraphStagedReadDirectoryV1, withBuildGraphOptionalPublicationV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";')
edit(A, 'read: (key) => { try { return brotliDecompressSync(rawReadFileSync(leaf(key))).toString("utf8"); } catch { return undefined; } },', 'read: (key) => { const name = `${createHash("sha256").update(key).digest("hex").slice(0, 24)}.json`, shared = repositoryBuildGraphStagedReadDirectoryV1("inference"); for (const path of [leaf(key), shared === undefined ? undefined : join(shared, "ip", name)]) if (path !== undefined) { try { return brotliDecompressSync(rawReadFileSync(path)).toString("utf8"); } catch {} } return undefined; },')

# ---- observation: detached inference and replay ---------------------------------------------------------------------------
O = B + "/👁️observation/🟨️.mjs"
edit(O, "/** 📊️ Reports owned observation counts without exposing or discarding original source facts. */", '''/** 🛰️ Computes one project's value and the facts it consumed without persisting anything, for a worker process whose parent persists the result under its own custody; `validation` is shared by the projects of one worker so a fact many projects consumed is probed once. */
export async function observeDetachedProjectInferenceV1({validation,records,hints=new Map()},compute){
 requireCurrentObservationV1();loadSealed=true;
 const observations=new Map(),started=Date.now();
 const value=await inferenceValidationScopes.run(validation,()=>recordScopes.run({records,hints},()=>scopes.run(observations,compute)));
 requireCurrentObservationV1();
 const image=jsonImage(value);if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");
 return {value:JSON.parse(image),image,rows:[...observations.values()],started};
}

/** 🛰️ Replays what a worker process consumed into the project scope that awaits it, exactly as if this process had read it: the facts join the scope and its capture, the parser records join the project's record store. */
export function replayDetachedProjectInferenceV1(rows,records,sealed){
 retainObservationsV1(scopes.getStore(),rows);retainSourceFactsV1(rows);
 const scope=recordScopes.getStore();
 if(scope?.records)for(const[key,bytes]of records)scope.records.put(key,bytes);
 if(sealed!==undefined&&scope?.sealing)scope.sealing.sealed=sealed;
}

/** 🪞️ Marks the JSON text of a project value computed elsewhere, so the observing process parses it once instead of walking, printing and parsing the value again. */
const PREIMAGED_PROJECT=Symbol("preimaged project");
export function preimagedProjectValueV1(image){return {[PREIMAGED_PROJECT]:image};}

/** 🔖️ The digest the record of the project under observation will carry, which a worker process needs to seal that record. */
export function currentProjectRecordDigestV1(){return recordScopes.getStore()?.sealing?.digest;}

/** 📦️ Persists the blocks a sealed record names: bytes the worker sent are written, blocks this process already holds are checked, and a block it cannot hold makes the seal unusable. */
function persistSealedBlocksV1(directory,sealed){
 const sent=new Map(sealed.blocks.map(block=>[block.digest,block]));
 for(const digest of sealed.blockDigests)if(!persistInferenceBlockV1(directory,sent.get(digest)??{digest,only:true}))return false;
 return true;
}

/** 🛰️ Seals the record of a project a worker process computed: the exact bytes the observing process would write (the header naming the shared blocks, compressed together with the value) and the compressed bytes of every block that process does not hold yet; the observing process writes them without walking, printing or compressing anything. */
const sealedBlockDigests=new Set();
export function sealInferenceRecordV1({digest,image,rows,known,emitted=sealedBlockDigests}){
 const collected=[],header=inferenceHeaderTextV1(digest,createHash("sha256").update(image).digest("hex"),[],rows,"",(_directory,block)=>{let identity=blocksByIdentity.get(block.key);if(identity===undefined){identity={digest:createHash("sha256").update(observationRowsJsonV1(block.rows)).digest("hex")};blocksByIdentity.set(block.key,identity);}block.digest=identity.digest;collected.push(block);return true;}),blocks=[];
 for(const block of collected)if(!known.has(block.digest)&&!emitted.has(block.digest)){emitted.add(block.digest);blocks.push({digest:block.digest,packed:brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows)),{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22}})});}
 const text=Buffer.from(`${header}\n${image}`),packed=brotliCompressSync(text,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:text.length}});
 return {digest,rowCount:rows.length,packed,headerDigest:createHash("sha256").update(header).digest("hex"),blockDigests:collected.map(block=>block.digest),blocks};
}

/** 🪢️ Shares one row object between every consumer of the same fact, whichever process read it. */
export function canonicalObservationRowV1(row){return canonicalObservationV1(row.kind,row.path,row.fact);}

/** 🔖️ Names one fact by the digest of its exact content key (128 bits), so processes can agree that they hold the same fact without sending it. */
export function observationRowDigestV1(row){return createHash("sha256").update(observationKeyV1(row)).digest("hex").slice(0,32);}

/** 🛰️ Analyses one source file with a registered parser outside every scope and answers the parser record the analysis store keeps for its bytes, so a worker process prepares what its parent will look up. */
export function analyzeSourceRecordV1(provider,path){
 const registered=sourceAnalysisProviderV1(provider),file=absolute(path),bytes=fs.readFileSync(file),sha256=createHash("sha256").update(bytes).digest("hex");
 const result=sourceCaptures.run(undefined,()=>recordScopes.run(undefined,()=>scopes.run(undefined,()=>sourceAnalysisImageV1(registered,file,bytes,sha256))));
 return {key:recordKey("source-analysis-v1",registered.provider.id,registered.provider.revision,file,sha256),bytes:Buffer.from(JSON.stringify(result))};
}

/** 🛰️ Answers the parser records of every analysis this process still holds in memory, so a worker process starts with what this process already parsed instead of parsing it again. */
export function exportSourceAnalysisRecordsV1(){
 return [...sourceAnalysisImages].map(([key,row])=>[key,Buffer.from(JSON.stringify(row.result))]);
}

/** 🛰️ Lists, of the given files, those whose parser record the record store of the current observation does not hold. */
export function missingSourceAnalysisRecordsV1(provider,paths){
 const registered=sourceAnalysisProviderV1(provider),store=recordScopes.getStore()?.records,missing=[];
 if(!store)return missing;
 for(const path of paths){const file=absolute(path);let bytes;try{bytes=fs.readFileSync(file);}catch{continue;}if(store.get(recordKey("source-analysis-v1",registered.provider.id,registered.provider.revision,file,createHash("sha256").update(bytes).digest("hex")))===undefined)missing.push(file);}
 return missing;
}

/** 🛰️ Stores parser records prepared elsewhere in the record store of the current observation and answers how many it stored. */
export function putSourceAnalysisRecordsV1(entries){
 const store=recordScopes.getStore()?.records;if(!store)return 0;
 for(const[key,bytes]of entries)store.put(key,bytes);
 return entries.length;
}

/** 📊️ Reports owned observation counts without exposing or discarding original source facts. */''')
OD = B + "/👁️observation/🟨️.d.mts"
edit(OD, "/** 🧿️ Seals and fingerprints the original module-load authority without issuing a grant. */", '''/** 🛰️ Computes one project without persisting it, for a worker process whose parent persists the result. */
export function observeDetachedProjectInferenceV1<T>(options:Readonly<{validation:Map<unknown,unknown>;records:Readonly<{get(key:string):Uint8Array|undefined;put(key:string,bytes:Uint8Array):void}>;hints?:Map<string,unknown>}>,compute:()=>Promise<T>|T):Promise<{value:T;image:string;rows:SourceObservationV1[];started:number}>;
/** 🛰️ Replays what a worker process consumed into the project scope that awaits it, with the sealed record it built. */
export function replayDetachedProjectInferenceV1(rows:readonly SourceObservationV1[],records:readonly (readonly [string,Uint8Array])[],sealed?:SealedInferenceRecordV1):void;
/** 📦️ The bytes of one project record built in a worker process, and the blocks its parent lacks. */
export interface SealedInferenceRecordV1{readonly digest:string;readonly rowCount:number;readonly packed:Uint8Array;readonly headerDigest:string;readonly blockDigests:readonly string[];readonly blocks:readonly {digest:string;packed:Uint8Array}[]}
/** 🪞️ Marks the JSON text of a project value computed elsewhere. */
export function preimagedProjectValueV1(image:string):object;
/** 🔖️ The digest the record of the project under observation will carry. */
export function currentProjectRecordDigestV1():string|undefined;
/** 🛰️ Seals the record of a project a worker process computed. */
export function sealInferenceRecordV1(input:Readonly<{digest:string;image:string;rows:readonly SourceObservationV1[];known:ReadonlySet<string>;emitted?:Set<string>}>):SealedInferenceRecordV1;
/** 🪢️ Shares one row object between every consumer of the same fact. */
export function canonicalObservationRowV1(row:SourceObservationV1):SourceObservationV1;
/** 🔖️ Names one fact by the digest of its exact content key. */
export function observationRowDigestV1(row:SourceObservationV1):string;
/** 🛰️ Analyses one source file outside every scope and answers the parser record to store for its bytes. */
export function analyzeSourceRecordV1(provider:SourceAnalysisProviderV1,path:string):{key:string;bytes:Buffer};
/** 🛰️ Answers the parser records of every analysis this process holds in memory. */
export function exportSourceAnalysisRecordsV1():[string,Buffer][];
/** 🛰️ Lists the files whose parser record the current record store lacks. */
export function missingSourceAnalysisRecordsV1(provider:SourceAnalysisProviderV1,paths:readonly string[]):string[];
/** 🛰️ Stores parser records prepared elsewhere and answers how many it stored. */
export function putSourceAnalysisRecordsV1(entries:readonly (readonly [string,Uint8Array])[]):number;
/** 🧿️ Seals and fingerprints the original module-load authority without issuing a grant. */''')

exec((HERE / "ops_observation.py").read_text(encoding="utf8"))

# ---- revision: the farm joins the implementation identity --------------------------------------------------------------------
R = INF + "/♻️revision/🟨️.mjs"
farm_files = ["🏗️build/🏊️farm/🟨️.mjs", "🏗️build/🏊️farm/🧬️schema/🔣️.json", "🏗️build/🏊️farm/👷️worker/🟨️.mjs", "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs", "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs", "🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🏊️module-farm/🟨️.mjs", "🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🏊️module-farm/👷️service/🟨️.mjs"]
edit(R, 'return ["🏗️build/📥️inputs/🔏️revision/🟨️.mjs",', "return [" + ", ".join(json.dumps(f, ensure_ascii=False) for f in farm_files) + ', "🏗️build/📥️inputs/🔏️revision/🟨️.mjs",')

# ---- host capacity: the worker processes ask the machine-wide gate ------------------------------------------------------------
H = B + "/🌡️host-capacity/🟦️.ts"
edit(H, "/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy:", '''/** 🏊️ Asks the machine-wide gate for the worker processes of the graph inference farm while the invocation already holds its own grant: the request is a fresh one (the inherited grant is set aside), waits at most `waitMilliseconds` and answers the jobs it was granted and how to give them back; answers all preferred jobs when the gate is off and undefined when no grant came in time. */
export async function acquireBuildFarmCapacityV1(input:HostCapacityPortsV1&Readonly<{preferredJobs:number;waitMilliseconds:number;environment?:NodeJS.ProcessEnv;directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void}>):Promise<Readonly<{jobs:number;release:()=>void}>|undefined>{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,environment:NodeJS.ProcessEnv={...(input.environment??process.env)};
 delete environment["SEMIO_HOST_CAPACITY_GRANT"];delete environment["SEMIO_HOST_CAPACITY_PLAN"];
 const request=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:"exec",preferredJobs:input.preferredJobs,kinds:["build"]});
 if(!policy.enabled||environment["SEMIO_HOST_CAPACITY"]==="off"||!request)return Object.freeze({jobs:input.preferredJobs,release:()=>{}});
 const controller=new AbortController(),control:ResourceLeaseControl=Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))});
 let release=()=>{};const held=new Promise<void>(done=>{release=done;});
 const granted=new Promise<HostCapacityHandleV1>((accept,reject)=>{withHostCapacityV1({...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment},async handle=>{accept(handle);await held;}).catch(reject);});
 const timer=setTimeout(()=>controller.abort(Error("Farm capacity wait elapsed")),input.waitMilliseconds);
 try{const handle=await granted;return Object.freeze({jobs:handle.jobs,release});}
 catch{release();return undefined;}
 finally{clearTimeout(timer);}
}

/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy:''')

# ---- dependency edges: the parser records are prepared in worker processes ---------------------------------------------------
D = INF + "/🕸️dependency-edges/🟨️.mjs"
edit(D, 'import { buildRelativeImportProjectV1 } from "../../🚪️admission/🟦️.ts";', 'import { buildRelativeImportProjectV1 } from "../../🚪️admission/🟦️.ts";\nimport { prefillModuleAnalysesV1 } from "./🏊️module-farm/🟨️.mjs";')
edit(D, "export async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add, publication, scheduledFiles,context={}) {\n  const owners=importProjectOwners(projects),jobs=[];\n  for(const[name,files]of Object.entries(projectFiles))for(const file of files)if(/\\.[cm]?[jt]sx?$/.test(file.file))jobs.push({name,file});\n  let cursor=0;", "export async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add, publication, scheduledFiles,context={}) {\n  const owners=importProjectOwners(projects),jobs=[];\n  for(const[name,files]of Object.entries(projectFiles))for(const file of files)if(/\\.[cm]?[jt]sx?$/.test(file.file))jobs.push({name,file});\n  await prefillModuleAnalysesV1({workspaceRoot,files:[...new Set(jobs.map(job=>join(workspaceRoot,buildPath(job.file.file))))],provider:prepareSourceAnalysesV1().module,signal:context.invocation?.control.signal});\n  let cursor=0;")

# ---- project nodes: shared body, farm ------------------------------------------------------------------------------------------
N = NODES + "/🟨️.mjs"
edit(N, 'import { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";', 'import { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";\nimport { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";')
edit(N, "/** 🛡️ Infers the project nodes of every authored emoji-named project config.", '''/** 🔖️ Names the implementation revision this component loaded, which a worker process must share with its parent. */
export const projectInferenceIdentityV1 = () => revisionIdentity.revision;

/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes. */
export function createProjectInferenceEnvironmentV1(session) {
  const { workspaceRoot, registry, commandInputs, projectsByRoot, componentRoots } = session, context = { workspaceRoot, registry }, scripts = createScriptInputCache(context);
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
  scripts.generatedOutput = path => readBuildGeneratedCompileOutputV1(path, workspaceRoot, contracts, registry);
  return { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot: new Map(projectsByRoot), componentRoots, nativeCache: new Map(), nativeClosures: new Map(), context };
}

/** 🧱️ Infers the node of one project: its defaults, its native or release preparation and its build modes; a source that does not parse withholds the project instead of failing the graph. */
export async function inferProjectBodyV1(environment, job, admission) {
  const { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context } = environment, { name, root, json, native, pipeline, selectedTargets, selectedInputs } = job;
  try {
    await yieldInferenceV1(context, "build:project-assembly", 0);
    const project = await projectWithDefaults(json, root, join(workspaceRoot, root), workspaceRoot, contracts, new Map(), commandInputs, scripts, undefined, "hash", admission ? selectedTargets ?? [] : undefined, selectedInputs);
    const prepare = (apply, predicate) => { const preparing = admission ? { ...project, targets: Object.fromEntries(Object.entries(project.targets).filter(([, target]) => predicate(target))) } : project; apply(preparing); project.targets = { ...project.targets, ...preparing.targets }; if (preparing.metadata !== undefined) project.metadata = preparing.metadata; };
    if (native) prepare(project => withNativePreparation(project, workspaceRoot, contracts, nativeCache, projectsByRoot, nativeClosures), target => target.metadata?.semio?.execution?.kind !== "source");
    if (pipeline) prepare(project => withReleasePipelinePreparation(project, workspaceRoot, contracts, componentRoots, nativeCache, nativeClosures), target => Boolean(target.metadata?.semio?.release));
    for (const [targetName, target] of Object.entries(project.targets)) project.targets[targetName] = loaded.targetBuildModes(targetName, target);
    return project;
  } catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; return { unavailable: unavailableProjectV1(root, name, workspaceRoot, error) }; }
}

/** 🛡️ Infers the project nodes of every authored emoji-named project config.''')
edit(N, "  const componentRoots = componentCrateRoots(configFiles, workspaceRoot);", "  const componentRoots = componentCrateRoots(configFiles, workspaceRoot);\n  const projectEnvironment = { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context };")
edit(N, "  const inferProject=async({result,name,json})=>{", '''  const farm=options?.ownerAdmission?undefined:await openProjectFarmV1({workspaceRoot,context,jobs:jobs.length,identity:revisionIdentity.revision,commandInputs,projectsByRoot,componentRoots,readRoot:workspaceDataDirectory(workspaceRoot)});
  if(farm)options={...options,projectConcurrency:Math.max(options?.projectConcurrency??8,farm.width*2)};
  const inferProject=async({result,name,json})=>{''')
body = span(N, 'async()=>{\n      try{\n      await yieldInferenceV1(context,"build:project-assembly",0);', 'unavailableProjectV1(root,name,workspaceRoot,error)};}\n    }')
edit(N, body, '''async()=>{
      if(farm){const farmed=await farm.infer({name,root,json,native,pipeline,nativeRoot,selectedInputs});if(farmed)return farm.replay(farmed);}
      return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission);
    }''')
edit(N, "await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});", "await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;}).finally(()=>farm?.close());")

new = [
    (B + "/🏊️farm/🟨️.mjs", "staged/new/farm/engine.mjs"),
    (B + "/🏊️farm/🟨️.d.mts", "staged/new/farm/engine.d.mts"),
    (B + "/🏊️farm/🧬️schema/🔣️.json", "staged/new/farm/schema.json"),
    (B + "/🏊️farm/👷️worker/🟨️.mjs", "staged/new/farm/worker.mjs"),
    (NODES + "/🏊️project-farm/🟨️.mjs", "staged/new/project-farm/glue.mjs"),
    (NODES + "/🏊️project-farm/👷️service/🟨️.mjs", "staged/new/project-farm/service.mjs"),
    (INF + "/🕸️dependency-edges/🏊️module-farm/🟨️.mjs", "staged/new/module-farm/glue.mjs"),
    (INF + "/🕸️dependency-edges/🏊️module-farm/👷️service/🟨️.mjs", "staged/new/module-farm/service.mjs"),
]
extra = HERE / "staged/new-files.json"
if extra.exists():
    new += [(item["file"], item["from"]) for item in json.loads(extra.read_text(encoding="utf8"))]
spec = {"version": 1, "edits": edits, "newFiles": [{"file": f, "from": s} for f, s in new]}
(HERE / "staged/ops.json").write_text(json.dumps(spec, ensure_ascii=False, indent=1), encoding="utf8")

print("edits", len(spec["edits"]), "new files", len(new), "root", root)
