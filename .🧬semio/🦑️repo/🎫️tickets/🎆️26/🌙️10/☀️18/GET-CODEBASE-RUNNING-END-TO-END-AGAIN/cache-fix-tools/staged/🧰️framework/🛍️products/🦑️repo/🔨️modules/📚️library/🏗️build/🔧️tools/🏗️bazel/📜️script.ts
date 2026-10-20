import{withRepositoryCacheStoreV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildOrchestrationContextV1} from "../../📥️caller/📤️orchestration/🟨️.mjs";
import {issueStoreReceiptV1} from "../../🧾️store-receipt/🟨️.mjs";
import {withBazelStorageV1,requireBazelStorageV1,bazelCacheFlagsV1,type BazelStorageV1} from "./💾️storage/🟦️.ts";
import {createHash,randomUUID} from "node:crypto";
import {spawn} from "node:child_process";
import {constants} from "node:fs";
import targetCacheSchema from"../../🏃️execution/🧬️schema/💾️cache.json" with { type: "json" };
import componentSchema from"./🧬️schema/🔣️.json" with { type: "json" };const {ActionBirth:birthSchema,OutputSnapshot:snapshotSchema,ModuleSetup:moduleSchema,ImplementationPolicy:policySchema,Planner:plannerSchema,Implementation:implementationSchema,Manifest:manifestSchema,Runtime:runtimeSchema,CacheDecision:cacheDecisionSchema,UncachedReport:uncachedReportSchema}=componentSchema.definitions;
import {createServer,createConnection,type Socket} from "node:net";
import {chmod,copyFile,link,lstat,mkdir,open,readFile,readdir,readlink,realpath,rename,rm,stat,symlink,writeFile} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {readBuildGraphV1,resolveBuildTargetV1,type BuildGraphV1,type BuildTargetV1} from "../../🕸️graph/🟦️.ts";
import type {BazelTaskPlanV1,BazelTaskV1} from "./📋️task-plan/🟦️.ts";
import {readReadinessDeclarationV1,resolveReadinessV1,watchReadinessV1,type ReadinessWatchV1} from "../../🟢️readiness/🟦️.ts";
import {buildExecutionArgumentsV1,issueBuildTaskEnvironmentV1,withBuildOrchestrationTaskEnvironmentV1,type NativeBuildCallerCapabilitiesV1} from "../../📥️caller/🟦️.ts";
import {BAZEL_VERSION,prepareBazel} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts";
import {buildExecEnvironmentV1} from "../../⚙️operations/🌱️exec-environment/🟦️.ts";
import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";
export {prepareBazel} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1,runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {advanceScriptInvocation,checkScriptInvocation,receiveScriptProcessInvocation,scriptInvocationBudget,scriptInvocationEnvironment,withScriptInvocationSignalV1,readIssuedScriptProcessEnvelopeV1,readScriptProcessEnvelope,withScriptProcessEnvelope,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1,loadBazelClosuresV1,managedBazelClosureFetchV1,workspaceBazelLockFilesV1,type BazelClosureFetchV1,type BazelClosuresV1} from "./🔗️closure/🟦️.ts";



const policy=componentSchema.definitions.Policy.const;
const inputExpansionPolicy=componentSchema.definitions.InputExpansionPolicy.const;


import{captureBuildSourceClosureV1,buildSourceClosureIdentityV1,buildSourceClosureLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";

declare global{interface ImportMeta{readonly semioCapturedImplementationIdentity?:string;readonly semioCapturedLoaderIdentity?:string}}

export interface BazelTaskRequestV1{tasks:string[];args?:string[];configuration?:string;parallel?:number;excludeDependencies?:boolean;skipCache?:boolean;skipRemoteCache?:boolean;bail?:boolean;verbose?:boolean;outputStyle?:"stream"|"stream-without-prefixes"|"static"}
type BazelImplementationV1={version:1;snapshot:string;runner:string;sha256:string;loaderSha256:string};
type BazelManifestImplementationV1={version:1;sha256:string;loaderSha256:string};
type BazelTaskManifestV1={version:1;task:BazelTaskV1;implementation:BazelManifestImplementationV1;orchestration?:object};
type BazelRuntimeV1={version:1;workspace:string;environment:NodeJS.ProcessEnv};
export interface BazelSnapshotContextV1{workspace:string;task:BazelTaskV1}
export type BazelUncachedReasonV1="continuous"|"declared"|"skipped"|"undeclared"|"unbounded"|"driver-owned"|"managed-output"|"side-effecting"|"live"|"aggregate"|"diagnostic"|"unclassified";
export interface BazelCacheDecisionV1{cache:boolean;reason?:BazelUncachedReasonV1}
export interface BazelUncachedReportV1{version:1;actions:number;cached:number;uncached:Record<string,number>;unbounded:{project:string;root:string;targets:{target:string;declared:"cache"|"class"}[]}[]}
export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch";restore:string[];cache:BazelCacheDecisionV1}
type SnapshotEntryV1={path:string;kind:"directory"|"file"|"symlink";mode:number;file?:string;sha256?:string;link?:string;linkKind?:"directory"|"file"};
type SnapshotV1={version:1;roots:string[];entries:SnapshotEntryV1[]};
const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),slash=(value:string)=>value.split(sep).join("/");
const BAZEL_MODULE='module(name = "semio_tasks", version = "0.0.0")\n';
if(validateJsonSchemaSubset(policySchema,policy).length)throw new Error("Bazel implementation policy refused");
const actions=componentSchema.definitions.Actions.const,sourceLabel=new RegExp(actions.sources.label);
if(validateJsonSchemaSubset(componentSchema.definitions.ActionPolicy,actions).length)throw new Error("Bazel action policy refused");

