import {renameSync} from "node:fs";
import protocolSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import {filesystemRaceOfErrorV1,filesystemRaceTableV1} from "./🌊️races/🟦️.ts";

const protocol=Object.freeze(JSON.parse(JSON.stringify(protocolSchema.definitions.Protocol.const)) as ChurnProtocolV1);
const sleeper=new Int32Array(new SharedArrayBuffer(4));

/** 🧬️ The declared kind of a concurrent edit, closed by the schema. */
export type ConcurrentEditReasonV1="bytes"|"identity"|"vanished"|"appeared"|"roster"|"length"|"ancestry"|"output"|"descriptor"|"generation"|"input"|"busy";
/** 🪜️ The layer a retry belongs to: `read` re-reads one file or directory, `snapshot` re-runs a batch of reads, `build` re-runs a whole compiler invocation. */
export type ChurnProfileNameV1="read"|"snapshot"|"build";
/** 📜️ One layer of the schema: its rank, its attempt bound and the pause before each further attempt. */
export type ChurnProfileV1=Readonly<{rank:number;maximumAttempts:number;backoffMs:readonly number[]}>;
/** 📖️ The closed protocol exactly as the schema declares it. */
export type ChurnProtocolV1=Readonly<{version:1;staleImplementationCode:string;staleImplementationCodes:readonly string[];concurrentEditCode:string;primaryFirstAggregates:readonly string[];languages:readonly string[];settleMilliseconds:number;profiles:Readonly<Record<ChurnProfileNameV1,ChurnProfileV1>>;reasons:Readonly<Record<string,Readonly<Record<ConcurrentEditReasonV1,string>>>>;texts:Readonly<Record<string,Readonly<Record<"retry"|"exhausted",string>>>>}>;
/** 📣️ One announced retry: the layer, the attempt about to start, what changed and how long the pause is. */
export type ChurnRetryEventV1=Readonly<{profile:ChurnProfileNameV1;attempt:number;maximum:number;path:string;reason:ConcurrentEditReasonV1;delayMs:number;text:string}>;
/** 🎛️ Cancellation, progress and clock authority of one retrying step; `retryable` lets a caller refuse a retry of a failure that came out of work it ran inside (a tracked unit that failed closed its owners, so running again can only fail again). */
export type ChurnOptionsV1=Readonly<{signal?:AbortSignal;retryable?:(error:unknown)=>boolean;environment?:Readonly<Record<string,string|undefined>>;onRetry?:(event:ChurnRetryEventV1)=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;sleepSync?:(milliseconds:number)=>void}>;

/** 📕️ The closed churn protocol: layers, attempt bounds, pauses and EN/DE texts. */
export function churnProtocolV1():ChurnProtocolV1{return protocol;}

/** 🌐️ The languages a message is rendered in: the chosen one (`SEMIO_LANGUAGE`, `LC_ALL`, `LANG`), every declared language when none is chosen. */
export function churnLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env):readonly string[]{
 const pick=(value:string|undefined):string|undefined=>{const code=String(value??"").toLowerCase().split(/[-_.:@]/u)[0]!;return protocol.languages.includes(code)?code:undefined;};
 const chosen=pick(environment.SEMIO_LANGUAGE)??pick(environment.LC_ALL)??pick(environment.LANG);
 return chosen?[chosen]:protocol.languages;
}

/** 💬️ Renders one declared message in one language; every placeholder must be given. */
export function churnTextV1(language:string,key:"retry"|"exhausted",values:Readonly<Record<string,string|number>>):string{
 const table=protocol.texts[language];if(!table||typeof table[key]!=="string")throw Error("Declared churn text required: "+language+"."+key);
 return table[key].replace(/\{([a-z]+)\}/gu,(_,name:string)=>{if(!Object.hasOwn(values,name))throw Error("Churn text placeholder required: "+name);return String(values[name]);});
}

const reasonOf=(language:string,reason:ConcurrentEditReasonV1):string=>protocol.reasons[language]![reason];
/** 📄️ The declared wording of what a concurrent edit changed, in one language. */
export function churnReasonTextV1(language:string,reason:ConcurrentEditReasonV1):string{const table=protocol.reasons[language];if(!table||typeof table[reason]!=="string")throw Error("Declared churn reason required: "+language+"."+reason);return table[reason];}
const lines=(environment:Readonly<Record<string,string|undefined>>,key:"retry"|"exhausted",values:(language:string)=>Readonly<Record<string,string|number>>):string=>churnLanguagesV1(environment).map(language=>churnTextV1(language,key,values(language))).join("\n");

