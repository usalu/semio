/** 🪦️ Laws of the orphan and open files guard of `bun test` processes: the ceiling follows an independent Python model, the descriptor count equals what `lsof` lists, the soft limit of a real process stops it at the ceiling, a real test process whose runner is killed ends itself and what it started (and survives without the guard), the runner side stops a real process above its ceiling by name (EN and DE) and stops and names a real process a file left behind. */
import{expect,test}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{createInterface}from"node:readline";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import{countOpenDescriptorsV1,createTestProcessWatchV1,describeOpenFilesExceededV1,describeOrphansV1,openFilesCeilingV1,parseTestProcessRowsV1,readTestProcessRowsV1,startTestOrphanGuardV1,type TestOrphansHostV1}from"../🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";

const here=(file:string)=>fileURLToPath(new URL(file,import.meta.url)),posix=process.platform==="darwin"||process.platform==="linux";
const lsofAvailable=posix&&spawnSync("lsof",["-v"],{stdio:"ignore"}).error===undefined;
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const until=async(condition:()=>boolean,milliseconds:number)=>{const end=Date.now()+milliseconds;while(Date.now()<end&&!condition())await sleep(50);return condition();};
const python=(...args:string[])=>spawnSync("python3",["-I",here("./🐍️oracle/🐍️.py"),...args],{encoding:"utf8"}).stdout.trim();
const firstLine=(command:string,args:string[],prefix:string,env:Record<string,string|undefined>=process.env)=>new Promise<{child:ReturnType<typeof spawn>;words:string[]}>((accept,reject)=>{
 const child=spawn(command,args,{stdio:["ignore","pipe","inherit"],env}),lines=createInterface({input:child.stdout!});
 lines.on("line",line=>{if(line.startsWith(prefix)){accept({child,words:line.split(" ").slice(1)});}});
 child.once("exit",()=>reject(new Error("exited before "+prefix)));
});

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Corpus"},fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the open files ceiling of an environment equals the independent Python model on every row",()=>{
 const expected=JSON.parse(python("ceilings",here("./🧫️fixtures/🔣️.json"))) as number[];
 expect(fixture.ceilings.map(row=>openFilesCeilingV1(row.environment))).toEqual(expected);
 expect(expected).toEqual(fixture.ceilings.map(row=>row.ceiling));
});

test.skipIf(!posix)("the process table parser reads the same pids as ps lists, with a start time and group each",async()=>{
 const rows=await readTestProcessRowsV1(),oracle=new Set(spawnSync("ps",["-A","-o","pid="],{encoding:"utf8"}).stdout.split("\n").map(Number).filter(Boolean));
 const seen=new Set(rows.map(row=>row.pid));
 expect(rows.length).toBeGreaterThan(5);
 const missing=[...seen].filter(pid=>!oracle.has(pid)&&alive(pid));
 expect(missing,"every parsed pid is a live process").toEqual([]);
 expect(rows.find(row=>row.pid===process.pid)).toMatchObject({parent:process.ppid,start:expect.stringMatching(/^[A-Za-z]{3} [A-Za-z]{3} \d+ [\d:]{8} \d{4}$/u)});
 expect(parseTestProcessRowsV1("garbage\n  12   1   12 Mon Oct 26 19:03:19 2026 /bin/x y\n")).toEqual([{pid:12,parent:1,group:12,start:"Mon Oct 26 19:03:19 2026",command:"/bin/x y"}]);
});

test.skipIf(!lsofAvailable)("the descriptor count of a real process equals what lsof lists (and Python counts through lsof)",async()=>{
 const{child,words}=await firstLine(process.execPath,[here("./👶️fixture/📜️holder.ts"),String(fixture.orphans.opened)],"READY");
 try{
  const pid=Number(words[0]),library=countOpenDescriptorsV1(pid),oracle=Number(python("descriptors",String(pid)));
  expect(library).toBeDefined();expect(Math.abs(library!-oracle),`library ${library} lsof ${oracle}`).toBeLessThanOrEqual(2);
  expect(library!).toBeGreaterThanOrEqual(fixture.orphans.opened);
 }finally{child.kill("SIGKILL");}
});

