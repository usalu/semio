/** 🌡️ What a birth handshake measures with: the monotonic clock, the wall clock of the owner's deadline, the host's contention and the timers; every part has a real default, laws replace them with a simulated host. */
export type BazelBirthClockV1=Readonly<{now?:()=>number;epoch?:()=>number;source?:ContentionSourceV1;schedule?:(run:()=>void,milliseconds:number)=>unknown;cancel?:(handle:unknown)=>void}>;

/** ⏱️ The handshake clock of one action birth, counted as work like every other budget (`🧪️testing/🎚️budget/⏲️contention`): until the parent says it is `waiting` or that the action `started`, every wall millisecond is worth `1/stretch` of `maximumMilliseconds`, the stretch being the host's contention clamped to the policy bounds, so a busy host stretches the budget and a hung parent is still stopped after at most `stretch` times it. `waiting` (the birth is accepted and now waits for the run's grant to grow and for a job of it) ends the work clock for good: from then on only the owner's own deadline bounds the birth (no deadline: it waits as long as the parent is alive and the action is not cancelled), so a quiet grant's growth in the fair queue never fails the action, and neither `check` nor the timer can fail it. `settle` ends it as well, so a child that did its work is never cancelled by the handshake. `check` is the one poll of the pre-start steps, so no step keeps a second wall clock; `arm` starts the timer once the socket exists and the owner's deadline is known. */
export function bazelBirthHandshakeV1(maximumMilliseconds:number,started:number,options:BazelBirthClockV1={}):Readonly<{check:()=>void;arm:(deadlineEpochMilliseconds:number|null,fail:(error:unknown)=>void)=>void;waiting:()=>void;settle:()=>void;phase:()=>"birth"|"waiting"|"settled"}>{
 const now=options.now??(()=>performance.now()),epoch=options.epoch??(()=>Date.now()),source=options.source??sharedHostContentionV1(),schedule=options.schedule??((run:()=>void,milliseconds:number)=>setTimeout(run,milliseconds)),cancel=options.cancel??((handle:unknown)=>clearTimeout(handle as ReturnType<typeof setTimeout>)),clock=createBudgetClockV1(maximumMilliseconds),tick=CONTENTION_POLICY_V1.tickMilliseconds;
 let phase:"birth"|"waiting"|"settled"="birth",last=started,handle:unknown,deadline:number|null=null,failure:((error:unknown)=>void)|undefined;
 const stretch=():number=>{try{return budgetStretchV1(source().ratio);}catch{return 1;}},left=():number|undefined=>deadline===null?undefined:Math.max(1,deadline-epoch());
 const spent=():boolean=>{const at=now();if(at-started<maximumMilliseconds)return false;clock.advance(at-last,stretch());last=at;return clock.expired();};
 const untilExpiry=():number=>{const elapsed=now()-started;return elapsed<maximumMilliseconds?maximumMilliseconds-elapsed:Math.min(tick,clock.wallUntilExpiryMilliseconds(stretch()));};
 const stop=():void=>{if(handle!==undefined)cancel(handle);handle=undefined;};
 const rearm=():void=>{stop();if(phase==="settled"||!failure)return;const rest=left(),work=phase==="birth"?Math.max(1,Math.ceil(untilExpiry())):undefined,next=work===undefined?rest:rest===undefined?work:Math.min(work,rest);if(next!==undefined)handle=schedule(fire,next);};
 const fire=():void=>{handle=undefined;if(phase==="settled"||!failure)return;if(deadline!==null&&epoch()>=deadline){failure(refuseBuildV1("cancelled","bazel.birth-deadline-exhausted"));return;}if(phase==="birth"&&spent()){failure(refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted"));return;}rearm();};
 return Object.freeze({
  check:()=>{if(phase==="birth"&&spent())throw refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted");},
  arm:(until:number|null,fail:(error:unknown)=>void)=>{deadline=until;failure=fail;rearm();},
  waiting:()=>{if(phase!=="birth")return;phase="waiting";rearm();},
  settle:()=>{phase="settled";stop();},
  phase:()=>phase
 });
}
