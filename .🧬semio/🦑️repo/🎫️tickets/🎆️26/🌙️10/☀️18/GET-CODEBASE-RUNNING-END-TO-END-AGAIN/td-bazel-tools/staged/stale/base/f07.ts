/** 🔒️ Law: a hermetic action run by the real Bazel from a generated workspace sees the bytes of the plan, however the checkout changes between planning and the action; a later plan captures the new bytes, prunes what vanished and Bazel's own input digests agree with the corpus. */
import {expect,test} from "bun:test";
import {makeTestTreeWritableV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as execution from "../../../🏃️execution/🟦️.ts";
import {readBazelSourceSnapshotV1} from "../../🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,host=execution.bazelExecutionPlatformV1(process.platform,process.arch);
const hermetic={version:1,tools:["bun"]},graph={version:1 as const,projects:{lib:{name:"lib",root:"lib",targets:{generate:{execution:{kind:"command",command:"bun {projectRoot}/gen.ts"},outputs:["{projectRoot}/gen"],inputs:["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],cache:true,dependsOn:[],metadata:{semio:{hermetic}}}}}},dependencies:[]} as any;
type Spawn={mnemonic:string;inputs:{path:string;digest?:{hash:string}}[]};
const readLog=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows;};

test("planned-bytes corpus is closed",()=>{expect(new Ajv({strict:false}).validate(corpusSchema,corpus)).toBe(true);});

test.skipIf(!binary)("law: an input edited, removed or added after planning does not reach a hermetic action; a re-plan captures the new bytes and prunes what vanished",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"planned-bytes-")),checkout=join(directory,"checkout"),workspace=join(directory,"ws"),api=await import("../../../📜️script.ts");
 const environment={...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};
 try{
  for(const[path,text]of Object.entries(corpus.files)){mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),text);}
  const request={tasks:["lib:generate"]},plan=api.planBazelTasksV1(graph,request),label=plan.tasks["lib:generate"]!.label;
  const generate=()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:planned-bytes",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","lib:generate"],"bazel",{}),Date.now()),async original=>{
   const implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
   await api.generateBazelWorkspaceV1(checkout,graph,request,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:planned-bytes",0),implementation,()=>{},undefined,{host:host.id,tools:{[host.id]:process.execPath}});
  });
  const flags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2}),common=[`--output_base=${join(directory,"base")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=120"];
  const bazel=(args:string[])=>spawnSync(binary!,[...common,...args],{cwd:workspace,encoding:"utf8",timeout:420000,env:environment});
  const run=(log:string)=>{const built=bazel(["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,`--execution_log_json_file=${log}`,"//:"+label]);expect(built.status,built.stderr).toBe(0);const bin=bazel(["info","bazel-bin","--lockfile_mode=off","--incompatible_autoload_externally=",...flags.command]).stdout.trim().split("\n").at(-1)!,snapshot=join(bin,label+".snapshot"),manifest=JSON.parse(readFileSync(join(snapshot,"snapshot.json"),"utf8")),entry=manifest.entries.find((row:any)=>row.path==="lib/gen/out.txt");return{text:readFileSync(join(snapshot,"files",entry.file),"utf8"),spawns:readLog(log).filter(row=>row.inputs.some(input=>input.path.endsWith("lib/src/a.txt")))};};
  const digests=(spawns:Spawn[],expected:Record<string,string>)=>{const found=new Map(spawns.flatMap(spawn=>spawn.inputs.map(input=>[input.path,input.digest?.hash] as const)));for(const[path,hash]of Object.entries(expected))expect(found.get(path),path).toBe(hash);};

  await generate();
  expect(lstatSync(join(workspace,"s")).isSymbolicLink()).toBe(false);expect(lstatSync(join(workspace,"a")).isSymbolicLink()).toBe(false);
  for(const step of corpus.afterPlanning){const target=join(checkout,step.path);if(step.op==="write"){mkdirSync(join(target,".."),{recursive:true});writeFileSync(target,(step as any).text);}else rmSync(target);}
  const planned=run(join(directory,"planned.json"));
  expect(planned.text,"the action must see the planned bytes").toBe(corpus.planned.output);digests(planned.spawns,corpus.planned.inputs);

  await generate();
  const rows=await readBazelSourceSnapshotV1(workspace);for(const path of corpus.replanned.vanished){expect(rows.has(path),path).toBe(false);expect(existsSync(join(workspace,"s",path)),path).toBe(false);}
  const replanned=run(join(directory,"replanned.json"));
  expect(replanned.text,"a later plan must capture the new bytes").toBe(corpus.replanned.output);digests(replanned.spawns,corpus.replanned.inputs);
 }finally{
  spawnSync(binary!,[`--output_base=${join(directory,"base")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  makeTestTreeWritableV1(directory);rmSync(directory,{recursive:true,force:true});
 }
},900000);
