/** 📊️ Contract: a declared build report name is admitted exactly when the authored pattern says so, in agreement with the corpus in `./🔣️.json`, with ajv and with a Python full match, across literal names, a length sweep around the 69 character maximum and values that are not strings; the authored retention policy is the pinned one. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {repositoryBuildReportNameV1} from "@UNIT/🟦️.ts";
import schema from "@UNIT/🧬️schema/🔣️.json";
import policy from "@UNIT/🔣️policy.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(names:unknown[]):boolean[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({names,limit:schema.maxLength}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const admits=(name:unknown):boolean=>{try{return repositoryBuildReportNameV1(name as string)===name;}catch(error){expect((error as Error).message).toBe("Declared build report name required");return false;}};

test("the corpus is closed by its schema and the retention policy is the one the unit's schema pins",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,names:[{name:"a.json",accepted:"yes"}]})).toBe(false);
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Policy"},policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Policy"},{...policy,maximumReports:65})).toBe(false);
 expect(policy.maximumReportBytes*4).toBe(policy.maximumRetainedBytes);
});

test("literal names, the length sweep and values that are not strings are admitted like the corpus, ajv and Python say",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({type:"string",pattern:schema.pattern,maxLength:schema.maxLength}),sweep=corpus.lengths;
 const names=[...corpus.names.map(row=>row.name),...Array.from({length:sweep.to-sweep.from+1},(_,index)=>sweep.stem.repeat(sweep.from+index-5)+".json")],answers=ask(names);
 for(const[index,name]of names.entries()){
  const row=index<corpus.names.length?corpus.names[index]!.accepted:name.length<=sweep.maximumAccepted;
  expect(admits(name),JSON.stringify(name)).toBe(row);expect(validate(name),JSON.stringify(name)).toBe(row);expect(answers[index],JSON.stringify(name)).toBe(row);
 }
 for(const value of corpus.notStrings){expect(admits(value),JSON.stringify(value)).toBe(false);expect(validate(value)).toBe(false);}
 expect(names.filter(name=>name.length===sweep.maximumAccepted&&name.startsWith(sweep.stem)).every(admits)).toBe(true);
 expect(admits(sweep.stem.repeat(sweep.maximumAccepted-5)+".json")).toBe(true);expect(admits(sweep.stem.repeat(sweep.maximumAccepted-4)+".json")).toBe(false);
});
