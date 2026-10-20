/** Stock-Nx proof of the pinned generation receipt on the fx2 fixture: cold graph build, receipt, receipt-gated reuse gives the same graph faster, an nx.json edit withdraws the receipt.
 * usage: bun verify-generation-stock.ts <overlay-or-live-library-dir> */
import {spawnSync} from "node:child_process";
import {appendFileSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";

const library=process.argv[2]!,module=await import(join(library,"⚡️caching/🕸️graph/🔏️seal/📌️generation/🟦️.ts"));
const root="/private/tmp/conc-nx/fx2",data=join(root,".nx/workspace-data"),environment={SEMIO_BUILD_GRAPH_DIGEST:"a".repeat(64),SEMIO_NX_IMPLEMENTATION_IDENTITY:"b".repeat(64),SEMIO_NX_TOOLING_IDENTITY:"c".repeat(64)};
function nx(reuse:boolean){
 const started=performance.now(),run=spawnSync("node",["node_modules/nx/dist/bin/nx.js","show","projects","--json"],{cwd:root,encoding:"utf8",env:{...process.env,NX_DAEMON:"false",NX_TUI:"false",NX_NO_CLOUD:"true",NX_ISOLATE_PLUGINS:"false",NX_FORCE_REUSE_CACHED_GRAPH:reuse?"true":"false"},maxBuffer:1<<28});
 return {seconds:(performance.now()-started)/1000,output:run.stdout,status:run.status};
}
rmSync(join(root,".nx"),{recursive:true,force:true});
const cold=nx(false),claim=(await module.nxGenerationClaimV1(root,environment))!;
console.log("cold build seconds",cold.seconds.toFixed(1),"status",cold.status);
console.log("receipt before build holds",await module.nxGenerationHoldsV1(root,data,claim));
console.log("receipt written",await module.writeNxGenerationReceiptV1(root,data,claim));
const holds=await module.nxGenerationHoldsV1(root,data,claim),reused=nx(holds);
console.log("holds",holds,"reuse seconds",reused.seconds.toFixed(1),"same projects",JSON.stringify(JSON.parse(cold.output).sort())===JSON.stringify(JSON.parse(reused.output).sort()),"speedup",(cold.seconds/reused.seconds).toFixed(1)+"x");
const original=readFileSync(join(root,"nx.json"),"utf8");
writeFileSync(join(root,"nx.json"),JSON.stringify({...JSON.parse(original),cli:{}}));
console.log("after nx.json edit holds",await module.nxGenerationHoldsV1(root,data,await module.nxGenerationClaimV1(root,environment)));
writeFileSync(join(root,"nx.json"),original);
console.log("after restoring nx.json holds",await module.nxGenerationHoldsV1(root,data,claim));
const graph=join(data,"project-graph.json");appendFileSync(graph," ");
console.log("after graph file change holds",await module.nxGenerationHoldsV1(root,data,claim));
