import {isAbsolute,resolve} from "node:path";
import {readRemoteCachePolicyV1,readRemoteCacheTokenV1,remoteCacheBaseUrlV1,remoteCacheEffectiveUploadV1,requireRemoteCacheShapeV1,resolveRemoteCacheContractV1,type RemoteCacheContractV1,type RemoteCacheRenderingV1} from "../🟦️.ts";
import {startRemoteCacheServerV1} from "../🖥️server/🟦️.ts";

/** 🔌️ The build tool's side of `cache remote status`: which HTTP protocol it speaks, which read-only route proves reachability and credentials, and how it renders the contract. */
export interface RemoteCacheToolV1{
 readonly tool:"nx"|"bazel";
 readonly protocol:"bazel-http"|"nx-self-hosted";
 readonly probePath:string;
 render(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>):RemoteCacheRenderingV1;
}

/** 🎛️ Carries the issued phase into the command: where the workspace is, the environment, cancellation, progress and output. */
export type RemoteCacheCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;progress:(stage:string)=>Promise<void>;write:(line:string)=>void;tool:()=>Promise<RemoteCacheToolV1>}>;

const VALUED=new Set(["--port","--host","--directory","--max-bytes"]);

/** 🧭️ Parses `--name value` and `--name=value` pairs of the allowed names and refuses everything else. */
export function parseRemoteCacheFlagsV1(args:readonly string[]):Readonly<Record<string,string>>{
 const values:Record<string,string>={};
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,separator=argument.indexOf("="),name=separator<0?argument:argument.slice(0,separator);
  if(!VALUED.has(name))throw Error(`Unknown cache serve option: ${argument}`);
  const value=separator<0?args[++index]:argument.slice(separator+1);
  if(value===undefined||value===""||value.startsWith("--"))throw Error(`Value required for ${name}`);
  values[name]=value;
 }
 return Object.freeze(values);
}

const whole=(name:string,value:string,minimum:number,maximum:number):number=>{
 const number=Number(value);
 if(!/^[0-9]+$/u.test(value)||!Number.isSafeInteger(number)||number<minimum||number>maximum)throw Error(`${name} must be a whole number from ${minimum} to ${maximum}`);
 return number;
};

/** 🚀️ Runs the repository's own cache server until the phase is cancelled; progress covers the store scan, listening and every eviction. */
export async function serveRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<void>{
 const policy=readRemoteCachePolicyV1().server,flags=parseRemoteCacheFlagsV1(args),read=context.environment[policy.tokenEnvironment.read],write=context.environment[policy.tokenEnvironment.write];
 for(const token of [read,write])if(token!==undefined&&token!=="")requireRemoteCacheShapeV1("Token",token,"Closed remote cache server token required");
 const directory=flags["--directory"]??policy.directory,instance=readRemoteCachePolicyV1().defaults.instance,maximumBytes=flags["--max-bytes"]===undefined?policy.maximumBytes:whole("--max-bytes",flags["--max-bytes"],1048576,Number.MAX_SAFE_INTEGER);
 await context.progress("remote-cache:open");
 const server=await startRemoteCacheServerV1({
  directory:isAbsolute(directory)?directory:resolve(context.root,directory),
  host:flags["--host"]??policy.host,
  port:flags["--port"]===undefined?policy.port:whole("--port",flags["--port"],0,65535),
  maximumBytes,
  maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(maximumBytes/2)),
  maximumAgeDays:policy.maximumAgeDays,
  defaultInstance:instance,
  tokens:{read:read===""?undefined:read,write:write===""?undefined:write},
  signal:context.signal,
  onEvent:event=>{
   if(event.stage==="listening"){context.write(`[cache] listening ${event.url}`);void context.progress("remote-cache:serving");}
   else if(event.stage==="scan"||event.stage==="ready")void context.progress("remote-cache:"+event.stage);
   else if(event.stage==="evict")context.write(`[cache] evicted ${event.entries} entries, ${event.bytes} bytes`);
   else if(event.stage==="stopped")context.write("[cache] stopped");
  }
 });
 await server.stopped;
}

/** 🔬️ Asks the endpoint one read-only question on the route the selected tool uses; it stores nothing and ends at the contract's timeout or when cancelled. */
async function probeRemoteCacheV1(contract:RemoteCacheContractV1,tool:RemoteCacheToolV1,token:string|undefined,signal:AbortSignal):Promise<{protocol:RemoteCacheToolV1["protocol"];reachable:boolean;authorized:boolean;status:number|null;milliseconds:number}>{
 const started=performance.now(),elapsed=()=>Math.round(performance.now()-started),combined=AbortSignal.any([signal,AbortSignal.timeout(contract.timeoutSeconds*1000)]);
 try{
  const response=await fetch(remoteCacheBaseUrlV1(contract)+tool.probePath,{method:"GET",signal:combined,headers:token===undefined?{}:{authorization:`Bearer ${token}`}});
  await response.body?.cancel();
  return {protocol:tool.protocol,reachable:true,authorized:response.status===200||response.status===404,status:response.status,milliseconds:elapsed()};
 }catch(error){
  if(signal.aborted)throw error;
  return {protocol:tool.protocol,reachable:false,authorized:false,status:null,milliseconds:elapsed()};
 }
}

/** 📋️ Prints the resolved contract, how the selected tool renders it and whether the endpoint answers; secrets never appear, only whether the token variable is set. */
export async function remoteCacheStatusV1(context:RemoteCacheCommandContextV1):Promise<unknown>{
 const tool=await context.tool(),contract=resolveRemoteCacheContractV1(context.environment);
 if(contract===undefined)return {version:1,tool:tool.tool,configured:false,contract:null,effectiveUpload:false,token:null,rendering:null,probe:null};
 await context.progress("remote-cache:probe");
 const token=readRemoteCacheTokenV1(contract,context.environment),status={version:1,tool:tool.tool,configured:true,contract,effectiveUpload:remoteCacheEffectiveUploadV1(contract),token:{environment:contract.auth.tokenEnvironment,present:token!==undefined},rendering:tool.render(contract,context.environment),probe:await probeRemoteCacheV1(contract,tool,token,context.signal)};
 requireRemoteCacheShapeV1("Status",status,"Closed remote cache status required");return status;
}

/** 🎬️ Routes `cache serve` and `cache remote status`; `args` are the words after `cache`. */
export async function runRemoteCacheCommandV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<unknown>{
 if(args[0]==="serve"){await serveRemoteCacheV1(args.slice(1),context);return undefined;}
 if(args[0]==="remote"&&args[1]==="status"&&args.length===2)return remoteCacheStatusV1(context);
 throw Error("Expected repo cache serve [--port <n>] [--host <address>] [--directory <path>] [--max-bytes <n>] or repo cache remote status");
}
