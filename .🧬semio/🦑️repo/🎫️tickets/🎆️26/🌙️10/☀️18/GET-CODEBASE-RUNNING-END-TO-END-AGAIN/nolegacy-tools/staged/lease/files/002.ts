import {spawnSync} from "node:child_process";
import {appendFileSync,mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {abortLeaseRootV1,leaseBoundComponentsV1,leaseEtaV1,leaseHistoryKeyV1,leaseHolderStateV1,leaseImplementationV1,leaseMovedBoundV1,leaseStandingSnapshotV1,leaseStandingTextV1,leaseStuckAfterMsV1,leaseYieldHandlerV1,probeLeaseActivityV1,type LeaseHolderStateV1,type LeaseImplementationV1,type LeaseStandingV1} from "../🟦️.ts";
import {leaseProcessStartedAtMsV1,leaseRegistryFolderV1,leaseRegistryRootV1,leaseRegistryV1,readLeaseHistoryV1,readLeaseRegistryEntriesV1} from "../../🧭️order/🗂️registry/🟦️.ts";
import {agentSessionAddressOfEnvironmentV1} from "../../../🪪️agent-sessions/🗄️store/🟦️.ts";
import {liveChildPidsV1} from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🤝️ A waiter's request to a holder to give a resource up. */
export type LeaseYieldRequestV1=Readonly<{version:1;holderPid:number;holderStartedAtMs:number;requesterPid:number;requesterStartedAtMs:number;resource:string;reason:"stale"|"stuck";atMs:number;requesterComponents:Readonly<Record<string,string>>}>;
/** 🛟️ How far a waiter may go against a holder that is stale or stuck: only report it, ask it to yield, or stop it. */
export type LeaseReclaimModeV1="report"|"request"|"terminate";

/** 🌐️ The languages a notice is written in: the chosen one (`SEMIO_LANGUAGE`, `LC_ALL`, `LANG`), both when none is chosen. */
export function leaseLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env):readonly ("en"|"de")[]{
 const pick=(value:string|undefined):"en"|"de"|undefined=>{const code=String(value??"").toLowerCase().split(/[-_.:@]/u)[0];return code==="en"||code==="de"?code:undefined;};
 const chosen=pick(environment.SEMIO_LANGUAGE)??pick(environment.LC_ALL)??pick(environment.LANG);
 return chosen?[chosen]:["en","de"];
}
/** 🎚️ The reclaim mode: the environment value when it names a declared mode, else the declared default. */
export function leaseReclaimModeV1(environment:Readonly<Record<string,string|undefined>>=process.env):LeaseReclaimModeV1{
 const value=environment[policy.reclaim.environment];return value==="report"||value==="request"||value==="terminate"?value:policy.reclaim.mode as LeaseReclaimModeV1;
}

/** ⚖️ Whether a judged holder is asked to give the resource up: a stale holder whose resource is bound by a moved component, or a stuck holder when the mode allows asking. */
export function leaseYieldDecisionV1(state:LeaseHolderStateV1["state"],mode:LeaseReclaimModeV1,bound:readonly string[]):"stale"|"stuck"|undefined{
 if(state==="stale"&&bound.length>0&&mode!=="report")return "stale";
 if(state==="stuck"&&mode!=="report")return "stuck";
 return undefined;
}

