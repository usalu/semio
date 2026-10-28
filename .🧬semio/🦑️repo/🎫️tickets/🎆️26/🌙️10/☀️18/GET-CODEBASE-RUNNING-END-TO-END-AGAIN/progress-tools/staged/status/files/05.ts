/** 📈️ Tests for leases standing. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import statusSchema from "../../📋️status/🧬️schema/🔣️.json";
import {buildLeaseReportV1,formatLeaseReportV1} from "../../📋️status/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {leaseElapsedSecondsV1,observeLeaseHolderV1,leaseTerminateDecisionV1,leaseYieldAcceptV1,leaseYieldDecisionV1,type LeaseTerminateInputV1,type LeaseYieldAcceptInputV1} from "../🤝️yield/🟦️.ts";
import {leaseActivityV1,leaseBoundComponentsV1,leaseMovedBoundV1,leaseEtaV1,probeLeaseActivityV1,setLeaseActivityPortsV1,leaseHolderStateV1,leaseOvertakeWindowV1,leasePriorityClassV1,leaseRealArrivalV1,leaseStandingTextV1,leaseTicketArrivalV1,noteLeaseProgressV1,leaseStandingSnapshotV1,observeLeaseStandingV1,parseLeaseProgressV1,registerLeaseImplementationProviderV1,leaseImplementationV1,simulateLeaseQueueV1,type LeaseHolderStateInputV1,type LeasePriorityV1} from "../🟦️.ts";

/** 🧪️ Contract: the standing laws (priority classes with aging, progress parsing, ETA, holder state) hold for the declared corpus, tonight's queue traces and a Python model that reads only the schema and the corpus. */
const model=fileURLToPath(new URL("./🐍️model.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),fixturePath=fileURLToPath(new URL("./🧫️fixtures/🔣️.json",import.meta.url));
import {createScriptProcessActivitySamplerV1} from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {leaseRegistryFolderV1} from "../../🧭️order/🗂️registry/🟦️.ts";
const oracle=(()=>{const run=Bun.spawnSync(["python3","-I",model,schemaPath,fixturePath]);if(run.exitCode!==0)throw Error(run.stderr.toString());return JSON.parse(run.stdout.toString()) as {yields:unknown[];terminates:unknown[];progress:unknown[];priority:string[];simulations:Record<string,Record<"fifo"|"priority",{id:string;start:number;wait:number}[]>>;states:{state:string;staleIds?:string[]}[];eta:unknown[];activity:unknown[]};})();
const policy=schema.definitions.Policy.const;

test("the policy and the corpus satisfy their closed schemas and refuse drift",()=>{
 const ajv=new Ajv({strict:false}),policyValid=ajv.compile({...schema.definitions.PolicySchema,definitions:schema.definitions}),corpusValid=ajv.compile({...schema.definitions.Corpus,definitions:schema.definitions});
 expect(policyValid(policy)).toBe(true);expect(policyValid({...policy,extra:1})).toBe(false);expect(policyValid({...policy,reclaim:{...policy.reclaim,mode:"kill"}})).toBe(false);
 expect(corpusValid(fixture)).toBe(true);expect(corpusValid({...fixture,etaCases:[]})).toBe(false);expect(corpusValid({...fixture,priorityCases:[{...fixture.priorityCases[0],expected:"urgent"}]})).toBe(false);
 const standing=ajv.compile({...schema.definitions.Standing,definitions:schema.definitions});
 expect(standing(fixture.stateCases[1]!.standing)).toBe(true);expect(standing({...fixture.stateCases[1]!.standing,priority:"urgent"})).toBe(false);
});

test("messages exist in English and German with the same placeholders",()=>{
 for(const[key,row]of Object.entries(schema.definitions.Messages.const)){
  const names=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]).sort().join(",");
  expect(row.en.length>0&&row.de.length>0).toBe(true);expect([key,names(row.de)]).toEqual([key,names(row.en)]);
 }
 expect(leaseStandingTextV1("de","eta",{minutes:5})).toBe("noch etwa 5 min");expect(leaseStandingTextV1("en","eta",{minutes:5})).toBe("about 5 min left");
});

