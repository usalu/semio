/** 📏️ Graph-free measurement on the LIVE generation (read-only): with the batch's code (run from a mirror), rebuilds the sidecar of a live sealed memo record including the edit index (cost of the sealing child), seals a scratch copy of the record, then answers one-line-edit journal rows with the real ports (rows from the live fact index, the live frontier, the planner's fact check with the source analyses registered); an analysis row is forced to replay its parser (recorded digest blanked) so the parse cost is measured too, and its provider id is mapped to the mirror's (the id embeds the module URL; in the product the executor and the planner child load the same paths). Usage (inside the mirror): bun livedit.ts <code root> <live root> <record.json> <scratch dir> <edited relative path> */
import {mkdirSync,readFileSync,writeFileSync,statSync} from "node:fs";
import {join} from "node:path";
const [code,live,recordFile,scratch,edited]=process.argv.slice(2) as [string,string,string,string,string];
const L=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const memo=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts");
const relevance=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts");
const command=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🚪️command/🟦️.ts");
const {readChangeStateV1}=await import(code+"/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts");
const rss=()=>Math.round(process.memoryUsage().rss/1048576),cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1e6;};
const step=async<T>(name:string,body:()=>Promise<T>|T):Promise<T>=>{const started=performance.now(),before=cpu();const value=await body();console.log("[DEBUG]",name.padEnd(28),JSON.stringify({ms:Math.round(performance.now()-started),cpu:+(cpu()-before).toFixed(3),rss:rss()}));return value;};
const record=JSON.parse(readFileSync(recordFile,"utf8"));
const engine=generation.createGenerationStoreV1({root:join(live,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest);if(!manifest)throw Error("[DEBUG] manifest retired");
mkdirSync(scratch,{recursive:true});
const state=readChangeStateV1(join(recordFile,"../.."))!,excluded=memo.memoExcluderV1(state.exclude,true),stem=join(scratch,"side");
const built=await step("seal: index view + slice",async()=>{
 const frontier=await engine.readFrontier(manifest.frontier),index=await engine.readIndexView(manifest.factIndex),ids=relevance.graphMemoSliceRowIdsV1(index,{digest:record.manifest,owners:manifest.owners},record.pin) as number[];
 const out=await step("seal: classify (with edits)",()=>relevance.buildGraphMemoRelevanceV1({root:live,rows:function*(){for(const id of ids)yield index.rows[id];},frontier,excluded,isLink:async()=>false}));
 const pointerFile=join(scratch,"current.json");writeFileSync(pointerFile,JSON.stringify({generation:manifest.generation,manifest:record.manifest}));
 await step("seal: write sidecar files",()=>relevance.writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:manifest.generation,manifest:record.manifest},out,true,{index:manifest.factIndex,frontier:manifest.frontier,ids}));
 engine.dropIndexes();return out;
});
console.log("[DEBUG] sidecar",JSON.stringify({rows:built.rows,named:built.edits.named.paths.length,inside:built.edits.inside.paths.length,above:built.edits.above.size,frontier:built.edits.frontier.size,members:built.edits.members.size,rixBytes:statSync(stem+".rix").size,relBytes:statSync(stem+".rel").size}));
const EPOCH="feedfacefeedface",rows:[string,number,string][]=[];
const journal={excluder:()=>excluded,cursor:async()=>({epoch:EPOCH,seq:rows.length}),since:async(from:{epoch:string;seq:number})=>({exact:true as const,cursor:{epoch:EPOCH,seq:rows.length},rows:rows.slice(from.seq)})};
const producer=await import(L+"/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;
const owner=String((producer.prepareSourceAnalysesV1() as {rust:{id:string}}).rust.id).split(":v1:")[1];
const ports=command.graphMemoEditPortsV1(live),forced={...ports,rows:async(index:string,ids:readonly number[])=>{const found=await ports.rows(index,ids);return found?.map((row:{kind:string;fact:any})=>row.kind==="analysis"?{...row,fact:{...row.fact,provider:String(row.fact.provider).replace(/:v1:[0-9a-f]{64}$/u,":v1:"+owner),source:{...row.fact.source,mtimeMs:1,sha256:"0".repeat(64)}}}:row);}};
const open=(edit:unknown)=>memo.openGraphMemoV1({root:live,directory:join(scratch,"memo"),client:journal as never,key:"d".repeat(64),insensitive:true,holds:(list:never)=>command.memoFactsHoldV1(live,list),edit:edit as never});
const plan=readFileSync(join(recordFile,"..",record.plan.sha256+".plan"),"utf8");
console.log("[DEBUG] seal",await (await open(forced)).seal({plan,pin:record.pin,projects:record.projects}),await (await open(forced)).seal({plan,pin:record.pin,projects:record.projects,sidecar:stem+".json"}));
const cut=edited.lastIndexOf("/"),directory=edited.slice(0,cut);
const cases:[string,[string,number,string][]][]=[["no change",[]],["in-place edit",[["m",0,edited]]],["atomic save",[["a",0,edited+".tmp"],["r",0,edited+".tmp"],["m",0,edited],["m",1,directory]]]];
for(const [name,list] of cases){
 rows.push(...list);
 const base=await step(name+": base lookup",async()=>(await open(undefined)).lookup());
 console.log("[DEBUG]",name,"base",JSON.stringify(base).slice(0,160));
 const answer=await step(name+": edit lookup",async()=>(await open(forced)).lookup());
 const {plan:_plan,pin:_pin,...shown}=answer as Record<string,unknown>;console.log("[DEBUG]",name,"edit",JSON.stringify(shown).slice(0,240));
}
const warm=await step("in-place again (warm)",async()=>{rows.push(["m",0,edited]);return(await open(forced)).lookup();});
console.log("[DEBUG] warm hit",(warm as {hit:boolean}).hit);
