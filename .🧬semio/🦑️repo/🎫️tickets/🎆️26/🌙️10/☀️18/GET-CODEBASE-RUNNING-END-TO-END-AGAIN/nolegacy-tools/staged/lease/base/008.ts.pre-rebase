import {createHash} from "node:crypto";
import {readdirSync,readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {classifyLeaseResourceV1,findLeaseCycleV1,formatLeaseViolationV1,leaseBlockersV1,leaseLanguageV1,leaseOrderPolicyV1,shortLeaseResourceV1,type LeaseCycleV1,type LeaseModeV1,type LeaseNodeV1,type LeaseViolationV1} from "../🧭️order/🟦️.ts";
import {leaseRegistryRootV1,listLeaseRegistryDirectoriesV1,readLeaseHistoryV1,readLeaseRegistryEntriesV1,readLeaseViolationsV1,type LeaseRegistryEntryV1} from "../🧭️order/🗂️registry/🟦️.ts";
import {agentSessionRegistryRootV1,agentSessionStoreV1,parseAgentSessionAddressV1,readAgentSessionV1} from "../../🪪️agent-sessions/🗄️store/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {leaseBoundComponentsV1,leaseEtaV1,leaseHistoryKeyV1,leaseHolderStateV1,leaseStandingTextV1,type LeaseImplementationV1,type LeasePriorityV1} from "../📈️standing/🟦️.ts";
import {readLeaseYieldRequestV1,type LeaseYieldRequestV1} from "../📈️standing/🤝️yield/🟦️.ts";

const protocol=schema.definitions.Protocol.const;
/** 🩺️ What a holder reports about itself and how it fares: state, phase, counters, time left, implementation age and ancestry (`HolderStanding` of the schema). */
export type LeaseHolderStandingV1=Readonly<{state:"legacy"|"progressing"|"silent"|"stuck"|"stale";priority?:LeasePriorityV1;phase?:string;done?:number;total?:number;silentSeconds?:number;staleIds?:readonly string[];etaMinutes?:number;etaOverdue?:boolean;implementationDigest?:string;implementationAgeSeconds?:number;parentPid?:number;chain?:readonly number[];yieldRequest?:Readonly<{requesterPid:number;reason:"stale"|"stuck";ageSeconds:number}>}>;
/** 🎒️ One held lease. */
export type LeaseHolderV1=Readonly<{resource:string;hash:string;mode:LeaseModeV1;pid:number;task:number;sinceMs:number;heldSeconds:number;class:string;rank:number;source:"registry"|"queue";command?:string;session?:string;agent?:string;standing?:LeaseHolderStandingV1}>;
/** ⏳️ One waiting request and the participants it waits for. */
export type LeaseWaiterV1=Readonly<{resource:string;hash:string;mode:LeaseModeV1;pid:number;task:number;sinceMs:number;waitedSeconds:number;class:string;rank:number;source:"registry"|"queue";position?:number;queued?:number;blockers:readonly string[];command?:string;session?:string;agent?:string;priority?:LeasePriorityV1}>;
/** ➡️ One edge of the wait-for graph. */
export type LeaseEdgeV1=Readonly<{from:string;to:string;resource:string}>;
/** 🚪️ The machine-wide admission queue beside the leases: what is granted and what waits for memory or load (`Admission` of the schema). */
export type LeaseAdmissionV1=Readonly<{floorMegabytes:number;headroomMegabytes:number;grants:readonly Readonly<{id:string;operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes?:number|null;heldSeconds:number}>[];waiters:readonly Readonly<{id:string;operation:string;pid:number;needMegabytes:number;waitedSeconds:number;reason:string;reasonText:string}>[]}>;
/** 📋️ The whole report (`Report` of the schema). */
export type LeaseReportV1=Readonly<{version:1;directory:string;atMs:number;holders:readonly LeaseHolderV1[];waiters:readonly LeaseWaiterV1[];waitFor:readonly LeaseEdgeV1[];cycles:readonly LeaseCycleV1[];violations:readonly LeaseViolationV1[];admission?:LeaseAdmissionV1}>;

/** ▶️ How many leading tickets of an arrival queue hold the lease: the exclusive head alone, or the whole run of shared tickets at the head. */
const holderRun=(tickets:readonly Readonly<{mode:LeaseModeV1}>[]):number=>{if(tickets[0]?.mode==='exclusive')return 1;let count=0;while(count<tickets.length&&tickets[count]!.mode==='shared')count++;return count;};
const hash=(resource:string):string=>createHash("sha256").update(resource).digest("hex");
type Ticket=Readonly<{hash:string;position:number;queued:number;arrivedAtMs:number;pid:number;mode:LeaseModeV1}>;

/** 🎟️ Reads every arrival queue of one lease directory; the order of tickets is the order of service. */
export function readLeaseQueuesV1(directory:string):readonly Ticket[]{
 const found:Ticket[]=[];let names:string[];
 try{names=readdirSync(directory).filter(name=>/^[a-f0-9]{64}\.queue$/u.test(name)).slice(0,protocol.maximumQueueEntries);}catch{return found;}
 for(const name of names){
  let tickets:string[];try{tickets=readdirSync(join(directory,name)).filter(ticket=>/^[0-9a-z]{10}-[0-9a-z]{7}-[A-Za-z0-9._-]{1,64}$/u.test(ticket)).sort();}catch{continue;}
  const rows:Omit<Ticket,"position"|"queued">[]=[];
  for(const ticket of tickets){
   let row:{pid?:unknown;mode?:unknown}|undefined;try{row=JSON.parse(readFileSync(join(directory,name,ticket),"utf8"));}catch{continue;}
   if(!row||(row.mode!=="shared"&&row.mode!=="exclusive")||!Number.isSafeInteger(row.pid))continue;
   rows.push({hash:name.slice(0,64),arrivedAtMs:Number.parseInt(ticket.slice(0,10),36),pid:row.pid as number,mode:row.mode});
  }
  rows.sort((left,right)=>left.arrivedAtMs-right.arrivedAtMs).forEach((row,index)=>found.push(Object.freeze({...row,position:index+1,queued:rows.length})));
 }
 return found;
}

/** 🧮️ Combines the registry, the arrival queues and the violation log of one lease directory into a report; a pure function of its arguments. */
export function buildLeaseReportV1(input:Readonly<{directory:string;atMs:number;entries:readonly LeaseRegistryEntryV1[];queues:readonly Ticket[];violations:readonly LeaseViolationV1[];agents?:ReadonlyMap<string,string>;history?:(key:string)=>readonly number[];requests?:ReadonlyMap<number,LeaseYieldRequestV1>}>,environment:Readonly<Record<string,string|undefined>>=process.env):LeaseReportV1{
 const policy=leaseOrderPolicyV1(environment),names=new Map<string,string>(),holders:LeaseHolderV1[]=[],waiters:LeaseWaiterV1[]=[],nodes=new Map<string,{holds:{key:string;mode:LeaseModeV1}[];wait?:{key:string;mode:LeaseModeV1;sinceMs:number;queued:boolean}}>();
 const node=(id:string)=>{let row=nodes.get(id);if(!row){row={holds:[]};nodes.set(id,row);}return row;};
 const known=new Set<string>(),commands=new Map<number,string>(),sessions=new Map<number,Readonly<{session:string;agent?:string}>>(),owner=(pid:number)=>{const row=sessions.get(pid);return row===undefined?{}:{session:row.session,...row.agent===undefined?{}:{agent:row.agent}};};
 const newest=input.entries.map(entry=>entry.standing?.implementation).filter((row):row is LeaseImplementationV1=>row!==undefined&&Object.keys(row.components).length>0).sort((left,right)=>right.loadedAtMs-left.loadedAtMs)[0],byPid=new Map(input.entries.map(entry=>[entry.pid,entry] as const));
 const standingOf=(entry:LeaseRegistryEntryV1|undefined,resource:string,sinceMs:number):LeaseHolderStandingV1=>{
  const standing=entry?.standing,state=leaseHolderStateV1({nowMs:input.atMs,holdSinceMs:sinceMs,resource,standing,...newest?{waiter:newest}:{},bound:leaseBoundComponentsV1(resource)});
  if(!standing||!entry)return{state:"legacy"};
  const eta=leaseEtaV1({elapsedMs:input.atMs-sinceMs,phaseElapsedMs:standing.phaseStartedAtMs===undefined?0:input.atMs-standing.phaseStartedAtMs,...standing.done===undefined||standing.total===undefined?{}:{done:standing.done,total:standing.total},historyMs:input.history?.(leaseHistoryKeyV1(resource,entry.command))??[]}),request=input.requests?.get(entry.pid);
  return{state:state.state,priority:standing.priority,...standing.phase===undefined?{}:{phase:standing.phase},...standing.done===undefined?{}:{done:standing.done},...standing.total===undefined?{}:{total:standing.total},...state.silentMs===undefined?{}:{silentSeconds:Math.floor(state.silentMs/1000)},...state.staleIds?{staleIds:state.staleIds}:{},...eta&&!eta.overdue?{etaMinutes:Math.max(1,Math.round(eta.seconds/60))}:{},...eta?.overdue?{etaOverdue:true}:{},implementationDigest:standing.implementation.digest,implementationAgeSeconds:Math.max(0,Math.floor((input.atMs-standing.implementation.loadedAtMs)/1000)),...standing.parentPid===undefined?{}:{parentPid:standing.parentPid},...standing.chain?{chain:standing.chain}:{},...request?{yieldRequest:{requesterPid:request.requesterPid,reason:request.reason,ageSeconds:Math.max(0,Math.floor((input.atMs-request.atMs)/1000))}}:{}};
 };
 for(const entry of input.entries){
  commands.set(entry.pid,entry.command);if(entry.session!==undefined)sessions.set(entry.pid,{session:entry.session,...input.agents?.has(entry.session)?{agent:input.agents.get(entry.session)!}:{}});
  for(const row of entry.holds){
   const key=hash(row.resource),cls=classifyLeaseResourceV1(policy,row.resource);names.set(key,row.resource);known.add(`h\0${entry.pid}\0${key}`);
   holders.push({resource:row.resource,hash:key,mode:row.mode,pid:entry.pid,task:row.task,sinceMs:row.sinceMs,heldSeconds:Math.max(0,Math.floor((input.atMs-row.sinceMs)/1000)),class:cls.id,rank:cls.rank,source:"registry",command:entry.command,...owner(entry.pid),...row.mode==="exclusive"||entry.standing?{standing:standingOf(entry,row.resource,row.sinceMs)}:{}});
   node(`${entry.pid}/${row.task}`).holds.push({key,mode:row.mode});
  }
  for(const row of entry.waits){
   const key=hash(row.resource);names.set(key,row.resource);known.add(`w\0${entry.pid}\0${key}`);
   node(`${entry.pid}/${row.task}`).wait={key,mode:row.mode,sinceMs:row.sinceMs,queued:row.queued};
  }
 }
 const byKey=new Map<string,Ticket[]>();
 for(const ticket of input.queues)byKey.set(ticket.hash,[...byKey.get(ticket.hash)??[],ticket]);
 for(const [key,tickets] of byKey)for(const ticket of tickets){
  const resource=names.get(key)??"",cls=resource?classifyLeaseResourceV1(policy,resource):{id:"unclassified",rank:0};
  const id=`${ticket.pid}/q`,run=holderRun(tickets),asHolder=ticket.position<=run;
  if(asHolder){
   if(known.has(`h\0${ticket.pid}\0${key}`))continue;
   holders.push({resource,hash:key,mode:ticket.mode,pid:ticket.pid,task:0,sinceMs:ticket.arrivedAtMs,heldSeconds:Math.max(0,Math.floor((input.atMs-ticket.arrivedAtMs)/1000)),class:cls.id,rank:cls.rank,source:"queue",...commands.has(ticket.pid)?{command:commands.get(ticket.pid)}:{},...owner(ticket.pid),...ticket.mode==="exclusive"?{standing:standingOf(byPid.get(ticket.pid),resource,ticket.arrivedAtMs)}:{}});
   node(id).holds.push({key,mode:ticket.mode});
  }else if(!known.has(`w\0${ticket.pid}\0${key}`)&&!known.has(`h\0${ticket.pid}\0${key}`)){
   node(id).wait={key,mode:ticket.mode,sinceMs:ticket.arrivedAtMs,queued:true};
  }
 }
 const list=[...nodes.entries()].map(([id,row])=>({id,holds:row.holds,...row.wait?{wait:row.wait}:{}}) as LeaseNodeV1),edges:LeaseEdgeV1[]=[],cycles:LeaseCycleV1[]=[],seen=new Set<string>();
 for(const row of list){
  if(!row.wait)continue;
  const blockers=leaseBlockersV1(list,row),pid=Number.parseInt(row.id,10),task=row.id.endsWith("/q")?0:Number.parseInt(row.id.slice(row.id.indexOf("/")+1),10),cls=names.has(row.wait.key)?classifyLeaseResourceV1(policy,names.get(row.wait.key)!):{id:"unclassified",rank:0};
  for(const to of blockers)edges.push({from:row.id,to,resource:names.get(row.wait.key)??""});
  const ticket=byKey.get(row.wait.key)?.find(candidate=>candidate.pid===pid);
  waiters.push({resource:names.get(row.wait.key)??"",hash:row.wait.key,mode:row.wait.mode,pid,task,sinceMs:row.wait.sinceMs,waitedSeconds:Math.max(0,Math.floor((input.atMs-row.wait.sinceMs)/1000)),class:cls.id,rank:cls.rank,source:known.has(`w\0${pid}\0${row.wait.key}`)?"registry":"queue",...ticket?{position:ticket.position,queued:ticket.queued}:{},blockers,...commands.has(pid)?{command:commands.get(pid)}:{},...owner(pid),...byPid.get(pid)?.standing?{priority:byPid.get(pid)!.standing!.priority}:{}});
  const cycle=findLeaseCycleV1(list,row.id);
  if(cycle){const signature=[...cycle.members].sort().join("|");if(!seen.has(signature)){seen.add(signature);cycles.push(cycle);}}
 }
 holders.sort((left,right)=>right.heldSeconds-left.heldSeconds||left.pid-right.pid);
 waiters.sort((left,right)=>right.waitedSeconds-left.waitedSeconds||left.pid-right.pid);
 return Object.freeze({version:1,directory:input.directory,atMs:input.atMs,holders,waiters,waitFor:edges,cycles,violations:input.violations});
}

const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{([A-Za-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
/** 🖨️ Renders a report as lines in the chosen language. */
export function formatLeaseReportV1(language:"en"|"de",report:LeaseReportV1):readonly string[]{
 const catalogue=schema.definitions.Messages.const,text=Object.fromEntries(Object.entries(catalogue).map(([key,value])=>[key,value[language]])) as Record<keyof typeof catalogue,string>,name=(row:{resource:string;hash:string})=>row.resource?shortLeaseResourceV1(row.resource):fill(text.unnamed,{hash:row.hash.slice(0,12)});
 const command=(row:{command?:string;session?:string;agent?:string})=>(row.session===undefined?"":fill(text.sessionTag,{session:row.session.slice(row.session.indexOf(".")+1),agent:row.agent??"?"}))+(row.command?`  ${row.command.slice(0,80)}`:"");
 const lines=[fill(text.header,{directory:report.directory,holders:report.holders.length,waiters:report.waiters.length,cycles:report.cycles.length,violations:report.violations.length})];
 if(report.holders.length===0&&report.waiters.length===0)lines.push(fill(text.empty,{directory:report.directory}));
 const standingLine=(row:LeaseHolderV1):string=>{
  const standing=row.standing!,say=(key:Parameters<typeof leaseStandingTextV1>[1],values:Record<string,string|number>={})=>leaseStandingTextV1(language,key,values);
  if(standing.state==="legacy")return say("status.legacy",{state:say("state.legacy")});
  const progress=standing.phase===undefined?say("holderIdle"):standing.done===undefined?say("holderPhase",{phase:standing.phase}):say("holderProgress",{phase:standing.phase,done:standing.done,total:standing.total??0}),eta=standing.etaMinutes!==undefined?" · "+say("eta",{minutes:standing.etaMinutes}):standing.etaOverdue?" · "+say("etaOverdue"):"";
  const state=standing.state==="silent"?say("state.silent",{seconds:standing.silentSeconds??0}):standing.state==="stuck"?say("state.stuck",{minutes:Math.round((standing.silentSeconds??0)/60)}):standing.state==="stale"?say("state.stale",{ids:(standing.staleIds??[]).join(", ")}):say(`state.${standing.state}`);
  const chain=standing.chain?.length?say("status.chain",{chain:standing.chain.join(" > ")}):standing.parentPid!==undefined?say("status.chain",{chain:String(standing.parentPid)}):"",request=standing.yieldRequest?say("status.request",{pid:standing.yieldRequest.requesterPid,reason:standing.yieldRequest.reason}):"";
  return say("status.standing",{priority:standing.priority??"?",age:standing.implementationAgeSeconds??0,progress,eta,state,chain,request});
 };
 for(const row of report.holders){
  lines.push(fill(text.holder,{mode:row.mode,resource:name(row),pid:row.pid,seconds:row.heldSeconds,class:`${row.class} ${row.rank}`,command:command(row)}));
  if(row.standing&&(row.mode==="exclusive"||row.standing.state==="stale"||row.standing.state==="stuck"))lines.push(standingLine(row));
 }
 for(const row of report.waiters)lines.push(fill(text.waiter,{mode:row.mode,resource:name(row),pid:row.pid,seconds:row.waitedSeconds,position:row.position===undefined?"":fill(text.position,{position:row.position,queued:row.queued??row.position}),behind:row.blockers.length?fill(text.behind,{blockers:row.blockers.join(", ")}):"",command:(row.priority===undefined?"":`  [${row.priority}]`)+command(row)}));
 for(const row of report.cycles)lines.push(fill(text.cycle,{members:row.members.join(" → ")+` (${row.victim})`}));
 for(const row of report.violations)lines.push(fill(text.violation,{message:formatLeaseViolationV1(language,row)}));
 if(report.admission){
  const admission=report.admission;
  lines.push(fill(text.admissionHeader,{spare:admission.headroomMegabytes,floor:admission.floorMegabytes,grants:admission.grants.length,waiters:admission.waiters.length}));
  for(const row of admission.grants)lines.push(fill(text.admissionGrant,{operation:row.operation,pid:row.pid,jobs:row.jobs,reserved:row.reservedMegabytes,observed:row.observedMegabytes??"n/a",seconds:row.heldSeconds}));
  for(const row of admission.waiters)lines.push(fill(text.admissionWaiter,{operation:row.operation,pid:row.pid,need:row.needMegabytes,seconds:row.waitedSeconds,reason:row.reasonText}));
 }
 return lines;
}

/** 🔌️ What the command touches: output, environment, clock and the default lease directory; tests replace them. */
export type LeasesCommandContextV1=Readonly<{environment:NodeJS.ProcessEnv;write:(line:string)=>void;directory?:string;now?:()=>number;admission?:(language:"en"|"de")=>Promise<LeaseAdmissionV1|undefined>}>;
const flag=(args:readonly string[],name:string):string|undefined=>{for(const argument of args)if(argument.startsWith(`--${name}=`))return argument.slice(name.length+3);return undefined;};

/** 📖️ Reads the live state of one lease directory. */
export function readLeaseReportV1(directory:string,environment:Readonly<Record<string,string|undefined>>,atMs:number,violations:number):LeaseReportV1{
 const root=leaseRegistryRootV1(environment),resolved=resolve(directory);
 const entries=readLeaseRegistryEntriesV1(root,resolved,atMs),agents=new Map<string,string>(),sessions=agentSessionRegistryRootV1(environment);
 for(const entry of entries){const address=parseAgentSessionAddressV1(entry.session);if(address===undefined||agents.has(entry.session!))continue;const found=readAgentSessionV1(agentSessionStoreV1(sessions,address.worktree),address.id);if(found!==undefined)agents.set(entry.session!,found.agent.id);}
 const requests=new Map<number,LeaseYieldRequestV1>();
 for(const entry of entries){const request=readLeaseYieldRequestV1(root,resolved,entry.pid,entry.startedAtMs,atMs);if(request)requests.set(entry.pid,request);}
 return buildLeaseReportV1({directory:resolved,atMs,entries,queues:readLeaseQueuesV1(resolved),violations:readLeaseViolationsV1(root,resolved,violations),agents,history:key=>readLeaseHistoryV1(root,resolved,key),requests},environment);
}

/** 🔒️ Dispatches `repo leases status [--json] [--lease-directory=<path>] [--all] [--violations=<n>]`; the machine-wide admission queue is appended when the context offers it. */
export async function runLeasesCommandV1(args:readonly string[],context:LeasesCommandContextV1):Promise<number>{
 if(args[0]!=="status")throw Error("Expected repo leases status");
 const known=new Set(["--json","--all"]);
 for(const argument of args.slice(1))if(!known.has(argument)&&!/^--(lease-directory|violations)=./u.test(argument))throw Error(`Unknown leases option: ${argument}`);
 const now=(context.now??Date.now)(),language=leaseLanguageV1(context.environment),limit=Number(flag(args,"violations")??protocol.defaultViolations);
 if(!Number.isSafeInteger(limit)||limit<0)throw Error("Leases option --violations requires a non-negative integer");
 const root=leaseRegistryRootV1(context.environment),explicit=flag(args,"lease-directory"),directories=args.includes("--all")?[...new Set([...(context.directory?[resolve(context.directory)]:[]),...listLeaseRegistryDirectoriesV1(root)])]:[resolve(explicit??context.directory??"")];
 if(!args.includes("--all")&&explicit===undefined&&context.directory===undefined)throw Error("A lease directory is required");
 const admission=await context.admission?.(language).catch(()=>undefined),reports=directories.map((directory,index)=>({...readLeaseReportV1(directory,context.environment,now,limit),...admission&&index===0?{admission}:{}}) as LeaseReportV1);
 if(args.includes("--json")){context.write(JSON.stringify(args.includes("--all")?reports:reports[0],null,2));return 0;}
 for(const report of reports)for(const line of formatLeaseReportV1(language,report))context.write(line);
 return 0;
}
