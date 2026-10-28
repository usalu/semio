import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import {spawn,spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import fixtureSchema from "./🧫️fixtures/🧬️schema.json";
import componentSchema from "../🧬️schema/🔣️.json";
import descriptorNamespaceSchema from "../../../../../⚡️caching/🦀️cargo/📦️units/📥️descriptor/🔒️namespace/🧬️schema/🔣️.json" with {type:"json"};
import {BAZEL_SESSION_POLICY_V1 as policy,acquireBazelSessionV1,bazelGrowthAllowanceV1,bazelNestedServersV1,bazelRepositoryContentsV1,bazelRestoreResourcesV1,bazelServerOverrideV1,bazelSessionCommandV1,bazelSessionMessageV1,bazelSessionResourcesV1,bazelSessionStartupV1,bazelSessionStatusV1,bazelSessionWorkspaceV1,bazelSlotIdsV1,nextBazelSlotStateV1,planBazelServerReapV1,planBazelServersV1,planBazelSessionGarbageV1,rankBazelSlotsV1,renderBazelSessionStatusV1,withBazelSessionV1,type BazelSlotPortV1,type BazelSlotStateV1} from "../🟦️.ts";
import {bazelFreeBytesV1,bazelSlotPortV1,collectBazelPoolV1,dedupeBazelLaunchersV1,collectBazelSessionsV1,measureBazelTreeBytesV1,readBazelInventoryV1,readBazelServerPidV1,reapBazelServersV1,removeBazelTreeV1,stopBazelServerV1} from "../🔌️port/🟦️.ts";
import {startScriptProcessWatchdogV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(componentSchema,"sessions.json");
const key=(index:number)=>index.toString(16).padStart(16,"0"),abort=()=>new AbortController().signal;

/** 🧪️ The port in memory: claims are a set, the gate is a first-in-first-out mutex, time is a counter the sleep advances, and a hook lets other sessions act while one waits. */
function memoryPort(onSleep:(count:number)=>void|Promise<void>=()=>undefined){
 const held=new Set<string>(),states=new Map<string,BazelSlotStateV1>(),lines:string[]=[],writes:string[]=[],tails=new Map<string,Promise<void>>(),queues=new Map<string,number[]>(),events:string[]=[];const idle={megabytes:0},image={bytes:Infinity};let time=0,sleeps=0,gates=0,tickets=0;
 const port:BazelSlotPortV1={
  async claim(slot){if(held.has(slot))return undefined;held.add(slot);return()=>{held.delete(slot);};},
  async gate(signal,kind){signal.throwIfAborted();const previous=tails.get(kind)??Promise.resolve();let open!:()=>void;tails.set(kind,new Promise<void>(done=>{open=done;}));await previous;gates++;events.push('gate');return()=>{gates--;events.push('open');open();};},
  async spare(){return idle.megabytes;},
  async floorSpare(){return Infinity;},
  async imageFree(){return image.bytes;},
  queued:kind=>(queues.get(kind)??[]).length,
  enqueue:kind=>{const id=++tickets,list=queues.get(kind)??[];list.push(id);queues.set(kind,list);return {first:()=>list[0]===id,leave:()=>{const at=list.indexOf(id);if(at>=0)list.splice(at,1);}};},
  async read(slot){return states.get(slot);},
  async write(slot,state){writes.push(slot);states.set(slot,state);},
  async sleep(milliseconds,signal){signal.throwIfAborted();events.push(gates>0?'sleep-under-gate':'sleep');time+=milliseconds;sleeps++;await new Promise(done=>setImmediate(done));await onSleep(sleeps);signal.throwIfAborted();},
  now:()=>time,
  progress:line=>{lines.push(line);},
 };
 return {port,held,states,lines,writes,events,idle,image,gates:()=>gates};
}

test("the closed fixture agrees with its closed schema and the component schema",()=>{
 const validate=ajv.compile(fixtureSchema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const name of ["Policy","Messages"])expect(typeof (componentSchema.definitions as Record<string,{const?:unknown}>)[name]?.const).toBe("object");
 expect(policy.names.slotPattern).toBe("^[sd][0-9]{1,2}$");expect(policy.servers.maximum).toBeLessThanOrEqual(99);
});

test("the server plan bounds Bazel servers by memory above the floor, cores and policy exactly like an independent Python implementation",()=>{
 for(const row of fixture.plans){const plan=planBazelServersV1(row.host,row.memory);expect(plan as unknown,row.name).toEqual(row.expected);expect(plan.slots,row.name).toBeGreaterThanOrEqual(policy.servers.minimum);expect(plan.slots).toBeLessThanOrEqual(policy.servers.maximum);expect(plan.heapMegabytes).toBeLessThanOrEqual(policy.servers.heapMegabytes);}
 expect(()=>planBazelServersV1({cores:0,memoryMegabytes:1},fixture.plans[0]!.memory)).toThrow("Closed Bazel server host required");
 expect(()=>planBazelServersV1(fixture.plans[0]!.host,{...fixture.plans[0]!.memory,budgetBasisPoints:1} as never)).toThrow("Closed Bazel server host required");
 expect(bazelSlotIdsV1({slots:3,services:4})).toEqual(["s0","s1","s2"]);
 const wide=fixture.plans.find(row=>row.expected.slots===policy.servers.maximum)!;expect(wide.expected.limit).toBe("policy");
});

test("the number of servers can be overridden explicitly and anything unreadable is refused",()=>{
 const base=planBazelServersV1(fixture.plans[0]!.host,fixture.plans[0]!.memory);
 for(const row of fixture.overrides){
  if(row.slots==="refused")expect(()=>bazelServerOverrideV1(base,row.environment),JSON.stringify(row.environment)).toThrow("SEMIO_BAZEL_SERVERS");
  else{const plan=bazelServerOverrideV1(base,row.environment);expect(plan.slots as unknown).toBe(row.slots??base.slots);expect(plan.cpuPerServer).toBeGreaterThanOrEqual(1);expect(plan.memoryPerServerMegabytes).toBeGreaterThanOrEqual(256);}
 }
 expect(bazelSessionResourcesV1(base)).toEqual([`--local_resources=cpu=${base.cpuPerServer}`,`--local_resources=memory=${base.memoryPerServerMegabytes}`]);
 expect(bazelSessionStartupV1(base,600)).toEqual(["--max_idle_secs=600",`--host_jvm_args=-Xmx${base.heapMegabytes}m`,...policy.servers.jvmArguments.map(argument=>`--host_jvm_args=${argument}`)]);expect(policy.servers.jvmArguments).toEqual(["-XX:G1PeriodicGCInterval=10000","-XX:MinHeapFreeRatio=10","-XX:MaxHeapFreeRatio=30"]);expect(()=>bazelSessionStartupV1(base,0)).toThrow("Positive");
 expect(bazelSessionCommandV1("/c/contents")).toEqual(["--repo_contents_cache=/c/contents"]);expect(bazelRepositoryContentsV1("/p/bazel/outputs")).toBe("/p/bazel/contents");
});

test("a nested session gets overflow slots so it never waits for the slot its parent holds",async()=>{
 const base=planBazelServersV1(fixture.plans[0]!.host,fixture.plans[0]!.memory),nested=bazelNestedServersV1(base,true);
 expect(bazelNestedServersV1(base,false)).toBe(base);expect(nested.slots).toBe(base.slots+policy.servers.nestedExtra);expect(Math.min(99,nested.slots)).toBe(nested.slots);
 const memory=memoryPort(),parents=await Promise.all(Array.from({length:base.slots},(_,index)=>acquireBazelSessionV1(memory.port,base,{language:"en",key:key(index),signal:abort()})));
 const child=await acquireBazelSessionV1(memory.port,nested,{language:"en",key:key(99),signal:abort(),timeoutMilliseconds:0});expect(parents.some(row=>row.slot===child.slot)).toBe(false);expect(Number(child.slot.slice(1))).toBeGreaterThanOrEqual(base.slots);
 await child.release();for(const parent of parents)await parent.release();
});

test("long-running service sessions pin service slots only: any number of dev servers up to the service bound never block a build, and the two queues are independent",async()=>{
 const memory=memoryPort(),plan={slots:3,services:4};
 const services=await Promise.all(Array.from({length:plan.services},(_,index)=>acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(index),signal:abort(),kind:"service",timeoutMilliseconds:0})));
 expect(services.map(row=>row.slot)).toEqual(["d0","d1","d2","d3"]);
 const builds=await Promise.all(Array.from({length:plan.slots},(_,index)=>acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(10+index),signal:abort(),timeoutMilliseconds:0})));
 expect(builds.map(row=>row.slot)).toEqual(["s0","s1","s2"]);expect(memory.lines.some(line=>line.includes("waiting"))).toBe(false);
 const controller=new AbortController(),fifth=acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(20),signal:controller.signal,kind:"service"});void fifth.catch(()=>undefined);
 await builds[0]!.release();const next=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(21),signal:abort(),timeoutMilliseconds:0});expect(next.slot).toBe("s0");
 for(let turn=0;turn<20;turn++)await new Promise(done=>setImmediate(done));
 expect(memory.lines.some(line=>line==="[bazel] all 4 Bazel service slots are in use; waiting for one (0 s)")).toBe(true);expect(bazelSessionMessageV1("waiting-service",{slots:4,seconds:1},"de")).toBe("[bazel] alle 4 Bazel-Dienst-Slots sind belegt; warte auf einen freien Slot (1 s)");
 controller.abort(Error("stop waiting"));await new Promise(done=>setTimeout(done,20));await expect(fifth).rejects.toThrow("stop waiting");expect(memory.gates()).toBe(0);
 await services[0]!.release();const sixth=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(22),signal:abort(),kind:"service",timeoutMilliseconds:0});expect(sixth.slot).toBe("d0");
 for(const row of [...services.slice(1),...builds.slice(1),next,sixth])await row.release();expect(memory.held.size).toBe(0);
 expect(new Set([...bazelSlotIdsV1(plan,"build"),...bazelSlotIdsV1(plan,"service")]).size).toBe(plan.slots+plan.services);
});

