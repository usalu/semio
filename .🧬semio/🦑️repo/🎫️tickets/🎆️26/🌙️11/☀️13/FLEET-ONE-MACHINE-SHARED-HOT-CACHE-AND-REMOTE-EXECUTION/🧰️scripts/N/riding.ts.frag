
/** 🛟️ Submits like `executeOffloadV1` and, when the call is lost to the transport (the scheduler restarts: connection refused or reset, the stream ends without an answer), submits the same action again with jittered back-off for `budgetMilliseconds`. The scheduler deduplicates by action digest (a worker that kept running the action re-joins and the scheduler adopts it) and answers an action that already finished from the action cache, so the retry never runs the unit a second time; once the budget is spent the caller compiles locally as before. A refusal, a timeout or a cancel is never retried. */
export async function executeOffloadRidingThroughV1(input:Parameters<typeof executeOffloadV1>[0],budgetMilliseconds:number,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise<void>(done=>{setTimeout(done,milliseconds);})):Promise<OffloadExecutionV1>{
 const until=Date.now()+budgetMilliseconds;
 let execution=await executeOffloadV1(input);
 for(let attempt=0;execution.kind==="declined"&&execution.transient===true&&input.signal?.aborted!==true&&Date.now()<until;attempt++){
  await pause(Math.min(2000,250*2**attempt)*(0.5+Math.random()));
  execution=await executeOffloadV1(input);
 }
 return execution;
}
