import {buildPreparationBudgetMs,progressDeadlinePolicyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {withRepositoryPublishedCargoExecutableV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {scriptInvocationEnvironment,scriptResourceProcessEnvironmentV1,withRequestedIssuedResourceControlV1,withRenewableIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type CargoDependencyNetworkV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/📥️dependencies/📋️contract/🟦️.ts";
import {CARGO_BINARY_SOURCES_SCHEMA_V1,cargoBinarySourcesFreshnessV1,parseCargoBinarySourcesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {sayBootstrapV1} from "../../📖️texts/🟦️.ts";
import {readBuildMode,type BuildMode} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {compilerAdmissionBudgetV1,type CompilerAdmissionControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand,startOwnedCommandTimeoutV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import installationSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import orchestrationSchema from "../../../../🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json" with { type: "json" };
import executionSchema from "../../../../🏗️build/🏃️execution/🧬️schema/🔣️.json" with { type: "json" };
export type DashboardExecutionOptions={readonly mode:BuildMode;readonly network:CargoDependencyNetworkV1;readonly control:CompilerAdmissionControl;readonly invocation:ScriptInvocation;readonly workspace:string};
import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync, statSync } from "node:fs";
import { readdir, stat, open } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, basename, join, relative, resolve, isAbsolute } from "node:path";
import { spawn } from "node:child_process";
import { terminateOwnedChildTree } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

const cache=(root:string)=>join(root,".🧬semio/🦑️repo/⚡️cache");
const manifest=(root:string,mode:BuildMode)=>join(cache(root),"🎛️dashboard",readBuildMode(mode),"installed.json");
const slashes=(path:string)=>path.replaceAll("\\","/");

/** 📦️ The crate directory of the dashboard executable; its sources live in the sibling modules of `🎛️dashboard`. */
export const dashboardPackageRoot=resolve(import.meta.dirname,installationSchema.definitions.PackageOwner.const.installationRelative);

/** 🪵️ The source trees an installation was built from and the cheap digest they had at that moment. */
export type DashboardSources={readonly roots:readonly string[];readonly manifests:readonly string[];readonly digest:string};

/** 🗃️ What an installed executable was compiled from: the files of its own Cargo dep-info, relative to the installation root, and when that build started. */
export type DashboardCompiledSources={readonly builtAtMs:number;readonly sources:readonly string[]};

/** 🧾️ Checks current host, mode and immutable path against the closed installation contract. */
function validDashboardRecord(value:unknown,mode:BuildMode):boolean{
 if(validateJsonSchemaSubset(installationSchema,value).length)return false;
 const record=value as {platform:string;mode:string;path:string;digest:string},name=process.platform==="win32"?"semio.exe":"semio";
 return record.platform===process.platform&&record.mode===mode&&record.path===`.cache/b/t/m/dashboard-cli/${mode}/${record.digest}/${name}`;
}

const skippedDirectories=new Set(["target","node_modules","dist","build",".git","🗑️generated","🧪️tests","🧫️fixtures","coverage","__pycache__","storybook-static","temp"]);
const skippedFiles=/\.(?:tsx?|mts|mjs|js|py|log|md|feature|tsv|tsbuildinfo)$/;
const workspaceFiles=installationSchema.definitions.SourceInputProtocol.const.rootFiles;

/** 🔎️ Whether this host and mode have a valid installation record; no executable bytes are read. */
export function dashboardInstalled(root:string,mode:BuildMode):boolean { try{installedDashboard(root,mode,root);return true;}catch{return false;} }

/** 📍️ Resolves the installation record before the finite tools reader admits executable bytes. */
export function installedDashboard(root:string,mode:BuildMode,workspace:string):string {
  const file=manifest(root,mode);
  if(!existsSync(file))throw new Error("Dashboard is not installed. Start repo dashboard to install it automatically.");
  const value=JSON.parse(readFileSync(file,"utf8"));
  if(!validDashboardRecord(value,mode))throw new Error("Invalid dashboard installation record. Start repo dashboard to rebuild it automatically.");
  const executable=resolve(workspace,value.path);
  return executable;
}

/** 🧭️ The source roots of one crate: its module directory (`<module>/📦️packages/🦀️rust` owns its siblings) or itself. */
function crateRoot(directory:string):string {
  return basename(directory)==="🦀️rust"&&basename(dirname(directory))==="📦️packages"?dirname(dirname(directory)):directory;
}

type CargoTable=Readonly<Record<string,unknown>>;
const cargoTable=(value:unknown):CargoTable=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as CargoTable:{};

/** 🌳️ Resolves actual workspace manifests and the selected binary's dependency lock. */
function sourceInputs(packageRoot:string,workspace:string):Pick<DashboardSources,"roots"|"manifests"> {
  workspace=resolve(workspace);const cache=new Map<string,CargoTable>(),seen=new Set<string>(),pending=[resolve(packageRoot)],roots=new Set<string>(),manifests=new Set<string>();
  const inside=(path:string)=>{const local=relative(workspace,path);if(local===".."||local.startsWith("../")||local.startsWith("..\\")||isAbsolute(local))throw Error("Dashboard Cargo source must belong to the workspace");return path;};
  const read=(directory:string)=>{const file=join(inside(directory),"Cargo.toml");let value=cache.get(file);if(!value){value=cargoTable(runtimeParseTomlV1(readFileSync(file,"utf8")));cache.set(file,value);}return value;};
  const owner=(directory:string,manifest:CargoTable)=>{const declared=cargoTable(manifest.package).workspace;if(declared!==undefined){if(typeof declared!=="string")throw Error("Declared Cargo workspace path required");const actual=inside(resolve(directory,declared));if(!("workspace" in read(actual)))throw Error("Declared Cargo workspace manifest required");return actual;}for(let candidate=directory;;candidate=dirname(candidate)){if(existsSync(join(candidate,"Cargo.toml"))&&"workspace" in read(candidate))return candidate;if(candidate===workspace)break;}return directory;};
  const selected=inside(resolve(packageRoot));manifests.add(slashes(relative(workspace,join(owner(selected,read(selected)),installationSchema.definitions.SourceInputProtocol.const.selectedWorkspaceLock))));
  while(pending.length){
    const directory=inside(pending.pop()!);
    if(seen.has(directory))continue;
    seen.add(directory);
    const file=join(directory,"Cargo.toml");
    if(!existsSync(file))continue;
    const manifest=read(directory),workspaceRoot=owner(directory,manifest),shared=cargoTable(cargoTable(read(workspaceRoot).workspace).dependencies);
    manifests.add(slashes(relative(workspace,join(workspaceRoot,"Cargo.toml"))));
    roots.add(crateRoot(directory));
    const blocks=[manifest.dependencies,manifest["build-dependencies"],...Object.values(cargoTable(manifest.target)).flatMap(value=>{const target=cargoTable(value);return[target.dependencies,target["build-dependencies"]];})];
    for(const block of blocks)for(const[name,value]of Object.entries(cargoTable(block))){const dependency=cargoTable(value);if(typeof dependency.path==="string")pending.push(resolve(directory,dependency.path));else if(dependency.workspace===true){if(!(name in shared))throw Error("Declared inherited Cargo dependency required: "+name);const inherited=cargoTable(shared[name]);if(typeof inherited.path==="string")pending.push(resolve(workspaceRoot,inherited.path));}}
  }
  const ordered=[...roots].sort((a,b)=>a.length-b.length).filter((root,index,all)=>!all.slice(0,index).some(parent=>root===parent||root.startsWith(parent+"/")||root.startsWith(parent+"\\")));
  return{roots:ordered.map(root=>slashes(relative(workspace,root))).sort(),manifests:[...manifests].sort()};
}

/** 🪪️ Retains exact filesystem identity and nanosecond metadata without reading Source bytes. */
function sourceSignature(name:string,facts:Readonly<{dev:bigint;ino:bigint;size:bigint;mtimeNs:bigint;ctimeNs:bigint}>):string{
 return[slashes(name),...installationSchema.definitions.SourceSignatureProtocol.const.metadataFields.map(field=>String(facts[field as keyof typeof facts]))].join("\0");
}

/** 🏃️ Observes every relevant Source file below one root concurrently. */
async function collect(directory:string,workspace:string,entries:string[]):Promise<void> {
  let children;
  try { children=await readdir(directory,{withFileTypes:true}); } catch { return; }
  await Promise.all(children.map(async child=>{
    const path=join(directory,child.name);
    if(child.isDirectory()){if(!skippedDirectories.has(child.name))await collect(path,workspace,entries);return;}
    if(skippedFiles.test(child.name))return;
    try { const info=await stat(path,{bigint:true});entries.push(sourceSignature(relative(workspace,path),info)); } catch {}
  }));
}

/** 🔏️ Digests file identity, size and nanosecond timestamps; no Source content is read. */
export async function digestDashboardSources(workspace:string,sources:Pick<DashboardSources,"roots"|"manifests">):Promise<string> {
  const entries:string[]=[];
  await Promise.all([...sources.roots.map(root=>collect(resolve(workspace,root),workspace,entries)),...[...new Set([...workspaceFiles,...sources.manifests])].map(async name=>{
    try { const info=await stat(join(workspace,name),{bigint:true});entries.push(sourceSignature(name,info)); } catch {}
  })]);
  const ordered=[...new Set(entries)].sort();
  return `${ordered.length}:${createHash("sha1").update(ordered.join("\n")).digest("hex")}`;
}

/** 📸️ Captures the source roots and their digest; take it before building so edits made during the build stay detectable. */
export async function captureDashboardSources(packageRoot:string,workspace:string):Promise<DashboardSources> {
  const sources=sourceInputs(packageRoot,workspace);
  return {...sources,digest:await digestDashboardSources(workspace,sources)};
}

/** 🕰️ The first compiled source that is missing or was written, replaced or re-linked at or after the build started, or nothing: Cargo's own freshness rule (`cargoBinarySourcesFreshnessV1`) over the whole dependency closure of the binary, read on the later of modification and inode change time so a replacement that keeps old timestamps counts. Only stats, no bytes. */
export function changedDashboardSourceV1(root:string,compiled:DashboardCompiledSources):string|undefined{
 return cargoBinarySourcesFreshnessV1({schema:CARGO_BINARY_SOURCES_SCHEMA_V1,...compiled},path=>{try{const facts=statSync(resolve(root,path));return Math.max(facts.mtimeMs,facts.ctimeMs);}catch{return null;}}).changed??undefined;
}

/** 📥️ Admits the sources record Cargo published beside the executable as installation-root-relative paths; a missing or malformed record refuses the installation. */
function compiledDashboardSourcesV1(root:string,file:string):DashboardCompiledSources{
 let text:string;try{text=readFileSync(file,"utf8");}catch{throw Error("Dashboard executable sources record required");}
 const record=parseCargoBinarySourcesV1(text);if(record===null)throw Error("Dashboard executable sources record required");
 return {builtAtMs:record.builtAtMs,sources:[...new Set(record.sources.map(source=>slashes(relative(root,resolve(root,source)))))].sort()};
}

/** ✅️ The installed executable and the number of files it was compiled from when this host and mode's installation records exactly these captured sources, none of its compiled sources changed since its build and its immutable bytes are still in place with their recorded size and permissions; nothing otherwise. Only stats, no bytes. */
export function currentDashboardV1(root:string,mode:BuildMode,workspace:string,sources:DashboardSources):{readonly executable:string;readonly compiled:number}|undefined{
 let record;try{record=JSON.parse(readFileSync(manifest(root,mode),"utf8"));}catch{return;}
 if(!validDashboardRecord(record,mode)||record.sources.digest!==sources.digest||JSON.stringify(record.sources.roots)!==JSON.stringify(sources.roots)||JSON.stringify(record.sources.manifests)!==JSON.stringify(sources.manifests)||changedDashboardSourceV1(root,record.compiled)!==undefined)return;
 const executable=resolve(workspace,record.path);
 try{const facts=lstatSync(executable);return facts.isFile()&&facts.size===record.bytes&&(facts.mode&0o777)===record.permissions?{executable,compiled:record.compiled.sources.length}:undefined;}catch{return;}
}

/** ⏭️ Says that a build of this mode has nothing to do, in the language of the command. */
export function sayDashboardCurrentV1(mode:BuildMode,sources:DashboardSources,compiled:number):void{sayBootstrapV1("dashboard.installation-current",{mode,files:Number.parseInt(sources.digest),compiled});}

/** ⏳️ Why the installed executable no longer matches the sources, or nothing while it is current. */
export async function staleDashboard(root:string,mode:BuildMode):Promise<string|undefined> {
  if(!existsSync(manifest(root,mode)))return "it is not installed";
  let record;
  try { record=JSON.parse(readFileSync(manifest(root,mode),"utf8")); } catch { return "the installation record is unreadable"; }
  const sources=record?.sources;
  if(!validDashboardRecord(record,mode))return "the installation record does not name its sources";
  const current=await digestDashboardSources(root,sources);
  if(current!==sources.digest){const before=Number.parseInt(sources.digest),after=Number.parseInt(current);return `its sources changed since installation (${before===after?`${after} files`:`${before} -> ${after} files`})`;}
  const changed=changedDashboardSourceV1(root,record.compiled);
  return changed===undefined?undefined:`its compiled source ${changed} changed after the build that produced it`;
}

/** 🧊️ Publishes immutable executable bytes and one atomic native installation record. */
export async function installDashboard(packageRoot:string,root:string,options:DashboardExecutionOptions,sources?:DashboardSources):Promise<string> {
 const mode=readBuildMode(options.mode),control=options.control,name=process.platform==="win32"?"semio.exe":"semio",recorded=sources??await captureDashboardSources(packageRoot,root);compilerAdmissionBudgetV1(control);if(validateJsonSchemaSubset(installationSchema.properties.sources,recorded).length)throw Error("Dashboard installation source ownership refused");
 const {pinExecutableArtifact}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");
 return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>withRepositoryPublishedCargoExecutableV1(join(packageRoot,"Cargo.toml"),name,options.workspace,mode,options.invocation,control,async (artifact,control)=>{
  const compiled=compiledDashboardSourcesV1(root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=await pinExecutableArtifact(artifact.path,join(tools.root,"dashboard-cli",mode),`dashboard-installation:${mode}`,{control,signal:control.signal,leaseDirectory:join(options.workspace,".cache/b/l/m/data"),onWait:async progress=>{compilerAdmissionBudgetV1(control);await control.publish(progress);await control.yieldContinuation();}});await tools.recheck();const path=slashes(relative(options.workspace,executable));if(isAbsolute(path)||path.startsWith("../"))throw Error("Dashboard installation must stay inside workspace");const target=manifest(root,mode),temporary=target+"."+process.pid+".tmp";mkdirSync(dirname(target),{recursive:true});writeFileSync(temporary,JSON.stringify({version:4,platform:process.platform,mode,path,sources:recorded,compiled,digest:artifact.receipt.digest,bytes:artifact.receipt.bytes,permissions:artifact.receipt.permissions})+"\n");compilerAdmissionBudgetV1(control);renameSync(temporary,target);return executable;
 }));
}

/** 🏪️ Holds actual tools image and immutable executable reader custody through its complete consumer lifetime. */
export async function withInstalledDashboardV1<T>(root:string,options:DashboardExecutionOptions,body:(executable:string)=>Promise<T>):Promise<T>{
 const mode=readBuildMode(options.mode),control=options.control;compilerAdmissionBudgetV1(control);
 return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>{const executable=installedDashboard(root,mode,options.workspace),record=JSON.parse(readFileSync(manifest(root,mode),"utf8")),expected=join(tools.root,"dashboard-cli",mode,record.digest,process.platform==="win32"?"semio.exe":"semio");if(executable!==expected)throw Error("Original finite dashboard tools ownership changed");return withResourceLeases({directory:join(options.workspace,".cache/b/l/m/data"),control,signal:control.signal,resources:[{resource:"artifact:"+dirname(executable),mode:"shared"}]},async()=>{await tools.recheck();for(let parent=dirname(executable);;parent=dirname(parent)){const facts=lstatSync(parent);if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Physical dashboard tools ancestry required");if(parent===tools.root)break;}const facts=lstatSync(executable);if(!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||facts.size!==record.bytes||(facts.mode&0o777)!==record.permissions||process.platform!=="win32"&&(facts.mode&0o111)===0)throw Error("Exact immutable dashboard executable required");const file=await open(executable,"r"),hash=createHash("sha256"),buffer=Buffer.alloc(65536);let bytes=0;try{for(;;){checkResourceLeaseControl(control);const row=await file.read(buffer,0,buffer.length,null);if(!row.bytesRead)break;bytes+=row.bytesRead;if(bytes>record.bytes)throw Error("Dashboard executable changed during admission");hash.update(buffer.subarray(0,row.bytesRead));if(bytes%4194304===0){await control.publish({resource:"dashboard:installed-reader",mode:"shared",elapsedMs:0});await control.yieldContinuation();}}}finally{await file.close();}if(bytes!==record.bytes||hash.digest("hex")!==record.digest)throw Error("Dashboard executable content changed");const result=await body(executable);await tools.recheck();checkResourceLeaseControl(control);return result;});});
}

/** 🚀️ Recognizes native installed-tool launches while retaining the build runner for authoring tasks. */
export function dashboardInvocation(args:readonly string[]):string[]|undefined {
  if(args[0]!=="run")return;
  const prefix="@semio-tech/repo-dashboard-rs:",target=args[1]?.startsWith(prefix)?args[1].slice(prefix.length):undefined;
  if(!target||!["run","daemon","preferences"].includes(target))return;
  const rest=args.slice(2),separator=rest.indexOf("--"),options=separator<0?rest:rest.slice(0,separator);
  if(options.some(option=>/^--(?:graph|batch|parallel|exclude-dependencies)(?:=|$)/.test(option)))return;
  const forwarded=options.filter(option=>!/^--(?:output)=stream$/.test(option)&&!["--skip-cache"].includes(option));
  return [...(target==="run"?[]:[target]),...forwarded,...(separator<0?[]:rest.slice(separator+1))];
}

/** 🧩️ Reads exact conditional children from the physical dashboard owner before Original issuance. */
export function dashboardBuildDescendantsV1(args:readonly string[]):string[][] {
  const prefix="@semio-tech/repo-dashboard-rs:";if(args[0]!=="run"||!args[1]?.startsWith(prefix))return[];
  const target=args[1].slice(prefix.length).split(":")[0]!,file=join(dashboardPackageRoot,"📋️project.json"),facts=lstatSync(file);
  if(file.length>256||!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||facts.size>4194304)throw Error("Physical dashboard bootstrap owner required");
  const owner=JSON.parse(readFileSync(file,"utf8"));if(owner.name!==prefix.slice(0,-1))throw Error("Exact dashboard bootstrap owner required");
  const declaration=owner.targets?.[target]?.metadata?.semio?.execution;if(declaration===undefined)return[];
  if(declaration?.kind!=="orchestration"){if(validateJsonSchemaSubset(executionSchema,declaration).length)throw Error("Closed dashboard bootstrap declaration required");return[];}
  if(validateJsonSchemaSubset(orchestrationSchema,declaration).length)throw Error("Closed dashboard bootstrap declaration required");
  return declaration.children.map((child:{project:string;target:string;configuration?:string})=>{if(child.project!==owner.name||!Object.hasOwn(owner.targets,child.target))throw Error("Owned dashboard bootstrap child required");return["run",child.project+":"+child.target+(child.configuration?":"+child.configuration:"")];});
}

/** 🛟️ Whether these native arguments only observe or end what already runs, so an installation that is behind the sources may still answer them: a broken working tree must not stop a developer from stopping the daemon. */
export function mayRunStale(args:readonly string[]):boolean {
  const [verb,action]=args;
  return ["tasks","logs","stop","kill","open"].includes(verb??"")||(verb==="daemon"&&["stop","status"].includes(action??""));
}

/** 🔨️ Rebuilds and reinstalls through the package script with inherited output; Ctrl+C cancels the build. */
async function rebuildDashboard(reason:string,options:DashboardExecutionOptions):Promise<void> {
  console.log(`[dashboard] Rebuilding ${options.mode} because ${reason}`);
  await runOwnedCommand(process.execPath,[join(dashboardPackageRoot,"📜️script.ts"),"build"],dashboardPackageRoot,"dashboard:rebuild",compilerAdmissionBudgetV1(options.control),{env:scriptResourceProcessEnvironmentV1(options.control,scriptInvocationEnvironment(options.invocation,{...process.env,SEMIO_BUILD_MODE:options.mode,SEMIO_SCRIPT_POLICY:undefined})),signal:options.control.signal});
}

/** ▶️ Hands the terminal to the selected immutable binary under original cancellation and deadline. */
export async function launchDashboard(root:string,args:readonly string[],options:DashboardExecutionOptions):Promise<number> {
  const mode=readBuildMode(options.mode),control=options.control;compilerAdmissionBudgetV1(control);
  const reason=mayRunStale(args)&&dashboardInstalled(root,mode)?undefined:await staleDashboard(root,mode);compilerAdmissionBudgetV1(control);
  if(reason)await withRenewableIssuedResourceControlV1(control,progressDeadlinePolicyV1("native-child",0,process.env,{maximumMilliseconds:buildPreparationBudgetMs()}),0,preparation=>rebuildDashboard(reason,{...options,control:preparation}));
  const controller=new AbortController(),signal=AbortSignal.any([control.signal,controller.signal]),budget=compilerAdmissionBudgetV1(control),stopTimeout=startOwnedCommandTimeoutV1(budget,()=>controller.abort(Error("Original dashboard launch deadline exhausted")));
  try{return await withInstalledDashboardV1(root,options,executable=>new Promise<number>((accept,reject)=>{
    const child=spawn(executable,args,{cwd:root,env:scriptInvocationEnvironment(options.invocation,{...process.env,SEMIO_BUILD_MODE:mode}),signal,stdio:"inherit",windowsHide:true});
    child.once("error",reject);child.once("close",(code,closedSignal)=>accept(code??(closedSignal==="SIGINT"?130:1)));
  }));}finally{stopTimeout();}
}
