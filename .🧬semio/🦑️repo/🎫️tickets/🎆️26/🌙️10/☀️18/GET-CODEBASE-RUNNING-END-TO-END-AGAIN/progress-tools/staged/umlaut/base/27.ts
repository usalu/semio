import {afterAll,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {existsSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {estimateHostRatioV1,evaluateHostCapacityV1,hostFloorMegabytesV1,rampHostRequestV1,readHostCapacityPolicyV1,scaleHostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../🔣️policy.json" with {type:"json"};
import catalog from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {leasePriorityClassV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,buildCapacityAskV1,buildCapacityDeferredV1,createCapacityControlV1,grantedBuildArgumentsV1,withBuildCapacityV1} from "../🟦️.ts";
import {sizeHostRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {runCapacityCommandV1} from "../⌨️command/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hb-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});

test("The authored host capacity policy is schema-valid and covers exactly the gated catalogue operations plus the ad-hoc command block",()=>{
 expect(BUILD_HOST_CAPACITY_POLICY_V1).toEqual(readHostCapacityPolicyV1(buildPolicy.hostCapacity));
 const names=new Set(catalog.operations.map(row=>row.name));
 for(const row of BUILD_HOST_CAPACITY_POLICY_V1.operations)expect(names.has(row.name)||row.name==="command",row.name).toBe(true);
 for(const light of ["tools","reset","cache","preflight","capacity"])expect(BUILD_HOST_CAPACITY_POLICY_V1.operations.some(row=>row.name===light),light).toBe(false);
 for(const heavy of ["run","run-many","affected","exec","watch","cargo","show","graph","dependencies"])expect(BUILD_HOST_CAPACITY_POLICY_V1.operations.some(row=>row.name===heavy),heavy).toBe(true);
});

test("Invocations ask for the operation, the jobs and the kinds of their targets, and light commands ask for nothing",()=>{
 for(const item of fixture.asks)expect(buildCapacityAskV1(item.args,item.environment)??null,item.name).toEqual(item.expect as never);
});

test("A grant rewrites the parallelism option and never touches what follows the separator",()=>{
 for(const item of fixture.rewrites)expect(grantedBuildArgumentsV1(item.args,item.operation,item.jobs),item.args.join(" ")).toEqual(item.expect);
});

test("A heavy invocation runs under a reduced grant with rewritten arguments and inherited environment, then restores the environment",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_BUILD_PARALLEL:"10"},lines:string[]=[];
 const result=await withBuildCapacityV1(["--tool=bazel","run","@x/y:typecheck","--parallel=10"],environment,async granted=>{
  expect(granted.handle).toMatchObject({jobs:2,inherited:false});
  expect(granted.args).toEqual(["--tool=bazel","run","@x/y:typecheck","--parallel=2"]);
  expect(environment.SEMIO_BUILD_PARALLEL).toBe("2");
  expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBe(granted.handle!.id);
  expect(JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN!)).toMatchObject({jobs:2,cpu:2,memoryMegabytes:8192,fixedMegabytes:1536});
  expect(readdirSync(join(directory,"grants")).length).toBe(1);
  return "done";
 },{directory,policy:fast,strip,write:line=>lines.push(line),sample:async()=>({host:host({availableMegabytes:14000}),swappedOutMegabytes:0})});
 expect(result).toBe("done");
 expect(environment.SEMIO_BUILD_PARALLEL).toBe("10");
 expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBeUndefined();
 expect(environment.SEMIO_HOST_CAPACITY_PLAN).toBeUndefined();
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 expect(lines.at(-1)).toMatch(/admitted run with 2 of 10 job\(s\)/u);
});

test("Light commands and fixed-block operations leave the arguments and the parallelism alone, light ones without touching the gate",async()=>{
 const directory=join(fresh(),"gate"),environment:NodeJS.ProcessEnv={SEMIO_BUILD_PARALLEL:"7"};
 await withBuildCapacityV1(["cache","status"],environment,async granted=>{expect(granted).toEqual({args:["cache","status"],handle:undefined});},{directory,policy:fast,strip});
 expect(existsSync(directory)).toBe(false);
 await withBuildCapacityV1(["show","projects"],environment,async granted=>{expect(granted.args).toEqual(["show","projects"]);expect(environment.SEMIO_BUILD_PARALLEL).toBe("7");expect(granted.handle).toMatchObject({jobs:1,reservedMegabytes:5120});},{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0})});
});

