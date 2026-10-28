import{refuseBuildV1,sayBuildV1}from"../../../📖️texts/🟦️.ts";
/** 🎬️ The executor half of the Bazel adapter: everything an action runs inside its birth, with its own narrow source closure, so that editing planning, graph or launcher code never changes an action key. */
import {CONTENTION_POLICY_V1,budgetStretchV1,createBudgetClockV1,sharedHostContentionV1,type ContentionSourceV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {issueStoreReceiptV1,settleStoreReceiptV1} from "../../../🧾️store-receipt/🟨️.mjs";
import {bazelRestoreResourcesV1} from "../🧵️sessions/🟦️.ts";
import {createHash,randomUUID} from "node:crypto";
import {constants} from "node:fs";
import {createConnection} from "node:net";
import {chmod,copyFile,link,lstat,mkdir,open,readFile,readdir,readlink,realpath,rename,rm,stat,symlink,writeFile} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import type {BazelTaskV1} from "../📋️task-plan/🟦️.ts";
import {buildExecutionArgumentsV1,issueBuildTaskEnvironmentV1,withBuildOrchestrationTaskEnvironmentV1} from "../../../📥️caller/🟦️.ts";
import {buildExecEnvironmentV1} from "../../../⚙️operations/🌱️exec-environment/🟦️.ts";
import {bazelFailureTailDirectoryV1,createBazelFailureTailV1,writeBazelFailureTailV1} from "../🪵️failure-tail/🟦️.ts";
import {bazelTestEnvironmentV1,settleBazelTestV1} from "../🚦️test-classes/🟦️.ts";
import {testProtocolMessageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🚦️protocol/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {compilerAdmissionBudgetV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {advanceScriptInvocation,checkScriptInvocation,receiveScriptProcessInvocation,scriptInvocationBudget,scriptInvocationEnvironment,withScriptInvocationSignalV1,readIssuedScriptProcessEnvelopeV1,readScriptProcessEnvelope,withScriptProcessEnvelope,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {buildSourceClosureLoaderV1} from "../../../📥️source/🔏️closure/🟦️.ts";
import {readWorkerPolicyV1,requireWorkerPlanV1,workerActionKindV1,type WorkerPlanV1} from "../../../👷️workers/🟦️.ts";
import {installWorkerOutputCaptureV1,readWorkerInputV1,serveWorkerV1} from "../../../👷️workers/📨️protocol/🟦️.ts";
import {GlobV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import {isMainModuleV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {runtimeCollectGarbageV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import componentSchema from"../🧬️schema/🔣️.json" with { type: "json" };const {ActionBirth:birthSchema,OutputSnapshot:snapshotSchema,Implementation:implementationSchema,Manifest:manifestSchema,Runtime:runtimeSchema}=componentSchema.definitions;
import buildPolicy from "../../../🔣️policy.json" with { type: "json" };

const policy=componentSchema.definitions.Policy.const,workerPolicy=readWorkerPolicyV1(buildPolicy.workers);

declare global{interface ImportMeta{readonly semioCapturedImplementationIdentity?:string;readonly semioCapturedLoaderIdentity?:string}}

const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),slash=(value:string)=>value.split(sep).join("/");

export type BazelImplementationV1={version:1;snapshot:string;runner:string;sha256:string;loaderSha256:string};

type BazelManifestImplementationV1={version:1;sha256:string;loaderSha256:string};

export type BazelTaskManifestV1={version:1;task:BazelTaskV1;implementation:BazelManifestImplementationV1;orchestration?:object};

export type BazelRuntimeV1={version:1;workspace:string;environment:NodeJS.ProcessEnv};

export interface BazelSnapshotContextV1{workspace:string;task:BazelTaskV1}

type SnapshotEntryV1={path:string;kind:"directory"|"file"|"symlink";mode:number;file?:string;sha256?:string;link?:string;linkKind?:"directory"|"file"};

type SnapshotV1={version:1;roots:string[];entries:SnapshotEntryV1[]};

/** 📸️ Restricts generated snapshots and declared outputs to the real workspace. */
function workspacePath(root:string,path:string):string{
 const target=resolve(root,path),inside=relative(root,target);if(!inside||inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside))throw refuseBuildV1("invalid","bazel.output-outside-workspace",{path:String(path)});return target;
}

/** 🛡️ Refuses output ancestors that redirect a restore outside actual directory custody. */
async function outputAncestry(root:string,path:string):Promise<void>{
 for(let current=dirname(workspacePath(root,path));current!==resolve(root);current=dirname(current))try{const stat=await lstat(current);if(!stat.isDirectory()||stat.isSymbolicLink())throw refuseBuildV1("invalid","bazel.output-ancestry-not-real",{current:String(current)});}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
}

/** 🔏️ Streams an output identity with cancellation between reads. */
export async function fileHash(path:string,signal:AbortSignal):Promise<string>{
 const file=await open(path,"r"),digest=createHash("sha256");try{for await(const bytes of file.createReadStream({autoClose:false})){signal.throwIfAborted();digest.update(bytes);}return digest.digest("hex");}finally{await file.close();}
}

/** 🧩️ Expands portable workspace, project and owner option tokens; an undeclared `env.<name>` option is the value the task process receives. */
function interpolate(value:string,root:string,task:BazelTaskV1,environment?:()=>NodeJS.ProcessEnv):string{
 return value.replaceAll("{workspaceRoot}",root).replaceAll("{projectRoot}",resolve(root,task.root)).replaceAll("{projectName}",task.project).replace(/\{options\.([^}]+)\}/gu,(_match,key:string)=>{let option:unknown=task.definition.execution;for(const part of key.split("."))option=option&&typeof option==="object"?(option as Record<string,unknown>)[part]:undefined;if(option===undefined&&environment&&key.startsWith("env."))option=environment()[key.slice(4)];if(typeof option!=="string"&&typeof option!=="number"&&typeof option!=="boolean")throw refuseBuildV1("missing","bazel.task-option-required",{key:String(key),task:String(task.id)});return String(option);});
}

/** 🗝️ Refuses any manifest that is not closed or that names a different captured implementation. */
function requireBazelManifestV1(manifest:unknown,expected:BazelImplementationV1):asserts manifest is BazelTaskManifestV1{
 if(validateJsonSchemaSubset(manifestSchema,manifest).length)throw refuseBuildV1("invalid","bazel.task-manifest-required");
 const implementation=(manifest as BazelTaskManifestV1).implementation;if(implementation.sha256!==expected.sha256||implementation.loaderSha256!==expected.loaderSha256)throw refuseBuildV1("missing","bazel.implementation-required");
}

/** 🪢 Reads the runtime channel that lives beside the manifest's own immutable implementation. */
async function readBazelRuntimeV1(manifestPath:string,implementation:BazelImplementationV1,deadline:()=>void):Promise<BazelRuntimeV1>{
 if(dirname(manifestPath)!==dirname(implementation.snapshot))throw refuseBuildV1("invalid","bazel.runtime-channel-workspace-required");
 const value=JSON.parse((await readBazelBirthFileV1(join(dirname(implementation.snapshot),"runtime.json"),birthSchema.definitions.Protocol.const.maximumRuntimeBytes,deadline)).toString("utf8"));
 if(validateJsonSchemaSubset(runtimeSchema,value).length)throw refuseBuildV1("invalid","bazel.runtime-channel-required");
 return value;
}

const cargoDirectoryKeys=["CARGO_TARGET_DIR","CARGO_BUILD_TARGET_DIR","CARGO_BUILD_BUILD_DIR"];

/** 🗂️ Resolves the Cargo directories a task declares for itself; they are task data, not the caller's artifact authority, so they never pass the issuance check and are laid over its result. */
export function bazelTaskCargoDirectoriesV1(workspace:string,task:BazelTaskV1):Record<string,string>{
 return Object.fromEntries(cargoDirectoryKeys.flatMap(key=>{const value=task.definition.execution.env?.[key];return typeof value==="string"?[[key,resolve(workspace,value)]]:[];}));
}

/** 🔑️ Issues a task environment under the caller's authority and keeps the task's own declared Cargo directories. */
export function issueBazelTaskEnvironmentV1(workspace:string,invocation:ScriptInvocation,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 const issued={...issueBuildTaskEnvironmentV1(workspace,invocation,task.definition.execution,task.args,environment),...bazelTaskCargoDirectoriesV1(workspace,task)};
 if((task.configuration==="prototype"||task.configuration==="release")&&task.configuration!==issued.SEMIO_BUILD_MODE)throw refuseBuildV1("refused","bazel.build-modes-conflict",{task:String(task.id),configuration:String(task.configuration),mode:String(issued.SEMIO_BUILD_MODE)});
 return issued;
}

/** 🧰️ Shares task environment overrides between source fingerprints and actual command execution. */
export function bazelTaskBaseEnvironmentV1(workspace:string,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 return {...environment,...Object.fromEntries(Object.entries(task.definition.execution.env??{}).filter(([key])=>!cargoDirectoryKeys.includes(key))),SEMIO_BUILD_TOOL:"bazel",SEMIO_WORKSPACE_ROOT:workspace,SEMIO_TASK_PROJECT:task.project,SEMIO_TASK_TARGET:task.target,...(task.configuration?{SEMIO_TASK_CONFIGURATION:task.configuration}:{}),...(task.target==="repo-exec"?buildExecEnvironmentV1(task.project,task.root):{})};
}

/** 🔗️ Refuses a different captured executable at every private child boundary. */
export function requireBazelImplementationV1(value:unknown,expected:BazelImplementationV1):asserts value is BazelImplementationV1{if(validateJsonSchemaSubset(implementationSchema,value).length||JSON.stringify(value)!==JSON.stringify(expected)||![expected.snapshot,expected.runner].every(path=>isAbsolute(path)&&path.length<=256)||dirname(expected.snapshot)!==dirname(expected.runner))throw refuseBuildV1("missing","bazel.implementation-required");}

/** 🔒 Tells whether a declared output that a run did not produce is an error: a cached task must produce what it declares unless it declares `metadata.semio.optionalOutputs: true`, because an unproduced output is cached as present and lost on every cache hit. */
export function bazelMissingOutputRefusedV1(definition:{cache?:unknown;metadata?:unknown}):boolean{
 const semio=(definition.metadata as {semio?:{optionalOutputs?:unknown}}|undefined)?.semio;
 return definition.cache===true&&semio?.optionalOutputs!==true;
}

/** 🎚️ A target on the build-mode axis legitimately produces only the product directory of its own configuration. */
export function bazelOtherModeOutputV1(task:{configuration?:string;definition:Readonly<Record<string,unknown>>},path:string):boolean{
 const modes=(task.definition.metadata as {semio?:{buildMode?:{modes?:unknown}}}|undefined)?.semio?.buildMode?.modes,name=path.split("/").at(-1)!;
 return Array.isArray(modes)&&task.configuration!==undefined&&modes.includes(name)&&name!==task.configuration;
}

/** 📦️ Flattens declared products so cache paths remain short on every host. */
export async function captureBazelSnapshotV1(manifest:BazelSnapshotContextV1,output:string,invocation:ScriptInvocation):Promise<void>{
 const roots=new Set<string>(),entries:SnapshotEntryV1[]=[],environment=()=>issueBazelTaskEnvironmentV1(manifest.workspace,invocation,manifest.task,bazelTaskBaseEnvironmentV1(manifest.workspace,manifest.task,process.env));await mkdir(join(output,"files"),{recursive:true});
 for(const pattern of manifest.task.definition.outputs??[]){const absolute=interpolate(pattern,manifest.workspace,manifest.task,environment),path=slash(relative(manifest.workspace,resolve(manifest.workspace,absolute)));workspacePath(manifest.workspace,path);if(/[?*{[]/u.test(path)){for await(const match of new GlobV1(path).scan({cwd:manifest.workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}else try{await lstat(workspacePath(manifest.workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;if(bazelMissingOutputRefusedV1(manifest.task.definition)&&!bazelOtherModeOutputV1(manifest.task,path))throw refuseBuildV1("failed","bazel.replayed-output-missing",{task:String(manifest.task.id),path:String(path)});}}
 const visit=async(path:string):Promise<void>=>{checkScriptInvocation(invocation);await outputAncestry(manifest.workspace,path);const source=workspacePath(manifest.workspace,path),facts=await lstat(source),entry:SnapshotEntryV1={path,kind:facts.isSymbolicLink()?"symlink":facts.isDirectory()?"directory":"file",mode:facts.mode&0o777};if(entry.kind==="symlink"){entry.link=await readlink(source);const linked=await stat(source);checkScriptInvocation(invocation);if(!linked.isFile()&&!linked.isDirectory())throw refuseBuildV1("invalid","bazel.snapshot-link-target-required");entry.linkKind=linked.isDirectory()?"directory":"file";}else if(entry.kind==="directory")for(const child of await readdir(source))await visit(slash(join(path,child)));else{entry.file=hash(path).slice(0,32);await copyFile(source,join(output,"files",entry.file));entry.sha256=await fileHash(join(output,"files",entry.file),invocation.control.signal);}entries.push(entry);};
 for(const path of [...roots].sort().filter((path,_index,all)=>!all.some(parent=>parent!==path&&path.startsWith(parent+"/"))))await visit(path);
 await writeFile(join(output,"snapshot.json"),JSON.stringify({version:1,roots:[...roots].sort(),entries} satisfies SnapshotV1));
}

/** 🪪️ Changes only the captured directory inode while retaining the original control. */
async function chmodSnapshotDirectory(path:string,identity:{dev:bigint;ino:bigint},mode:number|((mode:number)=>number),invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
 return withBazelTaskCleanupV1(async()=>{
  checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const facts=await file.stat({bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  const verify=async()=>{const current=await lstat(path,{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!current.isDirectory()||current.isSymbolicLink()||current.dev!==identity.dev||current.ino!==identity.ino)throw refuseBuildV1("stale","bazel.output-directory-changed-during-mode-update");};
  if(!facts.isDirectory()||facts.dev!==identity.dev||facts.ino!==identity.ino)throw refuseBuildV1("stale","bazel.output-directory-changed-before-mode-update");
  await verify();checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await file.chmod(typeof mode==="number"?mode:mode(Number(facts.mode)&0o777));checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await verify();
 },async()=>{await file.close();checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);});
}

/** 📥️ Restores cached products before their consumer runs in the real workspace. */
export async function restoreBazelSnapshotV1(root:string,snapshot:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const signal=phaseControl.signal;
 const resources=bazelRestoreResourcesV1((JSON.parse(await readFile(join(snapshot,"snapshot.json"),"utf8")) as SnapshotV1).roots);return withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal,control:phaseControl,resources:[...resources]},async()=>{
 await advanceScriptInvocation(invocation,"bazel:snapshot-restore","running");let visited=0;const progress=async()=>{checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(++visited%128===0)await advanceScriptInvocation(invocation,"bazel:snapshot-restore","running");};const contents=JSON.parse(await readFile(join(snapshot,"snapshot.json"),"utf8")) as SnapshotV1;
 if(validateJsonSchemaSubset(snapshotSchema,contents).length)throw refuseBuildV1("invalid","bazel.output-snapshot-invalid");
 for(const path of contents.roots)await outputAncestry(root,path);
 const actual=new Set<string>(),pending=[...contents.roots],directories=new Map<string,{dev:bigint;ino:bigint}>(),restored=new Map<string,{dev:bigint;ino:bigint}>();let same=true;
 while(pending.length){await progress();const path=pending.pop()!;try{const stat=await lstat(workspacePath(root,path),{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);actual.add(path);if(stat.isDirectory()&&!stat.isSymbolicLink()){directories.set(path,{dev:stat.dev,ino:stat.ino});for(const child of await readdir(workspacePath(root,path)))pending.push(slash(join(path,child)));}}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;same=false;}}
 for(const entry of contents.entries){await progress();const path=workspacePath(root,entry.path);try{const stat=await lstat(path);if(entry.kind!=="symlink"&&(stat.mode&0o777)!==entry.mode)same=false;if(entry.kind==="directory"?!stat.isDirectory()||stat.isSymbolicLink():entry.kind==="symlink"?!stat.isSymbolicLink()||await readlink(path)!==entry.link:!stat.isFile()||stat.isSymbolicLink()||!entry.sha256||await fileHash(path,signal)!==entry.sha256)same=false;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;same=false;}}
 if(same&&actual.size===contents.entries.length)return;
 for(const[path,identity]of [...directories].sort((a,b)=>a[0].length-b[0].length)){await progress();await outputAncestry(root,path);await chmodSnapshotDirectory(workspacePath(root,path),identity,mode=>mode|0o700,invocation,phaseControl);}
 for(const path of contents.roots){await progress();await rm(workspacePath(root,path),{recursive:true,force:true});}
 for(const entry of [...contents.entries].sort((a,b)=>a.path.length-b.path.length||a.path.localeCompare(b.path))){await progress();const target=workspacePath(root,entry.path);await mkdir(dirname(target),{recursive:true});if(entry.kind==="directory"){await mkdir(target,{recursive:true});const facts=await lstat(target,{bigint:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!facts.isDirectory()||facts.isSymbolicLink())throw refuseBuildV1("invalid","bazel.restored-output-not-directory");restored.set(entry.path,{dev:facts.dev,ino:facts.ino});}else if(entry.kind==="symlink")await symlink(entry.link!,target,entry.linkKind==="directory"?"dir":"file");else{if(!/^[a-f0-9]{32}$/u.test(entry.file??""))throw refuseBuildV1("invalid","bazel.snapshot-file-identity-invalid");await copyFile(join(snapshot,"files",entry.file!),target);}if(entry.kind==="file")await chmod(target,entry.mode);}
 for(const entry of contents.entries.filter(entry=>entry.kind==="directory").sort((a,b)=>b.path.length-a.path.length)){await progress();await outputAncestry(root,entry.path);await chmodSnapshotDirectory(workspacePath(root,entry.path),restored.get(entry.path)!,entry.mode,invocation,phaseControl);}
 });
}

/** 👷️ Gives the commands a persistent worker starts an empty standard input, because the worker's own standard input carries the protocol. */
let workerChildInputV1:string|undefined;

/** ⚙️ Executes the direct owner command with the original caller's native capabilities. */
async function executeTask(manifest:BazelTaskManifestV1,workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,failures:string):Promise<void>{
 const {task}=manifest,execution=task.definition.execution;if(execution.kind==="noop")return;
 if(task.definition.continuous)console.log(`[bazel] task-started ${task.id}`);
 if(manifest.orchestration&&JSON.stringify((manifest.orchestration as any).task)!==JSON.stringify({project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{})}))throw refuseBuildV1("stale","bazel.orchestration-task-changed");
 if(manifest.orchestration){const context=manifest.orchestration as any,row=context.targets?.find((row:any)=>row.project===task.project&&row.target===task.target);if(JSON.stringify(row?.delegation)!==JSON.stringify((task.definition.metadata as any)?.semio?.execution))throw refuseBuildV1("stale","bazel.orchestration-declaration-changed");}
 const tail=createBazelFailureTailV1(),keepTail=async<T>(body:()=>Promise<T>):Promise<T>=>{try{return await body();}catch(error){if(!phaseControl.signal.aborted)await writeBazelFailureTailV1(failures,task.id,tail).catch(()=>undefined);throw error;}};
 const runTask=async(environment:NodeJS.ProcessEnv,actual:ResourceLeaseControl=phaseControl)=>{const cwd=execution.cwd?resolve(workspace,execution.program===undefined?interpolate(execution.cwd,workspace,task,()=>environment):execution.cwd):workspace;
 const operations=execution.program!==undefined?[execution.program]:execution.commands??(execution.command?[execution.command]:[]),args=buildExecutionArgumentsV1({...execution,forwardAllArgs:true},task.args);if(!operations.length)throw refuseBuildV1("missing","bazel.build-command-required",{task:String(task.id)});
 const controller=new AbortController(),signal=AbortSignal.any([actual.signal,controller.signal]),run=async(operation:string|{command:string;forwardAllArgs?:boolean})=>{const command=typeof operation==="string"?operation:operation.command,forward=typeof operation==="object"?operation.forwardAllArgs??execution.forwardAllArgs:execution.forwardAllArgs;let source=execution.program===undefined?interpolate(command,workspace,task,()=>environment):"'"+command.replaceAll("'","'\\''")+"'";if(!execution.ownerArguments&&forward!==false&&args.length)source+=" "+args.map(value=>"'"+value.replaceAll("'","'\\''")+"'").join(" ");await withIssuedResourceOperationV1(actual,()=>{checkResourceLeaseControl(actual);const remaining=actual.remainingMilliseconds();if(remaining!==null&&remaining<=0)throw refuseBuildV1("cancelled","bazel.task-phase-expired");return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal,onLine:line=>tail.push(line),...(workerChildInputV1===undefined?{}:{input:workerChildInputV1})});});};
 if(execution.parallel===false)for(const operation of operations)await run(operation);else{const pending=operations.map(run);try{await Promise.all(pending);}catch(error){controller.abort(error);await Promise.allSettled(pending);throw error;}}
 };
 const base=bazelTaskBaseEnvironmentV1(workspace,task,process.env);
 const receipt={workspaceRoot:workspace,projectRoot:task.root,target:task.target,outputs:task.definition.outputs},issued=issueStoreReceiptV1(receipt),startedAtMs=Date.now();
 for(const[key,value]of Object.entries(issued))if(value===undefined)delete base[key];else base[key]=value;
 const settled=async<T>(body:()=>Promise<T>):Promise<T>=>{let value:T;try{value=await body();}catch(error){await settleStoreReceiptV1(receipt,issued,{succeeded:false,startedAtMs}).catch(()=>undefined);throw error;}await settleStoreReceiptV1(receipt,issued,{succeeded:true,startedAtMs});return value;};
 if(manifest.orchestration)return settled(()=>keepTail(()=>withBuildOrchestrationTaskEnvironmentV1(workspace,invocation,phaseControl,execution,task.args,base,manifest.orchestration!,bazelOrchestrationPlanV1(invocation),(environment,actual)=>runTask({...environment,...bazelTaskCargoDirectoriesV1(workspace,task)},actual))));
 return settled(()=>keepTail(()=>runTask(issueBazelTaskEnvironmentV1(workspace,invocation,task,base))));
}

/** 📡️ Relays cancellation to actual local actions and waits for their command trees to close. */
/** 📨️ Configures the current Bazel control socket's text framing. */
export function initializeBazelControlTextV1(socket:{setEncoding(encoding:"utf8"):unknown}):void{socket.setEncoding("utf8");}

/** 🧹️ Retains the task primary while completing and reporting bounded Bazel cleanup. */
export async function withBazelTaskCleanupV1<T>(body:()=>Promise<T>,cleanup:()=>Promise<void>):Promise<T>{
 let failed=false,primary:unknown;try{return await body();}catch(error){failed=true;primary=error;throw error;}finally{try{await cleanup();}catch(error){if(!failed)throw error;if(primary instanceof Error)try{const prior=Object.getOwnPropertyDescriptor(primary,"bazelCleanupFailures")?.value;Object.defineProperty(primary,"bazelCleanupFailures",{value:[...(Array.isArray(prior)?prior.slice(-7):[]),error],configurable:true});}catch{}try{let message="Bazel cleanup refused";try{message=String(error).slice(0,4096);}catch{}sayBuildV1("bazel.cleanup-refusal-retained",{message},"warn");}catch{}}}
}

/** 🪺️ Matches only finite complete argv vectors authored by the current Bazel owner. */
export function bazelBirthParametersAllowedV1(grants:readonly (readonly string[])[]|undefined,parameters:readonly string[]):boolean{
 return grants!==undefined&&validateJsonSchemaSubset(birthSchema.definitions.ParameterGrants,grants,componentSchema).length===0&&grants.some(vector=>vector.length===parameters.length&&vector.every((value,index)=>value===parameters[index]));
}

/** 📻️ Keeps original process authority while receiving its owning Bazel invocation's cancellation. */
async function withBazelCancellation<T>(environment:NodeJS.ProcessEnv,invocation:ScriptInvocation,run:(owned:ScriptInvocation)=>Promise<T>,cancel?:AbortSignal):Promise<T>{
 readIssuedScriptProcessEnvelopeV1(invocation);const url=environment.SEMIO_BAZEL_CONTROL_URL,match=url&&/^tcp:\/\/127\.0\.0\.1:(\d+)\/([a-f0-9-]+)$/u.exec(url);if(!match)throw refuseBuildV1("missing","bazel.cancellation-control-required");const cancelled=new AbortController(),socket=createConnection({host:"127.0.0.1",port:Number(match[1])});initializeBazelControlTextV1(socket);const token=match[2];let id:string|undefined,pending="",acknowledged=false,started=false,acceptJoin!:()=>void,refuseJoin!:(error:unknown)=>void,acceptClosed!:()=>void,refuseClosed!:(error:unknown)=>void;const joined=new Promise<void>((accept,refuse)=>{acceptJoin=accept;refuseJoin=refuse;}),terminated=new Promise<void>((accept,refuse)=>{acceptClosed=accept;refuseClosed=refuse;});void terminated.catch(()=>{});const fail=(error:unknown)=>{cancelled.abort(error);refuseJoin(error);refuseClosed(error);},interrupt=()=>{if(!started){const reason=invocation.control.signal.reason??refuseBuildV1("cancelled","bazel.handshake-cancelled");cancelled.abort(reason);refuseJoin(reason);if(!id)socket.destroy(reason);}},remaining=invocation.control.remainingMilliseconds(),handshake=remaining===null?undefined:setTimeout(()=>{refuseJoin(refuseBuildV1("cancelled","bazel.handshake-deadline-exhausted"));if(!id)socket.destroy();},remaining);invocation.control.signal.addEventListener("abort",interrupt,{once:true});if(invocation.control.signal.aborted)interrupt();cancel?.addEventListener("abort",()=>cancelled.abort(cancel.reason??refuseBuildV1("cancelled","bazel.work-request-cancelled")),{once:true});if(cancel?.aborted)cancelled.abort(cancel.reason);
 socket.on("error",fail);socket.once("close",()=>{if(!acknowledged)fail(refuseBuildV1("failed","bazel.action-control-disconnected"));});socket.once("connect",()=>socket.write(JSON.stringify({version:1,event:"join",token})+"\n"));socket.on("data",chunk=>{pending+=chunk.toString();if(Buffer.byteLength(pending)>policy.maximumControlMessageBytes){fail(refuseBuildV1("invalid","bazel.action-control-message-too-large"));socket.destroy();return;}let end:number;while((end=pending.indexOf("\n"))>=0){let message;try{message=JSON.parse(pending.slice(0,end));}catch(error){fail(error);socket.destroy();return;}pending=pending.slice(end+1);if(!message||typeof message!=="object"||message.version!==1){fail(refuseBuildV1("invalid","bazel.action-control-invalid"));socket.destroy();return;}if(message.event==="joined"&&typeof message.id==="string"&&!id){id=message.id;if(!cancelled.signal.aborted)socket.write(JSON.stringify({version:1,event:"start",token,id})+"\n");}else if(message.event==="started"&&id&&!started){started=true;if(handshake)clearTimeout(handshake);acceptJoin();}else if(message.event==="cancelled")cancelled.abort(refuseBuildV1("cancelled","bazel.invocation-cancelled"));else if(message.event==="acknowledged"){acknowledged=true;acceptClosed();}else{fail(refuseBuildV1("invalid","bazel.action-control-event-invalid"));socket.destroy();return;}}});
 return withBazelTaskCleanupV1(async()=>{await joined;return withScriptInvocationSignalV1(invocation,cancelled.signal,run);},async()=>{if(handshake)clearTimeout(handshake);invocation.control.signal.removeEventListener("abort",interrupt);const timer=setTimeout(()=>socket.destroy(refuseBuildV1("cancelled","bazel.action-termination-receipt-deadline-exhausted")),policy.terminationReceiptMilliseconds);try{if(id&&!socket.destroyed)socket.write(JSON.stringify({version:1,event:"closed",token,id})+"\n");await terminated;}finally{clearTimeout(timer);socket.destroy();}});
}

/** 📖️ Reads finite birth material without treating its contents as authority. */
export async function readBazelBirthFileV1(path:string,maximumBytes:number,deadline:()=>void):Promise<Buffer>{
 deadline();const file=await open(path,constants.O_RDONLY|constants.O_NOFOLLOW);let failed=false;try{const before=await file.stat({bigint:true});if(!before.isFile()||before.nlink!==1n||before.size>BigInt(maximumBytes))throw refuseBuildV1("invalid","bazel.birth-input-required");const bytes=Buffer.alloc(Number(before.size));let offset=0;while(offset<bytes.length){deadline();const row=await file.read(bytes,offset,Math.min(birthSchema.definitions.Protocol.const.readChunkBytes,bytes.length-offset),offset);deadline();if(row.bytesRead===0)throw refuseBuildV1("stale","bazel.birth-input-shortened");offset+=row.bytesRead;}const after=await file.stat({bigint:true}),named=await lstat(path,{bigint:true});deadline();if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key as keyof typeof before]!==after[key as keyof typeof after]||before[key as keyof typeof before]!==named[key as keyof typeof named]))throw refuseBuildV1("stale","bazel.birth-input-changed",{path:String(path)});return bytes;}catch(error){failed=true;throw error;}finally{try{await file.close();}catch(error){if(!failed)throw error;}}
}

/** 🌡️ What a birth handshake measures with: the monotonic clock, the wall clock of the owner's deadline, the host's contention and the timers; every part has a real default, laws replace them with a simulated host. */
export type BazelBirthClockV1=Readonly<{now?:()=>number;epoch?:()=>number;source?:ContentionSourceV1;schedule?:(run:()=>void,milliseconds:number)=>unknown;cancel?:(handle:unknown)=>void}>;

/** ⏱️ The handshake clock of one action birth, counted as work like every other budget (`🧪️testing/🎚️budget/⏲️contention`): until the parent says it is `waiting` or that the action `started`, every wall millisecond is worth `1/stretch` of `maximumMilliseconds`, the stretch being the host's contention clamped to the policy bounds, so a busy host stretches the budget and a hung parent is still stopped after at most `stretch` times it. `waiting` (the birth is accepted and now waits for the run's grant to grow and for a job of it) ends the work clock for good: from then on only the owner's own deadline bounds the birth (no deadline: it waits as long as the parent is alive and the action is not cancelled), so a quiet grant's growth in the fair queue never fails the action, and neither `check` nor the timer can fail it. `settle` ends it as well, so a child that did its work is never cancelled by the handshake. `check` is the one poll of the pre-start steps, so no step keeps a second wall clock; `arm` starts the timer once the socket exists and the owner's deadline is known. */
export function bazelBirthHandshakeV1(maximumMilliseconds:number,started:number,options:BazelBirthClockV1={}):Readonly<{check:()=>void;arm:(deadlineEpochMilliseconds:number|null,fail:(error:unknown)=>void)=>void;waiting:()=>void;settle:()=>void;phase:()=>"birth"|"waiting"|"settled"}>{
 const now=options.now??(()=>performance.now()),epoch=options.epoch??(()=>Date.now()),source=options.source??sharedHostContentionV1(),schedule=options.schedule??((run:()=>void,milliseconds:number)=>setTimeout(run,milliseconds)),cancel=options.cancel??((handle:unknown)=>clearTimeout(handle as ReturnType<typeof setTimeout>)),clock=createBudgetClockV1(maximumMilliseconds),tick=CONTENTION_POLICY_V1.tickMilliseconds;
 let phase:"birth"|"waiting"|"settled"="birth",last=started,handle:unknown,deadline:number|null=null,failure:((error:unknown)=>void)|undefined;
 const stretch=():number=>{try{return budgetStretchV1(source().ratio);}catch{return 1;}},left=():number|undefined=>deadline===null?undefined:Math.max(1,deadline-epoch());
 const spent=():boolean=>{const at=now();if(at-started<maximumMilliseconds)return false;clock.advance(at-last,stretch());last=at;return clock.expired();};
 const untilExpiry=():number=>{const elapsed=now()-started;return elapsed<maximumMilliseconds?maximumMilliseconds-elapsed:Math.min(tick,clock.wallUntilExpiryMilliseconds(stretch()));};
 const stop=():void=>{if(handle!==undefined)cancel(handle);handle=undefined;};
 const rearm=():void=>{stop();if(phase==="settled"||!failure)return;const rest=left(),work=phase==="birth"?Math.max(1,Math.ceil(untilExpiry())):undefined,next=work===undefined?rest:rest===undefined?work:Math.min(work,rest);if(next!==undefined)handle=schedule(fire,next);};
 const fire=():void=>{handle=undefined;if(phase==="settled"||!failure)return;if(deadline!==null&&epoch()>=deadline){failure(refuseBuildV1("cancelled","bazel.birth-deadline-exhausted"));return;}if(phase==="birth"&&spent()){failure(refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted"));return;}rearm();};
 return Object.freeze({
  check:()=>{if(phase==="birth"&&spent())throw refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted");},
  arm:(until:number|null,fail:(error:unknown)=>void)=>{deadline=until;failure=fail;rearm();},
  waiting:()=>{if(phase!=="birth")return;phase="waiting";rearm();},
  settle:()=>{phase="settled";stop();},
  phase:()=>phase
 });
}

/** 🐣️ Obtains unchanged current authority from the live owned parent before the actual action child is born; the parent may first say that it is `waiting` (for the grant's growth and a job of it), which ends the handshake clock. */
async function launchBazelActionBirthV1(kind:string,file:string,parameters:readonly string[],implementation:BazelImplementationV1,worker?:AbortSignal):Promise<void>{
 const protocol=birthSchema.definitions.Protocol.const,started=performance.now(),handshake=bazelBirthHandshakeV1(protocol.maximumHandshakeMilliseconds,started),initial=()=>handshake.check(),manifestPath=await realpath(resolve(file)),manifestBytes=await readBazelBirthFileV1(manifestPath,protocol.maximumManifestBytes,initial),manifest=JSON.parse(manifestBytes.toString("utf8")) as BazelTaskManifestV1;requireBazelManifestV1(manifest,implementation);if(typeof manifest.task.id!=="string")throw refuseBuildV1("missing","bazel.birth-rendezvous-required");const environment=(await readBazelRuntimeV1(manifestPath,implementation,initial)).environment,expected=environment.SEMIO_SCRIPT_PROCESS_INVOCATION;if(typeof expected!=="string")throw refuseBuildV1("missing","bazel.birth-wire-expectation-required");const inProcess=worker!==undefined&&bazelWorkerInProcessV1(manifest,environment);const envelope=readScriptProcessEnvelope(JSON.parse(expected),Date.now()),remaining=()=>{initial();if(envelope.deadlineEpochMilliseconds!==null&&envelope.deadlineEpochMilliseconds<=Date.now())throw refuseBuildV1("cancelled","bazel.birth-deadline-exhausted");},match=/^tcp:\/\/127\.0\.0\.1:(\d+)\/([a-f0-9-]+)$/u.exec(environment.SEMIO_BAZEL_CONTROL_URL??"");if(!match)throw refuseBuildV1("missing","bazel.birth-owned-rendezvous-required");remaining();
 traceBazelImplementationV1(environment,manifest.task,kind);const birth={kind,task:manifest.task.id,manifestSha256:hash(manifestBytes),parameters:parameters.map(path=>resolve(path))};if(validateJsonSchemaSubset(birthSchema,birth).length||birth.parameters.some(path=>path.length>256))throw refuseBuildV1("missing","bazel.birth-required");const controller=new AbortController(),socket=createConnection({host:"127.0.0.1",port:Number(match[1])});initializeBazelControlTextV1(socket);let id:string|undefined,pending="",acknowledged=false,issued=false,acceptJoin!:(wire:string)=>void,refuseJoin!:(error:unknown)=>void,acceptClosed!:()=>void,refuseClosed!:(error:unknown)=>void;const joined=new Promise<string>((accept,refuse)=>{acceptJoin=accept;refuseJoin=refuse;}),closed=new Promise<void>((accept,refuse)=>{acceptClosed=accept;refuseClosed=refuse;});void closed.catch(()=>{});const fail=(error:unknown)=>{controller.abort(error);refuseJoin(error);refuseClosed(error);},send=(value:object)=>{const message=JSON.stringify(value)+"\n";if(Buffer.byteLength(message)>policy.maximumControlMessageBytes)throw refuseBuildV1("invalid","bazel.birth-message-too-large");socket.write(message);};handshake.arm(envelope.deadlineEpochMilliseconds,fail);
 socket.on("error",fail);socket.once("close",()=>{if(!acknowledged)fail(refuseBuildV1("failed","bazel.birth-parent-disconnected"));});socket.once("connect",()=>{try{remaining();send({version:1,event:"join",token:match[2],birth});}catch(error){fail(error);}});socket.on("data",chunk=>{pending+=chunk.toString();if(Buffer.byteLength(pending)>policy.maximumControlMessageBytes){fail(refuseBuildV1("invalid","bazel.birth-response-too-large"));return;}let end:number;while((end=pending.indexOf("\n"))>=0){try{const message=JSON.parse(pending.slice(0,end));pending=pending.slice(end+1);if(message.version!==1)throw refuseBuildV1("refused","bazel.birth-response-refused");if(message.event==="joined"&&!id&&typeof message.id==="string"&&message.wire===expected){remaining();id=message.id;send({version:1,event:"start",token:match[2],id});}else if(message.event===protocol.waitingEvent&&!id&&!issued)handshake.waiting();else if(message.event==="started"&&id&&!issued){issued=true;handshake.settle();acceptJoin(expected);}else if(message.event==="cancelled")fail(refuseBuildV1("cancelled","bazel.birth-parent-cancelled"));else if(message.event==="acknowledged"){acknowledged=true;acceptClosed();}else throw refuseBuildV1("refused","bazel.birth-response-refused");}catch(error){fail(error);}}});
 worker?.addEventListener("abort",()=>fail(worker.reason??refuseBuildV1("cancelled","bazel.work-request-cancelled")),{once:true});if(worker?.aborted)fail(worker.reason??refuseBuildV1("cancelled","bazel.work-request-cancelled"));
 let primaryFailed=false,primaryError:unknown;const testStarted=performance.now();try{const wire=await joined;await withScriptProcessEnvelope(readScriptProcessEnvelope(JSON.parse(wire),Date.now()),original=>withScriptInvocationSignalV1(original,controller.signal,async invocation=>{await advanceScriptInvocation(invocation,"bazel:owned-action-child","running");const childEnvironment=scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256,...(kind==="test"?bazelTestEnvironmentV1(process.env):{})});if(inProcess)await inProcessLockV1(()=>runBazelActionInProcessV1([kind,manifestPath,...birth.parameters],implementation,childEnvironment,worker));else await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,kind,manifestPath,...birth.parameters],process.cwd(),"bazel:owned-action-child",scriptInvocationBudget(invocation,0),{env:childEnvironment,signal:invocation.control.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:owned-action-child","running")});checkScriptInvocation(invocation);}));}catch(error){primaryFailed=true;primaryError=error;throw error;}finally{if(kind==="test")try{settleBazelTestV1(process.env,{label:manifest.task.id,durationMs:performance.now()-testStarted,...(primaryFailed?{failure:String((primaryError as Error|undefined)?.message??primaryError).slice(0,2000)}:{})});}catch(error){process.stderr.write(testProtocolMessageV1("reportFailed",{reason:String(error)})+"\n");}handshake.settle();const timer=setTimeout(()=>socket.destroy(refuseBuildV1("cancelled","bazel.birth-termination-receipt-deadline-exhausted")),policy.terminationReceiptMilliseconds);try{if(id&&!socket.destroyed){send({version:1,event:"closed",token:match[2],id});await closed;}}catch(error){if(!primaryFailed)throw error;}finally{clearTimeout(timer);socket.destroy();}}
}

/** 🚀️ Places the pinned runtime as one action's launcher without paying its bytes again: a hard link when both sit on one volume, else a clone of the single copy kept in the output base (APFS, btrfs and XFS clones share blocks; other volumes fall back to a plain copy), so 69 launchers of 59 MB cost one copy instead of 4 GB per output base. */
export async function placeBazelLauncherV1(source:string,output:string,hardLink:typeof link=link):Promise<void>{
 try{await hardLink(source,output);return;}catch{}
 const parts=output.split(sep),marker=parts.lastIndexOf("execroot");
 if(marker>0)try{
  const facts=await stat(source),shared=join(parts.slice(0,marker).join(sep),"launchers",createHash("sha256").update(source+"\0"+facts.size+"\0"+facts.mtimeMs).digest("hex").slice(0,32));
  if(!await stat(shared).then(()=>true,()=>false)){await mkdir(dirname(shared),{recursive:true});const temporary=shared+"."+randomUUID();try{await copyFile(source,temporary);await rename(temporary,shared);}catch(error){await rm(temporary,{force:true});if(!await stat(shared).then(()=>true,()=>false))throw error;}}
  await copyFile(shared,output,constants.COPYFILE_FICLONE);return;
 }catch{}
 await copyFile(source,output);
}
/** 🏃️ Runs one captured action, launcher or launch under the original birth the caller holds. */
async function runBazelActionV1(argv:string[],implementation:BazelImplementationV1,cancel?:AbortSignal):Promise<void>{
 const [command,file,...parameters]=argv,output=command==="launch"||command==="test"?undefined:parameters.shift(),dependencies=parameters;if(!["action","launch","launcher","test"].includes(command??"")||!file||command!=="launch"&&command!=="test"&&!output)throw refuseBuildV1("invalid","bazel.action-manifest-output-expected");
 const actualBirth=readScriptProcessEnvelope(JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION??"null"),Date.now()),checkBirth=()=>{if(actualBirth.deadlineEpochMilliseconds!==null&&actualBirth.deadlineEpochMilliseconds<=Date.now())throw refuseBuildV1("cancelled","bazel.child-deadline-exhausted");},manifestPath=await realpath(resolve(file)),manifestBytes=await readBazelBirthFileV1(manifestPath,birthSchema.definitions.Protocol.const.maximumManifestBytes,checkBirth);if(!/^[a-f0-9]{64}$/u.test(process.env.SEMIO_BAZEL_MANIFEST_SHA256??"")||hash(manifestBytes)!==process.env.SEMIO_BAZEL_MANIFEST_SHA256)throw refuseBuildV1("stale","bazel.birth-manifest-changed");const manifest=JSON.parse(manifestBytes.toString("utf8")) as BazelTaskManifestV1;requireBazelManifestV1(manifest,implementation);const runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;traceBazelImplementationV1(environment,manifest.task,command);
 await receiveScriptProcessInvocation(environment,async original=>{if(command==="launcher"){await placeBazelLauncherV1(process.execPath,resolve(output!));await chmod(resolve(output!),0o755);return;}await withBazelCancellation(environment,original,async invocation=>{const actionPhase=createIssuedResourceControlV1(invocation,"bazel:task-admission",0),run=async()=>withRepositoryCacheStoreV1(runtime.workspace,"graph",invocation,actionPhase,async(_graph,graphPhase)=>{for(const dependency of dependencies)await restoreBazelSnapshotV1(runtime.workspace,resolve(dependency),invocation,actionPhase);const previous=process.env;process.env={...previous,...environment};try{await executeTask(manifest,runtime.workspace,invocation,graphPhase,bazelFailureTailDirectoryV1(dirname(manifestPath)));if(command==="action")await captureBazelSnapshotV1({workspace:runtime.workspace,task:manifest.task},resolve(output!),invocation);}finally{process.env=previous;}});if(command==="launch")await run();else await withResourceLeases({directory:join(runtime.workspace,".cache/b/l/m/data"),signal:invocation.control.signal,control:actionPhase,resources:[{resource:"bazel-task-schedule-"+hash(dirname(manifestPath)).slice(0,16),mode:manifest.task.definition.parallelism===false?"exclusive":"shared"}]},run);},cancel);});
}

/** ▶️ Runs the action body in this process under the environment its own child process would have received. */
async function runBazelActionInProcessV1(argv:string[],implementation:BazelImplementationV1,environment:NodeJS.ProcessEnv,cancel?:AbortSignal):Promise<void>{
 const previous=process.env;process.env={...environment};
 try{await runBazelActionV1(argv,implementation,cancel);}finally{process.env=previous;}
}

let workerPlanV1:WorkerPlanV1|undefined,inProcessTailV1:Promise<unknown>=Promise.resolve();

/** 🔐️ Runs in-process action bodies one at a time, because they borrow the process environment; actions that run in child processes stay concurrent. */
function inProcessLockV1<T>(run:()=>Promise<T>):Promise<T>{
 const next=inProcessTailV1.then(run,run);inProcessTailV1=next.catch(()=>{});return next;
}

/** 🧵️ Tells whether the plan of this invocation runs the task's action kind in the worker process. */
function bazelWorkerInProcessV1(manifest:BazelTaskManifestV1,environment:NodeJS.ProcessEnv):boolean{
 const encoded=environment.SEMIO_BAZEL_WORKER_PLAN;if(!encoded)return false;
 const plan=JSON.parse(encoded);requireWorkerPlanV1(plan);workerPlanV1=plan;
 return plan.kinds[workerActionKindV1(workerPolicy,manifest.task.target,manifest.task.definition.continuous===true)].inProcess;
}

/** 📬️ Handles one work request: manifest, output and dependency snapshots name one action birth. */
async function runBazelWorkRequestV1(parameters:readonly string[],implementation:BazelImplementationV1,worker?:AbortSignal):Promise<void>{
 const [manifest,output,...snapshots]=parameters;
 if(!manifest||!output)throw refuseBuildV1("missing","bazel.work-request-manifest-required");
 await launchBazelActionBirthV1("action",manifest,[output,...snapshots],implementation,worker);
}

/** 💼️ Serves a persistent Bazel worker over the JSON worker protocol, or runs the single request of one flagfile like any other action. Everything the actions print is returned in their responses, so standard output carries nothing but responses. */
async function serveBazelWorkV1(args:readonly string[],implementation:BazelImplementationV1,recheck?:()=>void):Promise<void>{
 if(!args.includes("--persistent_worker")){
  const flagfile=args[0];if(args.length!==1||!flagfile?.startsWith("@"))throw refuseBuildV1("usage","bazel.work-action-arguments-expected");
  await runBazelWorkRequestV1((await readFile(flagfile.slice(1),"utf8")).split("\n").filter(Boolean),implementation);return;
 }
 const capture=installWorkerOutputCaptureV1();workerChildInputV1="";
 try{
  await serveWorkerV1({input:readWorkerInputV1(),write:capture.writeProtocol,concurrency:1024,
   handler:async(request,{signal})=>{const result=await capture.run(async()=>{try{recheck?.();await runBazelWorkRequestV1(request.arguments,implementation,signal);return 0;}catch(error){process.stderr.write(String((error as Error)?.stack??error)+"\n");return 1;}});return {exitCode:result.value,output:result.output};},
   afterRequest:served=>{const plan=workerPlanV1;if(plan&&(served%plan.resetAfterRequests===0||process.memoryUsage.rss()>plan.memory.perWorkerMegabytes*1048576))runtimeCollectGarbageV1(true);}
  });
 }finally{capture.restore();workerChildInputV1=undefined;}
}

/** 🎼️ The planner of orchestrated tasks: only the adapter implementation carries the planning closure, so the narrow action implementation refuses them. */
export type BazelOrchestrationV1=(invocation:ScriptInvocation)=>Parameters<typeof withBuildOrchestrationTaskEnvironmentV1>[7];

let bazelOrchestrationV1:BazelOrchestrationV1|undefined;

/** 🔌 Installs the planner an adapter process lends to the orchestrated tasks it executes. */
export function installBazelOrchestrationV1(orchestration:BazelOrchestrationV1):void{bazelOrchestrationV1=orchestration;}

/** 🗺️ Plans the children of an orchestrated task or refuses when this implementation has no planner. */
function bazelOrchestrationPlanV1(invocation:ScriptInvocation):Parameters<typeof withBuildOrchestrationTaskEnvironmentV1>[7]{
 if(!bazelOrchestrationV1)throw refuseBuildV1("refused","bazel.orchestrated-tasks-adapter-only");
 return bazelOrchestrationV1(invocation);
}

/** 🔭️ Opens the opt-in source trace of this process when the caller named a directory: it records which captured modules and Sources an action class really touches, the corpus that proves a narrow implementation scope. */
function traceBazelImplementationV1(environment:NodeJS.ProcessEnv,task:BazelTaskV1,command:string):void{
 const directory=environment.SEMIO_BUILD_SOURCE_TRACE,trace=(globalThis as {[key:symbol]:{enable(directory:string):void;record(kind:string,text:string):void}|undefined})[Symbol.for("semio.sourceTrace.v1")];
 if(!directory||!trace)return;
 const semio=(task.definition.metadata as {semio?:{execution?:{kind?:string}}}|undefined)?.semio;
 trace.enable(directory);trace.record("a",JSON.stringify({command,id:task.id,workerKind:workerActionKindV1(workerPolicy,task.target,task.definition.continuous===true),executionKind:task.definition.execution.kind,semioKind:semio?.execution?.kind??"none",owner:task.definition.execution.nativeOwnerCommand===undefined?"none":"native"}));
}

/** 🧊️ Binds the captured proof of this process to the exact snapshot, loader and runner it was started from. */
export function bazelCapturedImplementationV1(proof?:{version:1;snapshot:string;sha256:string;loaderSha256?:string;recheck?:()=>void}):BazelImplementationV1{
 if(!proof||proof.version!==1||import.meta.semioCapturedImplementationIdentity!==proof.sha256||import.meta.semioCapturedLoaderIdentity!==proof.loaderSha256||proof.loaderSha256!==hash(buildSourceClosureLoaderV1))throw refuseBuildV1("missing","bazel.loader-identity-required");
 const implementation:BazelImplementationV1={version:1,snapshot:proof.snapshot,runner:join(dirname(proof.snapshot),"📜️script.ts"),sha256:proof.sha256,loaderSha256:proof.loaderSha256};requireBazelImplementationV1(implementation,implementation);return implementation;
}

/** ⛔️ Executes one action-side entry of a captured implementation: a persistent worker, a birth or a direct action. */
export async function runBazelActionEntryV1(argv:string[],implementation:BazelImplementationV1,recheck?:()=>void):Promise<void>{
 if(argv[0]==="work-action"){await serveBazelWorkV1(argv.slice(1),implementation,recheck);return;}
 if(argv[0]?.startsWith("birth-")){await launchBazelActionBirthV1(argv[0].slice(6),argv[1]!,argv.slice(2),implementation);}else await runBazelActionV1(argv,implementation);
}

/** 🔨️ Executes only the checksum-bound captured action implementation inside the actual child birth. */
export async function runCapturedBuildSourceV1(argv:string[],proof?:{version:1;snapshot:string;sha256:string;loaderSha256?:string;recheck?:()=>void}):Promise<void>{
 await runBazelActionEntryV1(argv,bazelCapturedImplementationV1(proof),proof?.recheck);
}

if(isMainModuleV1(import.meta.url))throw refuseBuildV1("missing","bazel.task-entry-loader-required");
