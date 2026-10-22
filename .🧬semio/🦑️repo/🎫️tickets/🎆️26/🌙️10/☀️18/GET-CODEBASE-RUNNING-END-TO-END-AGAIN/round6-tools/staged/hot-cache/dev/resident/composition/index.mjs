/** 🏗 Composition root of `repo service start --foreground`: binds the repository's graph, warm workers, custody and the native watcher to the resident service. */
import {createHash} from "node:crypto";
import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {dirname,join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {createResidentServiceV1} from "../service/index.mjs";
import {createBuildGraphPortV1} from "../ports/graph.mjs";
import {scanListingV1} from "../ports/listing.mjs";
import {createDelegatedExecutionPortV1,createWarmPoolV1,writeWarmPreloadV1} from "../delegate/index.mjs";
import {createExecutorPoolV1} from "../delegate/executor.mjs";
import {POLICY_V1} from "../protocol/policy.mjs";
import {RESIDENT_EXCLUDES_V1,RESIDENT_FENCE_DIRECTORY_V1} from "../command/index.mjs";

/** 🧬 Digest and workspace-relative paths of the service's own source files; an edit to any of them makes the running service stale. */
export function residentImplementationV1(workspaceRoot){
 const member=join(dirname(fileURLToPath(import.meta.url)),".."),files=[];
 const walk=directory=>{for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()){if(entry.name!==TESTS_DIRECTORY)walk(path);}else if(/\.(mjs|json|rs)$/u.test(entry.name))files.push(path);}};
 walk(member);files.sort();
 const hash=createHash("sha256");for(const file of files)hash.update(relative(member,file)+"\0").update(readFileSync(file));
 return{implementation:hash.digest("hex"),implementationPaths:files.map(file=>relative(workspaceRoot,file).split(sep).join("/"))};
}

const TESTS_DIRECTORY="\u{1F9EA}\uFE0Ftests";

/** 🔌 Adapts the library's custody pool (`with(run)`, `close()`) to the pin protocol of the service. */
export function custodyPortFromPoolV1(pool){
 return{acquire:()=>pool.with(async()=>undefined),release:()=>pool.close()};
}

/** ▶ Builds the foreground runner. `dependencies` are injected by the CLI wiring so this module imports nothing from the repository. */
export function createForegroundRunnerV1(dependencies){
 const{loadGraph,script,execPath=process.execPath,warmModules=[],custody,salt,watcherBinary,generationOf,environmentPaths,environmentPrefixes,warmSize=process.platform==="win32"?0:1,executorSize=Number(dependencies.environment?.SEMIO_RESIDENT_EXECUTORS??process.env.SEMIO_RESIDENT_EXECUTORS??0),capacity,log=()=>{}}=dependencies;
 return async({runtime,root,environment})=>{
  const binary=watcherBinary&&existsSync(watcherBinary)?watcherBinary:undefined,{implementation,implementationPaths}=dependencies.implementation??residentImplementationV1(root);
  const preload=writeWarmPreloadV1(runtime.directory);
  const pool=createWarmPoolV1({command:[execPath,"--preload",preload,script],environment:{...environment,SEMIO_WARM_MODULES:JSON.stringify(warmModules),SEMIO_BUILD_SERVICE:"off"},cwd:root,size:warmSize,idleMilliseconds:POLICY_V1.idleShutdownMilliseconds,log});
  const filterHolder={excluded:()=>false};
  const graph=createBuildGraphPortV1({loadGraph,salt,generation:generationOf,environmentPaths,environmentPrefixes,listing:async({root:r,exclude,filter})=>{Object.assign(filterHolder,filter);return scanListingV1({root:r,binary,exclude,filter});}});
  const executors=executorSize>0&&process.platform!=="win32"?createExecutorPoolV1({command:[execPath,script,"repo","service","executor"],environment:{...environment,SEMIO_BUILD_SERVICE:"off"},cwd:root,size:executorSize,log}):undefined;
  const execution=createDelegatedExecutionPortV1({pool,executors,coldCommand:[execPath,script],argvFor:id=>["repo","run",id,"--service=off"],environment:{...environment,SEMIO_BUILD_SERVICE:"off"},cwd:root});
  const service=createResidentServiceV1({root,runtime,implementation,implementationPaths,backend:binary?"auto":"portable",watcherBinary:binary,exclude:{rooted:[...RESIDENT_EXCLUDES_V1.rooted],names:[...RESIDENT_EXCLUDES_V1.names]},fenceDirectory:RESIDENT_FENCE_DIRECTORY_V1,ports:{graph,execution,custody,capacity},log});
  const stop=reason=>void service.stop(reason);
  process.once("SIGTERM",()=>stop("SIGTERM"));process.once("SIGINT",()=>stop("SIGINT"));
  await service.start();pool.top();executors?.top();
  const reason=await service.closed;await pool.close();await executors?.close();
  return reason==="stale"||reason==="memory"?75:0;
 };
}