test("service slots have their own count and override and the plan carries both",()=>{
 const base=planBazelServersV1(fixture.plans[0]!.host,fixture.plans[0]!.memory);expect(base.services).toBe(policy.servers.serviceMaximum);
 expect(bazelServerOverrideV1(base,{SEMIO_BAZEL_SERVICES:"7"}).services).toBe(7);expect(bazelServerOverrideV1(base,{SEMIO_BAZEL_SERVICES:"7"}).slots).toBe(base.slots);
 expect(bazelServerOverrideV1(base,{SEMIO_BAZEL_SERVERS:"2",SEMIO_BAZEL_SERVICES:"1"})).toMatchObject({slots:2,services:1});
 for(const value of ["0","100","x"])expect(()=>bazelServerOverrideV1(base,{SEMIO_BAZEL_SERVICES:value})).toThrow("SEMIO_BAZEL_SERVICES");
 expect(bazelNestedServersV1(base,true).services).toBe(base.services);
});

test("the slot order follows warm, unused, then least recently used and agrees with an SQL ORDER BY",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of fixture.ranks){
   const states=row.states as Record<string,BazelSlotStateV1|undefined>,order=rankBazelSlotsV1(row.slots,states,row.key);expect(order as unknown,row.name).toEqual(row.expected);
   database.exec("DROP TABLE IF EXISTS slots;CREATE TABLE slots(slot TEXT,idx INTEGER,tier INTEGER,used INTEGER)");
   for(const slot of row.slots){const state=states[slot],tier=state===undefined?1:state.key===row.key?0:2;database.query("INSERT INTO slots VALUES(?1,?2,?3,?4)").run(slot,Number(slot.slice(1)),tier,state?.usedAt??0);}
   const sql=database.query("SELECT slot FROM slots ORDER BY tier, CASE WHEN tier=0 THEN -used WHEN tier=2 THEN used ELSE 0 END, idx").all() as {slot:string}[];
   expect(sql.map(entry=>entry.slot),row.name+" (sqlite)").toEqual(order as string[]);
  }
 }finally{database.close();}
});

test("package paths are short, slot-scoped and refuse every shape outside the authored patterns",()=>{
 for(const row of fixture.workspaces.accepted)expect(bazelSessionWorkspaceV1(fixture.workspaces.root,row.slot,row.key)).toBe(row.path);
 for(const row of fixture.workspaces.refused)expect(()=>bazelSessionWorkspaceV1(fixture.workspaces.root,row.slot,row.key),JSON.stringify(row)).toThrow("Bazel session");
 expect(()=>bazelSessionWorkspaceV1("/"+"x".repeat(250),"s0",key(1))).toThrow("256");
 expect(bazelSessionWorkspaceV1("/p",  "s0",key(1))).not.toBe(bazelSessionWorkspaceV1("/p","s1",key(1)));
 expect(bazelSessionWorkspaceV1("/p","s0",key(1)),"every selection of a slot lives at the one path its server is bound to").toBe(bazelSessionWorkspaceV1("/p","s0",key(2)));expect(bazelSessionWorkspaceV1("/p","s0",key(1)).endsWith("/"+policy.names.stablePackage)).toBe(true);expect(policy.names.stablePackage).toMatch(new RegExp(policy.names.keyPattern));
});

test("messages render in English and German exactly like str.format and the language follows the environment",()=>{
 for(const row of fixture.messages){expect(bazelSessionMessageV1(row.id,row.values as Record<string,string|number>,"en")).toBe(row.en);expect(bazelSessionMessageV1(row.id,row.values as Record<string,string|number>,"de")).toBe(row.de);}
 expect(()=>bazelSessionMessageV1("nope",{},"en")).toThrow("Unknown");expect(()=>bazelSessionMessageV1("waiting",{slots:1,services:4},"en")).toThrow("Missing");
 for(const row of fixture.languages){const pick=()=>resolveLanguageV1(CATALOG_LANGUAGES_V1,[],row.environment,()=>[]).language as string;if(row.expected===null)expect(pick,JSON.stringify(row.environment)).toThrow("--lang=en|de");else expect(pick(),JSON.stringify(row.environment)).toBe(row.expected);}
 expect(Object.keys(componentSchema.definitions.Messages.const.en).sort()).toEqual(Object.keys(componentSchema.definitions.Messages.const.de).sort());
});

test("three sessions claim three slots at once, a fourth waits behind the fair gate with progress and takes the slot a session frees",async()=>{
 const memory=memoryPort(async count=>{if(count===2)await first.release();}),plan={slots:3,services:4};
 const first=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(1),signal:abort()}),second=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(2),signal:abort()}),third=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(3),signal:abort()});
 expect([first.slot,second.slot,third.slot]).toEqual(["s0","s1","s2"]);expect(new Set([first.mode,second.mode,third.mode])).toEqual(new Set(["fresh"]));expect(memory.gates()).toBe(0);
 const fourth=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(4),signal:abort()});
 expect(fourth.slot).toBe("s0");expect(fourth.mode).toBe("switch");expect(fourth.previous?.key).toBe(key(1));
 expect(memory.lines.some(line=>line==="[bazel] all 3 Bazel server slots are in use; waiting for one (0 s)")).toBe(true);expect(memory.gates()).toBe(0);
 await second.release();await third.release();await fourth.release();expect(memory.held.size).toBe(0);
});

test("a selection returns to the slot of its warm server and the same selection twice at once lands in two slots",async()=>{
 const memory=memoryPort(),plan={slots:3,services:4};
 const a=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(1),signal:abort()}),b=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(2),signal:abort()});await a.release();await b.release();
 const again=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(2),signal:abort()}),twice=await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(2),signal:abort()});
 expect(again.slot).toBe("s1");expect(again.mode).toBe("warm");expect(twice.slot).not.toBe(again.slot);expect(twice.mode).toBe(twice.slot==="s0"?"switch":"fresh");
 expect(bazelSessionWorkspaceV1("/w",again.slot,key(2))).not.toBe(bazelSessionWorkspaceV1("/w",twice.slot,key(2)));
 await again.release();await twice.release();
 const kept=memory.states.get("s0")!;expect(kept.keys.length).toBeLessThanOrEqual(policy.retention.packagesPerSlot);expect(kept.keys[0]).toBe(kept.key);
});

test("waiting honours the timeout and the cancellation, and a failed or cancelled session never leaks a slot or the gate",async()=>{
 const full=memoryPort(),plan={slots:2,services:4},sessions=[await acquireBazelSessionV1(full.port,plan,{language:"en",key:key(1),signal:abort()}),await acquireBazelSessionV1(full.port,plan,{language:"en",key:key(2),signal:abort()})];
 await expect(acquireBazelSessionV1(full.port,plan,{language:"en",key:key(3),signal:abort(),timeoutMilliseconds:1000})).rejects.toThrow("timed out");expect(full.gates()).toBe(0);
 const controller=new AbortController(),waiting=acquireBazelSessionV1(full.port,plan,{language:"en",key:key(3),signal:controller.signal});controller.abort(Error("cancelled by the caller"));await expect(waiting).rejects.toThrow("cancelled by the caller");expect(full.gates()).toBe(0);expect(full.held.size).toBe(2);
 for(const session of sessions)await session.release();expect(full.held.size).toBe(0);
 const failing=memoryPort();await expect(withBazelSessionV1(failing.port,plan,{language:"en",key:key(1),signal:abort()},async()=>{throw Error("body failed");})).rejects.toThrow("body failed");expect(failing.held.size).toBe(0);expect(failing.writes).toEqual(["s0"]);
 const broken=memoryPort(),brokenPort:BazelSlotPortV1={...broken.port,async write(){throw Error("disk full");}};
 await expect(withBazelSessionV1(brokenPort,plan,{language:"en",key:key(1),signal:abort()},async()=>1)).rejects.toThrow("disk full");expect(broken.held.size).toBe(0);
 await expect(withBazelSessionV1(brokenPort,plan,{language:"en",key:key(1),signal:abort()},async()=>{throw Error("body first");})).rejects.toThrow("body first");expect(broken.held.size).toBe(0);
 const session=await acquireBazelSessionV1(memoryPort().port,plan,{language:"en",key:key(1),signal:abort()});await session.release();await session.release();
});

test("progress lines are German when the language says so and the status table lists held and free slots",async()=>{
 const memory=memoryPort(),plan={slots:2,services:4},held=await acquireBazelSessionV1(memory.port,plan,{key:key(7),signal:abort(),language:"de"});
 expect(memory.lines.at(-1)).toBe("[bazel] Sitzungs-Slot s0: frischer Server");
 const rows=await bazelSessionStatusV1(memory.port,plan,abort());expect(rows.map(row=>row.held).slice(0,2)).toEqual([true,false]);expect(rows.length).toBe(6);
 expect(renderBazelSessionStatusV1(rows,"en")).toContain("in use");expect(renderBazelSessionStatusV1(rows,"de")).toContain("belegt");expect(renderBazelSessionStatusV1(rows,"de")).toContain("frei");
 expect(memory.held.has("s0")).toBe(true);expect(memory.held.has("s1")).toBe(false);await held.release();
 expect(nextBazelSlotStateV1("s3",undefined,key(1),5).keys).toEqual([key(1)]);
});

test("garbage collection decisions agree with an independent Python implementation",()=>{
 for(const row of fixture.garbage)expect(planBazelSessionGarbageV1(row.inventory as never,{slots:row.slots,services:row.services},row.now,new Set(row.held),policy,(row as {pressure?:{freeBytes:number;softBytes:number;targetBytes:number}}).pressure) as unknown,row.name).toEqual(row.expected);
 expect(()=>planBazelSessionGarbageV1({slots:[{slot:"x",outputBase:true,packages:[]}],foreign:[]} as never,{slots:1,services:4},0)).toThrow("Closed Bazel session inventory required");
});

const oracle=(rows:readonly unknown[],mode:"garbage"|"plan"|"growth"|"reap"="garbage"):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify({policy,rows,mode}))});expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);return JSON.parse(new TextDecoder().decode(run.stdout));};

