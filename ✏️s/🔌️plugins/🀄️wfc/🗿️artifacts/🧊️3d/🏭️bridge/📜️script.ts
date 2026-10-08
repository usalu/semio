#!/usr/bin/env bun
/** 🏭️ Runs only this artifact's contributed production mutation inventory. */
import {resolve} from "node:path";
import {runMutationInventoryCargoProducerCommand} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🟦️.ts";
import {readMutationInventoryExecutionPolicy} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {readCargoTestPolicyV1,runCargoTestsV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
const manifestPath=resolve(import.meta.dir,"../📦️packages/🦀️rust/Cargo.toml"),cwd=resolve(import.meta.dir,"..");
if(process.argv[2]==="check"){
  if(process.argv.length!==3)throw Error("check accepts no overrides");
  const controller=new AbortController(),cancel=()=>controller.abort(),policy=readMutationInventoryExecutionPolicy(process.env);
  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  if(process.env.CARGO_TARGET_DIR!==policy.compilerStorage.targetDirectory||process.env.CARGO_BUILD_BUILD_DIR!==policy.compilerStorage.buildDirectory)throw Error("Compiler custody differs from the authored child policy");
  const lease=await acquireCargoBuildLeaseV1({directory:policy.compilerStorage.leaseDirectory,buildDirectory:policy.compilerStorage.buildDirectory,args:process.argv.slice(2),signal:controller.signal,onWait:()=>console.error("[mutation-inventory] waiting for child compiler lease")});
  try{await runCargoTestsV1({manifestPath,packages:["semio-s-artifact-wfc-3d"],cwd,extraArgs:["--offline","--locked","--features","mutation-inventory","--bin","semio-wfc-3d-mutation-bridge"],environment:process.env,signal:controller.signal},readCargoTestPolicyV1(process.env));}
  finally{lease.release();process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
}else await runMutationInventoryCargoProducerCommand({manifestPath,binary:"semio-wfc-3d-mutation-bridge",cwd});
