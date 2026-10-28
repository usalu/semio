/** ⏱️ Read-only probe: the sealing cost (classify + write the sidecar files) of a live sealed memo record's slice with the code of a given root, for a base-versus-batch comparison. Usage: bun sealcost.ts <code root> <live root> <record.json> <scratch stem> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [code,live,recordFile,stem]=process.argv.slice(2) as [string,string,string,string];
const L=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs"),memo=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts"),relevance=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts");
const {readChangeStateV1}=await import(code+"/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts");
const cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1e6;},rss=()=>Math.round(process.memoryUsage().rss/1048576);
const record=JSON.parse(readFileSync(recordFile,"utf8")),engine=generation.createGenerationStoreV1({root:join(live,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest),frontier=await engine.readFrontier(manifest.frontier),index=await engine.readIndexView(manifest.factIndex),ids=relevance.graphMemoSliceRowIdsV1(index,{digest:record.manifest,owners:manifest.owners},record.pin) as number[];
const excluded=memo.memoExcluderV1(readChangeStateV1(join(recordFile,"../.."))!.exclude,true);
let at=cpu();const built=await relevance.buildGraphMemoRelevanceV1({root:live,rows:function*(){for(const id of ids)yield index.rows[id];},frontier,excluded,isLink:async()=>false});const classify=cpu()-at;
at=cpu();relevance.writeGraphMemoSidecarFilesV1(stem,{file:"/x",generation:manifest.generation,manifest:record.manifest},built,true,{index:manifest.factIndex,frontier:manifest.frontier,ids});const write=cpu()-at;
console.log("[DEBUG] seal",JSON.stringify({classify:+classify.toFixed(2),write:+write.toFixed(2),rss:rss()}));
