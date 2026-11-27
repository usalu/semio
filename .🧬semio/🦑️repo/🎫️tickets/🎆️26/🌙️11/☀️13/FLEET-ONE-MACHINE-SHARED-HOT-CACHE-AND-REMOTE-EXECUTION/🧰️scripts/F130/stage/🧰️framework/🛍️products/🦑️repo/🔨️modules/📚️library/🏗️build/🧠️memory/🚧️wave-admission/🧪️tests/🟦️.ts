import{expect,test}from"bun:test";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{FOOTPRINT_MESSAGES_V1,FOOTPRINT_POLICY_V1,createFootprintGateV1,createFootprintMeterV1,footprintOverGrantV1}from"../../🦶️footprint/🟨️.mjs";
import{createMemoryGovernorV1}from"../../🟨️.mjs";
import{WAVE_ADMISSION_MESSAGES_V1,WAVE_ADMISSION_POLICY_V1,createHostPressureReaderV1,createWaveAdmissionV1,decideWaveAdmissionV1,renderWaveAdmissionMessageV1}from"../🟨️.mjs";
import vectors from"./🔣️.json";

/** 🚧️ Contract: above 100 % of its grant a process admits no new project, with no cooldown; a host that pages NOW (pressure, swap-out rate, available floor, never the swap used) holds the start of a wave; both wait with bilingual progress, stay cancellable and, after a bounded wait, go on at one project at a time, never zero.
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const MB=1048576,P=WAVE_ADMISSION_POLICY_V1,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number,signal?:AbortSignal)=>{signal?.throwIfAborted();at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};
const hostOf=(row:{pressure:string;available:number;swapOut:number|null;swapUsed:number|null})=>({pressure:row.pressure as"normal",availableMegabytes:row.available,swapOutMegabytesPerSecond:row.swapOut,swapUsedMegabytes:row.swapUsed});
const calm={pressure:"normal" as const,availableMegabytes:8000,swapOutMegabytesPerSecond:0,swapUsedMegabytes:9830};

test("policy and messages equal the schema file and validate against it; both languages carry the same placeholders",()=>{
 expect(WAVE_ADMISSION_POLICY_V1).toEqual(schema.definitions.Policy.const);expect(WAVE_ADMISSION_MESSAGES_V1).toEqual(schema.definitions.Messages.const);
 const ajv=new Ajv({strict:false});expect(ajv.validateSchema(schema)).toBe(true);
 expect(Object.keys(WAVE_ADMISSION_MESSAGES_V1.en)).toEqual(Object.keys(WAVE_ADMISSION_MESSAGES_V1.de));
 for(const key of Object.keys(WAVE_ADMISSION_MESSAGES_V1.en))expect(holes(WAVE_ADMISSION_MESSAGES_V1.de[key as"hostBusy"])).toEqual(holes(WAVE_ADMISSION_MESSAGES_V1.en[key as"hostBusy"]));
 expect(Object.keys(FOOTPRINT_MESSAGES_V1.de)).toEqual(Object.keys(FOOTPRINT_MESSAGES_V1.en));expect(FOOTPRINT_POLICY_V1.overPercent).toBe(100);
 expect(P.reducedWidth).toBeGreaterThanOrEqual(1);
});

test("every language-agnostic vector decides as declared, the swap used never holds",()=>{
 for(const row of vectors.decisions)expect(decideWaveAdmissionV1({footprintBytes:row.footprint*MB,grantBytes:row.grant===null?undefined:row.grant*MB,host:row.host===null?undefined:hostOf(row.host as never)}),row.name).toBe(row.expect);
 expect(footprintOverGrantV1({footprintBytes:100*MB,grantBytes:100*MB})).toBe(true);expect(footprintOverGrantV1({footprintBytes:99*MB,grantBytes:100*MB})).toBe(false);expect(footprintOverGrantV1({footprintBytes:NaN,grantBytes:100*MB})).toBe(false);
});

test("the footprint gate has no cooldown over the grant: every admit waits again, while the soft tier keeps its cooldown",async()=>{
 const time=clock();let held=1100*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0}),lines:string[]=[];
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:time.sleep,now:time.now,report:line=>lines.push(line),heapCensus:async()=>undefined,environment:{SEMIO_LANGUAGE:"en,de"}});
 await meter.refresh();expect(gate.overGrant()).toBe(true);
 for(let index=1;index<=3;index++){expect(await gate.admit()).toBe("wait");expect(gate.stats().waits).toBe(index);}
 expect(lines.some(line=>line.startsWith("graph: the process holds 1100 MB, 110 % of its grant of 1000 MB; no new project is admitted"))).toBe(true);expect(lines.some(line=>line.startsWith("Graph: der Prozess hält 1100 MB, 110 %"))).toBe(true);
 held=950*MB;await meter.refresh();expect(gate.overGrant()).toBe(false);
 const soft=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:time.sleep,now:time.now,heapCensus:async()=>undefined});
 expect(await soft.admit()).toBe("wait");const after=time.now();expect(await soft.admit()).toBe("wait");expect(time.now()).toBe(after);expect(soft.stats().waits).toBe(1);
});

function fixture(initial:{footprint:number;host?:typeof calm}){
 const time=clock(),lines:string[]=[],state={footprint:initial.footprint*MB,host:initial.host??calm,settled:0};
 const gate={grantBytes:()=>3072*MB,footprintBytes:()=>state.footprint,settle:async()=>{state.settled++;return state.footprint;}};
 const admission=createWaveAdmissionV1({gate:()=>gate as never,host:{current:()=>state.host,refresh:async()=>state.host},sleep:time.sleep,now:time.now,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 return{time,lines,state,admission};
}

test("over the grant admission stops, the process releases between waves, and it resumes as soon as the footprint is under the grant",async()=>{
 const f=fixture({footprint:4700});let steps=0;
 const sleep=f.time.sleep;
 const admission=createWaveAdmissionV1({gate:()=>({grantBytes:()=>3072*MB,footprintBytes:()=>f.state.footprint,settle:async()=>{f.state.settled++;return f.state.footprint;}}) as never,host:{current:()=>f.state.host,refresh:async()=>f.state.host},sleep:async(milliseconds,signal)=>{await sleep(milliseconds,signal);if(++steps===40)f.state.footprint=2500*MB;},now:f.time.now,report:line=>f.lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 expect(admission.holds()).toBe("overGrant");
 expect(await admission.between()).toBe("admit");
 expect(admission.holds()).toBe("admit");expect(admission.narrow()).toBe(false);
 expect(f.state.settled).toBeGreaterThanOrEqual(1);expect(f.time.now()).toBe(40*P.stepMilliseconds);
 expect(f.lines.some(line=>line.startsWith("graph: no new project is admitted, the process holds 4700 MB, 153 % of its grant of 3072 MB"))).toBe(true);expect(f.lines.some(line=>line.startsWith("Graph: kein neues Projekt wird aufgenommen, der Prozess hält 4700 MB"))).toBe(true);
 expect(f.lines.some(line=>line.startsWith("graph: admission resumed after 40 s"))).toBe(true);
 expect(await admission.between()).toBe("admit");
});

test("a busy host (swap-out rate) waits with progress, then degrades to one project at a time, never zero, and recovers when the host calms",async()=>{
 const f=fixture({footprint:1000,host:{...calm,swapOutMegabytesPerSecond:120}});
 expect(f.admission.holds()).toBe("hostBusy");
 expect(await f.admission.between()).toBe("reduced");
 expect(f.time.now()).toBe(P.hostMaxWaitMilliseconds);expect(f.admission.narrow()).toBe(true);expect(P.reducedWidth).toBe(1);
 expect(f.lines.filter(line=>line.startsWith("graph: no new project is admitted, the host is paging now: swap-out 120 MB/s")).length).toBe(Math.ceil(P.hostMaxWaitMilliseconds/P.progressMilliseconds));
 expect(f.lines.some(line=>line.includes("swap used 9830 MB (information only)"))).toBe(true);expect(f.lines.some(line=>line.startsWith("Graph: kein neues Projekt wird aufgenommen, der Rechner lagert gerade aus"))).toBe(true);
 expect(f.lines.some(line=>line.startsWith("graph: the wait of 120 s ended while hostBusy; going on at 1 project at a time"))).toBe(true);
 const at=f.time.now();expect(await f.admission.between()).toBe("reduced");expect(f.time.now()).toBe(at);
 f.state.host={...calm};expect(f.admission.narrow()).toBe(false);expect(await f.admission.between()).toBe("admit");
});

test("a high swap USED alone, an ungated process and an unknown host sample never hold",async()=>{
 const f=fixture({footprint:500,host:{...calm,swapUsedMegabytes:9830}});expect(await f.admission.between()).toBe("admit");expect(f.time.now()).toBe(0);expect(f.lines).toEqual([]);
 const bare=createWaveAdmissionV1({gate:()=>undefined,host:{current:()=>({...calm,pressure:"critical"}),refresh:async()=>undefined}});expect(bare.holds()).toBe("admit");expect(await bare.between()).toBe("admit");
 const blind=createWaveAdmissionV1({gate:()=>({grantBytes:()=>100*MB,footprintBytes:()=>10*MB,settle:async()=>0}) as never,host:{current:()=>undefined,refresh:async()=>undefined}});expect(blind.holds()).toBe("admit");
});

test("cancellation is honoured while it waits, over the grant and on a busy host",async()=>{
 for(const initial of[{footprint:4000},{footprint:100,host:{...calm,pressure:"critical" as const}}]){
  const f=fixture(initial),controller=new AbortController();
  const admission=createWaveAdmissionV1({gate:()=>({grantBytes:()=>3072*MB,footprintBytes:()=>f.state.footprint,settle:async()=>0}) as never,host:{current:()=>f.state.host,refresh:async()=>f.state.host},sleep:async(milliseconds,signal)=>{f.time.advance(milliseconds);if(f.time.now()>=3000)controller.abort(new Error("stop"));signal?.throwIfAborted();},now:f.time.now});
  await expect(admission.between(controller.signal)).rejects.toThrow("stop");expect(f.time.now()).toBeLessThan(P.hostMaxWaitMilliseconds);
 }
 const pre=new AbortController();pre.abort(new Error("early"));await expect(fixture({footprint:4000}).admission.between(pre.signal)).rejects.toThrow("early");
});

test("the host pressure reader samples through the sampler port once per ttl, keeps the previous reading for the rate, and treats a stale or failed reading as unknown",async()=>{
 const time=clock();const seen:(unknown)[]=[];let fail=false;
 const reader=createHostPressureReaderV1({now:time.now,read:async previous=>{seen.push(previous);if(fail)throw Error("no sampler");return{host:{atMs:time.now(),pressure:"normal",availableMegabytes:5000,swapUsedMegabytes:1,swapOutMegabytesPerSecond:7},swappedOutMegabytes:time.now()} as never;}});
 expect(reader.current()).toBeUndefined();await reader.refresh();expect(reader.current()?.swapOutMegabytesPerSecond).toBe(7);
 for(let index=0;index<20;index++)reader.current();expect(seen.length).toBe(2);
 time.advance(P.sampleTtlMilliseconds);void reader.current();await reader.refresh();expect((seen.at(-1) as{atMs:number}).atMs).toBeGreaterThanOrEqual(0);
 fail=true;time.advance(P.sampleStaleMilliseconds+1);expect(reader.current()).toBeUndefined();await reader.refresh();expect(reader.current()).toBeUndefined();
});

test("the real sampler port answers on this host: pressure, available memory and the swap numbers are present, the rate after the second reading",async()=>{
 const reader=createHostPressureReaderV1({ttlMilliseconds:0}),first=await reader.refresh();
 expect(first===undefined||["normal","warn","critical"].includes(first.pressure)).toBe(true);
 if(first!==undefined){expect(first.availableMegabytes).toBeGreaterThan(0);const second=await reader.refresh();expect(second?.swapOutMegabytesPerSecond===null||typeof second?.swapOutMegabytesPerSecond==="number").toBe(true);}
});

test("the governor holds a new project over the grant even with none running, asks between, and narrows to one at a time",async()=>{
 let over=true,between=0,narrow=true,running=0,peak=0,done=0;
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>1<<30,estimateBytes:MB,hold:()=>over,between:async()=>{between++;over=false;},narrow:()=>narrow});
 const task=async()=>{running++;peak=Math.max(peak,running);await new Promise(resolve=>setTimeout(resolve,250));running--;done++;};
 await Promise.all(Array.from({length:6},()=>governor.run(task)));governor.dispose();
 expect(between).toBeGreaterThanOrEqual(1);expect(done).toBe(6);expect(peak).toBe(1);
 narrow=false;const wide=createMemoryGovernorV1({workers:4,headroom:()=>1<<30,estimateBytes:MB,narrow:()=>false});running=0;peak=0;
 await Promise.all(Array.from({length:6},()=>wide.run(task)));wide.dispose();expect(peak).toBeGreaterThan(1);
});

test("the English line names the numbers",()=>{
 expect(renderWaveAdmissionMessageV1("resumed",{seconds:9,rate:3,pressure:"normal",available:4000},{SEMIO_LANGUAGE:"en"})).toEqual(["graph: admission resumed after 9 s (host swap-out 3 MB/s, pressure normal, available 4000 MB)"]);
});
