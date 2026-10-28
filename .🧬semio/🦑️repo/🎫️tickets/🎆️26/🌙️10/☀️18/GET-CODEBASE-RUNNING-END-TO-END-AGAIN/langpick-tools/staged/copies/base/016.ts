import {stat} from "node:fs/promises";
import {isAbsolute,join,resolve} from "node:path";
import {readRemoteCachePolicyV1,readRemoteCacheTokenV1,remoteCacheBaseUrlV1,remoteCacheEffectiveUploadV1,requireRemoteCacheShapeV1,resolveRemoteCacheContractV1,type RemoteCacheContractV1,type RemoteCacheRenderingV1} from "../🟦️.ts";
import type {RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";
import {collectRemoteCacheDirectoryV1,scanRemoteCacheDirectoryV1,type RemoteCacheCollectionReportV1} from "../🧹️collector/🟦️.ts";
import {RemoteCacheDirectoryBusyV1,startRemoteCacheServerV1,type RemoteCacheServerStatusV1} from "../🖥️server/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};

/** 🔌️ The build tool's side of `remote-cache status`: which HTTP protocol it speaks, which read-only route proves reachability and credentials, and how it renders the contract. */
export interface RemoteCacheToolV1{
 readonly tool:string;
 readonly protocol:string;
 readonly probePath:string;
 render(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>):RemoteCacheRenderingV1;
}

/** 🎛️ Carries the issued phase into the command: where the workspace is, the environment, cancellation, progress, output and the two build tools. */
export type RemoteCacheCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;progress:(stage:string)=>Promise<void>;write:(line:string)=>void;tools:()=>Promise<readonly RemoteCacheToolV1[]>}>;

/** 🏁️ How a command ended: the process exit code and an optional machine-readable document to print. */
export type RemoteCacheCommandResultV1=Readonly<{exitCode:number;result?:unknown}>;

/** 🗣️ The languages of all messages; there is no default among them. */
export type RemoteCacheLanguageV1="en"|"de";

type Operation="serve"|"status"|"gc";

const LANGUAGES=texts.languages as readonly RemoteCacheLanguageV1[],
 OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
  serve:{valued:["--port","--host","--directory","--max-bytes","--tls-certificate","--tls-key","--lang"],plain:["--read-only"]},
  status:{valued:["--directory","--lang"],plain:[]},
  gc:{valued:["--directory","--max-bytes","--lang"],plain:["--dry-run","--json"]}
 };

