import os,re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226"
P="🧰️framework/🔨️modules/🏃️process/🔒️leases"
def rd(p,src="base"): return open(f"{S}/{src}/{p}",encoding="utf8").read()
def wr(p,t): open(f"{S}/new/{p}","w",encoding="utf8").write(t)
def once(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old)); return t.replace(old,new)
# registry
rp=P+"/🧭️order/🗂️registry/🟦️.ts"; t=rd(rp)
t=once(t,'/** 📖️ Reads every live entry of one lease directory and removes the entries of dead or silent processes. */\nexport function readLeaseRegistryEntriesV1(root:string,directory:string,now:number=Date.now(),prune=true):readonly LeaseRegistryEntryV1[]{',
 '/** 📖️ Reads every live entry of one lease directory and removes the entries of dead or silent processes; `maximumAgeMs` is how long an entry may go without renewal before it counts as silent (a reader that wants to judge a silent holder itself passes `Infinity` and does not prune). */\nexport function readLeaseRegistryEntriesV1(root:string,directory:string,now:number=Date.now(),prune=true,maximumAgeMs:number=STALE_MILLISECONDS):readonly LeaseRegistryEntryV1[]{')
t=once(t,'now-value.updatedAtMs>STALE_MILLISECONDS){if(prune','now-value.updatedAtMs>maximumAgeMs){if(prune')
t=once(t,'holderOf(directory:string,resource:string,mode:LeaseModeV1,ownTask?:number):Readonly<{pid:number;heldSeconds:number;command?:string;self?:true}>|undefined{','holderOf(directory:string,resource:string,mode:LeaseModeV1,ownTask?:number):Readonly<{pid:number;heldSeconds:number;command?:string;self?:true;renewedSeconds?:number}>|undefined{')
t=once(t,'  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false)){\n   if(entry.pid===process.pid)continue;\n   const hold=entry.holds.find(row=>row.resource===resource&&(row.mode==="exclusive"||mode==="exclusive"));\n   if(hold)return Object.freeze({pid:entry.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-hold.sinceMs)/1000)),command:entry.command.slice(0,240)});',
 '  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false,Infinity)){\n   if(entry.pid===process.pid)continue;\n   const hold=entry.holds.find(row=>row.resource===resource&&(row.mode==="exclusive"||mode==="exclusive"));\n   if(hold)return Object.freeze({pid:entry.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-hold.sinceMs)/1000)),command:entry.command.slice(0,240),renewedSeconds:Math.max(0,Math.floor((Date.now()-entry.updatedAtMs)/1000))});')
wr(rp,t)
# order
op=P+"/🧭️order/🟦️.ts"; t=rd(op)
t=once(t,'import type {HeldLeaseV1, LeaseClassV1,','import type {HeldLeaseV1, LeaseClassV1, LeaseWaitV1,')
t=once(t,'export type {HeldLeaseV1, LeaseClassV1,','export type {HeldLeaseV1, LeaseClassV1, LeaseWaitV1,')
t=once(t,'const UNCLASSIFIED:LeaseClassV1=Object.freeze({id:"unclassified",rank:0,scope:"session",owner:"unclassified",title:Object.freeze({en:"Unclassified",de:"Nicht eingeordnet"}),match:Object.freeze([])});',
 'const UNCLASSIFIED=Object.freeze({id:"unclassified",rank:0,scope:"session",owner:"unclassified",title:Object.freeze({en:"Unclassified",de:"Nicht eingeordnet"}),match:Object.freeze([])}) as unknown as LeaseClassV1;')
t=once(t,' let factor=1;try{const value=Number(waitScale());if(Number.isFinite(value)&&value>1)factor=Math.min(16,value);}catch{}\n return Object.freeze({boundMilliseconds:Math.ceil(policy.groundWait.boundMilliseconds*factor),factor});\n}',
 ' const factor=waitFactorV1();\n return Object.freeze({boundMilliseconds:Math.ceil(policy.groundWait.boundMilliseconds*factor),factor});\n}\nconst waitFactorV1=():number=>{try{const value=Number(waitScale());return Number.isFinite(value)&&value>1?Math.min(16,value):1;}catch{return 1;}};\n'
 '/** ⏱️ The wait of one resource as its class declares it, scaled by the machine speed factor: the bound of a request that brings no timeout, the ceiling of a timeout a caller brings, the heartbeat, the silence after which a registered holder counts as stalled, and the hint of the refusal. An unclassified resource takes the policy\'s `unclassifiedWait`, so no lease wait is without a bound.\n * @see ./🧬️schema/🔣️.json `Wait` */\nexport type LeaseWaitDeclaredV1=Readonly<{classId:string;factor:number;baseBound:number;bound:number;maximum:number;heartbeat:number;silent:number;hint:Readonly<{en:string;de:string}>}>;\n'
 'export function leaseWaitOfV1(policy:LeaseOrderPolicyV1,resource:string):LeaseWaitDeclaredV1{\n const row=classifyLeaseResourceV1(policy,resource),wait=row===UNCLASSIFIED?policy.unclassifiedWait:row.wait,factor=waitFactorV1();\n return Object.freeze({classId:row.id,factor,baseBound:wait.boundMilliseconds,bound:Math.ceil(wait.boundMilliseconds*factor),maximum:Math.ceil(wait.maximumMilliseconds*factor),heartbeat:wait.heartbeatMilliseconds,silent:wait.silentMilliseconds,hint:wait.hint});\n}\n'
 '/** 🪜️ The timeout a request really gets: none brought means the class bound, one brought is kept up to the class ceiling, a try-lock (0) stays one. */\nexport function leaseEffectiveTimeoutV1(wait:LeaseWaitDeclaredV1,timeoutMs:number|undefined):number{return timeoutMs===0?0:timeoutMs===undefined?wait.bound:Math.min(timeoutMs,wait.maximum);}\n'
 '/** 🪪️ A holder the lease registry names: process, age, command and how long ago it last renewed its entry (absent for another task of this process). */\nexport type LeaseHolderV1=Readonly<{pid:number;heldSeconds:number;command?:string;self?:true;renewedSeconds?:number}>;\n'
 '/** ⚖️ The stale-holder rule of a class: `progressing` while the holder renews its entry within `silent`, `silent` when it lives but renews no longer (blocked, or its process number was reused after a restart), `unregistered` when the registry names nobody. A pure function. */\nexport function leaseHolderVerdictV1(wait:Pick<LeaseWaitDeclaredV1,"silent">,holder:LeaseHolderV1|undefined):"progressing"|"silent"|"unregistered"{return holder===undefined?"unregistered":holder.self===true||holder.renewedSeconds===undefined||holder.renewedSeconds*1000<=wait.silent?"progressing":"silent";}\n')
