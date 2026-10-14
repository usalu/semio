#!/usr/bin/env bun
/** 🏭️ Runs only this artifact's contributed production mutation inventory. */
import {resolve} from "node:path";
import {runMutationInventoryCargoProducerCommand} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🟦️.ts";
import {readMutationInventoryExecutionPolicy} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
import {readCargoTestPolicyV1,runCargoTestsV1} from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
const manifestPath=resolve(import.meta.dir,"📦️packages/🦀️rust/Cargo.toml"),cwd=resolve(import.meta.dir,"..");
if(process.argv[2]==="check"){
  if(process.argv.length!==3)throw Error("check accepts no overrides");
  const controller=new AbortController(),cancel=()=>controller.abort(),policy=readMutationInventoryExecutionPolicy(process.env);
  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  if(process.env.CARGO_TARGET_DIR!==policy.compilerStorage.targetDirectory||process.env.CARGO_BUILD_BUILD_DIR!==policy.compilerStorage.buildDirectory)throw Error("Compiler custody differs from the authored child policy");
  try{await runCargoTestsV1({manifestPath,packages:["semio-s-artifact-wfc-grid2d-mutation-bridge"],cwd,extraArgs:["--offline","--locked","--bin","semio-wfc-grid2d-mutation-bridge","--","--nocapture"],environment:process.env,signal:controller.signal},readCargoTestPolicyV1(process.env));}
  finally{process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
}else await runMutationInventoryCargoProducerCommand({manifestPath,binary:"semio-wfc-grid2d-mutation-bridge",cwd});
