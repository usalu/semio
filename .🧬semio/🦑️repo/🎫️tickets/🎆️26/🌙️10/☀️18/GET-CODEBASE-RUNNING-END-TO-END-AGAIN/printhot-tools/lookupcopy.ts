/** 🔁️ Graph-free end-to-end cost of a memo lookup with the batch's carry port, on a COPY of a live record: `bun lookupcopy.ts <module-root> <live-root> <memo-dir> <record-name> <scratch-dir> [carry|base]`. Copies the record and its plan/relevance/recheck/stamp files into the scratch directory, then runs `openGraphMemoV1().lookup()` from `<module-root>` (a mirror with batch `carry`) against the live journal and generation store; only the scratch copy is written. Prints the answer (without the plan), wall, CPU and RSS. */
import {copyFileSync,mkdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
const [moduleRoot,root,directory,name,scratch,mode="carry"]=process.argv.slice(2) as [string,string,string,string,string,string?];
const L=join(moduleRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),{carryGraphMemoPinV1}=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🧳️carry/🟦️.ts")),journal=await import(join(L,"🏡️resident/📜️journal/🟦️.ts")),endpoint=await import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs")),observation=await import(join(L,"👁️observation/🟨️.mjs"));
const record=JSON.parse(readFileSync(join(directory,name+".json"),"utf8")),policy=memo.GRAPH_MEMO_POLICY_V1;
mkdirSync(scratch,{recursive:true});
for(const file of [name+".json",record.plan.sha256+policy.planSuffix,record.relevance.id+policy.relevanceSuffix,record.relevance.id+policy.recheckSuffix,record.relevance.stamps+policy.stampSuffix])copyFileSync(join(directory,file),join(scratch,file));
const t=performance.now(),c=process.cpuUsage();
const opened=await memo.openGraphMemoV1({root,directory:scratch,client:journal.connectChangeJournalV1(endpoint.canonicalRootV1(root),process.env),key:record.key,environment:process.env,
 holds:(rows:unknown[],signal?:AbortSignal)=>observation.observationsHold(rows,64,new Set(),true,signal),
 ...(mode==="carry"?{carry:(input:any,signal?:AbortSignal)=>carryGraphMemoPinV1({generations:join(root,".cache/b/g/m/generations"),...input,...(signal?{signal}:{})})}:{})});
const answer=await opened.lookup(),cpu=process.cpuUsage(c);
const {plan,pin,...rest}=answer as any;
console.log(JSON.stringify({mode,answer:rest,planBytes:plan?.length,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((cpu.user+cpu.system)/1000),rssMb:Math.round(process.memoryUsage().rss/1048576)}));
