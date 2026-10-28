import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {liveChildPidsV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {attachHostCapacityProcessV1,hostCapacityDirectoryV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,takeHostLockV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const base=readHostCapacityPolicyV1(decisionFixture.policy),policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hc-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const sampler=(current:()=>Partial<HostSampleV1>)=>async()=>({host:host({...current()}),swappedOutMegabytes:0});
const control=(signal=new AbortController().signal,remaining:()=>number|null=()=>null)=>{const published:unknown[]=[];return {published,value:{signal,remainingMilliseconds:remaining,publish:async(row:unknown)=>{published.push(row);},yieldContinuation:async()=>undefined}};};
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control().value,environment:{},write:()=>undefined,sample:sampler(()=>({})),...over});

test("Requests that do not fit together run one after the other in arrival order and the machine never holds more than its budget",async()=>{
 const directory=fresh(),order:string[]=[];let running=0,peak=0;
 const jobs=[0,1,2,3].map(async index=>{await sleep(index*15);return withHostCapacityV1(options(directory),async handle=>{running++;peak=Math.max(peak,running);order.push(handle.id);await sleep(40);running--;});});
 const ids=await Promise.all(jobs).then(()=>order);
 expect(peak).toBe(1);
 expect(ids.length).toBe(4);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
},30000);

test("A lock that a stale-lock breaker removes between creation and the owner write is lost without a crash, and the lock of a dead owner is broken once and then taken",()=>{
 const directory=fresh(),lock=join(directory,"lock");
 expect(takeHostLockV1(directory,Date.now(),base,process.pid,()=>rmSync(lock,{recursive:true,force:true}))).toBe(false);
 expect(takeHostLockV1(directory,Date.now(),base,process.pid)).toBe(true);
 expect(readFileSync(join(lock,"owner"),"utf8")).toMatch(new RegExp("^"+process.pid+":\\d+$"));
 expect(takeHostLockV1(directory,Date.now(),base,process.pid)).toBe(false);
 expect(existsSync(lock)).toBe(true);
 rmSync(lock,{recursive:true,force:true});
 const dead=spawnSync(process.execPath,["-e","0"]).pid;
 mkdirSync(lock);writeFileSync(join(lock,"owner"),`${dead}:${Date.now()}`);
 expect(takeHostLockV1(directory,Date.now(),base,process.pid)).toBe(false);
 expect(existsSync(lock)).toBe(false);
 expect(takeHostLockV1(directory,Date.now(),base,process.pid)).toBe(true);
});

test("Two requests that fit together run together and a third waits for one of them",async()=>{
 const directory=fresh();let running=0,peak=0;
 await Promise.all([0,1,2].map(async index=>{await sleep(index*15);return withHostCapacityV1(options(directory,{request:request({fixedMegabytes:5000})}),async()=>{running++;peak=Math.max(peak,running);await sleep(120);running--;});}));
 expect(peak).toBe(2);
},30000);

test("Waiting reports progress lines in English and German and publishes to the caller control",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],holder=control(),waiting=control();
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory,{control:holder.value}),async()=>hold);
 await sleep(60);
 const second=withHostCapacityV1(options(directory,{control:waiting.value,write:line=>english.push(line),environment:{LANG:"en_US.UTF-8"}}),async()=>undefined);
 const third=withHostCapacityV1(options(directory,{write:line=>german.push(line),environment:{LANG:"de_DE.UTF-8"}}),async()=>undefined);
 await sleep(150);release();await Promise.all([first,second,third]);
 expect(english.some(line=>/^\[capacity\] waiting \d+s for run: (?:an earlier request is still waiting|not enough free memory)/u.test(line))).toBe(true);
 expect(english.at(-1)).toMatch(/^\[capacity\] admitted run: 1 job\(s\), 8000 MB reserved/u);
 expect(german.some(line=>/^\[capacity\] wartet seit \d+ s auf run: (?:eine frühere Anfrage wartet noch|nicht genug freier Speicher)/u.test(line))).toBe(true);
 expect(german.at(-1)).toMatch(/^\[capacity\] zugelassen run: 1 Job\(s\), 8000 MB reserviert/u);
 expect(waiting.published.length).toBeGreaterThan(0);
 expect(waiting.published[0]).toMatchObject({resource:"semio.host.capacity/v1",mode:"shared"});
},30000);

