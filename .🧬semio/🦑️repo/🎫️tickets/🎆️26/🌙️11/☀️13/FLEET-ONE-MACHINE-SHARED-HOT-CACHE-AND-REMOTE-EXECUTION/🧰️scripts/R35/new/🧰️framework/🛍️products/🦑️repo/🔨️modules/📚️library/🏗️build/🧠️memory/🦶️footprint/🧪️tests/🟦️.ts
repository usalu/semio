import{expect,test}from"bun:test";
import Ajv from"ajv";
import{execFileSync}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readFileSync}from"node:fs";
import{createFootprintTraceV1,heapCensusV1,FOOTPRINT_MESSAGES_V1,FOOTPRINT_POLICY_V1,createFootprintGateV1,createFootprintMeterV1,decideFootprintV1,footprintGateV1,footprintHeadroomBytesV1,grantBytesOfV1,readProcessFootprintBytesV1,releaseRuntimeMemoryV1,renderFootprintMessageV1,settleWaveMemoryV1,useFootprintGateV1}from"../🟨️.mjs";
import{createGraphParentMemoryGovernorV1,createMemoryGovernorV1,serviceMemoryHeadroomBytesV1}from"../../🟨️.mjs";
import{createClosureSettlerV1,useCapacityGrantV1}from"../../../🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createFactIndexV1,type GenerationRowV1}from"../../../🕸️graph/🕰️generations/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";