test("priority classes resolve like the model",()=>{
 fixture.priorityCases.forEach((row,index)=>{
  const got=leasePriorityClassV1(row.environment,{agentSession:row.agentSession,interactiveTerminal:row.interactiveTerminal});
  expect([row.id,got]).toEqual([row.id,row.expected]);expect([row.id,oracle.priority[index]]).toEqual([row.id,row.expected]);
 });
});

test("progress lines parse like the model",()=>{
 fixture.progressCases.forEach((row,index)=>{
  expect([row.id,parseLeaseProgressV1(row.resource)??null]).toEqual([row.id,row.expected]);expect([row.id,oracle.progress[index]]).toEqual([row.id,row.expected]);
 });
});

test("yield requests are honoured only when the holder verifies the claim, and a holder is stopped only under every safety condition",()=>{
 fixture.yieldCases.forEach((row,index)=>{
  const got=leaseYieldAcceptV1(row as unknown as LeaseYieldAcceptInputV1),want=row.expected;
  expect([row.id,got.accept,got.refusal,got.ids]).toEqual([row.id,want.accept,(want as {refusal?:string}).refusal,(want as {ids?:string[]}).ids]);
  expect([row.id,oracle.yields[index]]).toEqual([row.id,want]);
 });
 fixture.terminateCases.forEach((row,index)=>{
  const got=leaseTerminateDecisionV1(row.input as unknown as LeaseTerminateInputV1),want=row.expected;
  expect([row.id,got.terminate,got.refusal]).toEqual([row.id,want.terminate,(want as {refusal?:string}).refusal]);
  expect([row.id,oracle.terminates[index]]).toEqual([row.id,want]);
 });
 expect([leaseYieldDecisionV1("stale","request",["a"]),leaseYieldDecisionV1("stale","request",[]),leaseYieldDecisionV1("stale","report",["a"]),leaseYieldDecisionV1("stuck","request",[]),leaseYieldDecisionV1("stuck","report",[]),leaseYieldDecisionV1("silent","terminate",["a"])]).toEqual(["stale",undefined,undefined,"stuck",undefined,undefined]);
});

test("holder states and ETAs match the corpus and the model",()=>{
 fixture.stateCases.forEach((row,index)=>{
  const got=leaseHolderStateV1(row as unknown as LeaseHolderStateInputV1);
  expect([row.id,got.state,got.staleIds]).toEqual([row.id,row.expected.state,(row.expected as {staleIds?:string[]}).staleIds]);expect([row.id,oracle.states[index]!.state,oracle.states[index]!.staleIds]).toEqual([row.id,row.expected.state,(row.expected as {staleIds?:string[]}).staleIds]);
 });
 fixture.etaCases.forEach((row,index)=>{
  const got=leaseEtaV1({elapsedMs:row.elapsedMs,phaseElapsedMs:row.phaseElapsedMs,...(row as {done?:number}).done===undefined?{}:{done:(row as {done:number}).done,total:(row as {total:number}).total},historyMs:row.historyMs})??null;
  expect([row.id,got] as unknown[]).toEqual([row.id,row.expected]);expect([row.id,oracle.eta[index]]).toEqual([row.id,row.expected as unknown]);
 });
});

const advance=(priority:LeasePriorityV1)=>policy.priority.classes.find(row=>row.id===priority)!.advanceMilliseconds;

test("the filed arrival orders tickets exactly like an SQL ORDER BY over the same rows",()=>{
 const database=new Database(":memory:");database.run("create table t(id text, class text, arrival integer, advance integer)");
 const insert=database.prepare("insert into t values(?,?,?,?)");
 for(const sim of fixture.simulations)for(const job of sim.jobs)insert.run(sim.id+"/"+job.id,job.class,job.arrival*1000+1700000000000,advance(job.class as LeasePriorityV1));
 const sql=(database.query("select id from t order by arrival-advance, id").all() as {id:string}[]).map(row=>row.id);
 const rows=fixture.simulations.flatMap(sim=>sim.jobs.map(job=>({id:sim.id+"/"+job.id,filed:leaseTicketArrivalV1(job.class as LeasePriorityV1,job.arrival*1000+1700000000000)})));
 expect(rows.sort((left,right)=>left.filed-right.filed||(left.id<right.id?-1:1)).map(row=>row.id)).toEqual(sql);
 for(const priority of ["interactive","agent","background"] as const){const real=1700000000000;expect(leaseRealArrivalV1(priority,leaseTicketArrivalV1(priority,real))).toBe(real);}
 expect(leaseTicketArrivalV1("interactive",5)).toBe(0);
});