test("Cancelling a waiting request removes it from the queue, rejects with the caller's reason and leaves the holder alone",async()=>{
 const directory=fresh();let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const controller=new AbortController(),waiting=withHostCapacityV1(options(directory,{control:control(controller.signal).value}),async()=>"never");
 await sleep(80);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 controller.abort(Error("stop waiting"));
 await expect(waiting).rejects.toThrow("stop waiting");
 expect(readdirSync(join(directory,"queue")).length).toBe(0);
 expect(readdirSync(join(directory,"grants")).length).toBe(1);
 release();await first;
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
},30000);

test("An exhausted caller deadline stops the wait",async()=>{
 const directory=fresh();let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const started=Date.now();
 await expect(withHostCapacityV1(options(directory,{control:control(new AbortController().signal,()=>150-(Date.now()-started)).value}),async()=>"never")).rejects.toThrow("deadline exhausted");
 release();await first;
});

test("A failing body still gives its reservation back",async()=>{
 const directory=fresh();
 await expect(withHostCapacityV1(options(directory),async()=>{throw Error("body failed");})).rejects.toThrow("body failed");
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 await withHostCapacityV1(options(directory),async()=>undefined);
});

test("A grant of a dead process is reclaimed instead of blocking the machine",async()=>{
 const directory=fresh(),dead=spawnSync(process.execPath,["--eval","process.exit(0)"]),id=newHostCapacityIdV1();
 mkdirSync(join(directory,"grants"),{recursive:true});
 const bootAt=Math.round((Date.now()-require("node:os").uptime()*1000)/1000)*1000;
 writeFileSync(join(directory,"grants",`${id}.json`),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:13000,admittedAtMs:Date.now(),observedMegabytes:null,pid:dead.pid,bootAtMs:bootAt}));
 const started=Date.now();
 await withHostCapacityV1(options(directory),async()=>undefined);
 expect(Date.now()-started).toBeLessThan(2000);
 expect(existsSync(join(directory,"grants",`${id}.json`))).toBe(false);
});

test("A grant from before a reboot is reclaimed even when its pid is alive again",async()=>{
 const directory=fresh(),id=newHostCapacityIdV1();
 mkdirSync(join(directory,"grants"),{recursive:true});
 writeFileSync(join(directory,"grants",`${id}.json`),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:13000,admittedAtMs:1,observedMegabytes:null,pid:process.pid,bootAtMs:1000}));
 await withHostCapacityV1(options(directory),async()=>undefined);
 expect(existsSync(join(directory,"grants",`${id}.json`))).toBe(false);
});

test("A child inside a grant inherits it without queueing even when the machine is full",async()=>{
 const directory=fresh();let seen:unknown;
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:13000})}),async outer=>{
  expect(outer.environment.SEMIO_HOST_CAPACITY_GRANT).toBe(outer.id);
  const inner=await withHostCapacityV1(options(directory,{environment:outer.environment,request:request({fixedMegabytes:13000})}),async handle=>handle);
  seen=inner;expect(inner).toMatchObject({inherited:true,id:outer.id,jobs:outer.jobs});
 });
 expect(seen).toBeDefined();
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
});

test("A process started outside the owner's tree (a build server) is attached to the grant, counted by the heartbeat and learned with the run, and its record goes with the grant",async()=>{
 const directory=fresh(),runs:{peakMegabytes:number}[]=[],trees:Record<number,number>={[process.pid]:1000,777:600,778:50};
 await withHostCapacityV1(options(directory,{observe:async pid=>trees[pid]??null,learn:run=>{runs.push(run);}}),async handle=>{
  const environment={...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  expect(attachHostCapacityProcessV1({},777),"no grant in the environment: nothing to attach to").toBe(false);
  expect(attachHostCapacityProcessV1(environment,777)).toBe(true);expect(attachHostCapacityProcessV1(environment,777)).toBe(true);expect(attachHostCapacityProcessV1(environment,778)).toBe(true);expect(attachHostCapacityProcessV1(environment,0)).toBe(false);
  expect(readFileSync(join(directory,"grants",`${handle.id}.attached`),"utf8")).toBe("777\n778\n");
  await sleep(150);
  expect(JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8")).observedMegabytes,"own tree 1000 + attached 600 + 50").toBe(1650);
  const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});expect(status.grants.length,"the attached record is not a grant").toBe(1);
  trees[777]=-1;delete trees[778];await sleep(100);
 });
 expect(readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".attached"))).toEqual([]);expect(runs.length).toBe(1);expect(runs[0]!.peakMegabytes).toBe(1650);
});

