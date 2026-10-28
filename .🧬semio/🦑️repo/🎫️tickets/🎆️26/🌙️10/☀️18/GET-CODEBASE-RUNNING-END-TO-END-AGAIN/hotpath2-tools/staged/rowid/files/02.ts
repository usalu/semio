import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {BAZEL_RUNTIME_IDENTITY_POLICY_V1,bazelRuntimeIdentityV1,bazelRuntimeSubjectV1,bazelRuntimeToolRowV1,bazelRuntimeWordsV1,checkBazelRuntimeRowV1,measureBazelRuntimeRowV1,parseBazelRuntimeSubjectV1,systemBazelRuntimeHostV1,type BazelRuntimeHostV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),schemaPath=join(here,"..","🧬️schema","🔣️.json"),oracle=join(here,"🧮️resources","🐍️oracle","🐍️.py"),root=process.cwd();
const python=(input:unknown)=>{const run=spawnSync(process.platform==="win32"?"python":"python3",["-I",oracle,schemaPath],{input:JSON.stringify(input),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {words:(string[]|null)[];identities:string[]};};
const environment={SEMIO_BUILD_MODE:"production",PATH:process.env.PATH};
const sha=(text:string)=>new Bun.CryptoHasher("sha256").update(text).digest("hex");

const workspace=()=>{
 const directory=mkdtempSync(join(tmpdir(),"rowid-")),program=join(directory,"program"),runtime=join(directory,"runtime");
 writeFileSync(program,"#!/bin/sh\necho v1\n");chmodSync(program,0o755);writeFileSync(runtime,"#!/bin/sh\n");chmodSync(runtime,0o755);
 const real=systemBazelRuntimeHostV1(root),host:BazelRuntimeHostV1={runtime,stat:real.stat,resolve:()=>({command:program,prefix:[],source:"path"})};
 return {directory,program,runtime,host};
};
const listed="repo tools exec --light bun -- --version";

test("The rows the repository authors are listed, shell syntax and unlisted rows have no identity, and Python's shlex agrees on every word list",()=>{
 const nx=JSON.parse(readFileSync(join(root,"nx.json"),"utf8")) as {namedInputs:{sharedGlobals:{runtime?:string}[]}};
 const authored=nx.namedInputs.sharedGlobals.flatMap(row=>row.runtime===undefined?[]:[row.runtime]);
 expect(authored.length).toBeGreaterThan(1);
 for(const text of authored)expect(bazelRuntimeToolRowV1(text),text).toBeDefined();
 for(const row of fixture.rows){
  expect(bazelRuntimeWordsV1(row.text),row.text).toEqual(row.words===null?undefined:row.words);
  expect(bazelRuntimeToolRowV1(row.text)!==undefined,row.text).toBe(row.listed);
 }
 expect(python({platform:process.platform,arch:process.arch,words:fixture.rows.map(row=>row.text),identities:[]}).words).toEqual(fixture.rows.map(row=>row.words));
 expect(schema.definitions.Policy.const).toEqual(BAZEL_RUNTIME_IDENTITY_POLICY_V1);
});

test("The identity of a row holds the resolved program, the runtime and the stat fields: unchanged is equal, any rewrite, touch, mode change or replacement differs, and an independent Python digest agrees",()=>{
 const {directory,program,runtime,host}=workspace();
 try{
  const identity=()=>bazelRuntimeIdentityV1(root,listed,environment,host);
  const first=identity();
  expect(first).toMatch(/^[0-9a-f]{64}$/u);
  expect(identity()).toBe(first);
  expect(python({platform:process.platform,arch:process.arch,words:[],identities:[{tool:"bun",arguments:["--version"],source:"path",prefix:[],paths:[runtime,program]}]}).identities).toEqual([first!]);
  const seen=new Set([first]),step=(label:string,change:()=>void)=>{change();const next=identity();expect(seen.has(next!),label).toBe(false);seen.add(next!);};
  step("touch",()=>utimesSync(program,new Date(1e12),new Date(1e12)));
  step("rewrite with equal size",()=>{const bytes=readFileSync(program);bytes[bytes.length-3]=bytes[bytes.length-3]===0x31?0x32:0x31;writeFileSync(program,bytes);});
  step("append",()=>writeFileSync(program,"#!/bin/sh\necho v1 and more\n"));
  step("mode",()=>chmodSync(program,0o700));
  step("replaced by rename",()=>{const other=join(directory,"other");writeFileSync(other,readFileSync(program));chmodSync(other,0o700);renameSync(other,program);});
  step("runtime touched",()=>utimesSync(runtime,new Date(2e12),new Date(2e12)));
  const moved=join(directory,"moved");writeFileSync(moved,readFileSync(program));chmodSync(moved,0o700);
  expect(seen.has(bazelRuntimeIdentityV1(root,listed,environment,{...host,resolve:()=>({command:moved,prefix:[],source:"path"})})!)).toBe(false);
  expect(bazelRuntimeIdentityV1(root,listed,environment,{...host,resolve:()=>({command:join(directory,"missing"),prefix:[],source:"path"})})).toBeUndefined();
  expect(bazelRuntimeIdentityV1(root,listed,environment,{...host,resolve:()=>undefined})).toBeUndefined();
  expect(bazelRuntimeIdentityV1(root,"repo tools exec --light bun -- --revision",environment,host)).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("A recorded row is answered without a process while the identity holds, runs again when it changed, and compares the digest of the new output",async()=>{
 const {directory,program,host}=workspace();
 try{
  let runs=0,output="1.4.2\n";
  const run=async()=>{runs+=1;return output;},digest=(stdout:string)=>sha(JSON.stringify(stdout));
  const recorded=await measureBazelRuntimeRowV1({root,runtime:listed,environment,run,host});
  expect(runs).toBe(1);
  expect(parseBazelRuntimeSubjectV1(recorded.subject)).toEqual({runtime:listed,mode:"production",identity:bazelRuntimeIdentityV1(root,listed,environment,host)});
  const expected=digest(recorded.stdout),check=(subject=recorded.subject,env:Record<string,string|undefined>=environment)=>checkBazelRuntimeRowV1({root,subject,expected,environment:env,run,digest,host});
  expect(await check()).toBe(true);expect(await check()).toBe(true);
  expect(runs).toBe(1);
  expect(await check(recorded.subject,{...environment,SEMIO_BUILD_MODE:"development"})).toBe(false);
  expect(runs).toBe(1);
  utimesSync(program,new Date(1e12),new Date(1e12));
  expect(await check()).toBe(true);
  expect(runs).toBe(2);
  output="1.5.0\n";
  expect(await check()).toBe(false);
  expect(runs).toBe(3);
  expect(await check(bazelRuntimeSubjectV1(listed,"production"))).toBe(false);
  expect(runs).toBe(4);
  output="1.4.2\n";
  expect(await check(bazelRuntimeSubjectV1(listed,"production"))).toBe(true);
  expect(runs).toBe(5);
  expect(await check("")).toBe(false);
  expect(await check("a\0b\0c\0d")).toBe(false);
  expect(runs).toBe(5);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("An identity that moves while the row runs is not recorded, and a row that is not listed is recorded by its mode alone and always runs again",async()=>{
 const {directory,program,host}=workspace();
 try{
  let runs=0;
  const digest=(stdout:string)=>sha(JSON.stringify(stdout));
  const moving=await measureBazelRuntimeRowV1({root,runtime:listed,environment,host,run:async()=>{runs+=1;utimesSync(program,new Date(3e12),new Date(3e12));return "x";}});
  expect(parseBazelRuntimeSubjectV1(moving.subject)).toEqual({runtime:listed,mode:"production"});
  const other="repo tools exec --light node -- -p \"process.platform\"",unlisted=await measureBazelRuntimeRowV1({root,runtime:other,environment,host,run:async()=>{runs+=1;return "darwin";}});
  expect(parseBazelRuntimeSubjectV1(unlisted.subject)).toEqual({runtime:other,mode:"production"});
  runs=0;
  for(let round=0;round<3;round+=1)expect(await checkBazelRuntimeRowV1({root,subject:unlisted.subject,expected:digest("darwin"),environment,run:async()=>{runs+=1;return "darwin";},digest,host})).toBe(true);
  expect(runs).toBe(3);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("The real tool registry identifies the repository's authored rows, follows the program found on the path, and the identity of the running runtime is stable",()=>{
 const nx=JSON.parse(readFileSync(join(root,"nx.json"),"utf8")) as {namedInputs:{sharedGlobals:{runtime?:string}[]}};
 const authored=nx.namedInputs.sharedGlobals.flatMap(row=>row.runtime===undefined?[]:[row.runtime]);
 const first=authored.map(text=>bazelRuntimeIdentityV1(root,text,process.env)),again=authored.map(text=>bazelRuntimeIdentityV1(root,text,process.env));
 expect(first.every(value=>value!==undefined)).toBe(true);
 expect(again).toEqual(first);
 expect(new Set(first).size).toBe(first.length);
 const directory=mkdtempSync(join(tmpdir(),"rowid-path-"));
 try{
  mkdirSync(join(directory,"bin"));const fake=join(directory,"bin",process.platform==="win32"?"node.exe":"node");writeFileSync(fake,"#!/bin/sh\necho fake\n");chmodSync(fake,0o755);
  const node=authored.find(text=>text.includes(" node "))!,shadowed=bazelRuntimeIdentityV1(root,node,{...process.env,PATH:join(directory,"bin")+(process.platform==="win32"?";":":")+(process.env.PATH??"")});
  expect(shadowed).toBeDefined();
  expect(shadowed).not.toBe(first[authored.indexOf(node)]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
