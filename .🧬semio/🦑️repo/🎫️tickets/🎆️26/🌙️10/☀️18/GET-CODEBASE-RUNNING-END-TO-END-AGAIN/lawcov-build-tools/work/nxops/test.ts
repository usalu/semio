/** 🧬️ Contract: the Nx translation of `affected`, `cache` and `inputs` gives the vectors that the corpus in `./🔣️.json` states and that an independent Python implementation of the portable changed-file rule computes for every systematic argument combination, refuses `--projects` narrowing, and the exec wrapper really exports the neutral project names and the exit status of the wrapped command. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxAffectedArgumentsV1,nxExecEnvironmentMappingV1,nxExecWrapperV1,nxOperationArgumentsV1} from "@UNIT/🟦️.ts";
import {nxBuildArgumentsV1} from "@LIB/🏗️build/🧭️invocation/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(op:string,rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({op,rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const outcome=(action:()=>unknown):{value?:unknown;error?:string}=>{try{return{value:action()};}catch(error){return{error:(error as Error).message};}};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,operations:[{args:["cache"]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,exec:{...corpus.exec,exitCodes:[256]}})).toBe(false);
});

test("the authored examples of the portable changed-file rule hold",()=>{
 for(const row of corpus.authored)expect(nxAffectedArgumentsV1(row.args),row.args.join(" ")).toEqual(row.expected);
 for(const row of corpus.authored)expect(nxOperationArgumentsV1(row.args),"composition "+row.args.join(" ")).toEqual(nxAffectedArgumentsV1(nxBuildArgumentsV1(row.args)));
});

test("every systematic `affected` vector is rewritten like the Python implementation, never losing what it does not own",()=>{
 const answers=ask("affected",corpus.affected);let allRows=0,filesRows=0;
 for(const[index,args]of corpus.affected.entries()){
  const actual=outcome(()=>nxAffectedArgumentsV1(args));expect(actual,JSON.stringify(args)).toEqual(answers[index]);
  const value=actual.value as string[];const separator=args.indexOf("--");
  if(separator>=0)expect(value.slice(value.indexOf("--")),JSON.stringify(args)).toEqual(args.slice(separator));
  if(value[0]==="run-many")allRows++;if(value.some(item=>item.startsWith("--files=")))filesRows++;
  expect(value.filter(item=>item.startsWith("--base=")||item==="--uncommitted"||item==="--untracked").length,JSON.stringify(args)).toBeLessThanOrEqual(separator>=0?Math.max(1,args.slice(separator).length):1+args.length);
 }
 expect(allRows).toBeGreaterThan(20);expect(filesRows).toBeGreaterThan(20);
});

test("narrowing by projects is refused in every spelling",()=>{
 const answers=ask("affected",corpus.refused);
 for(const[index,args]of corpus.refused.entries()){
  const actual=outcome(()=>nxAffectedArgumentsV1(args));expect(actual.error,JSON.stringify(args)).toBe("Nx affected cannot be narrowed with --projects; select the projects with run-many or drop --projects");expect(answers[index],JSON.stringify(args)).toEqual({error:actual.error});
 }
 expect(nxAffectedArgumentsV1(["affected","--","--projects=a"])).toEqual(["affected","--base=HEAD","--","--projects=a"]);
});

test("cache and inputs operations are translated or refused like the corpus and the Python implementation",()=>{
 const answers=ask("operation",corpus.operations.map(row=>row.args));
 for(const[index,row]of corpus.operations.entries()){
  const actual=outcome(()=>nxOperationArgumentsV1(row.args)),label=row.args.join(" ");
  if("error" in row){expect(actual.error,label).toBe(row.error);expect(answers[index],label).toEqual({error:row.error});}
  else{expect(actual.value??null,label).toEqual(row.expected);expect(answers[index].value??null,label).toEqual(row.expected);}
 }
});

test("the exec wrapper exports the neutral names from the Nx names, keeps the rest of the environment and returns the exit status of the wrapped command",()=>{
 expect({...nxExecEnvironmentMappingV1}).toEqual(corpus.exec.mapping);
 const wrapper=nxExecWrapperV1();expect(wrapper.length).toBe(3);expect(wrapper[0]).toBe(process.execPath);expect(wrapper[1]).toBe("-e");expect(/["$`\\%!]/u.test(wrapper[2]!)).toBe(false);
 for(const code of corpus.exec.exitCodes){
  const script="import os,sys\nprint(os.environ['SEMIO_PROJECT_NAME']+'|'+os.environ['SEMIO_PROJECT_ROOT']+'|'+os.environ['KEEP'])\nsys.exit("+code+")";
  const run=spawnSync(wrapper[0]!,[wrapper[1]!,wrapper[2]!,python,"-I","-c",script],{encoding:"utf8",env:{...process.env,...corpus.exec.environment,KEEP:"kept"}});
  expect(run.status,String(code)).toBe(code);expect(run.stdout.trim(),String(code)).toBe("project-x|owners/x|kept");
 }
});
