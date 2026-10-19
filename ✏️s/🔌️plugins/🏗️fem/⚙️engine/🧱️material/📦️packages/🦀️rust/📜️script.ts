#!/usr/bin/env bun
import assert from "node:assert/strict";
import {resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {BundleScript,ScriptRouter,scriptInvocationBudget} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {readCargoTestPolicyV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
/** 🧱️ Checks both actual dimension-specific Material owners before native table admission and closure. */
class MaterialScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  assert(args.length===1&&["source","native"].includes(args[0]!),"test requires source or native");
  const oracle=await import("../../📋️table/🧪️tests/🟦️.ts");oracle.materialTableOracle();
  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env),laws=["native_material_receives_canonical_and_authored_original_values","native_material_receives_actual_declared_text_ordinals","native_material_refuses_invalid_parts_without_dropping_partial_fields","table::native_material_table_receives_original_order_and_closes_cancelled_frontiers","table::native_material_ranked_insert_remove_set_and_inverse_preserve_original_rows","table::native_material_reorder_preserves_original_field_issuers_and_inverse_order","patch::native_material_sparse_patch_preserves_original_field_authorities_and_all_cancelled_frontiers"],packageName="semio-s-fem-material";await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,cargoTargetDir:p.targetDirectory,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{[packageName]:resolve(this.root,"Cargo.toml")},groups:[{package:packageName,target:{kind:"lib",name:"semio_s_fem_material"},cargoArgs:["--offline"],laws:["planar","volumetric"].flatMap(d=>laws.map(law=>`${d}::native_material_tests::${law}`))}]});}
 }
}
await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",MaterialScript),{invocation}));