import {test,expect,spyOn} from "bun:test";
import {mkdtempSync,mkdirSync,readdirSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {acquireResourceLease,acquireQueuedResourceLease,upgradeQueuedResourceLease,type ResourceLease,type ResourceLeaseControl} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import policySchema from "../../⏳️wait/🧬️schema/📜️policy.json";
import progressSchema from "../../../🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1} from "../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";

/** 🏁️ Separate OS processes prove that queued writers are served within a bound, dead requesters are reaped by their successors, waiting is cheap, upgrades cannot deadlock and queue waits name their head request; Python SQLite readers are the independent third-party oracle for the unqueued starvation controls. */
const implementation=fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),worker=fileURLToPath(new URL("./📥️worker/🟦️.ts",import.meta.url)),bounds=fixture.bounds;
const python=`import sqlite3,sys,time
db,hold,until=sys.argv[1],float(sys.argv[2]),float(sys.argv[3])
end=time.time()+until
c=sqlite3.connect(db,timeout=0,isolation_level=None)
n=0
while time.time()<end:
    c.execute("BEGIN")
    c.execute("SELECT resource FROM semio_resource_lease").fetchone()
    time.sleep(hold)
    c.execute("ROLLBACK")
    n+=1
print('{"role":"reader","runtime":"python","done":true,"acquisitions":%d}'%n,flush=True)
`;
type Child={process:Bun.Subprocess<"ignore","pipe","pipe">;lines:Record<string,unknown>[];done:Promise<void>};
const children=new Set<Child>();
function root():string{const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit ticket generated artifact directory required");const parent=resolve(output);mkdirSync(parent,{recursive:true});return mkdtempSync(join(parent,"lf-"));}
function spawnChild(command:string[]):Child{
  const process=Bun.spawn(command,{stdin:"ignore",stdout:"pipe",stderr:"pipe"}),child:Child={process,lines:[],done:undefined as unknown as Promise<void>};
  child.done=(async()=>{let text="";const take=()=>{let at=text.indexOf("\n");while(at>=0){const line=text.slice(0,at);text=text.slice(at+1);if(line.startsWith("{"))child.lines.push(JSON.parse(line));at=text.indexOf("\n");}};for await(const chunk of process.stdout){text+=Buffer.from(chunk).toString();take();}await process.exited;text+="\n";take();})();
  children.add(child);return child;
}
function participant(runtime:"bun"|"node",config:Record<string,unknown>):Child{
  return spawnChild([runtime==="bun"?globalThis.process.execPath:"node",worker,JSON.stringify({implementation,holdMilliseconds:0,untilMilliseconds:0,offsetMilliseconds:0,owner:"p"+Math.random().toString(36).slice(2,10),mode:"shared",api:"queued",...config})]);
}
async function until(check:()=>boolean,milliseconds:number,what:string):Promise<void>{const end=Date.now()+milliseconds;while(!check()){if(Date.now()>end)throw Error("Timed out waiting for "+what);await Bun.sleep(20);}}
const queueOf=(directory:string,resource:string)=>join(directory,createHash("sha256").update(resource).digest("hex")+".queue");
const tickets=(directory:string,resource:string)=>{try{return readdirSync(queueOf(directory,resource));}catch{return[];}};
async function cleanup(directory:string):Promise<void>{
  for(const child of children){try{child.process.kill("SIGKILL");}catch{}}
  await Promise.allSettled([...children].map(child=>child.done));children.clear();rmSync(directory,{recursive:true,force:true});
}
const none=new AbortController().signal;

test("fairness fixtures and wait policy are closed schema-first data",()=>{
  expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
  expect(new Ajv({strict:true}).validate(schema,{...fixture,extra:true})).toBe(false);
  const ajv=new Ajv({strict:true}),validate=ajv.compile({$schema:policySchema.$schema,definitions:policySchema.definitions,$ref:"#/definitions/WaitPolicy"}),policy=policySchema.definitions.Policy.const;
  expect(validate(policy)).toBe(true);expect(validate({...policy,sweepIntervalMilliseconds:0})).toBe(false);
  expect(policy.pollInitialMilliseconds).toBeLessThanOrEqual(policy.pollMaximumMilliseconds);
  expect(policy.pollGrowthNumerator).toBeGreaterThanOrEqual(policy.pollGrowthDenominator);
  expect(progressSchema.const.queueWait.resourcePrefix).toBe("lease-queue:");
});

for(const row of fixture.starvation)test(`writer ${row.outcome} against ${row.readers} ${row.readerRuntime} readers: ${row.id}`,async()=>{
  const directory=root(),resource="fair-"+row.id;
  try{
    (await acquireResourceLease({directory,resource,mode:"exclusive",signal:none})).release();
    const database=join(directory,createHash("sha256").update(resource).digest("hex")+".sqlite"),readers:Child[]=[];
    for(let index=0;index<bounds.readers;index++){
      const offset=Math.floor(index*bounds.holdMilliseconds/bounds.readers);
      readers.push(row.readers==="python"
        ?spawnChild(["python3","-u","-c",python,database,String(bounds.holdMilliseconds/1000),String(bounds.observationMilliseconds/1000+offset/1000)])
        :participant(row.readerRuntime as "bun"|"node",{role:"reader",api:"queued",directory,resource,holdMilliseconds:bounds.holdMilliseconds,untilMilliseconds:bounds.observationMilliseconds,offsetMilliseconds:offset}));
    }
    const writer=participant(row.writerRuntime as "bun"|"node",{role:"writer",api:row.writer,directory,resource,offsetMilliseconds:bounds.writerStartOffsetMilliseconds,timeoutMilliseconds:row.outcome==="bounded"?bounds.boundedWriterMaximumMilliseconds:bounds.observationMilliseconds-bounds.writerStartOffsetMilliseconds-1500});
    await writer.done;
    const result=writer.lines.find(line=>line.role==="writer")!;
    if(row.outcome==="bounded"){expect(result.error,JSON.stringify(result)).toBeUndefined();expect(result.acquiredAfterMs as number).toBeLessThanOrEqual(bounds.boundedWriterMaximumMilliseconds);}
    else{expect(String(result.error)).toMatch(/Resource lease timed out/);}
    await Promise.all(readers.map(reader=>reader.done));
    const finished=readers.flatMap(reader=>reader.lines).filter(line=>line.done===true);
    expect(finished.length).toBe(bounds.readers);
    for(const line of finished)expect(line.acquisitions as number).toBeGreaterThan(2);
  }finally{await cleanup(directory);}
},120000);

test("dead predecessors are reaped by their successors without sweeping every ticket",async()=>{
  const row=fixture.reaping[0]!,directory=root(),resource="fair-reap";
  try{
    const holder=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"holder",signal:none,onWait:()=>{}});
    const dead:Child[]=[];
    for(let index=0;index<row.dead;index++){
      dead.push(participant(row.runtimes[index%row.runtimes.length] as "bun"|"node",{role:"queued",api:"queued",directory,resource,holdMilliseconds:100}));
      await until(()=>tickets(directory,resource).length===index+2,bounds.reapMaximumMilliseconds,"dead requester ticket "+index);
    }
    const survivor=participant("bun",{role:"queued",api:"queued",directory,resource,holdMilliseconds:50});
    await until(()=>tickets(directory,resource).length===row.dead+2,bounds.reapMaximumMilliseconds,"survivor ticket");
    for(const child of dead)child.process.kill("SIGKILL");
    await Promise.all(dead.map(child=>child.process.exited));
    await until(()=>tickets(directory,resource).length===2,bounds.reapMaximumMilliseconds,"dead tickets reaped while the holder still holds");
    expect(survivor.lines).toEqual([]);
    holder.release();
    await survivor.done;
    const acquired=survivor.lines.find(line=>line.acquiredAfterMs!==undefined);
    expect(acquired,JSON.stringify(survivor.lines)).toBeDefined();
    expect(tickets(directory,resource)).toEqual([]);
  }finally{await cleanup(directory);}
},120000);