for(const sim of fixture.simulations)test(`queue law ${sim.id}: TS equals the model, work is conserved, overtaking is bounded, equal classes stay in arrival order`,()=>{
 const jobs=sim.jobs.map(job=>({...job,class:job.class as LeasePriorityV1})),byId=new Map(jobs.map(job=>[job.id,job]));
 for(const mode of ["fifo","priority"] as const){
  const got=simulateLeaseQueueV1(jobs,mode==="fifo").map(row=>({id:row.id,start:row.start,wait:row.wait})),want=oracle.simulations[sim.id]![mode];
  expect(got).toEqual(want);
  const end=Math.max(...got.map(row=>row.start+byId.get(row.id)!.service));
  const busy=jobs.reduce((sum,job)=>sum+job.service,0);
  expect(end).toBeGreaterThanOrEqual(busy);
  if(mode==="fifo")expect(got.map(row=>row.id)).toEqual([...jobs].sort((left,right)=>left.arrival-right.arrival||(left.id<right.id?-1:1)).map(row=>row.id));
 }
 const priority=simulateLeaseQueueV1(jobs),fifo=simulateLeaseQueueV1(jobs,true);
 expect(Math.max(...priority.map(row=>row.start+byId.get(row.id)!.service))).toBe(Math.max(...fifo.map(row=>row.start+byId.get(row.id)!.service)));
 for(const later of priority)for(const earlier of priority){
  const a=byId.get(later.id)!,b=byId.get(earlier.id)!;
  if(a.arrival>b.arrival&&later.start<earlier.start)expect(a.arrival-b.arrival).toBeLessThan(leaseOvertakeWindowV1(a.class,b.class)/1000+1e-9);
 }
 for(const priorityClass of ["interactive","agent","background"] as const){
  const same=priority.filter(row=>row.class===priorityClass).sort((left,right)=>left.start-right.start);
  for(let index=1;index<same.length;index++)expect(byId.get(same[index]!.id)!.arrival).toBeGreaterThanOrEqual(byId.get(same[index-1]!.id)!.arrival);
 }
});

test("tonight's trace: interactive rebuilds wait far less, agents wait less, background waits at most the declared window longer",()=>{
 const sim=fixture.simulations.find(row=>row.id==="tonight-0-with-dev")!,jobs=sim.jobs.map(job=>({...job,class:job.class as LeasePriorityV1})),stat=(rows:readonly {class:LeasePriorityV1;wait:number}[],priorityClass:LeasePriorityV1)=>{const waits=rows.filter(row=>row.class===priorityClass).map(row=>row.wait);return{mean:waits.reduce((sum,value)=>sum+value,0)/waits.length,max:Math.max(...waits)};};
 const fifo=simulateLeaseQueueV1(jobs,true),priority=simulateLeaseQueueV1(jobs);
 expect(stat(priority,"interactive").mean).toBeLessThan(stat(fifo,"interactive").mean*0.5);
 expect(stat(priority,"agent").mean).toBeLessThan(stat(fifo,"agent").mean);
 expect(stat(priority,"background").max).toBeLessThanOrEqual(stat(fifo,"background").max+leaseOvertakeWindowV1("interactive","background")/1000);
 const stream=simulateLeaseQueueV1(fixture.simulations.find(row=>row.id==="background-under-interactive-stream")!.jobs.map(job=>({...job,class:job.class as LeasePriorityV1}))),background=stream.find(row=>row.id==="background")!;
 const latest=stream.filter(row=>row.class==="interactive"&&row.start<background.start).length;
 expect(latest).toBeLessThanOrEqual(Math.ceil(leaseOvertakeWindowV1("interactive","background")/1000/600));
});