/** 📬️ What a holder weighs before honouring a request: the request, its own identity and hold, and the components that bind and moved. */
export type LeaseYieldAcceptInputV1=Readonly<{request:LeaseYieldRequestV1;pid:number;startedAtMs:number;nowMs:number;holdsExclusive:boolean;holdSinceMs:number;standing:LeaseStandingV1;bound:readonly string[];moved:readonly string[]}>;
/** 🙋️ Whether a holder honours a request: it must be the addressed holder, the request must be fresh, the resource must be held exclusively, and the holder checks the claim itself against its own evidence, never against the claim alone: a stale request needs a bound component that differs from the requester's AND moved on disk since the holder loaded it (`moved`, read by the holder), a stuck request needs the holder's own clock, fed by the work its tree does, to be silent for the stuck time, so a misdirected, forged or outdated request cannot stop a healthy holder. */
export function leaseYieldAcceptV1(input:LeaseYieldAcceptInputV1):Readonly<{accept:boolean;refusal?:string;ids?:readonly string[]}>{
 const{request,standing}=input;
 if(request.holderPid!==input.pid||request.holderStartedAtMs!==input.startedAtMs)return{accept:false,refusal:"target"};
 if(input.nowMs-request.atMs>policy.yield.requestMaximumAgeMilliseconds)return{accept:false,refusal:"expired"};
 if(!input.holdsExclusive)return{accept:false,refusal:"not-held"};
 if(request.reason==="stale"){
  const ids=input.bound.filter(id=>input.moved.includes(id)&&standing.implementation.components[id]!==undefined&&request.requesterComponents[id]!==undefined&&standing.implementation.components[id]!==request.requesterComponents[id]).sort();
  return ids.length&&standing.implementation.loadedAtMs<=request.requesterStartedAtMs?{accept:true,ids}:{accept:false,refusal:"not-stale"};
 }
 const state=leaseHolderStateV1({nowMs:input.nowMs,holdSinceMs:input.holdSinceMs,resource:request.resource,standing});
 return state.state==="stuck"?{accept:true}:{accept:false,refusal:"not-stuck"};
}

/** 🔪️ What the decision to stop a holder weighs: the mode, the holder's state, the request time, progress since, both start times and whether the process may be signalled. */
export type LeaseTerminateInputV1=Readonly<{mode:LeaseReclaimModeV1;state:LeaseHolderStateV1["state"];nowMs:number;requestAtMs?:number;progressAtMs:number;registeredStartedAtMs?:number;osStartedAtMs?:number;signalable:boolean;self:boolean}>;
/** 🧯️ Whether a waiter may stop a holder that ignored the yield request. Only in `terminate` mode, only for a stuck holder (its own standing says so, aged from its acquisition), only after the grace period since the request, never when the holder made progress since, never without a verified identity (the operating system's start time equals the registered one), never another user's process or itself; a holder whose standing cannot be read is never stopped. */
export function leaseTerminateDecisionV1(input:LeaseTerminateInputV1):Readonly<{terminate:boolean;refusal?:"identity"|"progressed"|"owner"|"unknown"|"grace"}>{
 if(input.mode!=="terminate"||input.self||input.state!=="stuck")return{terminate:false};
 if(input.requestAtMs===undefined||input.nowMs-input.requestAtMs<policy.reclaim.graceMilliseconds)return{terminate:false,refusal:"grace"};
 if(input.progressAtMs>input.requestAtMs)return{terminate:false,refusal:"progressed"};
 if(input.osStartedAtMs===undefined)return{terminate:false,refusal:"unknown"};
 if(input.registeredStartedAtMs===undefined||Math.abs(input.osStartedAtMs-input.registeredStartedAtMs)>policy.reclaim.identityToleranceMilliseconds)return{terminate:false,refusal:"identity"};
 if(!input.signalable)return{terminate:false,refusal:"owner"};
 return{terminate:true};
}

