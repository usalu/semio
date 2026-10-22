import {DOCTOR_POLICY_V1,DOCTOR_PROTOCOL_V1,DOCTOR_RULES_V1,clipResourceV1,commandLabelV1,commandRelevantV1,fillDoctorV1,type DoctorAreaV1,type DoctorDataV1,type DoctorFactsV1,type DoctorFindingV1,type HolderFactsV1,type WaiterFactsV1,type DoctorPartyV1,type DoctorPhaseV1,type DoctorPolicyV1,type DoctorProcessRowV1,type DoctorReportV1,type DoctorRepairResultV1,type DoctorRuleV1,type DoctorSeverityV1,type DoctorVerdictV1,type DoctorWaitV1,type ProcessFactsV1} from "../🟦️.ts";

/** 🧮️ Everything the pure analysis derives from one facts document. */
export type DoctorAnalysisV1=Readonly<{findings:readonly DoctorFindingV1[];waits:readonly DoctorWaitV1[];processes:readonly DoctorProcessRowV1[]}>;
/** 🔌️ Policy and rule catalogue the analysis reads; tests replace them. */
export type DoctorAnalysisOptionsV1=Readonly<{policy?:DoctorPolicyV1;rules?:Readonly<Record<string,DoctorRuleV1>>}>;

const SEVERITY_RANK:Readonly<Record<DoctorSeverityV1,number>>={error:0,warn:1,info:2};
const PHASE_RANK:Readonly<Record<DoctorPhaseV1,number>>={"waiting-lease":0,"waiting-capacity":1,holding:2,admitted:3,orphan:4,running:5};
const ownerPattern=/^[a-z][a-z0-9-]*$/u;
const tenth=(value:number):number=>Math.round(value*10)/10;

/** 🧱️ Builds one finding from the catalogue: severity must be one the rule allows, data exactly what the rule declares. */
function finding(rules:Readonly<Record<string,DoctorRuleV1>>,id:string,severity:DoctorSeverityV1,subject:string,data:DoctorDataV1,owner?:string):DoctorFindingV1{
 const rule=rules[id];if(!rule)throw Error("Catalogued doctor rule required: "+id);
 if(!rule.severities.includes(severity))throw Error("Doctor rule "+id+" cannot take severity "+severity);
 const declared=[...rule.data].sort().join(","),given=Object.keys(data).sort().join(",");
 if(declared!==given)throw Error("Doctor rule "+id+" needs data "+declared+" but got "+given);
 const text=(value:{en:string;de:string})=>({en:fillDoctorV1(value.en,data),de:fillDoctorV1(value.de,data)});
 const fix=Object.freeze({kind:rule.fix.kind,...rule.fix.command===undefined?{}:{command:fillDoctorV1(rule.fix.command,data)},...rule.fix.repair===undefined?{}:{repair:rule.fix.repair},note:rule.fix.note});
 return Object.freeze({id,area:rule.area,owner:owner!==undefined&&ownerPattern.test(owner)?owner:rule.owner,severity,subject:subject.slice(0,300),data,title:text(rule.title),detail:text(rule.detail),fix});
}

