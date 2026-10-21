#!/usr/bin/env bun
/** 🥶 The same segment in a fresh process: what every rebuild pays without the resident executor (module load, the real door, admission, launch checks, first tool). Prints the timings; the token is minted from the service key like a launcher does. */
import {readFileSync} from "node:fs";
import {loadCoreV1,runAdmittedCommandV1,withHeldInvocationV1} from "./segment.mjs";
import {closureOf} from "./lib.mjs";

const started=performance.now(),root=process.env.BENCH_ROOT,overlay=process.env.BENCH_OVERLAY,id=process.argv[2];
const core=await loadCoreV1(overlay),loaded=performance.now(),graph=JSON.parse(readFileSync(root+"/graph.json","utf8"));
await withHeldInvocationV1(core,root,process.env,async({held,admission,runtime})=>{
 const doored=performance.now(),key=core.session.readServiceKeyV1(runtime.key);
 const worktree=core.session.worktreeIdV1(core.session.canonicalRootV1(root));
 const token=core.session.mintCapabilityV1(key,[{k:"scope",v:["run"]},{k:"notAfter",v:Date.now()+60000},{k:"worktree",v:worktree},{k:"agent",v:"bench"}]);
 const timing=await runAdmittedCommandV1({core,root,held,admission,request:{token,agent:"bench",cwd:root,args:["run",id],named:[id.split(":")[0]]},tool:process.env.BENCH_TOOL,stampDirectory:process.env.BENCH_STAMPS,workMs:Number(process.env.BENCH_WORK??"30")});
 console.log(JSON.stringify({cold:{...timing,loadMs:loaded-started,doorMs:doored-loaded}}));
});
process.exit(0);