/** 🛠️ Prepares Bazel's fixed embedded module catalog once for every cold task selection. */
export async function prepareBazelModulesV1(root:string,bazel:string,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl):Promise<{lock:string;repository:string}>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const environment=issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],storage.dependencies.environment),offline=environment.CARGO_NET_OFFLINE==="true",identity=hash(JSON.stringify({version:1,bazel:BAZEL_VERSION,platform:process.platform,arch:process.arch,module:BAZEL_MODULE})),directory=join(storage.paths.modules,identity.slice(0,16)),workspace=join(directory,"workspace"),repository=join(directory,"repositories"),lockPath=join(workspace,"MODULE.bazel.lock"),receiptPath=join(directory,"ready.json"),blobs=join(repository,"content_addressable/sha256");
 const read=async()=>{try{const receipt=JSON.parse(await readFile(receiptPath,"utf8"));if(validateJsonSchemaSubset(moduleSchema,receipt).length||receipt.identity!==identity)throw new Error("Invalid Bazel module setup receipt");const lock=await readFile(lockPath,"utf8");if(hash(lock)!==receipt.lockSha256)throw new Error("Bazel module lock checksum changed");for(const digest of receipt.blobs){checkScriptInvocation(invocation);if(await fileHash(join(blobs,digest,"file"),invocation.control.signal)!==digest)throw new Error("Bazel module repository checksum changed");}return{lock,repository};}catch(error){if(invocation.control.signal.aborted)throw error;return undefined;}};
 return withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal:phaseControl.signal,control:phaseControl,resources:[{resource:"bazel-modules-"+identity,mode:"exclusive"}]},async()=>{const ready=await read();if(ready)return ready;if(offline)throw new Error("Pinned Bazel module setup is not prepared for the original offline invocation");await mkdir(workspace,{recursive:true});await writeChanged(join(workspace,"MODULE.bazel"),BAZEL_MODULE);await writeChanged(join(workspace,"BUILD.bazel"),'load(":setup.bzl", "setup")\nsetup(name="setup")\n');await writeChanged(join(workspace,"setup.bzl"),'def _impl(ctx):\n    output=ctx.actions.declare_file("ready.txt")\n    ctx.actions.write(output,"ready")\n    return [DefaultInfo(files=depset([output]))]\nsetup=rule(implementation=_impl)\n');
  console.log("[bazel] preparing fixed embedded module catalog");await runOwnedCommand(bazel,["--batch",`--output_base=${join(directory,"base")}`,`--install_base=${join(storage.paths.install,BAZEL_VERSION,process.platform+"-"+process.arch)}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc","build","//:setup","--incompatible_autoload_externally=","--lockfile_mode=update",`--repository_cache=${repository}`,"--http_connector_attempts=2","--experimental_repository_downloader_retries=0"],workspace,"bazel:module-setup",Math.min(policy.moduleSetupMilliseconds,phaseControl.remainingMilliseconds()??policy.moduleSetupMilliseconds),{signal:phaseControl.signal,env:{...environment,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})}});
  const lock=await readFile(lockPath,"utf8"),files=(await readdir(blobs)).filter(value=>/^[a-f0-9]{64}$/u.test(value)).sort();await writeChanged(receiptPath,JSON.stringify({version:1,identity,lockSha256:hash(lock),blobs:files}));const prepared=await read();if(!prepared)throw new Error("Bazel fixed module setup validation failed");console.log(`[bazel] fixed module catalog ready blobs=${files.length}`);return prepared;
 });
}

/** 🧭️ Resolves task identity and its configuration without implementation-specific parsing. */
function taskIdentity(graph:BuildGraphV1,value:string,configuration?:string):{id:string;project:string;target:string;configuration?:string}{
 const parts=value.split(":"),project=parts[0]!,target=parts[1]!,definition=graph.projects[project]?.targets[target];
 if(parts.length<2||parts.length>3||!definition)throw new Error(`Unknown build task: ${value}`);
 const requested=parts[2]??configuration,selected=requested&&definition.configurations?.[requested]?requested:definition.defaultConfiguration as string|undefined,chosen=selected&&definition.configurations?.[selected]?selected:undefined;
 if(parts[2]&&!definition.configurations?.[parts[2]])throw new Error(`Unknown build configuration: ${value}`);
 return {project,target,...(chosen?{configuration:chosen}:{}),id:`${project}:${target}${chosen?":"+chosen:""}`};
}

/** 🧷 Refuses a declared task dependency whose project or target is absent from the graph, naming the depending task, the declaration and the missing half. */
export function requireBazelDependencyV1(graph:BuildGraphV1,owner:string,dependency:string):void{
 const[project,target]=dependency.split(":");
 if(!graph.projects[project!])throw new Error(`Unknown build task dependency: ${owner} depends on ${dependency}, but project ${project} is not in the build graph`);
 if(!graph.projects[project!]!.targets[target??""])throw new Error(`Unknown build task dependency: ${owner} depends on ${dependency}, but project ${project} defines no target ${target}`);
}

/** 🔎️ Lists every qualified task dependency, in base targets and configurations alike, that names an absent project or target, so owners fix them in one pass instead of one refusal at a time. */
export function bazelDanglingDependenciesV1(graph:BuildGraphV1):{task:string;dependency:string;missing:"project"|"target"}[]{
 const found=new Map<string,{task:string;dependency:string;missing:"project"|"target"}>();
 for(const[project,owner]of Object.entries(graph.projects))for(const[target,definition]of Object.entries(owner.targets))for(const row of [definition,...Object.values(definition.configurations??{})])for(const entry of (row as {dependsOn?:BuildTargetV1["dependsOn"]}).dependsOn??[]){
  const dependency=(typeof entry==="string"?entry:entry.target).replace(/^\^/u,"");if(!dependency.includes(":"))continue;
  const[name,goal]=dependency.split(":"),missing=!graph.projects[name!]?"project":!graph.projects[name!]!.targets[goal??""]?"target":undefined;
  if(missing)found.set(project+":"+target+"\0"+dependency,{task:project+":"+target,dependency,missing});
 }
 return [...found.values()].sort((left,right)=>left.task<right.task?-1:left.task>right.task?1:left.dependency<right.dependency?-1:left.dependency>right.dependency?1:0);
}

/** 🕸️ Projects exact portable dependency identities before Bazel sees any actions. */
export function planBazelTasksV1(graph:BuildGraphV1,request:BazelTaskRequestV1):BazelTaskPlanV1{
 const tasks:Record<string,BazelTaskV1>={},visiting=new Set<string>(),selected:string[]=[],explicit=new Set(request.tasks.map(value=>taskIdentity(graph,value,request.configuration).id)),projectDependencies=new Map<string,string[]>(),matchedTargets=new Map<string,string[]>(),targetNames=[...new Set(Object.values(graph.projects).flatMap(project=>Object.keys(project.targets)))];
 for(const edge of graph.dependencies)if(graph.projects[edge.target]){const dependencies=projectDependencies.get(edge.source)??[];dependencies.push(edge.target);projectDependencies.set(edge.source,dependencies);}
 const include=(value:string,args:string[],configuration?:string):string=>{
  const identity=taskIdentity(graph,value,configuration),{id,project,target}=identity,inherited=value.split(":")[2]??configuration??identity.configuration;
  if(visiting.has(id))throw new Error(`Build task dependency cycle: ${[...visiting,id].join(" -> ")}`);
  if(tasks[id])return id;
  if(explicit.has(id))args=request.args??[];
  visiting.add(id);const owner=graph.projects[project]!,definition=resolveBuildTargetV1(owner.targets[target]!,identity.configuration),dependencies=new Set<string>();
  for(const dependency of definition.dependsOn??[]){
   const specification=typeof dependency==="string"?{target:dependency}:dependency,pattern=specification.target,fromDependencies=pattern.startsWith("^")||specification.projects==="dependencies"||specification.dependencies===true,operation=pattern.replace(/^\^/u,""),forwarded=specification.params==="forward"?args:[];
   const add=(value:string)=>{requireBazelDependencyV1(graph,id,value);const dependency=taskIdentity(graph,value,inherited).id;if(dependency!==id&&(!request.excludeDependencies||explicit.has(dependency)))dependencies.add(include(value,forwarded,inherited));};
   if(operation.includes(":")){add(operation);continue;}
   const projects=specification.projects===undefined||specification.projects==="self"?[project]:typeof specification.projects==="string"?[specification.projects]:specification.projects;
   let operations=matchedTargets.get(operation);if(!operations){const matcher=new Bun.Glob(operation);operations=targetNames.filter(name=>matcher.match(name));matchedTargets.set(operation,operations);}
   for(const name of operations){
    if(fromDependencies){const seen=new Set<string>(),pending=[...projectDependencies.get(project)??[]];while(pending.length){const candidate=pending.pop()!;if(seen.has(candidate))continue;seen.add(candidate);if(graph.projects[candidate]!.targets[name])add(`${candidate}:${name}`);else pending.push(...projectDependencies.get(candidate)??[]);}}
    else for(const candidate of Object.keys(graph.projects).filter(name=>projects.some(pattern=>!pattern.startsWith("!")&&new Bun.Glob(pattern).match(name))&&!projects.some(pattern=>pattern.startsWith("!")&&new Bun.Glob(pattern.slice(1)).match(name))))if(graph.projects[candidate]!.targets[name])add(`${candidate}:${name}`);
   }
  }
  tasks[id]={...identity,label:`t_${hash(id).slice(0,16)}`,dependencies:[...dependencies].sort(),args:[...args],definition,root:owner.root};visiting.delete(id);return id;
 };
 for(const value of request.tasks)selected.push(include(value,request.args??[],request.configuration));
 if(!selected.length)throw new Error("Bazel requires at least one selected build task");
 return {version:1,tasks,selected:[...new Set(selected)]};
}

/** 🗂️ Restricts generated snapshots and declared outputs to the real workspace. */
function workspacePath(root:string,path:string):string{
 const target=resolve(root,path),inside=relative(root,target);if(!inside||inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside))throw new Error(`Build output must be inside the workspace: ${path}`);return target;
}

/** 🛡️ Refuses output ancestors that redirect a restore outside actual directory custody. */
async function outputAncestry(root:string,path:string):Promise<void>{
 for(let current=dirname(workspacePath(root,path));current!==resolve(root);current=dirname(current))try{const stat=await lstat(current);if(!stat.isDirectory()||stat.isSymbolicLink())throw new Error(`Build output requires real directory ancestry: ${current}`);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
}

/** 🔏️ Streams an output identity with cancellation between reads. */
async function fileHash(path:string,signal:AbortSignal):Promise<string>{
 const file=await open(path,"r"),digest=createHash("sha256");try{for await(const bytes of file.createReadStream({autoClose:false})){signal.throwIfAborted();digest.update(bytes);}return digest.digest("hex");}finally{await file.close();}
}

/** 🧵️ Hashes followed source content with finite directory admission and cycle identity. */
export async function bazelSourceIdentityV1(path:string,signal:AbortSignal):Promise<string>{
 const active=new Set<string>();let visited=0;
 const visit=async(path:string,depth:number):Promise<string>=>{
  signal.throwIfAborted();if(depth>64||++visited>1_000_000)throw new Error("Bazel source identity exceeds its bounded traversal");
  let info;try{info=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return hash("absent");throw error;}
  const digest=createHash("sha256");
  if(info.isSymbolicLink()){digest.update("link:"+await readlink(path)+"\0");try{info=await stat(path);}catch(error){if(["ENOENT","ELOOP"].includes((error as NodeJS.ErrnoException).code??""))return digest.update("unresolved").digest("hex");throw error;}}
  if(info.isFile())return digest.update("file:").update(await fileHash(path,signal)).digest("hex");
  if(!info.isDirectory())throw new Error(`Bazel source input requires an ordinary file or directory: ${path}`);
  const identity=`${info.dev}:${info.ino}`;if(active.has(identity))return digest.update("cycle:"+await realpath(path)).digest("hex");
  active.add(identity);try{digest.update("directory\0");for(const name of(await readdir(path)).sort())digest.update(name+"\0"+await visit(join(path,name),depth+1)+"\0");return digest.digest("hex");}finally{active.delete(identity);}
 };
 return visit(path,0);
}

/** 🧩️ Expands portable workspace, project and owner option tokens; an undeclared `env.<name>` option is the value the task process receives. */
function interpolate(value:string,root:string,task:BazelTaskV1,environment?:()=>NodeJS.ProcessEnv):string{
 return value.replaceAll("{workspaceRoot}",root).replaceAll("{projectRoot}",resolve(root,task.root)).replaceAll("{projectName}",task.project).replace(/\{options\.([^}]+)\}/gu,(_match,key:string)=>{let option:unknown=task.definition.execution;for(const part of key.split("."))option=option&&typeof option==="object"?(option as Record<string,unknown>)[part]:undefined;if(option===undefined&&environment&&key.startsWith("env."))option=environment()[key.slice(4)];if(typeof option!=="string"&&typeof option!=="number"&&typeof option!=="boolean")throw new Error(`Build task requires option ${key}: ${task.id}`);return String(option);});
}

/** 🌱️ Selects the smallest directory or literal-file roots needed by declared input patterns. */
export function bazelInputTraversalRootsV1(patterns:readonly string[]):string[]{
 const roots=[...new Set(patterns.map(pattern=>{const index=pattern.search(/[?*{[]/u),prefix=index<0?pattern:pattern.slice(0,index),root=slash(index<0?prefix:prefix.endsWith("/")?prefix:dirname(prefix)).replace(/^\.\//u,"").replace(/\/+$/u,"");return root==="."?"":root;}))].sort();
 if(roots[0]==="")return [""];const selected=new Set(roots);return roots.filter(root=>{for(let index=root.indexOf("/");index>=0;index=root.indexOf("/",index+1))if(selected.has(root.slice(0,index)))return false;return true;});
}

/** 🌳️ Enumerates selected inputs without opening unrelated directories when Git is unavailable. */
export async function bazelFilesystemSourceFilesV1(root:string,invocation:ScriptInvocation,patterns:readonly string[]):Promise<string[]>{
 const excluded=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]),pending=bazelInputTraversalRootsV1(patterns),files:string[]=[];
 while(pending.length){checkScriptInvocation(invocation);const directory=pending.pop()!;if(directory.split("/").some(part=>excluded.has(part)))continue;let info;try{info=await lstat(join(root,directory));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(info.isSymbolicLink()||info.isFile()){files.push(directory);continue;}if(!info.isDirectory())continue;for(const entry of await readdir(join(root,directory),{withFileTypes:true})){const path=slash(join(directory,entry.name));if(entry.isDirectory()&&!excluded.has(entry.name))pending.push(path);else if(entry.isFile()||entry.isSymbolicLink())files.push(path);}await invocation.control.yieldContinuation();}return [...new Set(files)].sort();
}

/** 📚️ Enumerates admitted source names through Git or a selected portable filesystem walk. */
async function sourceFiles(root:string,invocation:ScriptInvocation,patterns:string[]):Promise<string[]>{
 checkScriptInvocation(invocation);
 if(!patterns.length)return [];
 const prefixes=bazelInputTraversalRootsV1(patterns),pathspecs=prefixes.includes("")?[]:prefixes;
 const git=spawn("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z",...(pathspecs.length?["--",...pathspecs]:[])],{cwd:root,stdio:["ignore","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];let bytes=0;
 const cancel=()=>git.kill();invocation.control.signal.addEventListener("abort",cancel,{once:true});
 try{
  const result=await new Promise<number>((accept,reject)=>{git.stdout.on("data",chunk=>{bytes+=chunk.length;if(bytes>64*1024*1024){git.kill();reject(Error("Bazel source inventory exceeds 64 MiB"));}else chunks.push(chunk);});git.stderr.resume();git.once("error",reject);git.once("close",code=>accept(code??1));});
  checkScriptInvocation(invocation);if(result===0)return [...new Set(Buffer.concat(chunks).toString().split("\0").filter(Boolean))].filter(path=>!path.startsWith(".🧬semio/")&&!path.startsWith(".git/")).sort();
 }catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 finally{invocation.control.signal.removeEventListener("abort",cancel);}
 return bazelFilesystemSourceFilesV1(root,invocation,patterns);
}

/** 🧾️ Expands exact project-owned Source groups before either declarations or fingerprints select files; `^name` and `dependencies: true` reach the transitive dependency closure of the project, as Nx hashes them. */
export function bazelTaskInputsV1(graph:BuildGraphV1,task:BazelTaskV1,exactClosures?:BazelClosuresV1|"ignore"):unknown[]{
 const active=new Set<string>();let entries=0;
 const groups=(project:string):Record<string,unknown[]>=>{if(!graph.projects[project])throw Error("Unknown build input project: "+project);return{default:["{projectRoot}/**/*"],...graph.namedInputs,...graph.projects[project]!.namedInputs as Record<string,unknown[]>|undefined};};
 const adjacency=new Map<string,string[]>(),closures=new Map<string,string[]>();for(const edge of graph.dependencies)if(graph.projects[edge.target]){const row=adjacency.get(edge.source)??[];if(!row.includes(edge.target))row.push(edge.target);adjacency.set(edge.source,row);}
 const dependencies=(project:string):string[]=>{let known=closures.get(project);if(!known){known=[];const seen=new Set([project]),pending=[project];for(let position=0;position<pending.length;position++)for(const target of adjacency.get(pending[position]!)??[])if(!seen.has(target)){seen.add(target);known.push(target);pending.push(target);}closures.set(project,known);}return known;};
 const named=(name:string,project:string,depth:number):unknown[]=>{const values=groups(project)[name],key=project+":"+name;if(!Array.isArray(values))throw Error("Unknown build input group: "+key);if(active.has(key))throw Error("Build input group cycle: "+key);active.add(key);try{return expand(values,project,depth+1);}finally{active.delete(key);}};
 const expand=(inputs:readonly unknown[],project:string,depth=0):unknown[]=>{if(depth>inputExpansionPolicy.maximumDepth)throw Error("Build input expansion bound exceeded");const declared=groups(project);return inputs.flatMap(input=>{
  if(++entries>inputExpansionPolicy.maximumEntries)throw Error("Build input expansion bound exceeded");
  if(typeof input==="string"){
   if(input.startsWith("^")){const name=input.slice(1);return dependencies(project).flatMap(owner=>name.startsWith("{projectRoot}")||name.startsWith("{workspaceRoot}")?expand([name],owner,depth+1):named(name,owner,depth));}
   if(Object.hasOwn(declared,input))return named(input,project,depth);
   const projectRoot=graph.projects[project]!.root==="."?"":graph.projects[project]!.root;return[input.replaceAll("{projectRoot}",projectRoot).replaceAll("{workspaceRoot}/","").replaceAll("{workspaceRoot}","").replace(/^\.\//u,"").replace(/^\/+/u,"")];
  }
  const closure=bazelClosureMarkerV1(input);if(closure){if(exactClosures==="ignore")return[];if(!exactClosures)throw Error(`Bazel build input closure ${closure.kind} ${closure.key} requires its loaded manifest`);return[...exactClosures.files(closure)];}
  if(input&&typeof input==="object"&&"input"in input){const row=input as{input:string;projects?:string|string[];dependencies?:boolean};if(validateJsonSchemaSubset(componentSchema.definitions.NamedInputReference,row).length)throw Error("Closed build named input required");const owners=row.dependencies?dependencies(project):row.projects===undefined?[project]:typeof row.projects==="string"?[row.projects]:row.projects;if(!owners.length&&row.projects!==undefined||owners.some(owner=>typeof owner!=="string"||!owner))throw Error("Closed build named input required");return owners.flatMap(owner=>named(row.input,owner,depth));}
  return[input];
 });};
 return expand(task.definition.inputs??["{projectRoot}/**/*"],task.project);
}

/** 🧬️ Reads the closed cache decision the inference plugin attaches to a target; an invalid one is refused. */
export function bazelTargetCacheClassV1(task:BazelTaskV1):string|undefined{
 const row=(task.definition as {metadata?:{semio?:{cache?:unknown}}}).metadata?.semio?.cache;
 if(row===undefined)return undefined;
 if(validateJsonSchemaSubset(targetCacheSchema,row,targetCacheSchema).length)throw new Error(`Closed repository cache decision required: ${task.id}`);
 return (row as {class:string}).class;
}

/** 🏷️ Decides whether an action joins Bazel's action cache and records why it stays out; it fails closed: the plugin's cache class forbids replay for every class but deterministic, an explicit `cache: true` against such a class is a conflict, and only a deterministic or explicit `cache: true` with declared outputs is cacheable (a pure ordering task is always cacheable). */
export function bazelCacheDecisionV1(task:BazelTaskV1,skipCache:boolean):BazelCacheDecisionV1{
 const klass=bazelTargetCacheClassV1(task),forbidden=klass!==undefined&&klass!=="deterministic",noop=task.definition.execution.kind==="noop";
 if(forbidden&&!noop&&task.definition.cache===true)throw new Error(`Conflicting cache declaration: ${task.id} declares cache with class ${klass}`);
 const decision:BazelCacheDecisionV1=task.definition.continuous?{cache:false,reason:"continuous"}:skipCache?{cache:false,reason:"skipped"}:noop?{cache:true}:forbidden?{cache:false,reason:klass as BazelUncachedReasonV1}:task.definition.cache===false?{cache:false,reason:"declared"}:task.definition.cache!==true&&klass!=="deterministic"?{cache:false,reason:"undeclared"}:!Array.isArray(task.definition.outputs)?{cache:false,reason:"unbounded"}:{cache:true};
 if(validateJsonSchemaSubset(cacheDecisionSchema,decision).length)throw new Error(`Closed Bazel cache decision required: ${task.id}`);
 return decision;
}

/** 📣 Reports why actions stay out of Bazel's action cache and which targets declare cache without outputs: `unbounded` lists them per owning project so manifest owners add real outputs or an explicit empty `outputs` array for a pure check. */
export function bazelUncachedReportV1(plan:BazelTaskPlanV1,emitted:ReadonlyMap<string,BazelActionV1>):BazelUncachedReportV1{
 const uncached:Record<string,number>={},owners=new Map<string,{project:string;root:string;targets:{target:string;declared:"cache"|"class"}[]}>();let cached=0;
 for(const action of emitted.values()){
  if(action.cache.cache){cached++;continue;}
  const reason=action.cache.reason!;uncached[reason]=(uncached[reason]??0)+1;
  if(reason!=="unbounded")continue;
  const task=plan.tasks[action.id]!,owner=owners.get(task.project)??{project:task.project,root:task.root,targets:[]};owners.set(task.project,owner);owner.targets.push({target:task.target+(task.configuration?":"+task.configuration:""),declared:task.definition.cache===true?"cache":"class"});
 }
 const order=(left:string,right:string)=>left<right?-1:left>right?1:0,report:BazelUncachedReportV1={version:1,actions:emitted.size,cached,uncached:Object.fromEntries(Object.entries(uncached).sort(([left],[right])=>order(left,right))),unbounded:[...owners.values()].sort((left,right)=>order(left.project,right.project)).map(owner=>({...owner,targets:owner.targets.sort((left,right)=>order(left.target,right.target))}))};
 if(validateJsonSchemaSubset(uncachedReportSchema,report).length)throw new Error("Closed Bazel uncached report required");
 return report;
}

/** 🔖 Resolves each emitted action to the dependency outputs it consumes; pure ordering tasks vanish behind their producers and unselected ones are never emitted. */
export function bazelActionsV1(graph:BuildGraphV1,plan:BazelTaskPlanV1,skipCache:boolean):Map<string,BazelActionV1>{
 const byLabel=(left:BazelTaskV1,right:BazelTaskV1)=>left.label<right.label?-1:left.label>right.label?1:0,producers=new Map<string,BazelTaskV1[]>(),emitted=new Map<string,BazelActionV1>();
 const direct=(task:BazelTaskV1):BazelTaskV1[]=>{
  const known=producers.get(task.id);if(known)return known;
  const seen=new Set<string>(),found:BazelTaskV1[]=[],visit=(owner:BazelTaskV1)=>{for(const id of owner.dependencies){const dependency=plan.tasks[id]!;if(dependency.definition.continuous||seen.has(id))continue;seen.add(id);if(dependency.definition.execution.kind==="noop")visit(dependency);else found.push(dependency);}};
  visit(task);found.sort(byLabel);producers.set(task.id,found);return found;
 };
 const closure=(task:BazelTaskV1):BazelTaskV1[]=>{const found=new Map<string,BazelTaskV1>(),pending=[...direct(task)];while(pending.length){const next=pending.pop()!;if(found.has(next.id))continue;found.set(next.id,next);pending.push(...direct(next));}return [...found.values()].sort(byLabel);};
 const transitive=(task:BazelTaskV1)=>bazelTaskInputsV1(graph,task,"ignore").some(input=>typeof input==="object"&&input!==null&&"dependentTasksOutputFiles" in input&&(input as {transitive?:unknown}).transitive===true);
 for(const task of Object.values(plan.tasks)){
  const continuous=task.definition.continuous===true,pure=!continuous&&task.definition.execution.kind==="noop";
  if(pure&&!plan.selected.includes(task.id))continue;
  const kind=continuous?"launch":pure?"noop":"task",restore=kind!=="noop"&&transitive(task)?closure(task):direct(task);
  emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache:bazelCacheDecisionV1(task,skipCache)});
 }
 return emitted;
}

/** 🧷 Compiles one input pattern once, keeps a broad owner for unfamiliar syntax and exposes the literal prefix every matching path starts with. */
function inputMatcher(pattern:string,broad=true):{prefix:string;match:(path:string)=>boolean}{
 const normalized=pattern.replace(/^\//u,""),extended=normalized.search(/[+@?!*]\(/u),prefix=(extended<0?normalized:normalized.slice(0,extended).replace(/[^/]*$/u,"")).split(/[?*{[\\]/u)[0]??"";
 if(extended<0)try{const glob=new Bun.Glob(normalized);return{prefix,match:path=>path.startsWith(prefix)&&glob.match(path)};}catch{}
 return{prefix,match:path=>broad&&path.startsWith(prefix)};
}

/** 🔑️ One cacheable unit of a task's source selection: an include pattern minus exactly the exclusions that can overlap it, so every task and project that names the same pattern shares one cover. */
export interface BazelSourceKeyV1{include:string;excludes:readonly string[]}

/** 🧮️ A sorted workspace inventory whose distinct patterns each select by a binary-searched prefix range and are cached: `keys` splits a task's inputs into shareable units, `select` resolves one unit, `selectAll` resolves a whole task. */
export interface BazelSourceIndexV1{readonly files:readonly string[];keys(inputs:readonly unknown[]):{keys:BazelSourceKeyV1[];extra:string[]};select(key:BazelSourceKeyV1):Uint32Array;selectAll(inputs:readonly unknown[]):{marks:Uint8Array;extra:string[]};}

/** 🧮️ Indexes an inventory once. An exclusion can only remove files an include pattern matches when their literal prefixes are prefixes of one another, so a unit carries just those exclusions and equals the task-wide selection exactly; literal inputs and bundled manifests outside the inventory are reported as `extra`. */
export function bazelSourceIndexV1(inventory:readonly string[]):BazelSourceIndexV1{
 const files=[...new Set(inventory)].sort(),position=new Map(files.map((path,index)=>[path,index])),patterns=new Map<string,{prefix:string;match:(path:string)=>boolean;indices:Uint32Array}>(),selected=new Map<string,Uint32Array>(),literal=(value:string)=>!/[?*{[]/u.test(value);
 const lower=(prefix:string)=>{let low=0,high=files.length;while(low<high){const middle=(low+high)>>>1;if(files[middle]!<prefix)low=middle+1;else high=middle;}return low;};
 const compiled=(pattern:string,broad:boolean)=>{
  const key=(broad?"+":"-")+pattern;let row=patterns.get(key);
  if(!row){const{prefix,match}=inputMatcher(pattern,broad),found:number[]=[];for(let index=lower(prefix);index<files.length&&files[index]!.startsWith(prefix);index++)if(match(files[index]!))found.push(index);row={prefix,match,indices:Uint32Array.from(found)};patterns.set(key,row);}
  return row;
 };
 const keys=(inputs:readonly unknown[])=>{
  const strings=inputs.filter((value):value is string=>typeof value==="string"),included=[...new Set(strings.filter(value=>!value.startsWith("!")))],excluders=[...new Set(strings.filter(value=>value.startsWith("!")).map(value=>value.slice(1)))].map(pattern=>({pattern,...compiled(pattern,false)})),extra=new Set<string>();
  const units:BazelSourceKeyV1[]=included.filter(include=>compiled(include,true).indices.length||literal(include)&&position.has(include)).map(include=>{const{prefix}=compiled(include,true);return{include,excludes:excluders.filter(row=>row.indices.length&&(row.prefix.startsWith(prefix)||prefix.startsWith(row.prefix))).map(row=>row.pattern).sort()};});
  for(const value of included)if(literal(value)&&!position.has(value)&&!excluders.some(row=>row.match(value)))extra.add(value);
  for(const value of inputs.some(input=>typeof input==="object"&&input!==null&&"externalDependencies" in input)?actions.sources.bundled:[])if(position.has(value))units.push({include:value,excludes:[]});else extra.add(value);
  return{keys:[...new Map(units.map(unit=>[JSON.stringify(unit),unit])).values()],extra:[...extra].sort()};
 };
 const select=(unit:BazelSourceKeyV1):Uint32Array=>{
  const text=JSON.stringify(unit);let known=selected.get(text);
  if(!known){const excluders=unit.excludes.map(pattern=>compiled(pattern,false)),found=new Set<number>();for(const index of compiled(unit.include,true).indices)if(!excluders.some(row=>row.match(files[index]!)))found.add(index);const at=literal(unit.include)?position.get(unit.include):undefined;if(at!==undefined&&!excluders.some(row=>row.match(unit.include)))found.add(at);known=Uint32Array.from([...found].sort((left,right)=>left-right));selected.set(text,known);}
  return known;
 };
 return{files,keys,select,selectAll(inputs){const{keys:units,extra}=keys(inputs),marks=new Uint8Array(files.length);for(const unit of units)for(const index of select(unit))marks[index]=1;return{marks,extra};}};
}

/** 🧿 Selects the workspace paths that declared input patterns, literal inputs and bundled dependency manifests name. */
export function bazelTaskSourcePathsV1(inputs:readonly unknown[],files:readonly string[]):string[]{
 const index=bazelSourceIndexV1(files),{marks,extra}=index.selectAll(inputs);
 return [...new Set([...index.files.filter((_path,position)=>marks[position]),...extra])].sort();
}

/** 🚧️ Splits selected paths into files Bazel can declare by label and paths whose identity the fingerprint must hash instead. */
export async function bazelSourceClassesV1(root:string,paths:readonly string[],classes:Map<string,Promise<"file"|"hash"|"absent">>,boundaries:Map<string,Promise<boolean>>,invocation:ScriptInvocation):Promise<{files:string[];hashed:string[]}>{
 const boundary=(directory:string)=>{let known=boundaries.get(directory);if(!known){known=(async()=>{for(const name of actions.sources.boundaries)try{await lstat(join(root,directory,name));return true;}catch(error){if(!["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw error;}return false;})();boundaries.set(directory,known);}return known;};
 const classify=async(path:string):Promise<"file"|"hash"|"absent">=>{
  let info;try{info=await lstat(join(root,path));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return "absent";throw error;}
  if(info.isSymbolicLink())try{info=await stat(join(root,path));}catch(error){if(["ENOENT","ELOOP","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return "hash";throw error;}
  const parts=path.split("/");if(!info.isFile()||!sourceLabel.test(path)||parts.some(part=>part===""||part==="."||part===".."))return "hash";
  for(let depth=0;depth<parts.length;depth++)if(await boundary(parts.slice(0,depth).join("/")))return "hash";
  return "file";
 };
 const pending=paths.filter(path=>!classes.has(path));
 for(let offset=0;offset<pending.length;offset+=256){checkScriptInvocation(invocation);const batch=pending.slice(offset,offset+256);for(const path of batch)classes.set(path,classify(path));await Promise.all(batch.map(path=>classes.get(path)));}
 const files:string[]=[],hashed:string[]=[];for(const path of paths){const kind=await classes.get(path)!;if(kind==="file")files.push(path);else if(kind==="hash")hashed.push(path);}
 return {files,hashed};
}

/** 🏷️ Names a checkout file by the in-package Bazel label that reaches it through the source link, emoji and other non-ASCII names are valid Bazel target-name bytes, only package names must be ASCII. */
export function bazelSourceLabelV1(path:string):string{return actions.sources.link+"/"+path;}

/** 🗂️ One generated filegroup: nested groups and the individual files it lists. */
export interface BazelSourceGroupV1{groups:readonly string[];files:readonly string[]}

/** 🗂️ Covers selections of declarable files with per-directory filegroups. A directory whose every file is selected is one group (a project root is one group), a partly selected directory lists its selected files and recurses, and single-child chains collapse, so each file is listed once in its directory group however many tasks select it. `universe` is the sorted list of declarable files. `cover` turns a dense selection over it into the groups and loose files that expand to exactly that selection, `merge` unites covers, and `compose` names a cover as one shared group (`k` for a pattern unit shared by many tasks, `i` for a task), or returns the lone group a cover already is. */
export function bazelSourceCoverV1(universe:readonly string[]):{groups:Map<string,BazelSourceGroupV1>;cover(selected:ArrayLike<number>):BazelSourceGroupV1;merge(covers:readonly BazelSourceGroupV1[]):BazelSourceGroupV1;compose(cover:BazelSourceGroupV1,kind:"i"|"k"):string|undefined}{
 type Directory={path:string;low:number;high:number;files:number[];children:string[];rep?:Directory};
 const directories=new Map<string,Directory>(),groups=new Map<string,BazelSourceGroupV1>(),prefix=new Int32Array(universe.length+1),order=(left:string,right:string)=>left<right?-1:left>right?1:0;
 const ensure=(path:string,index:number):Directory=>{let row=directories.get(path);if(!row){row={path,low:index,high:index+1,files:[],children:[]};directories.set(path,row);if(path){const cut=path.lastIndexOf("/");ensure(cut<0?"":path.slice(0,cut),index).children.push(path);}}return row;};
 universe.forEach((file,index)=>{const cut=file.lastIndexOf("/");let path=cut<0?"":file.slice(0,cut);ensure(path,index).files.push(index);for(;;){directories.get(path)!.high=index+1;if(!path)break;const up=path.lastIndexOf("/");path=up<0?"":path.slice(0,up);}});
 for(const row of directories.values())row.children.sort(order);
 const representative=(row:Directory):Directory=>row.rep??=!row.files.length&&row.children.length===1?representative(directories.get(row.children[0]!)!):row;
 const define=(row:Directory):string=>{
  const owner=representative(row),name="d_"+hash(owner.path).slice(0,16);
  if(!groups.has(name))groups.set(name,{groups:owner.children.map(child=>define(directories.get(child)!)),files:owner.files.map(index=>universe[index]!)});
  return name;
 };
 const sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);
 return{groups,
  cover(selected){
   for(let index=0;index<universe.length;index++)prefix[index+1]=prefix[index]!+(selected[index]?1:0);
   const names:string[]=[],files:string[]=[],visit=(row:Directory)=>{
    const count=prefix[row.high]!-prefix[row.low]!;if(!count)return;
    if(count===row.high-row.low){names.push(define(row));return;}
    for(const index of row.files)if(selected[index])files.push(universe[index]!);
    for(const child of row.children)visit(directories.get(child)!);
   };
   if(universe.length)visit(directories.get("")!);
   return{groups:sorted(names),files:sorted(files)};
  },
  merge(covers){return{groups:sorted(covers.flatMap(cover=>cover.groups)),files:sorted(covers.flatMap(cover=>cover.files))};},
  compose(cover,kind){
   if(!cover.groups.length&&!cover.files.length)return undefined;
   if(cover.groups.length===1&&!cover.files.length)return cover.groups[0]!;
   const name=kind+"_"+hash(cover.groups.join("\0")+"\u0001"+cover.files.join("\0")).slice(0,16);groups.set(name,cover);return name;
  }};
}

/** 📑 Renders the BUILD file whose declarations alone define every action, its inputs and its cache policy. */
export function renderBazelBuildV1(input:{actions:readonly (BazelActionV1&{sources?:string})[];groups:ReadonlyMap<string,BazelSourceGroupV1>;bun:string;sourceIdentity:string;extension:string}):string{
 const quote=(value:string)=>JSON.stringify(value),list=(values:readonly string[])=>values.length?"[\n"+values.map(value=>"        "+quote(value)+",").join("\n")+"\n    ]":"[]",labels=(values:readonly string[])=>list(values.map(value=>":"+value)),order=(left:string,right:string)=>left<right?-1:left>right?1:0;
 const lines=['load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task")',""];
 for(const[name,sources]of [...input.groups].sort(([left],[right])=>order(left,right)))lines.push("filegroup(",`    name = ${quote(name)},`,`    srcs = ${list([...sources.groups.map(group=>":"+group),...sources.files.map(bazelSourceLabelV1)])},`,")","");
 for(const action of [...input.actions].sort((left,right)=>order(left.label,right.label))){
  const attributes:[string,string][]=[["name",quote(action.label)]];
  if(action.kind==="noop")attributes.push(["deps",labels(action.restore)]);
  else{
   const launch=action.kind==="launch",reason=launch?actions.kinds.launcher.reason:actions.kinds.task.reason;
   attributes.push(["manifest",quote(action.label+".json")]);
   if(!launch)attributes.push(["fingerprint",quote(action.label+".input")]);
   if(action.sources)attributes.push(["srcs",list([":"+action.sources])]);
   attributes.push(["deps",labels(action.restore)],["runner",quote("📜️script.ts")],["source",quote("implementation.json")],["source_identity",quote(input.sourceIdentity)],["bun",quote(input.bun)]);
   if(launch)attributes.push(["extension",quote(input.extension)]);else attributes.push(["identity",quote(action.id)]);
   attributes.push(["cache",action.cache.cache?"True":"False"]);
   if(action.cache.reason)attributes.push(["uncached_reason",quote(action.cache.reason)]);
   attributes.push(["host_state",quote(reason)]);
  }
  lines.push(`${action.kind==="noop"?"repo_noop":action.kind==="launch"?"repo_launch":"repo_task"}(`,...attributes.map(([key,value])=>`    ${key} = ${value},`),")","");
 }
 return lines.join("\n");
}

/** 🗝️ Builds the content-stable task manifest: it names the task and the implementation identity but no namespace, workspace or runtime path. */
export function bazelManifestV1(task:BazelTaskV1,implementation:Pick<BazelImplementationV1,"sha256"|"loaderSha256">,orchestration?:object):BazelTaskManifestV1{
 const manifest:BazelTaskManifestV1={version:1,task,implementation:{version:1,sha256:implementation.sha256,loaderSha256:implementation.loaderSha256},...(orchestration?{orchestration}:{})};
 if(validateJsonSchemaSubset(manifestSchema,manifest).length)throw new Error(`Closed Bazel task manifest required: ${task.id}`);
 return manifest;
}

/** 🗝️ Refuses any manifest that is not closed or that names a different captured implementation. */
function requireBazelManifestV1(manifest:unknown,expected:BazelImplementationV1):asserts manifest is BazelTaskManifestV1{
 if(validateJsonSchemaSubset(manifestSchema,manifest).length)throw new Error("Closed Bazel task manifest required");
 const implementation=(manifest as BazelTaskManifestV1).implementation;if(implementation.sha256!==expected.sha256||implementation.loaderSha256!==expected.loaderSha256)throw new Error("Exact captured Bazel implementation required");
}

/** 🪢 Publishes the live runtime beside the immutable implementation, outside every action key. */
async function writeBazelRuntimeV1(workspace:string,root:string,environment:NodeJS.ProcessEnv):Promise<void>{
 const value:BazelRuntimeV1={version:1,workspace:root,environment:Object.fromEntries(Object.entries(environment).filter((entry):entry is [string,string]=>entry[1]!==undefined))};
 if(validateJsonSchemaSubset(runtimeSchema,value).length)throw new Error("Closed Bazel runtime channel required");
 const path=join(workspace,"runtime.json");await writeFile(path,JSON.stringify(value),{mode:0o600});await chmod(path,0o600);
}

/** 🪢 Reads the runtime channel that lives beside the manifest's own immutable implementation. */
async function readBazelRuntimeV1(manifestPath:string,implementation:BazelImplementationV1,deadline:()=>void):Promise<BazelRuntimeV1>{
 if(dirname(manifestPath)!==dirname(implementation.snapshot))throw new Error("Bazel runtime channel requires the manifest's own workspace");
 const value=JSON.parse((await readBazelBirthFileV1(join(dirname(implementation.snapshot),"runtime.json"),birthSchema.definitions.Protocol.const.maximumRuntimeBytes,deadline)).toString("utf8"));
 if(validateJsonSchemaSubset(runtimeSchema,value).length)throw new Error("Closed Bazel runtime channel required");
 return value;
}

/** 🔌 Links the real checkout into the generated package so Bazel can name every declared source by an in-package label; no `.bazelignore` is written (listing the link there removes it from the execroot and disables Bazel caching); queries name the root package with `//:all`, which never crawls the checkout. */
export async function linkBazelSourcesV1(root:string,workspace:string):Promise<void>{
 const link=join(workspace,actions.sources.link);await rm(join(workspace,".bazelignore"),{force:true});
 try{if(await realpath(link)===await realpath(root))return;await rm(link,{force:true});}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 await symlink(root,link,process.platform==="win32"?"junction":"dir");
}

/** 🧊️ Identifies the full bounded captured executable closure. */
export async function bazelImplementationIdentityV1(root:string,entry:string,invocation:ScriptInvocation):Promise<string>{return buildSourceClosureIdentityV1(await captureBuildSourceClosureV1(root,entry,invocation));}

const cargoDirectoryKeys=["CARGO_TARGET_DIR","CARGO_BUILD_TARGET_DIR","CARGO_BUILD_BUILD_DIR"];

/** 🗂️ Resolves the Cargo directories a task declares for itself; they are task data, not the caller's artifact authority, so they never pass the issuance check and are laid over its result. */
export function bazelTaskCargoDirectoriesV1(workspace:string,task:BazelTaskV1):Record<string,string>{
 return Object.fromEntries(cargoDirectoryKeys.flatMap(key=>{const value=task.definition.execution.env?.[key];return typeof value==="string"?[[key,resolve(workspace,value)]]:[];}));
}

/** 🔑️ Issues a task environment under the caller's authority and keeps the task's own declared Cargo directories. */
export function issueBazelTaskEnvironmentV1(workspace:string,invocation:ScriptInvocation,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 return {...issueBuildTaskEnvironmentV1(workspace,invocation,task.definition.execution,task.args,environment),...bazelTaskCargoDirectoriesV1(workspace,task)};
}

/** 🧰️ Shares task environment overrides between source fingerprints and actual command execution. */
export function bazelTaskBaseEnvironmentV1(workspace:string,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 return {...environment,...Object.fromEntries(Object.entries(task.definition.execution.env??{}).filter(([key])=>!cargoDirectoryKeys.includes(key))),SEMIO_BUILD_TOOL:"bazel",SEMIO_WORKSPACE_ROOT:workspace,SEMIO_TASK_PROJECT:task.project,SEMIO_TASK_TARGET:task.target,...(task.configuration?{SEMIO_TASK_CONFIGURATION:task.configuration}:{}),...(task.target==="repo-exec"?buildExecEnvironmentV1(task.project,task.root):{})};
}

/** 🧮️ Fingerprints only what Bazel cannot read from declared files: host identity, declared environment and runtime commands (each command runs once per distinct text and build mode, like Nx's runtime hasher), and paths without a declarable label. */
async function taskFingerprint(root:string,graph:BuildGraphV1,task:BazelTaskV1,files:string[],memo:Map<string,Promise<string>>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,cached:boolean,implementation:string,exactClosures:BazelClosuresV1):Promise<string>{
 const inputs=cached&&task.definition.execution.kind!=="noop"?bazelTaskInputsV1(graph,task,exactClosures):[],digest=createHash("sha256");
 const original=issueBazelTaskEnvironmentV1(root,invocation,task,bazelTaskBaseEnvironmentV1(root,task,process.env));digest.update(JSON.stringify({platform:process.platform,arch:process.arch,bun:process.versions.bun,mode:original.SEMIO_BUILD_MODE}));digest.update(implementation);
 if(!cached)digest.update(randomUUID());
 for(const path of [...files].sort()){checkScriptInvocation(invocation);let identity=memo.get(path);if(!identity){identity=bazelSourceIdentityV1(join(root,path),invocation.control.signal);memo.set(path,identity);}digest.update(path+"\0"+await identity+"\0");}
 for(const input of inputs){if(typeof input!=="object"||!input)continue;const row=input as Record<string,unknown>;if(typeof row.runtime==="string"){const key="\0runtime\0"+row.runtime+"\0"+original.SEMIO_BUILD_MODE;let stdout=memo.get(key);if(!stdout){stdout=captureOwnedCommandV1(process.execPath,["exec",row.runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original});memo.set(key,stdout);}digest.update(JSON.stringify({runtime:row.runtime,stdout:await stdout}));}else if(typeof row.env==="string")digest.update(JSON.stringify({env:row.env,value:original[row.env]??null}));}
 return digest.digest("hex");
}

/** ✍️ Keeps unchanged graph inputs stable for Bazel's own incremental analysis. */
async function writeChanged(path:string,contents:string):Promise<void>{
 try{if(await readFile(path,"utf8")===contents)return;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 await mkdir(dirname(path),{recursive:true});const temporary=path+`.${process.pid}.tmp`;await writeFile(temporary,contents);await rename(temporary,path);
}

/** 🔗️ Refuses a different captured executable at every private child boundary. */
function requireBazelImplementationV1(value:unknown,expected:BazelImplementationV1):asserts value is BazelImplementationV1{if(validateJsonSchemaSubset(implementationSchema,value).length||JSON.stringify(value)!==JSON.stringify(expected)||![expected.snapshot,expected.runner].every(path=>isAbsolute(path)&&path.length<=256)||dirname(expected.snapshot)!==dirname(expected.runner))throw new Error("Exact captured Bazel implementation required");}

/** ✍️ Publishes bounded readonly inputs without following a previous leaf or replacing primary failures. */
async function writeBazelImmutableInputV1(path:string,contents:string,maximumBytes:number,original:ScriptInvocation):Promise<void>{
 checkScriptInvocation(original);if(path.length>256||Buffer.byteLength(contents)>maximumBytes)throw new Error("Finite immutable Bazel input required");let before:import("node:fs").BigIntStats|undefined;try{before=await lstat(path,{bigint:true});checkScriptInvocation(original);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n)throw new Error("Owned immutable Bazel input required");const prior=await readBazelBirthFileV1(path,maximumBytes,()=>checkScriptInvocation(original));if(prior.equals(Buffer.from(contents)))return;}catch(error){checkScriptInvocation(original);if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}const temporary=path+"."+randomUUID().slice(0,8)+".tmp";if(temporary.length>256)throw new Error("Immutable Bazel input path exceeds 256 characters");const file=await open(temporary,"wx",0o400);await withBazelTaskCleanupV1(async()=>{await withBazelTaskCleanupV1(async()=>{const bytes=Buffer.from(contents);for(let offset=0;offset<bytes.length;){checkScriptInvocation(original);const count=Math.min(65536,bytes.length-offset);let written=0;while(written<count){const row=await file.write(bytes,offset+written,count-written,offset+written);checkScriptInvocation(original);if(!row.bytesWritten)throw new Error("Bazel immutable input write made no progress");written+=row.bytesWritten;}offset+=count;}await file.sync();checkScriptInvocation(original);},()=>file.close());if(before){const expected=before,previous=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));await withBazelTaskCleanupV1(async()=>{const facts=await previous.stat({bigint:true});checkScriptInvocation(original);if(!facts.isFile()||facts.dev!==expected.dev||facts.ino!==expected.ino||facts.nlink!==1n)throw new Error("Immutable Bazel input changed before replacement");await previous.chmod(0o600);checkScriptInvocation(original);},()=>previous.close());}checkScriptInvocation(original);await rename(temporary,path);checkScriptInvocation(original);},async()=>{try{await rm(temporary);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}});
}

/** 📦️ Publishes one finite immutable executable input set while the workspace writer is held. */
export async function publishBazelImplementationV1(root:string,workspace:string,original:ScriptInvocation):Promise<BazelImplementationV1>{
 const captured=await captureBuildSourceClosureV1(root,slash(relative(root,import.meta.path)),original),encoded=JSON.stringify(captured),value:BazelImplementationV1={version:1,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:buildSourceClosureIdentityV1(captured),loaderSha256:hash(buildSourceClosureLoaderV1)};requireBazelImplementationV1(value,value);
 await writeBazelImmutableInputV1(value.snapshot,encoded,25165824,original);await writeBazelImmutableInputV1(value.runner,buildSourceClosureLoaderV1,25165824,original);
 return value;
}

const STARLARK_RULES=`def _requirements(ctx):
    requirements = {"no-remote-exec": "1"}
    if ctx.attr.host_state:
        requirements["no-sandbox"] = "1"
    if not ctx.attr.cache:
        if not ctx.attr.uncached_reason:
            fail("An uncached action requires a reason: " + ctx.label.name)
        requirements["no-cache"] = "1"
    return requirements

def _task_impl(ctx):
    output = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    snapshots = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    args = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-action", ctx.file.manifest.path, output.path] + [snapshot.path for snapshot in snapshots]
    ctx.actions.run(executable = ctx.attr.bun, arguments = args, inputs = depset([ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "RepoTask", progress_message = ctx.attr.identity, execution_requirements = _requirements(ctx), use_default_shell_env = True)
    return [DefaultInfo(files = depset([output]))]

repo_task = rule(implementation = _task_impl, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})

def _noop_impl(ctx):
    marker = ctx.actions.declare_file(ctx.label.name + ".noop")
    ctx.actions.write(marker, "")
    return [DefaultInfo(files = depset([marker], transitive = [dep[DefaultInfo].files for dep in ctx.attr.deps]))]

repo_noop = rule(implementation = _noop_impl, attrs = {"deps": attr.label_list()})

def _launch_impl(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ctx.attr.extension)
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-launcher", ctx.file.manifest.path, launcher.path], inputs = [ctx.file.manifest, ctx.file.runner, ctx.file.source], outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = _requirements(ctx))
    dependencies = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    return [DefaultInfo(executable = launcher, runfiles = ctx.runfiles(files = dependencies + [ctx.file.manifest, ctx.file.runner, ctx.file.source]))]

repo_launch = rule(implementation = _launch_impl, executable = True, attrs = {"manifest": attr.label(allow_single_file = True), "deps": attr.label_list(), "bun": attr.string(mandatory = True), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "extension": attr.string(default = ""), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})
`;

/** 📨️ Carries finite task data to the captured planner without granting authority through JSON. */
export async function publishBazelGraphV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl):Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const namespace=hash(JSON.stringify({selected:planBazelTasksV1(graph,request).selected,args:request.args,configuration:request.configuration})).slice(0,16),workspace=join(storage.paths.workspace,namespace);await mkdir(workspace,{recursive:true,mode:0o700});const implementation=await publishBazelImplementationV1(root,workspace,invocation),input=join(workspace,"planner-input.json"),output=join(workspace,"planner-output.json"),value=JSON.parse(JSON.stringify({version:1,root,workspace,graph,request,implementation}));if(validateJsonSchemaSubset(plannerSchema,value).length)throw new Error("Closed captured Bazel planner data required");const encoded=JSON.stringify(value);if(Buffer.byteLength(encoded)>plannerSchema.definitions.Policy.const.maximumDataBytes)throw new Error("Bazel planner data exceeds its finite bound");return withBazelTaskCleanupV1(async()=>{await writeBazelImmutableInputV1(input,encoded,plannerSchema.definitions.Policy.const.maximumDataBytes,invocation);checkScriptInvocation(invocation);const inputIdentity=hash(encoded);await advanceScriptInvocation(invocation,"bazel:captured-planner","running");await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,"planner",input,inputIdentity,output],root,"bazel:captured-planner",compilerAdmissionBudgetV1(phaseControl),{env:scriptInvocationEnvironment(invocation,process.env),signal:phaseControl.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:captured-planner","running")});const bytes=await readBazelBirthFileV1(output,plannerSchema.definitions.Policy.const.maximumDataBytes,()=>checkScriptInvocation(invocation)),result=JSON.parse(bytes.toString("utf8"));if(validateJsonSchemaSubset(plannerSchema.definitions.Result,result).length||result.workspace!==workspace||JSON.stringify(result.implementation)!==JSON.stringify(implementation)||result.plan.selected.some((id:string)=>!result.plan.tasks[id]))throw new Error("Closed captured Bazel planner result required");checkScriptInvocation(invocation);return result;},async()=>{const outcomes=await Promise.allSettled([input,output].map(path=>rm(path,{force:true})));const failed=outcomes.find((row):row is PromiseRejectedResult=>row.status==="rejected");if(failed)throw failed.reason;});
}

/** 🏗️ Publishes native Bazel actions from the captured semantic implementation. */
async function publishCapturedBazelGraphV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,recheck:()=>void):Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
 const plan=planBazelTasksV1(graph,request),namespace=hash(JSON.stringify({selected:plan.selected,args:request.args,configuration:request.configuration})).slice(0,16),workspace=join(storage.paths.workspace,namespace);
 return generateBazelWorkspaceV1(root,graph,request,plan,workspace,invocation,phaseControl,implementation,recheck);
}