test("The governor can be switched off explicitly or by policy and says so",async()=>{
 const directory=fresh(),lines:string[]=[];
 await withHostCapacityV1(options(directory,{environment:{SEMIO_HOST_CAPACITY:"off"},write:line=>lines.push(line),sample:async()=>{throw Error("must not sample");}}),async handle=>expect(handle).toMatchObject({bypassed:true,reservedMegabytes:0}));
 expect(lines[0]).toContain("SEMIO_HOST_CAPACITY=off");
 await withHostCapacityV1(options(directory,{policy:{...policy,enabled:false}}),async handle=>expect(handle.bypassed).toBe(true));
});

test("The heartbeat records what the owner's process tree holds",async()=>{
 const directory=fresh();
 await withHostCapacityV1(options(directory,{observe:async()=>4321}),async handle=>{
  await sleep(120);
  const row=JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));
  expect(row.observedMegabytes).toBe(4321);
  const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});
  expect(status.grants.map(grant=>grant.observedMegabytes)).toEqual([4321]);
  expect(status.evaluation.unrealizedMegabytes).toBe(8000-4321);
 });
});

test("Foreign load never makes a request wait: it scales jobs down, a request wider than the core ceiling still overrides when nothing of ours runs, and memory pressure at the hard level blocks",async()=>{
 const directory=fresh(),lines:string[]=[];
 const started=Date.now();
 await withHostCapacityV1(options(directory,{sample:sampler(()=>({load:33})),write:line=>lines.push(line),request:request({fixedMegabytes:5120,cpuPerJob:2})}),async handle=>expect(handle).toMatchObject({jobs:1,reservedMegabytes:5120}));
 expect(Date.now()-started).toBeLessThan(140);
 expect(lines.at(-1)).toMatch(/^\[capacity\] admitted run: 1 job\(s\), 5120 MB reserved/u);
 lines.length=0;
 const wide=Date.now();
 await withHostCapacityV1(options(directory,{sample:sampler(()=>({load:1})),write:line=>lines.push(line),request:request({cpuPerJob:25})}),async handle=>expect(handle.jobs).toBe(1));
 expect(Date.now()-wide).toBeGreaterThanOrEqual(140);
 expect(lines.some(line=>line.includes("waiting")&&line.includes("the cores are fully booked by our own running jobs"))).toBe(true);
 expect(lines.at(-1)).toMatch(/admitted run after \d+s although the cores are fully booked by our own running jobs/u);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
 await expect(withHostCapacityV1(options(directory,{control:control(controller.signal).value,sample:sampler(()=>({pressure:"critical"}))}),async()=>"never")).rejects.toThrow("gave up");
},30000);

test("Leaked disk images and an exhausted file table hold a request back, say why, and are overridden only when nothing of ours runs",async()=>{
 const directory=fresh(),lines:string[]=[];
 for(const [over,sentence] of [[{attachedImages:40},"too many disk images are attached"],[{openFiles:240000},"the machine has too many open files"]] as const){
  lines.length=0;
  const started=Date.now();
  await withHostCapacityV1(options(directory,{sample:sampler(()=>over),write:line=>lines.push(line)}),async handle=>expect(handle.jobs).toBe(1));
  expect(Date.now()-started).toBeGreaterThanOrEqual(140);
  expect(lines.some(line=>line.includes(`waiting`)&&line.includes(sentence)),sentence).toBe(true);
  expect(lines.at(-1),sentence).toContain(`although ${sentence}`);
 }
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100})}),async()=>hold);
 await sleep(60);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),300);
 await expect(withHostCapacityV1(options(directory,{control:control(controller.signal).value,sample:sampler(()=>({attachedImages:40})),request:request({fixedMegabytes:100})}),async()=>"never")).rejects.toThrow("gave up");
 release();await first;
},30000);

