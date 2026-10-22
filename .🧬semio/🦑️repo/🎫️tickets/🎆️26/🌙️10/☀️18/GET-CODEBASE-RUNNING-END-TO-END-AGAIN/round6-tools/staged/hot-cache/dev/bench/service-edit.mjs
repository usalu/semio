#!/usr/bin/env bun
/** 📊 Group 3 benchmark, edit then run: `service-edit.mjs --overlay <applied overlay root> --bin <native dir> [--owners 400] [--n 30] [--executors 4] [--mode warm|cold|both]`.
 * The service, the executor loop, the watcher, the client and the authority core (door, grant, admission, launch checks) are the real code; the project graph is synthetic and the tool a stand-in that stamps its start. */
import {appendFileSync,copyFileSync,cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {createResidentServiceV1} from "../resident/service/index.mjs";
import {endpointsV1} from "../resident/protocol/endpoint.mjs";
import {forwardCommandV1} from "../resident/client/index.mjs";
import {createDelegatedExecutionPortV1} from "../resident/delegate/index.mjs";
import {createExecutorPoolV1} from "../resident/delegate/executor.mjs";
import {buildWorkspace} from "./workspace.mjs";
import {benchTemporaryDirectoryV1,benchLanguagesV1,nativeDirectoryV1,nativeToolV1} from "./environment.mjs";
import {createCustodyPort,createGraphPort} from "./ports.mjs";
import {clearStamps,closureOf,load,nowUs,percentiles,readStamps,round,timed} from "./lib.mjs";

const flags=process.argv.slice(2),flag=(name,fallback)=>{const at=flags.indexOf(name);return at>=0?flags[at+1]:fallback;};
const overlay=realpathSync(flag("--overlay")),bin=realpathSync(nativeDirectoryV1(flag("--bin"))),owners=Number(flag("--owners","400")),count=Number(flag("--n","30")),size=Number(flag("--executors","4")),mode=flag("--mode","both"),languages=benchLanguagesV1(flag("--languages"));
const here=fileURLToPath(new URL(".",import.meta.url)),scratch=benchTemporaryDirectoryV1("hb-"),stamps=join(scratch,"stamps");
process.env.XDG_RUNTIME_DIR=join(scratch,"run");mkdirSync(process.env.XDG_RUNTIME_DIR,{recursive:true,mode:0o700});
const generated=join(scratch,"ws"),graph=buildWorkspace({root:generated,owners,filesPerOwner:10,layers:6});
cpSync(join(generated,"pkgs"),join(overlay,"pkgs"),{recursive:true});copyFileSync(join(generated,"graph.json"),join(overlay,"graph.json"));
for(const file of["package.json","tsconfig.base.json","bun.lock"])if(!existsSync(join(overlay,file)))copyFileSync(join(generated,file),join(overlay,file));
mkdirSync(join(overlay,".out"),{recursive:true});
const root=overlay,runtime=endpointsV1(root),target="p"+String(owners-1).padStart(4,"0")+":build",closure=closureOf(graph,target.split(":")[0]);
const environment={...process.env,BENCH_ROOT:root,BENCH_OVERLAY:overlay,BENCH_TOOL:nativeToolV1(bin,"faketool"),BENCH_STAMPS:stamps,BENCH_WORK:"30"};
const executors=createExecutorPoolV1({command:["bun",join(here,"executor.mjs")],environment,cwd:root,size,readyTimeoutMilliseconds:60000});
const execution=createDelegatedExecutionPortV1({pool:{take:()=>undefined},executors,coldCommand:["false"],argvFor:id=>["repo","run",id],environment,cwd:root});
const service=createResidentServiceV1({root,runtime,implementation:"0".repeat(64),backend:"auto",watcherBinary:nativeToolV1(bin,"semio-repo-watch"),exclude:{rooted:[".git",".cache"],names:["node_modules","target"]},fenceDirectory:".semio-fence",
 ports:{graph:createGraphPort({root,scanBinary:nativeToolV1(bin,"semio-repo-watch")}),execution,custody:createCustodyPort()},log:(kind,detail)=>{if(flags.includes("--verbose"))process.stderr.write("[service] "+kind+" "+detail+"\n");}});
const started=await service.start();
executors.top();
const run=async()=>{
 const out=[],err=[];
 const result=await forwardCommandV1({root,command:"run",args:[target],cwd:root,languages,environment:process.env,stdout:{write:text=>out.push(String(text))},stderr:{write:text=>err.push(String(text))}});
 return{...result,out:out.join(""),err:err.join("")};
};
const report=row=>process.stdout.write(JSON.stringify({load:load(),...row})+"\n");
const timings=text=>text.split("\n").filter(line=>line.startsWith('{"executor"')).map(line=>JSON.parse(line).executor);
try{
 const settle=Date.now()+60000;
 while(executors.idleCount()<size&&Date.now()<settle)await new Promise(resolve=>setTimeout(resolve,50));
 const prime=await run();
 report({what:"prime: first run executes the whole closure through the executors",handled:prime.handled,exit:prime.exit,served:prime.served,refused:prime.refused??null,tasks:timings(prime.out).length,files:started.files,owners:started.owners,executors:executors.stats()});
 if(!prime.handled||prime.exit!==0)throw new Error("prime run failed: "+JSON.stringify({handled:prime.handled,reason:prime.reason,refused:prime.refused,exit:prime.exit,err:prime.err.slice(0,500)}));
 const noop=[];for(let i=0;i<20;i++){const t=performance.now();await run();noop.push(performance.now()-t);}
 report({what:"warm no-op after the executor run, in-process client (ms)",roundTrip:percentiles(noop)});
 if(mode==="warm"||mode==="both"){
  const toTool=[],wall=[],admit=[],check=[],tasks=[],fence=[],plan=[],spawn=[];
  for(let i=0;i<count;i++){
   const owner=closure[(i*7919)%closure.length],file=join(root,"pkgs",owner,"src","f"+(i%10)+".ts");
   clearStamps(stamps);const t0=nowUs();appendFileSync(file,"// edit "+i+"\n");
   const t=performance.now(),r=await run();wall.push(performance.now()-t);
   if(r.exit!==0)throw new Error("edit run failed "+JSON.stringify({exit:r.exit,refused:r.refused,err:r.err.slice(0,300)}));
   const times=Object.values(readStamps(stamps)),first=Math.min(...times),rows=timings(r.out);
   toTool.push((first-t0)/1000);tasks.push(rows.length);admit.push(Math.min(...rows.map(row=>row.admitMs)));check.push(Math.min(...rows.map(row=>row.checkMs)));
   const s=r.serviceTiming??{};fence.push(s.fenceMs??0);plan.push(s.planMs??0);spawn.push(s.spawnMs??0);
  }
  report({what:"warm: edit one file, run; first tool start measured from the tool's own stamp (ms)",target,editToToolStartMs:percentiles(toTool),clientWallMs:percentiles(wall),firstAdmissionMs:percentiles(admit),firstLaunchChecksMs:percentiles(check),serviceFenceMs:percentiles(fence),servicePlanMs:percentiles(plan),serviceSpawnMs:percentiles(spawn),tasksExecuted:percentiles(tasks),executors:executors.stats()});
 }
 if(mode==="cold"||mode==="both"){
  const toTool=[],wall=[],load_=[],door=[],admit=[],check=[],total=[];
  for(let i=0;i<Math.min(count,15);i++){
   const owner=closure[(i*104729)%closure.length],file=join(root,"pkgs",owner,"src","f"+(i%10)+".ts"),id=owner+":build";
   await run();clearStamps(stamps);
   const t0=nowUs();appendFileSync(file,"// cold edit "+i+"\n");
   const t=performance.now(),r=await timed("bun",[join(here,"cold.mjs"),id],{cwd:root,env:environment});wall.push(performance.now()-t);
   const row=JSON.parse(r.stdout.trim().split("\n").filter(line=>line.startsWith('{"cold"')).pop()).cold,times=Object.values(readStamps(stamps));
   toTool.push((Math.min(...times)-t0)/1000);load_.push(row.loadMs);door.push(row.doorMs);admit.push(row.admitMs);check.push(row.checkMs);total.push(row.loadMs+row.doorMs+row.admitMs+row.checkMs);
  }
  report({what:"cold: the same segment in a fresh process, edit to first tool start (ms); no service, no executor",editToToolStartMs:percentiles(toTool),processWallMs:percentiles(wall),moduleLoadMs:percentiles(load_),doorMs:percentiles(door),admissionMs:percentiles(admit),launchChecksMs:percentiles(check),segmentMs:percentiles(total)});
 }
}catch(error){
 process.stderr.write("[bench] failed: "+String(error?.stack??error)+"\n");process.exitCode=1;
}finally{
 await service.stop("bench");await executors.close();
 const audit=existsSync(runtime.audit)?readFileSync(runtime.audit,"utf8").trim().split("\n"):[];
 report({what:"audit journal rows written by the core",rows:audit.length,sample:audit.slice(0,flags.includes("--audit")?audit.length:0).map(line=>JSON.parse(line))});
 for(const path of[join(overlay,"pkgs"),join(overlay,"graph.json"),join(overlay,".out"),join(overlay,".semio-fence"),scratch])rmSync(path,{recursive:true,force:true});
 process.exit(process.exitCode??0);
}
