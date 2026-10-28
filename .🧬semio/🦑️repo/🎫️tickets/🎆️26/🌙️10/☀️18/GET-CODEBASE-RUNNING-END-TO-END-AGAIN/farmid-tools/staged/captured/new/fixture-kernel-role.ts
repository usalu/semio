/** 🧪️ A process that enters the shared verb kernel entry with an authenticated proof and the graph farm worker role as its whole command line, the way a farm worker started from a snapshot does; a verb body that runs is a failure. */
import{createHash}from"node:crypto";
import{runCapturedRepositoryKernelV1}from"../../../../../../../⌨️command/📥️entry/🟦️.ts";
import{buildSourceClosureStreamLoaderV1}from"../../../../../../../📥️source/🔏️closure/🟦️.ts";
import{GRAPH_FARM_WORKER_ROLE_V1}from"../../../🪆️captured/🟨️.mjs";

const[root,digest]=process.argv.slice(2),loader=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex");
await runCapturedRepositoryKernelV1([GRAPH_FARM_WORKER_ROLE_V1],{version:1,root:root!,snapshot:null,sha256:digest!,loaderSha256:loader,recheck:()=>undefined},digest,loader,async()=>{throw Error("A verb body ran for the graph farm worker role");});
