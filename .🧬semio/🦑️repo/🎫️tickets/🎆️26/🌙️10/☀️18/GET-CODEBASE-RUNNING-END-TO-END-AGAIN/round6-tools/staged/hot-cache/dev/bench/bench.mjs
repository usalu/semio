#!/usr/bin/env bun
/** 📊 Prototype benchmarks against a running service: noop | direct | edit | watch | legacy | concurrency. */
import {appendFileSync,readFileSync,writeFileSync} from "node:fs";
import {realpathSync} from "node:fs";
import {forwardCommandV1} from "../resident/client/index.mjs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {clearStamps,closureOf,load,nowUs,percentiles,readStamps,round,timed} from "./lib.mjs";
import {benchLanguagesV1} from "./environment.mjs";

const[,, mode,rawRoot,...flags]=process.argv,root=realpathSync(rawRoot);
const flag=(name,fallback)=>{const at=flags.indexOf(name);return at>=0?flags[at+1]:fallback;};
const count=Number(flag("--n","100")),stamps=flag("--stamps",process.env.STAMPS??join(tmpdir(),"semio-bench-stamps")),languages=benchLanguagesV1(flag("--languages"));
const graph=JSON.parse(readFileSync(root+"/graph.json","utf8"));
const client=["bun",fileURLToPath(new URL("../resident/client/main.mjs",import.meta.url)),"--root",root,"--languages",languages.join(","),"--timing","--"];
const target=flag("--target","p1499:build");
const goBinary=flag("--client","bun")==="go"?(process.env.SEMIO_REPO_BINARY??(()=>{throw new Error("--client go needs SEMIO_REPO_BINARY");})()):undefined;
const invoke=(verb,id)=>goBinary?timed(goBinary,[verb,id],{cwd:root,env:{...process.env,SEMIO_LANGUAGE:languages[0]}}):timed(client[0],[...client.slice(1),verb,id]);
const parseTiming=text=>{try{return JSON.parse(text.trim().split("\n").pop());}catch{return{client:{resultMs:0},service:{}};}};
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
const report=row=>process.stdout.write(JSON.stringify({mode,load:load(),...row})+"\n");

if(mode==="noop"){
 const prime=await invoke("run",target);
 const walls=[],resultMs=[],service=[],fence=[];
 for(let i=0;i<count;i++){
  const r=await invoke("run",target);
  walls.push(r.ms);const t=parseTiming(r.stderr);resultMs.push(t.client.resultMs);service.push(t.service?.totalMs??0);fence.push(t.service?.fenceMs??0);
 }
 report({what:"warm no-op, bun client process included (ms)",target,wall:percentiles(walls),clientConnectToResult:percentiles(resultMs),serviceTotal:percentiles(service),serviceFence:percentiles(fence),primed:prime.stderr.includes("memo")||prime.stdout.includes("Up")});
}
if(mode==="direct"){
 await forwardCommandV1({root,command:"run",args:[target],cwd:root,languages,stdout:{write(){}},stderr:{write(){}}});
 const rt=[],st=[];
 for(let i=0;i<count;i++){
  const started=performance.now();
  const r=await forwardCommandV1({root,command:"run",args:[target],cwd:root,languages,stdout:{write(){}},stderr:{write(){}}});
  rt.push(performance.now()-started);st.push(r.serviceTiming.totalMs);
 }
 report({what:"warm no-op, in-process client, no process spawn (ms)",roundTrip:percentiles(rt),serviceTotal:percentiles(st)});
}
if(mode==="edit"){
 const delay=Number(flag("--delay","0")),closure=closureOf(graph,target.split(":")[0]);
 await invoke("run",target);
 const toExec=[],wall=[],tasks=[],spawn=[],plan=[],fence=[],run=[];
 for(let i=0;i<count;i++){
  const owner=closure[(i*7919)%closure.length],file=root+"/pkgs/"+owner+"/src/f"+(i%20)+".ts";
  clearStamps(stamps);
  const t0=nowUs();appendFileSync(file,"// edit "+i+"\n");const tWrite=nowUs();
  if(delay)await sleep(delay);
  const r=await invoke("run",target);
  const times=Object.values(readStamps(stamps)),first=Math.min(...times);
  const t=parseTiming(r.stderr).service??{};
  toExec.push((first-t0)/1000);wall.push(r.ms);tasks.push(times.length);spawn.push(t.spawnMs??0);plan.push(t.planMs??0);fence.push(t.fenceMs??0);run.push(t.runMs??0);
 }
 report({what:"edit one file then run, first tool process start measured from the tool's own stamp",delayMs:delay,target,editToToolStartMs:percentiles(toExec),clientWallMs:percentiles(wall),tasksExecuted:percentiles(tasks),serviceFenceMs:percentiles(fence),servicePlanMs:percentiles(plan),serviceSpawnMs:percentiles(spawn),serviceRunMs:percentiles(run)});
}
if(mode==="watch"){
 const closure=closureOf(graph,target.split(":")[0]);
 clearStamps(stamps);
 const{spawn}=await import("node:child_process");
 const child=spawn(client[0],[...client.slice(1),"watch",target],{stdio:["ignore","pipe","pipe"]});
 await sleep(3000);
 const toExec=[];
 for(let i=0;i<count;i++){
  const owner=closure[(i*104729)%closure.length],file=root+"/pkgs/"+owner+"/src/f"+(i%20)+".ts";
  clearStamps(stamps);await sleep(400);
  const t0=nowUs();appendFileSync(file,"// watch edit "+i+"\n");
  let first;for(let waited=0;waited<5000;waited+=2){await sleep(2);const times=Object.values(readStamps(stamps));if(times.length){first=Math.min(...times);break;}}
  toExec.push(first?(first-t0)/1000:NaN);
  await sleep(1500);
 }
 child.kill("SIGINT");
 report({what:"watch mode: edit to first tool process start (ms)",target,editToToolStartMs:percentiles(toExec.filter(Number.isFinite)),missed:toExec.filter(v=>!Number.isFinite(v)).length});
}
if(mode==="concurrency"){
 const clients=Number(flag("--clients","16")),per=Number(flag("--per","50")),all=[],refused=[];
 await forwardCommandV1({root,command:"run",args:[target],cwd:root,languages,stdout:{write(){}},stderr:{write(){}}});
 const started=performance.now();
 await Promise.all(Array.from({length:clients},async(_,c)=>{
  for(let i=0;i<per;i++){
   const s=performance.now();
   const r=await forwardCommandV1({root,command:"run",args:[target],cwd:root,languages,lane:"agent-"+c,stdout:{write(){}},stderr:{write(){}}});
   all.push(performance.now()-s);if(r.refused)refused.push(r.refused);
  }
 }));
 const total=performance.now()-started;
 report({what:"concurrent in-process clients, warm no-op",clients,perClient:per,requests:all.length,wallMs:round(total),requestsPerSecond:round(all.length/(total/1000)),latency:percentiles(all),refused:refused.length});
}
