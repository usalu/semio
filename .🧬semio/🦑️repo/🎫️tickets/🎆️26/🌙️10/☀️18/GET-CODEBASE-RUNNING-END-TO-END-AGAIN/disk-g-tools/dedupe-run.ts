import {dedupeTreeV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🟦️.ts";
import {cacheRunnerPortsV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts";
const [base,mode,quiet="3600000",minimum="1048576",maximum="400000"]=process.argv.slice(2);
if(!base||!["dry","apply"].includes(mode??""))throw Error("usage: dedupe-run.ts <base> dry|apply [quietMs] [minimumBytes] [maximumFiles]");
const controller=new AbortController();process.on("SIGTERM",()=>controller.abort(Error("terminated")));
const ports=cacheRunnerPortsV1({signal:controller.signal,progress:async stage=>{const rss=process.memoryUsage().rss;if(rss>500*1024*1024){controller.abort(Error("[DEBUG] rss above 500 MB: "+rss));}console.error("[DEBUG] "+stage+" rss="+Math.round(rss/1048576)+"MB");},exclusive:async()=>({release:()=>{}})});
const outcome=await dedupeTreeV1({base,minimumBytes:Number(minimum),quietMs:Number(quiet),maximumFiles:Number(maximum),dryRun:mode==="dry",language:"en",write:line=>console.log(line)},ports);
console.log(JSON.stringify({groups:outcome.plan.groups,files:outcome.plan.files,planned:outcome.plan.replacements.length,reclaimableBytes:outcome.plan.reclaimableBytes,replaced:outcome.replaced,freedBytes:outcome.freedBytes,skipped:outcome.skipped,failed:outcome.failed.length,partial:outcome.partial}));
