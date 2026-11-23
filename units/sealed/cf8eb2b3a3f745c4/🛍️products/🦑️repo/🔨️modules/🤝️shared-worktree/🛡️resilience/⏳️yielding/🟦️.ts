/** ⏳️ A cooperative yield for long loops over a whole manifest, queue or event batch: `turn()` costs next to nothing per call and, once `budgetMilliseconds` (8) of work have passed since the last yield, returns a promise that settles on the next turn of the event loop, so timers, signals, the store guard and the heartbeats of the daemon are served between the slices. Use it as `await turn()` inside the loop. */
export function yieldingV1(budgetMilliseconds:number=8):()=>Promise<void>|undefined{
 let calls=0,started=performance.now();
 return ()=>{
  if((++calls&63)!==0||performance.now()-started<budgetMilliseconds)return undefined;
  return new Promise<void>(done=>{setImmediate(()=>{started=performance.now();done();});});
 };
}