test.skipIf(!posix)("the soft limit stops a real process at its ceiling and never raises it",async()=>{
 const run=(ceiling:number)=>JSON.parse(spawnSync(process.execPath,[here("./👶️fixture/📜️limit.ts"),String(ceiling)],{encoding:"utf8"}).stdout) as{limit:number|null;opened:number;code:string};
 for(const ceiling of[300,1024]){const row=run(ceiling);expect(row.limit).toBe(ceiling);expect(row.code).toBe("EMFILE");expect(row.opened).toBeGreaterThan(ceiling-60);expect(row.opened).toBeLessThanOrEqual(ceiling);}
 const raised=spawnSync(process.execPath,["-e",`const{limitOpenFilesV1}=await import(${JSON.stringify(here("../👶️child/🟦️.ts"))});console.log(limitOpenFilesV1(1e9))`],{encoding:"utf8",env:{...process.env}});
 expect(Number(raised.stdout.trim()),"a limit above the current one leaves the current one").toBeLessThanOrEqual(1e9);
});

const childrenOf=(pid:number)=>spawnSync("ps",["-A","-o","pid=,ppid="],{encoding:"utf8"}).stdout.split("\n").map(line=>line.trim().split(/\s+/u).map(Number)).filter(([, parent])=>parent===pid).map(([child])=>child!);
const killedRunner=async(flags:string[],env:Record<string,string|undefined>=process.env)=>{
 const{child,words}=await firstLine(process.execPath,[here("./👶️fixture/📜️runner.ts"),process.execPath,here("./👶️fixture/📜️blocked.ts"),...flags],"CHILD",env),pid=Number(words[0]);
 await until(()=>childrenOf(pid).length>0,8000);
 const helpers=childrenOf(pid);child.kill("SIGKILL");
 return{pid,helpers};
};

test.skipIf(!posix)("a test process whose runner is killed ends itself and what it started within seconds; without the guard it survives",async()=>{
 const guarded=await killedRunner(["--preload",here("../📜️script.ts")]);
 expect(guarded.helpers.length,"the test process was blocked in a child").toBeGreaterThan(0);
 expect(await until(()=>!alive(guarded.pid)&&guarded.helpers.every(helper=>!alive(helper)),8000),"guarded test process and its child ended").toBe(true);
 const control=await killedRunner([],{...process.env,SEMIO_TEST_ORPHAN_GUARD:"off"});
 await sleep(3000);
 const survived=alive(control.pid)&&control.helpers.every(helper=>alive(helper));
 for(const pid of[...control.helpers,control.pid])try{process.kill(pid,"SIGKILL");}catch{}
 expect(survived,"control: without the guard the orphan and its child keep running").toBe(true);
},90000);

const fakeHost=(over:Partial<TestOrphansHostV1>):TestOrphansHostV1=>({rows:async()=>[],descriptors:()=>undefined,terminate:()=>{},sleep:async()=>{},...over});
const row=(pid:number,parent:number,group:number,start="S",command="c")=>({pid,parent,group,start,command});

test("the runner side stops a tree once at 90 % of its ceiling and names the file, the process and the count in English and German",async()=>{
 const stops:string[]=[];let count=100;
 const watch=createTestProcessWatchV1({host:fakeHost({rows:async()=>[row(10,1,10,"A","bun test"),row(11,10,10,"B","helper")],descriptors:pid=>pid===11?count:5}),environment:{LANG:"en_US.UTF-8"},intervalMilliseconds:5});
 const end=watch.watch(10,{file:"a/b.ts",ceiling:1000,terminate:reason=>stops.push(reason),report:()=>{}});
 await sleep(40);expect(stops).toEqual([]);count=899;await sleep(40);expect(stops).toEqual([]);count=900;await sleep(60);
 expect(stops).toEqual(["[tests] a/b.ts holds 900 open files in process 11 (helper) and was stopped at its ceiling of 1000 open files"]);
 expect((await end()).peakDescriptors).toBe(900);
 expect(describeOpenFilesExceededV1("a/b.ts",900,11,"helper",1000,{LANG:"de_DE.UTF-8"})).toBe("[tests] a/b.ts hält 900 offene Dateien im Prozess 11 (helper) und wurde an seiner Obergrenze von 1000 offenen Dateien angehalten");
});

