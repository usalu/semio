/** 🧪️ Tests for build typescript incremental. */
import {typeScriptCheckCommandV1} from "../../🟦️.ts";
import {createHermeticServiceV1,createTypeScriptCheckEngineV1,typeScriptCheckSwitchesAdmittedV1,type TypeScriptCheckEngineV1} from "../../🔁️incremental/🟦️.ts";
import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import schema from "./🧬️schema/🔣️.json";
import corpus from "./🔣️.json";

/** 🔁️ Law: a warm incremental check (`typeScriptCheckCommandV1`, one state file kept across every step) and the in-memory engine (`createTypeScriptCheckEngineV1`, one engine kept across every step) report exactly the diagnostics of a cold `tsc --noEmit -p` and of the compiler API on the same tree, for each step of a language-agnostic edit corpus (hand-written expectations per step); the engine's exit code equals the cold compiler's. Stale compiler state can therefore never mask or invent an error.
 * @see ../../🟦️.ts — the command line the live check runs
 * @see ../../🔁️incremental/🟦️.ts — the engine */
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output)throw Error("Compiler tests require the original artifact directory");
const compiler=createRequire(import.meta.url).resolve("typescript/bin/tsc");

const codesOf=(text:string):string[]=>unique(text.split("\n").flatMap(line=>{const match=/^([^(]+)\(\d+,\d+\): error (TS\d+):/u.exec(line);return match?[`${match[1]}:${match[2]}`]:[];}));

function runOfCli(root:string,args:readonly string[]):{codes:string[];status:number}{
 const run=spawnSync(process.execPath,[...args,"--pretty","false"],{cwd:root,encoding:"utf8",maxBuffer:16*1024*1024});
 if(run.error)throw run.error;
 return {codes:codesOf(run.stdout),status:run.status??-1};
}

const diagnosticsOfCli=(root:string,args:readonly string[]):string[]=>runOfCli(root,args).codes;

function diagnosticsOfApi(root:string):string[]{
 const parsed=ts.getParsedCommandLineOfConfigFile(join(root,"tsconfig.json"),{},{...ts.sys,onUnRecoverableConfigFileDiagnostic:diagnostic=>{throw Error(String(diagnostic.messageText));}});
 if(!parsed)throw Error("tsconfig.json required");
 const program=ts.createProgram({rootNames:parsed.fileNames,options:parsed.options});
 return unique(ts.getPreEmitDiagnostics(program).flatMap(diagnostic=>diagnostic.file&&diagnostic.code?[`${diagnostic.file.fileName.slice(root.length+1)}:TS${diagnostic.code}`]:[]));
}

const unique=(rows:readonly string[]):string[]=>[...new Set(rows)].sort();

test("the edit corpus is closed data",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...corpus,steps:[{name:"x",expected:["no-code"]}]})).toBe(false);
 expect(validate({...corpus,extra:1})).toBe(false);
 expect(new Set(corpus.steps.map(step=>step.name)).size).toBe(corpus.steps.length);
});

test("the live warm command line is the cold command plus the incremental state file only",()=>{
 const warm=typeScriptCheckCommandV1("tsc.js","tsconfig.json",["--listFiles"],"owned/state.tsbuildinfo"),parsed=ts.parseCommandLine(warm.slice(1));
 expect(parsed.errors).toEqual([]);
 expect(warm).toEqual(["tsc.js","--noEmit","-p","tsconfig.json","--listFiles","--incremental","true","--tsBuildInfoFile","owned/state.tsbuildinfo"]);
 expect(parsed.options.noEmit).toBe(true);expect(parsed.options.incremental).toBe(true);
});

type Row={name:string;expected:string[];warm:string[];cold:string[];api:string[];engine:string[];coldStatus:number;engineStatus:number};

