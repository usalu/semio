/** 🧱️ Contract: the Bazel rendering of a host capacity grant gives the flags, the granted worker host, the per kind resource sets and the local concurrency that the corpus in `./🔣️.json` states and that an independent Python closed form, a minimum and an execution of the generated Starlark as Python confirm. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {bazelGrantedWorkerHostV1,bazelHostPlanV1,bazelHostResourceSettingsV1,bazelLocalConcurrencyV1,bazelResourceSetsV1} from "@UNIT/🟦️.ts";
import {bazelWorkerMnemonicV1} from "@LIB/🏗️build/🔧️tools/🏗️bazel/👷️workers/🟦️.ts";
import {WORKER_ACTION_KINDS_V1} from "@LIB/🏗️build/👷️workers/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,concurrency:[[1,2,3]]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,settings:[{...corpus.settings[0],flags:["--jobs=2","--local_resources=memory=1"]}]})).toBe(false);
});

test("the flags and the honesty mapping follow the grant field by field",()=>{
 for(const row of corpus.settings){
  const settings=bazelHostResourceSettingsV1(row.plan as never);
  expect([...settings.flags],row.name).toEqual(row.flags);
  expect(settings.mapping.map(item=>item.field),row.name).toEqual(["jobs","cpu","memoryMegabytes","fixedMegabytes","floorMegabytes"]);
  expect(settings.mapping.map(item=>item.support),row.name).toEqual(["exact","exact","exact","unsupported","unsupported"]);
  expect(settings.mapping[0]!.setting).toBe("--jobs="+row.plan.jobs+" (worker plan)");expect(settings.mapping[1]!.setting).toBe(row.flags[0]);expect(settings.mapping[2]!.setting).toBe(row.flags[1]);
  expect(Object.isFrozen(settings)).toBe(true);expect(Object.isFrozen(settings.flags)).toBe(true);expect(Object.isFrozen(settings.mapping)).toBe(true);
 }
 expect(bazelHostPlanV1({})).toBeUndefined();
});

test("the worker pools never see more than the grant, equal to the Python minimum",()=>{
 const answers=ask({op:"host",rows:corpus.hosts});
 for(const[index,row]of corpus.hosts.entries()){
  const actual=bazelGrantedWorkerHostV1((row.plan??undefined) as never,row.host);expect(actual,row.name).toEqual(answers[index]);
  if(row.plan===null)expect(actual,row.name).toBe(row.host);else{expect(actual.cores).toBeLessThanOrEqual(row.host.cores);expect(actual.memoryBytes).toBeLessThanOrEqual(row.host.memoryBytes);}
 }
});

test("the generated Starlark runs as Python and books every action kind at its estimate under its mnemonic",()=>{
 const texts=corpus.kinds.map(row=>bazelResourceSetsV1(row.kinds as never)),answers=ask({op:"resourceSets",texts}),kinds=WORKER_ACTION_KINDS_V1.filter(kind=>kind!=="service");
 for(const[index,row]of corpus.kinds.entries()){
  expect(Object.keys(answers[index]).sort(),row.name).toEqual(kinds.map(kind=>bazelWorkerMnemonicV1(kind)).sort());
  for(const kind of kinds)expect(answers[index][bazelWorkerMnemonicV1(kind)],row.name+" "+kind).toEqual({cpu:(row.kinds as any)[kind].cpu,memory:(row.kinds as any)[kind].megabytes});
  expect(texts[index]!.includes("service"),row.name).toBe(false);expect(texts[index]!.endsWith("\n\n"),row.name).toBe(true);
 }
});

test("Bazel's local resource manager equals the closed form over the whole grid of pools, actions and counts",()=>{
 const answers=ask({op:"concurrency",rows:corpus.concurrency});let capped=0,idle=0,bounded=0;
 for(const[index,row]of corpus.concurrency.entries()){
  const[jobs,cpu,memoryMegabytes,actionCpu,actionMegabytes,count]=row as [number,number,number,number,number,number],actual=bazelLocalConcurrencyV1({jobs,cpu,memoryMegabytes},{cpu:actionCpu,memoryMegabytes:actionMegabytes},count);
  expect(actual,JSON.stringify(row)).toBe(answers[index]);expect(actual).toBeLessThanOrEqual(Math.min(count,jobs));
  if(count>=1&&jobs>=1)expect(actual,"an idle pool always starts one action").toBeGreaterThanOrEqual(1);
  if(actual===jobs)capped++;if(actual===0)idle++;if(actual>1&&actual<Math.min(jobs,count))bounded++;
 }
 expect(capped).toBeGreaterThan(50);expect(idle).toBeGreaterThan(50);expect(bounded).toBeGreaterThan(50);
});