/** 🧨️ A concurrent edit another process made to something this step reads. Raised by a read that sees the world move under it; `rank` is the widest layer that already exhausted its attempts on it (0 while raw), so a wider layer may retry it and a narrower one never multiplies attempts. */
export class ConcurrentEditErrorV1 extends Error{
 readonly code:string;readonly reason:ConcurrentEditReasonV1;readonly path:string;readonly rank:number;readonly attempts:number;
 constructor(reason:ConcurrentEditReasonV1,path:string,detail?:string,rank:number=0,exhausted?:Readonly<{attempts:number;message:string;cause:unknown}>){
  super(exhausted?.message??(detail??reasonOf("en",reason))+": "+path,exhausted?{cause:exhausted.cause}:undefined);
  this.name="ConcurrentEditErrorV1";this.code=protocol.concurrentEditCode;this.reason=reason;this.path=path;this.rank=rank;this.attempts=exhausted?.attempts??0;
 }
}

/** 🚩️ Builds the error a read throws when it sees a concurrent edit; `detail` keeps the site's own wording as the message head. `from` names the narrowest layer whose retry can cure it: `read` (default) when reading again fixes it, `snapshot` or `build` when the read contradicts an earlier read of the same snapshot, so only a new snapshot helps. */
export function concurrentEditV1(reason:ConcurrentEditReasonV1,path:string,detail?:string,from:ChurnProfileNameV1="read"):ConcurrentEditErrorV1{return new ConcurrentEditErrorV1(reason,path,detail,protocol.profiles[from].rank-1);}

/** 🧾️ What a concurrent edit changed: the kind, the path and the widest layer that already gave up on it. */
export type ConcurrentEditFactsV1=Readonly<{reason:ConcurrentEditReasonV1;path:string;rank:number}>;
const maximumCauseDepthV1=4;
type EditFactsV1=ConcurrentEditFactsV1&Readonly<{holder?:object}>;
const racesV1=new WeakMap<object,ConcurrentEditFactsV1|null>(),exhaustedRanksV1=new WeakMap<object,number>();
/** 🌊️ The one place a raw filesystem error becomes a concurrent edit: decided once per error object (the first sight is the verdict, so every layer reads the same answer), `transient` as a raw edit any layer may retry, `persistent` as an edit the `read` layer cannot cure (rank of the read layer), and remembered with the widest layer that gave up on it, so the error itself, with its `code`, travels on unchanged. */
function raceOf(error:object):EditFactsV1|undefined{
 let decided=racesV1.get(error);
 if(decided===undefined){
  let decision:ReturnType<typeof filesystemRaceOfErrorV1>|undefined;try{decision=filesystemRaceOfErrorV1(error);}catch{}
  decided=decision===undefined||decision.disposition==="none"||decision.reason===undefined?null:{reason:decision.reason,path:decision.path,rank:decision.disposition==="persistent"?protocol.profiles.read.rank:0};
  racesV1.set(error,decided);
 }
 return decided?{...decided,rank:Math.max(decided.rank,exhaustedRanksV1.get(error)??0),holder:error}:undefined;
}
function editOf(error:unknown,depth:number=0):EditFactsV1|undefined{
 if(error instanceof ConcurrentEditErrorV1)return{reason:error.reason,path:error.path,rank:error.rank};
 if(error instanceof AggregateError&&protocol.primaryFirstAggregates.includes(error.message))return editOf(error.errors[0],depth);
 if(error instanceof AggregateError){let first:EditFactsV1|undefined;for(const member of error.errors){const edit=editOf(member,depth);if(!edit)return;first??=edit;if(edit.rank>first.rank)first=edit;}return first;}
 if(error&&typeof error==="object"){
  const row=error as {name?:unknown;code?:unknown;path?:unknown;reason?:unknown;rank?:unknown;cause?:unknown};
  if(row.name==="FileObservationErrorV1"&&row.code==="changed"&&typeof row.path==="string")return{reason:"descriptor",path:row.path,rank:0};
  if(row.code===protocol.concurrentEditCode&&typeof row.path==="string"&&typeof row.reason==="string"&&Object.hasOwn(protocol.reasons.en!,row.reason))return{reason:row.reason as ConcurrentEditReasonV1,path:row.path,rank:typeof row.rank==="number"?row.rank:0};
  if(typeof row.code==="string"&&typeof row.path==="string"){const race=raceOf(error);if(race)return race;}
  if(row.cause!==undefined&&depth<maximumCauseDepthV1)return editOf(row.cause,depth+1);
 }
 return;
}
/** ✏️ Raises a concurrent edit from any language: the plain error a reader without access to this module throws carries `code`, `reason`, `path` and the `rank` of the narrowest layer whose retry cures it (`profiles[from].rank - 1`), all declared by the schema. */
export function concurrentEditWireV1(reason:ConcurrentEditReasonV1,path:string,message:string,from:ChurnProfileNameV1="snapshot"):Error{return Object.assign(new Error(message),{code:protocol.concurrentEditCode,reason,path,rank:protocol.profiles[from].rank-1});}

