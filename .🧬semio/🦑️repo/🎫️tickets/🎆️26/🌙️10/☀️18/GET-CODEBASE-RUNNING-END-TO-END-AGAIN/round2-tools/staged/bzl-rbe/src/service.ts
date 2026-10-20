import {spawn} from "node:child_process";
import {mkdir,readFile,readdir,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🐳️remote-execution-service.json" with {type:"json"};
import descriptor from "./🔣️.json" with {type:"json"};

/** 🐳️ The authored local remote execution service: images pinned by digest, loopback ports, worker platforms. */
export type RemoteExecutionServiceV1=Readonly<typeof descriptor>;
/** 🖥️ The execution platform id the workers serve. */
export type RemoteExecutionServicePlatformV1=(typeof descriptor.platforms)[number];
/** 🧾️ Everything the container runtime and the clients need for one chosen platform and port pair. */
export type RemoteExecutionServiceRenderingV1=Readonly<{platform:RemoteExecutionServicePlatformV1;compose:Readonly<{file:string;project:string;environment:Readonly<Record<string,string>>}>;client:Readonly<Record<"SEMIO_REMOTE_EXECUTION_ENDPOINT"|"SEMIO_REMOTE_EXECUTION_PLATFORM"|"SEMIO_REMOTE_EXECUTION_INSTANCE",string>>}>;
/** 🔎️ Whether the container runtime answers, with its version or the reason it does not. */
export type RemoteExecutionRuntimeProbeV1={available:true;version:string}|{available:false;reason:string};
/** 🧩️ One compose invocation: the rendered project directory, its file, the project name and the variables the file reads. */
export type RemoteExecutionComposeV1=Readonly<{directory:string;file:string;project:string;environment:Readonly<Record<string,string>>}>;
/** 🔌️ The container runtime behind the service; docker compose is the one implementation, a test double is the other. */
export interface RemoteExecutionRuntimeV1{
 probe(signal:AbortSignal):Promise<RemoteExecutionRuntimeProbeV1>;
 compose(plan:RemoteExecutionComposeV1,args:readonly string[],options:Readonly<{signal?:AbortSignal;onLine?:(line:string)=>void}>):Promise<Readonly<{code:number;output:string}>>;
}
/** 🎛️ Carries the issued phase into the command: workspace, environment, host, cancellation, progress, output and the swappable runtime and fetch. */
export type RemoteExecutionCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;host:Readonly<{platform:string;arch:string}>;signal:AbortSignal;progress:(stage:string)=>Promise<void>;write:(line:string)=>void;runtime?:RemoteExecutionRuntimeV1;fetch?:typeof fetch}>;

const STACK=join(dirname(fileURLToPath(import.meta.url)),"🐳️stack"),RUNTIME_DIRECTORY=".cache/remote-execution",VALUED=new Set(["--platform","--port","--diagnostics-port"]);

/** 🛡️ Validates one value against a named definition of the service schema and names the first problem. */
export function requireRemoteExecutionServiceShapeV1(definition:keyof typeof schema.definitions,value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🏛️ Reads the authored service and refuses any drift from its schema. */
export function readRemoteExecutionServiceV1(value:unknown=descriptor):RemoteExecutionServiceV1{
 requireRemoteExecutionServiceShapeV1("Service",value,"Closed remote execution service required");return value as RemoteExecutionServiceV1;
}

/** 🧭️ Parses `--name value` and `--name=value` pairs of the allowed names and refuses everything else. */
export function parseRemoteExecutionFlagsV1(args:readonly string[]):Readonly<Record<string,string>>{
 const values:Record<string,string>={};
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,separator=argument.indexOf("="),name=separator<0?argument:argument.slice(0,separator);
  if(!VALUED.has(name))throw Error(`Unknown remote-execution option: ${argument}`);
  const value=separator<0?args[++index]:argument.slice(separator+1);
  if(value===undefined||value===""||value.startsWith("--"))throw Error(`Value required for ${name}`);
  values[name]=value;
 }
 return Object.freeze(values);
}

const port=(name:string,value:string):number=>{
 const number=Number(value);
 if(!/^[0-9]{4,5}$/u.test(value)||number<1024||number>65535)throw Error(`${name} must be a port from 1024 to 65535`);
 return number;
};