test("the garbage plan equals the independent Python oracle on every fixture row and on 400 seeded random inventories with pressure, sizes, held slots and leftover capture folders",()=>{
 const rows=fixture.garbage.map(row=>({inventory:row.inventory,slots:row.slots,services:row.services,now:row.now,held:row.held,...(row as {pressure?:unknown}).pressure?{pressure:(row as {pressure?:unknown}).pressure}:{}}));
 expect(oracle(rows)).toEqual(fixture.garbage.map(row=>row.expected));
 let seed=20261024;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},pick=<T,>(values:readonly T[])=>values[next(values.length)]!,now=86400000*200,random:Record<string,unknown>[]=[];
 for(let index=0;index<400;index++){
  const slots=pick(["s0","s1","s2","s3","s7","s14","d0","d1","d3","d9"] as const),used=new Set<string>(),entries=[];
  for(const slot of ["s0","s1","s2","s5","s14","d0","d2","d9"].filter(()=>next(4)>0)){
   const keys=Array.from({length:next(6)},()=>key(next(8)+1)).filter((value,at,all)=>all.indexOf(value)===at),sized=next(2)===0,withBase=next(2)===0;used.add(slot);
   entries.push({slot,outputBase:withBase,...(sized&&withBase?{outputBaseBytes:next(4000)*1048576}:{}),...next(3)>0&&keys.length?{state:{version:1,slot,key:pick(keys),keys:[...keys],usedAt:now-next(30*24)*3600000}}:{},packages:keys.map(value=>({key:value,modifiedAt:now-next(12*24*60)*60000,...(sized?{bytes:next(3000)*1048576}:{}),...next(4)===0?{temporary:true}:{}}))});
  }
  random.push({inventory:{slots:entries,foreign:next(2)?[]:["server/"+key(99)]},slots:next(4)+1,services:next(4)+1,now,held:[...used].filter(()=>next(5)===0),...next(3)>0?{pressure:{freeBytes:next(8192)*1048576,softBytes:3221225472,targetBytes:6442450944}}:{}});
  void slots;
 }
 const expected=oracle(random);
 random.forEach((row,at)=>expect(planBazelSessionGarbageV1(row.inventory as never,{slots:row.slots as number,services:row.services as number},now,new Set(row.held as string[]),policy,row.pressure as never) as unknown,JSON.stringify(row)).toEqual(expected[at]));
});

test("under pressure a real collection measures the trees, frees the oldest unprotected packages and leftover capture folders, and never touches a claimed slot",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-pressure-")),directories={leases:join(root,"l"),server:join(root,"o"),workspaces:join(root,"w"),pool:join(root,"p")};
 try{
  const port=bazelSlotPortV1(directories),write=(path:string,bytes:number)=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,Buffer.alloc(bytes,1));};
  write(join(root,"p/ab/"+"a".repeat(62)),1000);
  mkdirSync(join(directories.server,"s0/execroot"),{recursive:true});mkdirSync(join(directories.server,"s1/execroot"),{recursive:true});
  await port.write("s0",nextBazelSlotStateV1("s0",undefined,key(1),Date.now()));await port.write("s1",nextBazelSlotStateV1("s1",undefined,key(4),Date.now()));
  for(const[slot,name,bytes]of[["s0",key(1),65536],["s0",key(2),262144],["s0",key(3),131072],["s1",key(4),65536],["s1",key(5),262144]] as const)write(join(directories.workspaces,slot,name,"a/file"),bytes);
  mkdirSync(join(directories.workspaces,"s0",key(1),"snapshot-tmp"),{recursive:true});writeFileSync(join(directories.workspaces,"s0",key(1),"snapshot-tmp/0"),"half");
  const old=(Date.now()-3*3600000)/1000;for(const[slot,name]of[["s0",key(2)],["s0",key(3)],["s1",key(5)]] as const)utimesSync(join(directories.workspaces,slot,name),old,old);
  expect(await measureBazelTreeBytesV1(join(directories.workspaces,"s0",key(2)),abort())).toBeGreaterThanOrEqual(262144);
  const signal=abort(),claimed=await port.claim("s1",signal),lines:string[]=[],reclaim={softAvailableBytes:3221225472,targetAvailableBytes:6442450944},free=reclaim.softAvailableBytes-1;
  const result=await collectBazelSessionsV1(directories,port,{slots:3,services:4},signal,async()=>undefined,{freeBytes:()=>free,reclaim,language:"de",progress:line=>lines.push(line)});
  expect(result.packages.map(row=>row.slot+row.key+row.reason)).toEqual(["s0"+key(2)+"pressure","s0"+key(3)+"pressure"]);expect(result.temporaries).toEqual([{slot:"s0",key:key(1)}]);expect(result.outputBases).toEqual([]);
  expect(readdirSync(join(directories.workspaces,"s0"))).toEqual([key(1)]);expect(existsSync(join(directories.workspaces,"s0",key(1),"snapshot-tmp"))).toBe(false);expect(readdirSync(join(directories.workspaces,"s1")).sort()).toEqual([key(4),key(5)]);
  expect(existsSync(join(root,"p/ab/"+"a".repeat(62)))).toBe(true);
  expect(lines[0]).toContain("das Produkt-Image hat");expect(lines.some(line=>line.includes("Paket "+key(2)))).toBe(true);expect(lines.at(-1)).toContain("freigegeben");claimed!();
  const quiet=await collectBazelSessionsV1(directories,port,{slots:3,services:4},signal,async()=>undefined,{language:"en",freeBytes:()=>reclaim.softAvailableBytes*2,reclaim,progress:line=>lines.push(line)});
  expect(quiet.packages).toEqual([]);expect(readdirSync(join(directories.workspaces,"s1")).sort()).toEqual([key(4),key(5)]);
  expect(bazelFreeBytesV1(join(root,"absent/deeper"))).toBeGreaterThan(0);
 }finally{await removeBazelTreeV1(root);}
},60000);

test("under pressure the plan counts only what a free slot can give: candidates of a claimed slot never use up the need",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-claimed-")),directories={leases:join(root,"l"),server:join(root,"o"),workspaces:join(root,"w")};
 try{
  const port=bazelSlotPortV1(directories),write=(path:string,bytes:number)=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,Buffer.alloc(bytes,1));};
  for(const slot of["s0","s1"]){mkdirSync(join(directories.server,slot,"execroot"),{recursive:true});await port.write(slot,nextBazelSlotStateV1(slot,undefined,key(slot==="s0"?1:4),Date.now()));}
  for(const[slot,name,bytes,age]of[["s0",key(1),65536,0],["s0",key(2),262144,3],["s0",key(3),262144,5],["s1",key(4),65536,0],["s1",key(5),262144,8]] as const){write(join(directories.workspaces,slot,name,"a/file"),bytes);const when=(Date.now()-age*3600000)/1000;utimesSync(join(directories.workspaces,slot,name),when,when);}
  const signal=abort(),claimed=await port.claim("s1",signal),free=1000000,reclaim={softAvailableBytes:free+1,targetAvailableBytes:free+100000};
  const result=await collectBazelSessionsV1(directories,port,{slots:3,services:4},signal,async()=>undefined,{language:"en",freeBytes:()=>free,reclaim});
  expect(result.packages.map(row=>row.slot+row.key)).toEqual(["s0"+key(3)]);expect(readdirSync(join(directories.workspaces,"s1")).sort()).toEqual([key(4),key(5)]);expect(readdirSync(join(directories.workspaces,"s0")).sort()).toEqual([key(1),key(2)]);claimed!();
 }finally{await removeBazelTreeV1(root);}
},60000);

test("the session that collects keeps its own selection's package under pressure, however old it is",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-keep-")),directories={leases:join(root,"l"),server:join(root,"o"),workspaces:join(root,"w")};
 try{
  const port=bazelSlotPortV1(directories),write=(path:string,bytes:number)=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,Buffer.alloc(bytes,1));};
  mkdirSync(join(directories.server,"s0/execroot"),{recursive:true});await port.write("s0",nextBazelSlotStateV1("s0",undefined,key(1),Date.now()));
  for(const[name,age]of[[key(1),0],[key(2),3],[key(3),5]] as const){write(join(directories.workspaces,"s0",name,"a/file"),65536);const when=(Date.now()-age*3600000)/1000;utimesSync(join(directories.workspaces,"s0",name),when,when);}
  const signal=abort(),reclaim={softAvailableBytes:1000001,targetAvailableBytes:2000000},held=await port.claim("s0",signal);
  const result=await collectBazelSessionsV1(directories,port,{slots:3,services:4},signal,async()=>undefined,{language:"en",holding:"s0",keep:key(2),freeBytes:()=>1000,reclaim});
  expect(result.packages.map(row=>row.slot+row.key)).toEqual(["s0"+key(3)]);expect(readdirSync(join(directories.workspaces,"s0")).sort()).toEqual([key(1),key(2)]);held!();
 }finally{await removeBazelTreeV1(root);}
},60000);

test("under pressure an unheld growth slot is retired whole, its idle server stopped first, while a younger growth slot, a planned slot and a claimed growth slot stay",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-growth-")),directories={leases:join(root,"l"),server:join(root,"o"),workspaces:join(root,"w")};
 try{
  const port=bazelSlotPortV1(directories),write=(path:string,bytes:number)=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,Buffer.alloc(bytes,1));},now=Date.now();
  for(const[slot,id,idleMinutes]of[["s0",1,0],["s3",2,60],["s4",3,2],["s5",4,90]] as const){
   write(join(directories.server,slot,"execroot/file"),131072);write(join(directories.workspaces,slot,key(id),"a/file"),262144);await port.write(slot,{...nextBazelSlotStateV1(slot,undefined,key(id),now),usedAt:now-idleMinutes*60000});
  }
  const signal=abort(),claimed=await port.claim("s5",signal),stopped:string[]=[],lines:string[]=[],reclaim={softAvailableBytes:1000001,targetAvailableBytes:100000000};
  const result=await collectBazelSessionsV1(directories,port,{slots:3,services:4},signal,async base=>{stopped.push(base.slice(directories.server.length+1));expect(existsSync(base)).toBe(true);},{language:"en",freeBytes:()=>1000,reclaim,progress:line=>lines.push(line)});
  expect(result.packages.map(row=>row.slot+row.reason)).toEqual(["s3growth"]);expect(result.outputBases.map(row=>row.slot+row.reason)).toEqual(["s3growth"]);expect(stopped).toEqual(["s3"]);
  expect(existsSync(join(directories.server,"s3"))).toBe(false);expect(existsSync(join(directories.server,"s3.session.json"))).toBe(false);expect(readdirSync(join(directories.workspaces,"s3"))).toEqual([]);
  for(const slot of["s0","s4","s5"])expect(existsSync(join(directories.server,slot)),slot).toBe(true);expect(readdirSync(join(directories.workspaces,"s4")).length).toBe(1);claimed!();
 }finally{await removeBazelTreeV1(root);}
},60000);