/** 🔎️ The concurrent edit behind `error` when a layer of `rank` may retry it: raw errors and errors exhausted only by narrower layers; `undefined` for anything else. */
export function concurrentEditOfV1(error:unknown,rank:number=Number.POSITIVE_INFINITY):ConcurrentEditFactsV1|undefined{const edit=editOf(error);return edit&&edit.rank<rank?{reason:edit.reason,path:edit.path,rank:edit.rank}:undefined;}
/** ❓️ Whether `error` is a concurrent edit at all, whatever layer exhausted it. */
export function isConcurrentEditV1(error:unknown):boolean{return editOf(error)!==undefined;}

const defaultSleep=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((accept,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);accept();},milliseconds),abort=():void=>{clearTimeout(timer);reject(signal!.reason);};
 signal?.addEventListener("abort",abort,{once:true});
});
const defaultSleepSync=(milliseconds:number):void=>{if(milliseconds>0)Atomics.wait(sleeper,0,0,milliseconds);};
const defaultReport=(event:ChurnRetryEventV1):void=>{try{process.stderr.write(event.text+"\n");}catch{}};

function plan(profileName:ChurnProfileNameV1,error:unknown,attempt:number,options:ChurnOptionsV1):Readonly<{retry:ChurnRetryEventV1}>|Readonly<{fail:Error}>|undefined{
 const profile=protocol.profiles[profileName],edit=editOf(error);if(!edit||edit.rank>=profile.rank||options.retryable?.(error)===false)return;
 const environment=options.environment??process.env;
 if(attempt>=profile.maximumAttempts){
  if(edit.holder){exhaustedRanksV1.set(edit.holder,profile.rank);return{fail:error as Error};}
  const message=lines(environment,"exhausted",language=>({path:edit.path,maximum:profile.maximumAttempts,reason:reasonOf(language,edit.reason)}));
  return{fail:new ConcurrentEditErrorV1(edit.reason,edit.path,undefined,profile.rank,{attempts:attempt,message,cause:error})};
 }
 const delayMs=profile.backoffMs[attempt-1]??0;
 const text=lines(environment,"retry",language=>({path:edit.path,reason:reasonOf(language,edit.reason),attempt:attempt+1,maximum:profile.maximumAttempts,delay:delayMs}));
 return{retry:Object.freeze({profile:profileName,attempt:attempt+1,maximum:profile.maximumAttempts,path:edit.path,reason:edit.reason,delayMs,text})};
}

/** 🔁️ Runs `body` as one snapshot of its layer: on a concurrent edit the layer pauses as the schema declares, announces the retry in the declared languages and runs `body` again with the next attempt number, until the layer's attempts are used up and the edit fails the step naming its path. Cancellation ends the wait at once. */
export async function retryOnConcurrentEditV1<T>(profile:ChurnProfileNameV1,body:(attempt:number)=>Promise<T>,options:ChurnOptionsV1={}):Promise<T>{
 for(let attempt=1;;attempt++){
  options.signal?.throwIfAborted();
  try{return await body(attempt);}
  catch(error){
   const next=plan(profile,error,attempt,options);if(!next)throw error;
   if("fail"in next)throw next.fail;
   (options.onRetry??defaultReport)(next.retry);
   await(options.sleep??defaultSleep)(next.retry.delayMs,options.signal);
  }
 }
}

