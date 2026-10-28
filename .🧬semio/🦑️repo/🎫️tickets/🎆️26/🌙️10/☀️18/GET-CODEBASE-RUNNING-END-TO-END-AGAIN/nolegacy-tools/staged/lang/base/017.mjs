/** ⏱️ Times and reports the phases of one graph construction: a phase has a total, advances monotonically, publishes its counter at most every `minimumMilliseconds` (first, last and every completed state included) and leaves one timing row; a phase also names the work currently running in it (at most `maximumNames` oldest first, how many run, how long the oldest has run in steps of `secondsStep`), published under the same throttle so a long single item is never anonymous; the clock owns no I/O, callers inject the clock, the progress publisher and the row writer. */
export function createGraphPhaseClockV1({publish,write,now=()=>performance.now(),minimumMilliseconds=250,maximumNames=3,secondsStep=5,maximumResourceCharacters=480,maximumNameCharacters=96}={}){
 const phases=new Map(),rows=[];
 const resourceOf=(phase,state,done)=>"build:graph-phase:"+phase+":"+done+":"+state.total+state.detail;
 const emit=(phase,state,done)=>{state.lastAt=now();state.lastDone=done;state.lastResource=resourceOf(phase,state,done);return publish?.(state.lastResource);};
 const detailOf=(phase,total,running)=>{
  if(running.length===0)return "";
  const base=("build:graph-phase:"+phase+":"+total+":"+total).length,seconds=Math.floor(Math.max(0,running[0].seconds)/secondsStep)*secondsStep,names=[];
  let size=base+(":"+running.length+":"+seconds+":").length;
  for(const row of running.slice(0,maximumNames)){const text=encodeURIComponent(String(row.name).slice(0,maximumNameCharacters)),extra=text.length+(names.length?1:0);if(size+extra>maximumResourceCharacters)break;names.push(text);size+=extra;}
  return ":"+running.length+":"+seconds+":"+names.join(",");
 };
 return Object.freeze({
  begin(phase,total){
   if(typeof phase!=="string"||!/^[a-z]+$/u.test(phase)||!Number.isSafeInteger(total)||total<0||phases.has(phase))throw Error("Closed graph phase and finite total required");
   const state={total,done:0,startedAt:now(),lastAt:-Infinity,lastDone:-1,detail:"",lastResource:""};phases.set(phase,state);
   return emit(phase,state,0);
  },
  advance(phase,done){
   const state=phases.get(phase);if(!state)throw Error("Begun graph phase required");
   if(!Number.isSafeInteger(done)||done<state.done)throw Error("Monotonic graph phase progress required");
   state.done=Math.min(done,state.total);
   if(state.done===state.lastDone&&resourceOf(phase,state,state.done)===state.lastResource||state.done!==state.total&&now()-state.lastAt<minimumMilliseconds)return undefined;
   return emit(phase,state,state.done);
  },
  running(phase,running){
   const state=phases.get(phase);if(!state)throw Error("Begun graph phase required");
   if(!Array.isArray(running))throw Error("Running work list required");
   state.detail=detailOf(phase,state.total,running);
   if(resourceOf(phase,state,state.done)===state.lastResource||now()-state.lastAt<minimumMilliseconds)return undefined;
   return emit(phase,state,state.done);
  },
  async end(phase,details={}){
   const state=phases.get(phase);if(!state)throw Error("Begun graph phase required");
   state.detail="";
   if(state.lastDone!==state.done||resourceOf(phase,state,state.done)!==state.lastResource)await emit(phase,state,state.done);
   phases.delete(phase);
   const row=Object.freeze({phase,milliseconds:Math.round(now()-state.startedAt),done:state.done,total:state.total,...details});
   rows.push(row);write?.(row);return row;
  },
  rows:()=>rows.slice(),
 });
}

/** 🖨️ Writes one machine readable timing row to stderr when the profile switch is on. */
export function writeGraphPhaseRowV1(row){
 if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"||process.env.SEMIO_GRAPH_PHASES==="1")process.stderr.write("[graph-phase] "+JSON.stringify({...row,rssMegabytes:Math.round(process.memoryUsage().rss/1048576)})+"\n");
}

/** 🔁️ Writes one machine readable row per repeated settlement round: the round, the first conflicting source or the failure (message, code, project) that made the whole inference start over; never silent, at most one row per round. */
export function writeGraphSettleRowV1(round){
 const text=value=>{try{return String(value?.message??value).slice(0,512);}catch{return"";}};
 process.stderr.write("[graph-settle] "+JSON.stringify({round:round.round,...(round.conflict===undefined?{}:{conflict:round.conflict}),...(Object.hasOwn(round,"failure")?{failure:text(round.failure),...(round.failure?.code?{code:String(round.failure.code)}:{}),...(round.failure?.semioProject?{project:String(round.failure.semioProject)}:{})}:{})})+"\n");
}

/** 🖨️ Writes the summary of a finished construction: always one line, so a slow cold build explains itself. */
export function writeGraphPhaseSummaryV1(rows){
 process.stderr.write("[graph-phase] "+JSON.stringify({summary:rows.map(row=>[row.phase,row.milliseconds])})+"\n");
}