/** 🪡 Writes the complete Bazel package for a planned selection into a workspace directory: manifests, fingerprints, source links, rules and BUILD, independent of storage and of any Bazel server. */
export async function generateBazelWorkspaceV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,plan:BazelTaskPlanV1,workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1):Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>{
 const emitted=bazelActionsV1(graph,plan,request.skipCache===true),exactClosures=await loadBazelClosuresV1(bazelClosureMarkersForPlanV1(graph,Object.values(plan.tasks)),closureFetch??managedBazelClosureFetchV1(root,phaseControl),workspaceBazelLockFilesV1(root)),declared=[...emitted.values()].filter(action=>action.kind==="task"&&action.cache.cache),inputs=new Map(declared.map(action=>[action.id,bazelTaskInputsV1(graph,plan.tasks[action.id]!,exactClosures)])),bundled=(values:unknown[])=>values.some(input=>typeof input==="object"&&input!==null&&"externalDependencies" in input)?actions.sources.bundled:[];
 const files=await sourceFiles(root,invocation,[...new Set([...inputs.values()].flatMap(values=>[...values.filter((value):value is string=>typeof value==="string"&&!value.startsWith("!")),...bundled(values)]))]);
 await mkdir(workspace,{recursive:true,mode:0o700});await chmod(workspace,0o700);if(dirname(implementation.snapshot)!==workspace)throw new Error("Captured Bazel planner workspace changed");recheck();await writeChanged(join(workspace,"MODULE.bazel"),BAZEL_MODULE);await writeChanged(join(workspace,"tasks.bzl"),STARLARK_RULES);await linkBazelSourcesV1(root,workspace);
 const index=bazelSourceIndexV1(files),union=new Uint8Array(index.files.length),literals=new Set<string>(),units=new Map<string,BazelSourceKeyV1>(),keyed=new Map<string,ReturnType<typeof index.keys>>(),classes=new Map<string,Promise<"file"|"hash"|"absent">>(),boundaries=new Map<string,Promise<boolean>>(),memo=new Map<string,Promise<string>>(),sources=new Map<string,string>(),hashed=new Map<string,string[]>();
 for(const action of declared){checkScriptInvocation(invocation);const values=inputs.get(action.id)!,text=JSON.stringify(values);if(!keyed.has(text)){const row=index.keys(values);keyed.set(text,row);for(const unit of row.keys)units.set(JSON.stringify(unit),unit);for(const path of row.extra)literals.add(path);}}
 for(const unit of units.values())for(const position of index.select(unit))union[position]=1;
 const classified=await bazelSourceClassesV1(root,[...index.files.filter((_path,position)=>union[position]),...[...literals].sort()],classes,boundaries,invocation),declarable=new Set(classified.files),flagged=new Set(classified.hashed),universe=index.files.filter(path=>declarable.has(path)),compact=new Int32Array(index.files.length).fill(-1);
 for(let position=0,next=0;position<index.files.length;position++)if(declarable.has(index.files[position]!))compact[position]=next++;
 const tree=bazelSourceCoverV1(universe),shared=new Map<string,{cover:BazelSourceGroupV1;hashed:string[]}>(),composed=new Map<string,{source?:string;hashed:string[]}>();
 const unitCover=(unit:BazelSourceKeyV1)=>{const text=JSON.stringify(unit);let row=shared.get(text);if(!row){const chosen=new Uint8Array(universe.length),hashing:string[]=[];for(const position of index.select(unit)){const path=index.files[position]!;if(compact[position]!>=0)chosen[compact[position]!]=1;else if(flagged.has(path))hashing.push(path);}const cover=tree.cover(chosen);row={cover:cover.groups.length+cover.files.length>2?{groups:[tree.compose(cover,"k")!],files:[]}:cover,hashed:hashing};shared.set(text,row);}return row;};
 for(const action of declared){
  checkScriptInvocation(invocation);const text=JSON.stringify(inputs.get(action.id)!);let row=composed.get(text);
  if(!row){const{keys:units,extra}=keyed.get(text)!,parts=units.map(unitCover);row={source:tree.compose(tree.merge([...parts.map(part=>part.cover),{groups:[],files:extra.filter(path=>declarable.has(path))}]),"i"),hashed:[...new Set([...parts.flatMap(part=>part.hashed),...extra.filter(path=>flagged.has(path))])].sort()};composed.set(text,row);}
  if(row.source)sources.set(action.id,row.source);hashed.set(action.id,row.hashed);
 }
 let authorized:Set<string>|undefined;
 for(const action of emitted.values()){
  if(action.kind==="noop")continue;
  checkScriptInvocation(invocation);const task=plan.tasks[action.id]!;const metadata=graph.projects[task.project]?.targets[task.target]?.metadata as {semio?:{execution?:{kind?:string;children?:unknown}}}|undefined,delegation=metadata?.semio?.execution,orchestration=delegation?.kind==="orchestration"&&delegation.children!==undefined?buildOrchestrationContextV1(graph.projects,{project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{})}):undefined;
  if(orchestration){if(!authorized){const caller=readIssuedScriptProcessEnvelopeV1(invocation).capabilities as any,{resolveBuildInvocationV1,parseBuildInvocationV1}=await import("../../🧭️invocation/🟦️.ts"),{selectBuildTasksV1}=await import("../../🧭️selection/🟦️.ts");if(caller.tool!=="bazel"||caller.selection?.[0]!=="repo")throw new Error("Original Bazel orchestration selection required");const selected=parseBuildInvocationV1(caller.selection.slice(1),{...process.env,SEMIO_BUILD_TOOL:caller.tool,SEMIO_BUILD_MODE:caller.buildMode}),resolved=resolveBuildInvocationV1(root,selected.args,{...process.env,SEMIO_BUILD_MODE:caller.buildMode}),request=await selectBuildTasksV1(root,graph,resolved.args,invocation);authorized=new Set(Object.keys(planBazelTasksV1(graph,request).tasks));}if(!authorized.has(task.id))throw new Error("Original Bazel orchestration task is unreachable");}
  await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelManifestV1(task,implementation,orchestration)));
  if(action.kind==="task"){const fingerprint=await taskFingerprint(root,graph,task,hashed.get(action.id)??[],memo,invocation,phaseControl,action.cache.cache,implementation.sha256+implementation.loaderSha256,exactClosures);await writeChanged(join(workspace,task.label+".input"),fingerprint);}
 }
 recheck();checkScriptInvocation(invocation);await writeChanged(join(workspace,"BUILD.bazel"),renderBazelBuildV1({actions:[...emitted.values()].map(action=>({...action,...(sources.has(action.id)?{sources:sources.get(action.id)}:{})})),groups:tree.groups,bun:process.execPath,sourceIdentity:implementation.sha256,extension:process.platform==="win32"?".exe":""}));await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,process.env));return {workspace,plan,implementation};
}

