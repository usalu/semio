/** 📍️ Contract: a funded compiler workspace is exactly one unit directly below the mode's units directory with its own `.tmp` sibling; the unit's own corpus in `./🔣️.json`, the extra corpus `./📋️extra.json` and an independent Python reading of posixpath and ntpath agree, ajv agrees on closed requests, and malformed requests are refused rather than answered.*/
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "@FW/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {rustUnitFundedWorkspacePathV1} from "@UNIT/🟦️.ts";
import unitSchema from "@UNIT/🧬️schema/🔣️.json";
import layoutPolicy from "@LIB/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🔣️policy.json";
import own from "@UNIT/🧪️tests/🔣️.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(rows:unknown[]):boolean[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({policy:layoutPolicy,rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};

test("both corpora are closed by their schemas and the request schema agrees between ajv and the repository validator",()=>{
 const ajv=new Ajv({strict:false}),request={...unitSchema,$ref:"#/definitions/Request"},ownCase={...unitSchema,$ref:"#/definitions/Case"};
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],extra:1}]})).toBe(false);
 for(const row of own)expect(ajv.validate(ownCase,row),row.name+JSON.stringify(ajv.errors)).toBe(true);
 for(const row of [...own,...corpus.cases])expect(validateJsonSchemaSubset(unitSchema.definitions.Request,row.request,unitSchema).length===0,row.name).toBe(ajv.validate(request,row.request));
});

test("the unit's own cases and the extra cases hold, refusals are worded as authored and the verdict equals the Python one",()=>{
 const valid=corpus.cases.filter(row=>row.error===""),answers=ask(valid.map(row=>row.request));
 for(const row of own)expect(rustUnitFundedWorkspacePathV1(row.request as never),row.name).toBe(row.expected);
 for(const row of corpus.cases){
  let verdict:boolean|null=null;const message=refusal(()=>{verdict=rustUnitFundedWorkspacePathV1(row.request as never);});
  expect(message,row.name).toBe(row.error);
  if(row.error==="")expect(verdict,row.name).toBe(row.expected);
  if(row.error==="")expect(answers[valid.indexOf(row)],row.name).toBe(row.expected);
 }
});

test("the answer is independent of how the layout spells the units directory: it follows the authored namespace and mode letters",()=>{
 const request={version:1,root:"/owned",mode:"release",directory:"/owned/"+layoutPolicy.buildNamespace+"/"+layoutPolicy.modes.release+"/units/u",temporary:"/owned/"+layoutPolicy.buildNamespace+"/"+layoutPolicy.modes.release+"/units/u.tmp",platform:"posix"} as const;
 expect(rustUnitFundedWorkspacePathV1(request)).toBe(true);
 expect(rustUnitFundedWorkspacePathV1({...request,mode:"prototype"})).toBe(false);
});
