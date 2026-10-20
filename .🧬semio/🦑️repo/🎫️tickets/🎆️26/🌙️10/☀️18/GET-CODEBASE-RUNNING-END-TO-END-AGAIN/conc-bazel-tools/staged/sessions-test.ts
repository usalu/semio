import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import {spawn} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import fixtureSchema from "./🧫️fixtures/🧬️schema.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {BAZEL_SESSION_POLICY_V1 as policy,acquireBazelSessionV1,bazelNestedServersV1,bazelRepositoryContentsV1,bazelRestoreResourcesV1,bazelServerOverrideV1,bazelSessionCommandV1,bazelSessionLanguageV1,bazelSessionMessageV1,bazelSessionResourcesV1,bazelSessionStartupV1,bazelSessionStatusV1,bazelSessionWorkspaceV1,bazelSlotIdsV1,nextBazelSlotStateV1,planBazelServersV1,planBazelSessionGarbageV1,rankBazelSlotsV1,renderBazelSessionStatusV1,withBazelSessionV1,type BazelSlotPortV1,type BazelSlotStateV1} from "../🟦️.ts";
import {bazelSlotPortV1,collectBazelSessionsV1,readBazelInventoryV1,removeBazelTreeV1,stopBazelServerV1} from "../🔌️port/🟦️.ts";

const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(componentSchema,"sessions.json");
const key=(index:number)=>index.toString(16).padStart(16,"0"),abort=()=>new AbortController().signal;

/** 🧪️ The port in memory: claims are a set, the gate is a first-in-first-out mutex, time is a counter the sleep advances, and a hook lets other sessions act while one waits. */
function memoryPort(onSleep:(count:number)=>void|Promise<void>=()=>undefined){
 const held=new Set<string>(),states=new Map<string,BazelSlotStateV1>(),lines:string[]=[],writes:string[]=[];let time=0,tail:Promise<void>=Promise.resolve(),sleeps=0,gates=0;
 const port:BazelSlotPortV1={
  async claim(slot){if(held.has(slot))return undefined;held.add(slot);return()=>{held.delete(slot);};},
  async gate(signal){signal.throwIfAborted();const previous=tail;let open!:()=>void;tail=new Promise<void>(done=>{open=done;});await previous;gates++;return()=>{gates--;open();};},
  async read(slot){return states.get(slot);},
  async write(slot,state){writes.push(slot);states.set(slot,state);},
  async sleep(milliseconds,signal){signal.throwIfAborted();time+=milliseconds;sleeps++;await onSleep(sleeps);signal.throwIfAborted();},
  now:()=>time,
  progress:line=>{lines.push(line);},
 };
 return {port,held,states,lines,writes,gates:()=>gates};
}

test("the closed fixture agrees with its closed schema and the component schema",()=>{
 const validate=ajv.compile(fixtureSchema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const name of ["Policy","Messages"])expect(typeof (componentSchema.definitions as Record<string,{const?:unknown}>)[name]?.const).toBe("object");
 expect(policy.names.slotPattern).toBe("^s[0-9]{1,2}$");expect(policy.servers.maximum).toBeLessThanOrEqual(99);
});

test("the server plan bounds Bazel servers by memory above the floor, cores and policy exactly like an independent Python implementation",()=>{
 for(const row of fixture.plans){const plan=planBazelServersV1(row.host,row.memory);expect(plan as unknown,row.name).toEqual(row.expected);expect(plan.slots,row.name).toBeGreaterThanOrEqual(policy.servers.minimum);expect(plan.slots).toBeLessThanOrEqual(policy.servers.maximum);expect(plan.heapMegabytes).toBeLessThanOrEqual(policy.servers.heapMegabytes);}
 expect(()=>planBazelServersV1({cores:0,memoryMegabytes:1},fixture.plans[0]!.memory)).toThrow("Closed Bazel server host required");
 expect(()=>planBazelServersV1(fixture.plans[0]!.host,{...fixture.plans[0]!.memory,budgetBasisPoints:1} as never)).toThrow("Closed Bazel server host required");
 expect(bazelSlotIdsV1({slots:3})).toEqual(["s0","s1","s2"]);
 const wide=fixture.plans.find(row=>row.expected.slots===policy.servers.maximum)!;expect(wide.expected.limit).toBe("policy");
});