test("A waiter that cannot fit yet does not hold back a smaller one behind it, unless finishing our own grants would admit it and it has waited its patience",async()=>{
 const directory=fresh(),order:string[]=[];let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const holder=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const head=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:13000})}),async()=>{order.push("head");});
 await sleep(40);
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:1000})}),async()=>{order.push("small");});
 expect(order).toEqual(["small"]);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 release();await holder;await head;
 expect(order).toEqual(["small","head"]);
 const strict=fresh(),patient={...policy,wait:{...policy.wait,bypassMilliseconds:0}};let free!:()=>void;const kept=new Promise<void>(resolve=>{free=resolve;});
 const first=withHostCapacityV1(options(strict,{policy:patient}),async()=>kept);
 await sleep(60);
 const big=withHostCapacityV1(options(strict,{policy:patient,request:request({fixedMegabytes:13000})}),async()=>"big");
 await sleep(40);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
 await expect(withHostCapacityV1(options(strict,{policy:patient,control:control(controller.signal).value,request:request({fixedMegabytes:1000})}),async()=>"never")).rejects.toThrow("gave up");
 free();await first;await big;
},30000);

test("The directory is per user and shared by every checkout, with an explicit override",()=>{
 expect(hostCapacityDirectoryV1({},"darwin","/Users/x")).toBe("/Users/x/Library/Caches/semio/host-capacity");
 expect(hostCapacityDirectoryV1({XDG_CACHE_HOME:"/c"},"linux","/home/x")).toBe("/c/semio/host-capacity");
 expect(hostCapacityDirectoryV1({},"linux","/home/x")).toBe("/home/x/.cache/semio/host-capacity");
 expect(hostCapacityDirectoryV1({SEMIO_HOST_CAPACITY_DIRECTORY:"/tmp/g"},"darwin","/Users/x")).toBe("/tmp/g");
 expect(hostCapacityDirectoryV1({LOCALAPPDATA:"C:\\L"},"win32","C:\\Users\\x")).toContain("semio");
});

test("Separate operating-system processes share one budget, a killed holder frees its reservation, and a killed holder or waiter leaves nothing behind",async()=>{
 const directory=fresh(),log=join(directory,"log.jsonl"),moduleUrl=new URL("../../🟦️.ts",import.meta.url).href,hostFile=join(directory,"host.json");
 writeFileSync(hostFile,JSON.stringify(host()));
 const script=join(directory,"child.ts");
 writeFileSync(script,`import {appendFileSync,readFileSync} from "node:fs";
import {setTimeout as sleep} from "node:timers/promises";
import {withHostCapacityV1} from ${JSON.stringify(moduleUrl)};
const [directory,logFile,hostFile,policyText,mode]=process.argv.slice(2);
const policy=JSON.parse(policyText);
const id=process.env.CHILD_ID;
await withHostCapacityV1({directory,policy,request:{id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1},control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},environment:{},write:()=>undefined,sample:async()=>({host:{...JSON.parse(readFileSync(hostFile,"utf8")),atMs:Date.now()},swappedOutMegabytes:0})},async()=>{
 appendFileSync(logFile,JSON.stringify({id,event:"start",at:Date.now()})+"\\n");
 await sleep(mode==="hold"?600000:250);
 appendFileSync(logFile,JSON.stringify({id,event:"end",at:Date.now()})+"\\n");
});`);
 const child=(id:string,mode:string)=>spawn(process.execPath,[script,directory,log,hostFile,JSON.stringify(policy),mode],{env:{...process.env,CHILD_ID:id},stdio:"ignore"});
 const ids=[newHostCapacityIdV1(),newHostCapacityIdV1(),newHostCapacityIdV1()],children=ids.map((id,index)=>{const row=child(id,"short");return sleep(0).then(()=>row);});
 const spawned=await Promise.all(children);
 await Promise.all(spawned.map(row=>new Promise(resolve=>row.once("exit",resolve))));
 const events=readFileSync(log,"utf8").trim().split("\n").map(line=>JSON.parse(line)),intervals=ids.map(id=>({start:events.find(row=>row.id===id&&row.event==="start")!.at as number,end:events.find(row=>row.id===id&&row.event==="end")!.at as number})).sort((left,right)=>left.start-right.start);
 for(let index=1;index<intervals.length;index++)expect(intervals[index]!.start,`process ${index} starts after process ${index-1} ends`).toBeGreaterThanOrEqual(intervals[index-1]!.end);
 const holderId=newHostCapacityIdV1(),holder=child(holderId,"hold");
 for(let waited=0;waited<200&&!existsSync(join(directory,"grants",`${holderId}.json`));waited++)await sleep(25);
 expect(existsSync(join(directory,"grants",`${holderId}.json`))).toBe(true);
 const waiterId=newHostCapacityIdV1(),waiter=child(waiterId,"short");
 for(let waited=0;waited<400&&!readdirSync(join(directory,"queue")).some(name=>name.includes(waiterId));waited++)await sleep(25);
 expect(readdirSync(join(directory,"queue")).some(name=>name.includes(waiterId))).toBe(true);
 holder.kill("SIGKILL");
 await new Promise(resolve=>waiter.once("exit",resolve));
 const after=readFileSync(log,"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(after.some(row=>row.id===waiterId&&row.event==="end")).toBe(true);
 const sleeper=child(newHostCapacityIdV1(),"hold");
 for(let waited=0;waited<400&&readdirSync(join(directory,"grants")).length<1;waited++)await sleep(25);
 const queued=child(newHostCapacityIdV1(),"short");
 for(let waited=0;waited<400&&readdirSync(join(directory,"queue")).length<1;waited++)await sleep(25);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 queued.kill("SIGKILL");sleeper.kill("SIGKILL");
 await sleep(150);
 const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});
 expect(status.waiters).toEqual([]);
 expect(status.grants).toEqual([]);
},60000);

