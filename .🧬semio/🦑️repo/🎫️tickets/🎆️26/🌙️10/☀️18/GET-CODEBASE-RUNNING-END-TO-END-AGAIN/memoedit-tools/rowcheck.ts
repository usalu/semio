/** 🔬️ Read-only probe: the rows a sealed scratch memo names for one path, each checked by the planner's fact check of the given code root, with the analysis provider revision it carries (provider ids mapped to the code root's, as in livedit.ts). Usage: bun rowcheck.ts <code root> <live root> <scratch dir> <relative path> */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const [code,live,scratch,path]=process.argv.slice(2) as [string,string,string,string];
const M=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo";
const memo=await import(M+"/🟦️.ts"),command=await import(M+"/🚪️command/🟦️.ts");
const directory=join(scratch,"memo"),record=JSON.parse(readFileSync(join(directory,readdirSync(directory).find(name=>name.startsWith("dddd"))!),"utf8"));
const edits=memo.unpackMemoEditIndexV1(readFileSync(join(directory,record.relevance.edit.id+".rix")))!,ids=edits.named(path);
const producer0=await import(code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await producer0.libraryBootstrap;const owner=String((producer0.prepareSourceAnalysesV1() as {rust:{id:string}}).rust.id).split(":v1:")[1];
const rows=(await command.graphMemoEditPortsV1(live).rows(record.relevance.edit.index,ids))?.map((row:any)=>row.kind==="analysis"?{...row,fact:{...row.fact,provider:String(row.fact.provider).replace(/:v1:[0-9a-f]{64}$/u,":v1:"+owner)}}:row);
for(const row of rows??[])console.log("[DEBUG]",row.kind,String(row.path).slice(-60),row.kind==="analysis"?JSON.stringify({provider:(row.fact as any).provider,revision:(row.fact as any).revision}):"",await command.memoFactsHoldV1(live,[row]));
const producer=await import(code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;
console.log("[DEBUG] registered",JSON.stringify(Object.fromEntries(Object.entries(producer.prepareSourceAnalysesV1()).map(([key,value]:[string,any])=>[key,value?.id+" "+value?.revision?.slice(0,12)]))));