test("Law: `dependencies ensure` asks for no capacity up front (a fresh receipt never queues), only the installer asks, and sync and lock still ask",async()=>{
 expect(fixture.deferred.map(item=>buildCapacityDeferredV1(item.args)),"fixture rows").toEqual(fixture.deferred.map(item=>item.deferred));
 const directory=join(fresh(),"gate"),environment:NodeJS.ProcessEnv={},sample=async()=>({host:host(),swappedOutMegabytes:0});
 await withBuildCapacityV1(["dependencies","ensure"],environment,async granted=>{expect(granted).toEqual({args:["dependencies","ensure"],handle:undefined});},{directory,policy:fast,strip,sample});
 expect(existsSync(directory),"no gate entry for the fresh path").toBe(false);
 await withBuildCapacityV1(["dependencies","ensure"],environment,async granted=>{expect(granted.handle).toMatchObject({reservedMegabytes:6144});},{directory,policy:fast,strip,sample,admitDeferred:true});
 for(const args of [["dependencies","sync","--all"],["dependencies","lock","--all"]])await withBuildCapacityV1(args,environment,async granted=>{expect(granted.handle,args.join(" ")).toMatchObject({reservedMegabytes:6144});},{directory,policy:fast,strip,sample});
 const starved=async()=>({host:host({availableMegabytes:1000,pressure:"critical"}),swappedOutMegabytes:0});
 let ran=false;await withBuildCapacityV1(["dependencies","ensure"],environment,async()=>{ran=true;},{directory:join(fresh(),"starved"),policy:fast,strip,sample:starved});
 expect(ran,"a starved host does not delay the fresh path").toBe(true);
});

test("Law: a native dashboard launch asks for no capacity (it only hands the terminal to the installed binary), a dashboard build still does",async()=>{
 const directory=join(fresh(),"gate"),environment:NodeJS.ProcessEnv={},sample=async()=>({host:host(),swappedOutMegabytes:0});
 for(const args of fixture.nativeLaunch.free)await withBuildCapacityV1(args,environment,async granted=>{expect(granted,args.join(" ")).toEqual({args,handle:undefined});},{directory,policy:fast,strip,sample});
 expect(existsSync(directory),"no gate entry for a launch").toBe(false);
 for(const args of fixture.nativeLaunch.asking)await withBuildCapacityV1(args,environment,async granted=>{expect(granted.handle,args.join(" ")).toBeDefined();},{directory,policy:fast,strip,sample});
});

test("A nested invocation inherits the grant of its parent through the environment and waits for nothing",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={};
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async outer=>{
  const nested:NodeJS.ProcessEnv={...environment};
  await withBuildCapacityV1(["run","@x/z:build","--parallel=2"],nested,async inner=>expect(inner.handle).toMatchObject({inherited:true,id:outer.handle!.id}),{directory,policy:fast,strip,sample:async()=>({host:host({availableMegabytes:7000}),swappedOutMegabytes:0})});
 },{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0})});
});

test("An interrupt while waiting cancels the wait and leaves nothing in the queue",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={},emitter=new EventEmitter();
 const sample=async()=>({host:host({availableMegabytes:6600}),swappedOutMegabytes:0}),control=createCapacityControlV1(emitter as never);
 const waiting=withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async()=>"never",{directory,policy:fast,strip,sample,control:control.control,write:()=>undefined});
 await sleep(80);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 emitter.emit("SIGINT");
 await expect(waiting).rejects.toMatchObject({code:"cancelled",key:"capacity.wait-cancelled",values:{signal:"SIGINT"}});
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
 control.dispose();
 expect(emitter.listenerCount("SIGINT")).toBe(0);
});

