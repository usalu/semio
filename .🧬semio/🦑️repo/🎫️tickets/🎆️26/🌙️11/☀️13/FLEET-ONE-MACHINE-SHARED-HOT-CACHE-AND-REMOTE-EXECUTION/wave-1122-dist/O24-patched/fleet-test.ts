import {expect,test}from"bun:test";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {testPythonV1}from"../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {admitFleetComponentJobV1,describeFleetComponentJobsV1,FleetPlacementFailedErrorV1,runPassesInJobSlotsV1,type FleetJobAdmitHostV1,type FleetJobEventV1,type LocalJobEventV1,type LocalJobPressureV1,type LocalJobSampleV1,type PassJobSlotV1}from"../../🟦️.ts";
const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
const pause=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
const until=async(condition:()=>boolean)=>{for(let step=0;step<400&&!condition();step++)await pause(5);expect(condition()).toBe(true);};
const wasm=(count:number)=>Array.from({length:count},(_,index)=>({id:"p"+index,target:"wasm32-wasip2"}));
type Memory={current:LocalJobSampleV1};
const normal=(availableMegabytes:number|null,pressure:LocalJobPressureV1="normal"):LocalJobSampleV1=>({availableMegabytes,ownedMegabytes:0,pressure});
function lane(count:number,options:Readonly<{jobs:number;local?:{maximum:number;peak:number};fleet:{maximum:number;slots:{current:number|null};footprint?:number}}>,memory:Memory,fail:ReadonlySet<string>=new Set()){
 const started:string[]=[],slots:Record<string,PassJobSlotV1>={},gates=new Map<string,()=>void>(),fleetEvents:FleetJobEventV1[]=[],localEvents:LocalJobEventV1[]=[],failedOnce=new Set<string>();
 let running=0,maximumRunning=0,fleetRunning=0;
 const done=runPassesInJobSlotsV1(wasm(count),{jobs:options.jobs,check:()=>undefined,
  ...options.local?{local:{maximum:options.local.maximum,floorMegabytes:4000,peak:()=>options.local!.peak,sample:async()=>memory.current,pollMilliseconds:5,progressMilliseconds:0,report:(_pass,event)=>{localEvents.push(event);}}}:{},
  fleet:{maximum:options.fleet.maximum,slots:async()=>options.fleet.slots.current===null?null:Math.max(0,options.fleet.slots.current-fleetRunning),footprintMegabytes:options.fleet.footprint??512,floorMegabytes:512,sample:async()=>memory.current,pollMilliseconds:5,progressMilliseconds:0,report:(_pass,event)=>{fleetEvents.push(event);}},
  run:(pass,slot)=>new Promise<void>((resolve,reject)=>{
   started.push(pass.id);slots[pass.id]=slot;running++;maximumRunning=Math.max(maximumRunning,running);if(slot==="fleet")fleetRunning++;
   const end=()=>{running--;if(slot==="fleet")fleetRunning--;};
   if(slot==="fleet"&&fail.has(pass.id)&&!failedOnce.has(pass.id)){failedOnce.add(pass.id);end();started.pop();delete slots[pass.id];reject(new FleetPlacementFailedErrorV1(pass.id+": no worker took the final"));return;}
   gates.set(pass.id,()=>{end();resolve();});
  })});
 return{started,slots,fleetEvents,localEvents,done,running:()=>running,maximumRunning:()=>maximumRunning,finish:(id:string)=>gates.get(id)!()};
}
test("the fleet slot law is the one of the Python model for every state of a generated grid, and the spec vectors hold",()=>{
 const base:FleetJobAdmitHostV1={running:0,maximum:4,slots:2,availableMegabytes:8000,reservedMegabytes:0,floorMegabytes:512,pressure:"normal",footprintMegabytes:512};
 expect(admitFleetComponentJobV1({...base,pressure:"warn",availableMegabytes:6300})).toEqual({admit:true,reason:"fleet-fits",freeMegabytes:5788});
 expect(admitFleetComponentJobV1({...base,pressure:"critical"}).reason).toBe("fleet-pressure");
 expect(admitFleetComponentJobV1({...base,availableMegabytes:null})).toEqual({admit:true,reason:"fleet-fits",freeMegabytes:0});
 expect(admitFleetComponentJobV1({...base,slots:null}).reason).toBe("fleet-none");
 expect(admitFleetComponentJobV1({...base,slots:0}).reason).toBe("fleet-busy");
 expect(admitFleetComponentJobV1({...base,running:4}).reason).toBe("fleet-cap");
 expect(admitFleetComponentJobV1({...base,availableMegabytes:1000,floorMegabytes:512}).reason).toBe("fleet-memory");
 const cases:FleetJobAdmitHostV1[]=[];
 for(const running of[0,2,4])for(const slots of[null,0,1,5])for(const availableMegabytes of[null,0,900,1100,2000,6300,20000])for(const reservedMegabytes of[0,512,2048])for(const pressure of[null,"normal","warn","critical"] as const)for(const footprintMegabytes of[256,512])cases.push({running,maximum:4,slots,availableMegabytes,reservedMegabytes,floorMegabytes:512,pressure,footprintMegabytes});
 expect(cases.length).toBeGreaterThan(1000);
 const expected=python(cases);
 cases.forEach((host,index)=>expect(admitFleetComponentJobV1(host),JSON.stringify(host)).toEqual(expected[index] as never));
 expect(()=>admitFleetComponentJobV1({...base,footprintMegabytes:0})).toThrow();
});
test("1 local + 4 fleet slots under raised local memory pressure run 5 passes at once, the finals of 4 placed on workers; the sixth waits and says which slot it waits for",async()=>{
 const memory:Memory={current:normal(6300,"warn")},slots={current:4},run=lane(6,{jobs:24,local:{maximum:3,peak:18865},fleet:{maximum:23,slots}},memory);
 await until(()=>run.started.length===5);await pause(40);
 expect(run.started).toEqual(["p0","p1","p2","p3","p4"]);expect(run.slots).toEqual({p0:"local",p1:"fleet",p2:"fleet",p3:"fleet",p4:"fleet"});expect(run.running()).toBe(5);
 const waiting=run.fleetEvents.filter(event=>event.kind==="waiting");
 expect(waiting.length).toBeGreaterThan(0);expect(waiting.every(event=>event.waitsFor==="either")).toBe(true);expect(waiting[0]!.fleetReason).toBe("fleet-busy");expect(waiting[0]!.localReason).toBe("local-pressure");
 expect(run.fleetEvents.filter(event=>event.kind==="started").length).toBe(4);expect(run.localEvents.some(event=>event.kind==="started")).toBe(false);
 run.finish("p1");await until(()=>run.started.length===6);expect(run.slots.p5).toBe("fleet");
 for(const id of["p0","p2","p3","p4","p5"])run.finish(id);await run.done;
 expect(run.maximumRunning()).toBe(5);
});
test("a fleet pass whose worker placement fails gives its slot back and waits for a LOCAL slot, never starting in unreserved memory, without holding back the next passes; freed local memory resumes it",async()=>{
 const memory:Memory={current:normal(6300,"warn")},slots={current:2},run=lane(3,{jobs:8,local:{maximum:2,peak:3000},fleet:{maximum:7,slots}},memory,new Set(["p1"]));
 await until(()=>run.started.includes("p2"));await pause(60);
 expect(run.slots.p0).toBe("local");expect(run.slots.p2,"the next pass took the free fleet slot").toBe("fleet");expect(run.started.includes("p1")).toBe(false);
 const waits=run.fleetEvents.filter(event=>event.kind==="waiting"&&event.waitsFor==="local"&&event.localReason==="local-pressure");
 expect(waits.length,"the failed pass waits for a local slot").toBeGreaterThan(0);expect(waits[0]!.fleetReason).toBeNull();
 memory.current=normal(30000,"normal");await until(()=>run.started.includes("p1"));
 expect(run.slots.p1).toBe("local");expect(run.localEvents.some(event=>event.kind==="started"&&event.peakMegabytes===3000)).toBe(true);
 for(const id of["p0","p1","p2"])run.finish(id);await run.done;
});
test("pressure release resumes the local slots: a pass that waited for either kind starts on the local one when memory frees while the fleet stays busy",async()=>{
 const memory:Memory={current:normal(9000,"warn")},slots={current:0},run=lane(3,{jobs:8,local:{maximum:3,peak:2000},fleet:{maximum:7,slots}},memory);
 await until(()=>run.started.length===1);await pause(40);
 expect(run.started).toEqual(["p0"]);expect(run.fleetEvents.some(event=>event.kind==="waiting"&&event.waitsFor==="either"&&event.fleetReason==="fleet-busy")).toBe(true);
 memory.current=normal(30000,"normal");await until(()=>run.started.length===3);
 expect(run.slots).toEqual({p0:"local",p1:"local",p2:"local"});expect(run.fleetEvents.filter(event=>event.kind==="started").length).toBe(0);
 for(const id of["p0","p1","p2"])run.finish(id);await run.done;
});
test("fleet slots follow the live worker capacity: they grow while the lane runs, a silent fleet leaves the passes waiting for a local slot, and a finished local job frees the next pass",async()=>{
 const memory:Memory={current:normal(20000)},slots:{current:number|null}={current:1},run=lane(6,{jobs:24,fleet:{maximum:23,slots}},memory);
 await until(()=>run.started.length===2);await pause(30);expect(run.started).toEqual(["p0","p1"]);expect(run.fleetEvents.some(event=>event.kind==="waiting"&&event.waitsFor==="fleet"&&event.fleetReason==="fleet-busy")).toBe(true);
 slots.current=3;await until(()=>run.started.length===4);expect(run.started).toEqual(["p0","p1","p2","p3"]);
 slots.current=null;await pause(30);expect(run.started.length).toBe(4);expect(run.fleetEvents.some(event=>event.fleetReason==="fleet-none"&&event.waitsFor==="local")).toBe(true);
 slots.current=4;await until(()=>run.started.length===5);expect(run.slots.p4).toBe("fleet");
 run.finish("p0");await until(()=>run.started.length===6);expect(run.slots.p5).toBe("local");
 for(const id of["p1","p2","p3","p4","p5"])run.finish(id);await run.done;
});
test("critical memory pressure keeps even a fleet pass away, raised pressure does not, and the first job always starts",async()=>{
 const memory:Memory={current:normal(900,"critical")},slots={current:5},run=lane(3,{jobs:8,fleet:{maximum:7,slots}},memory);
 await until(()=>run.started.length===1);await pause(40);expect(run.started).toEqual(["p0"]);expect(run.fleetEvents.some(event=>event.fleetReason==="fleet-pressure")).toBe(true);
 memory.current=normal(6000,"warn");await until(()=>run.started.length===3);expect(run.slots).toEqual({p0:"local",p1:"fleet",p2:"fleet"});
 for(const id of["p0","p1","p2"])run.finish(id);await run.done;
});
test("the fleet slot lines are English then German and name the kind of slot a pass waits for",()=>{
 const values={pass:"release-04",running:4,slots:0,footprint:512,peak:3774,free:-16277,floor:6144,reason:"fleet-busy",localReason:"local-pressure"};
 const either=describeFleetComponentJobsV1("waiting",{...values,waitsFor:"either"}),local=describeFleetComponentJobsV1("waiting",{...values,waitsFor:"local",reason:"fleet-none"}),fleet=describeFleetComponentJobsV1("waiting",{...values,waitsFor:"fleet",reason:"fleet-cap"});
 expect(either).toContain("release-04 waits for a fleet slot or a local slot: no worker slot is free (every worker slot that fits a final is taken) and a local job needs 3774 MB, -16277 MB are free above the 6144 MB floor (memory pressure is raised, so no further local final starts)");
 expect(either).toContain(" / [component-jobs] release-04 wartet auf einen Flotten-Platz oder einen lokalen Platz: kein Worker-Platz ist frei (jeder Worker-Platz, der ein Endstück trägt, ist belegt)");
 expect(local).toContain("waits for a local slot");expect(local).toContain("wartet auf einen lokalen Platz");expect(fleet).toContain("waits for a fleet slot");expect(fleet).toContain("wartet auf einen Flotten-Platz");
 const started=describeFleetComponentJobsV1("started",values),plan=describeFleetComponentJobsV1("plan",{operation:"component build",local:3,fleet:23,footprint:512,seconds:5,slots:4});
 expect(started).toContain("starts on a fleet slot");expect(started).toContain("startet auf einem Flotten-Platz");expect(plan).toContain("3 local slot(s) and up to 23 fleet slot(s)");expect(plan).toContain("3 lokale Plätze und bis zu 23 Flotten-Plätze");
});
