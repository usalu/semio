/** 🚦 Capacity governor: bounded concurrency, bounded queue, round-robin fairness across lanes, cancellation while queued. */

/** 🚨 Raised when the queue is full. */
export class BusyErrorV1 extends Error{constructor(inFlight,queued){super("busy");this.name="BusyErrorV1";this.code="busy";this.inFlight=inFlight;this.queued=queued;}}

/** 🚦 Creates the governor; `enter` resolves with a release function once a slot is free. */
export function createGovernorV1({maximumInFlight,maximumQueued}){
 const lanes=new Map(),ring=[];
 let inFlight=0,queued=0,cursor=0;
 const grant=()=>{
  while(inFlight<maximumInFlight&&queued>0){
   let picked;
   for(let step=0;step<ring.length;step++){
    const lane=ring[(cursor+step)%ring.length],queue=lanes.get(lane);
    if(queue?.length){picked=queue.shift();cursor=(cursor+step+1)%Math.max(ring.length,1);break;}
   }
   if(!picked)return;
   queued--;inFlight++;
   picked.resolve(once(()=>{inFlight--;grant();}));
  }
  for(const lane of[...ring])if(!lanes.get(lane)?.length){lanes.delete(lane);ring.splice(ring.indexOf(lane),1);}
  cursor=ring.length?cursor%ring.length:0;
  for(const lane of ring){const queue=lanes.get(lane);queue.forEach((waiter,at)=>waiter.onPosition?.(positionOf(lane,at)));}
 };
 const positionOf=(lane,at)=>{let ahead=at;for(const other of ring)if(other!==lane)ahead+=Math.min(lanes.get(other).length,at+1);return ahead;};
 const once=fn=>{let done=false;return()=>{if(!done){done=true;fn();}};};
 return{
  enter(lane,signal,onPosition){
   if(signal?.aborted)return Promise.reject(signal.reason??new Error("aborted"));
   if(inFlight<maximumInFlight&&queued===0){inFlight++;return Promise.resolve(once(()=>{inFlight--;grant();}));}
   if(queued>=maximumQueued)return Promise.reject(new BusyErrorV1(inFlight,queued));
   return new Promise((resolve,reject)=>{
    let queue=lanes.get(lane);if(!queue){queue=[];lanes.set(lane,queue);ring.push(lane);}
    const waiter={resolve,onPosition};
    queue.push(waiter);queued++;
    onPosition?.(positionOf(lane,queue.length-1));
    signal?.addEventListener("abort",()=>{const at=queue.indexOf(waiter);if(at>=0){queue.splice(at,1);queued--;reject(signal.reason??new Error("aborted"));grant();}},{once:true});
   });
  },
  get inFlight(){return inFlight;},get queued(){return queued;},
 };
}
