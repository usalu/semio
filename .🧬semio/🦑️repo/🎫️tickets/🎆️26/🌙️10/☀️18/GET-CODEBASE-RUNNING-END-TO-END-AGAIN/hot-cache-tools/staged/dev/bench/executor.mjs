#!/usr/bin/env bun
/** 🏭 Benchmark executor: the real executor loop and the real authority core (admission, launch checks) with a stand-in tool and no custody. Started by the executor pool with fd 3 as its control channel. */
import {createReadStream,writeSync} from "node:fs";
import {serveExecutorV1,readLinesV1} from "../resident/executor/index.mjs";
import {readFileSync} from "node:fs";
import {loadCoreV1,runAdmittedCommandV1,withHeldInvocationV1} from "./segment.mjs";
import {closureOf} from "./lib.mjs";

const root=process.env.BENCH_ROOT,overlay=process.env.BENCH_OVERLAY,tool=process.env.BENCH_TOOL,stampDirectory=process.env.BENCH_STAMPS,workMs=Number(process.env.BENCH_WORK??"30");
const core=await loadCoreV1(overlay),graph=JSON.parse(readFileSync(root+"/graph.json","utf8"));
await withHeldInvocationV1(core,root,process.env,async({held,admission})=>{
 await serveExecutorV1({
  lines:readLinesV1(createReadStream(null,{fd:3,autoClose:false,encoding:"utf8"})),
  write:text=>writeSync(3,text),
  limits:{maximumRequests:1e9,maximumAgeMilliseconds:1e9},
  openSession:async()=>({run:async body=>body({}),close:async()=>undefined}),
  run:async request=>{
   const timing=await runAdmittedCommandV1({core,root,held,admission,request:{token:request.client?.token??"",agent:request.client?.agent??"",cwd:request.cwd,args:request.argv,named:request.client?.named},tool,stampDirectory,workMs});
   console.log(JSON.stringify({executor:timing}));
   if(timing.exit!==0)process.exitCode=timing.exit;
  },
 });
},async named=>new Set(named.flatMap(name=>closureOf(graph,name))));
process.exit(0);
