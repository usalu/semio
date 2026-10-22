/** 👷️ Contract: the Nx rendering of a resolved worker plan sets NX_PARALLEL to the local jobs, switches batch mode on only where a worker kind multiplexes and an executor can batch, and reports every contract field exactly once with the honest support level, equal to an independent Python rendering for every plan of `./🔣️.json`; batch capability is read from executors files as the corpus and Python say. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxBatchCapableV1,nxWorkerSettingsV1} from "@UNIT/🟦️.ts";
import {requireWorkerPlanV1,workerContractFieldsV1} from "@LIB/🏗️build/👷️workers/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema and every authored plan is a valid worker plan",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,executors:[{name:"x",value:null}]})).toBe(false);
 for(const row of corpus.plans)expect(()=>requireWorkerPlanV1(row.plan),row.name).not.toThrow();
});

test("every plan is rendered like the Python rendering with and without batch capable executors, mapping each contract field exactly once",()=>{
 const rows=corpus.plans.flatMap(row=>[true,false].map(batchCapable=>({name:row.name,plan:row.plan,batchCapable}))),answers=ask({op:"render",rows});let batchOn=0;
 const fields=[...workerContractFieldsV1()].sort();
 for(const[index,row]of rows.entries()){
  const settings=nxWorkerSettingsV1(row.plan as never,{batchCapable:row.batchCapable}),label=row.name+" "+row.batchCapable;
  expect(JSON.parse(JSON.stringify(settings)),label).toEqual(answers[index]);
  expect(settings.mapping.map(item=>item.field).sort(),label).toEqual(fields);
  expect(settings.environment.NX_PARALLEL,label).toBe(String(row.plan.localJobs));
  if(settings.environment.NX_BATCH_MODE!==undefined){batchOn++;expect(settings.environment.NX_BATCH_MODE,label).toBe("true");expect(row.batchCapable,label).toBe(true);}
  expect(Object.isFrozen(settings)&&Object.isFrozen(settings.environment)&&Object.isFrozen(settings.mapping),label).toBe(true);
 }
 expect(batchOn).toBeGreaterThan(5);
});

test("batch capability is read from executors files exactly as the corpus and Python say",()=>{
 const answers=ask({op:"batch",values:corpus.executors.map(row=>row.value)});
 for(const[index,row]of corpus.executors.entries()){expect(nxBatchCapableV1(row.value),row.name).toBe(row.expected);expect(answers[index],row.name).toBe(row.expected);}
});