/** 🔬️ Derives the findings, the wait chains and the process phases from the facts; a pure function of its arguments. */
export function analyzeDoctorV1(facts:DoctorFactsV1,options:DoctorAnalysisOptionsV1={}):DoctorAnalysisV1{
 const policy=options.policy??DOCTOR_POLICY_V1,rules=options.rules??DOCTOR_RULES_V1,found:DoctorFindingV1[]=[];
 const add=(id:string,severity:DoctorSeverityV1,subject:string,data:DoctorDataV1,owner?:string):void=>{found.push(finding(rules,id,severity,subject,data,owner));};
 const table=new Map<number,ProcessFactsV1>((facts.processes??[]).map(row=>[row.pid,row]));
 const label=new Map<number,string>();
 for(const row of facts.leases?.holders??[])label.set(row.pid,commandLabelV1(row.command));
 for(const row of facts.leases?.waiters??[])label.set(row.pid,commandLabelV1(row.command));
 for(const row of table.values())label.set(row.pid,commandLabelV1(row.command));
 const commandOf=(pid:number):string=>label.get(pid)??"?";
 const leases=facts.leases,capacity=facts.capacity,host=facts.host,storage=facts.storage,deadHolders=new Set(leases?.deadHolders??[]);

 for(const probe of facts.probes){
  if(probe.status==="failed")add("doctor.probe-failed","warn",probe.area,{area:probe.area,reason:probe.reason??"unknown"});
  else if(probe.status==="timeout")add("doctor.probe-timeout","warn",probe.area,{area:probe.area,seconds:Math.round((policy.probe.timeoutOverrides[probe.area]??policy.probe.timeoutMilliseconds)/1000)});
  else if(probe.status==="unavailable")add("doctor.probe-unavailable","info",probe.area,{area:probe.area,reason:probe.reason??"nothing to read"});
 }

 const waits:DoctorWaitV1[]=[],waiting=new Set<number>([...(leases?.waiters??[]).map(row=>row.pid),...(capacity?.waiters??[]).map(row=>row.pid)]);
 const holdersOf=(hash:string):readonly HolderFactsV1[]=>(leases?.holders??[]).filter(row=>row.hash===hash);
 for(const waiter of leases?.waiters??[]){
  const behind:DoctorPartyV1[]=[],seen=new Set<number>();
  for(const pid of waiter.blockers){
   if(seen.has(pid)||pid===waiter.pid)continue;seen.add(pid);
   const held=holdersOf(waiter.hash).find(row=>row.pid===pid);
   behind.push(held?{pid,command:commandOf(pid),role:"holder",heldSeconds:held.heldSeconds,reservedMegabytes:null}:{pid,command:commandOf(pid),role:"ahead",heldSeconds:null,reservedMegabytes:null});
  }
  const pickHolder=(rows:readonly DoctorPartyV1[]):DoctorPartyV1|undefined=>[...rows].filter(row=>row.role==="holder").sort((left,right)=>(right.heldSeconds??0)-(left.heldSeconds??0)||left.pid-right.pid)[0]??rows[0];
  let root=pickHolder(behind),state:"running"|"waiting"|"gone"|"unknown"="unknown";const visited=new Set<number>([waiter.pid]);
  for(let hop=0;root!==undefined&&hop<8;hop++){
   visited.add(root.pid);
   const next=(leases?.waiters??[]).find(row=>row.pid===root!.pid);
   if(!next){state=deadHolders.has(root.pid)?"gone":table.has(root.pid)?"running":"unknown";break;}
   const rows:DoctorPartyV1[]=next.blockers.filter(pid=>pid!==next.pid).map(pid=>{const held=holdersOf(next.hash).find(row=>row.pid===pid);return held?{pid,command:commandOf(pid),role:"holder" as const,heldSeconds:held.heldSeconds,reservedMegabytes:null}:{pid,command:commandOf(pid),role:"ahead" as const,heldSeconds:null,reservedMegabytes:null};});
   const candidate=pickHolder(rows);
   if(!candidate||visited.has(candidate.pid)){state="waiting";break;}
   root=candidate;
  }
  waits.push(Object.freeze({kind:"lease",pid:waiter.pid,command:commandOf(waiter.pid),waitedSeconds:waiter.waitedSeconds,subject:clipResourceV1(waiter.resource),reason:null,owner:ownerPattern.test(waiter.owner)?waiter.owner:"leases",position:waiter.position,behind:Object.freeze(behind),root:root?Object.freeze({pid:root.pid,command:root.command,heldSeconds:root.heldSeconds,state}):null}));
 }
 const grants=[...capacity?.grants??[]].sort((left,right)=>right.reservedMegabytes-left.reservedMegabytes||left.pid-right.pid);
 for(const waiter of capacity?.waiters??[]){
  const behind:DoctorPartyV1[]=grants.slice(0,3).map(grant=>({pid:grant.pid,command:commandOf(grant.pid),role:"grant" as const,heldSeconds:grant.heldSeconds,reservedMegabytes:grant.reservedMegabytes})),top=grants[0];
  waits.push(Object.freeze({kind:"capacity",pid:waiter.pid,command:commandOf(waiter.pid),waitedSeconds:waiter.waitedSeconds,subject:waiter.operation,reason:waiter.reason,owner:"host-capacity",position:null,behind:Object.freeze(behind),root:top?Object.freeze({pid:top.pid,command:commandOf(top.pid),heldSeconds:top.heldSeconds,state:waiting.has(top.pid)?"waiting" as const:table.has(top.pid)?"running" as const:"unknown" as const}):null}));
 }
 waits.sort((left,right)=>right.waitedSeconds-left.waitedSeconds||left.pid-right.pid);

 if(host){
  if(host.pressure==="critical")add("host.memory-pressure","error","memory",{pressure:host.pressure});
  else if(host.pressure==="warn")add("host.memory-pressure","warn","memory",{pressure:host.pressure});
  if(host.floorMegabytes>0){
   const data={availableMegabytes:host.availableMegabytes,floorMegabytes:host.floorMegabytes,totalMegabytes:host.totalMegabytes};
   if(host.availableMegabytes<=host.floorMegabytes)add("host.low-memory","error","memory",data);
   else if(host.availableMegabytes*100<host.floorMegabytes*(100+policy.host.lowMemoryHeadroomPercent))add("host.low-memory","warn","memory",data);
  }
  if((host.swapUsedMegabytes!==null&&host.swapUsedMegabytes>=policy.host.swapWarnMegabytes)||(host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.host.swapOutWarnMegabytesPerSecond))add("host.swap-high","warn","swap",{swapUsedMegabytes:host.swapUsedMegabytes,swapOutMegabytesPerSecond:host.swapOutMegabytesPerSecond===null?null:tenth(host.swapOutMegabytesPerSecond)});
  if(host.load!==null){
   const ratio=host.load/host.cores,data={load:tenth(host.load),cores:host.cores};
   if(ratio>=policy.host.loadPerCoreError)add("host.load-high","error","load",data);else if(ratio>=policy.host.loadPerCoreWarn)add("host.load-high","warn","load",data);
  }
  if(host.openFiles!==null&&host.openFilesMaximum!==null){
   const percent=Math.floor(100*host.openFiles/host.openFilesMaximum);
   if(percent>=policy.host.openFilesWarnPercent)add("host.open-files-high","warn","files",{openFiles:host.openFiles,openFilesMaximum:host.openFilesMaximum,percent});
  }
  if(host.attachedImages!==null){
   if(host.attachedImages>=policy.host.imagesError)add("host.images-many","error","images",{attachedImages:host.attachedImages,limit:policy.host.imagesError});
   else if(host.attachedImages>=policy.host.imagesWarn)add("host.images-many","warn","images",{attachedImages:host.attachedImages,limit:policy.host.imagesWarn});
  }
  if(host.governor==="off")add("capacity.governor-off","warn","SEMIO_HOST_CAPACITY",{});
 }

 if(storage){
  if(storage.hostFreeBytes!==null&&storage.hostTotalBytes!==null&&storage.hostTotalBytes>0){
   const data={freeBytes:storage.hostFreeBytes,totalBytes:storage.hostTotalBytes,freePercent:Math.floor(100*storage.hostFreeBytes/storage.hostTotalBytes)};
   if(storage.hostFreeBytes<policy.storage.hostFreeErrorBytes)add("storage.host-free-low","error","cache-disk",data);else if(storage.hostFreeBytes<policy.storage.hostFreeWarnBytes)add("storage.host-free-low","warn","cache-disk",data);
  }
  for(const part of storage.partitions){
   if(part.image==="damaged")add("storage.image-damaged","error",part.id,{partition:part.id});
   if(part.image==="ok"&&part.state==="absent"&&part.backing==="missing")add("storage.backing-missing","error",part.id,{partition:part.id});
   if(part.state==="detached")add("storage.image-detached","info",part.id,{partition:part.id});
   if(part.publication!=="none"){
    const age=part.publicationAgeSeconds??0;
    add("storage.publication-pending",age>=policy.storage.publicationErrorSeconds?"error":"info",part.id,{partition:part.id,which:part.publication,ageSeconds:age});
   }
   if(part.state==="mounted"&&part.freeBytes!==null&&part.capacityBytes!==null&&part.capacityBytes>0){
    const floor=policy.storage.floorBytes[part.id],percent=Math.floor(100*part.freeBytes/part.capacityBytes);
    if(floor!==undefined&&part.freeBytes<floor)add("storage.image-below-floor","error",part.id,{partition:part.id,freeBytes:part.freeBytes,floorBytes:floor});
    else if(percent<policy.storage.freePercentError)add("storage.image-low","error",part.id,{partition:part.id,freeBytes:part.freeBytes,capacityBytes:part.capacityBytes,freePercent:percent});
    else if(percent<policy.storage.freePercentWarn)add("storage.image-low","warn",part.id,{partition:part.id,freeBytes:part.freeBytes,capacityBytes:part.capacityBytes,freePercent:percent});
   }
  }
 }

 const essential=new Set(policy.tools.essential);
 for(const row of facts.tools?.rows??[]){
  if(row.state==="missing")add(essential.has(row.id)?"tools.missing-essential":"tools.missing",essential.has(row.id)?"error":"warn",row.id,{tool:row.id});
  else if(row.state==="drift")add("tools.pin-drift","warn",row.id,{tool:row.id,found:row.found,pinned:row.pin});
 }

 if(leases){
  for(const cycle of leases.cycles)add("leases.cycle","error",cycle.members.join(" → "),{members:cycle.members.join(", "),victim:cycle.victim});
  const seenHold=new Set<string>();
  for(const holder of leases.holders){
   const key=holder.pid+"\0"+holder.hash;if(seenHold.has(key)||holder.mode!=="exclusive")continue;seenHold.add(key);
   const behind=leases.waiters.filter(row=>row.hash===holder.hash&&row.pid!==holder.pid).length,data={resource:holder.resource,class:holder.class,owner:holder.owner,mode:holder.mode,pid:holder.pid,command:commandOf(holder.pid),heldSeconds:holder.heldSeconds,waiters:behind};
   if(behind>0&&holder.heldSeconds>=policy.leases.holdErrorSeconds)add("leases.hold-long","error",holder.resource,data,holder.owner);
   else if(behind>0&&holder.heldSeconds>=policy.leases.holdWarnSeconds)add("leases.hold-long","warn",holder.resource,data,holder.owner);
   else if(holder.heldSeconds>=policy.leases.holdErrorSeconds)add("leases.hold-long","warn",holder.resource,data,holder.owner);
  }
  for(const waiter of leases.waiters){
   if(waiter.waitedSeconds<policy.leases.waitWarnSeconds)continue;
   const chain=waits.find(row=>row.kind==="lease"&&row.pid===waiter.pid&&row.subject===clipResourceV1(waiter.resource)),root=chain?.root??null;
   add("leases.wait-long",waiter.waitedSeconds>=policy.leases.waitErrorSeconds?"error":"warn",waiter.resource,{resource:clipResourceV1(waiter.resource),class:waiter.class,owner:waiter.owner,mode:waiter.mode,pid:waiter.pid,command:commandOf(waiter.pid),waitedSeconds:waiter.waitedSeconds,position:waiter.position,queued:waiter.queued,rootPid:root?.pid??null,rootCommand:root?.command??null,rootHeldSeconds:root?.heldSeconds??null},waiter.owner);
  }
  const byHash=new Map<string,WaiterFactsV1[]>();
  for(const waiter of leases.waiters)byHash.set(waiter.hash,[...byHash.get(waiter.hash)??[],waiter]);
  for(const rows of byHash.values()){
   const queued=Math.max(rows.length,...rows.map(row=>row.queued??0));
   if(queued>=policy.leases.queueDepthWarn)add("leases.queue-deep","warn",rows[0]!.resource,{resource:clipResourceV1(rows[0]!.resource),class:rows[0]!.class,owner:rows[0]!.owner,queued},rows[0]!.owner);
  }
  const kinds=new Map<string,typeof leases.violations[number][]>();
  for(const row of leases.violations)kinds.set(row.kind,[...kinds.get(row.kind)??[],row]);
  for(const [kind,rows] of kinds){const last=rows[rows.length-1]!;add("leases.order-violation",kind==="out-of-order"?"info":"warn",kind,{kind,count:rows.length,held:last.held,requested:last.requested});}
  if(leases.registry.orphanFolders>0||leases.registry.deadEntries>0)add("leases.registry-stale",leases.registry.orphanFolders>=policy.leases.registryOrphanFoldersWarn?"warn":"info","lease-registry",{orphanFolders:leases.registry.orphanFolders,deadEntries:leases.registry.deadEntries,folders:leases.registry.folders});
  if(leases.deadTickets>0)add("leases.dead-ticket","info","queue",{count:leases.deadTickets});
  for(const pid of [...deadHolders].sort((left,right)=>left-right)){
   const behind=leases.waiters.filter(row=>row.blockers.includes(pid));
   if(behind.length===0)continue;
   const longest=Math.max(...behind.map(row=>row.waitedSeconds));
   add("leases.dead-holder",longest>=policy.leases.waitWarnSeconds?"warn":"info",String(pid),{pid,resource:clipResourceV1(behind[0]!.resource),waiters:behind.length,waitedSeconds:longest});
  }
 }

 if(capacity){
  for(const waiter of capacity.waiters){
   if(waiter.waitedSeconds<policy.capacity.waitWarnSeconds)continue;
   add("capacity.wait-long",waiter.waitedSeconds>=policy.capacity.waitErrorSeconds?"error":"warn",waiter.operation,{operation:waiter.operation,pid:waiter.pid,command:commandOf(waiter.pid),waitedSeconds:waiter.waitedSeconds,reason:waiter.reason,needMegabytes:waiter.needMegabytes,headroomMegabytes:capacity.headroomMegabytes,floorMegabytes:host?.floorMegabytes??0});
  }
  if(capacity.waiters.length>0)for(const grant of capacity.grants){
   if(grant.heldSeconds>=policy.capacity.idleGrantSeconds&&grant.observedMegabytes!==null&&grant.reservedMegabytes>0&&grant.observedMegabytes*100<grant.reservedMegabytes*policy.capacity.idleObservedPercent)add("capacity.grant-idle","warn",grant.operation,{operation:grant.operation,pid:grant.pid,command:commandOf(grant.pid),heldSeconds:grant.heldSeconds,reservedMegabytes:grant.reservedMegabytes,observedMegabytes:grant.observedMegabytes});
  }
 }

 const sessions=facts.sessions;
 if(sessions){
  if(sessions.gone>0)add("sessions.gone","info","agent-sessions",{count:sessions.gone});
  if(sessions.live>=policy.sessions.crowdedLive)add("sessions.crowded","warn","agent-sessions",{live:sessions.live,idle:sessions.idle});
  if(sessions.damaged>0)add("sessions.damaged","warn","agent-sessions",{count:sessions.damaged});
 }

 const graph=facts.graph;
 if(graph){
  const slowGraph=(leases?.waiters??[]).some(row=>row.owner==="graph"&&row.waitedSeconds>=policy.leases.waitWarnSeconds);
  if(graph.pointer==="absent")add("graph.unpublished",slowGraph?"error":"warn","graph",{});
  else if(graph.pointer==="damaged")add("graph.damaged","error","graph",{file:"current.json"});
  else if(graph.manifest==="missing"||graph.manifest==="damaged")add("graph.damaged","error","graph",{file:"manifest"});
  else if(graph.implementationNewerSeconds!==null&&graph.implementationNewerSeconds>0)add("graph.implementation-newer","info","graph",{generation:graph.generation,ageSeconds:graph.publishedAgeSeconds,newerSeconds:graph.implementationNewerSeconds});
 }

 const dependencies=facts.dependencies;
 if(dependencies){
  if(dependencies.pointers==="absent")add("dependencies.unpublished","error","dependencies",{});
  for(const scope of dependencies.scopes){
   if(scope.directory!=="ready")add("dependencies.generation-missing","error",scope.scope,{scope:scope.scope,generation:scope.generation.slice(0,20)});
   if(scope.inputsNewer!==null)add("dependencies.inputs-newer","warn",scope.scope,{scope:scope.scope,file:scope.inputsNewer});
  }
  if(dependencies.orphans>0)add("dependencies.orphans",dependencies.orphans>=policy.dependencies.orphanCountWarn?"warn":"info","dependencies",{count:dependencies.orphans,estimatedBytes:dependencies.orphanEstimatedBytes});
  const building=(leases?.holders??[]).some(row=>row.mode==="exclusive"&&/dependenc/iu.test(row.resource));
  if(!building&&dependencies.unready>0&&(dependencies.unreadyOldestSeconds??0)>=policy.dependencies.unreadyAgeSeconds)add("dependencies.unready-generation","warn","dependencies",{count:dependencies.unready,oldestSeconds:dependencies.unreadyOldestSeconds});
 }

 const caches=facts.caches;
 if(caches){
  if(caches.deadPublications>0){
   const declared=policy.caches.deadPublications[0]?.partition??"graph",part=storage?.partitions.find(row=>row.id===declared),floor=policy.storage.floorBytes[declared],tight=part!==undefined&&part.freeBytes!==null&&floor!==undefined&&part.freeBytes<floor;
   add("caches.dead-publication",tight?"error":caches.deadPublicationBytes>=policy.caches.deadPublicationWarnBytes?"warn":"info",declared,{count:caches.deadPublications,bytes:caches.deadPublicationBytes,partition:declared});
  }
  const shown=(names:readonly string[]):string=>names.slice(0,policy.caches.retiredNamesShown).join(", ")+(names.length>policy.caches.retiredNamesShown?", …":"");
  if(caches.retired.length>0)add("caches.retired-roots","info","cache",{count:caches.retired.length,names:shown(caches.retired)});
  if(caches.undeclared.length>0)add("caches.undeclared","info","cache",{count:caches.undeclared.length,names:shown(caches.undeclared)});
 }

 const processes=analyzeProcesses(facts,policy,commandOf,add,waiting);

 found.sort((left,right)=>SEVERITY_RANK[left.severity]-SEVERITY_RANK[right.severity]||DOCTOR_PROTOCOL_V1.areas.indexOf(left.area)-DOCTOR_PROTOCOL_V1.areas.indexOf(right.area)||(left.id<right.id?-1:left.id>right.id?1:0)||(left.subject<right.subject?-1:left.subject>right.subject?1:0));
 return Object.freeze({findings:Object.freeze(found.slice(0,DOCTOR_PROTOCOL_V1.maximumFindings)),waits:Object.freeze(waits.slice(0,DOCTOR_PROTOCOL_V1.maximumWaitRows)),processes});
}

