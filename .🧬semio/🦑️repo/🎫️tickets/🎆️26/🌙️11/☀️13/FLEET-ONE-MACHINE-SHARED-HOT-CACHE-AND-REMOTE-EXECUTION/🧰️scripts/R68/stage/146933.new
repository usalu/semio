import Ajv from "ajv";
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {OWNERSHIP_WALK_POLICY_V1 as P,OWNERSHIP_WALK_TEXTS_V1 as TEXTS,ownershipWalkLineV1,ownershipWalkStartV1,ownershipWalkTickV1} from "../🟦️.ts";

/** 🕓️ Contract: a walk that takes an hour tells its count and its speed every half minute and saves its settled listings every two minutes, so a try that is stopped is followed by one that goes on. The fixture is language-agnostic; a Python oracle replays it. */
const ajv=new Ajv({strict:false});
const replay=()=>{let state=ownershipWalkStartV1(0);return fixture.steps.map(step=>{const tick=ownershipWalkTickV1(state,{nowMs:step.nowMs,operation:step.operation,count:step.count});state=tick.state;return tick;});};

test("the fixture and the policy are valid against the schema, and both languages have the same placeholders",()=>{
 expect(ajv.validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(ajv.validate({$ref:"#/definitions/Policy",definitions:schema.definitions},P)).toBe(true);
 const holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 expect(holes(TEXTS.de.report)).toEqual(holes(TEXTS.en.report));
});

test("reports and checkpoints fall as the fixture says",()=>{
 const ticks=replay();
 fixture.steps.forEach((step,index)=>{expect(ticks[index]!.report!==undefined,step.name).toBe(step.report);expect(ticks[index]!.checkpoint===true,step.name).toBe(step.checkpoint);});
});

test("the Python oracle replays the steps and agrees on every number",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {report:boolean;checkpoint:boolean;elapsedSeconds?:number;perSecond?:number;recentPerSecond?:number}[];
 expect(oracle).toEqual(replay().map(tick=>({report:tick.report!==undefined,checkpoint:tick.checkpoint===true,...tick.report===undefined?{}:{elapsedSeconds:tick.report.elapsedSeconds,perSecond:tick.report.perSecond,recentPerSecond:tick.report.recentPerSecond}})));
});

test("an hour-long walk of the measured shape (7 700 steps per second falling to 800) prints a line every half minute and checkpoints about every two minutes",()=>{
 let state=ownershipWalkStartV1(0),count=0,reports=0,checkpoints=0;
 for(let second=1;second<=3600;second++){count+=Math.round(7700-6900*Math.min(1,second/1800));const tick=ownershipWalkTickV1(state,{nowMs:second*1000,operation:"frontier",count});state=tick.state;if(tick.report)reports++;if(tick.checkpoint)checkpoints++;}
 expect(reports).toBe(120);expect(checkpoints).toBe(30);
});

test("the human line is English and German and carries every number",()=>{
 const report={operation:"frontier",completedOperations:477056,elapsedSeconds:62,perSecond:7694,recentPerSecond:5000};
 expect(ownershipWalkLineV1(report,"en")).toBe("Ownership walk: 477056 steps in 62 s, 7694 per second since the start and 5000 per second lately (frontier).");
 expect(ownershipWalkLineV1(report,"de")).toContain("477056 Schritte in 62 s");
});
