// 🛑 Drives the hand-started persistent worker through a protocol cancel request and prints one JSON result line.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const root=process.cwd(),at=(path:string)=>join(root,path);
const {receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(at("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const {issueRepositoryScriptProvidersV1}=await import(at("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
const api=await import(join(import.meta.dir,"live/script.ts"));
issueRepositoryScriptProvidersV1(process.env);
const graph=JSON.parse(readFileSync(process.env.BW_GRAPH!,"utf8")),task=process.env.BW_TASKS!,base=process.env.BW_BASE!;
let result:unknown;
await receiveScriptProcessInvocation(process.env,async (original:any)=>{
 const phase=createIssuedResourceControlV1(original,"bworkers:cancel",0);
 result=await api.probeBazelWorkerCancelScratchV1(root,graph,{tasks:[task],args:[],parallel:2,skipCache:true,workers:true},original,phase,{bazel:process.env.BW_BAZEL!,workspace:join(base,"ws"),outputBase:join(base,"ob"),installBase:"/private/tmp/bworkers/install",diskCache:join(base,"disk"),environment:{...process.env},options:[]},{task,afterMilliseconds:Number(process.env.BW_AFTER??6000),leafMarker:process.env.BW_MARKER!});
});
console.log("BW_RESULT "+JSON.stringify(result));