test("progress notes publish a changed phase at once and everything else at most once per interval",()=>{
 let heard=0;observeLeaseStandingV1(()=>{heard++;});
 const base=Date.now();noteLeaseProgressV1({activity:{resource:"build:graph-phase:inference:1:566"}},base);
 expect(heard).toBe(1);noteLeaseProgressV1({activity:{resource:"build:graph-phase:inference:2:566"}},base+100);expect(heard).toBe(1);
 noteLeaseProgressV1({activity:{resource:"build:graph-phase:inference:9:566"}},base+policy.progress.publishIntervalMilliseconds+1);expect(heard).toBe(2);
 noteLeaseProgressV1({activity:{resource:"build:graph-phase:assembly:1:20"}},base+policy.progress.publishIntervalMilliseconds+2);expect(heard).toBe(3);
 noteLeaseProgressV1({activity:{resource:"lease-queue:{}"}},base+20000);
 const snapshot=leaseStandingSnapshotV1(false,base+30000,{});
 expect([snapshot.phase,snapshot.done,snapshot.total,snapshot.progressAtMs]).toEqual(["assembly",1,20,base+20000]);
 expect(snapshot.priority).toBe("agent");expect(leaseStandingSnapshotV1(false,base,{SEMIO_LEASE_PRIORITY:"background"}).priority).toBe("background");
});

test("the implementation identity follows the registered provider and binds resources by its table",()=>{
 expect(leaseImplementationV1().components).toEqual({});expect(leaseBoundComponentsV1("repository-graph-facts:/w:shard:7")).toEqual([]);
 registerLeaseImplementationProviderV1({components:()=>({"graph.generation":"aaaaaaaaaaaa"}),bound:resource=>resource.startsWith("repository-graph-facts:")?["graph.generation"]:[]});
 const first=leaseImplementationV1();registerLeaseImplementationProviderV1({components:()=>({"graph.generation":"bbbbbbbbbbbb"}),bound:()=>[]});
 expect([first.digest.length,leaseImplementationV1().digest===first.digest]).toEqual([16,false]);expect(leaseBoundComponentsV1("repository-graph-facts:/w:shard:7")).toEqual([]);
});

test("the status report shows holder age, progress, time left, implementation freshness and ancestry in English and German, and validates against its schema",()=>{
 registerLeaseImplementationProviderV1({components:()=>({}),bound:resource=>resource.startsWith("repository-graph-facts:")?["graph.generation"]:[]});
 const now=1800000000000,resource="repository-graph-facts:/w:shard:7",implementation=(loadedAtMs:number,generation:string)=>({digest:"0123456789abcdef",loadedAtMs,components:{"graph.generation":generation}});
 const standing=(loadedAtMs:number,generation:string,progressAtMs:number,extra:Record<string,unknown>={})=>({version:1 as const,priority:"agent" as const,implementation:implementation(loadedAtMs,generation),progressAtMs,...extra});
 const entry=(pid:number,holds:{resource:string;mode:"shared"|"exclusive";sinceMs:number;task:number}[],standingRow?:ReturnType<typeof standing>)=>({version:1 as const,pid,startedAtMs:now-9000000,updatedAtMs:now,command:"./script.ts repo run x",directory:"/w/lease",...standingRow?{standing:standingRow}:{},holds,waits:[]});
 const report=buildLeaseReportV1({directory:"/w/lease",atMs:now,entries:[
  entry(100,[{resource,mode:"exclusive",sinceMs:now-6000000,task:1}],standing(now-7000000,"aaaaaaaaaaaa",now-1000,{phase:"inference",done:100,total:400,phaseStartedAtMs:now-600000,parentPid:90,chain:[80,90]})),
  entry(200,[{resource:"artifact:/w/x",mode:"shared",sinceMs:now-1000,task:1}],standing(now-1000000,"bbbbbbbbbbbb",now-1000)),
  entry(300,[{resource:"build-graph-state:/w:selected:7",mode:"exclusive",sinceMs:now-7000000,task:1}],standing(now-1000000,"bbbbbbbbbbbb",now-6000000)),
 ],queues:[{hash:"a".repeat(64),position:1,queued:2,arrivedAtMs:now-5000000,pid:400,mode:"exclusive"},{hash:"a".repeat(64),position:2,queued:2,arrivedAtMs:now-100000,pid:500,mode:"exclusive"}],violations:[],requests:new Map([[100,{version:1,holderPid:100,holderStartedAtMs:now-9000000,requesterPid:200,requesterStartedAtMs:now-1000000,resource,reason:"stale",atMs:now-30000,requesterComponents:{"graph.generation":"bbbbbbbbbbbb"}}]]),history:()=>[]},{});
 const holder=(pid:number)=>report.holders.find(row=>row.pid===pid)!;
 expect(holder(100).standing).toMatchObject({state:"stale",staleIds:["graph.generation"],phase:"inference",done:100,total:400,etaMinutes:30,chain:[80,90],yieldRequest:{requesterPid:200,reason:"stale",ageSeconds:30}});
 expect(holder(300).standing).toMatchObject({state:"stuck"});expect(holder(400).standing).toEqual({state:"legacy"});
 expect(holder(200).standing).toMatchObject({state:"progressing"});
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...statusSchema.definitions.Report,definitions:statusSchema.definitions});
 expect(validate(report)).toBe(true);expect(validate({...report,holders:[{...holder(100),standing:{state:"dead"}}]})).toBe(false);
 const english=formatLeaseReportV1("en",report).join("\n"),german=formatLeaseReportV1("de",report).join("\n");
 for(const text of ["STALE: runs older code (graph.generation)","inference: 100 of 400","about 30 min left","started by 80 > 90","yield requested by process 200 (stale)","STUCK: no progress for 100 min","no standing published"])expect(english).toContain(text);
 for(const text of ["VERALTET: läuft mit älterem Code (graph.generation)","inference: 100 von 400","noch etwa 30 min","gestartet von 80 > 90","Freigabe angefordert von Prozess 200 (stale)","FESTGEFAHREN: seit 100 min kein Fortschritt","kein Zustand veröffentlicht"])expect(german).toContain(text);
});

