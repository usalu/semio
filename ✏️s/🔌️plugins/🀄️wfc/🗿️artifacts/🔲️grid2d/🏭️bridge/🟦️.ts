import { createIssuedResourceControlV1 } from "@semio-tech/framework/process/routing";
import {join} from "node:path";
import { runRepositoryMutationBridgeScriptV1 } from "@semio-tech/framework/test/mutation/inventory/execution";
import { runAdmittedCargoCommandV1 } from "@semio-tech/framework/process/cargo-test";
import { findWorkspaceRoot } from "@semio-tech/framework/process/routing";
import { receiveScriptProcessInvocation } from "@semio-tech/framework/process/routing";
import collector from "./🔣️.json" with {type:"json"};

const packageRoot=join(import.meta.dir,"📦️packages/🦀️rust"),binary="semio-wfc-grid2d-mutation-bridge";
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  if(process.argv[2]==="check"){
   if(process.argv.length!==3)throw new Error("check accepts no overrides");
   await receiveScriptProcessInvocation(process.env,original=>runAdmittedCargoCommandV1(original,createIssuedResourceControlV1(original,"cargo:bridge",0),findWorkspaceRoot(),["test","--manifest-path",join(packageRoot,"Cargo.toml"),"--offline","--locked","--bin",binary,"--","--nocapture"],import.meta.dir,process.env,collector.maximumMilliseconds));
  }else await runRepositoryMutationBridgeScriptV1(packageRoot,binary,{...collector,version:1});
}
