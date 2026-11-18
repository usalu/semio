/** 🚀️ Contract: the inference of the owners a reader needs is released as soon as the take pass has looked at them, or after `releaseMilliseconds` when the pass is slow, and never waits for the warm pass, which is stopped at the release so that its children are free for the inference; the pass and the warm pass are awaited at the end and a failing pass reaches the caller.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{TAKE_POLICY_V1,runTakeBesideInferV1}from"../../🟨️.mjs";

const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("the inference starts when the take pass ends although the warm pass would run for ever, and the warm pass is stopped then",async()=>{
 const events:string[]=[];let stop:(()=>void)|undefined;
 const warm=()=>new Promise<void>(done=>{events.push("warm-start");stop=()=>{events.push("warm-stopped");done();};});
 await runTakeBesideInferV1({take:async()=>{events.push("take-start");await sleep(20);events.push("take-end");},warm,stopWarm:()=>stop?.(),release:()=>events.push("release"),infer:async()=>{events.push("infer-start");await sleep(5);events.push("infer-end");},releaseMilliseconds:5000});
 expect(events.indexOf("release")).toBeGreaterThan(events.indexOf("take-end")-1);
 expect(events.indexOf("infer-start")).toBeGreaterThan(events.indexOf("take-end"));
 expect(events).toContain("warm-stopped");
 expect(events.indexOf("warm-stopped")).toBeLessThan(events.indexOf("infer-end"));
});

test("a slow take pass releases the inference after the declared time, the pass keeps running beside it",async()=>{
 const began=performance.now();let releasedAt=-1,passEnded=false;
 await runTakeBesideInferV1({take:async()=>{await sleep(300);passEnded=true;},release:()=>{releasedAt=performance.now()-began;},infer:async()=>{await sleep(10);expect(passEnded).toBe(false);},releaseMilliseconds:60});
 expect(releasedAt).toBeGreaterThanOrEqual(50);
 expect(releasedAt).toBeLessThan(200);
 expect(passEnded).toBe(true);
});

test("the policy declares a release time that is far shorter than the bound of the pass",()=>{
 expect(TAKE_POLICY_V1.releaseMilliseconds).toBeGreaterThan(0);
 expect(TAKE_POLICY_V1.releaseMilliseconds).toBeLessThanOrEqual(TAKE_POLICY_V1.passBoundMilliseconds/10);
});

test("the end of the inference stops the pass, a failing pass is rethrown after the inference, a failing inference wins",async()=>{
 let ended=false;
 await runTakeBesideInferV1({take:()=>new Promise<void>(done=>{const timer=setInterval(()=>{if(ended){clearInterval(timer);done();}},5);}),release:()=>{},infer:async()=>{await sleep(10);},end:()=>{ended=true;},releaseMilliseconds:5});
 expect(ended).toBe(true);
 let inferred=false;
 await expect(runTakeBesideInferV1({take:async()=>{throw new Error("take failed");},release:()=>{},infer:async()=>{await sleep(20);inferred=true;},releaseMilliseconds:5000})).rejects.toThrow("take failed");
 expect(inferred).toBe(true);
 await expect(runTakeBesideInferV1({take:async()=>{throw new Error("take failed");},release:()=>{},infer:async()=>{throw new Error("infer failed");},releaseMilliseconds:5000})).rejects.toThrow("infer failed");
});