/** 🖥️ Chooses the worker platform: the requested id when the service offers it, else the one matching this machine's CPU, because the workers run natively next to the container runtime. */
export function remoteExecutionServicePlatformV1(service:RemoteExecutionServiceV1,host:Readonly<{arch:string}>,requested?:string):RemoteExecutionServicePlatformV1{
 const id=requested??(host.arch==="arm64"?"linux-arm64":"linux-x64");
 if(!(service.platforms as readonly string[]).includes(id))throw Error(`Unsupported worker platform: ${id}`);
 return id as RemoteExecutionServicePlatformV1;
}

/** 🐳️ Names the container platform of a worker platform id. */
export function remoteExecutionServiceDockerPlatformV1(platform:RemoteExecutionServicePlatformV1):string{
 const[os,arch]=platform.split("-");return `${os}/${arch==="x64"?"amd64":arch}`;
}

/** 🧮️ Renders the service for a platform and ports into the variables of the compose file and the contract variables a client sets. */
export function renderRemoteExecutionServiceV1(service:RemoteExecutionServiceV1,options:Readonly<{platform:RemoteExecutionServicePlatformV1;port?:number;diagnosticsPort?:number}>):RemoteExecutionServiceRenderingV1{
 const names=service.environment,endpointPort=options.port??service.port,diagnosticsPort=options.diagnosticsPort??service.diagnostics.port;
 const rendering:RemoteExecutionServiceRenderingV1={platform:options.platform,compose:{file:service.compose,project:service.project,environment:{[names.platform]:options.platform,[names.dockerPlatform]:remoteExecutionServiceDockerPlatformV1(options.platform),[names.port]:String(endpointPort),[names.diagnosticsPort]:String(diagnosticsPort)}},client:{SEMIO_REMOTE_EXECUTION_ENDPOINT:`grpc://${service.host}:${endpointPort}`,SEMIO_REMOTE_EXECUTION_PLATFORM:options.platform,SEMIO_REMOTE_EXECUTION_INSTANCE:service.instance}};
 requireRemoteExecutionServiceShapeV1("Rendering",rendering,"Closed remote execution rendering required");return rendering;
}

/** 🔗️ Names the loopback URL of the diagnostics route that answers once the frontend serves. */
export function remoteExecutionServiceHealthUrlV1(service:RemoteExecutionServiceV1,diagnosticsPort:number=service.diagnostics.port):string{
 return `http://${service.host}:${diagnosticsPort}${service.diagnostics.path}`;
}

/** 📦️ Lists the files of the packaged stack with their relative POSIX paths. */
async function stackFilesV1(directory:string,prefix=""):Promise<string[]>{
 const rows:string[]=[];
 for(const entry of (await readdir(directory,{withFileTypes:true})).sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0)){
  if(entry.isDirectory())rows.push(...await stackFilesV1(join(directory,entry.name),`${prefix}${entry.name}/`));
  else if(entry.isFile())rows.push(`${prefix}${entry.name}`);
 }
 return rows;
}

/** 📤️ Copies the packaged stack to an ASCII-only runtime directory below the workspace, because container bind mounts must not depend on the emoji folder names of the repository; returns the compose invocation. */
export async function writeRemoteExecutionStackV1(root:string,service:RemoteExecutionServiceV1,rendering:RemoteExecutionServiceRenderingV1,source:string=STACK):Promise<RemoteExecutionComposeV1>{
 const directory=join(root,RUNTIME_DIRECTORY);
 await rm(directory,{recursive:true,force:true});
 for(const relative of await stackFilesV1(source)){
  const target=join(directory,relative);await mkdir(dirname(target),{recursive:true});await writeFile(target,await readFile(join(source,relative)));
 }
 return Object.freeze({directory,file:join(directory,service.compose),project:rendering.compose.project,environment:rendering.compose.environment});
}

const lines=(text:string):string[]=>text.split(/\r?\n/u).filter(line=>line!=="");