test("waiting behind a long holder costs a bounded fraction of one core per waiter",async()=>{
  const directory=root(),resource="fair-cost";
  try{
    const holder=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"holder",signal:none,onWait:()=>{}});
    const waiters=Array.from({length:bounds.waiters},(_,index)=>participant(index%2?"node":"bun",{role:"waiter",api:"queued",directory,resource,timeoutMilliseconds:bounds.waiterObservationMilliseconds,offsetMilliseconds:index*20}));
    await Promise.all(waiters.map(waiter=>waiter.done));
    holder.release();
    for(const waiter of waiters){const line=waiter.lines.find(row=>row.role==="waiter")!;expect(line.outcome,JSON.stringify(line)).toBe("timed-out");expect(line.cpuMs as number).toBeLessThanOrEqual(bounds.waiterObservationMilliseconds*bounds.waiterCpuFractionMaximum);}
    await until(()=>tickets(directory,resource).length===0,bounds.reapMaximumMilliseconds,"timed-out waiter tickets removed");
  }finally{await cleanup(directory);}
},120000);

test("a queued wait names the request it stands behind through the structured progress channel",async()=>{
  const directory=root(),resource="fair-observe",expected=fixture.observability,controller=new AbortController(),writes:string[]=[];
  const spy=spyOn(globalThis.process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof globalThis.process.stderr.write);
  try{
    await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fairness-observe",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
      const holder=await acquireQueuedResourceLease({directory,resource,mode:expected.holderMode as "exclusive",owner:"holder",signal:none,onWait:()=>{}});
      try{
        const control=createRequestedIssuedResourceControlV1(createIssuedResourceControlV1(invocation,"lease:fairness-observe",0),0,controller.signal);
        const waiting=acquireQueuedResourceLease({directory,resource,mode:"shared",owner:"observer",signal:control.signal,control,sharedCohort:true}).then(()=>"acquired",error=>String(error));
        await until(()=>writes.some(line=>line.includes("lease-queue:")),bounds.reapMaximumMilliseconds,"queue wait progress");
        controller.abort(Error("observed"));
        expect(await waiting).toMatch(/observed/);
      }finally{holder.release();}
    });
  }finally{spy.mockRestore();controller.abort();await cleanup(directory);}
  const structured=writes.filter(line=>line.startsWith("[script] {")).map(line=>JSON.parse(line.slice("[script] ".length))).filter(event=>event.activity?.resource?.startsWith("lease-queue:"));
  expect(structured.length).toBeGreaterThanOrEqual(1);
  const event=structured[0]!,payload=JSON.parse(event.activity.resource.slice("lease-queue:".length));
  expect(event.activity.elapsedMs).toBe(0);expect(event.activity.mode).toBe("shared");
  expect(payload).toEqual({version:1,position:expected.expectedPosition,queued:expected.expectedQueued,holderMode:expected.holderMode,holderPid:process.pid,holderSeconds:payload.holderSeconds,resource});
  expect(payload.holderSeconds).toBeGreaterThanOrEqual(0);
  expect(writes.some(line=>line.startsWith("[script] ")&&!line.startsWith("[script] {")&&line.includes("Waiting for access to "+resource+" (shared): position 2 of 2, behind the request of process "+process.pid))).toBe(true);
  expect(writes.some(line=>line.includes('"resource":"'+resource+'"'))).toBe(true);
},60000);