/** 🔂️ The synchronous layer of {@link retryOnConcurrentEditV1} for readers that cannot suspend (it pauses the thread). */
export function retryOnConcurrentEditSyncV1<T>(profile:ChurnProfileNameV1,body:(attempt:number)=>T,options:ChurnOptionsV1={}):T{
 for(let attempt=1;;attempt++){
  options.signal?.throwIfAborted();
  try{return body(attempt);}
  catch(error){
   const next=plan(profile,error,attempt,options);if(!next)throw error;
   if("fail"in next)throw next.fail;
   (options.onRetry??defaultReport)(next.retry);
   (options.sleepSync??defaultSleepSync)(next.retry.delayMs);
  }
 }
}

/** 🧱️ The failure of a process whose own implementation changed on disk after it loaded it (a peer edited a reader it runs): the outermost command starts again with the current implementation, bounded by the declared format stamp protocol, instead of failing. The code is the one the stamp table recognises.
 * @see ../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs */
export function staleImplementationV1(message:string):Error{return Object.assign(new Error(message),{name:"StaleImplementationErrorV1",code:protocol.staleImplementationCode,moved:Object.freeze([])});}

const restartableHandlerKeyV1=Symbol.for("semio.restartableFailureHandler.v1");
/** 🪝️ Installs the one handler that turns a failure a fresh process cures (stale implementation, half-edited module, escaped concurrent edit) into the declared exit code of the process; the repository library installs it, a script entry point calls {@link handleRestartableFailureV1} without depending on the library. */
export function installRestartableFailureHandlerV1(handler:(error:unknown)=>void):void{(globalThis as unknown as Record<symbol,unknown>)[restartableHandlerKeyV1]=handler;}
/** 🚪️ Lets the installed handler end the process with the declared exit code when `error` is a failure a fresh process cures; returns normally for any other failure and when no handler is installed. */
export function handleRestartableFailureV1(error:unknown):void{const handler=(globalThis as unknown as Record<symbol,unknown>)[restartableHandlerKeyV1];if(typeof handler==="function")(handler as (error:unknown)=>void)(error);}
/** 🏷️ The error code that marks a stale implementation. */
export const STALE_IMPLEMENTATION_CODE_V1:string=protocol.staleImplementationCode;
/** ❌️ Every error code that means the implementation this process loaded is outdated: the code {@link staleImplementationV1} raises and the graph layer's `BUILD_AUTHORITY_CHANGED`, which both end in a restart with the current implementation. */
export const STALE_IMPLEMENTATION_CODES_V1:ReadonlySet<string>=new Set(protocol.staleImplementationCodes);

/** 🤝️ A custody a failing command holds that its replacement may take over instead of queueing for it again (the machine capacity grant): `offer` publishes the custody to the process about to be started and returns the environment that process needs to claim it, `settle` ends this process's hold (idempotent; the custody was claimed, refused or never offered). */
export type RestartCustodyV1=Readonly<{name:string;offer:(environment:Readonly<Record<string,string|undefined>>)=>Promise<Readonly<Record<string,string>>>;settle:()=>void|Promise<void>}>;
type RestartStateV1={custodies:Set<RestartCustodyV1>;scopes:boolean[]};
const restartStateKeyV1=Symbol.for("semio.restartCustody.v1");
const restartState=():RestartStateV1=>{const holder=globalThis as unknown as Record<symbol,RestartStateV1|undefined>;return holder[restartStateKeyV1]??=({custodies:new Set(),scopes:[]});};
/** 🔄️ Opens the scope of one restart wrapper (`outermost`: it will start a replacement process when the command yields for a restart); returns the function that closes it. A custody is only parked while an outermost scope is open, so no parked custody can outlive a command nobody restarts. */
export function enterRestartScopeV1(outermost:boolean):()=>void{const scopes=restartState().scopes;scopes.push(outermost);let open=true;return()=>{if(!open)return;open=false;const at=scopes.lastIndexOf(outermost);if(at>=0)scopes.splice(at,1);};}
/** 🔓️ Whether a restart wrapper is open that will start a replacement when the command yields for a restart. */
export function restartExpectedV1():boolean{return restartState().scopes.includes(true);}
/** 📋️ Registers a custody that survives a restart failure; returns the function that withdraws it. */
export function registerRestartCustodyV1(custody:RestartCustodyV1):()=>void{const set=restartState().custodies;set.add(custody);return()=>{set.delete(custody);};}
/** 📤️ Offers every registered custody to the replacement process about to start and merges the environment each one needs; a custody that cannot offer is skipped (the replacement then waits for it like any other process). */
export async function offerRestartCustodiesV1(environment:Readonly<Record<string,string|undefined>>=process.env):Promise<Readonly<Record<string,string>>>{
 let merged:Record<string,string>={};for(const custody of [...restartState().custodies])try{merged={...merged,...await custody.offer(environment)};}catch{}
 return merged;
}
/** 🧹️ Ends this process's hold on every registered custody (the replacement ended or none was started). */
export async function settleRestartCustodiesV1():Promise<void>{for(const custody of [...restartState().custodies])try{await custody.settle();}catch{}restartState().custodies.clear();}