test("a head holder without any registry entry is unknown, not legacy: a pruned heartbeat never gets a healthy holder stopped",async()=>{
 const registry=mkdtempSync(join(tmpdir(),"lease-unknown-")),calls:[number,string|number][]=[],now=Date.now();
 try{
  const ports={alive:()=>true,signal:(pid:number,signal:"SIGTERM"|"SIGKILL"|0)=>{calls.push([pid,signal]);return true;},osStartedAtMs:()=>now-200*60000,sleep:async()=>{},write:()=>{}};
  const notice=await observeLeaseHolderV1({directory:"/virtual/leases",resource:"repository-graph-facts:/w:shard:7",headPid:process.pid+1,headMode:"exclusive",headArrivalMs:now-120*60000,nowMs:now,environment:{SEMIO_LEASE_RECLAIM:"terminate",SEMIO_LANGUAGE:"en"},root:registry,ports});
  expect(notice?.holderState).toBe("legacy");
  expect(calls.filter(([,signal])=>signal!==0)).toEqual([]);
 }finally{rmSync(registry,{recursive:true,force:true});}
});

test("the elapsed-time column of ps parses in every shape and refuses anything else",()=>{
 expect([leaseElapsedSecondsV1("   05:07\n"),leaseElapsedSecondsV1("01:02:03"),leaseElapsedSecondsV1("2-03:04:05"),leaseElapsedSecondsV1("12-00:00:01\n"),leaseElapsedSecondsV1(""),leaseElapsedSecondsV1("Mon Oct 26 07:46:10 2026"),leaseElapsedSecondsV1("5")]).toEqual([307,3723,183845,1036801,undefined,undefined,undefined]);
});

test("activity probes renew liveness exactly like the model: due only after the probe interval, busy tree or CPU floor counts, an idle process does not, newer progress is never lowered",()=>{
 fixture.activityCases.forEach((row,index)=>{
  const got=leaseActivityV1({nowMs:row.nowMs,progressAtMs:row.progressAtMs,...(row as {lastProbeAtMs?:number}).lastProbeAtMs===undefined?{}:{lastProbeAtMs:(row as {lastProbeAtMs:number}).lastProbeAtMs},sample:()=>row.sample});
  expect([row.id,got]).toEqual([row.id,row.expected]);expect([row.id,oracle.activity[index]]).toEqual([row.id,row.expected]);
 });
 let read=0;leaseActivityV1({nowMs:1000,progressAtMs:0,lastProbeAtMs:999,sample:()=>{read++;return {treeActive:true,cpuDeltaMilliseconds:0};}});
 expect(read,"the sample is not even read before the probe is due").toBe(0);
});