test("the number of servers can be overridden explicitly and anything unreadable is refused",()=>{
 const base=planBazelServersV1(fixture.plans[0]!.host,fixture.plans[0]!.memory);
 for(const row of fixture.overrides){
  if(row.slots==="refused")expect(()=>bazelServerOverrideV1(base,row.environment),JSON.stringify(row.environment)).toThrow("SEMIO_BAZEL_SERVERS");
  else{const plan=bazelServerOverrideV1(base,row.environment);expect(plan.slots as unknown).toBe(row.slots??base.slots);expect(plan.cpuPerServer).toBeGreaterThanOrEqual(1);expect(plan.memoryPerServerMegabytes).toBeGreaterThanOrEqual(256);}
 }
 expect(bazelSessionResourcesV1(base)).toEqual([`--local_resources=cpu=${base.cpuPerServer}`,`--local_resources=memory=${base.memoryPerServerMegabytes}`]);
 expect(bazelSessionStartupV1(base,120)).toEqual(["--max_idle_secs=120",`--host_jvm_args=-Xmx${base.heapMegabytes}m`]);expect(()=>bazelSessionStartupV1(base,0)).toThrow("Positive");
 expect(bazelSessionCommandV1("/c/contents")).toEqual(["--repo_contents_cache=/c/contents"]);expect(bazelRepositoryContentsV1("/p/bazel/outputs")).toBe("/p/bazel/contents");
});

test("a nested session gets overflow slots so it never waits for the slot its parent holds",async()=>{
 const base=planBazelServersV1(fixture.plans[0]!.host,fixture.plans[0]!.memory),nested=bazelNestedServersV1(base,true);
 expect(bazelNestedServersV1(base,false)).toBe(base);expect(nested.slots).toBe(base.slots+policy.servers.nestedExtra);expect(Math.min(99,nested.slots)).toBe(nested.slots);
 const memory=memoryPort(),parents=await Promise.all(Array.from({length:base.slots},(_,index)=>acquireBazelSessionV1(memory.port,base,{key:key(index),signal:abort()})));
 const child=await acquireBazelSessionV1(memory.port,nested,{key:key(99),signal:abort(),timeoutMilliseconds:0});expect(parents.some(row=>row.slot===child.slot)).toBe(false);expect(Number(child.slot.slice(1))).toBeGreaterThanOrEqual(base.slots);
 await child.release();for(const parent of parents)await parent.release();
});

test("the slot order follows warm, kept, unused, then least recently used and agrees with an SQL ORDER BY",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of fixture.ranks){
   const states=row.states as Record<string,BazelSlotStateV1|undefined>,order=rankBazelSlotsV1(row.slots,states,row.key);expect(order as unknown,row.name).toEqual(row.expected);
   database.exec("DROP TABLE IF EXISTS slots;CREATE TABLE slots(slot TEXT,idx INTEGER,tier INTEGER,used INTEGER)");
   for(const slot of row.slots){const state=states[slot],tier=state===undefined?2:state.key===row.key?0:state.keys.includes(row.key)?1:3;database.query("INSERT INTO slots VALUES(?1,?2,?3,?4)").run(slot,Number(slot.slice(1)),tier,state?.usedAt??0);}
   const sql=database.query("SELECT slot FROM slots ORDER BY tier, CASE WHEN tier IN (0,1) THEN -used WHEN tier=3 THEN used ELSE 0 END, idx").all() as {slot:string}[];
   expect(sql.map(entry=>entry.slot),row.name+" (sqlite)").toEqual(order as string[]);
  }
 }finally{database.close();}
});