/** 🔌️ What reclaiming touches outside the pure decisions; tests replace it. */
export type LeaseReclaimPortsV1=Readonly<{alive(pid:number):boolean;signal(pid:number,signal:"SIGTERM"|"SIGKILL"|0):boolean;osStartedAtMs(pid:number):number|undefined;sleep(milliseconds:number):Promise<void>;write(text:string):void;children?(pid:number):readonly number[]|undefined;now?():number}>;
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
/** ⏱️ Seconds in the elapsed-time column of `ps` (`[[dd-]hh:]mm:ss`), which carries no time zone. */
export function leaseElapsedSecondsV1(text:string):number|undefined{
 const match=/^\s*(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)\s*$/u.exec(text);
 return match?Number(match[1]??0)*86400+Number(match[2]??0)*3600+Number(match[3])*60+Number(match[4]):undefined;
}
/** 🧷️ The real ports: `ps` elapsed time for the start time on POSIX systems (unknown elsewhere, which refuses to stop anything). */
export const leaseReclaimPortsV1:LeaseReclaimPortsV1=Object.freeze({
 alive,
 signal:(pid,signal)=>{try{process.kill(pid,signal);return true;}catch{return false;}},
 osStartedAtMs:pid=>{
  if(process.platform==="win32")return undefined;
  const run=spawnSync("ps",["-o","etime=","-p",String(pid)],{encoding:"utf8",env:{...process.env,LC_ALL:"C"}}),seconds=leaseElapsedSecondsV1(run.stdout);
  return run.status===0&&seconds!==undefined?Date.now()-seconds*1000:undefined;
 },
 sleep:milliseconds=>new Promise(done=>setTimeout(done,milliseconds)),
 write:text=>{try{process.stderr.write(text);}catch{}},
 children:pid=>liveChildPidsV1(pid),
});

const requestPath=(root:string,directory:string,pid:number,startedAtMs:number):string=>join(leaseRegistryFolderV1(root,directory),policy.yield.directory,`${pid}-${startedAtMs}.json`);
/** ✍️ Files a request atomically. */
export function writeLeaseYieldRequestV1(root:string,directory:string,request:LeaseYieldRequestV1):void{
 const path=requestPath(root,directory,request.holderPid,request.holderStartedAtMs);mkdirSync(join(leaseRegistryFolderV1(root,directory),policy.yield.directory),{recursive:true,mode:0o700});
 const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(request));renameSync(temporary,path);
}
/** 📖️ Reads the request addressed to a holder; a malformed one, one of a dead requester or an expired one counts as none. */
export function readLeaseYieldRequestV1(root:string,directory:string,pid:number,startedAtMs:number,nowMs:number=Date.now()):LeaseYieldRequestV1|undefined{
 let row:LeaseYieldRequestV1;try{row=JSON.parse(readFileSync(requestPath(root,directory,pid,startedAtMs),"utf8"));}catch{return undefined;}
 const valid=row?.version===1&&row.holderPid===pid&&row.holderStartedAtMs===startedAtMs&&Number.isSafeInteger(row.requesterPid)&&Number.isSafeInteger(row.requesterStartedAtMs)&&typeof row.resource==="string"&&(row.reason==="stale"||row.reason==="stuck")&&Number.isSafeInteger(row.atMs)&&row.requesterComponents!==null&&typeof row.requesterComponents==="object";
 return valid&&nowMs-row.atMs<=policy.yield.requestMaximumAgeMilliseconds&&alive(row.requesterPid)?row:undefined;
}

