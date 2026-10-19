import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {mkdirSync} from "node:fs";
const api=await import(process.env.BZL_STAGE_ADAPTER!);
await receiveScriptProcessInvocation(process.env,async original=>{
 mkdirSync(process.env.BZL_STAGE_WS!,{recursive:true});
 const impl=await api.publishBazelImplementationV1(process.cwd(),process.env.BZL_STAGE_WS!,original);
 console.log("implementation ok",impl.sha256.slice(0,12));
});
