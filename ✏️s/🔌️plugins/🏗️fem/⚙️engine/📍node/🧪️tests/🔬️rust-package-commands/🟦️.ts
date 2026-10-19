import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { scriptInvocationBudget } from "@semio-tech/process/routing";
import { runOwnedCommand } from "@semio-tech/process/owned-execution";
import { readCargoTestPolicyV1 } from "@semio-tech/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/testing/cargo/exact";
import { runAdmittedExactCargoLawsV1 as runExactCargoLaws } from "@semio-tech/process/cargo-test";
import { resolveTypescriptCompilerV1 } from "@semio-tech/process/execution";

/** 🏃️ Runs the `NodeScript` command under the caller's roots and invocation. */
export async function runNodeScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {assert(args.length===1&&["source","native"].includes(args[0]!),"test requires source or native");const source=resolve(context.root,"../../🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(context.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],context.repoRoot,"node:source:types",scriptInvocationBudget(context.invocation,120000),{signal:context.invocation.control.signal});(await import("../🟦️.ts")).nodeOracle();if(args[0]==="source")return;const p=readCargoTestPolicyV1(process.env),name="semio-s-fem-node";await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(context.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(context.invocation,300000)},cwd:context.repoRoot,manifestPaths:{[name]:resolve(context.root,"Cargo.toml")},groups:[{package:name,target:{kind:"lib",name:"semio_s_fem_node"},cargoArgs:["--offline"],laws:["planar","volumetric"].map(d=>`${d}::tests::native_node_original_receiving_projection_and_every_cancelled_frontier`)}]});}