test("A grant whose process only waits in queues is lent back so a blocked admission gets through, and it ramps again when the process continues",async()=>{
 const directory=fresh();let episode=0,skew=0;
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:8000}),observe:async()=>100,waited:()=>episode,children:()=>[],now:()=>Date.now()+skew}),async first=>{
  let second="pending";const waiting=withHostCapacityV1(options(directory),async()=>{second="admitted";});
  await sleep(200);expect(second,"while the first grant works, its unrealized ramp keeps the second out").toBe("pending");
  expect(JSON.parse(readFileSync(join(directory,"grants",`${first.id}.json`),"utf8")).lentSinceMs).toBeUndefined();
  episode=40000;await sleep(150);expect(JSON.parse(readFileSync(join(directory,"grants",`${first.id}.json`),"utf8")).lentSinceMs,"a wait that began before the grant does not lend it at once").toBeUndefined();skew=40000;await waiting;expect(second).toBe("admitted");
  const lent=JSON.parse(readFileSync(join(directory,"grants",`${first.id}.json`),"utf8"));expect(typeof lent.lentSinceMs).toBe("number");expect(lent.resumedAtMs).toBeUndefined();
  const during=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});expect(during.evaluation.unrealizedMegabytes).toBe(0);
  episode=0;await sleep(150);
  const resumed=JSON.parse(readFileSync(join(directory,"grants",`${first.id}.json`),"utf8"));expect(resumed.lentSinceMs).toBeUndefined();expect(typeof resumed.resumedAtMs).toBe("number");
  const after=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});expect(after.evaluation.unrealizedMegabytes).toBe(8000-100);
 });
},30000);

test("A waiting process that still has a child process is not lent, one that lost its children is",async()=>{
 const directory=fresh();let children:readonly number[]=[4242],skew=0;
 await withHostCapacityV1(options(directory,{observe:async()=>100,waited:()=>40000,children:()=>children,now:()=>Date.now()+skew}),async handle=>{skew=40000;
  const read=()=>JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));
  await sleep(200);expect(read().lentSinceMs).toBeUndefined();
  children=[];await sleep(200);expect(typeof read().lentSinceMs).toBe("number");
  children=[4243];await sleep(200);expect(read().lentSinceMs).toBeUndefined();expect(typeof read().resumedAtMs).toBe("number");
 });
},30000);

/** 🛡️ An unreadable process table counts as a live child, so a grant is never lent on a guess (the host cannot run `ps`, as inside the macOS `sandbox-exec` seal of the mirror tool). */
test("A waiting process whose process table cannot be read is never lent",async()=>{
 const directory=fresh();let skew=0;
 await withHostCapacityV1(options(directory,{observe:async()=>100,waited:()=>40000,children:()=>undefined,now:()=>Date.now()+skew}),async handle=>{skew=40000;
  const read=()=>JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));
  await sleep(400);expect(read().lentSinceMs).toBeUndefined();expect(read().resumedAtMs).toBeUndefined();
 });
},30000);

