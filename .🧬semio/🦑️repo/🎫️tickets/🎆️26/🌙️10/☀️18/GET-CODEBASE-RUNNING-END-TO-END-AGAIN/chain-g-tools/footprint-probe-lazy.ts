import "./kernel-lazy.ts";
import{mkdirSync,readdirSync,readFileSync}from"node:fs";
export async function runCapturedBuildSourceV1(args:readonly string[],proof:{recheck:()=>void}){
 proof.recheck();Bun.gc(true);
 const dir="/private/tmp/chain-g-trace-"+process.pid;mkdirSync(dir,{recursive:true});(globalThis as any)[Symbol.for("semio.sourceTrace.v1")].enable(dir);
 const file=readdirSync(dir)[0]!,lines=readFileSync(dir+"/"+file,"utf8").split("\n"),modules=lines.filter(line=>line.startsWith("m ")).length;
 const after=process.memoryUsage();
 console.log("FOOTPRINT "+JSON.stringify({modules,rssMB:Math.round(after.rss/1048576),heapUsedMB:Math.round(after.heapUsed/1048576)}));
}
