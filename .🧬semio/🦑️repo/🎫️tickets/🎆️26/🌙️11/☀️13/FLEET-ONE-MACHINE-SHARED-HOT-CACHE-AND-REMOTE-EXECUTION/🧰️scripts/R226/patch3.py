S="/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226"
P="🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts"
t=open(f"{S}/base/{P}",encoding="utf8").read()
def once(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old)); return t.replace(old,new)
t=once(t,'leaseHolderOfV1,formatLeaseTimeoutV1,type LeaseOrderGuardV1}','leaseHolderOfV1,formatLeaseTimeoutV1,formatLeaseWaitingV1,leaseHolderVerdictV1,leaseWaitOfV1,leaseEffectiveTimeoutV1,type LeaseWaitDeclaredV1,type LeaseOrderGuardV1}')
t=once(t,'import { createHash, randomBytes } from "node:crypto";','import {noticeLanguagesV1} from "../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";\nimport { createHash, randomBytes } from "node:crypto";')
t=once(t,'  readonly timeoutMs?: number;\n  readonly onWait?','  readonly timeoutMs?: number;\n  /** 🥁️ Set by the lease layer when `timeoutMs` is the bound its class declares and not one the caller brought, so the self-deadlock rule still treats the request as unbounded. */\n  readonly waitDeclared?: true;\n  /** 🫀️ Set by the lease layer on a request whose heartbeat already runs, so a nested acquisition of the same wait does not say it twice. */\n  readonly waitBeating?: true;\n  readonly onWait?')
old='''      const row = {resource: this.resource, mode: this.mode, seconds, ...holder === undefined ? {} : {blockerPid: holder.pid, blockerHeldSeconds: holder.heldSeconds, ...holder.command === undefined ? {} : {blockerCommand: holder.command}, ...holder.self === true ? {blockerSelf: true as const} : {}}};'''
new='''      let declared;
      try {const wait = leaseWaitOfV1(leaseOrderPolicyV1(), this.resource); declared = {classId: wait.classId, limitSeconds: Math.round(wait.bound / 1000), baseSeconds: Math.round(wait.baseBound / 1000), factor: wait.factor, verdict: leaseHolderVerdictV1(wait, holder), silentSeconds: holder?.renewedSeconds ?? 0, hint: wait.hint};} catch {}
      const row = {resource: this.resource, mode: this.mode, seconds, ...declared === undefined ? {} : {declared}, ...holder === undefined ? {} : {blockerPid: holder.pid, blockerHeldSeconds: holder.heldSeconds, ...holder.command === undefined ? {} : {blockerCommand: holder.command}, ...holder.self === true ? {blockerSelf: true as const} : {}}};'''