/** 📦️ Flattens declared products so cache paths remain short on every host. */
export async function captureBazelSnapshotV1(manifest:BazelSnapshotContextV1,output:string,invocation:ScriptInvocation):Promise<void>{
 const roots=new Set<string>(),entries:SnapshotEntryV1[]=[],environment=()=>issueBazelTaskEnvironmentV1(manifest.workspace,invocation,manifest.task,bazelTaskBaseEnvironmentV1(manifest.workspace,manifest.task,process.env));await mkdir(join(output,"files"),{recursive:true});
 for(const pattern of manifest.task.definition.outputs??[]){const absolute=interpolate(pattern,manifest.workspace,manifest.task,environment),path=slash(relative(manifest.workspace,resolve(manifest.workspace,absolute)));workspacePath(manifest.workspace,path);if(/[?*{[]/u.test(path)){for await(const match of new Bun.Glob(path).scan({cwd:manifest.workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}else try{await lstat(workspacePath(manifest.workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
 const visit=async(path:string):Promise<void>=>{checkScriptInvocation(invocation);await outputAncestry(manifest.workspace,path);const source=workspacePath(manifest.workspace,path),facts=await lstat(source),entry:SnapshotEntryV1={path,kind:facts.isSymbolicLink()?"symlink":facts.isDirectory()?"directory":"file",mode:facts.mode&0o777};if(entry.kind==="symlink"){entry.link=await readlink(source);const linked=await stat(source);checkScriptInvocation(invocation);if(!linked.isFile()&&!linked.isDirectory())throw new Error("Bazel snapshot link requires an existing file or directory target");entry.linkKind=linked.isDirectory()?"directory":"file";}else if(entry.kind==="directory")for(const child of await readdir(source))await visit(slash(join(path,child)));else{entry.file=hash(path).slice(0,32);await copyFile(source,join(output,"files",entry.file));entry.sha256=await fileHash(join(output,"files",entry.file),invocation.control.signal);}entries.push(entry);};
 for(const path of [...roots].sort().filter((path,_index,all)=>!all.some(parent=>parent!==path&&path.startsWith(parent+"/"))))await visit(path);
 await writeFile(join(output,"snapshot.json"),JSON.stringify({version:1,roots:[...roots].sort(),entries} satisfies SnapshotV1));
}

/** 🪪️ Changes only the captured directory inode while retaining the original control. */
async function chmodSnapshotDirectory(path:string,identity:{dev:bigint;ino:bigint},mode:number|((mode:number)=>number),invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
 return withBazelTaskCleanupV1(async()=>{
  checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const facts=await file.stat({bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  const verify=async()=>{const current=await lstat(path,{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!current.isDirectory()||current.isSymbolicLink()||current.dev!==identity.dev||current.ino!==identity.ino)throw new Error("Original Bazel output directory changed during mode update");};
  if(!facts.isDirectory()||facts.dev!==identity.dev||facts.ino!==identity.ino)throw new Error("Original Bazel output directory changed before mode update");
  await verify();checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await file.chmod(typeof mode==="number"?mode:mode(Number(facts.mode)&0o777));checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await verify();
 },async()=>{await file.close();checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);});
}

/** 📥️ Restores cached products before their consumer runs in the real workspace. */
export async function restoreBazelSnapshotV1(root:string,snapshot:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const signal=phaseControl.signal;
 return withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal,control:phaseControl,resources:[{resource:"bazel-snapshot-restore",mode:"exclusive"}]},async()=>{
 await advanceScriptInvocation(invocation,"bazel:snapshot-restore","running");let visited=0;const progress=async()=>{checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(++visited%128===0)await advanceScriptInvocation(invocation,"bazel:snapshot-restore","running");};const contents=JSON.parse(await readFile(join(snapshot,"snapshot.json"),"utf8")) as SnapshotV1;
 if(validateJsonSchemaSubset(snapshotSchema,contents).length)throw new Error("Invalid Bazel output snapshot");
 for(const path of contents.roots)await outputAncestry(root,path);
 const actual=new Set<string>(),pending=[...contents.roots],directories=new Map<string,{dev:bigint;ino:bigint}>(),restored=new Map<string,{dev:bigint;ino:bigint}>();let same=true;
 while(pending.length){await progress();const path=pending.pop()!;try{const stat=await lstat(workspacePath(root,path),{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);actual.add(path);if(stat.isDirectory()&&!stat.isSymbolicLink()){directories.set(path,{dev:stat.dev,ino:stat.ino});for(const child of await readdir(workspacePath(root,path)))pending.push(slash(join(path,child)));}}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;same=false;}}
 for(const entry of contents.entries){await progress();const path=workspacePath(root,entry.path);try{const stat=await lstat(path);if(entry.kind!=="symlink"&&(stat.mode&0o777)!==entry.mode)same=false;if(entry.kind==="directory"?!stat.isDirectory()||stat.isSymbolicLink():entry.kind==="symlink"?!stat.isSymbolicLink()||await readlink(path)!==entry.link:!stat.isFile()||stat.isSymbolicLink()||!entry.sha256||await fileHash(path,signal)!==entry.sha256)same=false;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;same=false;}}
 if(same&&actual.size===contents.entries.length)return;
 for(const[path,identity]of [...directories].sort((a,b)=>a[0].length-b[0].length)){await progress();await outputAncestry(root,path);await chmodSnapshotDirectory(workspacePath(root,path),identity,mode=>mode|0o700,invocation,phaseControl);}
 for(const path of contents.roots){await progress();await rm(workspacePath(root,path),{recursive:true,force:true});}
 for(const entry of [...contents.entries].sort((a,b)=>a.path.length-b.path.length||a.path.localeCompare(b.path))){await progress();const target=workspacePath(root,entry.path);await mkdir(dirname(target),{recursive:true});if(entry.kind==="directory"){await mkdir(target,{recursive:true});const facts=await lstat(target,{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!facts.isDirectory()||facts.isSymbolicLink())throw new Error("Restored Bazel output requires an ordinary directory");restored.set(entry.path,{dev:facts.dev,ino:facts.ino});}else if(entry.kind==="symlink")await symlink(entry.link!,target,entry.linkKind==="directory"?"dir":"file");else{if(!/^[a-f0-9]{32}$/u.test(entry.file??""))throw new Error("Invalid Bazel snapshot file identity");await copyFile(join(snapshot,"files",entry.file!),target);}if(entry.kind==="file")await chmod(target,entry.mode);}
 for(const entry of contents.entries.filter(entry=>entry.kind==="directory").sort((a,b)=>b.path.length-a.path.length)){await progress();await outputAncestry(root,entry.path);await chmodSnapshotDirectory(workspacePath(root,entry.path),restored.get(entry.path)!,entry.mode,invocation,phaseControl);}
 });
}

/** 🚀️ Executes the direct owner command with the original caller's native capabilities. */
async function executeTask(manifest:BazelTaskManifestV1,workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 const {task}=manifest,execution=task.definition.execution;if(execution.kind==="noop")return;
 if(task.definition.continuous)console.log(`[bazel] task-started ${task.id}`);
 if(manifest.orchestration&&JSON.stringify((manifest.orchestration as any).task)!==JSON.stringify({project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{})}))throw new Error("Original Bazel orchestration task changed");
 if(manifest.orchestration){const context=manifest.orchestration as any,row=context.targets?.find((row:any)=>row.project===task.project&&row.target===task.target);if(JSON.stringify(row?.delegation)!==JSON.stringify((task.definition.metadata as any)?.semio?.execution))throw new Error("Original Bazel orchestration declaration changed");}
 const runTask=async(environment:NodeJS.ProcessEnv,actual:ResourceLeaseControl=phaseControl)=>{const cwd=execution.cwd?resolve(workspace,execution.program===undefined?interpolate(execution.cwd,workspace,task,()=>environment):execution.cwd):workspace;
 const operations=execution.program!==undefined?[execution.program]:execution.commands??(execution.command?[execution.command]:[]),args=buildExecutionArgumentsV1({...execution,forwardAllArgs:true},task.args);if(!operations.length)throw new Error(`Build command required: ${task.id}`);
 const controller=new AbortController(),signal=AbortSignal.any([actual.signal,controller.signal]),run=async(operation:string|{command:string;forwardAllArgs?:boolean})=>{const command=typeof operation==="string"?operation:operation.command,forward=typeof operation==="object"?operation.forwardAllArgs??execution.forwardAllArgs:execution.forwardAllArgs;let source=execution.program===undefined?interpolate(command,workspace,task,()=>environment):"'"+command.replaceAll("'","'\\''")+"'";if(!execution.ownerArguments&&forward!==false&&args.length)source+=" "+args.map(value=>"'"+value.replaceAll("'","'\\''")+"'").join(" ");await withIssuedResourceOperationV1(actual,()=>{checkResourceLeaseControl(actual);const remaining=actual.remainingMilliseconds();if(remaining!==null&&remaining<=0)throw new Error("Inherited Bazel task phase expired");return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal});});};
 if(execution.parallel===false)for(const operation of operations)await run(operation);else{const pending=operations.map(run);try{await Promise.all(pending);}catch(error){controller.abort(error);await Promise.allSettled(pending);throw error;}}
 };
 const base=bazelTaskBaseEnvironmentV1(workspace,task,process.env);
 for(const[key,value]of Object.entries(issueStoreReceiptV1({workspaceRoot:workspace,projectRoot:task.root,target:task.target,outputs:task.definition.outputs})))if(value===undefined)delete base[key];else base[key]=value;
 if(manifest.orchestration)return withBuildOrchestrationTaskEnvironmentV1(workspace,invocation,phaseControl,execution,task.args,base,manifest.orchestration,planBazelTasksV1,(environment,actual)=>runTask({...environment,...bazelTaskCargoDirectoriesV1(workspace,task)},actual));
 return runTask(issueBazelTaskEnvironmentV1(workspace,invocation,task,base));
}

/** 📡️ Relays cancellation to actual local actions and waits for their command trees to close. */
/** 📨️ Configures the current Bazel control socket's text framing. */
export function initializeBazelControlTextV1(socket:{setEncoding(encoding:"utf8"):unknown}):void{socket.setEncoding("utf8");}

/** 🧹️ Retains the task primary while completing and reporting bounded Bazel cleanup. */
export async function withBazelTaskCleanupV1<T>(body:()=>Promise<T>,cleanup:()=>Promise<void>):Promise<T>{
 let failed=false,primary:unknown;try{return await body();}catch(error){failed=true;primary=error;throw error;}finally{try{await cleanup();}catch(error){if(!failed)throw error;if(primary instanceof Error)try{const prior=Object.getOwnPropertyDescriptor(primary,"bazelCleanupFailures")?.value;Object.defineProperty(primary,"bazelCleanupFailures",{value:[...(Array.isArray(prior)?prior.slice(-7):[]),error],configurable:true});}catch{}try{let message="Bazel cleanup refused";try{message=String(error).slice(0,4096);}catch{}process.stderr.write("[bazel] Cleanup refusal retained after primary: "+message+"\n");}catch{}}}
}

/** 🪺️ Matches only finite complete argv vectors authored by the current Bazel owner. */
export function bazelBirthParametersAllowedV1(grants:readonly (readonly string[])[]|undefined,parameters:readonly string[]):boolean{
 return grants!==undefined&&validateJsonSchemaSubset(birthSchema.definitions.ParameterGrants,grants,componentSchema).length===0&&grants.some(vector=>vector.length===parameters.length&&vector.every((value,index)=>value===parameters[index]));
}

async function bazelCancellationControl(invocation:ScriptInvocation,signal:AbortSignal,births:ReadonlyMap<string,{digest:string;parameters:Readonly<Record<string,readonly (readonly string[])[]>>}>):Promise<{url:string;commandSignal:AbortSignal;stop:()=>Promise<void>}>{
 const token=randomUUID(),pendingBirths=new Set<Promise<void>>(),active=new Map<string,{socket:Socket;role:"birth"|"execution"}>(),connections=new Set<Socket>(),commands=new AbortController();let closed=false,stopping:Promise<void>|undefined,drained:(()=>void)|undefined;
 const server=createServer(socket=>{initializeBazelControlTextV1(socket);connections.add(socket);socket.once("close",()=>connections.delete(socket));socket.on("error",()=>{});let pending="",id:string|undefined,authorized=false,role:"birth"|"execution"="execution";socket.on("data",chunk=>{pending+=chunk.toString();if(Buffer.byteLength(pending)>policy.maximumControlMessageBytes){socket.destroy();return;}let end:number;while((end=pending.indexOf("\n"))>=0){let message;try{message=JSON.parse(pending.slice(0,end));}catch{socket.destroy();return;}pending=pending.slice(end+1);if(!message||typeof message!=="object"||message.version!==1||message.token!==token){socket.destroy();return;}if(message.event==="join"&&!id&&!closed){id=randomUUID();const joined=async()=>{let wire:string|undefined;if(message.birth!==undefined){if(validateJsonSchemaSubset(birthSchema,message.birth).length)throw new Error("Closed Bazel action birth required");const grant=births.get(message.birth.task);if(!grant||grant.digest!==message.birth.manifestSha256||!bazelBirthParametersAllowedV1(grant.parameters[message.birth.kind],message.birth.parameters))throw new Error("Original Bazel action manifest is not admitted");await advanceScriptInvocation(invocation,"bazel:action-birth","running");checkScriptInvocation(invocation);if(closed)throw new Error("Bazel action birth closed");wire=JSON.stringify(readIssuedScriptProcessEnvelopeV1(invocation));}const response=JSON.stringify({version:1,event:"joined",id,...(wire?{wire}:{})})+"\n";if(Buffer.byteLength(response)>policy.maximumControlMessageBytes)throw new Error("Bazel action birth response exceeds finite control boundary");role=message.birth===undefined?"execution":"birth";authorized=true;socket.write(response);};const pending=joined();pendingBirths.add(pending);void pending.then(()=>pendingBirths.delete(pending),error=>{pendingBirths.delete(pending);socket.destroy(error instanceof Error?error:Error("Bazel action birth refused"));});}else if(message.event==="start"&&authorized&&id===message.id&&!closed&&!active.has(id!)){active.set(id!,{socket,role});socket.write('{"version":1,"event":"started"}\n');}else if(message.event==="closed"&&id===message.id){active.delete(id!);socket.end('{"version":1,"event":"acknowledged"}\n',()=>{if(!active.size)drained?.();});}else{socket.destroy();return;}}});});
 await new Promise<void>((accept,refuse)=>{server.once("error",refuse);server.listen(0,"127.0.0.1",()=>{server.off("error",refuse);accept();});});const address=server.address();if(!address||typeof address==="string")throw new Error("Bazel action control address required");
 const stop=()=>stopping??=(async()=>{closed=true;for(const{socket,role}of active.values())if(role===birthSchema.definitions.Protocol.const.cooperativeCancellationRole&&!socket.destroyed)socket.write('{"version":1,"event":"cancelled"}\n');try{if(active.size)await new Promise<void>((accept,refuse)=>{const timer=setTimeout(()=>refuse(Error("Bazel action termination acknowledgement deadline exhausted")),policy.terminationReceiptMilliseconds);drained=()=>{clearTimeout(timer);accept();};if(!active.size)drained();});}finally{commands.abort(signal.reason??Error("Bazel task closure completed"));for(const socket of connections)socket.destroy();await Promise.allSettled([...pendingBirths]);await new Promise<void>((accept,refuse)=>server.close(error=>error?refuse(error):accept()));signal.removeEventListener("abort",cancel);}})();
 const cancel=()=>{void stop().catch(()=>{});};signal.addEventListener("abort",cancel,{once:true});if(signal.aborted)cancel();return{url:`tcp://127.0.0.1:${address.port}/${token}`,commandSignal:commands.signal,stop};
}

/** 📻️ Keeps original process authority while receiving its owning Bazel invocation's cancellation. */
async function withBazelCancellation<T>(environment:NodeJS.ProcessEnv,invocation:ScriptInvocation,run:(owned:ScriptInvocation)=>Promise<T>):Promise<T>{
 readIssuedScriptProcessEnvelopeV1(invocation);const url=environment.SEMIO_BAZEL_CONTROL_URL,match=url&&/^tcp:\/\/127\.0\.0\.1:(\d+)\/([a-f0-9-]+)$/u.exec(url);if(!match)throw new Error("Original loopback Bazel action cancellation control required");const cancelled=new AbortController(),socket=createConnection({host:"127.0.0.1",port:Number(match[1])});initializeBazelControlTextV1(socket);const token=match[2];let id:string|undefined,pending="",acknowledged=false,started=false,acceptJoin!:()=>void,refuseJoin!:(error:unknown)=>void,acceptClosed!:()=>void,refuseClosed!:(error:unknown)=>void;const joined=new Promise<void>((accept,refuse)=>{acceptJoin=accept;refuseJoin=refuse;}),terminated=new Promise<void>((accept,refuse)=>{acceptClosed=accept;refuseClosed=refuse;});void terminated.catch(()=>{});const fail=(error:unknown)=>{cancelled.abort(error);refuseJoin(error);refuseClosed(error);},interrupt=()=>{if(!started){const reason=invocation.control.signal.reason??Error("Original Bazel handshake cancelled");cancelled.abort(reason);refuseJoin(reason);if(!id)socket.destroy(reason);}},remaining=invocation.control.remainingMilliseconds(),handshake=remaining===null?undefined:setTimeout(()=>{refuseJoin(Error("Original Bazel handshake deadline exhausted"));if(!id)socket.destroy();},remaining);invocation.control.signal.addEventListener("abort",interrupt,{once:true});if(invocation.control.signal.aborted)interrupt();
 socket.on("error",fail);socket.once("close",()=>{if(!acknowledged)fail(Error("Original Bazel action control disconnected"));});socket.once("connect",()=>socket.write(JSON.stringify({version:1,event:"join",token})+"\n"));socket.on("data",chunk=>{pending+=chunk.toString();if(Buffer.byteLength(pending)>policy.maximumControlMessageBytes){fail(Error("Bazel action control exceeds its bounded message"));socket.destroy();return;}let end:number;while((end=pending.indexOf("\n"))>=0){let message;try{message=JSON.parse(pending.slice(0,end));}catch(error){fail(error);socket.destroy();return;}pending=pending.slice(end+1);if(!message||typeof message!=="object"||message.version!==1){fail(Error("Invalid Bazel action control"));socket.destroy();return;}if(message.event==="joined"&&typeof message.id==="string"&&!id){id=message.id;if(!cancelled.signal.aborted)socket.write(JSON.stringify({version:1,event:"start",token,id})+"\n");}else if(message.event==="started"&&id&&!started){started=true;if(handshake)clearTimeout(handshake);acceptJoin();}else if(message.event==="cancelled")cancelled.abort(Error("Original Bazel invocation cancelled"));else if(message.event==="acknowledged"){acknowledged=true;acceptClosed();}else{fail(Error("Invalid Bazel action control event"));socket.destroy();return;}}});
 return withBazelTaskCleanupV1(async()=>{await joined;return withScriptInvocationSignalV1(invocation,cancelled.signal,run);},async()=>{if(handshake)clearTimeout(handshake);invocation.control.signal.removeEventListener("abort",interrupt);const timer=setTimeout(()=>socket.destroy(Error("Bazel action termination receipt deadline exhausted")),policy.terminationReceiptMilliseconds);try{if(id&&!socket.destroyed)socket.write(JSON.stringify({version:1,event:"closed",token,id})+"\n");await terminated;}finally{clearTimeout(timer);socket.destroy();}});
}

/** 🎛️ Runs Bazel with owned cancellation and restores cache hits into the workspace. */
export async function runBazelTasksV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,onStarted?:()=>void|Promise<void>):Promise<number>{requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);return withBazelStorageV1(root,invocation,phaseControl,(storage,actualPhase)=>runBazelTasksInStorageV1(root,graph,request,invocation,storage,actualPhase,onStarted));}

/** 🧱️ Runs every generated Bazel writer and original child under retained fixed backing. */
async function runBazelTasksInStorageV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl,onStarted?:()=>void|Promise<void>):Promise<number>{
 await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);
 checkScriptInvocation(invocation);const signal=phaseControl.signal,original=issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],process.env),bazel=await prepareBazel(root,storage.tools,phaseControl,event=>{if(event.phase!=="downloading")console.log(`[bazel] ${event.phase} ${event.bytes}/${event.total}`);},original.CARGO_NET_OFFLINE==="true"),modules=await prepareBazelModulesV1(root,bazel,invocation,storage,phaseControl);
 const namespace=hash(JSON.stringify({selected:planBazelTasksV1(graph,request).selected,args:request.args,configuration:request.configuration})).slice(0,16),chain=(process.env.SEMIO_BAZEL_SELECTION_CHAIN??"").split(":").filter(Boolean);if(chain.includes(namespace))throw new Error("Recursive Bazel task selection refused");
 return withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal,control:phaseControl,resources:[{resource:"bazel-workspace-"+namespace,mode:"exclusive"}]},async()=>{
  const {workspace,plan,implementation}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl),emitted=bazelActionsV1(graph,plan,request.skipCache===true),outputRoot=join(storage.paths.outputs,hash(JSON.stringify({version:BAZEL_VERSION,platform:process.platform,arch:process.arch,idle:policy.serverIdleSeconds,jvm:"-Xmx2048m"})).slice(0,16)),cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache}),startup=[`--output_user_root=${outputRoot}`,`--max_idle_secs=${policy.serverIdleSeconds}`,`--install_base=${join(storage.paths.install,BAZEL_VERSION,process.platform+"-"+process.arch)}`,"--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(root,".bazelrc")}`,...cache.startup],options=["--lockfile_mode=off","--incompatible_strict_action_env",...cache.command,`--jobs=${request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism()))}`];
  await writeChanged(join(workspace,"MODULE.bazel.lock"),modules.lock);options[0]="--lockfile_mode=error";options.push("--incompatible_autoload_externally=",`--repository_cache=${modules.repository}`,"--repository_disable_download");const environment={...storage.products.environment,SEMIO_BAZEL_SELECTION_CHAIN:[...chain,namespace].join(":"),...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
  if(request.bail!==true)options.push("--keep_going");if(request.verbose)options.push("--subcommands");
  const rendering=request.outputStyle?{outputStyle:request.outputStyle,output:(readIssuedScriptProcessEnvelopeV1(invocation).capabilities as NativeBuildCallerCapabilitiesV1).native.transport,control:phaseControl}:{};
  const bin=resolve((await captureOwnedCommandV1(bazel,[...startup,"info","bazel-bin",...options.filter(option=>!option.startsWith("--jobs="))],workspace,{signal,control:phaseControl,env:environment})).trim().split("\n").at(-1)!);
  const controller=new AbortController(),cancel=()=>controller.abort(signal.reason),services:{task:BazelTaskV1;completion:Promise<void>}[]=[];let building:Promise<void>|undefined,rejectFailure!:(error:unknown)=>void;const failed=new Promise<never>((_resolve,reject)=>{rejectFailure=reject;});void failed.catch(()=>{});const fail=(error:unknown)=>{if(!controller.signal.aborted){rejectFailure(error);controller.abort(error);}};signal.addEventListener("abort",cancel,{once:true});if(signal.aborted)cancel();
  if(!bin.includes(sep+"execroot"+sep))throw new Error("Original Bazel execution root required");const births=new Map<string,{digest:string;parameters:Readonly<Record<string,readonly (readonly string[])[]>>}>();for(const action of emitted.values()){if(action.kind==="noop")continue;checkScriptInvocation(invocation);const task=plan.tasks[action.id]!,source=await readFile(join(workspace,task.label+".json")),extension=process.platform==="win32"?".exe":"";births.set(task.id,{digest:hash(source),parameters:action.kind==="launch"?{launcher:[[join(bin,task.label+extension)]],launch:[action.restore.map(label=>join(bin,label+".snapshot"))]}:{action:[[join(bin,task.label+".snapshot"),...action.restore.map(label=>join(bin,label+".snapshot"))]]}});}const control=await bazelCancellationControl(invocation,controller.signal,births);Object.assign(environment,{SEMIO_BAZEL_CONTROL_URL:control.url});
  await withBazelTaskCleanupV1(async()=>{
   await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
   for(const task of Object.values(plan.tasks).filter(task=>task.definition.continuous)){
    let accept!:()=>void,refuse!:(error:unknown)=>void,observed=false;const ready=new Promise<void>((resolve,reject)=>{accept=resolve;refuse=reject;}),required=typeof task.definition.execution.readyWhen==="string"?[task.definition.execution.readyWhen]:Array.isArray(task.definition.execution.readyWhen)?task.definition.execution.readyWhen as string[]:[],seen=new Set<string>(),dependency=Object.values(plan.tasks).some(consumer=>consumer.dependencies.includes(task.id));
    const declaration=required.length?undefined:readReadinessDeclarationV1(task.definition.metadata,task.id),watch:ReadinessWatchV1|undefined=declaration?watchReadinessV1(resolveReadinessV1(declaration,{...process.env,...task.definition.execution.env},task.id),controller.signal):undefined;
    if(watch)void watch.ready.then(address=>{if(!observed){observed=true;console.log(`[bazel] task-ready ${task.id} ${address}`);accept();}});
    const completion=runOwnedCommand(bazel,[...startup,"run",...options,"//:"+task.label,"--",implementation.runner,implementation.snapshot,implementation.sha256,"birth-launch",join(workspace,task.label+".json"),...emitted.get(task.id)!.restore.map(label=>join(bin,label+".snapshot"))],workspace,task.id,compilerAdmissionBudgetV1(phaseControl),{...rendering,signal:control.commandSignal,env:environment,onLine:line=>{if(watch){watch.observe(line+"\n");return;}for(const marker of required)if(line.includes(marker))seen.add(marker);if(required.length?seen.size===required.length:line.includes(`[bazel] task-started ${task.id}`)){observed=true;accept();}}});
    completion.then(()=>{watch?.stop();if(!controller.signal.aborted&&(!observed||dependency)){const error=Error(`Continuous task exited ${observed?"while required by selected tasks":"before readiness"}: ${task.id}`);refuse(error);fail(error);}},error=>{watch?.stop();refuse(error);fail(error);});services.push({task,completion});await Promise.race([ready,failed]);
   }
   building=runOwnedCommand(bazel,[...startup,"build",...options,`--build_event_json_file=${bazelBuildEventsFileV1(storage.paths,workspace)}`,...plan.selected.map(id=>"//:"+plan.tasks[id]!.label)],workspace,"bazel:build",compilerAdmissionBudgetV1(phaseControl),{...rendering,signal:control.commandSignal,env:environment}).finally(()=>recordBazelBuildMetricsV1(root,storage.paths,workspace,phaseControl));
   await Promise.race([building,failed]);
   for(const id of plan.selected){const action=emitted.get(id)!;if(action.kind==="task")await restoreBazelSnapshotV1(root,join(bin,action.label+".snapshot"),invocation,phaseControl);}
   await Promise.race([Promise.resolve(onStarted?.()),failed]);
   await Promise.race([Promise.all(services.filter(service=>plan.selected.includes(service.task.id)).map(service=>service.completion)),failed]);
  },async()=>{controller.abort(Error("Bazel selected task closure completed"));await withBazelTaskCleanupV1(()=>control.stop(),async()=>{await Promise.allSettled([...services.map(service=>service.completion),...(building?[building]:[])]);signal.removeEventListener("abort",cancel);});});
  return 0;
 });
}