test("the content pool evicts entries by the time they entered it, keeps younger ones, removes emptied folders and leaves everything in a dry run",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-pool-")),pool=join(root,"pool");
 try{
  for(const name of["ab/"+"1".repeat(62),"ab/"+"2".repeat(62)+".x","cd/"+"3".repeat(62)]){mkdirSync(join(pool,name,".."),{recursive:true});writeFileSync(join(pool,name),Buffer.alloc(4096,1));}
  const hour=3600000,maximumAgeMs=policy.pool.maximumAgeHours*hour,signal=abort();
  expect(await collectBazelPoolV1(pool,signal,{now:Date.now(),maximumAgeMs})).toEqual({entries:0,bytes:0});
  const dry=await collectBazelPoolV1(pool,signal,{now:Date.now()+maximumAgeMs+hour,maximumAgeMs,dryRun:true});expect(dry.entries).toBe(3);expect(dry.bytes).toBeGreaterThanOrEqual(3*4096);expect(readdirSync(join(pool,"ab")).length).toBe(2);
  const real=await collectBazelPoolV1(pool,signal,{now:Date.now()+maximumAgeMs+hour,maximumAgeMs});expect(real).toEqual(dry);expect(readdirSync(pool)).toEqual([]);
  expect(await collectBazelPoolV1(join(root,"absent"),signal,{now:0,maximumAgeMs:0})).toEqual({entries:0,bytes:0});
  const stopped=new AbortController();stopped.abort();mkdirSync(join(pool,"ef"),{recursive:true});await expect(collectBazelPoolV1(pool,stopped.signal,{now:0,maximumAgeMs:0})).rejects.toThrow();
 }finally{await removeBazelTreeV1(root);}
});

test("every session message exists in both languages with the same placeholders",()=>{
 const english=componentSchema.definitions.Messages.const.en as Record<string,string>,german=componentSchema.definitions.Messages.const.de as Record<string,string>,holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 expect(Object.keys(german).sort()).toEqual(Object.keys(english).sort());for(const id of Object.keys(english))expect(holes(german[id]!),id).toEqual(holes(english[id]!));
});

test("restore locks are hierarchical: two restores conflict exactly when one tree contains the other, against a brute-force path oracle",()=>{
 for(const row of fixture.restore)expect(bazelRestoreResourcesV1(row.roots) as unknown,row.name).toEqual(row.expected);
 for(const row of fixture.restoreRefused)expect(()=>bazelRestoreResourcesV1(row.roots),row.name).toThrow("Invalid Bazel output snapshot");
 const universe=["a","a/b","a/b/c","a/b/d","a/e","f","f/g","f/g/h"],conflict=(left:string,right:string)=>left===right||right.startsWith(left+"/")||left.startsWith(right+"/");
 for(const left of universe)for(const right of universe){
  const first=bazelRestoreResourcesV1([left]),second=bazelRestoreResourcesV1([right]),shared=first.filter(a=>second.some(b=>b.resource===a.resource&&(a.mode==="exclusive"||b.mode==="exclusive")));
  expect(shared.length>0,`${left} vs ${right}`).toBe(conflict(left,right));
 }
 const unrelated=[bazelRestoreResourcesV1(["a/b/c"]),bazelRestoreResourcesV1(["f/g/h"])];expect(unrelated[0]!.filter(a=>unrelated[1]!.some(b=>b.resource===a.resource&&(a.mode==="exclusive"||b.mode==="exclusive")))).toEqual([]);
});

/** 🔰️ Starts a child process that claims a slot through the production port and holds it until its input closes. */
function holder(directories:{leases:string;server:string;workspaces:string},slot:string):Promise<{kill:()=>void;exited:Promise<void>}>{
 const code=`import {bazelSlotPortV1} from ${JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim(${JSON.stringify(slot)},new AbortController().signal);console.log(claim?"held":"busy");if(claim){await new Promise(done=>process.stdin.on("close",done).resume());claim();}`;
 const script=join(directories.server,"..","holder.ts");mkdirSync(join(directories.server,".."),{recursive:true});writeFileSync(script,code);
 const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]}),exited=new Promise<void>(done=>child.on("exit",()=>done()));
 return new Promise((resolve,reject)=>{child.stdout.setEncoding("utf8");child.stdout.on("data",text=>{if(text.includes("held"))resolve({kill:()=>child.kill("SIGKILL"),exited});else reject(Error("child did not claim "+text));});child.on("error",reject);});
}