test("package paths are short, slot-scoped and refuse every shape outside the authored patterns",()=>{
 for(const row of fixture.workspaces.accepted)expect(bazelSessionWorkspaceV1(fixture.workspaces.root,row.slot,row.key)).toBe(row.path);
 for(const row of fixture.workspaces.refused)expect(()=>bazelSessionWorkspaceV1(fixture.workspaces.root,row.slot,row.key),JSON.stringify(row)).toThrow("Bazel session");
 expect(()=>bazelSessionWorkspaceV1("/"+"x".repeat(250),"s0",key(1))).toThrow("256");
 expect(bazelSessionWorkspaceV1("/p",  "s0",key(1))).not.toBe(bazelSessionWorkspaceV1("/p","s1",key(1)));
});

test("messages render in English and German exactly like str.format and the language follows the environment",()=>{
 for(const row of fixture.messages){expect(bazelSessionMessageV1(row.id,row.values as Record<string,string|number>,"en")).toBe(row.en);expect(bazelSessionMessageV1(row.id,row.values as Record<string,string|number>,"de")).toBe(row.de);}
 expect(()=>bazelSessionMessageV1("nope",{},"en")).toThrow("Unknown");expect(()=>bazelSessionMessageV1("waiting",{slots:1},"en")).toThrow("Missing");
 for(const row of fixture.languages)expect(bazelSessionLanguageV1(row.environment) as string,JSON.stringify(row.environment)).toBe(row.expected);
 expect(Object.keys(componentSchema.definitions.Messages.const.en).sort()).toEqual(Object.keys(componentSchema.definitions.Messages.const.de).sort());
});

test("three sessions claim three slots at once, a fourth waits behind the fair gate with progress and takes the slot a session frees",async()=>{
 const memory=memoryPort(async count=>{if(count===2)await first.release();}),plan={slots:3};
 const first=await acquireBazelSessionV1(memory.port,plan,{key:key(1),signal:abort()}),second=await acquireBazelSessionV1(memory.port,plan,{key:key(2),signal:abort()}),third=await acquireBazelSessionV1(memory.port,plan,{key:key(3),signal:abort()});
 expect([first.slot,second.slot,third.slot]).toEqual(["s0","s1","s2"]);expect(new Set([first.mode,second.mode,third.mode])).toEqual(new Set(["fresh"]));expect(memory.gates()).toBe(0);
 const fourth=await acquireBazelSessionV1(memory.port,plan,{key:key(4),signal:abort()});
 expect(fourth.slot).toBe("s0");expect(fourth.mode).toBe("switch");expect(fourth.previous?.key).toBe(key(1));
 expect(memory.lines.some(line=>line==="[bazel] all 3 Bazel server slots are in use; waiting for one (0 s)")).toBe(true);expect(memory.gates()).toBe(0);
 await second.release();await third.release();await fourth.release();expect(memory.held.size).toBe(0);
});

test("a selection returns to the slot of its warm server and the same selection twice at once lands in two slots",async()=>{
 const memory=memoryPort(),plan={slots:3};
 const a=await acquireBazelSessionV1(memory.port,plan,{key:key(1),signal:abort()}),b=await acquireBazelSessionV1(memory.port,plan,{key:key(2),signal:abort()});await a.release();await b.release();
 const again=await acquireBazelSessionV1(memory.port,plan,{key:key(2),signal:abort()}),twice=await acquireBazelSessionV1(memory.port,plan,{key:key(2),signal:abort()});
 expect(again.slot).toBe("s1");expect(again.mode).toBe("warm");expect(twice.slot).not.toBe(again.slot);expect(twice.mode).toBe(twice.slot==="s0"?"switch":"fresh");
 expect(bazelSessionWorkspaceV1("/w",again.slot,key(2))).not.toBe(bazelSessionWorkspaceV1("/w",twice.slot,key(2)));
 await again.release();await twice.release();
 const kept=memory.states.get("s0")!;expect(kept.keys.length).toBeLessThanOrEqual(policy.retention.packagesPerSlot);expect(kept.keys[0]).toBe(kept.key);
});

