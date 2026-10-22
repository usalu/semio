/** 🦀️ Contract: a selected Rust compiler inventory yields exactly the literal workspace paths the corpora in `./🔣️.json` (the unit's own) and `./📋️extra.json` state, falls back on anything non-literal and refuses anything unsafe, in agreement with an independent Python reading and with ajv on what a closed request is. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "@FW/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {selectedRustNativeInputPathsV1} from "@UNIT/🟦️.ts";
import requestSchema from "@UNIT/🧬️schema/🔣️.json";
import own from "@UNIT/🧪️tests/🔣️.json";
import extra from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(rows:unknown[]):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const expanded=(row:any):any=>{
 if(!row.expand)return row.request;
 const names=Array.from({length:row.expand.count},(_,index)=>"{workspaceRoot}/"+row.expand.prefix+String(index).padStart(5,"0")+row.expand.suffix),tasks=structuredClone(row.request.tasks);tasks[0].inputs.push(...names);
 return{...row.request,tasks};
};

test("both corpora are closed by schemas and the request schema agrees between ajv and the repository validator",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,extra),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...extra,cases:[{...extra.cases[0],count:0}]})).toBe(false);
 for(const row of [...own.cases,...extra.cases]){
  const request=expanded(row);
  expect(validateJsonSchemaSubset(requestSchema,request).length===0,"id" in row?(row as any).id:"").toBe(ajv.validate(requestSchema,request));
 }
});

test("the unit's own cases hold",()=>{
 for(const row of own.cases)expect(selectedRustNativeInputPathsV1(row.request)??null,row.id).toEqual(row.paths);
});

test("every extra case matches the corpus, the Python reading and the refusal message",()=>{
 const answers=ask(extra.cases.map(row=>({tasks:expanded(row).tasks??[],engineInputs:expanded(row).engineInputs??[]})));
 for(const[index,row]of extra.cases.entries()){
  const request=expanded(row);let paths:readonly string[]|null=null;const message=refusal(()=>{paths=selectedRustNativeInputPathsV1(request)??null;});
  expect(message,row.id).toBe(row.error);
  if(row.error===""){
   if(row.count!==null)expect(paths!.length,row.id).toBe(row.count);else expect(paths,row.id).toEqual(row.paths);
   if(row.count===null||row.error==="")expect(answers[index].paths===null?null:answers[index].paths.length,row.id).toBe(paths===null?null:(paths as readonly string[]).length);
   if(row.count===null)expect(answers[index].paths,row.id).toEqual(row.paths);
  }else if(validateJsonSchemaSubset(requestSchema,request).length===0)expect(answers[index].error,row.id).toBe(row.error);
 }
});

test("the result is frozen, sorted by code unit and never aliases the request",()=>{
 const request=structuredClone(own.cases[0]!.request),paths=selectedRustNativeInputPathsV1(request)!;
 expect(Object.isFrozen(paths)).toBe(true);expect([...paths]).toEqual([...paths].sort());
 request.tasks[1]!.inputs!.push("{workspaceRoot}/later.rs");expect(paths).not.toContain("later.rs");
});