const spawnChild=(source:string)=>Bun.spawn(["bun","-e",source],{stdout:"ignore",stderr:"ignore"});
test("Law: a holder whose child burns CPU for longer than the stuck bound is never asked and never accepts a stuck request; a holder whose whole tree idles ages into stuck as before",async()=>{
 const resource="repository-graph-facts:/w:shard:7",bound=policy.freshness.stuckAfterMilliseconds.find(row=>row.prefix==="repository-graph-facts:")!.milliseconds,t0=Date.now(),twoHoursAgo=t0-2*bound;
 const request={version:1 as const,holderPid:process.pid,holderStartedAtMs:1,requesterPid:process.pid+1,requesterStartedAtMs:t0,resource,reason:"stuck" as const,atMs:t0,requesterComponents:{}};
 const accept=(at:number)=>{const standing=leaseStandingSnapshotV1(false,at,{});return {state:leaseHolderStateV1({nowMs:at,holdSinceMs:twoHoursAgo,resource,standing}).state,accept:leaseYieldAcceptV1({request:{...request,atMs:at},pid:process.pid,startedAtMs:1,nowMs:at,holdsExclusive:true,holdSinceMs:twoHoursAgo,standing,bound:[],moved:[]}).accept};};
 const busy=spawnChild("let x=0;for(;;)x=(x+1)%1000003;");
 try{
  await Bun.sleep(1500);
  setLeaseActivityPortsV1(undefined);
  noteLeaseProgressV1({stage:"started"},twoHoursAgo);
  expect(accept(t0),"without a probe the silent holder is stuck, which is the false positive of a long compile").toEqual({state:"stuck",accept:true});
  probeLeaseActivityV1(t0);noteLeaseProgressV1({stage:"started"},twoHoursAgo);await Bun.sleep(700);probeLeaseActivityV1(t0+31000);
  expect(accept(t0+31000),"the second probe saw the CPU time of the child grow").toEqual({state:"progressing",accept:false});
  expect(leaseYieldDecisionV1(leaseHolderStateV1({nowMs:t0+31000,holdSinceMs:twoHoursAgo,resource,standing:leaseStandingSnapshotV1(false,t0+31000,{})}).state,"request",[])).toBeUndefined();
 }finally{busy.kill(9);await busy.exited;}
 const idle=spawnChild("setTimeout(()=>{},120000)");
 try{
  await Bun.sleep(1800);
  setLeaseActivityPortsV1({tree:createScriptProcessActivitySamplerV1({note:()=>{}}),cpuMilliseconds:()=>0});
  probeLeaseActivityV1(t0);noteLeaseProgressV1({stage:"started"},twoHoursAgo);await Bun.sleep(700);probeLeaseActivityV1(t0+31000);probeLeaseActivityV1(t0+62000);
  expect(accept(t0+62000),"an idle tree and an idle process age into stuck").toEqual({state:"stuck",accept:true});
 }finally{idle.kill(9);await idle.exited;setLeaseActivityPortsV1(undefined);}
},60000);

test("the holder reads the moved stamps from the disk itself: a failing or silent provider moves nothing, a provider's answer is passed on as it is",()=>{
 registerLeaseImplementationProviderV1({components:()=>({}),bound:()=>["graph.generation"]});
 expect(leaseMovedBoundV1("repository-graph-facts:/w:shard:7"),"no `moved` member: nothing moved").toEqual([]);
 registerLeaseImplementationProviderV1({components:()=>({}),bound:()=>["graph.generation"],moved:()=>{throw Error("table unreadable");}});
 expect(leaseMovedBoundV1("repository-graph-facts:/w:shard:7"),"an unreadable table is never a reason to yield").toEqual([]);
 registerLeaseImplementationProviderV1({components:()=>({}),bound:()=>["graph.generation"],moved:resource=>resource.startsWith("repository-graph-facts:")?["graph.generation"]:[]});
 expect(leaseMovedBoundV1("repository-graph-facts:/w:shard:7")).toEqual(["graph.generation"]);expect(leaseMovedBoundV1("artifact:/w/x")).toEqual([]);
 registerLeaseImplementationProviderV1({components:()=>({}),bound:()=>[]});
});