test("waiting honours the timeout and the cancellation, and a failed or cancelled session never leaks a slot or the gate",async()=>{
 const full=memoryPort(),plan={slots:2},sessions=[await acquireBazelSessionV1(full.port,plan,{key:key(1),signal:abort()}),await acquireBazelSessionV1(full.port,plan,{key:key(2),signal:abort()})];
 await expect(acquireBazelSessionV1(full.port,plan,{key:key(3),signal:abort(),timeoutMilliseconds:1000})).rejects.toThrow("timed out");expect(full.gates()).toBe(0);
 const controller=new AbortController(),waiting=acquireBazelSessionV1(full.port,plan,{key:key(3),signal:controller.signal});controller.abort(Error("cancelled by the caller"));await expect(waiting).rejects.toThrow("cancelled by the caller");expect(full.gates()).toBe(0);expect(full.held.size).toBe(2);
 for(const session of sessions)await session.release();expect(full.held.size).toBe(0);
 const failing=memoryPort();await expect(withBazelSessionV1(failing.port,plan,{key:key(1),signal:abort()},async()=>{throw Error("body failed");})).rejects.toThrow("body failed");expect(failing.held.size).toBe(0);expect(failing.writes).toEqual(["s0"]);
 const broken=memoryPort(),brokenPort:BazelSlotPortV1={...broken.port,async write(){throw Error("disk full");}};
 await expect(withBazelSessionV1(brokenPort,plan,{key:key(1),signal:abort()},async()=>1)).rejects.toThrow("disk full");expect(broken.held.size).toBe(0);
 await expect(withBazelSessionV1(brokenPort,plan,{key:key(1),signal:abort()},async()=>{throw Error("body first");})).rejects.toThrow("body first");expect(broken.held.size).toBe(0);
 const session=await acquireBazelSessionV1(memoryPort().port,plan,{key:key(1),signal:abort()});await session.release();await session.release();
});

test("progress lines are German when the language says so and the status table lists held and free slots",async()=>{
 const memory=memoryPort(),plan={slots:2},held=await acquireBazelSessionV1(memory.port,plan,{key:key(7),signal:abort(),language:"de"});
 expect(memory.lines.at(-1)).toBe("[bazel] Sitzungs-Slot s0: frischer Server");
 const rows=await bazelSessionStatusV1(memory.port,plan,abort());expect(rows.map(row=>row.held)).toEqual([true,false]);
 expect(renderBazelSessionStatusV1(rows,"en")).toContain("in use");expect(renderBazelSessionStatusV1(rows,"de")).toContain("belegt");expect(renderBazelSessionStatusV1(rows,"de")).toContain("frei");
 expect(memory.held.has("s0")).toBe(true);expect(memory.held.has("s1")).toBe(false);await held.release();
 expect(nextBazelSlotStateV1("s3",undefined,key(1),5).keys).toEqual([key(1)]);
});

test("garbage collection decisions agree with an independent Python implementation",()=>{
 for(const row of fixture.garbage)expect(planBazelSessionGarbageV1(row.inventory as never,{slots:row.slots},row.now,new Set(row.held)) as unknown,row.name).toEqual(row.expected);
 expect(()=>planBazelSessionGarbageV1({slots:[{slot:"x",outputBase:true,packages:[]}],foreign:[]} as never,{slots:1},0)).toThrow("Closed Bazel session inventory required");
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

/** 🧪️ Starts a child process that claims a slot through the production port and holds it until its input closes. */
function holder(directories:{leases:string;server:string;workspaces:string},slot:string):Promise<{kill:()=>void;exited:Promise<void>}>{
 const code=`import {bazelSlotPortV1} from ${JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim(${JSON.stringify(slot)},new AbortController().signal);console.log(claim?"held":"busy");if(claim)await new Promise(done=>process.stdin.on("close",done).resume());`;
 const script=join(directories.server,"..","holder.ts");mkdirSync(join(directories.server,".."),{recursive:true});writeFileSync(script,code);
 const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]}),exited=new Promise<void>(done=>child.on("exit",()=>done()));
 return new Promise((resolve,reject)=>{child.stdout.setEncoding("utf8");child.stdout.on("data",text=>{if(text.includes("held"))resolve({kill:()=>child.kill("SIGKILL"),exited});else reject(Error("child did not claim "+text));});child.on("error",reject);});
}

