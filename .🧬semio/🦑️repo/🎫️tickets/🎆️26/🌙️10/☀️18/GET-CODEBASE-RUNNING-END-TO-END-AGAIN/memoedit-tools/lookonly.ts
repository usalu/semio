/** 📏️ Read-only probe: one edit lookup in a fresh process against a scratch memo that `livedit.ts` sealed (the record's cursor is replayed with irrelevant rows), to measure the lookup's own wall, CPU and peak RSS. Usage: bun lookonly.ts <code root> <live root> <scratch dir> <edited relative path> */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const [code,live,scratch,edited]=process.argv.slice(2) as [string,string,string,string];
const L=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",M=L+"/🧭️invocation/🕸️graph/🧠️memo";
const rss0=process.memoryUsage().rss,started=performance.now();
const memo=await import(M+"/🟦️.ts"),command=await import(M+"/🚪️command/🟦️.ts");
const directory=join(scratch,"memo"),record=JSON.parse(readFileSync(join(directory,readdirSync(directory).find(name=>name.startsWith("dddd"))!),"utf8")),seq=Number(String(record.cursor).split(":").pop());
const rows:[string,number,string][]=Array.from({length:seq},()=>["m",0,"nowhere/at/all"] as [string,number,string]);rows.push(["m",0,edited]);
const journal={excluder:()=>()=>false,cursor:async()=>({epoch:"feedfacefeedface",seq:rows.length}),since:async(from:{epoch:string;seq:number})=>({exact:true as const,cursor:{epoch:"feedfacefeedface",seq:rows.length},rows:rows.slice(from.seq)})};
const producer=await import(L+"/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;const owner=String((producer.prepareSourceAnalysesV1() as {rust:{id:string}}).rust.id).split(":v1:")[1];
const ports=command.graphMemoEditPortsV1(live),edit={...ports,rows:async(index:string,ids:readonly number[])=>(await ports.rows(index,ids))?.map((row:{kind:string;fact:any})=>row.kind==="analysis"?{...row,fact:{...row.fact,provider:String(row.fact.provider).replace(/:v1:[0-9a-f]{64}$/u,":v1:"+owner),source:{...row.fact.source,mtimeMs:1,sha256:"0".repeat(64)}}}:row)};
const loaded=performance.now(),cpu0=process.cpuUsage();
const answer=await (await memo.openGraphMemoV1({root:live,directory,client:journal as never,key:"d".repeat(64),insensitive:true,holds:(list:never)=>command.memoFactsHoldV1(live,list),edit:edit as never})).lookup();
const usage=process.cpuUsage(cpu0);
console.log("[DEBUG] lookup",JSON.stringify({hit:answer.hit,edited:(answer as {edited?:unknown}).edited,importMs:Math.round(loaded-started),lookupMs:Math.round(performance.now()-loaded),lookupCpu:+((usage.user+usage.system)/1e6).toFixed(2),rssStartMb:Math.round(rss0/1048576),rssEndMb:Math.round(process.memoryUsage().rss/1048576)}));