/** 🦶️ Contract: the memory of a graph read is bounded by its grant. The physical footprint of the parent (not the resident set, not the managed heap) is read without blocking, compared with the grant, the admission of new work slows before the grant is overrun, and the process settles after every published wave and says so in one line.
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const MB=1048576,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};

test("policy and messages equal the schema file; both languages have the same keys and placeholders",()=>{
 expect(FOOTPRINT_POLICY_V1).toEqual(schema.definitions.Policy.const);expect(FOOTPRINT_MESSAGES_V1).toEqual(schema.definitions.Messages.const);
 expect(Object.keys(FOOTPRINT_MESSAGES_V1.en)).toEqual(Object.keys(FOOTPRINT_MESSAGES_V1.de));
 for(const key of Object.keys(FOOTPRINT_MESSAGES_V1.en))expect(holes(FOOTPRINT_MESSAGES_V1.de[key as"slowed"])).toEqual(holes(FOOTPRINT_MESSAGES_V1.en[key as"slowed"]));
 expect(renderFootprintMessageV1("waveSettled",{generation:7,before:9000,after:1200,managed:300,grant:10462},{SEMIO_LANGUAGE:"en,de"})).toEqual(["graph: wave 7 settled from 9000 MB to 1200 MB (managed 300 MB, grant 10462 MB)","Graph: Welle 7 beruhigt von 9000 MB auf 1200 MB (verwaltet 300 MB, Zusage 10462 MB)"]);
 expect(FOOTPRINT_POLICY_V1.softPercent).toBeLessThan(FOOTPRINT_POLICY_V1.hardPercent);
});

test("the decision law: run below soft, settle from soft, wait from hard, run without a known grant",()=>{
 const grantBytes=1000*MB,at=(percent:number)=>decideFootprintV1({footprintBytes:grantBytes*percent/100,grantBytes});
 expect([at(0),at(59),at(60),at(84),at(85),at(300)]).toEqual(["run","run","settle","settle","wait","wait"]);
 expect(decideFootprintV1({footprintBytes:50000*MB,grantBytes:undefined})).toBe("run");expect(decideFootprintV1({footprintBytes:50000*MB,grantBytes:0})).toBe("run");expect(decideFootprintV1({footprintBytes:Number.NaN,grantBytes})).toBe("run");
 expect(footprintHeadroomBytesV1({footprintBytes:800*MB,grantBytes})).toBe(50*MB);expect(footprintHeadroomBytesV1({footprintBytes:900*MB,grantBytes})).toBeLessThan(0);expect(footprintHeadroomBytesV1({footprintBytes:1,grantBytes:undefined})).toBe(Infinity);
 const ajv=new Ajv({strict:false});ajv.addSchema(schema,"s");for(const percent of[0,70,99])expect(ajv.validate({$ref:"s#/definitions/Decision"},at(percent))).toBe(true);
});

test("the grant of a capacity source is the declared limit, else the reserved megabytes, else none",()=>{
 expect(grantBytesOfV1(()=>({limit:{memoryMegabytes:4096},handle:{reservedMegabytes:1}}))).toBe(4096*MB);
 expect(grantBytesOfV1(()=>({handle:{reservedMegabytes:512}}))).toBe(512*MB);expect(grantBytesOfV1(()=>undefined)).toBeUndefined();expect(grantBytesOfV1(undefined)).toBeUndefined();expect(grantBytesOfV1(()=>({handle:{reservedMegabytes:0}}))).toBeUndefined();
});

test("the reader answers the footprint of one process, equal to the system tool, and never the resident set when its reading fails",async()=>{
 const own=await readProcessFootprintBytesV1(process.pid);expect(own).toBeGreaterThan(8*MB);
 if(process.platform==="darwin"){
  const text=execFileSync("footprint",["-f","bytes","--noCategories","-p",String(process.pid)],{encoding:"utf8"}),tool=Number(/phys_footprint:\s+(\d+)\s+B/u.exec(text)?.[1]??/Footprint:\s+(\d+)\s+B/u.exec(text)?.[1]);
  expect(Math.abs(own!-tool)).toBeLessThan(Math.max(64*MB,tool*0.5));
 }
 if(process.platform==="linux"){
  const text=readFileSync(`/proc/${process.pid}/smaps_rollup`,"utf8"),kilobytes=(name:string)=>Number(new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(text)?.[1]??0);
  expect(Math.abs(own!-(kilobytes("Pss")+kilobytes("SwapPss"))*1024)).toBeLessThan(Math.max(64*MB,own!*0.5));
 }
 const failing=async()=>{throw Error("no tool");};
 expect(await readProcessFootprintBytesV1(1,{platform:"darwin",run:failing,rss:()=>99})).toBeUndefined();expect(await readProcessFootprintBytesV1(1,{platform:"linux",read:failing,rss:()=>99})).toBeUndefined();
 expect(await readProcessFootprintBytesV1(1,{platform:"darwin",run:async()=>"bun [1]: 64-bit    Footprint: 123456789 B\n    phys_footprint: 123456789 B\n",rss:()=>99})).toBe(123456789);
 expect(await readProcessFootprintBytesV1(1,{platform:"win32",rss:()=>99})).toBe(99);
});

test("the meter answers at once, reads in the background once per ttl, ignores a failed reading and prefers the reading to the resident set",async()=>{
 const time=clock();let calls=0,fail=false,value=500*MB;
 const meter=createFootprintMeterV1({read:async()=>{calls++;if(fail)throw Error("x");return value;},rss:()=>7000*MB,now:time.now,ttlMilliseconds:5000});
 expect(meter.bytes()).toBe(7000*MB);expect(calls).toBe(1);await meter.refresh();expect(meter.bytes()).toBe(500*MB);expect(calls).toBe(2);
 meter.bytes();meter.bytes();expect(calls).toBe(2);time.advance(5000);meter.bytes();meter.bytes();expect(calls).toBe(3);await meter.refresh();expect(calls).toBe(4);
 fail=true;value=900*MB;time.advance(6000);await meter.refresh();expect(meter.bytes()).toBe(500*MB);expect(meter.peakBytes()).toBe(500*MB);
});

test("fresh() reuses a reading younger than the ttl and reads again after it",async()=>{
 const time=clock();let calls=0;const meter=createFootprintMeterV1({read:async()=>{calls++;return 100*MB;},now:time.now,ttlMilliseconds:5000});
 await meter.fresh();await meter.fresh();await meter.fresh();expect(calls).toBe(1);time.advance(5000);await meter.fresh();expect(calls).toBe(2);
});

test("a gate below the soft limit admits at once and touches nothing; without a grant it never does",async()=>{
 const time=clock();let reads=0,released=0;
 const meter=createFootprintMeterV1({read:async()=>{reads++;return 300*MB;},now:time.now});await meter.refresh();reads=0;
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{released++;},sleep:time.sleep,now:time.now});
 for(let index=0;index<50;index++)expect(await gate.admit()).toBe("run");
 expect(released).toBe(0);expect(gate.stats().waits).toBe(0);expect(gate.stats().admitted).toBe(50);
 expect(await createFootprintGateV1({meter,grantBytes:()=>undefined,release:()=>{released++;}}).admit()).toBe("run");expect(released).toBe(0);
});

test("between soft and hard the gate only collects and shrinks, at most once per minimum, and lets the work run; the relievers stay untouched",async()=>{
 const time=clock(),lines:string[]=[],order:string[]=[];let held=700*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{order.push("release");held=200*MB;},sleep:time.sleep,now:time.now,report:line=>lines.push(line),heapCensus:async()=>undefined,environment:{SEMIO_LANGUAGE:"en"}});
 gate.relieve(()=>{order.push("reliever");});
 expect(await gate.admit()).toBe("run");expect(order).toEqual(["release"]);expect(gate.stats().settled).toBe(1);
 expect(lines).toEqual(["graph: the process held 700 MB of its grant of 1000 MB; collected and shrunk to 200 MB"]);
 held=700*MB;await meter.refresh();expect(await gate.admit()).toBe("settle");expect(gate.stats().settled).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.settleMinimumMilliseconds);expect(await gate.admit()).toBe("run");expect(gate.stats().settled).toBe(2);expect(order).toEqual(["release","release"]);
});

test("at the hard limit the relievers drop what a pass can re-read after the collection did not help, and the work runs when that is enough",async()=>{
 const time=clock(),order:string[]=[];let held=950*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{order.push("release");},sleep:time.sleep,now:time.now});
 const off=gate.relieve(()=>{order.push("reliever");held=300*MB;});
 expect(await gate.admit()).toBe("run");expect(order).toEqual(["release","reliever","release"]);expect(gate.stats().waits).toBe(0);
 off();expect(gate.relieve(()=>undefined)).toBeInstanceOf(Function);
});

test("when nothing helps the gate waits in steps, at most the bound per call, never forever, repeats no wait within the cooldown and stops when the footprint falls",async()=>{
 const time=clock(),lines:string[]=[];let held=950*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:time.sleep,now:time.now,report:line=>lines.push(line),heapCensus:async()=>undefined,environment:{SEMIO_LANGUAGE:"en,de"}});
 const began=time.now();expect(await gate.admit()).toBe("wait");
 expect(time.now()-began).toBe(FOOTPRINT_POLICY_V1.waitBoundMilliseconds);expect(gate.stats().waits).toBe(1);expect(gate.stats().waitedMilliseconds).toBe(FOOTPRINT_POLICY_V1.waitBoundMilliseconds);
 expect(lines.some(line=>line.startsWith("graph: the process holds 950 MB of its grant of 1000 MB; new work waits up to 20 s"))).toBe(true);expect(lines.some(line=>line.startsWith("Graph: der Prozess hält 950 MB"))).toBe(true);
 const after=time.now();for(let index=0;index<5;index++)expect(await gate.admit()).toBe("wait");
 expect(time.now()).toBe(after);expect(gate.stats().waits).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.waitCooldownMilliseconds);expect(await gate.admit()).toBe("wait");expect(gate.stats().waits).toBe(2);
 const calm=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:async milliseconds=>{time.advance(milliseconds);held=300*MB;},now:time.now});
 const start=time.now();expect(await calm.admit()).toBe("run");expect(time.now()-start).toBeLessThanOrEqual(FOOTPRINT_POLICY_V1.waitStepMilliseconds);
});

test("a wait is cancellable",async()=>{
 const meter=createFootprintMeterV1({read:async()=>990*MB,ttlMilliseconds:0}),controller=new AbortController();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined});
 const asking=gate.admit(controller.signal);setTimeout(()=>controller.abort(Error("stop")),30);
 await expect(asking).rejects.toThrow("stop");
});

test("the governor starts no further owner while the footprint of the parent stands above its limit, and the first owner always runs",async()=>{
 let released=false,running=0,heldPeak=0,done=0;const started:string[]=[];
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>1e12,hold:()=>!released,estimateBytes:MB,peaks:{read:()=>({k:[MB]}),write:()=>undefined}});
 const task=(name:string,first:boolean)=>async()=>{started.push(name);running++;if(!released)heldPeak=Math.max(heldPeak,running);await Bun.sleep(first?400:20);running--;done++;if(first)released=true;};
 await Promise.all([governor.run(task("a",true),{kind:"k"}),governor.run(task("b",false),{kind:"k"}),governor.run(task("c",false),{kind:"k"})]);governor.dispose();
 expect(done).toBe(3);expect(started[0]).toBe("a");expect(heldPeak).toBe(1);
});

test("the parent governor takes its hold and the service headroom from the footprint gate of the process, not from the resident set",async()=>{
 expect(footprintGateV1()).toBeUndefined();
 const meter=createFootprintMeterV1({read:async()=>9000*MB,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>10000*MB,release:()=>undefined});useFootprintGateV1(gate);
 try{
  const declared=serviceMemoryHeadroomBytesV1({environment:{SEMIO_GRAPH_MEMORY_LIMIT_MB:"10000"}});
  expect(declared).toBeLessThan(10000*MB-9000*MB);
  expect(gate.headroomBytes()).toBeLessThan(0);
  const governor=createGraphParentMemoryGovernorV1({workers:2,headroom:()=>1e12});expect(governor).toBeDefined();
 }finally{useFootprintGateV1(undefined);}
 expect(footprintGateV1()).toBeUndefined();
});

test("the wave settling logs ONE line per language with the footprint before and after and survives every failure",async()=>{
 const lines:string[]=[];let held=8000*MB,dropped=0;
 const meter=createFootprintMeterV1({read:async()=>held,ttlMilliseconds:0});
 const result=await settleWaveMemoryV1({generation:12,release:()=>{dropped++;},meter,managed:()=>300*MB,grantBytes:()=>10462*MB,collect:()=>{held=1500*MB;},heapCensus:async()=>undefined,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 expect(result).toEqual({beforeMegabytes:8000,afterMegabytes:1500,managedMegabytes:300,grantMegabytes:10462});expect(dropped).toBe(1);
 expect(lines).toEqual(["graph: wave 12 settled from 8000 MB to 1500 MB (managed 300 MB, grant 10462 MB)","Graph: Welle 12 beruhigt von 8000 MB auf 1500 MB (verwaltet 300 MB, Zusage 10462 MB)"]);
 expect(await settleWaveMemoryV1({generation:1,release:()=>{throw Error("boom");},meter,report:()=>undefined})).toBeUndefined();
});

test("releaseRuntimeMemoryV1 collects, shrinks and collects again, in that order",()=>{
 const order:string[]=[];releaseRuntimeMemoryV1({collect:()=>order.push("collect"),shrink:()=>order.push("shrink")});expect(order).toEqual(["collect","shrink","collect"]);
});

const SIZE={projects:5000,waves:20,index:160*MB,rowsPerProject:300};
async function syntheticRead(settle:boolean){
 const meter=createFootprintMeterV1({ttlMilliseconds:0}),per=SIZE.projects/SIZE.waves,footprints:number[]=[],heaps:number[]=[];
 let kept:unknown[][]=[],persisted=0;
 for(let wave=1;wave<=SIZE.waves;wave++){
  const index=Buffer.alloc(SIZE.index,wave&255),patched=Buffer.from(index),packed=Buffer.from(patched.subarray(0,SIZE.index>>3));
  const rows:unknown[][]=[];
  for(let project=0;project<per;project++){const one:unknown[]=[];for(let row=0;row<SIZE.rowsPerProject;row++)one.push({kind:"file",path:`p${wave}/${project}/${row}/${"x".repeat(48)}`,fact:{size:row,digest:"d".repeat(40)+row}});rows.push(one);}
  persisted+=packed.length;
  kept=settle?rows:kept.concat(rows);
  if(settle)releaseRuntimeMemoryV1();
  footprints.push(Math.round((await meter.refresh())/MB));
  Bun.gc(true);heaps.push(Math.round(process.memoryUsage().heapUsed/MB));
 }
 return{footprints,heaps,persisted,kept:kept.length};
}

test("a synthetic read of 5000 projects over 20 waves in one process holds a flat footprint from wave 3 on once every wave settles",async()=>{
 const settled=await syntheticRead(true);
 const tail=settled.footprints.slice(2),floor=Math.min(...tail),ceiling=Math.max(...tail);
 console.log("[R35] settled footprint MB per wave:",settled.footprints.join(" "));console.log("[R35] settled managed heap MB per wave:",settled.heaps.join(" "));
 expect(settled.footprints.length).toBe(20);
 expect(ceiling-floor).toBeLessThan(Math.max(320,floor*1.5));
 expect(ceiling).toBeLessThan(900);
 expect(Math.max(...settled.heaps.slice(2))-Math.min(...settled.heaps.slice(2))).toBeLessThan(80);
},180000);

test("without the settling the same read keeps all waves and its managed heap grows with them (control)",async()=>{
 const retained=await syntheticRead(false);
 console.log("[R35] retained footprint MB per wave:",retained.footprints.join(" "));
 expect(retained.kept).toBe(SIZE.projects);
 console.log("[R35] retained managed heap MB per wave:",retained.heaps.join(" "));expect(retained.heaps[19]).toBeGreaterThan(100);expect(retained.heaps[19]).toBeGreaterThan(retained.heaps[2]*3);
},180000);

test("the closure settle asks the gate of the process before every derivation, registers a reliever that drops the pass round while it runs and removes it afterwards",async()=>{
 const asked:string[]=[],relievers=new Set<()=>unknown>();let dropped=0;
 const gate={admit:async()=>{asked.push("admit");return"run" as const;},relieve:(reliever:()=>unknown)=>{relievers.add(reliever);return()=>relievers.delete(reliever);},settle:async()=>0,headroomBytes:()=>Infinity,footprintBytes:()=>0,grantBytes:()=>undefined,stats:()=>({admitted:0,settled:0,waits:0,waitedMilliseconds:0,peakBytes:0})};
 useFootprintGateV1(gate);
 try{
  const rows:GenerationRowV1[]=Array.from({length:6},(_,index)=>({kind:"derived",path:`q${index}`,fact:"answer"})),names=rows.map(derivedOwnerNameV1);
  const index=createFactIndexV1(new Map([["holder",rows],...rows.map((row,at)=>[names[at],[row,{kind:"file",path:`s${at}`,fact:"h"}]] as[string,GenerationRowV1[]])]));
  let seen=0;
  const derive=async(_row:unknown,_signal:unknown,pass:{round?:Map<string,unknown>})=>{pass.round??=new Map([["k",1]]);seen++;if(seen===3){for(const reliever of relievers)reliever();dropped=pass.round===undefined?1:0;}return{fact:"answer",rows:[{kind:"file",path:"s",fact:"h"}]};};
  await createClosureSettlerV1().settle({pseudoRows:suspectRowsV1(index,new Set(names)),derive:derive as never,hold:async()=>true});
  expect(asked.length).toBeGreaterThanOrEqual(rows.length);expect(dropped).toBe(1);expect(relievers.size).toBe(0);
 }finally{useFootprintGateV1(undefined);}
});

test("binding a capacity grant to the process installs the gate with that grant, unbinding removes it",()=>{
 try{
  useCapacityGrantV1(()=>({handle:{reservedMegabytes:2048},limit:{memoryMegabytes:1024}}));
  expect(footprintGateV1()?.grantBytes()).toBe(1024*MB);
  useCapacityGrantV1(undefined);expect(footprintGateV1()).toBeUndefined();
 }finally{useCapacityGrantV1(undefined);}
});

test("the census names the heap by counters without walking it, in both languages, and survives a runtime without counters",async()=>{
 const stats=async()=>({heapSize:300*MB,extraMemorySize:120*MB,objectCount:5000000,objectTypeCounts:{Array:3000000,Map:900,String:1200000,Object:800000,Set:10,Function:5}});
 expect(await heapCensusV1({stats,types:3})).toEqual({heapMegabytes:300,extraMegabytes:120,objects:5000000,types:"Array 3000000, String 1200000, Object 800000"});
 expect(await heapCensusV1({stats:async()=>{throw Error("none");}})).toBeUndefined();
 const real=await heapCensusV1();expect(real?.objects).toBeGreaterThan(0);
 const lines:string[]=[];let held=900*MB;
 const meter=createFootprintMeterV1({read:async()=>held,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{held=300*MB;},heapCensus:()=>heapCensusV1({stats}),report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 await gate.admit();
 expect(lines).toContain("graph: heap census at 900 MB: managed 300 MB, external 120 MB, 5000000 objects; most numerous: Array 3000000, String 1200000, Object 800000, Map 900, Set 10, Function 5");
 expect(lines.some(line=>line.startsWith("Graph: Heap-Zählung bei 900 MB: verwaltet 300 MB, extern 120 MB, 5000000 Objekte; am zahlreichsten: Array 3000000"))).toBe(true);
});

test("the watcher settles a process whose footprint passed the soft limit while no derivation asks, and stops with the gate",async()=>{
 const time=clock(),calls:number[]=[];let held=700*MB,tick:(()=>unknown)|undefined,timerCleared=0;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{calls.push(held);held=200*MB;},now:time.now,setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>{timerCleared++;}});
 gate.watch();gate.watch();expect(typeof tick).toBe("function");
 await tick!();expect(calls).toEqual([700*MB]);
 held=900*MB;await meter.refresh();await tick!();expect(calls.length).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.settleMinimumMilliseconds);await tick!();expect(calls.length).toBe(2);
 gate.stop();gate.stop();expect(timerCleared).toBe(1);
 useFootprintGateV1(gate);useFootprintGateV1(undefined);
});

test("the trace appends every line with its time to a file of this process, stops at its bound and never throws",()=>{
 const directory=mkdtempSync(join(tmpdir(),"footprint-trace-"));
 try{
  const trace=createFootprintTraceV1({directory:join(directory,"t"),pid:4242,maximumBytes:120,now:()=>"2026-11-15T09:00:00.000Z"});
  trace("graph: wave 1 settled from 9000 MB to 900 MB");trace("graph: wave 2 settled from 9000 MB to 900 MB");trace("graph: wave 3 settled from 9000 MB to 900 MB");trace("graph: wave 4 settled from 9000 MB to 900 MB");
  const text=readFileSync(trace.path,"utf8");
  expect(trace.path.endsWith("4242.log")).toBe(true);expect(text.startsWith("2026-11-15T09:00:00.000Z graph: wave 1 settled")).toBe(true);expect(text.split("\n").filter(Boolean).length).toBeLessThan(4);
  expect(()=>createFootprintTraceV1({directory:"/proc/none/x"})("x")).not.toThrow();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the watcher writes a heartbeat row every interval, also while the footprint is far below the soft limit",async()=>{
 const time=clock(),rows:{footprintBytes:number;grantBytes:number}[]=[];let tick:(()=>unknown)|undefined;
 const meter=createFootprintMeterV1({read:async()=>100*MB,now:time.now,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,now:time.now,heartbeat:row=>rows.push(row),setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>undefined});
 gate.watch();await tick!();await tick!();expect(rows.length).toBe(1);expect(rows[0]!.footprintBytes).toBe(100*MB);expect(rows[0]!.grantBytes).toBe(1000*MB);
 time.advance(FOOTPRINT_POLICY_V1.heartbeatMilliseconds);await tick!();expect(rows.length).toBe(2);gate.stop();
});