/** 🧪️ A child process that takes a grant and then either waits through the process watchdog's declared waits (`waiting`) or lives on a plain timer (`working`) for four seconds. */
function lendingChild(directory:string,mode:"waiting"|"working",authored:object):string{
 const script=join(directory,`lending-${mode}.ts`),gate=JSON.stringify(join(import.meta.dir,"../../🟦️.ts")),watchdog=JSON.stringify(join(import.meta.dir,"../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts"));
 writeFileSync(script,`import {withHostCapacityV1} from ${gate};import {waitingDelayV1} from ${watchdog};
const host={version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
const request={id:"${mode}-aaaaaaaa",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:1000,perJobMegabytes:0,cpuPerJob:1};
await withHostCapacityV1({directory:${JSON.stringify(directory)},policy:${JSON.stringify(authored)},request,control,environment:{},write:()=>undefined,sample:async()=>({host:{...host,atMs:Date.now()},swappedOutMegabytes:0}),observe:async()=>100},async()=>{const end=Date.now()+4000;while(Date.now()<end)${mode==="waiting"?"await waitingDelayV1(50);":"await new Promise(done=>setTimeout(done,50));"}});`);
 return script;
}

test("In a real process a grant is lent only while the process is inside declared queue waits with no child, and taken back when it stops waiting",async()=>{
 const directory=fresh(),short=readHostCapacityPolicyV1({...policy,wait:{...policy.wait,lendAfterMilliseconds:1000}});
 const run=async(mode:"waiting"|"working"):Promise<boolean>=>{
  const child=spawn(process.execPath,[lendingChild(directory,mode,short)],{stdio:["ignore","inherit","inherit"]}),exited=new Promise<void>(done=>child.on("exit",()=>done())),file=join(directory,"grants",`${mode}-aaaaaaaa.json`);let lent=false;
  try{for(let attempt=0;attempt<60&&child.exitCode===null;attempt++){await sleep(100);try{if(typeof JSON.parse(readFileSync(file,"utf8")).lentSinceMs==="number"){lent=true;break;}}catch{}}}finally{child.kill();await exited;}
  return lent;
 };
 expect(await run("working"),"a process that works on a plain timer is never lent").toBe(false);
 const ps=["/bin/ps","/usr/bin/ps"].find(path=>existsSync(path)),readable=process.platform==="win32"||ps!==undefined&&spawnSync(ps,["-o","pid=","-p",String(process.pid)],{encoding:"utf8",timeout:10000}).status===0;
 expect(liveChildPidsV1()!==undefined,"the production process-table reader answers exactly where an independent ps call can read the table").toBe(readable);
 expect(await run("waiting"),readable?"a process that only waits is lent after the authored time":"a process whose process table cannot be read is never lent").toBe(readable);
},60000);

test("A finished grant reports its peak tree megabytes and its active time without the time it was lent to a queue wait, and a grant that was never observed reports nothing",async()=>{
 const directory=fresh(),runs:{jobs:number;peakMegabytes:number;activeMilliseconds:number;endedAtMs:number}[]=[];
 let step=0;const series=[400,2600,900];
 await withHostCapacityV1(options(directory,{policy:readHostCapacityPolicyV1({...policy,wait:{...policy.wait,lendAfterMilliseconds:1000}}),request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>series[Math.min(step++,series.length-1)]!,waited:()=>1_000_000,children:()=>[]}),async()=>sleep(1800));
 expect(runs.length).toBe(1);
 expect(runs[0]).toMatchObject({jobs:1,peakMegabytes:2600});
 expect(runs[0]!.activeMilliseconds).toBeGreaterThan(900);expect(runs[0]!.activeMilliseconds).toBeLessThan(1400);
 const silent:unknown[]=[];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{silent.push(run);},observe:async()=>null}),async()=>sleep(80));
 expect(silent).toEqual([]);
 await expect(withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:()=>{throw Error("store gone");},observe:async()=>500}),async()=>{await sleep(80);return "kept";})).resolves.toBe("kept");
},30000);