/** 📖️ Reads finite birth material without treating its contents as authority. */
async function readBazelBirthFileV1(path:string,maximumBytes:number,deadline:()=>void):Promise<Buffer>{
 deadline();const file=await open(path,constants.O_RDONLY|constants.O_NOFOLLOW);let failed=false;try{const before=await file.stat({bigint:true});if(!before.isFile()||before.nlink!==1n||before.size>BigInt(maximumBytes))throw new Error("Bounded regular Bazel birth input required");const bytes=Buffer.alloc(Number(before.size));let offset=0;while(offset<bytes.length){deadline();const row=await file.read(bytes,offset,Math.min(birthSchema.definitions.Protocol.const.readChunkBytes,bytes.length-offset),offset);deadline();if(row.bytesRead===0)throw new Error("Bazel birth input shortened");offset+=row.bytesRead;}const after=await file.stat({bigint:true}),named=await lstat(path,{bigint:true});deadline();if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key as keyof typeof before]!==after[key as keyof typeof after]||before[key as keyof typeof before]!==named[key as keyof typeof named]))throw new Error("Bazel birth input changed");return bytes;}catch(error){failed=true;throw error;}finally{try{await file.close();}catch(error){if(!failed)throw error;}}
}

/** 🐣️ Obtains unchanged current authority from the live owned parent before the actual action child is born. */
async function launchBazelActionBirthV1(kind:string,file:string,parameters:readonly string[],implementation:BazelImplementationV1):Promise<void>{
 const protocol=birthSchema.definitions.Protocol.const,started=performance.now(),initial=()=>{if(performance.now()-started>=protocol.maximumHandshakeMilliseconds)throw new Error("Bazel action birth handshake deadline exhausted");},manifestPath=await realpath(resolve(file)),manifestBytes=await readBazelBirthFileV1(manifestPath,protocol.maximumManifestBytes,initial),manifest=JSON.parse(manifestBytes.toString("utf8")) as BazelTaskManifestV1;requireBazelManifestV1(manifest,implementation);if(typeof manifest.task.id!=="string")throw new Error("Bazel birth rendezvous required");const environment=(await readBazelRuntimeV1(manifestPath,implementation,initial)).environment,expected=environment.SEMIO_SCRIPT_PROCESS_INVOCATION;if(typeof expected!=="string")throw new Error("Bazel birth wire expectation required");const envelope=readScriptProcessEnvelope(JSON.parse(expected),Date.now()),remaining=()=>{initial();if(envelope.deadlineEpochMilliseconds!==null&&envelope.deadlineEpochMilliseconds<=Date.now())throw new Error("Original Bazel action birth deadline exhausted");},match=/^tcp:\/\/127\.0\.0\.1:(\d+)\/([a-f0-9-]+)$/u.exec(environment.SEMIO_BAZEL_CONTROL_URL??"");if(!match)throw new Error("Owned Bazel birth rendezvous required");remaining();
 const birth={kind,task:manifest.task.id,manifestSha256:hash(manifestBytes),parameters:parameters.map(path=>resolve(path))};if(validateJsonSchemaSubset(birthSchema,birth).length||birth.parameters.some(path=>path.length>256))throw new Error("Closed Bazel action birth required");const controller=new AbortController(),socket=createConnection({host:"127.0.0.1",port:Number(match[1])});initializeBazelControlTextV1(socket);let id:string|undefined,pending="",acknowledged=false,issued=false,acceptJoin!:(wire:string)=>void,refuseJoin!:(error:unknown)=>void,acceptClosed!:()=>void,refuseClosed!:(error:unknown)=>void;const joined=new Promise<string>((accept,refuse)=>{acceptJoin=accept;refuseJoin=refuse;}),closed=new Promise<void>((accept,refuse)=>{acceptClosed=accept;refuseClosed=refuse;});void closed.catch(()=>{});const fail=(error:unknown)=>{controller.abort(error);refuseJoin(error);refuseClosed(error);},send=(value:object)=>{const message=JSON.stringify(value)+"\n";if(Buffer.byteLength(message)>policy.maximumControlMessageBytes)throw new Error("Bazel birth message exceeds finite control boundary");socket.write(message);},handshake=setTimeout(()=>fail(Error("Bazel action birth handshake deadline exhausted")),Math.max(1,Math.min(protocol.maximumHandshakeMilliseconds-(performance.now()-started),envelope.deadlineEpochMilliseconds===null?protocol.maximumHandshakeMilliseconds:envelope.deadlineEpochMilliseconds-Date.now())));
 socket.on("error",fail);socket.once("close",()=>{if(!acknowledged)fail(Error("Original Bazel birth parent disconnected"));});socket.once("connect",()=>{try{remaining();send({version:1,event:"join",token:match[2],birth});}catch(error){fail(error);}});socket.on("data",chunk=>{pending+=chunk.toString();if(Buffer.byteLength(pending)>policy.maximumControlMessageBytes){fail(Error("Bazel birth response exceeds finite control boundary"));return;}let end:number;while((end=pending.indexOf("\n"))>=0){try{const message=JSON.parse(pending.slice(0,end));pending=pending.slice(end+1);if(message.version!==1)throw new Error("Bazel birth response refused");if(message.event==="joined"&&!id&&typeof message.id==="string"&&message.wire===expected){remaining();id=message.id;send({version:1,event:"start",token:match[2],id});}else if(message.event==="started"&&id&&!issued){issued=true;clearTimeout(handshake);acceptJoin(expected);}else if(message.event==="cancelled")fail(Error("Original Bazel birth parent cancelled"));else if(message.event==="acknowledged"){acknowledged=true;acceptClosed();}else throw new Error("Bazel birth response refused");}catch(error){fail(error);}}});
 let primaryFailed=false;try{const wire=await joined;await withScriptProcessEnvelope(readScriptProcessEnvelope(JSON.parse(wire),Date.now()),original=>withScriptInvocationSignalV1(original,controller.signal,async invocation=>{await advanceScriptInvocation(invocation,"bazel:owned-action-child","running");await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,kind,manifestPath,...birth.parameters],process.cwd(),"bazel:owned-action-child",scriptInvocationBudget(invocation,0),{env:scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256}),signal:invocation.control.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:owned-action-child","running")});checkScriptInvocation(invocation);}));}catch(error){primaryFailed=true;throw error;}finally{clearTimeout(handshake);const timer=setTimeout(()=>socket.destroy(Error("Bazel birth termination receipt deadline exhausted")),policy.terminationReceiptMilliseconds);try{if(id&&!socket.destroyed){send({version:1,event:"closed",token:match[2],id});await closed;}}catch(error){if(!primaryFailed)throw error;}finally{clearTimeout(timer);socket.destroy();}}
}

/** 🏃️ Executes only the checksum-bound captured backend inside the actual child birth. */
export async function runCapturedBuildSourceV1(argv:string[],proof?:{version:1;snapshot:string;sha256:string;loaderSha256?:string;recheck?:()=>void}):Promise<void>{
 if(!proof||proof.version!==1||import.meta.semioCapturedImplementationIdentity!==proof.sha256||import.meta.semioCapturedLoaderIdentity!==proof.loaderSha256||proof.loaderSha256!==hash(buildSourceClosureLoaderV1))throw new Error("Captured Bazel loader identity required");const implementation:BazelImplementationV1={version:1,snapshot:proof.snapshot,runner:join(dirname(proof.snapshot),"📜️script.ts"),sha256:proof.sha256,loaderSha256:proof.loaderSha256};requireBazelImplementationV1(implementation,implementation);if(argv[0]==="planner"){if(typeof proof.recheck!=="function")throw new Error("Captured planner Source recheck required");const recheck=proof.recheck;await receiveScriptProcessInvocation(process.env,async original=>{const[input,digest,output]=argv.slice(1);if(!input||!output||!isAbsolute(input)||!isAbsolute(output)||dirname(input)!==dirname(implementation.snapshot)||dirname(output)!==dirname(input))throw new Error("Owned captured planner data paths required");const bytes=await readBazelBirthFileV1(input,plannerSchema.definitions.Policy.const.maximumDataBytes,()=>checkScriptInvocation(original));if(hash(bytes)!==digest)throw new Error("Captured planner data changed");const value=JSON.parse(bytes.toString("utf8"));if(validateJsonSchemaSubset(plannerSchema,value).length)throw new Error("Closed captured planner data required");requireBazelImplementationV1(value.implementation,implementation);issueBuildTaskEnvironmentV1(value.root,original,{kind:"noop"},[],process.env);const graph=readBuildGraphV1(value.graph);recheck();await withBazelStorageV1(value.root,original,createIssuedResourceControlV1(original,"cache:entry",0),async(storage,actualPhase)=>{if(value.workspace!==dirname(implementation.snapshot)||!value.workspace.startsWith(storage.paths.workspace+sep))throw new Error("Declared captured planner workspace required");const result=await publishCapturedBazelGraphV1(value.root,graph,value.request,original,storage,actualPhase,implementation,recheck);if(validateJsonSchemaSubset(plannerSchema.definitions.Result,result).length)throw new Error("Closed captured Bazel planner result required");recheck();checkScriptInvocation(original);await writeBazelImmutableInputV1(output,JSON.stringify(result),plannerSchema.definitions.Policy.const.maximumDataBytes,original);checkScriptInvocation(original);});});return;}if(argv[0]?.startsWith("birth-")){await launchBazelActionBirthV1(argv[0].slice(6),argv[1]!,argv.slice(2),implementation);}else{
 const [command,file,...parameters]=argv,output=command==="launch"?undefined:parameters.shift(),dependencies=parameters;if(!["action","launch","launcher"].includes(command??"")||!file||command!=="launch"&&!output)throw new Error("Expected Bazel action manifest output [dependency snapshots]");
 const actualBirth=readScriptProcessEnvelope(JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION??"null"),Date.now()),checkBirth=()=>{if(actualBirth.deadlineEpochMilliseconds!==null&&actualBirth.deadlineEpochMilliseconds<=Date.now())throw new Error("Original Bazel child deadline exhausted");},manifestPath=await realpath(resolve(file)),manifestBytes=await readBazelBirthFileV1(manifestPath,birthSchema.definitions.Protocol.const.maximumManifestBytes,checkBirth);if(!/^[a-f0-9]{64}$/u.test(process.env.SEMIO_BAZEL_MANIFEST_SHA256??"")||hash(manifestBytes)!==process.env.SEMIO_BAZEL_MANIFEST_SHA256)throw new Error("Original Bazel action birth manifest changed");const manifest=JSON.parse(manifestBytes.toString("utf8")) as BazelTaskManifestV1;requireBazelManifestV1(manifest,implementation);const runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;
 await receiveScriptProcessInvocation(environment,async original=>{if(command==="launcher"){await link(process.execPath,resolve(output!)).catch(()=>copyFile(process.execPath,resolve(output!)));await chmod(resolve(output!),0o755);return;}await withBazelCancellation(environment,original,async invocation=>{const actionPhase=createIssuedResourceControlV1(invocation,"bazel:task-admission",0),run=async()=>withRepositoryCacheStoreV1(runtime.workspace,"graph",invocation,actionPhase,async(_graph,graphPhase)=>{for(const dependency of dependencies)await restoreBazelSnapshotV1(runtime.workspace,resolve(dependency),invocation,actionPhase);const previous=process.env;process.env={...previous,...environment};try{await executeTask(manifest,runtime.workspace,invocation,graphPhase);if(command==="action")await captureBazelSnapshotV1({workspace:runtime.workspace,task:manifest.task},resolve(output!),invocation);}finally{process.env=previous;}});if(command==="launch")await run();else await withResourceLeases({directory:join(runtime.workspace,".cache/b/l/m/data"),signal:invocation.control.signal,control:actionPhase,resources:[{resource:"bazel-task-schedule-"+hash(dirname(manifestPath)).slice(0,16),mode:manifest.task.definition.parallelism===false?"exclusive":"shared"}]},run);});});
}
}

if(import.meta.main)throw new Error("Bazel task entry requires its captured implementation loader");