test("the production port claims slots through real repository leases across processes, frees a killed holder's slot and keeps states",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")};
 try{
  const port=bazelSlotPortV1(directories),signal=abort(),child=await holder(directories,"s0");
  expect(await port.claim("s0",signal)).toBeUndefined();const other=await port.claim("s1",signal);expect(other).toBeDefined();other!();
  const rows=await bazelSessionStatusV1(port,{slots:2},signal);expect(rows.map(row=>row.held)).toEqual([true,false]);
  child.kill();await child.exited;
  let reclaimed:(()=>void)|undefined;for(let attempt=0;attempt<50&&reclaimed===undefined;attempt++){reclaimed=await port.claim("s0",signal);if(reclaimed===undefined)await new Promise(done=>setTimeout(done,100));}
  expect(reclaimed,"a killed holder frees its slot").toBeDefined();reclaimed!();
  const release=await port.gate(signal);release();
  const state=nextBazelSlotStateV1("s0",undefined,key(1),Date.now());await port.write("s0",state);expect(existsSync(join(directories.server,"s0.session.json")),"nothing is written without an output base").toBe(false);mkdirSync(join(directories.server,"s0"),{recursive:true});mkdirSync(join(directories.server,"s1"),{recursive:true});await port.write("s0",state);expect(await port.read("s0")).toEqual(state);expect(await port.read("s1")).toBeUndefined();
  writeFileSync(join(directories.server,"s1.session.json"),"{");expect(await port.read("s1")).toBeUndefined();writeFileSync(join(directories.server,"s1.session.json"),JSON.stringify({...state,slot:"s0"}));expect(await port.read("s1"),"a state of another slot is ignored").toBeUndefined();
  expect(()=>port.read("../x")).toThrow("Bazel session slot required");
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("two sessions through the production port end in two slots and a third waits until one releases",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},lines:string[]=[];
 try{
  const port=bazelSlotPortV1(directories,{write:line=>lines.push(line)}),plan={slots:2},signal=abort();
  const one=await acquireBazelSessionV1(port,plan,{key:key(1),signal}),two=await acquireBazelSessionV1(port,plan,{key:key(2),signal});expect(new Set([one.slot,two.slot]).size).toBe(2);
  let third:string|undefined;const waiting=acquireBazelSessionV1(port,plan,{key:key(3),signal,timeoutMilliseconds:20000}).then(async session=>{third=session.slot;await session.release();});
  await new Promise(done=>setTimeout(done,700));expect(third).toBeUndefined();await one.release();await waiting;expect(third).toBe(one.slot);await two.release();
  expect(lines.some(line=>line.includes("waiting for one"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("the inventory reads slots, packages and foreign entries and the collector removes what the plan names, stops servers first and skips held slots",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"bazel-slots-")),directories={leases:join(root,"leases"),server:join(root,"p/server"),workspaces:join(root,"p/workspaces")},day=86400000;
 try{
  const port=bazelSlotPortV1(directories),stopped:string[]=[],layout=(path:string,files:string[])=>{for(const file of files){mkdirSync(join(path,file,".."),{recursive:true});writeFileSync(join(path,file),"x");}};
  layout(directories.server,["s0/execroot/x","s2/execroot/x","s0.events.json","metrics.json","0123456789abcdef/execroot/x","0123456789abcdef.events.json"]);
  await port.write("s2",nextBazelSlotStateV1("s2",undefined,key(5),Date.now()));await port.write("s0",nextBazelSlotStateV1("s0",undefined,key(1),Date.now()));
  for(const name of [key(1),key(2),key(3)]){layout(join(directories.workspaces,"s0"),[name+"/MODULE.bazel"]);}
  layout(join(directories.workspaces),["0123456789abcdef/MODULE.bazel","s2/"+key(5)+"/MODULE.bazel"]);
  const old=(Date.now()-20*day)/1000;utimesSync(join(directories.workspaces,"s0",key(2)),old,old);utimesSync(join(directories.workspaces,"s0",key(3)),old,old);utimesSync(join(directories.workspaces,"s0",key(1)),Date.now()/1000,Date.now()/1000);
  mkdirSync(join(directories.server,"s2/execroot/locked"),{recursive:true});writeFileSync(join(directories.server,"s2/execroot/locked/ro"),"x");if(process.platform!=="win32"){chmodSync(join(directories.server,"s2/execroot/locked/ro"),0o444);chmodSync(join(directories.server,"s2/execroot/locked"),0o555);}
  const inventory=await readBazelInventoryV1(directories,port);
  expect(inventory.slots.map(row=>row.slot)).toEqual(["s0","s2"]);expect(inventory.slots[0]!.packages.length).toBe(3);expect(inventory.foreign).toEqual(["server/0123456789abcdef","workspaces/0123456789abcdef"]);
  const signal=abort(),dry=await collectBazelSessionsV1(directories,port,{slots:2},signal,async()=>undefined,{dryRun:true});expect(dry.outputBases).toEqual([{slot:"s2",reason:"capacity"}]);expect(existsSync(join(directories.server,"s2"))).toBe(true);
  const heldZero=await port.claim("s0",signal),heldTwo=await port.claim("s2",signal),skipped=await collectBazelSessionsV1(directories,port,{slots:2},signal,async base=>{stopped.push(base);});
  expect(skipped.outputBases).toEqual([]);expect(skipped.packages).toEqual([]);expect(skipped.foreign).toEqual(["server/0123456789abcdef","workspaces/0123456789abcdef"]);expect(stopped).toEqual([join(directories.server,"0123456789abcdef")]);expect(existsSync(join(directories.server,"s2"))).toBe(true);expect(readdirSync(join(directories.workspaces,"s0")).length).toBe(3);heldZero!();heldTwo!();
  stopped.length=0;const removed=await collectBazelSessionsV1(directories,port,{slots:2},signal,async base=>{stopped.push(base.slice(directories.server.length+1));expect(existsSync(base)).toBe(true);});
  expect(removed.outputBases).toEqual([{slot:"s2",reason:"capacity"}]);expect(removed.packages.map(row=>row.slot+row.key)).toEqual(["s0"+key(2),"s0"+key(3),"s2"+key(5)]);expect(removed.foreign).toEqual([]);expect(stopped).toEqual(["s2"]);
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
 expect(script).toContain("withBazelSlotV1(root,bazelSelectionKeyV1(graph,request),phaseControl,hold=>withBazelStorageV1(");expect(script).toContain("withBazelSlotV1(root,key,phaseControl,hold=>withBazelStorageV1(");
 expect(script).not.toContain('"bazel-workspace-"');expect(script).not.toContain('"bazel-snapshot-restore"');expect(script).not.toContain("-Xmx2048m");expect(script).toContain("bazelRestoreResourcesV1(");
 expect(script).toContain("bazelSessionWorkspaceV1(storage.paths.workspace,slot,namespace)");expect(script).not.toContain("join(storage.paths.workspace,namespace)");expect(storage).not.toContain("createHash");
 expect(script).toContain("session.slot");expect(policy.leases.restorePrefix.startsWith("bazel-restore")).toBe(true);
});