async function walk(upTo:string,flags:readonly string[],engine:TypeScriptCheckEngineV1=createTypeScriptCheckEngineV1(ts as never)):Promise<Row[]>{
 const root=mkdtempSync(join(output!,"incremental-")),state=join(root,"..",`state-${process.pid}-${Date.now()}.tsbuildinfo`),rows:Row[]=[];
 try{
  const write=(files:Readonly<Record<string,string>>):void=>{for(const[name,text]of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}};
  write(corpus.files);
  for(const step of corpus.steps){
   write((step as {write?:Record<string,string>}).write??{});
   for(const name of(step as {delete?:string[]}).delete??[])rmSync(join(root,name),{force:true});
   const cold=runOfCli(root,[compiler,"--noEmit","-p","tsconfig.json"]),answer=await engine.check({cwd:root,project:"tsconfig.json"});
   rows.push({name:step.name,expected:unique(step.expected),warm:diagnosticsOfCli(root,[...typeScriptCheckCommandV1(compiler,"tsconfig.json",[],state),...flags]),cold:cold.codes,api:diagnosticsOfApi(root),engine:codesOf(answer.output),coldStatus:cold.status,engineStatus:answer.exitCode});
   expect(existsSync(state),`state after ${step.name}`).toBe(true);
   if(step.name===upTo)break;
  }
  expect(readFileSync(state,"utf8").length).toBeGreaterThan(100);
  return rows;
 }finally{rmSync(root,{recursive:true,force:true});rmSync(state,{force:true});}
}

test("warm incremental diagnostics and the in-memory engine equal the cold compiler and the compiler API on every corpus step",async()=>{
 for(const row of await walk(corpus.steps.at(-1)!.name,[])){
  expect(row.cold,`cold ${row.name}`).toEqual(row.expected);
  expect(row.api,`api ${row.name}`).toEqual(row.expected);
  expect(row.warm,`warm ${row.name}`).toEqual(row.expected);
  expect(row.engine,`engine ${row.name}`).toEqual(row.expected);
  expect(row.engineStatus,`engine exit ${row.name}`).toBe(row.coldStatus);
 }
},600000);

test("the law has teeth: a warm check that skips semantic checking is told apart from the cold compiler at the first break",async()=>{
 const rows=await walk("transitive-break",["--noCheck"]),broken=rows.find(row=>row.name==="transitive-break")!;
 expect(broken.cold).toEqual(broken.expected);
 expect(broken.warm).not.toEqual(broken.expected);
},600000);

test("the law has teeth for the engine: reusing module resolutions across a file created outside the root set is told apart from the cold compiler",async()=>{
 const stale={...ts,createIncrementalCompilerHost:(options:unknown,system:unknown)=>new Proxy((ts as any).createIncrementalCompilerHost(options,system),{set(target,key,value){if(key!=="hasInvalidatedResolutions")(target as any)[key]=value;return true;}})};
 const rows=await walk("nested-created",[],createTypeScriptCheckEngineV1(stale as never)),created=rows.find(row=>row.name==="nested-created")!;
 expect(created.cold).toEqual(created.expected);
 expect(created.engine).not.toEqual(created.expected);
},600000);

function fixture(files:Readonly<Record<string,string>>=corpus.files):string{
 const root=mkdtempSync(join(output!,"engine-"));
 for(const[name,text]of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}
 return root;
}