test("Law: a finished grant reports when its tree first came within 5 % of its peak, so a late peak is learned as late, and how its body ended",async()=>{
 const directory=fresh(),runs:{peakMegabytes:number;peakAtMilliseconds:number;activeMilliseconds:number;status:string}[]=[];
 let step=0;const series=[300,500,900,2000,2560,2600,2640,2400];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>series[Math.min(step++,series.length-1)]!}),async()=>sleep(450));
 expect(runs.length).toBe(1);
 expect(runs[0]).toMatchObject({peakMegabytes:2640,status:"ok"});
 const reached=policy.wait.heartbeatMilliseconds*series.findIndex(value=>value*10000>=2640*9500);
 expect(runs[0]!.peakAtMilliseconds,"the first reading at or above 95 % of the peak, not the last one").toBeGreaterThanOrEqual(reached-policy.wait.heartbeatMilliseconds);
 expect(runs[0]!.peakAtMilliseconds).toBeLessThan(runs[0]!.activeMilliseconds);
 expect(runs[0]!.peakAtMilliseconds,"a gradual climb is not a late peak").toBeLessThan(policy.wait.heartbeatMilliseconds*(series.length-1));
 const ended:string[]=[];
 const failing=options(directory,{request:request({fixedMegabytes:100}),learn:run=>{ended.push(run.status);},observe:async()=>500});
 await expect(withHostCapacityV1(failing,async()=>{await sleep(80);throw Error("build failed");})).rejects.toThrow("build failed");
 const abort=new AbortController(),aborted=options(directory,{request:request({fixedMegabytes:100}),control:control(abort.signal).value,learn:run=>{ended.push(run.status);},observe:async()=>500});
 await expect(withHostCapacityV1(aborted,async()=>{await sleep(60);abort.abort();throw Error("stopped");})).rejects.toThrow("stopped");
 await expect(withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{ended.push(run.status);},observe:async()=>500}),async()=>{await sleep(80);throw Object.assign(Error("cancelled by the user"),{code:"cancelled"});})).rejects.toThrow("cancelled");
 expect(ended).toEqual(["failed","cancelled","cancelled"]);
},30000);

test("Law: the grant carries the time to peak and the ramp slot of its request, the request is refused for a slot while the slots are taken, and the status line says why in English and German",async()=>{
 const directory=fresh(),shrunk={rampMilliseconds:900000,rampSlot:true};
 const hold=(index:number)=>withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100,...shrunk}),pid:process.pid}),async handle=>{
  const grant=JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));
  expect(grant,`grant ${index}`).toMatchObject(shrunk);
  return handle.id;
 });
 await hold(0);
 const lines:string[]=[],taken=policy.memory.maximumRamping;
 const running=Array.from({length:taken},()=>{let release=()=>{};const done=new Promise<void>(accept=>{release=accept;});const started=new Promise<void>(accept=>{void withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100,...shrunk})}),async()=>{accept();await done;});});return {started,release,done};});
 await Promise.all(running.map(row=>row.started));
 const abort=new AbortController(),waiting=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100,...shrunk}),control:control(abort.signal).value,write:line=>lines.push(line)}),async()=>"never").catch(error=>error);
 await sleep(150);
 const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});
 expect(status.evaluation.decisions.map(row=>row.reason)).toEqual(["ramp"]);
 expect(lines.at(-1)).toContain("too many learned reservations are still growing to their peak");
 const german:string[]=[];
 const abortGerman=new AbortController(),waitingGerman=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100,...shrunk}),environment:{SEMIO_LANGUAGE:"de"},control:control(abortGerman.signal).value,write:line=>german.push(line)}),async()=>"never").catch(error=>error);
 await sleep(150);
 expect(german.at(-1)).toContain("zu viele gelernte Reservierungen wachsen noch auf ihren Spitzenwert an");
 abort.abort();abortGerman.abort();await waiting;await waitingGerman;
 for(const row of running)row.release();await Promise.all(running.map(row=>row.done));
 const plain:string[]=[];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100,rampMilliseconds:300000}),write:line=>plain.push(line)}),async handle=>{expect(JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8")).rampSlot,"an unshrunk request takes no slot").toBeUndefined();});
},30000);