test("the production port claims slots through real repository leases across processes, frees a killed holder's slot and keeps states",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")};
 try{
  const port=bazelSlotPortV1(directories),signal=abort(),child=await holder(directories,"s0");
  expect(await port.claim("s0",signal)).toBeUndefined();const other=await port.claim("s1",signal);expect(other).toBeDefined();other!();
  const rows=await bazelSessionStatusV1(port,{slots:2,services:4},signal);expect(rows.map(row=>row.held).slice(0,2)).toEqual([true,false]);
  child.kill();await child.exited;
  let reclaimed:(()=>void)|undefined;for(let attempt=0;attempt<50&&reclaimed===undefined;attempt++){reclaimed=await port.claim("s0",signal);if(reclaimed===undefined)await new Promise(done=>setTimeout(done,100));}
  expect(reclaimed,"a killed holder frees its slot").toBeDefined();reclaimed!();
  const release=await port.gate(signal,"build");release();
  const state=nextBazelSlotStateV1("s0",undefined,key(1),Date.now());await port.write("s0",state);expect(existsSync(join(directories.server,"s0.session.json")),"nothing is written without an output base").toBe(false);mkdirSync(join(directories.server,"s0"),{recursive:true});mkdirSync(join(directories.server,"s1"),{recursive:true});await port.write("s0",state);expect(await port.read("s0")).toEqual(state);expect(await port.read("s1")).toBeUndefined();
  writeFileSync(join(directories.server,"s1.session.json"),"{");expect(await port.read("s1")).toBeUndefined();writeFileSync(join(directories.server,"s1.session.json"),JSON.stringify({...state,slot:"s0"}));expect(await port.read("s1"),"a state of another slot is ignored").toBeUndefined();
  expect(()=>port.read("../x")).toThrow("Bazel session slot required");
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("the production port keeps one fair queue per slot class",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")};
 try{
  const port=bazelSlotPortV1(directories,{write:()=>undefined}),signal=abort(),buildGate=await port.gate(signal,"build");
  const service=await acquireBazelSessionV1(port,{slots:2,services:2},{language:"en",key:key(1),signal,kind:"service",timeoutMilliseconds:20000});expect(service.slot).toBe("d0");await service.release();
  buildGate();const build=await acquireBazelSessionV1(port,{slots:2,services:2},{language:"en",key:key(2),signal,timeoutMilliseconds:20000});expect(build.slot).toBe("s0");await build.release();
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("two sessions through the production port end in two slots and a third waits until one releases",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},lines:string[]=[];
 try{
  const port=bazelSlotPortV1(directories,{write:line=>lines.push(line)}),plan={slots:2,services:4},signal=abort();
  const one=await acquireBazelSessionV1(port,plan,{language:"en",key:key(1),signal}),two=await acquireBazelSessionV1(port,plan,{language:"en",key:key(2),signal});expect(new Set([one.slot,two.slot]).size).toBe(2);
  let third:string|undefined;const waiting=acquireBazelSessionV1(port,plan,{language:"en",key:key(3),signal,timeoutMilliseconds:20000}).then(async session=>{third=session.slot;await session.release();});
  await new Promise(done=>setTimeout(done,700));expect(third).toBeUndefined();await one.release();await waiting;expect(third).toBe(one.slot);await two.release();
  expect(lines.some(line=>line.includes("waiting for one"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("the inventory reads slots, packages and foreign entries and the collector removes what the plan names, stops servers first and skips held slots",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},day=86400000;
 try{
  const port=bazelSlotPortV1(directories),stopped:string[]=[],layout=(path:string,files:string[])=>{for(const file of files){mkdirSync(join(path,file,".."),{recursive:true});writeFileSync(join(path,file),"x");}};
  layout(directories.server,["s0/execroot/x","s14/execroot/x","s0.events.json","metrics.json","0123456789abcdef/execroot/x","0123456789abcdef.events.json"]);
  await port.write("s14",nextBazelSlotStateV1("s14",undefined,key(5),Date.now()));await port.write("s0",nextBazelSlotStateV1("s0",undefined,key(1),Date.now()));
  for(const name of [key(1),key(2),key(3)]){layout(join(directories.workspaces,"s0"),[name+"/MODULE.bazel"]);}
  layout(join(directories.workspaces),["0123456789abcdef/MODULE.bazel","s14/"+key(5)+"/MODULE.bazel"]);
  const old=(Date.now()-20*day)/1000;utimesSync(join(directories.workspaces,"s0",key(2)),old,old);utimesSync(join(directories.workspaces,"s0",key(3)),old,old);utimesSync(join(directories.workspaces,"s0",key(1)),Date.now()/1000,Date.now()/1000);
  mkdirSync(join(directories.server,"s14/execroot/locked"),{recursive:true});writeFileSync(join(directories.server,"s14/execroot/locked/ro"),"x");if(process.platform!=="win32"){chmodSync(join(directories.server,"s14/execroot/locked/ro"),0o444);chmodSync(join(directories.server,"s14/execroot/locked"),0o555);}
  const inventory=await readBazelInventoryV1(directories,port);
  expect(inventory.slots.map(row=>row.slot)).toEqual(["s0","s14"]);expect(inventory.slots[0]!.packages.length).toBe(3);expect(inventory.foreign).toEqual(["server/0123456789abcdef","workspaces/0123456789abcdef"]);
  const signal=abort(),dry=await collectBazelSessionsV1(directories,port,{slots:2,services:4},signal,async()=>undefined,{language:"en",dryRun:true});expect(dry.outputBases).toEqual([{slot:"s14",reason:"capacity"}]);expect(existsSync(join(directories.server,"s14"))).toBe(true);
  const heldZero=await port.claim("s0",signal),heldTwo=await port.claim("s14",signal),skipped=await collectBazelSessionsV1(directories,port,{slots:2,services:4},signal,async base=>{stopped.push(base);},{language:"en"});
  expect(skipped.outputBases).toEqual([]);expect(skipped.packages).toEqual([]);expect(skipped.foreign).toEqual(["server/0123456789abcdef","workspaces/0123456789abcdef"]);expect(stopped).toEqual([join(directories.server,"0123456789abcdef")]);expect(existsSync(join(directories.server,"s14"))).toBe(true);expect(readdirSync(join(directories.workspaces,"s0")).length).toBe(3);heldZero!();heldTwo!();
  stopped.length=0;const removed=await collectBazelSessionsV1(directories,port,{slots:2,services:4},signal,async base=>{stopped.push(base.slice(directories.server.length+1));expect(existsSync(base)).toBe(true);},{language:"en"});
  expect(removed.outputBases).toEqual([{slot:"s14",reason:"capacity"}]);expect(removed.packages.map(row=>row.slot+row.key)).toEqual(["s0"+key(2),"s0"+key(3),"s14"+key(5)]);expect(removed.foreign).toEqual([]);expect(stopped).toEqual(["s14"]);
  expect(readdirSync(directories.server).sort()).toEqual(["metrics.json","s0","s0.events.json","s0.session.json"]);expect(readdirSync(join(directories.workspaces,"s0"))).toEqual([key(1)]);expect(existsSync(join(directories.workspaces,"0123456789abcdef"))).toBe(false);
  if(process.platform!=="win32"){mkdirSync(join(root,"ro/inner"),{recursive:true});writeFileSync(join(root,"ro/inner/f"),"x");chmodSync(join(root,"ro/inner/f"),0o444);chmodSync(join(root,"ro/inner"),0o555);await removeBazelTreeV1(join(root,"ro"));expect(existsSync(join(root,"ro"))).toBe(false);}
 }finally{await removeBazelTreeV1(root);}
},60000);

test("a server is stopped by removing its pid file, without the Bazel binary, and a missing server is already stopped",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-stop-")),base=join(root,"s0");
 try{
  mkdirSync(join(base,"server"),{recursive:true});await stopBazelServerV1(base,abort());await stopBazelServerV1(join(root,"absent"),abort());
  const pidFile=join(base,"server/server.pid.txt"),watcher=spawn(process.execPath,["-e","const f=process.argv[1];setInterval(()=>{if(!require('fs').existsSync(f))process.exit(0)},50)",pidFile],{stdio:"ignore"}),exited=new Promise<number|null>(done=>watcher.on("exit",code=>done(code)));
  writeFileSync(pidFile,String(watcher.pid));await stopBazelServerV1(base,abort());expect(await exited).toBe(0);expect(existsSync(pidFile)).toBe(false);
  const stubborn=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});writeFileSync(pidFile,String(stubborn.pid));
  await expect(stopBazelServerV1(base,abort(),{timeoutMilliseconds:300,pollMilliseconds:50})).rejects.toThrow("did not stop");stubborn.kill("SIGKILL");
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);

test("the adapter holds a slot instead of a per-selection lease and no other Bazel code names the old scheme",()=>{
 const script=readFileSync(join(import.meta.dir,"../../📜️script.ts"),"utf8"),storage=readFileSync(join(import.meta.dir,"../../💾️storage/🟦️.ts"),"utf8");
 expect(script).toContain("withBazelSlotV1(root,bazelSelectionV1(graph,request),phaseControl,hold=>withBazelStorageV1(");expect(script).toContain("withBazelSlotV1(root,{key,kind:\"build\"},phaseControl,hold=>withBazelStorageV1(");expect(script).toContain("task.definition.continuous");
 expect(script).not.toContain('"bazel-workspace-"');expect(script).not.toContain('"bazel-snapshot-restore"');expect(script).not.toContain("-Xmx2048m");expect(readFileSync(join(import.meta.dir,"../../🎬️executor/🟦️.ts"),"utf8")).toContain("bazelRestoreResourcesV1(");
 expect(script).toContain("bazelSessionWorkspaceV1(storage.paths.workspace,slot,namespace)");expect(script).not.toContain("join(storage.paths.workspace,namespace)");expect(storage).not.toContain("createHash");
 expect(script).toContain("session.slot");expect(policy.leases.restorePrefix.startsWith("bazel-restore")).toBe(true);
});


/** 🧮️ Independent first-in-first-out model of the waiter queue: the order in which `sessions` arrivals take `slots` slots when one slot is freed per round. */
function claimOrderOracle(arrivals:number,slots:number):number[]{
 const order:number[]=[],queue:number[]=[];
 for(let arrival=0;arrival<arrivals;arrival++){if(arrival<slots&&queue.length===0)order.push(arrival);else queue.push(arrival);}
 while(queue.length)order.push(queue.shift()!);
 return order;
}

test("the picker is a short critical section: nobody sleeps under it, and waiters are served first come first served exactly like an independent queue model",async()=>{
 const memory=memoryPort(),plan={slots:2,services:1},releases:Array<()=>Promise<void>>=[],claimed:number[]=[];
 const start=(index:number)=>acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(index),signal:abort(),timeoutMilliseconds:86_400_000}).then(session=>{claimed.push(index);releases.push(()=>session.release());return session;});
 const pending:Promise<unknown>[]=[];
 for(let index=0;index<6;index++){pending.push(start(index));for(let turn=0;turn<30;turn++)await new Promise(done=>setImmediate(done));}
 for(let round=0;round<4;round++){await new Promise(done=>setTimeout(done,20));const release=releases.shift();expect(release).toBeDefined();await release!();}
 await Promise.all(pending);
 expect(claimed).toEqual(claimOrderOracle(6,2));
 expect(memory.events.includes("sleep-under-gate")).toBe(false);expect(memory.events.includes("sleep")).toBe(true);expect(memory.gates()).toBe(0);
 for(const release of releases)await release();expect(memory.held.size).toBe(0);
});


/** ⏸️ Starts a child process that waits for a build slot through the production port and a real session, reports `claimed` and holds the slot until its input closes. */
function waiter(directories:{leases:string;server:string;workspaces:string},index:number):{lines:string[];kill:()=>void;release:()=>void;claimed:()=>Promise<void>}{
 const code=`import {bazelSlotPortV1} from ${JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts"))};import {acquireBazelSessionV1} from ${JSON.stringify(join(import.meta.dir,"../🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)},{write:()=>undefined});const session=await acquireBazelSessionV1(port,{slots:2,services:1},{language:"en",key:${JSON.stringify(key(index))},signal:new AbortController().signal,growth:false,timeoutMilliseconds:120000});console.log("claimed");await new Promise(done=>process.stdin.on("close",done).resume());await session.release();`;
 const script=join(directories.server,"..",`waiter-${index}.ts`);writeFileSync(script,code);
 const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]}),lines:string[]=[];child.stdout.setEncoding("utf8");child.stdout.on("data",text=>{lines.push(text.trim());});
 return {lines,kill:()=>child.kill("SIGKILL"),release:()=>child.stdin.end(),claimed:async()=>{for(let attempt=0;attempt<300&&lines.length===0;attempt++)await new Promise(done=>setTimeout(done,50));expect(lines,"waiter "+index+" claimed").toEqual(["claimed"]);}};
}

test("real processes waiting for a Bazel slot never hold the picker: the gate stays free for others, a killed waiter is skipped and the others are served in arrival order",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},children:{kill:()=>void}[]=[];
 try{
  mkdirSync(directories.server,{recursive:true});const port=bazelSlotPortV1(directories,{write:()=>undefined}),waiting=()=>readdirSync(directories.server).filter(name=>name.startsWith("waiter-build-")).length;
  const first=await holder(directories,"s0"),second=await holder(directories,"s1");children.push(first,second);
  const arrivals:ReturnType<typeof waiter>[]=[];
  for(const index of [0,1,2,3]){const row=waiter(directories,index);arrivals.push(row);children.push(row);for(let attempt=0;attempt<300&&waiting()<=index;attempt++)await new Promise(done=>setTimeout(done,50));expect(waiting(),"arrival "+index).toBe(index+1);}
  for(let probe=0;probe<8;probe++){const begun=Date.now(),release=await port.gate(AbortSignal.timeout(2000),"build");release();expect(Date.now()-begun,"the gate is free while four runs wait").toBeLessThan(1000);await new Promise(done=>setTimeout(done,100));}
  arrivals[1]!.kill();
  first.kill();await arrivals[0]!.claimed();expect(arrivals.map(row=>row.lines.length)).toEqual([1,0,0,0]);
  second.kill();await arrivals[2]!.claimed();expect(arrivals.map(row=>row.lines.length)).toEqual([1,0,1,0]);
  arrivals[0]!.release();await arrivals[3]!.claimed();expect(arrivals.map(row=>row.lines.length)).toEqual([1,0,1,1]);
  for(const row of arrivals)row.release();for(let attempt=0;attempt<100&&waiting()>0;attempt++)await new Promise(done=>setTimeout(done,50));expect(waiting()).toBe(0);
 }finally{for(const child of children)try{child.kill();}catch{}rmSync(root,{recursive:true,force:true});}
},120000);

test("slot growth follows the live host: with every planned slot in use a build opens an extra slot only while idle memory reaches the authored margin, up to the authored maximum, and never for services or when the caller pinned the count",async()=>{
 const memory=memoryPort(),plan={slots:3,services:2},sessions:Awaited<ReturnType<typeof acquireBazelSessionV1>>[]=[];
 for(let index=0;index<3;index++)sessions.push(await acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(index),signal:abort(),timeoutMilliseconds:0}));
 expect(sessions.map(row=>row.slot)).toEqual(["s0","s1","s2"]);
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(10),signal:abort(),timeoutMilliseconds:0})).rejects.toThrow("timed out");
 memory.idle.megabytes=policy.servers.growthSpareMegabytes-1;
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(11),signal:abort(),timeoutMilliseconds:0})).rejects.toThrow("timed out");
 memory.idle.megabytes=policy.servers.growthSpareMegabytes;memory.image.bytes=policy.servers.growthImageFreeBytes-1;
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(13),signal:abort(),timeoutMilliseconds:0})).rejects.toThrow("timed out");
 expect(memory.lines.filter(line=>line.includes("products image")).length,"reported once per acquisition").toBe(1);memory.image.bytes=Infinity;
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(12),signal:abort(),timeoutMilliseconds:0,growth:false})).rejects.toThrow("timed out");
 const grown:Awaited<ReturnType<typeof acquireBazelSessionV1>>[]=[],more=(index:number)=>acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(index),signal:abort(),timeoutMilliseconds:0});
 grown.push(await more(20));
 await expect(more(21),"one margin of idle memory affords one extra server, however many sessions see the same headroom").rejects.toThrow("timed out");
 memory.idle.megabytes=policy.servers.growthSpareMegabytes*2;grown.push(await more(22));
 await expect(more(23),"two margins afford two").rejects.toThrow("timed out");
 memory.idle.megabytes=policy.servers.growthSpareMegabytes*(policy.servers.growthMaximum-plan.slots);
 for(let index=grown.length;index<policy.servers.growthMaximum-plan.slots;index++)grown.push(await more(30+index));
 expect(grown.map(row=>row.slot)).toEqual(Array.from({length:policy.servers.growthMaximum-plan.slots},(_,index)=>"s"+(plan.slots+index)));
 expect(memory.lines.filter(line=>line.includes("extra slot")).length).toBe(grown.length);
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(40),signal:abort(),timeoutMilliseconds:0})).rejects.toThrow("timed out");
 const services=await Promise.all(Array.from({length:plan.services},(_,index)=>acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(50+index),signal:abort(),kind:"service",timeoutMilliseconds:0})));
 await expect(acquireBazelSessionV1(memory.port,plan,{language:"en",key:key(60),signal:abort(),kind:"service",timeoutMilliseconds:0})).rejects.toThrow("timed out");
 for(const row of [...sessions,...grown,...services])await row.release();expect(memory.held.size).toBe(0);
 expect(bazelSessionMessageV1("grown",{slot:"s3",spare:5000},"de")).toContain("zusätzlichen Slot s3");
});