test("a second check of an unchanged tree parses nothing and a one-file edit parses one file",async()=>{
 const root=fixture(),engine=createTypeScriptCheckEngineV1(ts as never);
 try{
  const first=await engine.check({cwd:root,project:"tsconfig.json"}),again=await engine.check({cwd:root,project:"tsconfig.json"});
  expect(first.stats.warm).toBe(false);expect(first.stats.parsed).toBe(first.stats.files);
  expect(again.stats).toMatchObject({warm:true,parsed:0,files:first.stats.files});
  writeFileSync(join(root,"d.json"),'{ "k": 2 }\n');
  const edited=await engine.check({cwd:root,project:"tsconfig.json"});
  expect(edited.stats.parsed).toBe(1);expect(edited.exitCode).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a cancelled check rejects and drops its program; the next check is cold and correct",async()=>{
 const root=fixture(),engine=createTypeScriptCheckEngineV1(ts as never);
 try{
  await engine.check({cwd:root,project:"tsconfig.json"});
  const controller=new AbortController();controller.abort(new Error("stop"));
  await expect(engine.check({cwd:root,project:"tsconfig.json",signal:controller.signal})).rejects.toThrow("stop");
  const phases:string[]=[];
  const live=new AbortController(),after=await engine.check({cwd:root,project:"tsconfig.json",signal:live.signal,onPhase:phase=>phases.push(phase)});
  expect(after.exitCode).toBe(0);expect(phases).toEqual(["configuration","program","diagnostics"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a check that leaves the collected heap above its bound releases everything and says why",async()=>{
 const root=fixture(),engine=createTypeScriptCheckEngineV1(ts as never,{maximumHeapBytes:1});
 try{
  const result=await engine.check({cwd:root,project:"tsconfig.json"});
  expect(result.stats.releasedFor).toBe("memory");expect(engine.retained()).toBe(0);
  expect((await engine.check({cwd:root,project:"tsconfig.json"})).stats.warm).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the engine keeps at most the configured number of programs, least recently used first",async()=>{
 const one=fixture(),two=fixture(),engine=createTypeScriptCheckEngineV1(ts as never,{retainedPrograms:1});
 try{
  await engine.check({cwd:one,project:"tsconfig.json"});await engine.check({cwd:two,project:"tsconfig.json"});
  expect(engine.retained()).toBe(1);
  expect((await engine.check({cwd:one,project:"tsconfig.json"})).stats.warm).toBe(false);
  expect(()=>createTypeScriptCheckEngineV1(ts as never,{retainedPrograms:0})).toThrow();
  expect(engine.release()).toBe(1);expect(engine.retained()).toBe(0);
 }finally{rmSync(one,{recursive:true,force:true});rmSync(two,{recursive:true,force:true});}
});

test("only message-text switches are admitted; an unusable invocation is exit 1 with the compiler's own diagnostic",async()=>{
 const root=fixture(),engine=createTypeScriptCheckEngineV1(ts as never);
 try{
  expect(typeScriptCheckSwitchesAdmittedV1([])).toBe(true);expect(typeScriptCheckSwitchesAdmittedV1(["--noErrorTruncation"])).toBe(true);
  for(const rejected of["--listFiles","--traceResolution","--extendedDiagnostics","--pretty"]){expect(typeScriptCheckSwitchesAdmittedV1([rejected])).toBe(false);await expect(engine.check({cwd:root,project:"tsconfig.json",args:[rejected]})).rejects.toThrow("not admitted");}
  const missing=await engine.check({cwd:root,project:"absent.json"});
  expect(missing.exitCode).toBe(1);expect(missing.output).toContain("TS5083");
  expect(runOfCli(root,[compiler,"--noEmit","-p","absent.json"]).status).toBe(missing.exitCode);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the hermetic service resolves the compiler for the requesting directory, keeps one engine per compiler module and gives it all up on release",async()=>{
 const root=fixture(),asked:string[]=[];let module:unknown=ts;
 const service=createHermeticServiceV1({resolve:(directory,name)=>{asked.push(`${directory}:${name}`);return module;}});
 try{
  const first=await service.handle({cwd:root,project:"tsconfig.json"}),again=await service.handle({cwd:root,project:"tsconfig.json"});
  expect(first).toMatchObject({exitCode:0,output:""});expect(first.stats.warm).toBe(false);expect(again.stats).toMatchObject({warm:true,parsed:0});
  expect(asked).toEqual([`${root}:typescript`,`${root}:typescript`]);
  module={...ts};expect((await service.handle({cwd:root,project:"tsconfig.json"})).stats.warm,"another compiler module starts a new engine").toBe(false);
  expect(service.release()).toBe(1);expect(service.release()).toBe(0);
  expect((await service.handle({cwd:root,project:"tsconfig.json"})).stats.warm).toBe(false);
  const controller=new AbortController();controller.abort(new Error("stop"));
  await expect(service.handle({cwd:root,project:"tsconfig.json"},controller.signal)).rejects.toThrow("stop");
 }finally{rmSync(root,{recursive:true,force:true});}
});
