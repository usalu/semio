#!/usr/bin/env bun
import assert from "node:assert/strict";
import {resolve} from "node:path";
import {BundleScript,ScriptRouter,scriptInvocationBudget} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {readCargoTestPolicyV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
/** 📍️ Verifies both actual dimensional node owner implementations and independent schema projections. */
class NodeScript extends BundleScript{
 async run(args:string[]):Promise<void>{assert(args.length===1&&["source","native"].includes(args[0]!),"test requires source or native");const source=resolve(this.root,"../../🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"node:source:types",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});(await import("../../🧪️tests/🟦️.ts")).nodeOracle();if(args[0]==="source")return;const p=readCargoTestPolicyV1(process.env),name="semio-s-fem-node";await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,cargoTargetDir:p.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{[name]:resolve(this.root,"Cargo.toml")},groups:[{package:name,target:{kind:"lib",name:"semio_s_fem_node"},cargoArgs:["--offline"],laws:["planar","volumetric"].map(d=>`${d}::tests::native_node_original_receiving_projection_and_every_cancelled_frontier`)}]});}
}
await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",NodeScript),{invocation}));