test("slot growth equals an independent Python model of the decision for every combination of used slots, busy growth slots and idle memory",async()=>{
 const rows:Array<{used:number;idle:number;image:number;kind:string;growth:boolean;slots:number;services:number;busyGrowth:number}>=[];
 for(const busyGrowth of [0,1,2,3,9])for(const idle of [0,2559,4095,4096,8191,8192,20000,10**6])for(const kind of ["build","service"])for(const image of [0,6442450943,6442450944,10**12])for(const growth of [true,false])for(const used of kind==="build"?[0,2,3]:[0,1,2])rows.push({used,idle,image,kind,growth,slots:3,services:2,busyGrowth});
 const expected=oracle(rows.map(row=>row),"growth") as string[];
 for(const [index,row] of rows.entries()){
  const memory=memoryPort();memory.idle.megabytes=row.idle;memory.image.bytes=row.image;const ids=row.kind==="build"?["s0","s1","s2"]:["d0","d1"];
  for(const slot of ids.slice(0,row.used))memory.held.add(slot);
  if(row.kind==="build"&&row.used>=3)for(let at=0;at<row.busyGrowth;at++)memory.held.add("s"+(3+at));
  const outcome=await acquireBazelSessionV1(memory.port,{slots:3,services:2},{language:"en",key:key(1),signal:abort(),kind:row.kind as "build"|"service",growth:row.growth,timeoutMilliseconds:0}).then(session=>ids.includes(session.slot)?"base":"growth",()=>"wait");
  expect(outcome,JSON.stringify(row)).toBe(expected[index]);
 }
 expect(policy.servers.growthMaximum).toBe(12);expect(policy.servers.growthSpareMegabytes).toBe(4096);expect(bazelGrowthAllowanceV1(8191)).toBe(1);expect(bazelGrowthAllowanceV1(-5)).toBe(0);
});

/** ⌛️ A child that holds the only slot, waits for it through the production port with the real stall watchdog on (stall 1 s, exit 124 like the envelope) and releases the slot after 3.5 s; `plain` swaps the port's wait for an undeclared timer. */
function slotWaitChild(directories:{leases:string;server:string;workspaces:string},plain:boolean):string{
 const script=join(directories.server,"..",plain?"wait-plain.ts":"wait-declared.ts"),module=JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts")),sessions=JSON.stringify(join(import.meta.dir,"../🟦️.ts")),watchdog=JSON.stringify(join(import.meta.dir,"../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts"));
 mkdirSync(join(directories.server,".."),{recursive:true});
 writeFileSync(script,`import {bazelSlotPortV1} from ${module};import {acquireBazelSessionV1} from ${sessions};import {observeScriptProcessOutputV1,startScriptProcessWatchdogV1} from ${watchdog};
const signal=new AbortController().signal,declared=bazelSlotPortV1(${JSON.stringify(directories)}),plan={slots:1,services:1},port=${plain?"{...declared,sleep:(milliseconds)=>new Promise<void>(done=>setTimeout(done,milliseconds))}":"declared"};
observeScriptProcessOutputV1();startScriptProcessWatchdogV1({children:()=>[],checkMilliseconds:100,onStall:()=>process.exit(124)});
const holder=await acquireBazelSessionV1(declared,plan,{language:"en",key:"0000000000000001",signal,growth:false});
const waiting=acquireBazelSessionV1(port,plan,{language:"en",key:"0000000000000002",signal,growth:false,timeoutMilliseconds:20000}).then(session=>session.release());
setTimeout(()=>void holder.release(),3500);await waiting;process.exit(0);`);
 return script;
}

test("a wait for a Bazel server slot is a declared queue wait: the stall watchdog never stops it, and the same wait on an undeclared timer is stopped (exit codes observed by an independent Python process)",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")};
 try{
  const scripts={declared:slotWaitChild(directories,false),plain:slotWaitChild(directories,true)};
  const program="import json,os,subprocess,sys\nscripts=json.load(sys.stdin)\nout={}\nfor name,path in scripts.items():\n  env=dict(os.environ,SEMIO_SCRIPT_STALL_MS='1000')\n  out[name]=subprocess.run([sys.argv[1],path],env=env,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=60).returncode\nprint(json.dumps(out))";
  const run=Bun.spawnSync(["python3","-I","-c",program,process.execPath],{stdin:new TextEncoder().encode(JSON.stringify(scripts))});expect(run.exitCode).toBe(0);
  expect(JSON.parse(new TextDecoder().decode(run.stdout))).toEqual({declared:0,plain:124});
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("the slot wait of the production port is declared in process: the watchdog sees no stall while sessions queue for a slot",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")};
 let stalls=0;const stop=startScriptProcessWatchdogV1({children:()=>[],stallMilliseconds:100,checkMilliseconds:10,onStall:()=>{stalls++;}});
 try{
  const port=bazelSlotPortV1(directories,{write:()=>undefined}),plan={slots:1,services:1},signal=abort(),holding=await acquireBazelSessionV1(port,plan,{language:"en",key:key(1),signal,growth:false});
  const waiting=acquireBazelSessionV1(port,plan,{language:"en",key:key(2),signal,growth:false,timeoutMilliseconds:20000}).then(session=>session.release());
  await new Promise(done=>setTimeout(done,1200));expect(stalls).toBe(0);await holding.release();await waiting;
 }finally{stop();rmSync(root,{recursive:true,force:true});}
},60000);

test("Bazel slot choice: concurrent agent sessions start on different slots, an edit loop keeps its slot, and the slot state records who used it",async()=>{
 const slots=bazelSlotIdsV1({slots:3,services:0},"build"),key="0123456789abcdef";
 expect(rankBazelSlotsV1(slots,{},key)).toEqual(["s0","s1","s2"]);
 for(const preference of[0,1,2,3,4,5]){const ranked=rankBazelSlotsV1(slots,{},key,preference);expect(ranked[0]).toBe(slots[preference%3]);expect([...ranked].sort()).toEqual(["s0","s1","s2"]);}
 const states={s1:nextBazelSlotStateV1("s1",undefined,key,100)},kept=rankBazelSlotsV1(slots,states,key,2);expect(kept[0]).toBe("s1");
 const holds=new Set<string>(),written:Record<string,unknown>={};
 const port={async claim(slot:string){if(holds.has(slot))return undefined;holds.add(slot);return ()=>holds.delete(slot);},async gate(){return ()=>undefined;},queued(){return 0;},async spare(){return 0;},async floorSpare(){return Infinity;},enqueue(){return {first:()=>true,leave(){}};},async read(slot:string){return written[slot] as never;},async write(slot:string,state:unknown){written[slot]=state;},async sleep(){},now:()=>1000,progress(){}};
 const signal=new AbortController().signal,first=await acquireBazelSessionV1(port as never,{slots:3,services:0},{language:"en",key,signal,preference:1,owner:{session:"0123456789ab.sa",agent:"alpha"}}),second=await acquireBazelSessionV1(port as never,{slots:3,services:0},{language:"en",key:"fedcba9876543210",signal,preference:1,owner:{session:"0123456789ab.sb",agent:"beta"}});
 expect([first.slot,second.slot]).toEqual(["s1","s2"]);await first.release();await second.release();
 expect(written.s1).toMatchObject({session:"0123456789ab.sa",agent:"alpha",key});expect(written.s2).toMatchObject({agent:"beta"});
 const again=await acquireBazelSessionV1(port as never,{slots:3,services:0},{language:"en",key,signal,preference:2});expect(again.slot).toBe("s1");expect(again.mode).toBe("warm");await again.release();
});

test("Bazel slot choice: selections that once shared a slot, and selections that alternate, each keep a slot of their own and stay warm",async()=>{
 const a="0000000000000001",b="0000000000000002",c="0000000000000003",plan={slots:3,services:0},written:Record<string,BazelSlotStateV1>={s0:nextBazelSlotStateV1("s0",nextBazelSlotStateV1("s0",undefined,a,1),b,2)},holds=new Set<string>();let clock=10;
 const port={async claim(slot:string){if(holds.has(slot))return undefined;holds.add(slot);return ()=>holds.delete(slot);},async gate(){return ()=>undefined;},queued(){return 0;},async spare(){return 0;},async floorSpare(){return Infinity;},enqueue(){return {first:()=>true,leave(){}};},async read(slot:string){return written[slot] as never;},async write(slot:string,state:unknown){written[slot]=state as BazelSlotStateV1;},async sleep(){},now:()=>clock++,progress(){}};
 const signal=new AbortController().signal,seen:Record<string,{slot:string;mode:string}[]>={[a]:[],[b]:[],[c]:[]};
 for(let round=0;round<4;round++)for(const key of[a,b,c]){const session=await acquireBazelSessionV1(port as never,plan,{language:"en",key,signal});await session.release();seen[key]!.push({slot:session.slot,mode:session.mode});}
 for(const key of[a,b,c]){expect(new Set(seen[key]!.map(row=>row.slot)).size,key+" never moves").toBe(1);expect(seen[key]!.slice(1).map(row=>row.mode),key+" stays warm").toEqual(["warm","warm","warm"]);}
 expect(new Set([a,b,c].map(key=>seen[key]![0]!.slot)).size).toBe(3);
});

test("launcher dedupe turns identical large launcher copies of an idle output base into clones of one file, leaves everything else alone and does not repeat, agreeing with an independent Python sha256",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"dedupe-")),base=join(root,"s0"),bin=join(base,"execroot","_main","bazel-out","darwin_arm64-fastbuild","bin"),size=policy.dedupe.minimumBytes+4096,same=Buffer.alloc(size,7),other=Buffer.alloc(size,9),place=(name:string,bytes:Buffer,mode=0o555)=>{const path=join(bin,name);writeFileSync(path,bytes);chmodSync(path,mode);utimesSync(path,new Date(1_700_000_000_000),new Date(1_700_000_000_000));return path;};
 mkdirSync(join(bin,"t_h.runfiles"),{recursive:true});
 const copies=["t_a","t_b","t_c","t_d","t_e"].map(name=>place(name,same)),different=place("t_f",other),small=place("t_g",Buffer.alloc(1<<20,7)),nested=join(bin,"t_h.runfiles","t_i");writeFileSync(nested,same);
 const before=new Map([...copies,different,small,nested].map(path=>[path,statSync(path).ino])),digest=(paths:string[])=>{const run=Bun.spawnSync(["python3","-I","-c","import hashlib,sys\nfor path in sys.argv[1:]:\n print(hashlib.sha256(open(path,'rb').read()).hexdigest())",...paths]);expect(run.exitCode).toBe(0);return run.stdout.toString().trim().split("\n");};
 const expected=digest([...copies,different,small,nested]),first=await dedupeBazelLaunchersV1(base,abort());
 expect(first).toEqual({files:copies.length,bytes:copies.length*size});
 expect(digest([...copies,different,small,nested])).toEqual(expected);
 for(const path of copies){expect(statSync(path).ino,path).not.toBe(before.get(path)!);expect(statSync(path).mode&0o777,path).toBe(0o555);expect(statSync(path).mtimeMs,path).toBe(1_700_000_000_000);}
 for(const path of[different,small,nested])expect(statSync(path).ino,path).toBe(before.get(path)!);
 const shared=readdirSync(join(base,policy.dedupe.directory)).filter(name=>name!==policy.dedupe.stamp);expect(shared.length).toBe(1);expect(shared[0]).toMatch(/^[0-9a-f]{32}$/u);
 const inodes=new Map([...copies].map(path=>[path,statSync(path).ino]));expect(await dedupeBazelLaunchersV1(base,abort())).toEqual({files:0,bytes:0});
 for(const path of copies)expect(statSync(path).ino,path).toBe(inodes.get(path)!);
 expect(await dedupeBazelLaunchersV1(join(root,"absent"),abort())).toEqual({files:0,bytes:0});
});
test("launcher dedupe messages exist in every language and the pressure collector reports them",()=>{for(const language of["en","de"] as const)expect(bazelSessionMessageV1("deduped",{slot:"s0",files:69,megabytes:4100},language)).toContain("69");});