test("repo capacity status prints one line per fact in English and German and a JSON snapshot on request",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],json:string[]=[];
 const sample=async()=>({host:host(),swappedOutMegabytes:0}),context=(environment:NodeJS.ProcessEnv,write:(line:string)=>void)=>({environment,write,directory,policy:fast,sample});
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],{},async()=>{
  expect(await runCapacityCommandV1(["status"],context({LANG:"en_US.UTF-8"},line=>english.push(line)))).toBe(0);
  expect(await runCapacityCommandV1(["status"],context({LANG:"de_DE.UTF-8"},line=>german.push(line)))).toBe(0);
  expect(await runCapacityCommandV1(["status","--json"],context({},line=>json.push(line)))).toBe(0);
 },{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0}),pid:process.pid});
 expect(english[0]).toMatch(/^\[capacity\] \d+ MB available of \d+ MB \(floor \d+ MB, -?\d+ MB spare\), pressure \w+, swap \S+ MB, load \S+, open files \S+, attached images \S+; 1 running, 0 waiting$/u);
 expect(english[1]).toMatch(/^\[capacity\]   running \S+ run: 2 job\(s\), \d+ MB reserved, \S+ MB observed, pid \d+, since \d+s$/u);
 expect(german[0]).toMatch(/verfügbar von .* laufen, 0 warten$/u);
 expect(german[1]).toContain("läuft");
 expect(JSON.parse(json.join("\n"))).toMatchObject({grants:[{operation:"run"}],waiters:[]});
});

test("repo capacity status marks a grant that is lent back in English and German and in the JSON snapshot",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],json:string[]=[];
 const sample=async()=>({host:host(),swappedOutMegabytes:0}),context=(environment:NodeJS.ProcessEnv,write:(line:string)=>void)=>({environment,write,directory,policy:fast,sample});
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],{},async()=>{
  const file=join(directory,"grants",readdirSync(join(directory,"grants"))[0]!);writeFileSync(file,JSON.stringify({...JSON.parse(readFileSync(file,"utf8")),lentSinceMs:Date.now()}));
  await runCapacityCommandV1(["status"],context({LANG:"en_US.UTF-8"},line=>english.push(line)));await runCapacityCommandV1(["status"],context({LANG:"de_DE.UTF-8"},line=>german.push(line)));await runCapacityCommandV1(["status","--json"],context({},line=>json.push(line)));
 },{directory,policy:fast,strip,sample,pid:process.pid});
 expect(english[1]).toMatch(/^\[capacity\]   lent \S+ run: its process only waits in a queue, so the \d+ MB reserved and \d+ core\(s\) count for nobody/u);
 expect(german[1]).toContain("verliehen");expect(JSON.parse(json.join("\n")).grants[0].lentSinceMs).toBeGreaterThan(0);
});

test("repo capacity run admits an arbitrary command, hands it the grant and returns its exit code",async()=>{
 const directory=fresh(),lines:string[]=[],context={environment:{...process.env,LANG:"en_US.UTF-8"} as NodeJS.ProcessEnv,write:(line:string)=>lines.push(line),directory,policy:fast,sample:async()=>({host:host(),swappedOutMegabytes:0})};
 expect(await runCapacityCommandV1(["run","--memory=100","--","node","-e","process.exit(process.env.SEMIO_HOST_CAPACITY_GRANT&&JSON.parse(process.env.SEMIO_HOST_CAPACITY_PLAN).fixedMegabytes===100?0:3)"],context)).toBe(0);
 expect(await runCapacityCommandV1(["run","--memory=100","--","node","-e","process.exit(7)"],context)).toBe(7);
 expect(lines.some(line=>/admitted command/u.test(line))).toBe(false);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 await expect(runCapacityCommandV1(["run","--memory=100"],context)).rejects.toThrow("requires a command");
 await expect(runCapacityCommandV1(["run","--memory=0","--","true"],context)).rejects.toThrow("positive integer");
 await expect(runCapacityCommandV1(["run","--operation=nonsense","--","true"],context)).rejects.toThrow("Unknown capacity operation");
 await expect(runCapacityCommandV1(["frobnicate"],context)).rejects.toThrow("Expected repo capacity status|run");
});

