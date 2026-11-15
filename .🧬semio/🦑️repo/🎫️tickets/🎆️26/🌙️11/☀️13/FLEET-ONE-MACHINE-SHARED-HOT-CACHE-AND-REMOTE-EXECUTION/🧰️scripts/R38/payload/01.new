/** 👷 A bounded pool of slice workers for the host service: loading a graph slice (membership, facts, re-inference of the stale owners) is long synchronous work, so it runs in its own process (never a thread: a thread busy in synchronous work cannot be stopped) and the service's event loop keeps answering status and cached plans. At most `maximum` loads run at once, loads of one workspace root run one after the other (they share one fact store), a load runs under the collector target of a graph child (`graphChildHeapBytesV1`), not the larger one of the resident service it inherits: its garbage, not its data, was most of its resident size (7.7 GB resident for a live set of 0.5 GB), and a load that large leaves the capacity gate no memory for the farm workers that would shorten it; a load is cancellable (its process group gets SIGINT, then SIGKILL after 2 s, from the service main thread) and reports a heartbeat so a client sees progress.
 * @see ../../../🏛️service/🌐️host/🟨️.mjs */
import {createProcessWorkerV1,errorOfMessageV1} from "./🧵️process/🟨️.mjs";
import {GRAPH_MEMORY_POLICY_V1} from "../../../../🧠️memory/🟨️.mjs";
import {graphChildEnvironmentV1} from "../../../../🧠️memory/🐏️child-environment/🟨️.mjs";
import {hostProfileV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import {createBoundedCacheV1} from "../../../../🧊️bounded/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import {planGuardVerdictV1,sliceAdmissionV1,sliceBookingV1,sliceCeilingMegabytesV1,sliceLimitPolicyV1,slicePlanCeilingMegabytesV1} from "./🧱️limit/🟨️.mjs";
import {createPressureWatchV1,hostPressureLevelV1} from "./🌡️pressure/🟨️.mjs";
import {SLICE_JOB_VARIABLE_V1,createReaperV1,deadServiceRowV1,listProcessesV1,sliceJobTokenV1} from "./🧹️reap/🟨️.mjs";

export function createSlicePoolV1({workerUrl,maximum=2,heartbeatMilliseconds=2000,budgetMegabytes=Infinity,marginMegabytes=0,elasticity={},headroom=()=>({limitBytes:Infinity,currentBytes:process.memoryUsage().rss}),sampleMilliseconds=1000,note=()=>undefined,createWorker=(url,data,env)=>createProcessWorkerV1(url,data,env,graceMilliseconds===undefined?{}:{graceMilliseconds}),pressure=()=>createPressureWatchV1(),limiter,learned={read:()=>0,write:()=>undefined},reserve,observe,unlearnedMegabytes=0,hostMegabytes=hostProfileV1().totalMegabytes,reserveFacts=()=>({floorMegabytes:0,limitSeconds:0}),graceMilliseconds,reaper=createReaperV1(graceMilliseconds===undefined?{}:{graceMilliseconds}),planGuard=HOST_POLICY_V1.slicePlanGuard,hostPressure=()=>hostPressureLevelV1(),report=line=>{try{process.stderr.write(line+"\n");}catch{}},planLimitMegabytes=hostMegabytes>0&&hostMegabytes<GRAPH_MEMORY_POLICY_V1.serveHostMegabytes&&process.env.SEMIO_GRAPH_PLAN_LIMIT_MB!=="off"?sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}):Infinity}){
 if(!Number.isSafeInteger(maximum)||maximum<1)throw new Error("Positive worker maximum required");
 if(!Number.isFinite(marginMegabytes)||marginMegabytes<0||!(budgetMegabytes>=0))throw new Error("Finite memory margin and budget required");
 const tails=createBoundedCacheV1({name:"host:slice-roots",maxEntries:64,maxBytes:64}),waiting=[],counters={started:0,completed:0,failed:0,cancelled:0,overBudget:0,pressured:0};
 const learnedPeak=()=>Math.min(learned.read(),sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}));
 let active=0,sequence=0,recent=[];const running=new Map(),loads=new Map(),stale=deadServiceRowV1(),reaping=new Set();
 const scopeOf=load=>({pids:load.pids,match:row=>row.token===load.token||process.platform==="win32"&&running.size===0&&row.ppid===process.pid&&!load.baseline.has(row.pid)});
 const reaped=load=>{const task=reaper.reap(scopeOf(load)).catch(()=>0).finally(()=>reaping.delete(task));reaping.add(task);return task;};
 const sweepAtExit=()=>{for(const load of loads.values())reaper.killNow(scopeOf(load));reaper.killNow({match:stale});};
 process.once("exit",sweepAtExit);
 void reaper.reap({match:stale}).catch(()=>0);
 const slot=signal=>new Promise((resolve,reject)=>{
  signal?.throwIfAborted();
  const entry={resolve:()=>{signal?.removeEventListener("abort",entry.abort);resolve();},abort:()=>{const at=waiting.indexOf(entry);if(at>=0)waiting.splice(at,1);reject(signal.reason);}};
  signal?.addEventListener("abort",entry.abort,{once:true});
  if(active<maximum){active++;entry.resolve();}else waiting.push(entry);
 });
 const reserving=async(megabytes,minimumMegabytes,signal,onProgress,reduced=false)=>{
  const started=performance.now(),say=()=>{const facts=reserveFacts(),available=Math.round((headroom().availableBytes??0)/1048576),spare=Math.max(0,available-facts.floorMegabytes);note();onProgress?.(Math.round(performance.now()-started),{phase:"reservation",done:0,total:megabytes,running:[],decision:{key:reduced?"reduced":"reserving",used:megabytes,budget:reduced?available:spare,minimum:minimumMegabytes,floor:facts.floorMegabytes,limit:facts.limitSeconds}});},beat=setInterval(say,heartbeatMilliseconds);beat.unref?.();
  if(reduced)say();
  try{return await reserve({megabytes,minimumMegabytes,signal});}finally{clearInterval(beat);}
 };
 const release=()=>{const next=waiting.shift();if(next)next.resolve();else active--;};
 const runWorker=(request,signal,onProgress,limit,reservation,booked,waveBasisPoints=10000)=>new Promise((resolve,reject)=>{
  counters.started++;const index=++sequence,token=sliceJobTokenV1(process.pid,index),load={token,pids:new Set(),baseline:new Set(process.platform==="win32"?listProcessesV1().filter(row=>row.ppid===process.pid).map(row=>row.pid):[])},worker=createWorker(workerUrl,{...request,heartbeatMilliseconds},{...process.env,SEMIO_SLICE_WAVE_BASIS_POINTS:String(waveBasisPoints),...graphChildEnvironmentV1(),...reservation?.megabytes>0&&reservation.megabytes<booked?{[GRAPH_MEMORY_POLICY_V1.budgetEnvironment]:String(Math.max(GRAPH_MEMORY_POLICY_V1.minimumBudgetMegabytes,Math.floor(reservation.megabytes*GRAPH_MEMORY_POLICY_V1.grantShare)))}:{},[HOST_POLICY_V1.memoryPurge.environmentVariable]:String(HOST_POLICY_V1.memoryPurge.delayMilliseconds),[SLICE_JOB_VARIABLE_V1]:token}),started=performance.now(),entry={index,sample:undefined};running.set(index,entry);loads.set(token,load);
  let settled=false;
  const finish=(done,value)=>{if(settled)return;settled=true;if(entry.peakMegabytes&&(done===resolve||value?.code==='memory-budget'))learned.write(Math.round(entry.peakMegabytes*(value?.code==='memory-budget'?(elasticity.learnedGrowthBasisPoints??10000)/10000:1)));running.delete(index);if(entry.sample!==undefined)recent=[{index,sample:entry.sample},...recent].slice(0,maximum);clearInterval(beat);clearInterval(guard);signal?.removeEventListener("abort",abort);void Promise.resolve(worker.terminate?.()).catch(()=>undefined).then(()=>reaped(load)).then(()=>limit?.release()).catch(()=>undefined).finally(()=>{loads.delete(token);reservation?.release();});done(value);};
  const abort=()=>{counters.cancelled++;finish(reject,signal.reason??new Error("cancelled"));};
  let settling;
  const ceilingMegabytes=hostMegabytes>0?Math.max(Math.floor(hostMegabytes*planGuard.ceilingBasisPoints/10000),planLimitMegabytes):Infinity;
  const judge=megabytes=>{
   if(settled)return;
   const answered=settling!==undefined&&(settling.answered||performance.now()-settling.at>=planGuard.settleWaitMilliseconds),stage=entry.stage??detail?.phase??"?";
   if(settling!==undefined&&!answered&&megabytes<=ceilingMegabytes)return;
   if(answered&&!settling.said){settling.said=true;const lines=renderBothV1("host-graph-plan-settled",{stage,from:settling.from,to:Math.round(megabytes)});for(const language of ["en","de"])report(lines[language]);}
   const available=headroom().availableBytes,availableMegabytes=available===undefined?undefined:Math.round(available/1048576),done=worker.send===undefined||answered,over=megabytes>planLimitMegabytes&&megabytes<=ceilingMegabytes&&done;
   const verdict=planGuardVerdictV1({treeMegabytes:megabytes,limitMegabytes:planLimitMegabytes,ceilingMegabytes,availableMegabytes,reserveMegabytes:planGuard.reserveMegabytes,pressure:over?hostPressure():undefined,settled:done});
   if(verdict.action==="settle"){settling={at:performance.now(),from:Math.round(megabytes),answered:false,said:false};if(!worker.send({type:"settle"}))settling.answered=true;return;}
   if(verdict.action==="refuse"){counters.overBudget++;finish(reject,Object.assign(new Error("Slice load grew past the memory a plan may take"),{code:"plan-memory",reason:verdict.reason,stage,operation:entry.operation??detail?.phase??"?",currentMegabytes:Math.round(megabytes),limitMegabytes:Math.round(planLimitMegabytes),availableMegabytes,reserveMegabytes:planGuard.reserveMegabytes,sample:entry.sample}));}
  };
  let detail;const watch=pressure(),base=headroom().currentBytes,beat=setInterval(()=>{note();onProgress?.(Math.round(performance.now()-started),detail);},heartbeatMilliseconds),guard=setInterval(()=>{if(worker.pid!==undefined&&observe!==undefined)void observe(worker.pid).then(megabytes=>{if(megabytes>(entry.peakMegabytes??0))entry.peakMegabytes=megabytes;judge(megabytes);},()=>undefined);const verdict=watch.sample();if(verdict.tripped){counters.pressured++;finish(reject,Object.assign(new Error("Slice load stalled by memory pressure"),{code:"memory-pressure",kind:verdict.kind,value:verdict.value,limit:verdict.limit,seconds:verdict.seconds}));return;}const{limitBytes,currentBytes}=headroom(),grown=Math.round((currentBytes-base)/1048576),current=Math.round(currentBytes/1048576),peak=(entry.peakMegabytes=Math.max(entry.peakMegabytes??0,grown)),limit=limitBytes===Infinity?Infinity:Math.round(limitBytes/1048576);if(grown>budgetMegabytes||current+marginMegabytes>limit){counters.overBudget++;finish(reject,Object.assign(new Error("Slice load would cross the memory limit"),{code:"memory-budget",currentMegabytes:current,limitMegabytes:limit===Infinity?budgetMegabytes:limit,marginMegabytes,grownMegabytes:grown}));}},sampleMilliseconds);beat.unref?.();guard.unref?.();
  signal?.addEventListener("abort",abort,{once:true});
  if(reservation&&worker.pid!==undefined)reservation.attach(worker.pid);
  if(limit){void limit.attach(worker.pid).catch(()=>undefined);limit.onBreach(breach=>{counters.overBudget++;entry.peakMegabytes=Math.max(entry.peakMegabytes??0,breach.limitMegabytes);finish(reject,Object.assign(new Error("Slice load overran its memory limit"),{code:"memory-budget",currentMegabytes:breach.peakMegabytes,limitMegabytes:breach.limitMegabytes,marginMegabytes,grownMegabytes:breach.peakMegabytes}));});}
  worker.on("message",message=>{
   note();if(message?.type==="child"&&Number.isSafeInteger(message.pid)){load.pids.add(message.pid);return;}
   if(message?.type==="progress"){detail={phase:message.phase,done:message.done,total:message.total,...message.running===undefined?{}:{running:message.running,...message.decision===undefined?{}:{decision:message.decision}}};return;}
   if(message?.type==="settled"){if(settling!==undefined)settling.answered=true;return;}
   if(message?.type==="stage"){entry.stage=String(message.stage);entry.operation=String(message.operation??message.stage);return;}
   if(message?.type==="memory"){entry.sample={heapMb:message.heapMb,extraMb:message.extraMb,objectCount:message.objectCount,objectTypes:message.objectTypes,at:Date.now()};return;}
   if(message?.type==="done"){counters.completed++;finish(resolve,{graph:message.graph,...message.generationPin===undefined?{}:{generationPin:message.generationPin},...message.stats===undefined?{}:{stats:message.stats}});}
   else if(message?.type==="error"){counters.failed++;finish(reject,errorOfMessageV1(message));}
  });
  worker.on("error",error=>{counters.failed++;finish(reject,error);});
  worker.on("exit",code=>{if(!settled){counters.failed++;finish(reject,Object.assign(new Error("Slice worker exited with code "+code),{exit:code,restart:code===HOST_POLICY_V1.sliceRestartExitCode}));}});
 });
 return Object.freeze({
  async run(request,{signal,onProgress}={}){
   const key=String(request.root),previous=tails.get(key)??Promise.resolve(),turn=previous.catch(()=>undefined).then(async()=>{
    await slot(signal);
    try{for(let restarts=0;;restarts++){
    let reservation;
    try{
     const room=headroom(),policy=limiter===undefined?undefined:sliceLimitPolicyV1(room,marginMegabytes,{...elasticity,hostMegabytes,planCeilingMegabytes:slicePlanCeilingMegabytesV1({hostMegabytes,ceilingBasisPoints:planGuard.ceilingBasisPoints,planLimitMegabytes})},learnedPeak());
     const{wantedMegabytes:booked,minimumMegabytes}=sliceBookingV1({declaredMegabytes:policy?.memoryMegabytes,learnedMegabytes:learnedPeak(),unlearnedMegabytes,maximumMegabytes:elasticity.maximumMegabytes,hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,minimumMegabytes:elasticity.minimumMegabytes,serviceLeftMegabytes:policy===undefined&&Number.isFinite(room.limitBytes)?Math.floor((room.limitBytes-room.currentBytes)/1048576)-marginMegabytes:Infinity});
     const available=headroom().availableBytes,admission=minimumMegabytes>0||elasticity.hostShareBasisPoints===undefined?{megabytes:booked,reduced:false,waveBasisPoints:10000}:sliceAdmissionV1({requestedMegabytes:booked,availableMegabytes:available===undefined?undefined:available/1048576,floorMegabytes:elasticity.hostFloorMegabytes,shareBasisPoints:elasticity.hostShareBasisPoints});
     reservation=reserve===undefined||!(booked>0)?undefined:await reserving(admission.megabytes,Math.min(minimumMegabytes,admission.megabytes),signal,onProgress,admission.reduced);
     const limit=policy===undefined?undefined:await limiter.create(policy,"slice-"+(sequence+1)).catch(()=>undefined);
     const owned=reservation,granted=Number(owned?.megabytes),waveBasisPoints=Number.isFinite(granted)&&granted>0?Math.min(admission.waveBasisPoints,Math.max(500,Math.floor(granted*10000/booked))):admission.waveBasisPoints;reservation=undefined;
     return await runWorker(request,signal,onProgress,limit,owned,booked,waveBasisPoints);
    }catch(error){
     reservation?.release();
     if(error?.restart!==true||signal?.aborted)throw error;
     if(restarts>=HOST_POLICY_V1.sliceRestartMaximum)throw Object.assign(new Error(Object.values(renderBothV1("host-slice-restarts",{restarts:restarts+1,exit:error.exit})).join("\n")),{code:"slice-restarts",restarts:restarts+1,exit:error.exit});
     note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"restarted",used:restarts+1,budget:HOST_POLICY_V1.sliceRestartMaximum,kind:String(error.exit)}});
    }
    }}finally{release();}
   });
   tails.set(key,turn.catch(()=>undefined));
   return turn;
  },
  stats(){return{active,queued:waiting.length,...counters};},
  memory(now=Date.now()){const row=(index,sample,live)=>({index,heapMb:sample.heapMb,extraMb:sample.extraMb,objectCount:sample.objectCount,objectTypes:sample.objectTypes,ageMilliseconds:Math.max(0,now-sample.at),running:live}),live=[...running.values()].filter(entry=>entry.sample!==undefined).map(entry=>row(entry.index,entry.sample,true));return[...live,...recent.filter(entry=>!running.has(entry.index)).map(entry=>row(entry.index,entry.sample,false))].slice(0,maximum);},
  busy(){return active+waiting.length;},
  /** 🛑 Waits for the reaping of every finished load and kills what a load left behind. */
  async close(){await Promise.all([...reaping]);sweepAtExit();process.off("exit",sweepAtExit);},
 });
}