const entryFile=(registry:string,directory:string,pid:number,startedAtMs:number,sinceMs:number,resource:string,now:number)=>{
 const folder=leaseRegistryFolderV1(registry,directory);mkdirSync(folder,{recursive:true});
 writeFileSync(`${folder}/${pid}-${startedAtMs}.json`,JSON.stringify({version:1,pid,startedAtMs,updatedAtMs:now,command:"./script.ts repo run x",directory,holds:[{resource,mode:"exclusive",sinceMs,task:1}],waits:[]}));
};
const hours=3600000;
test("Law: terminate mode ages a holder from its acquisition, not from its ticket, and stops it with its tree: children first named by their start time, a recycled pid or a changed head identity is never signalled, and a refusal is said once",async()=>{
 const registry=mkdtempSync(join(tmpdir(),"lease-terminate-")),now=Date.now(),resource="repository-graph-facts:/w:shard:7",directory="/virtual/leases",head=process.ppid,startedAtMs=now-8*hours;
 const environment={SEMIO_LEASE_RECLAIM:"terminate",SEMIO_LANGUAGE:"en"};
 try{
  const make=(overrides:{started?:Record<number,number[]>;alive?:Set<number>;children?:number[]}={})=>{
   const calls:[number,string|number][]=[],reads:Record<number,number>={},written:string[]=[],alive=overrides.alive??new Set([head,201,202,203]);let clock=now;
   const startOf=(pid:number)=>{const row=overrides.started?.[pid]??[startedAtMs];const at=reads[pid]=(reads[pid]??-1)+1;return row[Math.min(at,row.length-1)];};
   return {calls,written,ports:{alive:(pid:number)=>alive.has(pid),signal:(pid:number,signal:"SIGTERM"|"SIGKILL"|0)=>{if(signal!==0)calls.push([pid,signal]);return true;},osStartedAtMs:(pid:number):number|undefined=>startOf(pid),sleep:async(ms:number)=>{clock+=ms;},now:()=>clock,write:(text:string)=>{written.push(text);},children:()=>overrides.children??[201,202]}};
  };
  const observe=(ports:ReturnType<typeof make>["ports"],arrivalMs:number,holdSinceMs:number)=>{entryFile(registry,directory,head,startedAtMs,holdSinceMs,resource,now);return observeLeaseHolderV1({directory,resource,headPid:head,headMode:"exclusive",headArrivalMs:arrivalMs,nowMs:now,environment,root:registry,ports});};
  const young=make();
  await observe(young.ports,now-7*hours,now-30*60000);
  expect(young.calls,"queued 6 h, held 30 min (the process is older than its ticket): the 90 minutes of before would have stopped it").toEqual([]);
  const old=make({started:{201:[startedAtMs+1000],202:[startedAtMs+2000,startedAtMs+9000000]}});
  await observe(old.ports,now-7*hours,now-5*hours);
  expect(old.calls.filter(([,signal])=>signal==="SIGTERM"),"the head is asked first").toEqual([[head,"SIGTERM"]]);
  expect(old.calls.filter(([,signal])=>signal==="SIGKILL").map(([pid])=>pid).sort((left,right)=>left-right),"a head that ignores SIGTERM, and the child whose start time still matches; the recycled pid 202 is spared").toEqual([head,201].sort((left,right)=>left-right));
  const changed=make({started:{[head]:[startedAtMs,startedAtMs,startedAtMs+9000000]}});
  await observe(changed.ports,now-7*hours,now-5*hours);
  expect(changed.calls.filter(([pid,signal])=>pid===head&&signal==="SIGKILL"),"the head identity changed between the signals: no SIGKILL").toEqual([]);
  const gone=make({alive:new Set([201])});
  await observe(gone.ports,now-7*hours,now-5*hours);
  expect(gone.calls.filter(([,signal])=>signal==="SIGKILL").map(([pid])=>pid),"a head that ended on SIGTERM leaves no tree behind").toEqual([201]);
  const unknown=make();const noStart={...unknown.ports,osStartedAtMs:():number|undefined=>undefined};
  await observe(noStart,now-7*hours,now-5*hours);await observe(noStart,now-7*hours,now-5*hours);await observe(noStart,now-7*hours,now-5*hours);
  expect(unknown.written,"the refusal of an unknown start time is said once, not at every tick").toHaveLength(1);
  expect(unknown.calls.filter(([,signal])=>signal!==0)).toEqual([]);
 }finally{rmSync(registry,{recursive:true,force:true});}
},30000);

