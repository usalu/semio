import{listLeaseRegistryDirectoriesV1,leaseRegistryRootV1,readLeaseRegistryEntriesV1,type LeaseRegistryEntryV1}from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import{wantedTextV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{wantedServerPolicyV1,type WantedServerPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";

/** 👀️ Every live entry of the lease registry of this user, of all lease directories (nothing is pruned: the graph server only looks). */
export function readGraphLeaseEntriesV1(environment:Readonly<Record<string,string|undefined>>=process.env,nowMs:number=Date.now()):readonly LeaseRegistryEntryV1[]{
 const root=leaseRegistryRootV1(environment),found:LeaseRegistryEntryV1[]=[];
 try{for(const directory of listLeaseRegistryDirectoriesV1(root))found.push(...readLeaseRegistryEntriesV1(root,directory,nowMs,false));}catch{}
 return found;
}

/** 🚧️ A command that holds a resource of the read exclusively: the read cannot start behind it. */
export type GraphLeaseBlockerV1=Readonly<{pid:number;command:string;resource:string;heldMilliseconds:number;phase:string|undefined;progressMilliseconds:number}>;

/** 🚧️ The exclusive holds of other live commands on a resource whose name starts with one of `prefixes` (the native image every `repo` command attaches shared): the commands a start would wait behind, longest holder first. */
export function graphReadBlockersV1(input:Readonly<{entries:readonly LeaseRegistryEntryV1[];prefixes:readonly string[];ownPid:number;nowMs:number}>):readonly GraphLeaseBlockerV1[]{
 const found:GraphLeaseBlockerV1[]=[];
 for(const row of input.entries){
  if(row.pid===input.ownPid)continue;
  for(const hold of row.holds)if(hold.mode==="exclusive"&&input.prefixes.some(prefix=>hold.resource.startsWith(prefix)))found.push({pid:row.pid,command:row.command.slice(0,96),resource:hold.resource,heldMilliseconds:Math.max(0,input.nowMs-hold.sinceMs),phase:row.standing.phase,progressMilliseconds:Math.max(0,input.nowMs-row.standing.progressAtMs)});
 }
 return found.sort((left,right)=>right.heldMilliseconds-left.heldMilliseconds||left.pid-right.pid);
}

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise<void>(done=>{
 if(signal?.aborted)return done();
 const timer=setTimeout(done,milliseconds);
 signal?.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});
});

/** ⏳️ Holds the start of the read of a request while another command holds the native image exclusively (`tools setup all` runs for a quarter of an hour): a start now would only reserve gigabytes of the capacity gate and idle behind the hold. Looks every `leasePollMilliseconds`, tells the holder every `leaseNoticeMilliseconds`, and starts anyway after `leaseWaitMilliseconds` (the read then queues in the highest class, so the wait is bounded and never silent). Answers the milliseconds waited. */
export async function awaitGraphReadLeasesV1(input:Readonly<{blockers():readonly GraphLeaseBlockerV1[];language:"en"|"de";say(line:string):void;signal:AbortSignal|undefined;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;policy?:WantedServerPolicyV1}>):Promise<number>{
 const now=input.now??Date.now,policy=input.policy??wantedServerPolicyV1,sleep=input.sleep??pause,started=now();let noticed=-Infinity,last:GraphLeaseBlockerV1|undefined;
 for(;;){
  if(input.signal?.aborted)return 0;
  const found=input.blockers(),blocker=found[0],waited=now()-started;
  if(blocker===undefined)return waited>0?(input.say(wantedTextV1("leaseWaitFree",input.language,{resource:last?.resource??"",seconds:Math.round(waited/1000)})),waited):0;
  last=blocker;
  if(waited>=policy.leaseWaitMilliseconds){input.say(wantedTextV1("leaseWaitLimit",input.language,{resource:blocker.resource,seconds:Math.round(waited/1000)}));return waited;}
  if(now()-noticed>=policy.leaseNoticeMilliseconds){noticed=now();input.say(wantedTextV1("leaseWait",input.language,{resource:blocker.resource,holder:blocker.pid,command:blocker.command,phase:blocker.phase??"-",held:Math.round(blocker.heldMilliseconds/1000),progress:Math.round(blocker.progressMilliseconds/1000),seconds:Math.round(waited/1000)}));}
  await sleep(policy.leasePollMilliseconds,input.signal);
 }
}