/** 🏃️ Classifies the repo-relevant processes by what they hold or wait for, and reports orphans and holders that compute nothing. */
function analyzeProcesses(facts:DoctorFactsV1,policy:DoctorPolicyV1,commandOf:(pid:number)=>string,add:(id:string,severity:DoctorSeverityV1,subject:string,data:DoctorDataV1,owner?:string)=>void,waiting:ReadonlySet<number>):readonly DoctorProcessRowV1[]{
 const table=new Map<number,ProcessFactsV1>((facts.processes??[]).map(row=>[row.pid,row])),leases=facts.leases,capacity=facts.capacity;
 const waitingLease=new Set((leases?.waiters??[]).map(row=>row.pid)),waitingCapacity=new Set((capacity?.waiters??[]).map(row=>row.pid)),exclusive=new Map<number,string>(),holding=new Set<number>(),admitted=new Map<number,string>();
 for(const row of leases?.holders??[]){holding.add(row.pid);if(row.mode==="exclusive"&&!exclusive.has(row.pid))exclusive.set(row.pid,row.resource);}
 for(const row of capacity?.grants??[])admitted.set(row.pid,row.operation);
 const known=new Set<number>([...waitingLease,...waitingCapacity,...holding,...admitted.keys()]);
 const include=(pattern:string):RegExp=>new RegExp(pattern,"u"),patterns=policy.processes.patterns.map(include),excluded=policy.processes.exclude.map(include);
 const matches=(row:ProcessFactsV1):boolean=>commandRelevantV1(row.command,patterns,excluded);
 const ancestor=(pid:number):number|undefined=>{
  const visited=new Set<number>();let current=table.get(pid)?.parent;
  while(current!==undefined&&current>1&&!visited.has(current)){if(known.has(current))return current;visited.add(current);current=table.get(current)?.parent;}
  return undefined;
 };
 const children=new Map<number,number[]>();
 for(const row of table.values())children.set(row.parent,[...children.get(row.parent)??[],row.pid]);
 const treeCpu=(pid:number):number=>{
  let total=0;const stack=[pid],visited=new Set<number>();
  while(stack.length){const current=stack.pop()!;if(visited.has(current))continue;visited.add(current);total+=table.get(current)?.cpuPercent??0;stack.push(...children.get(current)??[]);}
  return total;
 };
 const rows:DoctorProcessRowV1[]=[];
 for(const row of table.values()){
  const via=known.has(row.pid)?undefined:ancestor(row.pid);
  if(!matches(row)&&!known.has(row.pid)&&via===undefined)continue;
  const own=waitingLease.has(row.pid)?"waiting-lease":waitingCapacity.has(row.pid)?"waiting-capacity":holding.has(row.pid)?"holding":admitted.has(row.pid)?"admitted":undefined;
  let phase:DoctorPhaseV1=own??(via!==undefined?(waiting.has(via)?"running":"admitted"):"running");
  const parentGone=row.parent<=1;
  if(own===undefined&&via===undefined&&parentGone&&row.ageSeconds>=policy.processes.orphanAgeSeconds&&matches(row))phase="orphan";
  rows.push(Object.freeze({pid:row.pid,parent:row.parent,ageSeconds:row.ageSeconds,rssMegabytes:row.rssMegabytes,cpuPercent:tenth(row.cpuPercent),phase,via:own===undefined?via??null:null,command:commandOf(row.pid)}));
  if(own!==undefined&&row.parent===1&&row.ageSeconds>=policy.processes.orphanAgeSeconds&&matches(row)){
   const holds=[...(leases?.holders??[])].filter(item=>item.pid===row.pid).length+(admitted.has(row.pid)?1:0);
   if(holds>0)add("processes.orphan","warn",String(row.pid),{pid:row.pid,ageSeconds:row.ageSeconds,command:commandOf(row.pid),holds});
  }
  if(own!==undefined&&own!=="waiting-lease"&&own!=="waiting-capacity"&&(exclusive.has(row.pid)||admitted.has(row.pid))){
   const held=Math.max(0,...(leases?.holders??[]).filter(item=>item.pid===row.pid&&item.mode==="exclusive").map(item=>item.heldSeconds),...(capacity?.grants??[]).filter(item=>item.pid===row.pid).map(item=>item.heldSeconds)),cpu=treeCpu(row.pid);
   if(held>=policy.processes.idleHolderSeconds&&cpu<policy.processes.idleCpuPercent)add("processes.idle-holder","warn",String(row.pid),{pid:row.pid,heldSeconds:held,cpuPercent:tenth(cpu),resource:exclusive.get(row.pid)??admitted.get(row.pid)??"grant",command:commandOf(row.pid)});
  }
 }
 rows.sort((left,right)=>PHASE_RANK[left.phase]-PHASE_RANK[right.phase]||right.ageSeconds-left.ageSeconds||left.pid-right.pid);
 return Object.freeze(rows.slice(0,DOCTOR_PROTOCOL_V1.maximumProcessRows));
}

