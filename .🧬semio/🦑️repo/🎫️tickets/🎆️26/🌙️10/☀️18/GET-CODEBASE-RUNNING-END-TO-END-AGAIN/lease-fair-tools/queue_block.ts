/** ⏳️ Closed poll, sweep and progress cadence declared by `⏳️wait/🧬️schema/📜️policy.json`. */
const waitPolicy=waitPolicySchema.definitions.Policy.const;
type QueuedTurn={readonly options:QueuedLeaseOptions;readonly queue:string;readonly ticket:string;readonly claim:ResourceLease;inner?:ResourceLease;consumed:boolean};
const queuedTurns=new WeakMap<ResourceLease,QueuedTurn>();
type QueueWaitInfo={version:1;position:number;queued:number;holderMode:"shared"|"exclusive";holderPid:number;holderSeconds:number;resource:string};

function queueTicketRow(queue:string,name:string):{pid:number;mode:"shared"|"exclusive"}|undefined{
  let row:Record<string,unknown>|undefined;try{row=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch{return undefined;}
  if(row===null||typeof row!=="object"||Object.keys(row).sort().join(",")!=="mode,pid,sharedCohort,version"||row.version!==1||row.pid!==queueTicketRequest(name).pid||(row.mode!=="shared"&&row.mode!=="exclusive")||typeof row.sharedCohort!=="boolean")return undefined;
  return{pid:row.pid as number,mode:row.mode};
}
function queueWaitInfo(options:QueuedLeaseOptions,queue:string,live:readonly string[],ticket:string,queued:number):QueueWaitInfo|undefined{
  const position=live.indexOf(ticket)+1;if(position<2)return undefined;
  const head=live[0]!,row=queueTicketRow(queue,head);if(!row)return undefined;
  const limit=waitPolicy.maximumReportedResourceCharacters,resource=options.resource.length>limit?"…"+options.resource.slice(-(limit-1)):options.resource;
  return{version:1,position,queued:Math.max(queued,position),holderMode:row.mode,holderPid:row.pid,holderSeconds:Math.max(0,Math.floor((Date.now()-arrival(head))/1000)),resource};
}
/** 🧹 Retires one predecessor ticket only when its independent kernel claim proves its requester gone. */
async function reapQueueTicket(options:QueuedLeaseOptions,queue:string,name:string,pendingOptions:()=>LeaseOptions):Promise<boolean>{
  const sweepCatalog=await queueClaimCatalog(pendingOptions(),"shared");let retired:ResourceLease|undefined;
  try{retired=await acquireResourceLease({...resourceQueueClaimV1(options.directory,options.resource,name),mode:"exclusive",signal:options.signal,control:options.control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});rmSync(join(queue,name),{force:true});return true;}
  catch(error){if(!String(error).includes("Resource lease timed out"))throw error;return false;}
  finally{retired?.release();sweepCatalog.release();}
}
function wrapQueuedLease(lease:ResourceLease,turn:QueuedTurn):ResourceLease{
  const queued:ResourceLease={resource:lease.resource,mode:lease.mode,release(){
    if(turn.consumed)return;
    try{lease.release();}finally{try{rmSync(join(turn.queue,turn.ticket),{force:true});}finally{turn.claim.release();}}
  }};
  originalLeases.set(queued,originalLeases.get(lease)!);queuedTurns.set(queued,turn);turn.inner=lease;return queued;
}

/** 🚦️ Serves original kernel-held tickets in arrival order; an exclusive SQLite claim alone proves abandoned custody. */
export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {
  options=Object.freeze({...options});
  options.signal.throwIfAborted();
  if("claimCatalogDirectory" in options)throw Error("Queue claim catalog selectors are forbidden");
  if(options.sharedCohort!==undefined&&(typeof options.sharedCohort!=="boolean"||options.sharedCohort&&options.mode!=="shared"))throw Error("Shared lease cohorts require shared access");
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw new Error("Invalid lease queue owner");
  const started = Date.now(); remaining(options, started);
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  if(options.control)await retireResourceQueueClaimsV1(options.directory,options.control,0);
  remaining(options,started);
  const ticket = resourceQueueTicketV1({arrivedAtMs:started,pid:process.pid,owner:options.owner});
  const admitted = resourceLeasePathV1(options.directory, options.resource, ticket);
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  if (queue !== admitted.replace(/\.sqlite$/u, ".queue")) throw Error("Resource queue materialized path changed");
  mkdirSync(queue, { recursive: true });
  const catalog=await queueClaimCatalog(pendingOptions(),"shared");let claim:ResourceLease;
  try{claim=await acquireResourceLease({...pendingOptions(),...resourceQueueClaimV1(options.directory,options.resource,ticket),mode:"shared",maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});try{remaining(options,started);writeFileSync(join(queue,ticket),JSON.stringify({version:1,pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true}),{flag:"wx"});}catch(error){claim.release();throw error;}}finally{catalog.release();}
  return awaitQueuedTurn(options,started,queue,ticket,claim);
}

/** ⬆️ Converts a held queued shared lease into an exclusive one at its original arrival position; the shared hold is consumed, so a failed upgrade holds nothing and concurrent upgraders cannot deadlock each other. */
export async function upgradeQueuedResourceLease(lease:ResourceLease,limits:{readonly timeoutMs?:number}={}):Promise<ResourceLease>{
  const turn=queuedTurns.get(lease),issued=originalLeases.get(lease);
  if(!turn||turn.consumed||!turn.inner||lease.mode!=="shared"||issued?.phase!=="held")throw Error("Live queued shared lease required for upgrade");
  if(limits.timeoutMs!==undefined&&(!Number.isFinite(limits.timeoutMs)||limits.timeoutMs<0))throw Error("Invalid resource lease timeout");
  const{sharedCohort:_cohort,timeoutMs:_timeout,...base}=turn.options,options:QueuedLeaseOptions=Object.freeze({...base,mode:"exclusive",...(limits.timeoutMs===undefined?{}:{timeoutMs:limits.timeoutMs})});
  const started=Date.now();remaining(options,started);
  const name=join(turn.queue,turn.ticket),temporary=join(dirname(turn.queue),turn.ticket+".u");
  try{writeFileSync(temporary,JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false}),{flag:"wx"});renameSync(temporary,name);}catch(error){rmSync(temporary,{force:true});throw error;}
  turn.consumed=true;
  try{turn.inner.release();}catch(error){try{rmSync(name,{force:true});}finally{turn.claim.release();}throw error;}
  return awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim);
}

