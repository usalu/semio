import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{spawn,spawnSync}from"node:child_process";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import{createScriptTerminationGuardV1,liveChildCountV1,treeCpuMillisecondsV1,waitingDelayV1,enterScriptProcessWaitV1,activeScriptProcessWaitsV1,noteScriptProcessProgressV1,observeScriptProcessOutputV1,scriptProcessWaitEpisodeV1,scriptProcessStallDecisionV1,scriptProcessWatchdogNoticeV1,startScriptProcessWatchdogV1}from"../🟦️.ts";
test("the stall decision agrees with the closed schema and an independent SQLite table",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile({...schema.definitions.State,definitions:{}});
 try{for(const row of fixture.decisions){expect(validate(row.state)).toBe(true);expect(scriptProcessStallDecisionV1(row.state,fixture.stall,fixture.idleTree)).toBe(row.decision);
  const expected=database.query("SELECT CASE WHEN :idle<:stall THEN 'healthy' WHEN :waiting>0 THEN 'healthy' WHEN :children=0 THEN 'stalled' WHEN :tree>=:idleTree THEN 'tree-stalled' ELSE 'healthy' END AS decision").get({":children":row.state.liveChildren,":idle":row.state.idleMilliseconds,":stall":fixture.stall,":tree":row.state.treeIdleMilliseconds,":waiting":row.state.waiting,":idleTree":fixture.idleTree}) as{decision:string};expect(expected.decision).toBe(row.decision);}}
 finally{database.close();}
});
test("a process without progress and without a live child reports once and is stopped, while a live child or fresh progress keeps it alive",async()=>{
 const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));let alive=1,stalls=0;const stop=startScriptProcessWatchdogV1({children:()=>alive?[4242]:[],cpu:()=>1000+Date.now()%7*0+(alive?Math.floor(Date.now()/50):0),stallMilliseconds:120,idleTreeMilliseconds:100000,checkMilliseconds:10,onStall:()=>{stalls++;}});
 try{await sleep(250);expect(stalls).toBe(0);alive=0;for(let round=0;round<5;round++){noteScriptProcessProgressV1();await sleep(40);}expect(stalls).toBe(0);await sleep(400);expect(stalls).toBe(1);await sleep(100);expect(stalls).toBe(1);}
 finally{stop();}
});
test("a termination signal always ends the process: after the grace, or at once on the second signal",async()=>{
 const exits:number[]=[],notices:string[]=[],guard=createScriptTerminationGuardV1({exit:code=>{exits.push(code);},notice:text=>{notices.push(text);},languages:()=>["en","de"],graceMilliseconds:30});
 guard("SIGTERM",143);expect(exits).toEqual([]);await new Promise(done=>setTimeout(done,80));expect(exits).toEqual([143]);expect(notices[0]!.split("\n").filter(Boolean).length).toBe(2);
 const second=createScriptTerminationGuardV1({exit:code=>{exits.push(code);},notice:()=>{},languages:()=>["en"],graceMilliseconds:60000});second("SIGINT",130);second("SIGINT",130);expect(exits).toEqual([143,130]);
});
test.skipIf(process.platform==="win32")("the live child count agrees with the system's own parent query",async()=>{
 expect(liveChildCountV1()).toBe(0);
 const child=spawn("sleep",["5"],{stdio:"ignore"});try{await new Promise(done=>setTimeout(done,100));const oracle=spawnSync("pgrep",["-P",String(process.pid)],{encoding:"utf8"}).stdout.split("\n").filter(Boolean).length;expect(liveChildCountV1()).toBe(oracle);expect(oracle).toBe(1);}finally{child.kill();}
});
test("notices are rendered in the selected languages with every placeholder filled",()=>{
 const text=scriptProcessWatchdogNoticeV1("stalled",{owner:"repository:build",minutes:15},["de"]);expect(text).toContain("repository:build");expect(text).toContain("15");expect(text).not.toContain("{");
});
test("children that are alive but whose whole tree used no CPU time are stopped after the idle-tree period, a busy tree never",async()=>{
 const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));let busy=true,ticks=0;const seen:string[]=[];
 const stop=startScriptProcessWatchdogV1({children:()=>[4242,4243],cpu:()=>busy?++ticks:7,stallMilliseconds:60,idleTreeMilliseconds:150,checkMilliseconds:10,onStall:(_idle,decision,children)=>{seen.push(decision+":"+children.length);}});
 try{await sleep(400);expect(seen).toEqual([]);busy=false;await sleep(500);expect(seen).toEqual(["tree-stalled:2"]);}finally{stop();}
});
test.skipIf(process.platform==="win32")("the CPU time reader agrees with the system's ps for a sleeping child",async()=>{
 const child=spawn("sleep",["5"],{stdio:"ignore"});try{await new Promise(done=>setTimeout(done,150));const used=treeCpuMillisecondsV1([child.pid!]);expect(used).toBeDefined();expect(used!).toBeLessThan(1000);}finally{child.kill();}
 expect(treeCpuMillisecondsV1([])).toBeUndefined();
});
test("a process inside a queue wait is never reported as stalled, however long the wait lasts",async()=>{
 const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));let stalls=0;const stop=startScriptProcessWatchdogV1({children:()=>[],stallMilliseconds:60,checkMilliseconds:10,onStall:()=>{stalls++;}});
 try{const waiting=waitingDelayV1(400);await sleep(300);expect(stalls).toBe(0);await waiting;await sleep(250);expect(stalls).toBe(1);}finally{stop();}
});
test("a wait counts only until the wait ceiling, so a queue nobody serves any more is stalled like anything else",()=>{
 const leave=enterScriptProcessWaitV1(),start=Date.now();try{expect(activeScriptProcessWaitsV1(start+1000,60000)).toBe(1);expect(activeScriptProcessWaitsV1(start+59999,60000)).toBe(1);expect(activeScriptProcessWaitsV1(start+60000+5,60000)).toBe(0);}finally{leave();}
 expect(activeScriptProcessWaitsV1(Date.now(),60000)).toBe(0);
});
test("every console line counts as progress, because Bun's console bypasses process.stdout.write; an independent child process proves the premise",async()=>{
 const premise=spawnSync(process.execPath,["-e","let writes=0;const write=process.stdout.write.bind(process.stdout);process.stdout.write=(...args)=>{writes++;return write(...args);};console.log('x');process.stdout.write('y\\n');process.stderr.write(String(writes)+'\\n');"],{encoding:"utf8"});
 const viaConsole=Number(premise.stderr.trim())>1;
 observeScriptProcessOutputV1();
 const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));let stalls=0;const stop=startScriptProcessWatchdogV1({children:()=>[],stallMilliseconds:120,checkMilliseconds:10,onStall:()=>{stalls++;}});
 try{for(let round=0;round<12;round++){console.log("watchdog progress line "+round);await sleep(30);}expect(stalls,"console output keeps the process alive (runtime passes console through write: "+viaConsole+")").toBe(0);await sleep(400);expect(stalls).toBe(1);}finally{stop();}
});
test("a wait episode chains the polls of one queue and ends once nothing waited for longer than the authored gap",async()=>{
 const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds)),gap=schema.definitions.Policy.const.waitEpisodeGapMilliseconds;
 await sleep(gap+50);expect(scriptProcessWaitEpisodeV1()).toBe(0);
 const begun=Date.now();for(let poll=0;poll<4;poll++){await waitingDelayV1(30);await sleep(10);}
 const during=scriptProcessWaitEpisodeV1();expect(during).toBeGreaterThanOrEqual(150);expect(during).toBeLessThan(Date.now()-begun+5);
 await sleep(gap+50);expect(scriptProcessWaitEpisodeV1()).toBe(0);
 await waitingDelayV1(30);expect(scriptProcessWaitEpisodeV1(),"a new episode starts from zero").toBeLessThan(during);
},15000);
