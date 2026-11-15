import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const.priority;
/** 🎚️ The service classes of a lease request, most urgent first: a person waiting at a terminal, an agent's run, unattended background work. */
export type LeasePriorityV1="interactive"|"agent"|"background";
export type LeasePriorityContextV1=Readonly<{agentSession:boolean;interactiveTerminal:boolean}>;
const advance=(priority:LeasePriorityV1):number=>policy.classes.find(row=>row.id===priority)!.advanceMilliseconds;

/** 🏷️ Resolves the class of the running process: the explicit environment value wins, a person at a terminal outside any agent session is interactive, everything else gets the declared default. The capacity queue reads the same function so both queues agree. */
export function leasePriorityClassV1(environment:Readonly<Record<string,string|undefined>>,context:LeasePriorityContextV1):LeasePriorityV1{
 const explicit=environment[policy.environment],known=policy.classes.find(row=>row.id===explicit);
 if(known)return known.id as LeasePriorityV1;
 return context.interactiveTerminal&&!context.agentSession?"interactive":policy.default as LeasePriorityV1;
}

/** 🎟️ The arrival a ticket is filed under: the real arrival moved earlier by the class advance (interactive) or later (background). Tickets are served in this order, so a class can overtake another only inside the window `leaseOvertakeWindowV1` and every request is overtaken by a bounded set: aging is the arrival time itself. */
export function leaseTicketArrivalV1(priority:LeasePriorityV1,arrivedAtMs:number):number{return Math.max(0,arrivedAtMs-advance(priority));}

/** ↩️ The real arrival behind a filed ticket arrival; the writer-preference clock must keep running on real time. */
export function leaseRealArrivalV1(priority:LeasePriorityV1,ticketArrivalMs:number):number{return ticketArrivalMs+advance(priority);}

/** 📏️ How long after an earlier request of class `earlier` a request of class `later` may still be served before it. */
export function leaseOvertakeWindowV1(later:LeasePriorityV1,earlier:LeasePriorityV1):number{return Math.max(0,advance(later)-advance(earlier));}

export type LeaseJobV1=Readonly<{id:string;class:LeasePriorityV1;arrival:number;service:number}>;
export type LeaseJobOutcomeV1=Readonly<{id:string;class:LeasePriorityV1;arrival:number;start:number;wait:number}>;
/** 🧪️ Serves one exclusive resource without preemption: whenever it frees up, the waiting job with the smallest filed arrival (ties by id) starts. `fifo` ignores classes. Times are in seconds. */
export function simulateLeaseQueueV1(jobs:readonly LeaseJobV1[],fifo=false):readonly LeaseJobOutcomeV1[]{
 const pending=[...jobs].sort((left,right)=>left.arrival-right.arrival||(left.id<right.id?-1:1)),waiting:LeaseJobV1[]=[],done:LeaseJobOutcomeV1[]=[],filed=(job:LeaseJobV1):number=>fifo?job.arrival:job.arrival-advance(job.class)/1000;
 let time=0,next=0;
 while(next<pending.length||waiting.length){
  while(next<pending.length&&pending[next]!.arrival<=time)waiting.push(pending[next++]!);
  if(!waiting.length){time=pending[next]!.arrival;continue;}
  waiting.sort((left,right)=>filed(left)-filed(right)||(left.id<right.id?-1:1));
  const job=waiting.shift()!;done.push(Object.freeze({id:job.id,class:job.class,arrival:job.arrival,start:time,wait:time-job.arrival}));time+=job.service;
 }
 return done;
}