async function awaitQueuedTurn(options:QueuedLeaseOptions,started:number,queue:string,ticket:string,claim:ResourceLease):Promise<ResourceLease>{
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  const swept=new Map<string,number>();let nextProgress=0,poll=waitPolicy.pollInitialMilliseconds;
  try {
    for (;;) {
      if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      const names=(await retryInterruptedFilesystemRead(options.signal,()=>readdirSync(queue),options.control)).sort((left,right)=>arrival(left)-arrival(right)||left.localeCompare(right,"en")),own=names.indexOf(ticket),live=own<0?[...names]:names.slice(0,own);
      while(live.length){
        const name=live[live.length-1]!,at=Date.now(),last=swept.get(name);
        if(last!==undefined&&at-last<waitPolicy.sweepIntervalMilliseconds)break;
        swept.set(name,at);if(!await reapQueueTicket(options,queue,name,pendingOptions))break;
        swept.delete(name);live.pop();
      }
      if(own>=0)live.push(ticket);
      remaining(options, started);
      const cohort=options.sharedCohort===true&&await retryInterruptedFilesystemRead(options.signal,()=>live.slice(0,live.indexOf(ticket)).every(name=>{let claim:unknown;try{claim=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return true;if(error instanceof SyntaxError)return false;throw error;}const row=claim as Record<string,unknown>|undefined;return row!==null&&typeof row==="object"&&Object.keys(row).sort().join(",")==="mode,pid,sharedCohort,version"&&row.version===1&&row.pid===queueTicketRequest(name).pid&&row.mode==="shared"&&row.sharedCohort===true;}),options.control);
      remaining(options, started);
      if (live[0] === ticket || cohort) {
        const lease = await acquireResourceLease({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        try { remaining(options, started); } catch (error) { lease.release(); throw error; }
        return wrapQueuedLease(lease,{options,queue,ticket,claim,consumed:false});
      }
      const elapsedMs = Date.now() - started;
      if (remaining(options, started) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info=options.control?queueWaitInfo(options,queue,live,ticket,names.length):undefined,control=options.control;
        await waitProgress(options, progress, started);
        if(info&&control)await waitForLeaseContinuation(options,started,()=>control.publish({resource:"lease-queue:"+JSON.stringify(info),mode:options.mode,elapsedMs:0}));
        nextProgress = elapsedMs + waitPolicy.progressIntervalMilliseconds;
      }
      await delay(Math.max(1, Math.min(poll, remaining(options, started))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
      poll=Math.min(waitPolicy.pollMaximumMilliseconds,Math.ceil(poll*waitPolicy.pollGrowthNumerator/waitPolicy.pollGrowthDenominator));
      remaining(options, started);
    }
  } catch (error) {
    try{rmSync(join(queue, ticket), { force: true });}finally{claim.release();}
    throw error;
  }
}

