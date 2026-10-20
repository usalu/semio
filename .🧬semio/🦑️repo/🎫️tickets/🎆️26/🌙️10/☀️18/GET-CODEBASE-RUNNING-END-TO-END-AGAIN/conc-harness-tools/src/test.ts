/** 🤹 Concurrent session conformance: the wait derivation, attribution, criteria, edits, digests and workload admission are checked against language-agnostic fixtures, an independent Python oracle and Bazel's own elapsed time; the runner is exercised with real processes that contend for real leases of the lease module; the command through its ports. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import fixture from "./@@FIXTURES@@/@@JSON@@" with {type:"json"};
import workloads from "../@@WORKLOADS@@/@@JSON@@" with {type:"json"};
import catalog from "../../../@@OPERATIONS@@/@@CATALOG@@" with {type:"json"};
import {incrementalLabelV1,type IncrementalSliceV1} from "../../@@TS@@";
import {concurrentAttributeV1,concurrentClassifyV1,concurrentDigestV1,concurrentEvaluateV1,concurrentIndependenceIssuesV1,concurrentLineKeeperV1,concurrentMergeWaitsV1,concurrentMutateV1,concurrentNxFitV1,concurrentRelationV1,concurrentReportV1,concurrentSelectV1,concurrentTraceWaitsV1,concurrentWaitsV1,concurrentWorkloadIssuesV1,renderConcurrentReportV1,renderConcurrentWorkloadsV1,type ConcurrentCriteriaV1,type ConcurrentRunV1,type ConcurrentSessionV1,type ConcurrentStepV1,type ConcurrentWorkloadSetV1,type ConcurrentWorkloadV1} from "../@@TS@@";
import {concurrentBarriersV1,concurrentCliPortsV1,concurrentPlanV1,concurrentRunWorkloadV1,concurrentSleepV1,concurrentSpawnV1,type ConcurrentHostV1,type ConcurrentPortsV1} from "../@@RUNNER@@/@@TS@@";
import {concurrencyArgumentsV1,concurrencyOptionNamesV1,renderConcurrentPlanV1,runConcurrencyCommandV1} from "../@@COMMAND@@/@@TS@@";
import type {IncrementalCommandPortsV1} from "../../@@COMMAND@@/@@TS@@";
import {validateJsonSchemaSubset} from "@@EIGHT@@/@@MODULES@@/@@SCHEMA@@/@@VALIDATOR@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@TS@@";

const findRoot=(from:string):string=>{for(let at=from;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"bun.lock")))return at;if(dirname(at)===at)throw Error("repository root not found");}};
const root=process.env.SEMIO_TEST_REPO_ROOT??findRoot(import.meta.dir),set=workloads as unknown as ConcurrentWorkloadSetV1,policy=schema.definitions.Policy.const;
const ajv=new Ajv({strict:true,allowUnionTypes:true}),define=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const oracle=join(import.meta.dir,"@@ORACLE@@/@@PY@@"),schemaPath=join(import.meta.dir,"../@@SCHEMA@@/@@JSON@@");
const python=(op:string,cases:readonly unknown[]):any[]=>{const run=spawnSync("python3",["-I",oracle],{input:JSON.stringify({op,schemaPath,cases}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const plain=(value:unknown):any=>JSON.parse(JSON.stringify(value));
const script="@@BUILDSCRIPT@@",leaseRoot=process.env.SEMIO_TEST_LEASE_ROOT??root,leasePath="@@FRAMEWORK@@/@@MODULES@@/@@PROCESS@@/@@LEASES@@/@@TS@@",stubPath=join(import.meta.dir,"@@STUB@@/@@TS@@");

test("the schema is valid and validates the fixture, the workloads and the result shapes the way the repository validator does",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(define("Fixture")(fixture)).toBe(true);
 expect(define("WorkloadSet")(workloads)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.WorkloadSet,workloads,schema)).toEqual([]);
 const title={en:"a",de:"b"},wait={class:"a",resource:"r",mode:"shared",source:"lease",precise:false,startMs:0,endMs:1,waitMs:1,holderPid:null,position:null};
 for(const[name,value,ok]of [
  ["Step",{kind:"edit",file:"a",op:"comment",language:"ts"},true],["Step",{kind:"edit",file:"a",op:"comment",language:"cobol"},false],["Step",{kind:"run",target:"nocolon"},false],["Step",{kind:"pause",milliseconds:0},false],["Step",{kind:"barrier",id:"Go"},false],["Step",{kind:"repo",args:[]},false],
  ["Rule",{id:"r",kind:"max-wait",title,owners:["conc-graph"],classes:["x"],relation:"unrelated",maximumMilliseconds:5},true],["Rule",{id:"r",kind:"max-wait",title,owners:["conc-graph"],classes:["x"],relation:"weird",maximumMilliseconds:5},false],["Rule",{id:"r",kind:"no-failure",title,owners:[]},false],["Rule",{id:"r",kind:"executes-once",title,owners:["a"],sessions:["*"],excludeUncached:true},true],
  ["Wait",wait,true],["Wait",{...wait,source:"nope"},false],["Wait",{...wait,precise:"yes"},false],["TraceRow",{version:1,resource:"r",mode:"shared",startedMs:1,acquiredMs:2,pid:3},true],["TraceRow",{version:1,resource:"r",mode:"shared",startedMs:1,acquiredMs:2,pid:0},false],
  ["Session",{id:"a",area:"x",group:"g",steps:[]},false],["Session",{id:"a",area:"x",group:"g",steps:[{kind:"barrier",id:"go"}]},true],["LeaseClass",{id:"x",label:title,scope:"host",patterns:[]},false]
 ] as const){
  expect(define(name)(value),name+JSON.stringify(value)).toBe(ok);
  expect(validateJsonSchemaSubset(schema.definitions[name as keyof typeof schema.definitions],value,schema).length===0,name+JSON.stringify(value)).toBe(ok);
 }
});

test("the authored workloads are closed, consistent, bilingual and name every executor and every lock class of the acceptance",()=>{
 expect(concurrentWorkloadIssuesV1(workloads)).toEqual([]);
 expect(set.workloads.map(row=>row.id)).toEqual(["disjoint-projects","same-target-cold","warm-fanout","readers-while-edit"]);
 const disjoint=set.workloads[0]!;
 expect(disjoint.sessions.map(row=>row.area)).toEqual(["disjoint-ts","disjoint-rust","disjoint-go","disjoint-schema"]);
 expect(new Set(disjoint.sessions.map(row=>row.group)).size).toBe(4);
 for(const row of set.workloads){expect(row.sessions.length).toBeLessThanOrEqual(policy.maximumSessions);expect(set.criteria.some(criteria=>criteria.id===row.criteria)).toBe(true);}
 for(const row of set.workloads)for(const owner of [{id:"setup",steps:row.setup},...row.sessions])for(const step of owner.steps)if(step.kind==="edit"){expect(existsSync(join(root,step.file)),step.file).toBe(true);expect(join(root,step.file).length).toBeLessThanOrEqual(policy.maximumPathLength);}
 const rules=set.criteria.find(row=>row.id==="default")!.rules,owners=new Set(rules.flatMap(rule=>rule.owners));
 expect(owners).toEqual(new Set(["conc-graph","conc-bazel","conc-cargo","conc-deps","conc-leases"]));
 for(const name of ["graph-facts","dependency-image","bazel-server","cargo-units","capacity-admission"])expect(rules.some(rule=>rule.kind==="max-wait"&&rule.classes.includes(name)&&rule.relation==="unrelated"&&rule.maximumMilliseconds===5000),name).toBe(true);
 expect(rules.some(rule=>rule.kind==="executes-once"&&rule.excludeUncached)).toBe(true);
 expect(rules.some(rule=>rule.kind==="serial-equal")&&rules.some(rule=>rule.kind==="no-failure")).toBe(true);
 const texts:{en:string;de:string}[]=[],walk=(value:unknown):void=>{if(Array.isArray(value))value.forEach(walk);else if(value&&typeof value==="object"){const row=value as Record<string,unknown>;if(typeof row.en==="string"&&typeof row.de==="string")texts.push(row as {en:string;de:string});else Object.values(row).forEach(walk);}};
 walk(workloads);
 expect(texts.length).toBeGreaterThan(80);
 for(const row of texts)expect(row.de,row.en).not.toBe(row.en);
 expect(set.classes.at(-1)!.id).toBe("other");
 for(const[resource,expected]of [["repository-graph-facts:/x","graph-facts"],["native-fixed-image:/x/.cache/b/d","dependency-image"],["bazel:server","bazel-server"],["cargo:build directory","cargo-units"],["semio.host.capacity/v1","capacity-admission"],["/Users/a/.bun/bin/bun","toolchain-acquisition"],["C:\\tools\\bun.exe","toolchain-acquisition"],["anything","other"]] as const)expect(concurrentClassifyV1(set.classes,resource),resource).toBe(expected);
});

test("lock waits are derived from stderr lines identically by TypeScript, the Python oracle and the recorded expectations",()=>{
 const answer=python("waits",fixture.lines.map(row=>({classes:row.classes,endMs:row.endMs,lines:row.lines})));
 for(const[index,row]of fixture.lines.entries()){
  expect(plain(concurrentWaitsV1(row.lines,row.endMs,row.classes as never)),row.name).toEqual(row.waits);
  expect(answer[index],row.name).toEqual(row.waits);
 }
 const named=(name:string)=>fixture.lines.find(row=>row.name===name)!.waits;
 expect(named("real-queue-log")).toEqual([{class:"dependency-image",resource:"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d",mode:"shared",source:"lease",precise:false,startMs:86,endMs:45406,waitMs:45306,holderPid:44358,position:3}]);
 expect(named("typed-progress-rows-are-ignored")).toEqual([]);expect(named("malformed-rows-are-ignored")).toEqual([]);expect(named("empty")).toEqual([]);
 expect(named("restart-splits-episodes").length).toBe(2);expect(named("gap-splits-episodes").length).toBe(2);expect(named("shared-and-exclusive-are-separate").length).toBe(2);
 expect(named("bazel-wait-with-continuations")).toMatchObject([{class:"bazel-server",source:"bazel",precise:true,startMs:2000,endMs:45000,waitMs:43000,holderPid:4242}]);
 expect(named("bazel-wait-open-at-end")).toMatchObject([{waitMs:10000,endMs:12000}]);
 expect(named("cargo-two-locks").map(row=>[row.resource,row.waitMs])).toEqual([["cargo:package cache",800],["cargo:build directory",1500]]);
 expect(named("capacity-and-toolchain-classes").map(row=>row.class).sort()).toEqual(["capacity-admission","cargo-units","other","toolchain-acquisition","toolchain-acquisition"]);
});

test("a real concurrent Bazel 8.4.2 build is measured within Bazel's own elapsed time",()=>{
 const real=fixture.lines.find(row=>row.name==="bazel-8-4-2-real-concurrent-build")!,[found]=real.waits,own=/Elapsed time: ([0-9.]+)s/u.exec(real.lines.at(-1)!.line)![1]!;
 expect(real.waits.length).toBe(1);expect(found!.class).toBe("bazel-server");expect(found!.holderPid).toBe(36644);
 expect(found!.waitMs).toBeLessThanOrEqual(Number(own)*1000);expect(found!.waitMs).toBeGreaterThan(Number(own)*1000-300);
});

test("the line keeper keeps the lock rows and wait lines of a command and drops the typed progress of other publishers and ordinary output",()=>{
 const keep=concurrentLineKeeperV1(),row=(resource:string)=>`[script] {"owner":"o","command":"c","stage":"running","activity":{"resource":${JSON.stringify(resource)},"mode":"shared","elapsedMs":5}}`;
 expect(keep("compiling")).toBe(false);expect(keep(row("repository-graph-facts:/x"))).toBe(true);expect(keep(row("javascript-source-provider:77"))).toBe(false);expect(keep(row("dependencies:store-input-replay"))).toBe(false);expect(keep(row("lease-queue:{}"))).toBe(true);
 expect(keep('[script] {"owner":"o","command":"c","stage":"running"}')).toBe(false);
 expect(keep("Another command (pid=4242) is running. Waiting for it to complete on the server (server_pid=77)...")).toBe(true);expect(keep("INFO: Invocation ID: a")).toBe(true);expect(keep("INFO: later")).toBe(false);
 expect(keep("    Blocking waiting for file lock on build directory")).toBe(true);expect(keep("   Compiling x")).toBe(true);expect(keep("   Compiling y")).toBe(false);
});

test("waits are attributed to the sessions that ran a command meanwhile, identically in TypeScript and the Python oracle",()=>{
 const answer=python("relation",fixture.relations.map(row=>({wait:row.wait,self:row.self,others:row.others})));
 for(const[index,row]of fixture.relations.entries()){
  expect(concurrentRelationV1(row.wait,row.self,row.others as never),row.name).toEqual({relation:row.relation,holders:row.holders} as never);
  expect({relation:answer[index].relation,holders:answer[index].holders},row.name).toEqual({relation:row.relation,holders:row.holders});
 }
 const wait=(start:number,end:number)=>({class:"c",resource:"r",mode:"shared" as const,source:"trace" as const,precise:true,startMs:start,endMs:end,waitMs:end-start,holderPid:null,position:null});
 const step=(start:number,end:number,waits:ReturnType<typeof wait>[])=>({index:0,kind:"run" as const,command:["run","t:t"],startMs:start,endMs:end,exit:0,waits,observation:null,observed:"none" as const,outputs:null,stdout:null});
 const attributed=concurrentAttributeV1([{id:"a",group:"ga",startMs:0,endMs:900,steps:[step(0,900,[wait(100,800)])]},{id:"b",group:"gb",startMs:0,endMs:500,steps:[step(0,500,[])]},{id:"c",group:"ga",startMs:600,endMs:900,steps:[step(600,900,[wait(650,700)])]}]);
 expect(attributed[0]!.steps[0]!.waits[0]).toMatchObject({relation:"related",holders:["b","c"]});
 expect(attributed[2]!.steps[0]!.waits[0]).toMatchObject({relation:"related",holders:["a"]});
});

test("the pass criteria judge a measurement identically in TypeScript and the Python oracle",()=>{
 const answer=python("evaluate",fixture.evaluations.map(row=>({criteria:row.criteria,measurement:row.measurement})));
 for(const[index,row]of fixture.evaluations.entries()){
  expect(plain(concurrentEvaluateV1(row.criteria as ConcurrentCriteriaV1,row.measurement as never)),row.name).toEqual(row.results);
  expect(answer[index],row.name).toEqual(row.results);
 }
 const by=(name:string)=>fixture.evaluations.find(row=>row.name===name)!.results;
 expect(by("default-all-pass").every(row=>row.status==="pass"||row.status==="skip")).toBe(true);
 expect(by("default-all-pass").find(row=>row.rule==="second-session-zero-actions")!.status).toBe("pass");
 expect(by("graph-facts-wait-fails-for-unrelated")[0]).toMatchObject({status:"fail",value:6900,limit:5000});
 expect(by("related-waits-are-not-limited")[0]!.status).toBe("pass");expect(by("external-waits-count-as-unrelated")[0]!.status).toBe("fail");expect(by("exact-limit-passes")[0]!.status).toBe("pass");
 expect(by("imprecise-wait-below-limit-is-unproven")[0]!.status).toBe("skip");expect(by("imprecise-wait-above-limit-fails")[0]!.status).toBe("fail");
 expect(by("executes-once-duplicate-fails")[0]).toMatchObject({status:"fail",value:1,detail:["y:build: a, b"]});
 expect(by("executes-once-uncached-excluded")[0]!.status).toBe("pass");expect(by("executes-once-same-session-twice-is-fine")[0]!.status).toBe("pass");
 expect(by("executes-once-ambiguous-is-skipped")[0]!.status).toBe("skip");expect(by("executes-once-without-runs-is-skipped")[0]!.status).toBe("skip");
 expect(by("serial-equal-skip-without-serial")[0]!.status).toBe("skip");expect(by("serial-equal-mismatches")[0]!.value).toBe(5);
 expect(()=>concurrentEvaluateV1({id:"x",title:{en:"a",de:"b"},rules:[]} as never,fixture.evaluations[0]!.measurement as never)).toThrow("Closed concurrent criteria required");
 expect(()=>concurrentEvaluateV1(fixture.evaluations[0]!.criteria as never,{} as never)).toThrow("Closed concurrent measurement required");
});

test("edits are rendered identically in TypeScript and the Python oracle, always differ from their input, and unsupported edits and nonces are refused",()=>{
 const answer=python("mutate",fixture.mutations);
 for(const[index,row]of fixture.mutations.entries()){
  const result=concurrentMutateV1(row.text,row.op as never,row.language as never,row.nonce);
  expect(result,row.name).toBe(row.result);expect(answer[index],row.name).toBe(row.result);expect(result,row.name).not.toBe(row.text);
 }
 expect(()=>concurrentMutateV1("x","comment","go","BAD")).toThrow("Closed concurrent nonce required");
 expect(()=>concurrentMutateV1("x","private-body","text","abc123")).toThrow("unsupported");
 expect(concurrentMutateV1("package a\n","public-api","go","abc123x9")).toBe("package a\nconst ConcurrentABC123X9 = "+Number.parseInt("abc123",36)%1000003+"\n");
});

test("independence, workload admission, Nx run records, digests, lease traces, merges and tallies equal the Python oracle and the recorded expectations",()=>{
 const independence=python("independence",fixture.independence);
 for(const[index,row]of fixture.independence.entries()){expect(concurrentIndependenceIssuesV1(row.rows),row.name).toEqual(row.issues);expect(independence[index],row.name).toEqual(row.issues);}
 const admission=python("workloads",fixture.workloads);
 for(const[index,row]of fixture.workloads.entries()){expect(concurrentWorkloadIssuesV1(row.set),row.name).toEqual(row.issues);expect(admission[index],row.name).toEqual(row.issues);}
 expect(fixture.workloads.find(row=>row.name==="authored")!.issues).toEqual([]);
 for(const row of fixture.workloads.slice(1))expect(row.issues.length,row.name).toBeGreaterThan(0);
 const nx=python("nx",fixture.nx);
 for(const[index,row]of fixture.nx.entries()){expect(plain(concurrentNxFitV1(row.record,row.ids,{startMs:row.windowStart,endMs:row.windowEnd},row.concurrent)),row.name).toEqual(row.result);expect(nx[index],row.name).toEqual(row.result);}
 const digests=python("digest",fixture.digests);
 for(const[index,row]of fixture.digests.entries()){expect(concurrentDigestV1(row.parts.map(part=>({path:part.path,bytes:part.text}))),row.name).toBe(row.digest);expect(digests[index],row.name).toBe(row.digest);}
 expect(fixture.digests[0]!.digest).toBe(fixture.digests[1]!.digest);expect(new Set(fixture.digests.map(row=>row.digest)).size).toBe(4);
 expect(concurrentDigestV1([{path:"a",bytes:new TextEncoder().encode("one")}])).toBe(concurrentDigestV1([{path:"a",bytes:"one"}]));
 const traces=python("traces",fixture.traces);
 for(const[index,row]of fixture.traces.entries()){expect(plain(concurrentTraceWaitsV1(row.text,row.origin,row.classes as never)),row.name).toEqual(row.waits);expect(traces[index],row.name).toEqual(row.waits);}
 expect(fixture.traces[0]!.waits).toMatchObject([{class:"dependency-image",waitMs:200,precise:true,source:"trace"},{class:"graph-facts",waitMs:8000,holderPid:4242,startMs:500}]);
 const merges=python("merges",fixture.merges);
 for(const[index,row]of fixture.merges.entries()){expect(plain(concurrentMergeWaitsV1(row.derived as never,row.traced as never)),row.name).toEqual(row.waits);expect(merges[index],row.name).toEqual(row.waits);}
 const tallies=python("tally",fixture.tallies);
 for(const[index,row]of fixture.tallies.entries()){const report=concurrentReportV1(row.runs as ConcurrentRunV1[]);expect({passed:report.passed,failed:report.failed,skipped:report.skipped,owners:report.owners},row.name).toEqual(row.result);expect(tallies[index],row.name).toEqual(row.result);}
});

test("workloads are selected by id and session count, and reports and lists render in both languages with every label",()=>{
 expect(concurrentSelectV1(set,"all",undefined).map(row=>row.sessions.length)).toEqual([3,3,3,3]);
 expect(concurrentSelectV1(set,["disjoint-projects"],4)[0]!.sessions.map(row=>row.id)).toEqual(["ts","rust","go","schema"]);
 expect(concurrentSelectV1(set,["warm-fanout"],1)[0]!.sessions.length).toBe(1);
 expect(()=>concurrentSelectV1(set,["nope"],undefined)).toThrow("Unknown workload: nope");
 for(const bad of [0,9,1.5])expect(()=>concurrentSelectV1(set,"all",bad)).toThrow("Sessions must be between");
 const runs=fixture.evaluations.filter(row=>row.criteria.id==="default").map(row=>({workload:"w",tool:"bazel" as const,criteria:"default",measurement:row.measurement as never,rules:row.results as never}));
 const report=concurrentReportV1(runs);
 const en=renderConcurrentReportV1(report,set,"en"),de=renderConcurrentReportV1(report,set,"de");
 expect(en).toContain("Concurrent session conformance");expect(de).toContain("Konformität gleichzeitiger Sitzungen");
 expect(en).toContain("acceptance by executor");expect(de).toContain("Abnahme je Executor");expect(en).toContain("conc-graph: ");
 expect(en).toContain("Bazel server");expect(en).toContain("Graph facts");expect(en).not.toBe(de);
 expect(renderConcurrentWorkloadsV1(set,"en")).toContain("disjoint-projects");expect(renderConcurrentWorkloadsV1(set,"de")).toContain(set.workloads[0]!.title.de);
 expect(()=>renderConcurrentReportV1(report,{...set,labels:{}} as never,"en")).toThrow("Unknown concurrent label");
});

test("barriers release together, release when a session leaves and reject on abort; sleeping is abortable",async()=>{
 const sessions=[{id:"a",steps:[{kind:"barrier",id:"go"}]},{id:"b",steps:[{kind:"barrier",id:"go"}]},{id:"c",steps:[{kind:"barrier",id:"go"}]}] as unknown as ConcurrentSessionV1[];
 const controller=new AbortController(),barriers=concurrentBarriersV1(sessions,controller.signal),order:string[]=[];
 const a=barriers.arrive("go","a").then(()=>order.push("a")),b=barriers.arrive("go","b").then(()=>order.push("b"));
 await new Promise(done=>setTimeout(done,30));expect(order).toEqual([]);
 const c=barriers.arrive("go","c").then(()=>order.push("c"));await Promise.all([a,b,c]);expect(order.sort()).toEqual(["a","b","c"]);
 const two=concurrentBarriersV1(sessions,controller.signal),first=two.arrive("go","a").then(()=>"released");two.leave(sessions[1]!);await new Promise(done=>setTimeout(done,20));two.leave(sessions[2]!);expect(await first).toBe("released");
 const stopped=new AbortController(),three=concurrentBarriersV1(sessions,stopped.signal),waiting=three.arrive("go","a");stopped.abort(Error("stop"));await expect(waiting).rejects.toThrow("stop");
 const sleeper=new AbortController(),sleeping=concurrentSleepV1(10000,sleeper.signal);sleeper.abort(Error("woke"));await expect(sleeping).rejects.toThrow("woke");
 await concurrentSleepV1(5,new AbortController().signal);
});

type SpecV1={leases?:{resource:string;mode:"shared"|"exclusive";holdMs:number}[];bazelWaitMs?:number;tasks?:{id:string;inputs:string[];ms:number}[];stdout?:string;counter?:boolean;exit?:number;workMs?:number};
const world=(base:string,config:{dedupe?:boolean;trace?:boolean;commands?:Record<string,SpecV1>;default?:SpecV1}={},extra:Record<string,string>={})=>{
 const copy=join(base,"copy"),scratch=join(base,"scratch");mkdirSync(join(copy,"@@BUILDDIR@@"),{recursive:true});mkdirSync(scratch);
 writeFileSync(join(copy,script),`import ${JSON.stringify(stubPath)};\n`);
 for(const[file,text]of Object.entries({"src/a.ts":"export const a=1;\n","src/b.ts":"export const b=1;\n","src/c.ts":"export const c=1;\n",...extra})){mkdirSync(dirname(join(copy,file)),{recursive:true});writeFileSync(join(copy,file),text);}
 writeFileSync(join(copy,"stub.json"),JSON.stringify({live:leaseRoot,lease:leasePath,dedupe:config.dedupe??true,trace:config.trace??false,commands:config.commands??{},default:config.default??{}}));
 return {copy,scratch,config};
};
const portsOf=(w:ReturnType<typeof world>,controller:AbortController,progress:string[]=[]):ConcurrentPortsV1=>{
 const host:ConcurrentHostV1={live:root,root:w.copy,script,executable:process.execPath,scratch:w.scratch,signal:controller.signal,environment:{PATH:process.env.PATH??"/usr/bin",HOME:process.env.HOME??tmpdir(),TMPDIR:tmpdir()},run:async()=>"",nxRecord:directory=>join(directory,"run.json"),adapter:async()=>{throw Error("unused");},inventory:async()=>[],now:()=>Date.now(),check:()=>controller.signal.throwIfAborted(),progress:async stage=>{progress.push(stage);}};
 return {...concurrentCliPortsV1(host,controller.signal),
  slice:async(_tool,target,files)=>{const tasks=Object.values(w.config.commands??{}).length?(w.config.commands?.[`run:${target}`]?.tasks??[]):[];return {version:1,selected:[],tasks:Object.fromEntries(tasks.map(task=>[task.id,{family:"bun-script",cache:true,outputs:true,dependsOn:[],consumes:[],inputs:task.inputs.filter(file=>files.includes(file))}]))} as IncrementalSliceV1;},
  observe:async(_tool,_slice,path)=>{if(!path||!existsSync(path))return {observation:null,observed:"none"};const report=JSON.parse(readFileSync(path,"utf8")) as {executed:string[];restored:string[]};rmSync(path);return {observation:{executed:report.executed,restored:report.restored,cached:[]},observed:"exact"};}};
};
const workload=(id:string,sessions:ConcurrentSessionV1[],setup:ConcurrentStepV1[]=[]):ConcurrentWorkloadV1=>({id,title:{en:id,de:id},summary:{en:id,de:id},criteria:"default",setup,sessions});
const session=(id:string,group:string,steps:ConcurrentStepV1[],delayMilliseconds?:number):ConcurrentSessionV1=>({id,area:"t",group,steps,...(delayMilliseconds===undefined?{}:{delayMilliseconds})});
const only=(...ids:string[]):ConcurrentCriteriaV1=>({...set.criteria[0]!,rules:set.criteria[0]!.rules.filter(rule=>ids.includes(rule.id))});
const withLimit=(limit:number):ConcurrentCriteriaV1=>({...only("graph-facts-unrelated-wait"),rules:only("graph-facts-unrelated-wait").rules.map(rule=>rule.kind==="max-wait"?{...rule,maximumMilliseconds:limit}:rule)});
const temp=(prefix:string)=>realpathSync(mkdtempSync(join(tmpdir(),prefix)));
const hold=(copy:string,milliseconds:number):SpecV1=>({leases:[{resource:`repository-graph-facts:${copy}`,mode:"exclusive",holdMs:milliseconds}]});

test("agents contending for one lock wait for each other and the harness measures every wait precisely from the lease trace, attributes it to unrelated agents and fails a 500 ms limit but not a generous one",async()=>{
 const base=temp("conc-lease-");
 try{
  const copy=join(base,"copy"),w=world(base,{trace:true,commands:{"run:p:t":hold(copy,2500)}}),controller=new AbortController(),stages:string[]=[];
  const measurement=await concurrentRunWorkloadV1(set,workload("contend",["a","b","c"].map(id=>session(id,"g-"+id,[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}]))),"bazel",portsOf(w,controller,stages));
  const waits=measurement.sessions.flatMap(row=>row.steps.flatMap(step=>step.waits));
  expect(waits.filter(wait=>wait.class==="graph-facts"&&wait.precise&&wait.source==="trace").length).toBeGreaterThanOrEqual(2);
  expect(Math.max(...waits.map(wait=>wait.waitMs))).toBeGreaterThan(2000);
  for(const wait of waits)expect(wait.relation).toBe("unrelated");
  expect(measurement.sessions.map(row=>row.steps[0]!.exit)).toEqual([0,0,0]);expect(measurement.serial!.map(row=>row.steps[0]!.waits.length)).toEqual([0,0,0]);
  expect(concurrentEvaluateV1(withLimit(500),measurement)[0]).toMatchObject({status:"fail",limit:500});
  expect(concurrentEvaluateV1(withLimit(60000),measurement)[0]).toMatchObject({status:"pass"});
  expect(stages).toContain("contend:bazel:concurrent:a:1");expect(stages).toContain("contend:bazel:serial:c:1");expect(stages[0]).toBe("contend:bazel:slice");
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("without a lease trace the progress rows give a lower bound: a long wait still fails, a short one is unproven",async()=>{
 const base=temp("conc-rows-");
 try{
  const copy=join(base,"copy"),w=world(base,{trace:false,commands:{"run:p:t":hold(copy,2500)}}),controller=new AbortController();
  const measurement=await concurrentRunWorkloadV1(set,workload("rows",["a","b"].map(id=>session(id,"g-"+id,[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}]))),"bazel",portsOf(w,controller));
  const waits=measurement.sessions.flatMap(row=>row.steps.flatMap(step=>step.waits));
  expect(waits.length).toBeGreaterThanOrEqual(1);expect(waits.every(wait=>wait.source==="lease"&&!wait.precise&&wait.class==="graph-facts")).toBe(true);expect(Math.max(...waits.map(wait=>wait.waitMs))).toBeGreaterThanOrEqual(1000);
  expect(concurrentEvaluateV1(withLimit(500),measurement)[0]!.status).toBe("fail");
  expect(concurrentEvaluateV1(withLimit(60000),measurement)[0]!.status).toBe("skip");
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("a task that executes in two sessions fails executes-once unless the implementation deduplicates; uncached tasks are excluded",async()=>{
 const base=temp("conc-once-");
 try{
  const task={id:"p:build",inputs:["src/a.ts"],ms:3000},results:Record<string,string>={};
  for(const dedupe of [false,true]){
   const dir=join(base,String(dedupe));mkdirSync(dir);
   const w=world(dir,{dedupe,commands:{"run:p:t":{tasks:[task]}}}),controller=new AbortController();
   const measurement=await concurrentRunWorkloadV1(set,workload("once",["a","b"].map(id=>session(id,"g-same",[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}])),[{kind:"edit",file:"src/a.ts",op:"private-body",language:"ts"}]),"bazel",portsOf(w,controller));
   const executed=measurement.sessions.flatMap(row=>row.steps.flatMap(step=>step.observation?.executed??[]));
   results[String(dedupe)]=concurrentEvaluateV1(only("second-session-zero-actions"),measurement)[0]!.status;
   expect(executed.length,String(dedupe)).toBe(dedupe?1:2);
   expect(readFileSync(join(dir,"copy/src/a.ts"),"utf8")).toBe("export const a=1;\n");
  }
  expect(results).toEqual({false:"fail",true:"pass"});
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("a session whose result differs from the serial run fails serial-equal, a failing command fails no-failure, and every edit is restored byte for byte",async()=>{
 const base=temp("conc-serial-");
 try{
  const w=world(base,{commands:{"show x":{counter:true,stdout:"listing"},"run:p:bad":{exit:3}}}),controller=new AbortController(),original=readFileSync(join(w.copy,"src/b.ts"),"utf8");
  const measurement=await concurrentRunWorkloadV1(set,workload("serial",[session("a","ga",[{kind:"edit",file:"src/b.ts",op:"comment",language:"ts"},{kind:"repo",args:["show","x"],stdout:true},{kind:"run",target:"p:bad"}]),session("b","gb",[{kind:"edit",file:"src/c.ts",op:"public-api",language:"ts"},{kind:"repo",args:["show","x"],stdout:true}])]),"bazel",portsOf(w,controller));
  const results=concurrentEvaluateV1(set.criteria[2]!,measurement);
  expect(results.map(row=>[row.rule,row.status])).toEqual([["all-steps-succeed","fail"],["outputs-equal-serial","fail"]]);
  expect(results[0]!.detail).toEqual(["a#2 exit 3"]);expect(results[1]!.detail.some(line=>line.includes("stdout"))).toBe(true);
  expect(readFileSync(join(w.copy,"src/b.ts"),"utf8")).toBe(original);expect(readFileSync(join(w.copy,"src/c.ts"),"utf8")).toBe("export const c=1;\n");
  expect(measurement.sessions.map(row=>row.steps.length)).toEqual([2,1]);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("barriers line the agents up, a delayed agent delays the others at the barrier, and Bazel's own wait line is measured as a precise Bazel server wait",async()=>{
 const base=temp("conc-barrier-");
 try{
  const w=world(base,{commands:{"run:p:t":{bazelWaitMs:1500}}}),controller=new AbortController();
  const measurement=await concurrentRunWorkloadV1(set,workload("align",[session("a","ga",[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}]),session("b","gb",[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}]),session("c","gc",[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}],1500)]),"bazel",portsOf(w,controller));
  const starts=measurement.sessions.map(row=>row.steps[0]!.startMs);
  expect(Math.min(...starts)).toBeGreaterThanOrEqual(1400);expect(Math.max(...starts)-Math.min(...starts)).toBeLessThan(800);
  const waits=measurement.sessions.flatMap(row=>row.steps.flatMap(step=>step.waits));
  expect(waits.length).toBe(3);for(const wait of waits){expect(wait).toMatchObject({class:"bazel-server",source:"bazel",precise:true,holderPid:4242});expect(wait.waitMs).toBeGreaterThanOrEqual(1400);}
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("a workload whose unrelated agents are not independent is refused before any command runs and leaves every file untouched",async()=>{
 const base=temp("conc-indep-");
 try{
  const task={id:"p:build",inputs:["src/a.ts"],ms:10},w=world(base,{commands:{"run:p:t":{tasks:[task]}}}),controller=new AbortController(),ports=portsOf(w,controller);
  const bad=workload("bad",[session("a","ga",[{kind:"edit",file:"src/a.ts",op:"comment",language:"ts"}]),session("b","gb",[{kind:"run",target:"p:t"}])]);
  const plan=await concurrentPlanV1(bad,"bazel",ports);
  expect(plan.issues).toEqual(["a edits src/a.ts, an input of the targets of b, but ga and gb are declared unrelated"]);expect(plan.targets).toEqual([{target:"p:t",tasks:1,uncached:[]}]);expect(plan.missing).toEqual([]);
  await expect(concurrentRunWorkloadV1(set,bad,"bazel",ports)).rejects.toThrow("Die Arbeitslast bad ist nicht unabhängig");
  expect(readFileSync(join(w.copy,"src/a.ts"),"utf8")).toBe("export const a=1;\n");expect(existsSync(join(w.copy,"calls.log"))).toBe(false);
  const missing=await concurrentPlanV1(workload("m",[session("a","ga",[{kind:"edit",file:"src/none.ts",op:"comment",language:"ts"}])]),"bazel",ports);
  expect(missing.missing).toEqual(["src/none.ts"]);
  expect(renderConcurrentPlanV1(set,[{workload:bad,tool:"bazel",plan}],"de")).toContain("abgelehnt");expect(renderConcurrentPlanV1(set,[{workload:bad,tool:"bazel",plan}],"en")).toContain("refused");
  await expect(concurrentRunWorkloadV1(set,workload("m",[session("a","ga",[{kind:"edit",file:"src/none.ts",op:"comment",language:"ts"}])]),"bazel",ports)).rejects.toThrow("fehlen in der Kopie");
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

test("cancelling a run terminates the running commands, releases their locks and restores every edit",async()=>{
 const base=temp("conc-cancel-");
 try{
  const copy=join(base,"copy"),w=world(base,{commands:{"run:p:t":hold(copy,60000)}}),controller=new AbortController(),original=readFileSync(join(copy,"src/a.ts"),"utf8");
  const running=concurrentRunWorkloadV1(set,workload("cancel",[session("a","ga",[{kind:"edit",file:"src/a.ts",op:"comment",language:"ts"},{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}]),session("b","gb",[{kind:"barrier",id:"go"},{kind:"run",target:"p:t"}])]),"bazel",portsOf(w,controller));
  const outcome=running.then(()=>"finished",error=>(error as Error).message);
  await new Promise(done=>setTimeout(done,4000));
  expect(readFileSync(join(copy,"src/a.ts"),"utf8")).not.toBe(original);
  controller.abort(Error("cancelled by the test"));
  expect(await outcome).toContain("cancelled by the test");
  expect(readFileSync(join(copy,"src/a.ts"),"utf8")).toBe(original);expect(existsSync(join(copy,"calls.log"))).toBe(false);
  const lease=await import(join(leaseRoot,leasePath)) as {acquireResourceLease(options:{directory:string;resource:string;mode:"exclusive";signal:AbortSignal;timeoutMs:number}):Promise<{release():void}>};
  const taken=await lease.acquireResourceLease({directory:join(copy,"leases"),resource:`repository-graph-facts:${copy}`,mode:"exclusive",signal:new AbortController().signal,timeoutMs:5000});taken.release();
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("the real spawner timestamps stderr lines, digests stdout, reports the exit code and terminates the process tree on abort",async()=>{
 const base=temp("conc-spawn-");
 try{
  const env={PATH:process.env.PATH??"/usr/bin"},keepAll=()=>true,now=()=>Date.now(),signal=new AbortController().signal;
  const done=await concurrentSpawnV1(process.execPath,["-e","process.stderr.write('one\\n');setTimeout(()=>{process.stderr.write('two\\r\\nthree');process.stdout.write('out');process.exit(4);},150);"],base,env,keepAll,signal,now);
  expect(done.code).toBe(4);expect(done.lines.map(row=>row.line)).toEqual(["one","two","three"]);expect(done.lines[1]!.atMs-done.lines[0]!.atMs).toBeGreaterThanOrEqual(100);
  expect(done.stdout).toBe(createHash("sha256").update("out").digest("hex"));
  const filtered=await concurrentSpawnV1(process.execPath,["-e","console.error('keep');console.error('drop');"],base,env,line=>line==="keep",signal,now);expect(filtered.lines.map(row=>row.line)).toEqual(["keep"]);
  const controller=new AbortController(),child=concurrentSpawnV1(process.execPath,["-e","setInterval(()=>{},1000);"],base,env,keepAll,controller.signal,now);
  setTimeout(()=>controller.abort(Error("stop it")),200);await expect(child).rejects.toThrow("stop it");
  await expect(concurrentSpawnV1(process.execPath,["-e","0"],base,env,keepAll,controller.signal,now)).rejects.toThrow("stop it");
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

const varint=(value:number):number[]=>{const out:number[]=[];let rest=value;while(rest>=128){out.push(rest%128|128);rest=Math.floor(rest/128);}out.push(rest);return out;};
const field=(number:number,text:string):number[]=>[number*8+2,...varint(Buffer.byteLength(text)),...Buffer.from(text)];
const compact=(spawns:{label:string;hit:boolean}[]):Uint8Array=>{const entries=spawns.flatMap(spawn=>{const body=[...field(7,spawn.label),...field(8,"Genrule"),...field(11,"local"),96,spawn.hit?1:0],entry=[58,...varint(body.length),...body];return [...varint(entry.length),...entry];});return (Bun as unknown as {zstdCompressSync(input:Uint8Array):Uint8Array}).zstdCompressSync(Uint8Array.from(entries));};
const tasks=(ids:string[])=>Object.fromEntries(ids.map(id=>[id,{family:"bun-script",cache:true,outputs:true,dependsOn:[],consumes:[],inputs:[]}]));

test("the real ports read and write only inside the copy, digest files and directories, name a task report for Bazel and a trace for every command, and read the reports back",async()=>{
 const base=temp("conc-ports-");
 try{
  const copy=join(base,"copy"),scratch=join(base,"scratch"),calls:string[][]=[];mkdirSync(join(copy,"dir/sub"),{recursive:true});mkdirSync(scratch);
  writeFileSync(join(copy,"f.txt"),"one");writeFileSync(join(copy,"dir/a.txt"),"a");writeFileSync(join(copy,"dir/sub/b.txt"),"b");
  let extra="";
  const host:ConcurrentHostV1={live:join(base,"live"),root:copy,script:"s.ts",executable:"bun",scratch,signal:new AbortController().signal,environment:{PATH:"/bin",SEMIO_X:"1",SEMIO_LANGUAGE:"de"},
   run:async(command,args)=>{calls.push([command,...args]);const rest=args.slice(args.indexOf("repo")+1).filter(argument=>!argument.startsWith("--tool="));if(rest[0]==="graph"){const file=rest[1]!.slice(7);writeFileSync(file,"{}");return file+"\n";}return JSON.stringify({tasks:{roots:[],tasks:{"lib:gen":{id:"lib:gen"},"lib:extra":{id:"lib:extra"}},dependencies:{"lib:gen":[],"lib:extra":[]}}},null,2);},
   nxRecord:directory=>join(directory,"run.json"),adapter:async()=>({...stubAdapter,planBazelTasksV1:()=>({selected:["lib:gen"],tasks:{"lib:gen":{id:"lib:gen",dependencies:[],definition:{outputs:["o"]}},"lib:extra":{id:"lib:extra",dependencies:["lib:gen"],definition:{outputs:["o"]}}}}),bazelTaskInputsV1:()=>["src/a.ts"]}) as never,inventory:async()=>["src/a.ts","other.ts"],now:()=>1700000000000,check:()=>{},progress:async()=>{},spawn:async(tool,args,environment)=>{extra=JSON.stringify([tool,args,environment]);return {code:0,lines:[],stdout:"",startMs:1,endMs:2};}};
  const ports=concurrentCliPortsV1(host,host.signal);
  expect(await ports.read("f.txt")).toBe("one");await ports.write("f.txt","two");expect(readFileSync(join(copy,"f.txt"),"utf8")).toBe("two");
  await expect(ports.write("../escape.txt","x")).rejects.toThrow("repository-relative path");expect(existsSync(join(base,"escape.txt"))).toBe(false);
  expect(await ports.digest(["f.txt"])).toBe(concurrentDigestV1([{path:"f.txt",bytes:"two"}]));
  expect(await ports.digest(["dir"])).toBe(concurrentDigestV1([{path:"dir/a.txt",bytes:"a"},{path:"dir/sub/b.txt",bytes:"b"}]));
  expect(await ports.digest(["nothing"])).toBe(concurrentDigestV1([{path:"nothing",bytes:"\0missing"}]));expect(await ports.digest([])).toBeNull();
  expect(ports.reportPath("bazel","a",2,"concurrent")!.startsWith(scratch)).toBe(true);expect(ports.reportPath("nx","a",2,"concurrent")).toBeUndefined();
  const trace=ports.tracePath("a",2,"concurrent");expect(trace.startsWith(scratch)).toBe(true);expect(trace).not.toBe(ports.tracePath("a",2,"concurrent"));
  expect(await ports.readTrace(trace)).toBe("");writeFileSync(trace,"row\n");expect(await ports.readTrace(trace)).toBe("row\n");expect(existsSync(trace)).toBe(false);
  const slice=await ports.slice("bazel","lib:gen",["src/a.ts","absent.ts"]);
  expect(Object.keys(slice.tasks).sort()).toEqual(["lib:extra","lib:gen"]);expect(slice.tasks["lib:gen"]!.inputs).toEqual(["src/a.ts"]);
  expect(calls[0]).toEqual(["bun",join(copy,"s.ts"),"repo","--tool=bazel","graph","--file="+join(scratch,"graph-bazel.json")]);expect(calls[1]!.slice(-3)).toEqual(["run","lib:gen","--dry-run"]);
  await ports.slice("bazel","lib:gen",[]);expect(calls.filter(call=>call.includes("graph")).length).toBe(1);
  await ports.spawn("nx",["run","a:b"],{SEMIO_LEASE_TRACE:"/t"},()=>true,host.signal);expect(JSON.parse(extra)).toEqual(["nx",["run","a:b"],{SEMIO_LEASE_TRACE:"/t"}]);
  const ids=["a:b","c:d","e:f"],ghost={...host,adapter:async()=>({...stubAdapter,planBazelTasksV1:()=>({selected:[],tasks:{"lib:gen":{id:"lib:gen",dependencies:[],definition:{outputs:["o"]}}}})}) as never};
  await expect(concurrentCliPortsV1(ghost as ConcurrentHostV1,host.signal).slice("nx","lib:gen",[])).rejects.toThrow("disagree about the closure of lib:gen: lib:extra");
  const bazelSlice={version:1,selected:[],tasks:tasks(ids)} as unknown as IncrementalSliceV1,report=join(scratch,"log.zst");
  if(typeof (Bun as unknown as {zstdCompressSync?:unknown}).zstdCompressSync==="function"){
   writeFileSync(report,compact([{label:`@@//:${incrementalLabelV1("t","a:b")}`,hit:false},{label:`//:${incrementalLabelV1("t","c:d")}`,hit:true},{label:"//:support",hit:false}]));
   expect(await ports.observe("bazel",bazelSlice,report,{startMs:0,endMs:1},true)).toEqual({observation:{executed:["a:b"],restored:["c:d"],cached:["e:f"]},observed:"exact"});expect(existsSync(report)).toBe(false);
  }
  writeFileSync(join(copy,"run.json"),JSON.stringify({run:{startTime:"1970-01-01T00:00:01.200Z",endTime:"1970-01-01T00:00:01.800Z"},tasks:[{taskId:"a:b",cacheStatus:"cache-miss",status:0},{taskId:"c:d",cacheStatus:"local-cache-hit",status:0}]}));
  expect(await ports.observe("nx",bazelSlice,undefined,{startMs:1000,endMs:2000},true)).toEqual({observation:{executed:["a:b"],restored:["c:d"],cached:[]},observed:"window"});
  expect(await ports.observe("nx",bazelSlice,undefined,{startMs:9000,endMs:9900},true)).toMatchObject({observation:null,observed:"ambiguous"});
  expect(ports.nonce()).toMatch(/^[0-9a-z]{6,32}$/u);expect(ports.nonce()).not.toBe(ports.nonce());
 }finally{rmSync(base,{recursive:true,force:true});}
});

const installedNx=(()=>{const path=join(root,"node_modules/nx/dist/src/tasks-runner/life-cycles/store-run-information-life-cycle.js");return existsSync(path)?realpathSync(path):undefined;})();
(installedNx?test:test.skip)("the installed Nx writes a run record whose own start and end the harness accepts inside its window and refuses outside it",()=>{
 const directory=temp("conc-nx-");
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source=`const {StoreRunInformationLifeCycle}=require(process.env.NX_LIFE);const out=[];const lc=new StoreRunInformationLifeCycle("nx run-many",d=>out.push(d));
const t=(id)=>({id,hash:"h",target:{project:id.split(":")[0],target:id.split(":")[1]},startTime:1,endTime:2});
lc.startCommand();lc.startTasks([t("p:build"),t("q:build")]);lc.endTasks([{task:t("p:build"),status:"success",code:0},{task:t("q:build"),status:"local-cache",code:0}]);lc.endCommand();process.stdout.write(JSON.stringify(out[0]));`;
  const before=Date.now(),run=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",env:{...process.env,NX_LIFE:installedNx!,NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv}),after=Date.now();
  expect(run.status,run.stderr).toBe(0);
  const record=JSON.parse(run.stdout);
  expect(plain(concurrentNxFitV1(record,["p:build","q:build"],{startMs:before,endMs:after},true))).toEqual({observation:{executed:["p:build"],restored:["q:build"],cached:[]},observed:"window"});
  expect(concurrentNxFitV1(record,["p:build","q:build"],{startMs:after+60000,endMs:after+61000},true).observed).toBe("ambiguous");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const stubAdapter={
 planBazelTasksV1:()=>({selected:["lib:gen"],tasks:{"lib:gen":{id:"lib:gen",dependencies:[],definition:{outputs:["o"]}}}}),
 bazelCacheDecisionV1:()=>({cache:true}),bazelTaskCoverageV1:()=>({family:"bun-script"}),
 bazelSourceIndexV1:(inventory:readonly string[])=>({files:[...inventory].sort(),selectAll:(inputs:readonly unknown[])=>({marks:Uint8Array.from([...inventory].sort().map(file=>inputs.includes(file)?1:0))})}),bazelTaskInputsV1:()=>[]
};
const commandPorts=(base:string,copy:string,out:string[],stages:string[],reports:unknown[],over:Partial<IncrementalCommandPortsV1>={}):IncrementalCommandPortsV1=>({
 write:line=>out.push(line),progress:async stage=>{stages.push(stage);},check:()=>{},signal:new AbortController().signal,environment:{SEMIO_LANGUAGE:"en",PATH:process.env.PATH??"/usr/bin",HOME:process.env.HOME??tmpdir()},platform:"linux",now:()=>Date.now(),
 run:async(_command,args)=>{const rest=args.slice(args.indexOf("repo")+1).filter(argument=>!argument.startsWith("--tool="));if(rest[0]==="graph"){const file=rest[1]!.slice(7);writeFileSync(file,"{}");return file+"\n";}return JSON.stringify({tasks:{roots:[],tasks:{"lib:gen":{id:"lib:gen"}},dependencies:{"lib:gen":[]}}},null,2);},
 report:async(file,value)=>{reports.push([file,value]);},files:async()=>[script],adapter:async()=>stubAdapter as never,nxRecord:directory=>join(directory,"run.json"),exists:async path=>existsSync(path),scratch:async()=>{const directory=join(base,"scratch-run");mkdirSync(directory,{recursive:true});return directory;},...over});

test("the command lists, plans and runs the authored workloads through its ports with progress, bilingual output, a report file and a failing exit",async()=>{
 const base=temp("conc-cmd-");
 try{
  const w=world(base,{commands:{}},{}),files:Record<string,string>={};
  for(const row of set.workloads)for(const owner of [{id:"setup",steps:row.setup},...row.sessions])for(const step of owner.steps)if(step.kind==="edit"&&!existsSync(join(w.copy,step.file))){mkdirSync(dirname(join(w.copy,step.file)),{recursive:true});writeFileSync(join(w.copy,step.file),"// source\n");files[step.file]="x";}
  const out:string[]=[],stages:string[]=[],reports:unknown[]=[],ports=(over:Partial<IncrementalCommandPortsV1>={})=>commandPorts(base,w.copy,out,stages,reports,over);
  await runConcurrencyCommandV1(root,["concurrency","list","--lang=de"],ports());
  expect(out.at(-1)).toContain(set.workloads[0]!.title.de);expect(out.at(-1)).toContain("ts[g-ts]");
  out.length=0;await runConcurrencyCommandV1(root,["list","--json"],ports());expect(JSON.parse(out[0]!)[0]).toMatchObject({id:"disjoint-projects",sessions:[{id:"ts",group:"g-ts"},{id:"rust",group:"g-rust"},{id:"go",group:"g-go"},{id:"schema",group:"g-schema"}]});
  await expect(runConcurrencyCommandV1(root,["concurrency","run","--root="+join(base,"none")],ports())).rejects.toThrow("Prepare the disposable root first");
  await expect(runConcurrencyCommandV1(root,["concurrency","run","--workloads=nope","--root="+w.copy],ports())).rejects.toThrow("Unknown workload: nope");
  await expect(runConcurrencyCommandV1(root,["concurrency","run","--criteria=nope","--root="+w.copy],ports())).rejects.toThrow("Unknown criteria: nope");
  out.length=0;stages.length=0;
  await runConcurrencyCommandV1(root,["concurrency","plan","--workloads=disjoint-projects","--implementations=bazel","--sessions=2","--root="+w.copy],ports());
  expect(out[0]).toContain("disjoint-projects / bazel: sessions ts[g-ts] rust[g-rust]");expect(out[0]).toContain("independent");expect(stages).toEqual(["disjoint-projects:bazel:plan"]);
  out.length=0;await runConcurrencyCommandV1(root,["concurrency","plan","--workloads=same-target-cold","--implementations=nx","--sessions=1","--root="+w.copy,"--json"],ports());expect(JSON.parse(out[0]!)[0]).toMatchObject({workload:"same-target-cold",tool:"nx",targets:[{target:"@semio-tech/framework-graph:test",tasks:1}],issues:[]});
  out.length=0;stages.length=0;
  await runConcurrencyCommandV1(root,["concurrency","run","--workloads=readers-while-edit","--implementations=bazel","--root="+w.copy,"--file=report.json","--criteria=smoke"],ports());
  expect(reports.length).toBe(1);expect(out.join("\n")).toContain("readers-while-edit / bazel");expect(out.join("\n")).toContain("pass criteria");expect(stages).toContain("readers-while-edit:bazel:concurrent:reader-a:1");
  expect(readFileSync(join(w.copy,(set.workloads[3]!.sessions[0]!.steps[0] as {file:string}).file),"utf8")).toBe("// source\n");
  writeFileSync(join(w.copy,"stub.json"),JSON.stringify({live:leaseRoot,lease:leasePath,dedupe:true,commands:{},default:{exit:2}}));
  out.length=0;
  await expect(runConcurrencyCommandV1(root,["concurrency","run","--workloads=readers-while-edit","--implementations=bazel","--root="+w.copy,"--lang=de"],ports())).rejects.toThrow("Konformität der Gleichzeitigkeit verletzt in");
  expect(out.join("\n")).toContain("Abnahmekriterien");expect(out.join("\n")).toContain("conc-graph: fehlgeschlagen");
  const cancelled=ports({check:()=>{throw Error("cancelled");}});
  await expect(runConcurrencyCommandV1(root,["concurrency","plan","--root="+w.copy],cancelled)).rejects.toThrow("cancelled");
 }finally{rmSync(base,{recursive:true,force:true});}
},180000);

test("the command arguments are closed, bilingual in their errors and default to both implementations, every workload and three sessions",()=>{
 expect(concurrencyArgumentsV1(["list"])).toMatchObject({operation:"list",workloads:"all",sessions:undefined,criteria:undefined,tools:["nx","bazel"],json:false});
 expect(concurrencyArgumentsV1(["run","--workloads=a,b","--sessions=4","--criteria=smoke","--implementations","bazel","--json","--root=/tmp/x","--file=r.json","--lang=de"])).toMatchObject({operation:"run",workloads:["a","b"],sessions:4,criteria:"smoke",tools:["bazel"],json:true,root:"/tmp/x",file:"r.json",language:"de"});
 expect(concurrencyArgumentsV1(["plan","--workloads=all"]).workloads).toBe("all");
 for(const[bad,message]of [[[],"Expected repo concurrency"],[["go"],"Expected repo concurrency"],[["run","extra"],"Expected repo concurrency"],[["run","--wat"],"Unknown concurrency option"],[["run","--json","--json"],"Repeated option"],[["run","--workloads"],"needs a value"],[["run","--sessions=0"],"Sessions must be a whole number"],[["run","--sessions=9"],"Sessions must be a whole number"],[["run","--sessions=1.5"],"Sessions must be a whole number"],[["run","--implementations=make"],"Unknown implementation"],[["run","--lang=fr"],"Unknown language"],[["run","--json=1"],"Unknown concurrency option"]] as const)
  expect(()=>concurrencyArgumentsV1(bad as never),JSON.stringify(bad)).toThrow(message);
 expect(()=>concurrencyArgumentsV1(["go"])).toThrow("Erwartet: repo concurrency");
});

test("the catalogue declares the concurrency operation with its subcommands, flags, bilingual texts and no dependency requirement, matching the parser",()=>{
 const row=catalog.operations.find(entry=>entry.name==="concurrency") as unknown as {name:string;requires:string;implementations:Record<string,string>;progress:boolean;cancellable:boolean;subcommands:{name:string;summary:{en:string;de:string}}[];flags:string[];summary:{en:string;de:string};examples:{command:string;description:{en:string;de:string}}[]};
 expect(row).toBeDefined();expect(row.requires).toBe("none");expect(row.implementations).toEqual({nx:"neutral",bazel:"neutral"});expect(row.progress&&row.cancellable).toBe(true);
 expect(row.subcommands.map(sub=>sub.name)).toEqual(["list","plan","run"]);
 const names=concurrencyOptionNamesV1(),accepted=new Set([...names.valued,...names.plain].filter(name=>name!=="--lang"));
 expect(new Set(row.flags)).toEqual(accepted);
 for(const name of accepted){const flag=catalog.flags.find(entry=>entry.name===name) as unknown as {value:string;operations:string[]};expect(flag,name).toBeDefined();expect(flag.operations,name).toContain("concurrency");expect(flag.value,name).toBe(names.plain.includes(name)?"none":"required");}
 for(const text of [row.summary,...row.examples.map(example=>example.description),...row.subcommands.map(sub=>sub.summary)]){expect(text.en).not.toBe("");expect(text.de).not.toBe(text.en);}
 for(const example of row.examples)expect(example.command.startsWith("repo concurrency")).toBe(true);
 expect(schema.definitions.Policy.const.leaseTrace).toBe("SEMIO_LEASE_TRACE");
});

const tracing=readFileSync(join(leaseRoot,leasePath),"utf8").includes("SEMIO_LEASE_TRACE")?test:test.skip;
tracing("the lease module appends one trace row for every acquisition that waited, queued or not, and none without the environment variable",async()=>{
 const base=temp("conc-trace-");
 try{
  const directory=join(base,"leases"),trace=join(base,"trace.jsonl"),url=JSON.stringify(join(leaseRoot,leasePath));
  const program=(call:string,hold:number)=>`const lease=await import(${url});const taken=await lease.${call}({directory:${JSON.stringify(directory)},resource:"r-trace",mode:"exclusive",owner:"t",signal:new AbortController().signal,onWait:()=>{}});await Bun.sleep(${hold});taken.release();`;
  const env={PATH:process.env.PATH??"/usr/bin",HOME:process.env.HOME??tmpdir()},signal=new AbortController().signal,now=()=>Date.now(),run=(code:string,extra:Record<string,string>)=>concurrentSpawnV1(process.execPath,["-e",code],base,{...env,...extra},()=>false,signal,now);
  for(const call of ["acquireResourceLease","acquireQueuedResourceLease"]){
   rmSync(trace,{force:true});
   const first=run(program(call,1500),{SEMIO_LEASE_TRACE:trace});await new Promise(done=>setTimeout(done,700));
   const second=await run(program(call,10),{SEMIO_LEASE_TRACE:trace});await first;
   const rows=readFileSync(trace,"utf8").trim().split("\n").map(line=>JSON.parse(line));
   expect(rows.length,call).toBeGreaterThanOrEqual(1);for(const row of rows)expect(define("TraceRow")(row),JSON.stringify(row)).toBe(true);
   const waited=rows.filter(row=>row.acquiredMs-row.startedMs>=300);expect(waited.length,call).toBeGreaterThanOrEqual(1);expect(waited[0]).toMatchObject({version:1,resource:"r-trace",mode:"exclusive"});expect(second.code).toBe(0);
   const parsed=concurrentTraceWaitsV1(readFileSync(trace,"utf8"),0,set.classes);expect(parsed.length).toBeLessThanOrEqual(rows.length);expect(Math.max(...parsed.map(wait=>wait.waitMs))).toBeGreaterThanOrEqual(300);
   rmSync(trace,{force:true});await run(program(call,10),{});expect(existsSync(trace)).toBe(false);
  }
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary&&existsSync(binary)?test:test.skip;
real("two real Bazel builds in one output base: the second waits on the server lock and the harness measures the wait as Bazel reports it",async()=>{
 const base=temp("conc-bazel-"),work=join(base,"ws"),flags=[`--output_user_root=${join(base,"root")}`,`--output_base=${join(base,"ob")}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=30","--host_jvm_args=-Xmx512m"];
 mkdirSync(work);writeFileSync(join(work,"MODULE.bazel"),'module(name = "conc_probe", version = "0.0.0")\n');
 writeFileSync(join(work,"BUILD.bazel"),'genrule(name = "slow", outs = ["slow.out"], cmd = "sleep 8; echo slow > $@")\ngenrule(name = "quick", outs = ["quick.out"], cmd = "echo quick > $@")\n');
 const env={PATH:process.env.PATH??"/usr/bin",HOME:process.env.HOME??tmpdir(),LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"},signal=new AbortController().signal,now=()=>Date.now();
 const build=(target:string)=>concurrentSpawnV1(binary!,[...flags,"build","--lockfile_mode=off","--incompatible_autoload_externally=","--noshow_progress","--",target],work,env,concurrentLineKeeperV1(),signal,now);
 try{
  expect((await concurrentSpawnV1(binary!,[...flags,"info","--lockfile_mode=off","server_pid"],work,env,()=>false,signal,now)).code).toBe(0);
  const first=build("//:slow");await new Promise(done=>setTimeout(done,2500));const origin=Date.now(),second=await build("//:quick"),firstDone=await first;
  expect(firstDone.code).toBe(0);expect(second.code).toBe(0);
  const waits=concurrentWaitsV1(second.lines.map(row=>({atMs:row.atMs-origin,line:row.line})),second.endMs-origin,set.classes);
  expect(waits.length).toBe(1);expect(waits[0]).toMatchObject({class:"bazel-server",source:"bazel",precise:true});expect(waits[0]!.waitMs).toBeGreaterThan(3000);expect(waits[0]!.waitMs).toBeLessThan(second.endMs-origin+1);
 }finally{spawnSync(binary!,[...flags,"shutdown"],{cwd:work});spawnSync("chmod",["-R","u+w",base]);rmSync(base,{recursive:true,force:true});}
},300000);
