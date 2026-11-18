/** 🪙️ Contract: what the memory governor reserves for an owner is what owners of its kind measurably take. A sample that sits at the ceiling, or that only reflects the garbage of a stretch the owners shared, is not a measurement and never makes a light owner reserve a heavy one's share; a kind that really takes memory alone keeps its measured peak; no owner reserves less than the floor.
 * @see ../../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{GRAPH_MEMORY_POLICY_V1,createMemoryGovernorV1}from"../../🟨️.mjs";

const MB=1048576,fast={...GRAPH_MEMORY_POLICY_V1,admitMinimumMilliseconds:0,pollMilliseconds:1};

test("persisted samples at the ceiling are unmeasured: eight owners of the kind run together within the budget instead of three",async()=>{
 const ceiling=GRAPH_MEMORY_POLICY_V1.grantMegabytes*MB,governor=createMemoryGovernorV1({workers:8,headroom:()=>6591*MB,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({"language:cargo":[ceiling,ceiling,ceiling,ceiling,ceiling,ceiling,ceiling,666*MB]}),write:()=>undefined}});
 const began=performance.now();let running=0,peak=0;
 await Promise.all(Array.from({length:8},(_,index)=>governor.run(async()=>{running++;peak=Math.max(peak,running);await Bun.sleep(40);running--;},{kind:"language:cargo",key:"font-"+index})));
 expect(peak,"eight owners at 666 MB fit 6591 MB").toBe(8);
 expect(governor.stats().kinds["language:cargo"]).toBeLessThanOrEqual(666);
 expect(performance.now()-began,"nobody waited for the 60 second bound").toBeLessThan(5000);
 governor.dispose();
});

test("garbage that the owners of a stretch shared is capped at the initial guess when the kind never measured more alone, in memory and in the persisted samples",async()=>{
 const written:Record<string,number[]>[]=[];let live=0;
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>40000*MB-live,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({"language:cargo":[100*MB]}),write:value=>{written.push(structuredClone(value) as Record<string,number[]>);}}});
 await Promise.all([1,2,3,4].map(index=>governor.run(async()=>{live+=3000*MB;await Bun.sleep(60);},{kind:"language:cargo",key:"p"+index})));
 expect(governor.stats().kinds["language:cargo"]).toBeLessThanOrEqual(256);
 expect(written.length).toBeGreaterThan(0);
 for(const row of written)for(const bytes of row["language:cargo"]!)expect(bytes).toBeLessThanOrEqual(256*MB);
 governor.dispose();
});

test("a kind that takes memory when it runs alone keeps its measured peak, and the owners that run beside it are then bounded by it",async()=>{
 let live=0;
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>40000*MB-live,estimateBytes:256*MB,policy:fast});
 await governor.run(async()=>{live+=900*MB;await Bun.sleep(30);live-=900*MB;},{kind:"project",key:"solo"});
 expect(governor.stats().kinds.project).toBeGreaterThanOrEqual(850);
 expect(governor.stats().kinds.project).toBeLessThanOrEqual(950);
 governor.dispose();
});

test("no owner reserves less than the floor: a kind that took nothing still cannot be admitted beside a running owner when the headroom is below the floor",async()=>{
 const floor=GRAPH_MEMORY_POLICY_V1.floorOwnerMegabytes;expect(floor).toBe(64);
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>(floor+20)*MB,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({"language:rust":[0,0]}),write:()=>undefined}});
 const order:string[]=[];
 const first=governor.run(async()=>{order.push("a:start");await Bun.sleep(80);order.push("a:end");},{kind:"language:rust",key:"a"});
 await Bun.sleep(10);
 const second=governor.run(async()=>{order.push("b:start");},{kind:"language:rust",key:"b"});
 await Promise.all([first,second]);
 expect(order).toEqual(["a:start","a:end","b:start"]);
 governor.dispose();
});

test("the policy declares the floor and the module schema and the test fixture agree",()=>{
 expect(GRAPH_MEMORY_POLICY_V1.floorOwnerMegabytes).toBe(64);
 expect(GRAPH_MEMORY_POLICY_V1.floorOwnerMegabytes).toBeLessThan(GRAPH_MEMORY_POLICY_V1.initialOwnerMegabytes);
});

test("a high persisted peak that only shared owners contradict decays after four windows of low shares, never sooner, and the owners then run together again",async()=>{
 let used=0;const window=GRAPH_MEMORY_POLICY_V1.peakSamples;
 const governor=createMemoryGovernorV1({workers:3,headroom:()=>1e15-used,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({k:[500*MB]}),write:()=>undefined}});
 const wave=(count:number)=>Promise.all(Array.from({length:count},(_,index)=>governor.run(async()=>{used+=10*MB;await Bun.sleep(8);used-=10*MB;},{kind:"k",key:"o"+index})));
 await wave(3*4);expect(governor.stats().kinds.k,"twelve low shares do not lower it").toBe(500);
 await wave(4*window*3);
 expect(governor.stats().kinds.k,"a window of evidence lowers it").toBeLessThan(100);
 governor.dispose();
});