test("Measured demand closes the loop: runs report their peak, the class learns it, and the next request of that class reserves its learned share while the build tool envelope stays at the default",async()=>{
 const directory=fresh(),learning=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,heartbeatMilliseconds:20}});
 const reserved:number[]=[],envelopes:number[]=[],lines:string[]=[],ports={directory,policy:learning,strip,write:(line:string)=>lines.push(line),sample:async()=>({host:host({availableMegabytes:30000}),swappedOutMegabytes:0}),observe:async()=>600,now:Date.now};
 for(let index=0;index<5;index++)await withBuildCapacityV1(["run","@x/y:typecheck","--parallel=1"],{},async granted=>{reserved.push(granted.handle!.reservedMegabytes);envelopes.push(granted.handle!.plan.memoryMegabytes);await sleep(120);},ports);
 expect(reserved[0]).toBe(1536+4096);
 expect(reserved.at(-1)!).toBeLessThan(reserved[0]!/2);
 expect(reserved.at(-1)!).toBeGreaterThanOrEqual(learning.estimates.minimumMegabytes);
 expect(new Set(envelopes)).toEqual(new Set([4096]));
 expect(lines.some(line=>/^\[capacity\] run: sized at \d+% of the default from \d+ earlier run\(s\) of its class$/u.test(line))).toBe(true);
 const german:string[]=[];
 await withBuildCapacityV1(["run","@x/y:typecheck","--parallel=1"],{LANG:"de_DE.UTF-8"},async()=>undefined,{...ports,write:line=>german.push(line)});
 expect(german.some(line=>/^\[capacity\] run: auf \d+% des Standardwerts bemessen, aus \d+ frueheren Laeufen seiner Klasse$/u.test(line))).toBe(true);
 const other:number[]=[];
 await withBuildCapacityV1(["run","@x/other:typecheck","--parallel=1"],{},async granted=>{other.push(granted.handle!.reservedMegabytes);},ports);
 expect(other[0]).toBeLessThan(reserved[0]!);
},60000);

test("Law: failed, cancelled and non-zero-exit runs never lower what a class learned, a failed run that already exceeds the estimate can raise it, and a completed run records when it peaked",async()=>{
 const directory=fresh(),learning=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,heartbeatMilliseconds:20}});
 let peak=3000,code:number|undefined=0;
 const ports=()=>({directory,policy:learning,strip,write:()=>undefined,sample:async()=>({host:host({availableMegabytes:30000}),swappedOutMegabytes:0}),observe:async()=>peak,now:Date.now,exitCode:()=>code}),args=["run","@x/y:typecheck","--parallel=1"],samples=()=>(JSON.parse(readFileSync(join(directory,"estimates.json"),"utf8")).classes["run"].samples as {ratioBasisPoints:number;peakAtMilliseconds?:number;activeMilliseconds:number}[]);
 for(let index=0;index<4;index++)await withBuildCapacityV1(args,{},async()=>{await sleep(100);},ports());
 expect(samples().length).toBe(4);
 expect(samples().every(row=>row.peakAtMilliseconds!==undefined&&row.peakAtMilliseconds<=row.activeMilliseconds),"every completed run records when it peaked").toBe(true);
 peak=200;code=1;
 await withBuildCapacityV1(args,{},async()=>{await sleep(100);},ports());
 expect(samples().length,"a run that exited non-zero with a small peak teaches nothing").toBe(4);
 code=0;
 await expect(withBuildCapacityV1(args,{},async()=>{await sleep(100);throw Error("died");},ports())).rejects.toThrow("died");
 expect(samples().length,"a run that threw with a small peak teaches nothing").toBe(4);
 await expect(withBuildCapacityV1(args,{},async()=>{await sleep(60);throw Object.assign(Error("stopped"),{code:"cancelled"});},ports())).rejects.toThrow("stopped");
 expect(samples().length,"a cancelled run with a small peak teaches nothing").toBe(4);
 peak=16000;
 await expect(withBuildCapacityV1(args,{},async()=>{await sleep(100);throw Error("died late");},ports())).rejects.toThrow("died late");
 expect(samples().length,"a failed run whose partial peak already exceeds the estimate raises the class").toBe(5);
 expect(Math.max(...samples().map(row=>row.ratioBasisPoints))).toBeGreaterThan(10000);
},30000);

