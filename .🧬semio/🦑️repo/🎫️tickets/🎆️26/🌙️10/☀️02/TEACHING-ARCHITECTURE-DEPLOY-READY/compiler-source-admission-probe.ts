import{resolve}from"node:path";
import{writeFileSync}from"node:fs";
import{captureCompilerBuildSourceClosureV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🏗️compiler/🟦️.ts";
import{buildSourceClosureTextV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const workspace=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts",output=resolve(import.meta.dir,"🗑️generated/compiler-source-admission-probe.json");
try{await receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"compiler-source-admission:read-only",0),snapshot=await captureCompilerBuildSourceClosureV1(workspace,entry,original,control),serialized=buildSourceClosureTextV1(snapshot);
 const summary={version:1,status:"captured",files:snapshot.files.length,sourceBytes:snapshot.files.reduce((sum,row)=>sum+Buffer.from(row.base64,"base64").byteLength,0),snapshotBytes:Buffer.byteLength(serialized.text),sha256:serialized.sha256};
 writeFileSync(output,JSON.stringify(summary,null,2)+"\n");console.log(JSON.stringify(summary));
 });}catch(error){const summary={version:1,status:"refused",message:error instanceof Error?error.message:String(error)};writeFileSync(output,JSON.stringify(summary,null,2)+"\n");console.error(JSON.stringify(summary));process.exitCode=1;}