t=once(t,'export function leaseHolderOfV1(directory:string,resource:string,mode:LeaseModeV1):Readonly<{pid:number;heldSeconds:number;command?:string;self?:true}>|undefined{','export function leaseHolderOfV1(directory:string,resource:string,mode:LeaseModeV1):LeaseHolderV1|undefined{')
t=once(t,'export type LeaseTimeoutRowV1=Readonly<{resource:string;mode:string;seconds:number;blockerPid?:number;blockerCommand?:string;blockerHeldSeconds?:number;blockerSelf?:true}>;',
 'export type LeaseTimeoutRowV1=Readonly<{resource:string;mode:string;seconds:number;blockerPid?:number;blockerCommand?:string;blockerHeldSeconds?:number;blockerSelf?:true;declared?:Readonly<{classId:string;limitSeconds:number;baseSeconds:number;factor:number;verdict:"progressing"|"silent"|"unregistered";silentSeconds:number;hint:Readonly<{en:string;de:string}>}>}>;')
old=' return fill(catalogue.timeout[language],{resource:shortLeaseResourceV1(row.resource),mode,seconds:row.seconds,blocker:'
i=t.index(old); j=t.index('\n}',i)
seg=t[i:j]
assert seg.rstrip().endswith('boundMilliseconds:0})});'),seg[-80:]
seg2=seg.replace(' return fill(catalogue.timeout[language],',' const declared=row.declared,base=fill(catalogue.timeout[language],',1).rstrip()
assert seg2.endswith('boundMilliseconds:0})});')
seg2=seg2[:-1]+'\n return declared===undefined?base:base+fill(catalogue.timeoutDeclared[language],{class:declared.classId,limit:declared.limitSeconds,base:declared.baseSeconds,factor:declared.factor,silent:declared.silentSeconds,hint:declared.hint[language],verdict:declared.verdict==="progressing"?catalogue.verdictProgressing[language]:declared.verdict==="silent"?fill(catalogue.verdictSilent[language],{silent:declared.silentSeconds}):catalogue.verdictUnregistered[language]});'
t=t[:i]+seg2+t[j:]
t=once(t,'const taskIds=new WeakMap<object,number>();',
 '/** 💓️ The line a waiting request says every heartbeat: the holder (process, command, since when, silent when it has not renewed its entry for the class\'s `silent`), the seconds waited and the bound it states. The shape is the one the ready supervisor reads as a wait on a named holder, so a declared wait with these lines is progress to it and not a stall.\n * @see ../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json `holderNamedPatterns` */\n'
 'export function formatLeaseWaitingV1(language:"en"|"de",row:Readonly<{resource:string;mode:string;seconds:number;limitSeconds:number;wait:Pick<LeaseWaitDeclaredV1,"silent">;holder:LeaseHolderV1|undefined;nowMs:number}>):string{\n'
 ' const catalogue=schema.definitions.Messages.const,holder=row.holder,mode=row.mode==="shared"?catalogue.modeShared[language]:catalogue.modeExclusive[language],common={resource:shortLeaseResourceV1(row.resource),mode,seconds:row.seconds,limit:row.limitSeconds};\n'
 ' if(holder===undefined)return fill(catalogue.waitingUnregistered[language],common);\n'
 ' const since=new Date(row.nowMs-holder.heldSeconds*1000).toISOString();\n'
 ' if(holder.self===true)return fill(catalogue.waitingSelf[language],{...common,since});\n'
 ' const silent=leaseHolderVerdictV1(row.wait,holder)==="silent"?fill(catalogue.waitingSilent[language],{silent:holder.renewedSeconds??0}):"";\n'
 ' return fill(catalogue.waiting[language],{...common,pid:holder.pid,command:(holder.command??"?").replace(/[\\r\\n]+/gu," ").slice(0,120),since,silent});\n}\n\n'
 'const taskIds=new WeakMap<object,number>();')
wr(op,t)
