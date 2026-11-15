/** ⏱️ Contract: the row of a published wave names where its seconds went, stage by stage (prime, plan, compute with its parts, fragments, patch, pack, publish), as milliseconds under language-neutral names; the row without stages stays valid, a row with damaged stages reads as nothing, and the log line words the stages in English and German. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{GRAPH_WAVE_POLICY_V1,announceGraphWaveV1,createGraphWaveMeterV1,parseGraphWaveRowV1,renderGraphWaveRowV1,renderGraphWaveTextV1}from"../../🟨️.mjs";

const validate=new Ajv({strict:false}).compile({...schema.definitions.Row,definitions:schema.definitions});
const stages={prime:20,plan:2210,compute:107400,"compute.setup":31000,"compute.projects":61400,"compute.tail":15000,fragments:600,patch:22300,pack:900,publish:2100};

test("the meter hands over the stages the build told with the wave, rounded, then starts the next wave without them",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0});
 meter.progress({done:48,total:4700,phases:{...stages,"": 5,bad:Number.NaN,negative:-4,fraction:1.4}} as never);
 const row=meter.take({generation:9,parentMegabytes:1000});
 expect(row.phases).toEqual({...stages,fraction:1});
 expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
 const next=meter.take({generation:10,parentMegabytes:1000});
 expect("phases" in next).toBe(false);expect(validate(next),JSON.stringify(validate.errors)).toBe(true);
});

test("a row with stages survives its stderr line; a row without stages is still a row; damaged stages read as nothing",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0});meter.progress({done:1,total:2,phases:stages} as never);
 const row=meter.take({generation:3,parentMegabytes:1}),line=renderGraphWaveRowV1(row);
 expect(parseGraphWaveRowV1(line)).toEqual(row);
 const bare=meter.take({generation:4,parentMegabytes:1});expect(parseGraphWaveRowV1(renderGraphWaveRowV1(bare))).toEqual(bare);
 const marker=GRAPH_WAVE_POLICY_V1.marker;
 for(const phases of [null,[],{a:"x"},{a:-1},{a:1.5},Object.fromEntries(Array.from({length:GRAPH_WAVE_POLICY_V1.maximumPhases+1},(_,at)=>["p"+at,1]))])expect(parseGraphWaveRowV1(marker+JSON.stringify({...row,phases}))).toBeUndefined();
 expect(parseGraphWaveRowV1(marker+JSON.stringify({...row,extra:1}))).toBeUndefined();
});

test("the log line names the stages in seconds in both languages and keeps the old line when there are none",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0});meter.progress({done:1,total:2,phases:stages} as never);
 const row=meter.take({generation:3,parentMegabytes:1}),bare=meter.take({generation:4,parentMegabytes:1});
 expect(renderGraphWaveTextV1(row,"en")).toEndWith(", phases: prime 0 s, plan 2 s, compute 107 s, compute.setup 31 s, compute.projects 61 s, compute.tail 15 s, fragments 1 s, patch 22 s, pack 1 s, publish 2 s");
 expect(renderGraphWaveTextV1(row,"de")).toEndWith(", Phasen: prime 0 s, plan 2 s, compute 107 s, compute.setup 31 s, compute.projects 61 s, compute.tail 15 s, fragments 1 s, patch 22 s, pack 1 s, publish 2 s");
 expect(renderGraphWaveTextV1(bare,"en")).not.toContain("phases");expect(renderGraphWaveTextV1(bare,"de")).not.toContain("Phasen");
});

test("a published wave announces the stages of its info",async()=>{
 const lines:string[]=[],meter=createGraphWaveMeterV1({now:()=>0});
 const row=await announceGraphWaveV1({generation:5,info:{done:48,total:96,phases:{plan:1500}} as never,environment:{SEMIO_GRAPH_WAVE_ROWS:"1"},write:line=>{lines.push(line);},meter,observe:()=>null,pid:1});
 expect(row?.phases).toEqual({plan:1500});expect(parseGraphWaveRowV1(lines[0]!)?.phases).toEqual({plan:1500});
});
