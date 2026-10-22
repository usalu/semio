/** 🧭️ Contract: the CI validation resolution turns a GitHub environment, event, history and ancestry into the baseline, the history issue, the number of API requests and of ancestry questions that an independent Python model derives for every scenario of `./🔣️.json` and that the authored key scenarios state; unsafe or inconsistent identities are refused with the authored messages. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {resolveCiValidation} from "@UNIT/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(scenarios:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({scenarios}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

async function resolveScenario(scenario:(typeof corpus.scenarios)[number]){
 let requests=0,ancestors=0,page=0;
 const transport=async(path:string)=>{requests++;if(path.includes("/workflows/")){const answer=scenario.pages[page++]??{status:404,body:null};return{status:answer.status,body:structuredClone(answer.body)};}return{status:scenario.run.status,body:structuredClone(scenario.run.body)};};
 try{
  const result=await resolveCiValidation({environment:scenario.environment,event:structuredClone(scenario.event),head:scenario.head,full:scenario.full},transport as never,async base=>{ancestors++;return scenario.ancestors[base as keyof typeof scenario.ancestors]===true;},new AbortController().signal);
  return{baseline:{mode:result.baseline.mode,base:result.baseline.base,runId:result.baseline.runId,reason:result.baseline.reason},issue:result.historyIssue,requests,ancestorCalls:ancestors};
 }catch(error){return{error:(error as Error).message};}
}

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,scenarios:[{...corpus.scenarios[0],expect:{mode:"some"}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,head:"abc"})).toBe(false);
});

test("every scenario resolves to what the Python model derives, requests and ancestry questions included",async()=>{
 const answers=ask(corpus.scenarios);let affected=0,full=0,issues=0,refused=0;
 for(const[index,scenario]of corpus.scenarios.entries()){
  const actual=await resolveScenario(scenario);expect(actual,scenario.name).toEqual(answers[index]);
  if("error" in actual)refused++;else{if(actual.baseline.mode==="affected")affected++;else full++;if(actual.issue)issues++;}
 }
 expect(affected).toBeGreaterThan(5);expect(full).toBeGreaterThan(15);expect(issues).toBeGreaterThan(5);expect(refused).toBeGreaterThan(8);
});

test("the authored key scenarios hold",async()=>{
 for(const scenario of corpus.scenarios.filter(item=>item.expect!==null)){
  const expected=scenario.expect!,actual=await resolveScenario(scenario) as any;
  if(expected.error!==""){expect(actual.error,scenario.name).toBe(expected.error);continue;}
  expect(actual.baseline,scenario.name).toEqual({mode:expected.mode,base:expected.base,runId:expected.runId,reason:expected.reason});
  expect(actual.issue,scenario.name).toBe(expected.issue);expect(actual.requests,scenario.name).toBe(expected.requests);expect(actual.ancestorCalls,scenario.name).toBe(expected.ancestorCalls);
 }
});

test("a cancelled resolution never starts and an aborted history is not mistaken for unavailable history",async()=>{
 const scenario=corpus.scenarios[0]!,controller=new AbortController();controller.abort(Error("cancelled"));
 await expect(resolveCiValidation({environment:scenario.environment,event:scenario.event,head:scenario.head,full:false},(async()=>({status:200,body:null})) as never,async()=>true,controller.signal)).rejects.toThrow("cancelled");
 const late=new AbortController();
 await expect(resolveCiValidation({environment:scenario.environment,event:structuredClone(scenario.event),head:scenario.head,full:false},(async()=>{late.abort(Error("late cancel"));return{status:200,body:structuredClone(scenario.run.body)};}) as never,async()=>true,late.signal)).rejects.toThrow("late cancel");
});