const refusedOnce=new Set<string>();
/** 📢️ What the queue notice of a waiter says about the holder: its state, phase, counters and the minutes left. */
export type LeaseHolderNoticeV1=Readonly<{holderState:LeaseHolderStateV1["state"];holderPhase?:string;holderDone?:number;holderTotal?:number;holderEtaMinutes?:number}>;
/** 🔭️ What observing the holder at the head of the queue needs: the lease directory, the resource, the head's pid, mode and arrival, and the injectable clock, environment, root and ports. */
export type LeaseObserveInputV1=Readonly<{directory:string;resource:string;headPid:number;headMode:"shared"|"exclusive";headArrivalMs:number;nowMs?:number;environment?:Readonly<Record<string,string|undefined>>;root?:string;ports?:LeaseReclaimPortsV1}>;
/** 👀️ The waiter's view of the holder at the head of the queue: its state, phase, counters and time left for the queue notice, and, by the reclaim mode, the request to give the resource up and (only in `terminate` mode, after the grace period, with a verified identity) the stop of a holder that ignored it. */
export async function observeLeaseHolderV1(input:LeaseObserveInputV1):Promise<LeaseHolderNoticeV1|undefined>{
 if(input.headMode!=="exclusive")return undefined;
 const environment=input.environment??process.env,root=input.root??leaseRegistryRootV1(environment),ports=input.ports??leaseReclaimPortsV1,nowMs=input.nowMs??Date.now(),mode=leaseReclaimModeV1(environment);
 const entry=readLeaseRegistryEntriesV1(root,input.directory,Date.now(),false).find(row=>row.pid===input.headPid),hold=entry?.holds.find(row=>row.resource===input.resource&&row.mode==="exclusive"),standing=entry?.standing,holdSinceMs=hold?.sinceMs??input.headArrivalMs;
 const bound=leaseBoundComponentsV1(input.resource),own=leaseImplementationV1(),judged=leaseHolderStateV1({nowMs,holdSinceMs,resource:input.resource,standing,waiter:own,bound});
 const eta=standing?leaseEtaV1({elapsedMs:nowMs-holdSinceMs,phaseElapsedMs:standing.phaseStartedAtMs===undefined?0:nowMs-standing.phaseStartedAtMs,...standing.done===undefined||standing.total===undefined?{}:{done:standing.done,total:standing.total},historyMs:readLeaseHistoryV1(root,input.directory,leaseHistoryKeyV1(input.resource,entry!.command))}):undefined;
 const notice:LeaseHolderNoticeV1=Object.freeze({holderState:judged.state,...standing?.phase===undefined?{}:{holderPhase:standing.phase},...standing?.done===undefined||standing.total===undefined?{}:{holderDone:standing.done,holderTotal:standing.total},...eta&&!eta.overdue?{holderEtaMinutes:Math.max(1,Math.round(eta.seconds/60))}:{}});
 const reason=leaseYieldDecisionV1(judged.state,mode,bound),languages=leaseLanguagesV1(environment),say=(key:Parameters<typeof leaseStandingTextV1>[1],values:Record<string,string|number>)=>ports.write(languages.map(language=>leaseStandingTextV1(language,key,values)).join("\n")+"\n");
 const existing=entry?readLeaseYieldRequestV1(root,input.directory,entry.pid,entry.startedAtMs,nowMs):undefined;
 if(reason&&entry&&!(existing&&alive(existing.requesterPid)&&existing.reason===reason)){
  writeLeaseYieldRequestV1(root,input.directory,{version:1,holderPid:entry.pid,holderStartedAtMs:entry.startedAtMs,requesterPid:process.pid,requesterStartedAtMs:leaseProcessStartedAtMsV1,resource:input.resource,reason,atMs:nowMs,requesterComponents:own.components});
  say("request.sent",{pid:entry.pid,resource:input.resource,reason});
 }
 const request=entry?readLeaseYieldRequestV1(root,input.directory,entry.pid,entry.startedAtMs,nowMs):undefined,candidate=mode==="terminate"&&judged.state==="stuck",osStartedAtMs=candidate?ports.osStartedAtMs(input.headPid):undefined;
 const decision=mode==="terminate"&&!candidate?{terminate:false} as ReturnType<typeof leaseTerminateDecisionV1>:leaseTerminateDecisionV1({mode,state:judged.state,nowMs,...request?{requestAtMs:request.atMs}:{},progressAtMs:standing?.progressAtMs??0,...entry?{registeredStartedAtMs:entry.startedAtMs}:{},...osStartedAtMs===undefined?{}:{osStartedAtMs},signalable:candidate&&ports.signal(input.headPid,0),self:input.headPid===process.pid});
 if(decision.refusal&&decision.refusal!=="grace"&&!refusedOnce.has(`${input.headPid}:${entry?.startedAtMs}:${decision.refusal}`)){refusedOnce.add(`${input.headPid}:${entry?.startedAtMs}:${decision.refusal}`);say("reclaim.refused",{pid:input.headPid,reason:languages.map(language=>leaseStandingTextV1(language,`refusal.${decision.refusal}` as "refusal.identity")).join(" / ")});}
 if(decision.terminate){
  const minutes=Math.round((nowMs-holdSinceMs)/60000),tolerance=policy.reclaim.identityToleranceMilliseconds,same=(pid:number,started:number|undefined):boolean=>started!==undefined&&ports.osStartedAtMs(pid)!==undefined&&Math.abs(ports.osStartedAtMs(pid)!-started)<=tolerance;
  const tree=(ports.children?.(input.headPid)??[]).map(pid=>({pid,started:ports.osStartedAtMs(pid)})).filter(row=>row.started!==undefined);
  ports.signal(input.headPid,"SIGTERM");
  const clock=ports.now??Date.now,until=clock()+policy.reclaim.killGraceMilliseconds;
  while(ports.alive(input.headPid)&&clock()<until)await ports.sleep(250);
  if(ports.alive(input.headPid)&&same(input.headPid,osStartedAtMs))ports.signal(input.headPid,"SIGKILL");
  for(const child of tree)if(ports.alive(child.pid)&&same(child.pid,child.started))ports.signal(child.pid,"SIGKILL");
  say("reclaim.terminated",{pid:input.headPid,resource:input.resource,minutes});
  try{mkdirSync(leaseRegistryFolderV1(root,input.directory),{recursive:true,mode:0o700});appendFileSync(join(leaseRegistryFolderV1(root,input.directory),policy.reclaim.logFile),JSON.stringify({version:1,atMs:nowMs,pid:input.headPid,resource:input.resource,minutes,by:process.pid,state:judged.state})+"\n");}catch{}
 }
 return notice;
}