test("the server plan equals the independent Python oracle on a grid of hosts and both floor rules, and a 16 GB laptop gets two capped servers",()=>{
 const rows:{host:{cores:number;memoryMegabytes:number};memory:{floorBasisPoints:number;floorMinimumMegabytes:number;floorMaximumMegabytes:number}}[]=[];
 for(const cores of [1,2,3,4,8,10,16,32])for(const memoryMegabytes of [2048,4096,6144,8192,12288,16384,24576,32768,65536,131072])for(const memory of [{floorBasisPoints:2000,floorMinimumMegabytes:4096,floorMaximumMegabytes:8192},{floorBasisPoints:1250,floorMinimumMegabytes:1536,floorMaximumMegabytes:4096}])rows.push({host:{cores,memoryMegabytes},memory});
 const expected=oracle(rows,"plan");rows.forEach((row,index)=>expect(planBazelServersV1(row.host,row.memory) as unknown,JSON.stringify(row)).toEqual(expected[index]));
 const laptop=planBazelServersV1({cores:10,memoryMegabytes:16384},{floorBasisPoints:1250,floorMinimumMegabytes:1536,floorMaximumMegabytes:4096});
 expect(laptop.slots).toBe(2);expect(laptop.heapMegabytes).toBe(1536);expect(laptop.slots*laptop.residentMegabytes,"the planned servers fit the authored share of the memory above the floor").toBeLessThanOrEqual((16384-2048)*policy.servers.memoryShareBasisPoints/10000);
 expect(planBazelServersV1({cores:10,memoryMegabytes:32768},{floorBasisPoints:1250,floorMinimumMegabytes:1536,floorMaximumMegabytes:4096}).slots).toBe(5);
});

test("the reap plan keeps the most recently used idle servers the budget leaves room for, stops the others least recently used first, and equals the independent Python oracle",()=>{
 const rows=[{slot:"s0",live:true,held:true,usedAt:5},{slot:"s1",live:true,held:false,usedAt:9},{slot:"s2",live:true,held:false,usedAt:1},{slot:"s3",live:true,held:false,usedAt:4},{slot:"s4",live:false,held:false,usedAt:7},{slot:"s5",live:false,held:true,usedAt:0}];
 expect(planBazelServerReapV1(rows,3)).toEqual(["s2","s3"]);expect(planBazelServerReapV1(rows,2)).toEqual(["s2","s3","s1"]);expect(planBazelServerReapV1(rows,5)).toEqual([]);expect(planBazelServerReapV1(rows,1,100)).toEqual(["s2","s3","s1"]);
 expect(planBazelServerReapV1(rows,5,-1),"below the memory floor no idle server stays").toEqual(["s2","s3","s1"]);expect(planBazelServerReapV1([],2)).toEqual([]);expect(()=>planBazelServerReapV1(rows,-1)).toThrow("budget");
 let seed=7;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},random:{rows:typeof rows;budget:number;headroom?:number}[]=[];
 for(let index=0;index<500;index++)random.push({rows:Array.from({length:next(9)},(_,at)=>({slot:"s"+at,live:next(3)>0,held:next(3)===0,usedAt:next(6)})),budget:next(6),...next(4)===0?{headroom:next(3000)-1500}:{}});
 const expected=oracle(random,"reap");random.forEach((row,index)=>expect(planBazelServerReapV1(row.rows,row.budget,row.headroom) as unknown,JSON.stringify(row)).toEqual(expected[index]));
});

test("the reaper stops surplus idle servers of free slots through their pid files, least recently used first, never a held slot, and reports in both languages; below the floor it stops every idle one",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-reap-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},children:ReturnType<typeof spawn>[]=[];
 try{
  const watch=(slot:string,usedAt:number)=>{const base=join(directories.server,slot),file=join(base,"server/server.pid.txt");mkdirSync(join(base,"server"),{recursive:true});const child=spawn(process.execPath,["-e","const f=process.argv[1];setInterval(()=>{if(!require('fs').existsSync(f))process.exit(0)},50)",file],{stdio:"ignore"});children.push(child);writeFileSync(file,String(child.pid));writeFileSync(join(directories.server,slot+".session.json"),JSON.stringify({version:1,slot,key:key(1),keys:[key(1)],usedAt}));return new Promise<number|null>(done=>child.on("exit",code=>done(code)));};
  const gone=async(exit:Promise<number|null>)=>await Promise.race([exit.then(()=>true),new Promise<boolean>(done=>setTimeout(()=>done(false),3000))]);
  const exits={s0:watch("s0",30),s1:watch("s1",90),s2:watch("s2",10),s3:watch("s3",20)},lines:string[]=[],headroom={value:5000};
  const port=bazelSlotPortV1(directories,{spare:async()=>headroom.value,floorSpare:async()=>headroom.value}),signal=abort(),held=await port.claim("s1",signal),plan={slots:3,services:1};
  expect(await readBazelServerPidV1(join(directories.server,"s0"))).toBeGreaterThan(0);expect(await readBazelServerPidV1(join(directories.server,"s9"))).toBeUndefined();
  const stopped=await reapBazelServersV1(directories,port,plan,signal,base=>stopBazelServerV1(base,signal),{budget:3,language:"en",progress:line=>lines.push(line)});
  expect(stopped,"3 budget - 1 held = 2 idle kept (s0 30, s3 20); s2 is the oldest").toEqual(["s2"]);expect(await gone(exits.s2)).toBe(true);
  expect(lines.length).toBe(1);expect(lines[0]).toBe(bazelSessionMessageV1("reaped",{slot:"s2",live:4,budget:3},"en"));expect(bazelSessionMessageV1("reaped",{slot:"s2",live:4,budget:3},"de")).toContain("untätigen Bazel-Server von Slot s2");
  expect(await readBazelServerPidV1(join(directories.server,"s0"))).toBeGreaterThan(0);expect(await readBazelServerPidV1(join(directories.server,"s1"))).toBeGreaterThan(0);expect(await readBazelServerPidV1(join(directories.server,"s3"))).toBeGreaterThan(0);
  expect(await reapBazelServersV1(directories,port,plan,signal,base=>stopBazelServerV1(base,signal),{budget:3,language:"en"}),"nothing more to stop").toEqual([]);
  headroom.value=-100;const sampled=bazelSlotPortV1(directories,{spare:async()=>headroom.value,floorSpare:async()=>headroom.value}),pressed=await reapBazelServersV1(directories,sampled,plan,signal,base=>stopBazelServerV1(base,signal),{budget:3,language:"de",progress:line=>lines.push(line)});
  expect(pressed,"below the floor every idle server stops, the held one stays").toEqual(["s3","s0"]);expect(await gone(exits.s3)).toBe(true);expect(await gone(exits.s0)).toBe(true);expect(await readBazelServerPidV1(join(directories.server,"s1"))).toBeGreaterThan(0);
  expect(lines.at(-1)).toContain("untätigen");held!();
  const failing=await reapBazelServersV1(directories,sampled,plan,signal,async()=>{throw Error("stuck");},{budget:0,language:"en",progress:line=>lines.push(line)});
  expect(failing,"a server that does not stop is reported, never thrown").toEqual([]);expect(lines.at(-1)).toBe(bazelSessionMessageV1("reap-failed",{slot:"s1",reason:"stuck"},"en"));expect(bazelSessionMessageV1("reap-failed",{slot:"s1",reason:"stuck"},"de")).toContain("wurde nicht gestoppt");
  expect(await gone(exits.s1)).toBe(false);
 }finally{for(const child of children)child.kill("SIGKILL");rmSync(root,{recursive:true,force:true});}
},40000);