/** 🌡️ What the diagnosis needs of the capacity gate: the host sample, the grants and the queue. */
export type GraphHostViewV1=Readonly<{host:Readonly<{pressure:string;availableMegabytes:number;totalMegabytes:number}>;grants:readonly Readonly<{id:string;pid:number;reservedMegabytes:number}>[];waiters:readonly Readonly<{pid:number;arrivedAtMs:number;request:Readonly<{id:string;priority:string}>}>[]}>;

const inTree=(entries:readonly LeaseRegistryEntryV1[],rootPid:number,row:LeaseRegistryEntryV1):boolean=>{
 const byPid=new Map(entries.map(entry=>[entry.pid,entry] as const));let at:LeaseRegistryEntryV1|undefined=row;
 for(let depth=0;depth<12&&at!==undefined;depth++){if(at.pid===rootPid)return true;if(at.standing.parentPid===rootPid)return true;at=at.standing.parentPid===undefined?undefined:byPid.get(at.standing.parentPid);}
 return false;
};

/** 🔬️ The lines that say what a silent read waits for: every lease its process tree waits for with the live command that holds it (its phase and last progress), the capacity grant it holds or its place in the queue behind others, the pressure, and its last output. Nothing registered says so too, so a silence is never unexplained. */
export function describeGraphReadStallV1(input:Readonly<{rootPid:number;entries:readonly LeaseRegistryEntryV1[];view:GraphHostViewV1|undefined;nowMs:number;silentMilliseconds:number;lastOutput:string;language:"en"|"de"}>):readonly string[]{
 const{language,nowMs}=input,lines=[wantedTextV1("stallHead",language,{pid:input.rootPid,seconds:Math.round(input.silentMilliseconds/1000)})];let explained=false;
 for(const row of input.entries.filter(entry=>inTree(input.entries,input.rootPid,entry))){
  for(const wait of row.waits){
   explained=true;
   const holder=input.entries.find(other=>other.pid!==row.pid&&other.holds.some(hold=>hold.resource===wait.resource&&(hold.mode==="exclusive"||wait.mode==="exclusive"))),hold=holder?.holds.find(item=>item.resource===wait.resource);
   const common={resource:wait.resource,mode:wait.mode,seconds:Math.round(Math.max(0,nowMs-wait.sinceMs)/1000)};
   lines.push(holder===undefined||hold===undefined?wantedTextV1("stallLeaseFree",language,common):wantedTextV1("stallLease",language,{...common,holder:holder.pid,command:holder.command.slice(0,96),phase:holder.standing.phase??"-",held:Math.round(Math.max(0,nowMs-hold.sinceMs)/1000),progress:Math.round(Math.max(0,nowMs-holder.standing.progressAtMs)/1000)}));
  }
 }
 const view=input.view;
 if(view!==undefined){
  const grant=view.grants.find(row=>row.pid===input.rootPid),waiting=view.waiters.find(row=>row.pid===input.rootPid),host={pressure:view.host.pressure,free:Math.round(view.host.availableMegabytes),total:Math.round(view.host.totalMegabytes)};
  if(waiting!==undefined){explained=true;lines.push(wantedTextV1("stallQueued",language,{id:waiting.request.id,class:waiting.request.priority,ahead:view.waiters.filter(row=>row.arrivedAtMs<waiting.arrivedAtMs).length,...host}));}
  else if(grant!==undefined)lines.push(wantedTextV1("stallGrant",language,{grant:grant.id,reserved:Math.round(grant.reservedMegabytes),grants:view.grants.length,waiters:view.waiters.length,...host}));
 }
 if(!explained&&lines.length===1)lines.push(wantedTextV1("stallNothing",language));
 if(input.lastOutput.trim()!=="")lines.push(wantedTextV1("stallOutput",language,{last:input.lastOutput.trim().split("\n").at(-1)!.slice(0,240)}));
 return lines;
}