test("what a file left behind is found by tracking, by its group and not by a reused pid; it is stopped and named in English and German",async()=>{
 const killed:number[]=[],said:string[]=[];let table=[row(10,1,10,"A"),row(20,10,10,"B","service one"),row(30,20,30,"C","detached child"),row(40,1,10,"D","group member")];
 const watch=createTestProcessWatchV1({host:fakeHost({rows:async()=>table,terminate:pid=>{killed.push(pid);}}),environment:{LANG:"en_US.UTF-8"},intervalMilliseconds:5,graceMilliseconds:1});
 const end=watch.watch(10,{file:"x.ts",terminate:()=>{},report:text=>said.push(text)});
 await sleep(30);
 table=[row(20,1,10,"B","service one"),row(30,1,30,"C","detached child"),row(40,1,10,"D","group member"),row(50,1,50,"E","unrelated"),row(60,1,60,"Z","reused pid")];
 const result=await end();
 expect(result.orphans.map(item=>item.pid).sort()).toEqual([20,30,40]);expect(killed.sort()).toEqual([20,30,40]);
 expect(said).toEqual(["[tests] x.ts left 3 process(es) running after it ended; they were stopped: 20 service one; 30 detached child; 40 group member"]);
 expect(describeOrphansV1("x.ts",result.orphans,{LANG:"de_DE.UTF-8"})).toBe("[tests] x.ts hat 3 Prozess(e) nach seinem Ende laufen lassen; sie wurden beendet: 20 service one; 30 detached child; 40 group member");
 table=[row(10,1,10,"A"),row(20,10,10,"B","service one")];
 const reuse=createTestProcessWatchV1({host:fakeHost({rows:async()=>table}),environment:{},intervalMilliseconds:5,graceMilliseconds:1}).watch(10,{file:"y.ts",terminate:()=>{},report:()=>{}});
 await sleep(30);table=[row(20,1,20,"OTHER","pid reused by a new process")];
 expect((await reuse()).orphans.length,"a pid with another start time is not the old process").toBe(0);
});

test.skipIf(!lsofAvailable)("a real process above its open files ceiling is stopped by name and a real detached process a file left behind is stopped and named",async()=>{
 const stops:string[]=[],watch=createTestProcessWatchV1({environment:{LANG:"en_US.UTF-8"},intervalMilliseconds:100,graceMilliseconds:300});
 const holder=await firstLine(process.execPath,[here("./👶️fixture/📜️holder.ts"),String(fixture.orphans.opened*2)],"READY"),pid=Number(holder.words[0]);
 const end=watch.watch(pid,{file:"holder.ts",ceiling:fixture.orphans.opened,terminate:reason=>{stops.push(reason);holder.child.kill("SIGKILL");}});
 try{
  expect(await until(()=>stops.length>0,5000)).toBe(true);
  expect(stops[0]).toMatch(new RegExp(`^\\[tests\\] holder\\.ts holds \\d+ open files in process ${pid} \\(.*\\) and was stopped at its ceiling of ${fixture.orphans.opened} open files$`,"u"));
  expect(Number(/holds (\d+) open/u.exec(stops[0]!)![1])).toBeGreaterThanOrEqual(fixture.orphans.opened*0.9);
 }finally{await end();holder.child.kill("SIGKILL");}
 const leaker=await firstLine(process.execPath,[here("./👶️fixture/📜️leaver.ts"),String(fixture.orphans.processes),"1500"],"CHILDREN"),children=leaker.words.map(Number),said:string[]=[];
 const finish=watch.watch(leaker.child.pid!,{file:"leaver.ts",terminate:()=>{},report:text=>said.push(text)});
 await new Promise<void>(done=>leaker.child.once("exit",()=>done()));
 const result=await finish();
 expect(result.orphans.map(item=>item.pid).sort()).toEqual([...children].sort());
 expect(await until(()=>children.every(child=>!alive(child)),3000),"every orphan was stopped").toBe(true);
 expect(said[0]).toMatch(new RegExp(`^\\[tests\\] leaver\\.ts left ${children.length} process\\(es\\) running after it ended; they were stopped: ${children[0]} sleep 60`,"u"));
},60000);

test("the guard is off with SEMIO_TEST_ORPHAN_GUARD=off and a process handed to init has no parent to watch",()=>{
 expect(startTestOrphanGuardV1({SEMIO_TEST_ORPHAN_GUARD:"off"})).toBeUndefined();
});