test("Law: the reaper measures memory pressure by the memory that is really free above the floor, never by the capacity ledger: a negative ledger headroom with gigabytes free keeps every budgeted idle server, a really starved host stops them",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-reap-floor-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},children:ReturnType<typeof spawn>[]=[];
 try{
  const watch=(slot:string,usedAt:number)=>{const base=join(directories.server,slot),file=join(base,"server/server.pid.txt");mkdirSync(join(base,"server"),{recursive:true});const child=spawn(process.execPath,["-e","const f=process.argv[1];setInterval(()=>{if(!require('fs').existsSync(f))process.exit(0)},50)",file],{stdio:"ignore"});children.push(child);writeFileSync(file,String(child.pid));writeFileSync(join(directories.server,slot+".session.json"),JSON.stringify({version:1,slot,key:key(1),keys:[key(1)],usedAt}));return new Promise<number|null>(done=>child.on("exit",code=>done(code)));};
  const exits={s0:watch("s0",30),s1:watch("s1",20)},plan={slots:3,services:1},signal=abort(),state={ledger:-5000,free:9000};
  const port=bazelSlotPortV1(directories,{spare:async()=>state.ledger,floorSpare:async()=>state.free});
  expect(await port.spare(),"the ledger says the machine is overcommitted while grants ramp").toBe(-5000);
  const kept=await reapBazelServersV1(directories,port,plan,signal,base=>stopBazelServerV1(base,signal),{budget:3,language:"en"});
  expect(kept,"nine gigabytes are really free: the two idle servers stay warm for the next session").toEqual([]);
  expect(await readBazelServerPidV1(join(directories.server,"s0"))).toBeGreaterThan(0);expect(await readBazelServerPidV1(join(directories.server,"s1"))).toBeGreaterThan(0);
  const starved=bazelSlotPortV1(directories,{spare:async()=>state.ledger,floorSpare:async()=>-300}),stopped=await reapBazelServersV1(directories,starved,plan,signal,base=>stopBazelServerV1(base,signal),{budget:3,language:"en"});
  expect([...stopped].sort(),"a host that really lacks memory still gives it back").toEqual(["s0","s1"]);
  await Promise.all([exits.s0,exits.s1]);
 }finally{for(const child of children)child.kill("SIGKILL");rmSync(root,{recursive:true,force:true});}
},30000);

test.skipIf(!process.env.SEMIO_TEST_BAZEL_BINARY)("a real server whose client was killed mid-command stops through its pid file within the authored timeout, however busy it still is",async()=>{
 const binary=process.env.SEMIO_TEST_BAZEL_BINARY!,root=mkdtempSync(join(realpathSync(tmpdir()),"bz-orphan-")),workspace=join(root,"w"),base=join(root,"b"),startup=[`--output_user_root=${join(root,"u")}`,`--output_base=${base}`,"--max_idle_secs=15","--nosystem_rc","--nohome_rc"],alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}},sleepers:number[]=[];
 try{
  mkdirSync(workspace,{recursive:true});writeFileSync(join(workspace,"MODULE.bazel"),'module(name="fx", version="0")\n');writeFileSync(join(workspace,"BUILD.bazel"),'genrule(name="slow",outs=["slow.out"],cmd="sleep 240; echo done > $@",tags=["no-sandbox","no-cache"])\n');
  const waitFor=async<T,>(read:()=>T|undefined,seconds:number):Promise<T|undefined>=>{for(let tries=0;tries<seconds*5;tries++){const value=read();if(value!==undefined)return value;await new Promise(done=>setTimeout(done,200));}return undefined;};
  const pidOf=()=>{try{return Number(readFileSync(join(base,"server/server.pid.txt"),"utf8").trim())||undefined;}catch{return undefined;}};
  const children=(parent:number):number[]=>{const rows=spawnSync("ps",["-A","-o","pid=,ppid=,command="],{encoding:"utf8"}).stdout.split("\n"),found:number[]=[],pending=[parent];for(let at=pending.pop();at!==undefined;at=pending.pop())for(const row of rows){const match=/^\s*(\d+)\s+(\d+)\s+(.*)$/u.exec(row);if(match&&Number(match[2])===at){found.push(Number(match[1]));pending.push(Number(match[1]));if(match[3]!.includes("sleep 240"))sleepers.push(Number(match[1]));}}return found;};
  const client=spawn(binary,[...startup,"build","--spawn_strategy=local","//:slow"],{cwd:workspace,stdio:"ignore"});
  const server=await waitFor(pidOf,120);expect(server).toBeGreaterThan(0);await waitFor(()=>{children(server!);return sleepers.length?true:undefined;},120);await new Promise(done=>setTimeout(done,1500));children(server!);expect(sleepers.length,"the action is running").toBeGreaterThan(0);
  client.kill("SIGKILL");await new Promise(done=>setTimeout(done,1000));expect(alive(server!),"the server does not notice the dead client while its action runs").toBe(true);
  const started=Date.now();await stopBazelServerV1(base,abort());expect(Date.now()-started).toBeLessThan(policy.stop.timeoutMilliseconds);expect(alive(server!)).toBe(false);
 }finally{for(const pid of sleepers)try{process.kill(pid,"SIGKILL");}catch{}rmSync(root,{recursive:true,force:true});}
},180000);

import {acquireQueuedResourceLease,acquireResourceLease,withResourceLeases} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

test("the production picker is granted under a held graph snapshot pin while the lock order is enforced",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},names=["SEMIO_LEASE_ORDER","SEMIO_LEASE_ORDER_POLICY","SEMIO_LEASE_REGISTRY_DIRECTORY"] as const,saved=names.map(name=>process.env[name]);
 process.env.SEMIO_LEASE_ORDER="enforce";delete process.env.SEMIO_LEASE_ORDER_POLICY;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"registry");
 try{
  const port=bazelSlotPortV1(directories,{write:()=>undefined}),signal=abort(),pin=await acquireQueuedResourceLease({directory:directories.leases,resource:"build-graph:"+join(root,"g/m/snapshots/"+"a".repeat(64)+".json"),mode:"shared",owner:"law",signal});
  try{
   const build=await acquireBazelSessionV1(port,{slots:2,services:2},{language:"en",key:key(1),signal,timeoutMilliseconds:20000});expect(build.slot).toBe("s0");await build.release();
   const service=await acquireBazelSessionV1(port,{slots:2,services:2},{language:"en",key:key(2),signal,kind:"service",timeoutMilliseconds:20000});expect(service.slot).toBe("d0");await service.release();
  }finally{pin.release();}
 }finally{names.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});rmSync(root,{recursive:true,force:true});}
},60000);

test("the leases a build takes under its graph snapshot pin and its Rust code pin are granted while the lock order is enforced",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),leases=join(root,"leases"),names=["SEMIO_LEASE_ORDER","SEMIO_LEASE_ORDER_POLICY","SEMIO_LEASE_REGISTRY_DIRECTORY"] as const,saved=names.map(name=>process.env[name]);
 process.env.SEMIO_LEASE_ORDER="enforce";delete process.env.SEMIO_LEASE_ORDER_POLICY;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"registry");
 try{
  const signal=abort(),pins=[];
  for(const resource of ["rust-code:"+join(root,"rc/"+"b".repeat(64)),"build-graph:"+join(root,"g/m/snapshots/"+"a".repeat(64)+".json")])pins.push(await acquireQueuedResourceLease({directory:leases,resource,mode:"shared",owner:"law",signal}));
  try{
   for(const resource of ["bazel-"+"c".repeat(64),"bazel-metrics","bazel-modules-"+"d".repeat(64)]){const lease=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal,timeoutMs:5000});lease.release();}
   await withResourceLeases({directory:leases,signal,resources:[...bazelRestoreResourcesV1(["🎓️teaching/🏛️architecture/📦️course"])]},async()=>undefined);
   const toolchain=await acquireResourceLease({directory:leases,resource:"rustup:home",mode:"shared",signal,timeoutMs:5000});toolchain.release();
   const store=await acquireQueuedResourceLease({directory:leases,resource:"build-graph-store:"+join(root,"g/m"),mode:"exclusive",owner:"law",signal});store.release();
  }finally{for(const pin of pins)pin.release();}
 }finally{names.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});rmSync(root,{recursive:true,force:true});}
},60000);

test("the leases a Rust descriptor consumer takes under its namespace, and the maintenance writer's gate then namespace, are granted while the lock order is enforced",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),leases=join(root,"leases"),names=["SEMIO_LEASE_ORDER","SEMIO_LEASE_ORDER_POLICY","SEMIO_LEASE_REGISTRY_DIRECTORY"] as const,saved=names.map(name=>process.env[name]),protocol=descriptorNamespaceSchema.definitions.Protocol.const,namespace=join(root,"cargo/prototype/captures/units");
 process.env.SEMIO_LEASE_ORDER="enforce";delete process.env.SEMIO_LEASE_ORDER_POLICY;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"registry");
 try{
  const signal=abort(),slot=await acquireResourceLease({directory:leases,resource:"bazel-slot-s0",mode:"exclusive",signal,timeoutMs:0}),reader=await acquireResourceLease({directory:leases,resource:protocol.resourcePrefix+namespace,mode:"shared",signal,timeoutMs:5000});
  try{
   for(const resource of ["bazel-"+"c".repeat(64),"bazel-metrics","bazel-modules-"+"d".repeat(64)]){const lease=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal,timeoutMs:5000});lease.release();}
   const store=await acquireQueuedResourceLease({directory:leases,resource:"build-graph-store:"+join(root,"g/m"),mode:"exclusive",owner:"law",signal});store.release();
  }finally{reader.release();slot.release();}
  const gate=await acquireResourceLease({directory:leases,resource:protocol.admissionPrefix+namespace,mode:"exclusive",signal,timeoutMs:5000});
  try{const writer=await acquireResourceLease({directory:leases,resource:protocol.resourcePrefix+namespace,mode:"exclusive",signal,timeoutMs:5000});writer.release();}finally{gate.release();}
 }finally{names.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});rmSync(root,{recursive:true,force:true});}
},60000);
