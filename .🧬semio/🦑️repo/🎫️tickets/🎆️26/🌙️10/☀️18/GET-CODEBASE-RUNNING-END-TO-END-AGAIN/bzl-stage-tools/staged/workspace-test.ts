/** 🗺️ Contract: the generated Bazel workspace links the checkout without letting recursive queries crawl it, and a task's own Cargo directories are task data resolved against the workspace. */
import {expect,test} from "bun:test";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";
import component from "../../🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../../../📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const api=await import("../../📜️script.ts"),python=Bun.which("python3"),workspace=join(tmpdir(),"bazel-workspace-fixture"),task=(env:Record<string,unknown>|null)=>({id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"bun ./📜️script.ts",...(env?{env}:{})}}}) as any;

test("Bazel workspace fixtures are closed and name the link the action policy declares",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 expect(corpus.link.name).toBe(component.definitions.Actions.const.sources.link);expect(corpus.link.ignore).toBe(corpus.link.name+"\n");
});

test("Bazel task Cargo directories are task data: they resolve against the workspace like an independent Python join and leave the base environment",()=>{
 const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});
 const values=corpus.cargo.cases.flatMap(row=>Object.values(row.directories)),independent=python?(()=>{const result=run("import json,os,sys\nworkspace,values=json.loads(sys.stdin.read())\nprint(json.dumps([os.path.normpath(os.path.join(workspace,value)) for value in values]))",[workspace,values]);expect(result.exitCode,result.stderr.toString()).toBe(0);return JSON.parse(result.stdout.toString()) as string[];})():undefined;
 let position=0;
 for(const row of corpus.cargo.cases){
  const actual=api.bazelTaskCargoDirectoriesV1(workspace,task(row.env)),expected=Object.fromEntries(Object.entries(row.directories).map(([key,value])=>[key,independent?independent[position++]!:resolve(workspace,value)]));
  expect(actual,row.name).toEqual(expected);for(const key of Object.keys(actual))expect(corpus.cargo.keys,row.name).toContain(key);
  const base=api.bazelTaskBaseEnvironmentV1(workspace,task(row.env),{AMBIENT:"yes"});
  for(const key of corpus.cargo.keys)expect(Object.hasOwn(base,key),row.name+" "+key).toBe(false);
  for(const[key,value]of Object.entries(row.base))expect(base[key],row.name+" "+key).toBe(value);
  expect(base.AMBIENT).toBe("yes");expect(base.SEMIO_BUILD_TOOL).toBe("bazel");expect(base.SEMIO_WORKSPACE_ROOT).toBe(workspace);
 }
});

test("Bazel issues a task's own Cargo directories on top of the caller's authority, which refuses them when declared as ordinary task environment",async()=>{
 const row=corpus.cargo.cases[0]!,root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{});
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:cargo-directories",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  const declared=task(row.env),environment=api.issueBazelTaskEnvironmentV1(root,original,declared,api.bazelTaskBaseEnvironmentV1(root,declared,process.env));
  for(const[key,value]of Object.entries(row.directories))expect(environment[key],key).toBe(resolve(root,value));
  expect(()=>issueBuildTaskEnvironmentV1(root,original,declared.definition.execution,declared.args,{...process.env,...row.env as Record<string,string>})).toThrow();
 });
},30000);

test("Bazel links the checkout into the workspace and lists the link in .bazelignore, idempotently and replacing a stale link",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-link-")),checkout=join(directory,"checkout"),other=join(directory,"other"),space=join(directory,"workspace");
 try{
  mkdirSync(checkout);mkdirSync(other);mkdirSync(space);writeFileSync(join(checkout,"a.txt"),"a\n");
  await api.linkBazelSourcesV1(checkout,space);
  expect(readFileSync(join(space,".bazelignore"),"utf8")).toBe(corpus.link.ignore);expect(realpathSync(join(space,corpus.link.name))).toBe(realpathSync(checkout));expect(readFileSync(join(space,corpus.link.name,"a.txt"),"utf8")).toBe("a\n");
  const past=new Date(Date.now()-60000);utimesSync(join(space,".bazelignore"),past,past);const before=lstatSync(join(space,".bazelignore")).mtimeMs;
  await api.linkBazelSourcesV1(checkout,space);expect(lstatSync(join(space,".bazelignore")).mtimeMs).toBe(before);
  writeFileSync(join(space,".bazelignore"),"wrong\n");await api.linkBazelSourcesV1(checkout,space);expect(readFileSync(join(space,".bazelignore"),"utf8")).toBe(corpus.link.ignore);
  await api.linkBazelSourcesV1(other,space);expect(realpathSync(join(space,corpus.link.name))).toBe(realpathSync(other));expect(existsSync(join(space,corpus.link.name,"a.txt"))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;

test.skipIf(!binary||process.platform==="win32")("the real Bazel crawls //... only with the link listed in .bazelignore, while explicit labels through the link keep resolving",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-ignore-")),checkout=join(directory,"checkout"),space=join(directory,"workspace"),base=join(directory,"base"),install=join(directory,"install"),
  bazel=(args:string[])=>spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=60",...args],{cwd:space,encoding:"utf8",timeout:240000,env:{...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})}});
 try{
  mkdirSync(checkout);mkdirSync(space);writeFileSync(join(checkout,"a.txt"),"a\n");symlinkSync(space,join(checkout,"workspace-loop"),"dir");
  writeFileSync(join(space,"MODULE.bazel"),'module(name="ignore")\n');writeFileSync(join(space,"BUILD.bazel"),`filegroup(name = "x", srcs = ["${corpus.link.name}/a.txt"])\n`);
  await api.linkBazelSourcesV1(checkout,space);
  const ignored=bazel(["query","--lockfile_mode=off","//..."]);expect(ignored.status,ignored.stderr).toBe(0);expect(ignored.stdout).toContain("//:x");
  const label=bazel(["query","--lockfile_mode=off",`//:${corpus.link.name}/a.txt`]);expect(label.status,label.stderr).toBe(0);
  rmSync(join(space,".bazelignore"));
  const crawled=bazel(["query","--lockfile_mode=off","//..."]);expect(crawled.status).not.toBe(0);expect(crawled.stderr).toMatch(/infinite symlink expansion/iu);
 }finally{spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:space,timeout:120000});rmSync(directory,{recursive:true,force:true});}
},600000);
