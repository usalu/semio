/** ⏲️ Contract: the time of the owner wave and of the closure settlement is attributed to named steps in the trace. The stage clock sums measurements, the owner clock charges steps to owners, names the slow ones, rolls up every fixed number of owners, and attributes the part no step covers to `other`; a settlement writes one `settle` line that names its counts and stages.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createFootprintTraceV1,createOwnerClockV1,createStageClockV1,noteTraceV1}from"../../🟨️.mjs";
import{createClosureSettlerV1}from"../../../../🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🕸️graph/🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
import{createFactIndexV1}from"../../../../🕸️graph/🕰️generations/🟨️.mjs";

test("the stage clock sums the milliseconds and the calls of every stage in the order the stages first ran, also when the work fails",async()=>{
 let time=0;const clock=createStageClockV1({now:()=>time});
 await clock.time("ask",async()=>{time+=30;});await clock.time("parts",async()=>{time+=5;});await clock.time("ask",async()=>{time+=12;});
 await expect(clock.time("flush",async()=>{time+=7;throw Error("boom");})).rejects.toThrow("boom");
 clock.add("support",100);
 expect(clock.totals()).toEqual({ask:42,parts:5,flush:7,support:100});
 expect(clock.text()).toBe("ask=42ms/2 parts=5ms/1 flush=7ms/1 support=100ms/1");
});

test("the owner clock names a slow owner with its steps, charges the rest to other, ignores steps of owners it does not know and rolls up the average per step and the concurrency",()=>{
 let time=0;const lines:string[]=[],clock=createOwnerClockV1({now:()=>time,trace:line=>lines.push(line),slowMilliseconds:1000,rollupOwners:3});
 clock.add("ghost","queue",5);
 clock.begin("a");clock.add("a","queue",100);clock.add("a","farm",600);time+=1000;clock.finish("a",{running:4});
 expect(lines).toEqual(["owner a wall=1000ms queue=100ms farm=600ms other=300ms"]);
 expect(clock.get("a","farm")).toBe(0);clock.begin("b");clock.add("b","farm",40);clock.add("b","farm",2);expect(clock.get("b","farm")).toBe(42);time+=100;clock.finish("b",{running:2});
 clock.begin("c");clock.add("c","queue",30);time+=200;clock.finish("c",{running:6});
 expect(lines.length).toBe(2);
 expect(lines[1]).toBe("owners n=3 wall.avg=433ms wall.p90=1000ms wall.max=1000ms running.avg=4.0 running.max=6 queue=43ms farm=214ms other=176ms");
 clock.begin("d");time+=10;clock.finish("d");clock.finish("never-begun");
 expect(lines.length).toBe(2);
});

test("noteTraceV1 appends one line with the time to the trace of the process",()=>{
 const directory=mkdtempSync(join(tmpdir(),"clocks-trace-"));
 try{
  const trace=createFootprintTraceV1({directory,pid:4242,now:()=>"T"});noteTraceV1("settle closures=3",trace);
  expect(readFileSync(trace.path,"utf8")).toBe("T settle closures=3\n");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a settlement of closures writes one settle line with the counts and the stages to the trace of the process",async()=>{
 const names=["q0","q1","q2"],row=(name:string)=>({kind:"derived",path:name,fact:"old"}) as never;
 const index=createFactIndexV1(new Map([["holder",names.map(row)],...names.map(name=>[derivedOwnerNameV1(row(name)),[row(name),{kind:"file",path:"own/"+name,fact:"v"}]] as never)]));
 const suspects=new Set(names.map(name=>derivedOwnerNameV1(row(name)))),store=mkdtempSync(join(tmpdir(),"clocks-store-")),before=existsSync(createFootprintTraceV1().path)?readFileSync(createFootprintTraceV1().path,"utf8").length:0;
 try{
  const settler=createClosureSettlerV1({store:createNodeStoreV1({root:store})});
  const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive:async()=>({fact:"old",parts:[[{kind:"file",path:"own/x",fact:"v"}]]}),hold:async()=>true});
  expect(settled.held.size+settled.kept.size+settled.changed.size).toBe(3);
  const text=readFileSync(createFootprintTraceV1().path,"utf8").slice(before),line=text.split("\n").find(each=>each.includes(" settle closures="));
  expect(line).toBeDefined();expect(line).toContain("closures=3");expect(line).toContain("summary=none");expect(line).toMatch(/wall=\d+ms/u);expect(line).toContain("ask=");
 }finally{rmSync(store,{recursive:true,force:true});}
});
import{existsSync}from"node:fs";