const held=async(directory:string,resource:string,owner:string,sharedCohort=true)=>acquireQueuedResourceLease({directory,resource,mode:"shared",owner,signal:none,sharedCohort,onWait:()=>{}});

test("an upgrade keeps its arrival position and later exclusive requesters wait behind it",async()=>{
  const directory=root(),resource="fair-upgrade-position",order:string[]=[];
  try{
    const mine=await held(directory,resource,"mine");
    const reader=participant("bun",{role:"holder",api:"queued",mode:"shared",directory,resource,holdMilliseconds:bounds.upgradeReaderHoldMilliseconds});
    await until(()=>reader.lines.some(line=>line.held===true),bounds.upgradeMaximumMilliseconds,"reader hold");
    const upgrading=upgradeQueuedResourceLease(mine,{timeoutMs:bounds.upgradeMaximumMilliseconds}).then(lease=>{order.push("upgrader");return lease;});
    await until(()=>tickets(directory,resource).length===2,bounds.upgradeMaximumMilliseconds,"upgrade ticket");
    const later=participant("node",{role:"queued",api:"queued",directory,resource,holdMilliseconds:50});
    await until(()=>tickets(directory,resource).length===3,bounds.upgradeMaximumMilliseconds,"later exclusive ticket");
    expect(order).toEqual([]);
    const exclusive=await upgrading;expect(exclusive.mode).toBe("exclusive");
    expect(later.lines.some(line=>line.acquiredAfterMs!==undefined)).toBe(false);
    mine.release();
    exclusive.release();
    await later.done;
    expect(later.lines.some(line=>line.acquiredAfterMs!==undefined),JSON.stringify(later.lines)).toBe(true);
    expect(tickets(directory,resource)).toEqual([]);
    await expect(upgradeQueuedResourceLease(mine)).rejects.toThrow(/Live queued shared lease required/);
  }finally{await cleanup(directory);}
},120000);

test("concurrent upgraders that each hold shared access never deadlock and never overlap",async()=>{
  const directory=root(),resource="fair-upgrade-concurrent";
  try{
    const first=await held(directory,resource,"first"),second=await held(directory,resource,"second");
    let active=0,overlap=0;const order:string[]=[];
    const run=async(name:string,lease:ResourceLease)=>{const exclusive=await upgradeQueuedResourceLease(lease,{timeoutMs:bounds.upgradeMaximumMilliseconds});active++;overlap=Math.max(overlap,active);order.push(name);await Bun.sleep(80);active--;exclusive.release();};
    await Promise.all([run("first",first),run("second",second)]);
    expect(overlap).toBe(1);expect(order).toEqual(["first","second"]);
    expect(tickets(directory,resource)).toEqual([]);
    const naive=await held(directory,resource,"naive");
    const blocked=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"naive-writer",signal:none,timeoutMs:300,onWait:()=>{}}).then(()=>"acquired",error=>String(error));
    expect(blocked).toMatch(/Resource lease timed out/);
    naive.release();
  }finally{await cleanup(directory);}
},120000);

test("a failed upgrade consumes the shared hold and leaves nothing behind",async()=>{
  const directory=root(),resource="fair-upgrade-failure";
  try{
    const mine=await held(directory,resource,"mine");
    const reader=participant("bun",{role:"holder",api:"queued",mode:"shared",directory,resource,holdMilliseconds:bounds.upgradeReaderHoldMilliseconds});
    await until(()=>reader.lines.some(line=>line.held===true),bounds.upgradeMaximumMilliseconds,"reader hold");
    await expect(upgradeQueuedResourceLease(mine,{timeoutMs:300})).rejects.toThrow(/Resource lease timed out/);
    expect(tickets(directory,resource).length).toBe(1);
    mine.release();
    await reader.done;
    const next=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"next",signal:none,timeoutMs:bounds.upgradeMaximumMilliseconds,onWait:()=>{}});
    next.release();
    expect(tickets(directory,resource)).toEqual([]);
  }finally{await cleanup(directory);}
},120000);
