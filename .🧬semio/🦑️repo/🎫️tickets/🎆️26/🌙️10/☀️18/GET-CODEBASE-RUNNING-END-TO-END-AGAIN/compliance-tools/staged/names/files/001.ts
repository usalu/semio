import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,statSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {closedPoolEnvironmentV1,emptyPoolV1,jobserverChildEnvironmentV1,jobserverClosedMessageV1,jobserverEndpointV1,jobserverPlanV1,jobserverStdioV1,JOBSERVER_VERIFIED_VARIABLE_V1 as census} from "../🟦️.ts";
import {buildJobserverProbeV1,type ProbeVerdictV1} from "./🦀️probe/🟦️.ts";
import {cargoScratchBaseV1} from "../../🧪️tests/🏖️scratch/🟦️.ts";

/** 🏁️ The grammar of a job-token pool, three ways: a language-agnostic fixture checked against the module, against an independent Python oracle written from the GNU make manual, and against the code that actually decides in production, the `jobserver` crate (Cargo, rustc and `cc` call it), driven on real pipes and fifos. */
const here=fileURLToPath(new URL(".",import.meta.url)),python=testPythonV1()??"python3",cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const base=cargoScratchBaseV1();
const scratch=()=>mkdtempSync(join(base,"jg-"));

test("the fixture is closed, and the module and the independent Python oracle agree with it on every pool and every closed-pool environment",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);
 const validate=ajv.getSchema(`${schema.$id}#/definitions/Fixture`)!;expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const row of fixture.cases)expect(jobserverEndpointV1(row.environment,row.platform as NodeJS.Platform,path=>(row.fifos as string[]).includes(path)),row.name).toEqual(row.expect as never);
 for(const row of fixture.closedPools)expect(closedPoolEnvironmentV1(row.environment,row.empty??undefined),row.name).toEqual(row.patch as unknown as Record<string,string>);
 const run=spawnSync(python,["-I",join(here,"🐍️oracle/🐍️.py")],{input:JSON.stringify(fixture),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {endpoints:unknown[];patches:unknown[]};
 expect(oracle.endpoints).toEqual(fixture.cases.map(row=>row.expect));expect(oracle.patches).toEqual(fixture.closedPools.map(row=>row.patch));
});

test("the jobserver crate, driven on real pipes and fifos, reaches the same verdict (a pool it can use, no pool, or a pool it cannot reach) for every POSIX pool the fixture names",()=>{
 const directory=scratch(),probe=buildJobserverProbeV1(join(directory,"probe"),cargo);let checked=0;
 const theirs=(verdict:ProbeVerdictV1)=>verdict.pool.client?"usable":["NoEnvVar","NoJobserver"].includes(verdict.pool.kind)?"none":"closed";
 const ours=(kind:string)=>["pipe","fifo","semaphore"].includes(kind)?"usable":kind;
 for(const [index,row] of fixture.cases.entries()){
  if(row.platform==="win32"||("stricter" in row&&row.stricter))continue;
  const root=join(directory,"case-"+index);mkdirSync(root,{recursive:true});
  const run=spawnSync(python,["-I",join(here,"🧫️fixtures/🪄️realise.py"),probe,root],{input:JSON.stringify(row),encoding:"utf8"});
  expect(run.status,row.name+": "+run.stderr).toBe(0);
  const verdict=JSON.parse(run.stdout) as ProbeVerdictV1;checked++;
  expect(ours(row.expect.kind),row.name+" / "+run.stdout.trim()).toBe(theirs(verdict));
  if(row.expect.kind==="closed")expect(verdict.variable,row.name).toBe((row.expect as {variable:string}).variable);
 }
 expect(checked).toBeGreaterThan(25);
},300000);

test("a plan hands an open pipe on under its own numbers, leaves a fifo, a semaphore and no pool alone, and replaces an unreachable pool by an empty one; a child never inherits the stand-in's census",()=>{
 const pipe=jobserverPlanV1({CARGO_MAKEFLAGS:"-j --jobserver-auth=3,5",[census]:"3,5",PATH:"/bin"},{platform:"linux"});
 expect(pipe.descriptors).toEqual([3,5]);expect(pipe.environment).toEqual({});expect(jobserverClosedMessageV1(pipe)).toBe("");
 expect(jobserverChildEnvironmentV1({PATH:"/bin",[census]:"3,5",GONE:undefined},pipe)).toEqual({PATH:"/bin"});
 expect(jobserverStdioV1([3,5],true)).toEqual(["inherit","pipe","pipe",3,"ignore",5]);
 expect(jobserverStdioV1([7,8],false)).toEqual(["inherit","inherit","inherit","ignore","ignore","ignore","ignore",7,8]);
 expect(jobserverStdioV1([],true)).toEqual(["inherit","pipe","pipe"]);
 for(const [environment,platform] of[[{MAKEFLAGS:"-j4 --jobserver-auth=fifo:/p"},"linux"],[{MAKEFLAGS:"--jobserver-auth=semaphore-name"},"win32"],[{},"linux"],[{MAKEFLAGS:"-j4"},"win32"]] as const){
  const plan=jobserverPlanV1(environment,{platform,isFifo:()=>true});expect(plan.descriptors,JSON.stringify(environment)).toEqual([]);expect(plan.environment).toEqual({});
 }
 const closed=jobserverPlanV1({CARGO_MAKEFLAGS:"-j --jobserver-auth=3,5",SEMIO_UNIT_CACHE_ROOT:"/cache"},{platform:"linux",empty:root=>root+"/bin/jobserver-empty"});
 expect(closed.descriptors).toEqual([]);expect(closed.environment).toEqual({NUM_JOBS:"1",CARGO_MAKEFLAGS:"-j1 --jobserver-fds=fifo:/cache/bin/jobserver-empty --jobserver-auth=fifo:/cache/bin/jobserver-empty"});
 const message=jobserverClosedMessageV1(closed);expect(message).toContain("job-token pool");expect(message).toContain("Job-Token-Pool");expect(message).toContain("CARGO_MAKEFLAGS");expect(message).not.toMatch(/\{\w+\}/u);
 const windows=jobserverPlanV1({CARGO_MAKEFLAGS:"-j --jobserver-auth=__rust_jobserver_semaphore_1"},{platform:"win32"});expect(windows.endpoint).toEqual({kind:"semaphore",name:"__rust_jobserver_semaphore_1"});expect(windows.descriptors).toEqual([]);
});

test("the empty pool is one named pipe in the cache's bin directory, made once, and there is none where no pipe can be made",()=>{
 const root=join(scratch(),"cache"),first=emptyPoolV1(root);
 expect(first).toBe(join(root,"bin","jobserver-empty"));expect(statSync(first!).isFIFO()).toBe(true);expect(emptyPoolV1(root)).toBe(first);
 expect(emptyPoolV1(root,"win32")).toBeUndefined();expect(emptyPoolV1(undefined)).toBeUndefined();
});
