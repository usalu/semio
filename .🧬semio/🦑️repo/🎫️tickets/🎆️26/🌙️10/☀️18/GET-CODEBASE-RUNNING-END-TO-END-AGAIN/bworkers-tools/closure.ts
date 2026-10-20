// 📦 Lists the captured implementation closure of the Bazel adapter (relative to the checkout) for the scratch-root copy.
import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureBuildSourceClosureV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import {relative,join} from "node:path";
const root=process.cwd(),entry=process.env.BW_ENTRY??relative(root,join(import.meta.dir,"staged/adapter.ts"));
await receiveScriptProcessInvocation(process.env,async original=>{
 const captured=await captureBuildSourceClosureV1(root,entry,original);
 await Bun.write(process.env.BW_OUT!,JSON.stringify({entry:captured.entry,files:captured.files.map((f:any)=>({path:f.path,bytes:Buffer.from(f.base64,"base64").length}))}));
 console.log(captured.files.length,"files",captured.files.reduce((n:number,f:any)=>n+Buffer.from(f.base64,"base64").length,0),"bytes");
});