/** ⚖️ Counts the findings by severity and names the verdict: any error blocks, any warning degrades. */
export function summarizeDoctorV1(findings:readonly DoctorFindingV1[]):Readonly<{verdict:DoctorVerdictV1;counts:Readonly<{error:number;warn:number;info:number}>}>{
 const counts={error:0,warn:0,info:0};for(const row of findings)counts[row.severity]++;
 return Object.freeze({verdict:counts.error>0?"blocked":counts.warn>0?"degraded":"ok",counts:Object.freeze(counts)});
}

/** 📋️ Assembles the report document from the facts, their analysis and the repairs that ran. */
export function buildDoctorReportV1(facts:DoctorFactsV1,analysis:DoctorAnalysisV1,repairs:readonly DoctorRepairResultV1[]=[]):DoctorReportV1{
 const summary=summarizeDoctorV1(analysis.findings);
 return Object.freeze({version:1,atMs:facts.atMs,platform:facts.platform,verdict:summary.verdict,counts:summary.counts,probes:facts.probes,findings:analysis.findings,waits:analysis.waits,processes:analysis.processes,repairs});
}

/** 🗂️ Names the areas a report touches, in contract order. */
export function doctorAreasOfV1(report:Pick<DoctorReportV1,"findings">):readonly DoctorAreaV1[]{
 const present=new Set(report.findings.map(row=>row.area));
 return DOCTOR_PROTOCOL_V1.areas.filter(area=>present.has(area));
}
