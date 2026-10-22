import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/⌨️command/📥️entry/🟦️.ts",[
('''import{withImplementationPinV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
''','''import{withImplementationPinV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
import{exitIfStaleImplementationV1}from"../../🧷️format-stamps/🟨️.mjs";
'''),
('''await receiveScriptProcessInvocation(process.env,original=>{const control=createIssuedResourceControlV1(original,"repository:command",0);return withIssuedResourceOperationV1(control,()=>withImplementationPinV1(()=>body(proof.root,args,original,control)));});proof.recheck();}''','''try{await receiveScriptProcessInvocation(process.env,original=>{const control=createIssuedResourceControlV1(original,"repository:command",0);return withIssuedResourceOperationV1(control,()=>withImplementationPinV1(()=>body(proof.root,args,original,control)));});}catch(error){exitIfStaleImplementationV1(error);throw error;}proof.recheck();}'''),
])
patch(L+"🏗️build/📜️script.ts",[
('''import { readBuildMode } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
''','''import { readBuildMode } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { runWithStaleReexecV1 } from "./🧷️format-stamps/🔄️reexec/🟨️.mjs";
'''),
('''if(import.meta.main)await runBuildCliV1(findWorkspaceRoot(import.meta.dir),process.argv.slice(2));''','''/** ♻️ Re-executes the outermost command once with the current implementation when it yielded because a declared format stamp moved; a nested command only passes the exit code on. */
export async function runBuildMainV1(root:string,argv:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {
  const{SCRIPT_PROCESS_INVOCATION_ENV}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
  const outcome=await runWithStaleReexecV1(()=>runBuildCliV1(root,argv,environment),{environment,outermost:environment[SCRIPT_PROCESS_INVOCATION_ENV]===undefined});
  if(outcome.status!=="completed")process.exit(outcome.code);
}

if(import.meta.main)await runBuildMainV1(findWorkspaceRoot(import.meta.dir),process.argv.slice(2));'''),
])