/** 💬️ Renders one catalogue message in one language. */
export function remoteCacheTextV1(language:RemoteCacheLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<RemoteCacheLanguageV1,string>>>>)[key];
 if(!message)throw Error(`Declared remote cache message required: ${key}`);
 return message[language].replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Remote cache message variable missing: ${name}`);return String(value);});
}

/** ☁️ Picks the output languages: the explicit request, else the one the environment names (`SEMIO_LANGUAGE`, then the locale), else all of them, because no language is the default. */
export function remoteCacheLanguagesV1(requested:string|undefined,environment:Readonly<Record<string,string|undefined>>):readonly RemoteCacheLanguageV1[]{
 if(requested!==undefined){
  if(!LANGUAGES.includes(requested as RemoteCacheLanguageV1))throw Error(both("error.language",{language:requested}));
  return [requested as RemoteCacheLanguageV1];
 }
 for(const value of[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const head=value?.slice(0,2).toLowerCase();if(head&&LANGUAGES.includes(head as RemoteCacheLanguageV1))return [head as RemoteCacheLanguageV1];}
 return LANGUAGES;
}

function both(key:string,variables:Readonly<Record<string,string|number>>={}):string{return LANGUAGES.map(language=>remoteCacheTextV1(language,key,variables)).join("\n");}

/** 🔠️ Spells a byte count in binary units, deterministically. */
export function formatRemoteCacheBytesV1(bytes:number):string{
 if(bytes<1024)return `${bytes} B`;
 const units=["KiB","MiB","GiB","TiB"];let value=bytes,index=-1;
 do{value/=1024;index++;}while(value>=1024&&index<units.length-1);
 return `${value.toFixed(2)} ${units[index]}`;
}

/** 📝️ The closed arguments of one `remote-cache` operation: valued options by name and the plain flags that were given. */
export type RemoteCacheArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>}>;

/** 🧭️ Reads `serve|status|gc` with its options (`--name value` or `--name=value`); every unknown, misplaced or repeated option is refused in both languages. */
export function parseRemoteCacheArgumentsV1(args:readonly string[]):RemoteCacheArgumentsV1{
 const operation=args[0];
 if(operation!=="serve"&&operation!=="status"&&operation!=="gc")throw Error(both("error.syntax"));
 const allowed=OPTIONS[operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>();
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  if(allowed.valued.includes(name)){
   const value=cut<0?args[++index]:argument.slice(cut+1);
   if(value===undefined||value===""||value.startsWith("--"))throw Error(both("error.value",{option:name}));
   values[name]=value;
  }else if(allowed.plain.includes(name)&&cut<0)flags.add(name);
  else throw Error(both("error.option",{operation,option:argument}));
  seen.add(name);
 }
 return Object.freeze({operation,values:Object.freeze(values),flags});
}

const whole=(name:string,value:string,minimum:number,maximum:number):number=>{
 const number=Number(value);
 if(!/^[0-9]+$/u.test(value)||!Number.isSafeInteger(number)||number<minimum||number>maximum)throw Error(both("error.number",{option:name,minimum,maximum}));
 return number;
};

const where=(root:string,path:string):string=>isAbsolute(path)?path:resolve(root,path);

function reporter(context:RemoteCacheCommandContextV1,languages:readonly RemoteCacheLanguageV1[]):(key:string,variables?:Readonly<Record<string,string|number>>)=>void{
 return (key,variables)=>{
  for(const language of languages){
   const resolved:Record<string,string|number>={};
   for(const [name,value] of Object.entries(variables??{}))resolved[name]=typeof value=="string"&&value.startsWith("@")?remoteCacheTextV1(language,value.slice(1)):value;
   context.write(remoteCacheTextV1(language,key,resolved));
  }
 };
}

/** 🚀️ Runs the repository's own cache server until the phase is cancelled; progress covers the store scan, listening and every eviction. */
export async function serveRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(["serve",...args]),flags=parsed.values,languages=remoteCacheLanguagesV1(flags["--lang"],context.environment),say=reporter(context,languages);
 const all=readRemoteCachePolicyV1(),policy=all.server,names=policy.environment,read=context.environment[policy.tokenEnvironment.read],write=context.environment[policy.tokenEnvironment.write];
 for(const token of[read,write])if(token!==undefined&&token!=="")requireRemoteCacheShapeV1("Token",token,"Closed remote cache server token required");
 const variable=(name:string):string|undefined=>{const value=context.environment[name];return value===undefined||value===""?undefined:value;};
 const tls=policy.tls as Readonly<{certificate:string;key:string}>|null,switched=variable(names.readOnly),readOnly=parsed.flags.has("--read-only")||(switched===undefined?policy.readOnly:switched==="true");
 if(switched!==undefined&&switched!=="true"&&switched!=="false")throw Error(both("error.option",{operation:"serve",option:`${names.readOnly}=${switched}`}));
 const certificate=flags["--tls-certificate"]??variable(names.tlsCertificate)??tls?.certificate,key=flags["--tls-key"]??variable(names.tlsKey)??tls?.key;
 if((certificate===undefined)!==(key===undefined))throw Error(both("error.tls"));
 const host=flags["--host"]??policy.host,tokens={read:read===""?undefined:read,write:write===""?undefined:write},loopback=["127.0.0.1","localhost","::1","[::1]"].includes(host);
 if(!loopback&&tokens.read===undefined&&tokens.write===undefined)throw Error(both("error.token"));
 const directory=where(context.root,flags["--directory"]??policy.directory),maximumBytes=flags["--max-bytes"]===undefined?policy.maximumBytes:whole("--max-bytes",flags["--max-bytes"],1048576,Number.MAX_SAFE_INTEGER);
 await context.progress("remote-cache:open");say("serve.open",{directory});
 const server=await startRemoteCacheServerV1({
  directory,host,
  port:flags["--port"]===undefined?policy.port:whole("--port",flags["--port"],0,65535),
  maximumBytes,
  maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(maximumBytes/2)),
  maximumAgeDays:policy.maximumAgeDays,
  defaultInstance:all.defaults.instance,
  tokens,readOnly,
  tls:certificate===undefined||key===undefined?undefined:{certificate:where(context.root,certificate),key:where(context.root,key)},
  validation:policy.actionResults as RemoteCacheValidationV1,
  maximumActionResultBytes:policy.maximumActionResultBytes,
  maintenanceSeconds:policy.maintenanceSeconds,
  signal:context.signal,
  onEvent:event=>{
   if(event.stage==="listening"){
    context.write(`[remote-cache] listening ${event.url}`);void context.progress("remote-cache:serving");
    say("serve.listening",{url:event.url,access:readOnly?"@serve.access.read-only":"@serve.access.read-write",transport:event.url.startsWith("https")?"@serve.transport.https":"@serve.transport.http",validation:"@serve.validation."+policy.actionResults});
    if(tokens.read===undefined&&tokens.write===undefined)say("serve.open-access");
    say("serve.client",{url:event.url});
   }else if(event.stage==="scan")void context.progress("remote-cache:scan");
   else if(event.stage==="ready"){void context.progress("remote-cache:ready");say("serve.ready",{entries:event.entries,bytes:formatRemoteCacheBytesV1(event.bytes)});}
   else if(event.stage==="evict")say("serve.evicted",{entries:event.entries,bytes:formatRemoteCacheBytesV1(event.bytes)});
   else if(event.stage==="stopped"){context.write("[remote-cache] stopped");say("serve.stopped");}
  }
 }).catch(error=>{throw error instanceof RemoteCacheDirectoryBusyV1?Error(both("error.busy",{directory:error.directory})):error;});
 await server.stopped;
 return {exitCode:0};
}

type Probe={protocol:RemoteCacheToolV1["protocol"];reachable:boolean;authorized:boolean;status:number|null;milliseconds:number};

async function probeRemoteCacheV1(contract:RemoteCacheContractV1,tool:RemoteCacheToolV1,token:string|undefined,signal:AbortSignal):Promise<Probe>{
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

async function askServerV1(contract:RemoteCacheContractV1,token:string|undefined,signal:AbortSignal):Promise<RemoteCacheServerStatusV1|null>{
 try{
  const response=await fetch(`${contract.endpoint}/status`,{signal:AbortSignal.any([signal,AbortSignal.timeout(contract.timeoutSeconds*1000)]),headers:token===undefined?{}:{authorization:`Bearer ${token}`}});
  if(response.status!==200){await response.body?.cancel();return null;}
  const document=await response.json();
  requireRemoteCacheShapeV1("ServerStatus",document,"Closed remote cache server status required");
  return document as RemoteCacheServerStatusV1;
 }catch(error){
  if(signal.aborted)throw error;
  return null;
 }
}

/** 📋️ Prints the resolved contract, how both build tools render it, whether the endpoint answers on both protocols, what the running server reports about itself and what the local store directory holds; secrets never appear, only whether the token variable is set. */
export async function remoteCacheStatusV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<unknown>{
 const parsed=parseRemoteCacheArgumentsV1(["status",...args]),all=readRemoteCachePolicyV1(),tools=await context.tools(),contract=resolveRemoteCacheContractV1(context.environment),directory=where(context.root,parsed.values["--directory"]??all.server.directory);
 await context.progress("remote-cache:scan");
 const root=join(directory,"v1"),present=await stat(root).then(info=>info.isDirectory(),()=>false);
 let store=null;
 if(present){
  const index=await scanRemoteCacheDirectoryV1(root,{signal:context.signal,onProgress:()=>{void context.progress("remote-cache:scan");}});
  let bytes=0;for(const row of index.values())bytes+=row.size;
  store={directory,entries:index.size,bytes,maximumBytes:all.server.maximumBytes};
 }
 if(contract===undefined)return {version:1,configured:false,contract:null,effectiveUpload:false,token:null,renderings:null,probes:null,server:null,store};
 await context.progress("remote-cache:probe");
 const token=readRemoteCacheTokenV1(contract,context.environment),probes:Probe[]=[],renderings:Record<string,RemoteCacheRenderingV1>={};
 for(const tool of tools){renderings[tool.tool]=tool.render(contract,context.environment);probes.push(await probeRemoteCacheV1(contract,tool,token,context.signal));}
 const status={version:1,configured:true,contract,effectiveUpload:remoteCacheEffectiveUploadV1(contract),token:{environment:contract.auth.tokenEnvironment,present:token!==undefined},renderings,probes,server:await askServerV1(contract,token,context.signal),store};
 requireRemoteCacheShapeV1("Status",status,"Closed remote cache status required");return status;
}

/** 🧹️ Collects the store directory like a running server would, from outside: removes unusable action results, entries idle for too long and the least recently used entries beyond the bound, under the collector lock; `--dry-run` only plans. Progress is published and cancelling stops at the next checkpoint. */
export async function collectRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(["gc",...args]),languages=remoteCacheLanguagesV1(parsed.values["--lang"],context.environment),say=reporter(context,languages),policy=readRemoteCachePolicyV1().server;
 const directory=where(context.root,parsed.values["--directory"]??policy.directory),maximumBytes=parsed.values["--max-bytes"]===undefined?policy.maximumBytes:whole("--max-bytes",parsed.values["--max-bytes"],1048576,Number.MAX_SAFE_INTEGER),dryRun=parsed.flags.has("--dry-run"),json=parsed.flags.has("--json");
 const talk=json?()=>{}:say;
 if(!await stat(join(directory,"v1")).then(info=>info.isDirectory(),()=>false)){
  talk("gc.absent",{directory});
  const absent={version:1,dryRun,directory,present:false};if(json)requireRemoteCacheShapeV1("GcResult",absent,"Closed remote cache gc result required");
  return {exitCode:0,...(json?{result:absent}:{})};
 }
 talk("gc.start",{directory,bytes:formatRemoteCacheBytesV1(maximumBytes),days:policy.maximumAgeDays,mode:dryRun?"@gc.mode.plan":"@gc.mode.apply"});
 let report;
 try{
  report=await collectRemoteCacheDirectoryV1({root:join(directory,"v1"),maximumBytes,maximumAgeMilliseconds:policy.maximumAgeDays*86400000,dryRun,verifyResults:policy.actionResults==="complete",signal:context.signal,onProgress:stage=>{void context.progress(`remote-cache:${stage}`);}});
 }catch(error){
  if(context.signal.aborted){talk("gc.cancelled");return {exitCode:130};}
  throw error;
 }
 if(report.locked){talk("gc.locked",{lock:report.lock});return {exitCode:1};}
 const value=report.value;
 talk(dryRun?"gc.summary.plan":"gc.summary.apply",{entries:value.removed.entries,total:value.scanned.entries,bytes:formatRemoteCacheBytesV1(value.removed.bytes),age:value.removed.age,budget:value.removed.budget,dangling:value.removed.dangling,remaining:value.remaining.entries,left:formatRemoteCacheBytesV1(value.remaining.bytes)});
 const document={directory,...value};if(json)requireRemoteCacheShapeV1("GcResult",document,"Closed remote cache gc result required");
 return {exitCode:0,...(json?{result:document}:{})};
}

/** 🧾️ Collects the workspace's store for `repo cache gc` and answers one language-neutral line for its report; the ledger delegates its remote cache area here, so both commands apply the same bounds. */
export async function collectRemoteCacheForLedgerV1(workspace:string,dryRun:boolean,signal:AbortSignal):Promise<string>{
 const policy=readRemoteCachePolicyV1().server,root=join(where(workspace,policy.directory),"v1");
 if(!await stat(root).then(info=>info.isDirectory(),()=>false))return "absent";
 const outcome=await collectRemoteCacheDirectoryV1({root,maximumBytes:policy.maximumBytes,maximumAgeMilliseconds:policy.maximumAgeDays*86400000,dryRun,verifyResults:policy.actionResults==="complete",signal});
 if(outcome.locked)return `busy: ${outcome.lock}`;
 const report:RemoteCacheCollectionReportV1=outcome.value;
 return `${dryRun?"would remove":"removed"} ${report.removed.entries} of ${report.scanned.entries} entries (${formatRemoteCacheBytesV1(report.removed.bytes)})`;
}

/** 🎬️ Routes `remote-cache serve|status|gc`; `args` are the words after `remote-cache`. */
export async function runRemoteCacheCommandV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(args);
 if(parsed.operation==="serve")return serveRemoteCacheV1(args.slice(1),context);
 if(parsed.operation==="gc")return collectRemoteCacheV1(args.slice(1),context);
 return {exitCode:0,result:await remoteCacheStatusV1(args.slice(1),context)};
}