test("lease status and order diagnostics name modes and priorities in the chosen language and never print a counter of an unknown total, checked against SQLite", async () => {
  const { formatLeaseViolationV1 } = await import("../../🧭️order/🟦️.ts"), orderSchema = (await import("../../🧭️order/🧬️schema/🔣️.json")).default, db = new Database(":memory:");
  const word = (json: unknown, key: string, language: string) => (db.query("select json_extract(?1,'$.\"'||?2||'\".'||?3) w").get(JSON.stringify(json), key, language) as { w: string }).w;
  const holder = { resource: "native-fixed-image:/x/y", hash: "a".repeat(64), mode: "exclusive" as const, pid: 7, task: 1, sinceMs: 0, heldSeconds: 12, class: "image", rank: 7, source: "registry" as const, standing: { state: "progressing" as const, priority: "agent" as const, phase: "dependencies:generation-verify", done: 81920, total: 0, implementationAgeSeconds: 3 } };
  const waiter = { resource: "x", hash: "b".repeat(64), mode: "shared" as const, pid: 8, task: 2, sinceMs: 0, waitedSeconds: 4, class: "image", rank: 7, source: "queue" as const, blockers: [] as string[], priority: "background" as const };
  const report = { version: 1 as const, directory: "/d", atMs: 0, waitFor: [], cycles: [], violations: [], holders: [holder, { ...holder, mode: "shared" as const, standing: undefined }], waiters: [waiter] };
  try {
    for (const language of ["en", "de"] as const) {
      const lines = formatLeaseReportV1(language, report as never), first = language === "en" ? "HELD" : "GEHALTEN", second = language === "en" ? "WAITING" : "WARTET";
      expect(lines[1], language).toMatch(new RegExp(`^${first}\\s+${word(statusSchema.definitions.Messages.const, "modeExclusive", language)} `, "u"));
      expect(lines[3], language).toMatch(new RegExp(`^${first}\\s+${word(statusSchema.definitions.Messages.const, "modeShared", language)} `, "u"));
      expect(lines[4], language).toMatch(new RegExp(`^${second}\\s+${word(statusSchema.definitions.Messages.const, "modeShared", language)} `, "u"));
      expect(lines[4], language).toContain(`[${word(schema.definitions.Messages.const, "priority.background", language)}]`);
      expect(lines[2], language).toContain(word(schema.definitions.Messages.const, "priority.agent", language));
      expect(lines[2], language).toContain("dependencies:generation-verify: 81920");
      expect(lines[2], language).not.toMatch(/ (?:of|von) 0\b/u);
      const violation = formatLeaseViolationV1(language, { kind: "out-of-order", held: "h", heldClass: "image", heldRank: 7, heldMode: "shared", requested: "r", requestedClass: "cargo", requestedRank: 2, requestedMode: "exclusive" } as never);
      expect(violation, language).toContain(`${word(orderSchema.definitions.Messages.const, "modeShared", language)}`);
      if (language === "de") { expect(violation).toContain("geteilt gehalten"); expect(violation).not.toMatch(/\bshared\b|\bexclusive\b/u); expect(lines.join("\n")).not.toMatch(/\bshared\b|\bexclusive\b/u); }
    }
    expect(word(statusSchema.definitions.Messages.const, "modeShared", "de")).toBe("geteilt");
    expect(word(statusSchema.definitions.Messages.const, "modeExclusive", "de")).toBe("exklusiv");
  } finally { db.close(); }
});