test("The floor scales with the memory of the machine, and a 16 GB machine carries several measured runs where the fixed 20 % floor of before admitted one",()=>{
 const floors=Object.fromEntries([8192,16384,32768,65536,131072].map(total=>[total,hostFloorMegabytesV1(BUILD_HOST_CAPACITY_POLICY_V1,{totalMegabytes:total})]));
 expect(floors).toEqual({8192:1536,16384:2048,32768:4096,65536:4096,131072:4096});
 const totals=Object.keys(floors).map(Number).sort((left,right)=>left-right);
 for(let index=1;index<totals.length;index++)expect(floors[totals[index]!]!).toBeGreaterThanOrEqual(floors[totals[index-1]!]!);
 for(const total of totals)expect(floors[total]!/total).toBeLessThanOrEqual(0.2);
 const before=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:{...buildPolicy.hostCapacity.memory,floorBasisPoints:2000,floorMinimumMegabytes:4096,floorMaximumMegabytes:8192}});
 const sample:HostSampleV1=host({totalMegabytes:16384,availableMegabytes:9000,cores:8,load:null}),ask=buildCapacityAskV1(["run","@x/y:build","--parallel=1"],{},BUILD_HOST_CAPACITY_POLICY_V1)!;
 const base=sizeHostRequestV1(BUILD_HOST_CAPACITY_POLICY_V1,{id:"aaaaaaaa1",operation:ask.operation,preferredJobs:1,kinds:ask.kinds})!;
 const queue=(ratio:number):HostWaiterV1[]=>Array.from({length:6},(_,index)=>({request:{...rampHostRequestV1(base,scaleHostRequestV1(BUILD_HOST_CAPACITY_POLICY_V1,{...base,id:`req${index}aaaaaa`},ratio,16384-2048),{ratioBasisPoints:ratio,rampMilliseconds:300000,samples:5,quick:false})},arrivedAtMs:index,pid:1}));
 const admitted=(policy:typeof before,ratio:number)=>evaluateHostCapacityV1(policy,sample,[],queue(ratio),1_000_000).decisions.filter(row=>row.verdict==="admit").length;
 expect(admitted(before,10000)).toBe(1);
 expect(admitted(BUILD_HOST_CAPACITY_POLICY_V1,10000)).toBe(1);
 expect(admitted(BUILD_HOST_CAPACITY_POLICY_V1,3000),"a learned 30 % reservation admits exactly the ramp slots at once, not the whole burst (the memory those runs really reach is the law in memory-fit)").toBe(BUILD_HOST_CAPACITY_POLICY_V1.memory.maximumRamping);
 const unshrunk=(ratio:number)=>queue(ratio).map(waiter=>({...waiter,request:{...waiter.request,rampSlot:false}}));
 expect(evaluateHostCapacityV1(BUILD_HOST_CAPACITY_POLICY_V1,sample,[],unshrunk(3000),1_000_000).decisions.filter(row=>row.verdict==="admit").length,"without slots the memory alone would admit four").toBeGreaterThanOrEqual(4);
});

test("A person at a terminal and an explicit interactive standing rank first, agents rank as agents, and the capacity queue reads the one function of the lease queue",()=>{
 const standing=(environment:Record<string,string>,client:string)=>leasePriorityClassV1(environment,{agentSession:client!=="human",interactiveTerminal:client==="human"});
 expect(standing({},"human")).toBe("interactive");
 expect(standing({},"claude")).toBe("agent");
 expect(standing({SEMIO_LEASE_PRIORITY:"interactive"},"codex")).toBe("interactive");
 expect(standing({SEMIO_LEASE_PRIORITY:"background"},"human")).toBe("background");
 expect(standing({SEMIO_LEASE_PRIORITY:"urgent"},"human")).toBe("interactive");
});