let watcher:ReturnType<typeof setInterval>|undefined;
const honoured=new Set<string>();
/** 👂️ Starts the holder's side once per process: every poll it reads the request addressed to it and, when the claim holds (`leaseYieldAcceptV1`), tells the registered handler to end the work; without a handler it aborts the root invocation with the message. */
export function ensureLeaseYieldWatcherV1(environment:Readonly<Record<string,string|undefined>>=process.env):void{
 if(watcher)return;
 const poll=():void=>{
  const root=leaseRegistryRootV1(environment),nowMs=Date.now(),agent=agentSessionAddressOfEnvironmentV1(environment)!==undefined;
  probeLeaseActivityV1(nowMs);
  for(const directory of leaseRegistryV1.directories()){
   const request=readLeaseYieldRequestV1(root,directory,process.pid,leaseProcessStartedAtMsV1,nowMs);
   if(!request)continue;
   const key=`${directory}\0${request.atMs}`;if(honoured.has(key))continue;
   const held=leaseRegistryV1.ownHolds(directory).find(row=>row.resource===request.resource&&row.mode==="exclusive");
   const verdict=leaseYieldAcceptV1({request,pid:process.pid,startedAtMs:leaseProcessStartedAtMsV1,nowMs,holdsExclusive:held!==undefined,holdSinceMs:held?.sinceMs??nowMs,standing:leaseStandingSnapshotV1(agent,nowMs,environment),bound:leaseBoundComponentsV1(request.resource),moved:leaseMovedBoundV1(request.resource)});
   if(!verdict.accept)continue;
   honoured.add(key);
   const languages=leaseLanguagesV1(environment),message=languages.map(language=>request.reason==="stale"?leaseStandingTextV1(language,"yield.stale",{resource:request.resource,pid:request.requesterPid,ids:(verdict.ids??[]).join(", ")}):leaseStandingTextV1(language,"yield.stuck",{resource:request.resource,pid:request.requesterPid,minutes:Math.round(leaseStuckAfterMsV1(request.resource)/60000)})).join("\n");
   try{process.stderr.write(message+"\n");}catch{}
   const handler=leaseYieldHandlerV1();
   if(handler)handler({reason:request.reason,resource:request.resource,requesterPid:request.requesterPid,message});else abortLeaseRootV1(Object.assign(Error(message),{code:"SEMIO_LEASE_YIELD"}));
   try{rmSync(requestPath(root,directory,process.pid,leaseProcessStartedAtMsV1),{force:true});}catch{}
  }
 };
 watcher=setInterval(poll,policy.yield.pollMilliseconds);watcher.unref();
}
