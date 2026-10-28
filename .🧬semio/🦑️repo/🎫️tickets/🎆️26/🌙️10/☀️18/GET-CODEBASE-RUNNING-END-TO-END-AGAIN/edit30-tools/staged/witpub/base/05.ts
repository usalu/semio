/** 🧾️ Contract: the Bazel input witness is the same digest whether it is computed by the reference function, by the real witness program run under Bun over a parameter file, or by an independent Python implementation, for every case of `./🔣️.json`; the program refuses a missing output path and writes atomically. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {BAZEL_WITNESS_RUNNER,BAZEL_WITNESS_STARLARK,bazelWitnessDigestV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const bytesOf=(content:any):Buffer=>Buffer.from("repeat" in content?content.repeat.repeat(content.times):content.text);
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"witness-")));writeFileSync(join(workspace,"runner.mjs"),BAZEL_WITNESS_RUNNER);});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],expected:{version:2,files:0,sha256:"x"}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[1],files:[{path:"a\nb",content:{text:"x"}}]}]})).toBe(false);
});

test("the reference digest equals the authored digest and the Python digest for every case",()=>{
 const answers=ask(corpus.cases);
 for(const[index,row]of corpus.cases.entries()){
  const files=row.files.map(file=>({path:file.path,bytes:bytesOf(file.content)}));
  expect<unknown>(bazelWitnessDigestV1(files),row.name).toEqual(row.expected);expect(answers[index],row.name).toEqual(row.expected);
  expect<unknown>(bazelWitnessDigestV1([...files].reverse()),row.name+" reversed").toEqual(row.expected);
 }
 const changed=bazelWitnessDigestV1([{path:"a.ts",bytes:Buffer.from("abc")}]),other=bazelWitnessDigestV1([{path:"a.ts",bytes:Buffer.from("abd")}]);
 expect(changed.sha256).not.toBe(other.sha256);
});

test("the real witness program under Bun writes exactly that digest for every case and replaces its output atomically",()=>{
 for(const[index,row]of corpus.cases.entries()){
  const base=join(workspace,"case"+index);mkdirSync(base,{recursive:true});
  for(const file of row.files){mkdirSync(dirname(join(base,...file.path.split("/"))),{recursive:true});writeFileSync(join(base,...file.path.split("/")),bytesOf(file.content));}
  const output="out/deep/"+index+".witness";writeFileSync(join(base,"params"),[output,...row.files.map(file=>file.path)].join("\n")+"\n");
  const run=spawnSync(process.execPath,[join(workspace,"runner.mjs"),"@params"],{cwd:base,encoding:"utf8",env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});
  expect(run.status,row.name+run.stderr).toBe(0);
  const text=readFileSync(join(base,output),"utf8");expect(text.endsWith("\n"),row.name).toBe(true);expect(JSON.parse(text),row.name).toEqual(row.expected);
  expect(readdirSync(join(base,"out/deep")),row.name).toEqual([index+".witness"]);
 }
});

test("the program refuses a parameter file without an output path and the rule keeps its actions local",()=>{
 const base=join(workspace,"empty");mkdirSync(base,{recursive:true});writeFileSync(join(base,"params"),"\n");
 const run=spawnSync(process.execPath,[join(workspace,"runner.mjs"),"@params"],{cwd:base,encoding:"utf8"});
 expect(run.status).not.toBe(0);expect(run.stderr).toContain("[witness] output path required");
 for(const required of['"no-sandbox": "1"','"no-remote-exec": "1"','ctx.actions.run(','declare_file(ctx.label.name + ".witness")','mnemonic = "RepoInputsWitness"'])expect(BAZEL_WITNESS_STARLARK,required).toContain(required);
 expect(BAZEL_WITNESS_STARLARK.includes("\t")).toBe(false);
});
