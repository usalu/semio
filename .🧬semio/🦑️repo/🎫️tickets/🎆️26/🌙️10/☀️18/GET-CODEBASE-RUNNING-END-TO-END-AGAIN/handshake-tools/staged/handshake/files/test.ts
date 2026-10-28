import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import fixture from "./🔣️.json";
import {bazelBirthHandshakeV1} from "../../🎬️executor/🟦️.ts";

type Row=(typeof fixture.rows)[number];
type Timer={at:number;id:number;run:()=>void};

const simulate=(row:Row)=>{
 let time=0,sequence=0;const queue:Timer[]=[],failures:{at:number;key:string}[]=[],checks:{at:number;threw:boolean}[]=[];
 const ratio=(at:number)=>[...row.ratios].reverse().find(step=>step[0]!<=at)![1]!;
 const schedule=(run:()=>void,milliseconds:number)=>{const timer={at:time+milliseconds,id:sequence++,run};queue.push(timer);return timer;};
 const handshake=bazelBirthHandshakeV1(fixture.budget,0,{now:()=>time,epoch:()=>fixture.epoch+time,source:()=>({ratio:ratio(time),load:ratio(time)*10,cores:10,source:"loadavg"}),schedule,cancel:handle=>{const index=queue.indexOf(handle as Timer);if(index>=0)queue.splice(index,1);}});
 handshake.arm(row.deadline===null?null:fixture.epoch+row.deadline,error=>failures.push({at:time,key:(error as {key:string}).key}));
 for(const [at,kind] of row.events)schedule(()=>{if(kind==="waiting")handshake.waiting();else if(kind==="settle")handshake.settle();else{try{handshake.check();checks.push({at,threw:false});}catch{checks.push({at,threw:true});}}},at);
 for(;;){queue.sort((a,b)=>a.at-b.at||a.id-b.id);const next=queue[0];if(!next||next.at>row.until)break;queue.shift();time=next.at;next.run();}
 return {failures,checks,handshake};
};

for(const row of fixture.rows)test(`birth handshake work clock: ${row.name}`,()=>{
 const {failures,checks}=simulate(row);
 if(row.fails===null)expect(failures).toEqual([]);
 else{expect(failures.length).toBe(1);expect(failures[0]!.key).toBe(row.fails[0]);expect(Math.abs(failures[0]!.at-Number(row.fails[1]))).toBeLessThanOrEqual(row.tolerance);}
 expect(checks).toEqual(row.checks.map(([at,threw])=>({at,threw})));
});

test("a settled or waiting handshake can never fail the birth, however late its clocks run",()=>{
 for(const phase of ["waiting","settle"] as const){
  const row={...fixture.rows[0]!,ratios:[[0,12]],events:[[100,phase],[3600000,"check"]],deadline:null,until:7200000,fails:null,checks:[[3600000,false]]} as unknown as Row;
  const {failures,checks,handshake}=simulate(row);
  expect(failures).toEqual([]);expect(checks).toEqual([{at:3600000,threw:false}]);expect(handshake.phase()).toBe(phase==="waiting"?"waiting":"settled");
 }
});

test("a throwing host probe counts as an idle host instead of failing the birth",()=>{
 let time=0;const queue:Timer[]=[];const failures:unknown[]=[];
 const handshake=bazelBirthHandshakeV1(fixture.budget,0,{now:()=>time,source:()=>{throw new Error("probe");},schedule:(run,milliseconds)=>{const timer={at:time+milliseconds,id:0,run};queue.push(timer);return timer;},cancel:handle=>{const index=queue.indexOf(handle as Timer);if(index>=0)queue.splice(index,1);}});
 handshake.arm(null,error=>failures.push(error));
 time=fixture.budget-1;expect(()=>handshake.check()).not.toThrow();time=fixture.budget;expect(()=>handshake.check()).toThrow("birth-handshake-deadline-exhausted");
});

test("the action birth polls the handshake instead of keeping a second wall clock",()=>{
 const source=readFileSync(new URL("../../🎬️executor/🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain("initial=()=>handshake.check()");
 expect(source.match(/maximumHandshakeMilliseconds/gu)?.length).toBe(1);
 expect(source).not.toMatch(/performance\.now\(\)-started>=/u);
});
