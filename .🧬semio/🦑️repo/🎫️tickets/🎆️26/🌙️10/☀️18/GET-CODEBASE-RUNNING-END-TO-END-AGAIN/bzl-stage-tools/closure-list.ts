import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureBuildSourceClosureV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import {writeFileSync} from "node:fs";
const entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
await receiveScriptProcessInvocation(process.env,async original=>{
 const captured:any=await captureBuildSourceClosureV1(process.cwd(),entry,original);
 const paths=(captured.files??captured.entries??[]).map((row:any)=>row.path);
 writeFileSync(process.argv[2]!,paths.join("\n")+"\n");
 console.log("closure files",paths.length,Object.keys(captured));
});