t=once(t,old,new)
t=once(t,'nonblocking:options.timeoutMs===0,bounded:options.timeoutMs!==undefined,','nonblocking:options.timeoutMs===0,bounded:options.timeoutMs!==undefined&&options.waitDeclared!==true,')
old=t[t.index('/** 📼️ Appends one row') if '/** 📼️ Appends one row' in t else t.index('/** 📼 Appends one row'):t.index('/** 🔤️ Admission guards')]
new='''/** 🕐️ Gives a request the wait its lease class declares: one without a timeout gets the class bound, one with a timeout keeps it up to the class ceiling, a try-lock (timeout 0) stays a try-lock; so no lease wait is unbounded. The request that waits also says every class heartbeat who it waits for (see `formatLeaseWaitingV1`) until it ends, whatever way it ends, so no wait is silent. A refusal at the bound is the typed `ResourceLeaseTimeoutErrorV1`.
 * @see ./🧭️order/🔣️policy.json the classes' `wait` */
function declareLeaseWaitV1<T extends LeaseOptions>(options:T):Readonly<{options:T;heartbeat:(started:number)=>()=>void}>{
  if(options.timeoutMs===0||options.waitBeating===true||typeof options.resource!=="string"||!options.resource.length||options.timeoutMs!==undefined&&!Number.isFinite(options.timeoutMs))return Object.freeze({options,heartbeat:()=>()=>{}});
  const wait=leaseWaitOfV1(leaseOrderPolicyV1(),options.resource),timeoutMs=leaseEffectiveTimeoutV1(wait,options.timeoutMs),declared:T=Object.freeze({...options,timeoutMs,waitBeating:true as const,...options.timeoutMs===undefined?{waitDeclared:true as const}:{}});
  return Object.freeze({options:declared,heartbeat:(started:number)=>leaseHeartbeatV1(declared,wait,started)});
}
function leaseHeartbeatV1(options:LeaseOptions,wait:LeaseWaitDeclaredV1,started:number):()=>void{
  let timer:ReturnType<typeof setTimeout>|undefined,stopped=false;
  const beat=():void=>{
    if(stopped)return;
    const now=Date.now();
    try{const holder=leaseHolderOfV1(resolve(options.directory),options.resource,options.mode),row={resource:options.resource,mode:options.mode,seconds:Math.round((now-started)/1000),limitSeconds:Math.round((options.timeoutMs??wait.bound)/1000),wait,holder,nowMs:now};process.stderr.write(noticeLanguagesV1(["en","de"] as const).map(language=>formatLeaseWaitingV1(language,row)).join("\\n")+"\\n");}catch{}
    if(!stopped){timer=setTimeout(beat,wait.heartbeat);timer.unref();}
  };
  timer=setTimeout(beat,wait.heartbeat);timer.unref();
  return()=>{stopped=true;if(timer!==undefined)clearTimeout(timer);};
}
/** 📼 Appends one row `{version,resource,mode,startedMs,acquiredMs,pid}` to the file named by `SEMIO_LEASE_TRACE` for every acquisition that waited at least 100 ms, so a harness can measure lock waits exactly instead of from the 15 s progress rows; never fails an acquisition. */
async function traceLeaseV1<T extends LeaseOptions>(request:T,acquire:(options:T)=>Promise<ResourceLease>):Promise<ResourceLease>{
  const declared=declareLeaseWaitV1(request),options=declared.options,stop=declared.heartbeat(Date.now());
  try{
    const path=process.env.SEMIO_LEASE_TRACE;
    if(!path)return admitResourceLeaseV1(options,await acquire(options));
    const startedMs=Date.now(),lease=await acquire(options),acquiredMs=Date.now();
    if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\\n");}catch{}
    return admitResourceLeaseV1(options,lease);
  }finally{stop();}
}
'''
print(old[:80])
t=t.replace(old,new,1)
t=once(t,'return traceLeaseV1(options,()=>acquireOrderedResourceLeaseV1(options));}','return traceLeaseV1(options,declared=>acquireOrderedResourceLeaseV1(declared));}')
t=once(t,'return traceLeaseV1(options,()=>acquireOrderedQueuedResourceLeaseV1(options));}','return traceLeaseV1(options,declared=>acquireOrderedQueuedResourceLeaseV1(declared));}')
old='const{sharedCohort:_cohort,timeoutMs:_timeout,...base}=turn.options,options:QueuedLeaseOptions=Object.freeze({...base,mode:"exclusive",...(limits.timeoutMs===undefined?{}:{timeoutMs:limits.timeoutMs})});'
new='const{sharedCohort:_cohort,timeoutMs:_timeout,waitDeclared:_declared,waitBeating:_beating,...base}=turn.options,exclusive:QueuedLeaseOptions=Object.freeze({...base,mode:"exclusive",...(limits.timeoutMs===undefined?{}:{timeoutMs:limits.timeoutMs})}),declared=declareLeaseWaitV1(exclusive),options:QueuedLeaseOptions=declared.options;'
t=once(t,old,new)
t=once(t,'  const started=Date.now();remaining(options,started);forgetLeaseV1(lease);const guard=orderGuard(options,true);\n  const name=join(turn.queue,turn.ticket)','  const started=Date.now();remaining(options,started);forgetLeaseV1(lease);const guard=orderGuard(options,true),stop=declared.heartbeat(started);\n  const name=join(turn.queue,turn.ticket)')
t=once(t,'try{upgraded=await awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim,guard);}catch(error){guard?.finish();throw error;}','try{upgraded=await awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim,guard);}catch(error){guard?.finish();throw error;}finally{stop();}')
open(f"{S}/new/{P}","w",encoding="utf8").write(t)