/** 🚧️ Whether a filesystem error says "busy" in the closed race table: the sharing violations (`EBUSY`, `EPERM`, `EACCES`) a Windows host raises while another process (a reader, a scanner, an indexer) holds the file open; on POSIX hosts only the codes the table lists for every platform. */
export function filesystemBusyErrorV1(error:unknown,platform:string=process.platform):boolean{
 const code=(error as {code?:unknown}|null)?.code;if(typeof code!=="string")return false;
 const table=filesystemRaceTableV1().codes;
 return (Object.hasOwn(table.all,code)?table.all[code]:platform==="win32"&&Object.hasOwn(table.win32,code)?table.win32[code]:undefined)==="busy";
}

/** 🛻️ Renames a staged file over its target, retrying a busy target (a Windows reader, scanner or indexer that holds it open) on the `read` layer's declared pauses; the last busy failure, or any other failure, is thrown as it was, so a publish never fails on the first sharing violation. See https://learn.microsoft.com/windows/win32/fileio/obtaining-a-handle-to-a-file-for-renaming */
export function renameOverBusyFileSyncV1(from:string,to:string,options:Readonly<{platform?:string;sleepSync?:(milliseconds:number)=>void;report?:(text:string)=>void;environment?:Readonly<Record<string,string|undefined>>;rename?:(from:string,to:string)=>void}>={}):void{
 const{rename=renameSync,...rest}=options,outcome=settleBusyFilesystemSyncV1(to,()=>{rename(from,to);},rest);
 if(!outcome.settled)throw outcome.error;
}

/** 🪞️ The verdict of {@link settleBusyFilesystemSyncV1}: the value, or the error that stayed after the bounded retries. */
export type BusySettleV1<T>=Readonly<{settled:true;value:T}>|Readonly<{settled:false;error:unknown}>;

/** 🏃️ Runs one filesystem operation that must not fail its caller (a lock release in a `finally`, a store rename): a busy file is retried on the `read` layer's declared pauses, anything else, or the last busy failure, is returned as `settled:false` and never thrown. `report` receives the bilingual exhaustion text once, so a stale leftover is never silent. */
export function settleBusyFilesystemSyncV1<T>(path:string,operation:()=>T,options:Readonly<{platform?:string;sleepSync?:(milliseconds:number)=>void;report?:(text:string)=>void;environment?:Readonly<Record<string,string|undefined>>}>={}):BusySettleV1<T>{
 const platform=options.platform??process.platform;let original:unknown;
 try{
  const value=retryOnConcurrentEditSyncV1("read",()=>{
   try{return operation();}
   catch(error){if(filesystemBusyErrorV1(error,platform)){original=error;throw concurrentEditV1("busy",path,String((error as {code?:unknown}).code));}throw error;}
  },{onRetry:()=>undefined,...options.sleepSync?{sleepSync:options.sleepSync}:{},...options.environment?{environment:options.environment}:{}});
  return Object.freeze({settled:true as const,value});
 }catch(error){
  const exhausted=error instanceof ConcurrentEditErrorV1&&original!==undefined;
  if(exhausted)try{options.report?.((error as Error).message);}catch{}
  return Object.freeze({settled:false as const,error:exhausted?original:error});
 }
}
