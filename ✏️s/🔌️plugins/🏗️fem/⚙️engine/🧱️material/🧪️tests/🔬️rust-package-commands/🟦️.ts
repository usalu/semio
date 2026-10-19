import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { scriptInvocationBudget } from "@semio-tech/process/routing";
import { readCargoTestPolicyV1 } from "@semio-tech/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/testing/cargo/exact";
import { runAdmittedExactCargoLawsV1 as runExactCargoLaws } from "@semio-tech/process/cargo-test";

/** 🏃️ Runs the `MaterialScript` command under the caller's roots and invocation. */
export async function runMaterialScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
assert(args.length===1&&["source","native"].includes(args[0]!),"test requires source or native");
const oracle=await import("../../📋️table/🧪️tests/🟦️.ts");oracle.materialTableOracle();
if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env),laws=["native_material_receives_canonical_and_authored_original_values","native_material_receives_actual_declared_text_ordinals","native_material_refuses_invalid_parts_without_dropping_partial_fields","table::native_material_table_receives_original_order_and_closes_cancelled_frontiers","table::native_material_ranked_insert_remove_set_and_inverse_preserve_original_rows","table::native_material_reorder_preserves_original_field_issuers_and_inverse_order","patch::native_material_sparse_patch_preserves_original_field_authorities_and_all_cancelled_frontiers"],packageName="semio-s-fem-material";await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(context.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(context.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:context.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{[packageName]:resolve(context.root,"Cargo.toml")},groups:[{package:packageName,target:{kind:"lib",name:"semio_s_fem_material"},cargoArgs:["--offline"],laws:["planar","volumetric"].flatMap(d=>laws.map(law=>`${d}::native_material_tests::${law}`))}]});}
 }