/** 🐳️ The docker implementation of the runtime: `docker version` proves the daemon, `docker compose` runs the project. */
export function dockerRemoteExecutionRuntimeV1(environment:Readonly<Record<string,string|undefined>>=process.env):RemoteExecutionRuntimeV1{
 const run=(args:readonly string[],extra:Readonly<Record<string,string>>,options:Readonly<{signal?:AbortSignal;onLine?:(line:string)=>void;directory?:string}>):Promise<Readonly<{code:number;output:string}>>=>new Promise((resolve,reject)=>{
  const child=spawn("docker",[...args],{cwd:options.directory,env:{...environment,...extra} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"],windowsHide:true});
  let output="",pending="";
  const take=(chunk:Buffer)=>{const text=chunk.toString("utf8");output+=text;pending+=text;const parts=pending.split(/\r?\n/u);pending=parts.pop()!;for(const line of parts)if(line!=="")options.onLine?.(line);};
  const stop=()=>{child.kill("SIGTERM");};
  if(options.signal?.aborted)stop();else options.signal?.addEventListener("abort",stop,{once:true});
  child.stdout.on("data",take);child.stderr.on("data",take);
  child.once("error",error=>{options.signal?.removeEventListener("abort",stop);reject(error);});
  child.once("close",code=>{options.signal?.removeEventListener("abort",stop);if(pending!=="")options.onLine?.(pending);resolve({code:code??1,output});});
 });
 return Object.freeze<RemoteExecutionRuntimeV1>({
  async probe(signal){
   try{const result=await run(["version","--format","{{.Server.Version}}"],{},{signal});
    return result.code===0?{available:true,version:lines(result.output)[0]??""}:{available:false,reason:lines(result.output).at(-1)??"docker did not answer"};
   }catch(error){return{available:false,reason:(error as Error).message};}
  },
  compose:(plan,args,options)=>run(["compose","--project-directory",plan.directory,"-f",plan.file,"-p",plan.project,...args],plan.environment,{...options,directory:plan.directory})
 });
}

const probeEndpoint=async(url:string,signal:AbortSignal,fetcher:typeof fetch):Promise<boolean>=>{
 try{return (await fetcher(url,{signal:AbortSignal.any([signal,AbortSignal.timeout(2000)])})).ok;}catch{return false;}
};

const sleep=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise(resolve=>{
 const timer=setTimeout(resolve,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});
});

/** 🧭️ Resolves the effective flags of every subcommand into a rendering. */
function resolveV1(args:readonly string[],context:RemoteExecutionCommandContextV1){
 const service=readRemoteExecutionServiceV1(),flags=parseRemoteExecutionFlagsV1(args),platform=remoteExecutionServicePlatformV1(service,context.host,flags["--platform"]);
 const rendering=renderRemoteExecutionServiceV1(service,{platform,...(flags["--port"]===undefined?{}:{port:port("--port",flags["--port"])}),...(flags["--diagnostics-port"]===undefined?{}:{diagnosticsPort:port("--diagnostics-port",flags["--diagnostics-port"])})});
 const diagnosticsPort=Number(rendering.compose.environment[service.environment.diagnosticsPort]);
 return{service,rendering,health:remoteExecutionServiceHealthUrlV1(service,diagnosticsPort)};
}

/** 🚀️ Runs the local cluster in the foreground until the phase is cancelled: preflight the runtime, render the stack, `compose up`, wait for the frontend, print the client variables, and on cancellation remove containers and volumes. */
export async function serveRemoteExecutionServiceV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<void>{
 const{service,rendering,health}=resolveV1(args,context),runtime=context.runtime??dockerRemoteExecutionRuntimeV1(context.environment),fetcher=context.fetch??fetch;
 await context.progress("remote-execution:preflight");
 const probe=await runtime.probe(context.signal);
 if(!probe.available)throw Error(`Container runtime unavailable: ${probe.reason}. Start Docker (Docker Desktop or the docker service) and retry.`);
 await context.progress("remote-execution:render");
 const plan=await writeRemoteExecutionStackV1(context.root,service,rendering);
 await context.progress("remote-execution:up");
 const controller=new AbortController(),stopUp=()=>controller.abort();
 context.signal.addEventListener("abort",stopUp,{once:true});
 const running=runtime.compose(plan,["up","--remove-orphans"],{signal:controller.signal,onLine:line=>context.write(`[rbe] ${line}`)});
 let exited:Readonly<{code:number;output:string}>|undefined;void running.then(result=>{exited=result;});
 const deadline=Date.now()+service.readyTimeoutSeconds*1000;let ready=false;
 while(!context.signal.aborted&&exited===undefined&&Date.now()<deadline&&!ready){
  ready=await probeEndpoint(health,controller.signal,fetcher);if(!ready)await sleep(1000,controller.signal);
 }
 if(ready){
  context.write(`[rbe] listening ${rendering.client.SEMIO_REMOTE_EXECUTION_ENDPOINT} (${rendering.platform}, instance ${service.instance})`);
  for(const[name,value]of Object.entries(rendering.client))context.write(`${name}=${value}`);
  await context.progress("remote-execution:serving");
  await running;
 }
 context.signal.removeEventListener("abort",stopUp);controller.abort();await running.catch(()=>undefined);
 await context.progress("remote-execution:down");
 const down=await runtime.compose(plan,["down","--volumes","--remove-orphans"],{onLine:line=>context.write(`[rbe] ${line}`)});
 context.write("[rbe] stopped");
 if(!ready&&!context.signal.aborted)throw Error(exited===undefined?`Remote execution service not ready within ${service.readyTimeoutSeconds} seconds`:`Remote execution service exited with ${exited.code} before it was ready`);
 if(down.code!==0)throw Error(`Removing the remote execution service failed with ${down.code}`);
}

/** 🧾️ Reads the containers of the project out of `compose ps --format json`, which prints one object per line or one array depending on the compose version. */
export function parseRemoteExecutionContainersV1(output:string):{service:string;state:string;exitCode?:number}[]{
 const rows:unknown[]=[];
 for(const line of lines(output)){
  try{const value=JSON.parse(line);if(Array.isArray(value))rows.push(...value);else rows.push(value);}catch{}
 }
 return rows.flatMap(row=>{
  const value=row as{Service?:unknown;State?:unknown;ExitCode?:unknown};
  return typeof value.Service==="string"&&typeof value.State==="string"?[{service:value.Service,state:value.State,...(typeof value.ExitCode==="number"?{exitCode:value.ExitCode}:{})}]:[];
 }).sort((left,right)=>left.service<right.service?-1:left.service>right.service?1:0);
}

/** 🩺️ Reports the runtime, the containers of the project and whether the frontend answers; nothing is started or changed. */
export async function statusRemoteExecutionServiceV1(args:readonly string[],context:RemoteExecutionCommandContextV1){
 const{service,rendering,health}=resolveV1(args,context),runtime=context.runtime??dockerRemoteExecutionRuntimeV1(context.environment),fetcher=context.fetch??fetch;
 await context.progress("remote-execution:status");
 const probe=await runtime.probe(context.signal);
 const listed=async()=>{try{return parseRemoteExecutionContainersV1((await runtime.compose({directory:join(context.root,RUNTIME_DIRECTORY),file:join(context.root,RUNTIME_DIRECTORY,service.compose),project:service.project,environment:rendering.compose.environment},["ps","--all","--format","json"],{signal:context.signal})).output);}catch{return [];}};
 const containers=probe.available?await listed():[];
 const status={version:1 as const,service:service.name,runtime:probe,containers,endpoint:{url:rendering.client.SEMIO_REMOTE_EXECUTION_ENDPOINT,answers:await probeEndpoint(health,context.signal,fetcher)},client:{...rendering.client}};
 requireRemoteExecutionServiceShapeV1("Status",status,"Closed remote execution status required");return status;
}

/** 📜️ Prints the client variables for the chosen platform and port, one `NAME=value` per line. */
export function envRemoteExecutionServiceV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Readonly<Record<string,string>>{
 return resolveV1(args,context).rendering.client;
}

/** 🎛️ Dispatches `remote-execution serve|status|env`; `serve` returns nothing, the others return the document the command prints. */
export async function runRemoteExecutionCommandV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<object|undefined>{
 const[command,...rest]=args;
 if(command==="serve"){await serveRemoteExecutionServiceV1(rest,context);return undefined;}
 if(command==="status")return statusRemoteExecutionServiceV1(rest,context);
 if(command==="env"){for(const[name,value]of Object.entries(envRemoteExecutionServiceV1(rest,context)))context.write(`${name}=${value}`);return undefined;}
 throw Error(`Unknown remote-execution command: ${command??""}. Use serve, status or env.`);
}
