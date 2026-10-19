#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { runAdmittedExactCargoLawsV1 as runExactCargoLaws } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
/** 📦️ procedural-generation3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { generation3dTerminologySelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { generation3dGraphKeyboardSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dMeshSelectionSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥽️edit-mesh-selection/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dWidgetInputSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dWidgetCreationSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🟦️.ts";
import { resolve } from "node:path";
import {configuredExactCargoLawPolicyV1} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { testGeneration3dIoAuthorityFixture } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔁️round-trip/🛡️authority/🟦️.ts";
import { assertGeneration3dSemanticWire } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧬️semantic-wire/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget, receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "original-widget-output") {
      const testRoot=join(this.repoRoot,"✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/💎️value/🧪️tests");
      if(segments[1]==="native") await runExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),manifestPaths:{"semio-s-artifact-procedural-3d-geometry-compute":join(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/📦️packages/🦀️rust/Cargo.toml"),"semio-s-artifact-procedural-3d-geometry-inputs":join(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔌️inputs/📦️packages/🦀️rust/Cargo.toml"),"semio-s-artifact-procedural-3d-geometry":join(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/📦️packages/🦀️rust/Cargo.toml"),"semio-framework-value-derive":join(this.repoRoot,"🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml")},env:{...process.env,CARGO_BUILD_BUILD_DIR:readCargoTestPolicyV1(process.env).buildDirectory},invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,3600000),lawMilliseconds:600000},cwd:this.repoRoot,cargoArgs:["--offline"],groups:[{package:"semio-s-artifact-procedural-3d-geometry-compute",target:{kind:"lib"},laws:["original_widget_output_moves_once_on_same_caller_and_retires_all_fields","original_widget_output_cancel_keeps_nested_payload_for_same_caller_retirement","original_math_values_construct_actual_outputs_close_inputs_and_conserve_system","original_math_value_cancellation_retains_partial_output_for_paid_close","original_geometry_fixed_output_reads_actual_mesh_and_nurbs_on_same_caller","original_geometry_plane_constructs_actual_topology_and_conserves_cancelled_backing","original_geometry_brep_line_constructs_shape_and_conserves_cancelled_shared_root","original_geometry_mesh_translation_copies_seven_owners_and_preserves_metadata_on_same_caller","original_geometry_mesh_translation_cancel_closes_all_seven_copied_owners_on_same_caller"]},{package:"semio-s-artifact-procedural-3d-geometry-inputs",target:{kind:"lib"},laws:["original_widget_inputs_constructor_quotes_utf8_pages_and_moves_actual_values_once"]},{package:"semio-s-artifact-procedural-3d-geometry",target:{kind:"lib"},laws:["original_geometry_inferred_field_resolves_real_graph_and_closes_every_dynamic_owner","original_geometry_inferred_field_keeps_actual_output_on_denied_handoff_and_cancel","original_geometry_engine_keeps_completed_rows_without_cloning_and_closes_snapshot_on_its_caller","original_geometry_number_policy_uses_explicit_host_and_closes_actual_inputs","original_geometry_cyclic_plan_publishes_real_localized_faults_on_same_caller","original_geometry_cyclic_fault_keeps_partial_utf8_on_refusal_and_paid_cancel","original_geometry_analytic_curves_create_read_and_close_real_graph_on_same_caller","original_geometry_mesh_edge_graph_publishes_three_actual_outputs_and_closes_on_same_caller","original_geometry_plane_surface_graph_creates_and_reads_actual_shape_on_same_caller","original_geometry_analytic_brep_translation_preserves_labels_and_real_graph_source_on_same_caller","original_geometry_dictionary_lookup_borrows_actual_utf8_entry_under_same_caller"]},{package:"semio-framework-value-derive",target:{kind:"test",name:"generic_retained_owner"},laws:["retained_generic_composite_fields_bound_owned_parameters_without_restricting_cold_declaration"]}]});
      else await runRepositoryTestCommand(process.execPath,["test",join(testRoot,"🟦️.ts"),join(testRoot,"../../🔌️inputs/🧪️tests/🟦️.ts"),join(testRoot,"../../🔢️math-values/🧪️tests/🎟️original/🟦️.ts"),join(testRoot,"../../⚙️compute/🎟️original/🧪️tests/🟦️.ts"),join(testRoot,"../../🚀️original-field/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,120000),signal:this.invocation.control.signal});
      return;
    }
    if (segments[0] === "geometry-context") {
      const testRoot=join(this.repoRoot,"✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any");
      runCmd("bun",["test",`${testRoot}/🚪️io/📐️geometry/🧪️tests/🟦️.ts`,`${testRoot}/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🔬️oracle/🟦️.ts`],{cwd:this.repoRoot});
      runCmd("bun",[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--skipLibCheck",`${testRoot}/🚪️io/📐️geometry/🧪️tests/🟦️.ts`],{cwd:this.repoRoot});
      if(segments[1]==="native")await runArtifactRustTests("semio-s-artifact-procedural-3d-geometry", this.repoRoot, ["--lib","geometry_compute_context_","--","--nocapture"], this.invocation);
      return;
    }
    if (segments[0] === "document-restoration-oracle") {
      const test = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/📄️document-restoration/🟦️.ts");
      runCmd("bun", ["test", test], {cwd: this.repoRoot});
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", "--resolveJsonModule", test], {cwd: this.repoRoot});
      return;
    }
    if (segments[0] === "generation3d-widget-inputs") {
      console.log(`generation3d-mesh-selection checks=${generation3dMeshSelectionSelfTests()} independentAjv=true`);
      console.log(`generation3d-widget-creation checks=${generation3dWidgetCreationSelfTests()} independentAjv=true`);
      console.log(`generation3d-terminology checks=${generation3dTerminologySelfTests()}`);
      const testRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input");
      await runRepositoryTestCommand(process.execPath, [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", "--resolveJsonModule", `${testRoot}/🧪️tests/🔬️unit/🟦️.ts`,join(testRoot,"../🥽️edit-mesh-selection/🧪️tests/🔬️unit/🟦️.ts")], { cwd: this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,120000),signal:this.invocation.control.signal });
      if(segments[1]==="native")for (const name of ["semio-s-artifact-procedural-3d-io-text","semio-s-artifact-procedural-3d-set-widget-input"]) await runArtifactRustTests(name,this.repoRoot,["--lib","widget_input_","--","--nocapture"],this.invocation);
      return;
    }
    if (segments[0] === "semantic-wire") {
      const { assertGeneration3dSemanticWire } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧬️semantic-wire/🟦️.ts");
      console.log(`generation3d-semantic-wire checks=${assertGeneration3dSemanticWire()} independentAjv=true`);
      if (segments[1] === "native") await runCargo(["test", "-p", "semio-s-artifact-procedural-generation3d", "--test", "host-laws", "semantic_wire_vectors", "--", "--nocapture"], this.repoRoot);
      return;
    }
if (segments[0] === "generation3d-document-io") {
      await runArtifactRustTests("semio-s-artifact-procedural-generation3d", this.repoRoot, ["quick", "--offline", "--test", "io-round-trip", "artifact_surface"], this.invocation);
      if (segments[1] === "native") await runArtifactRustTests("semio-s-artifact-procedural-generation3d", this.repoRoot, ["quick", "--offline", "--test", "editor-routes", "document_io"], this.invocation);
      return;
    }
if (segments[0] === "generation3d-preview-window-transient") {
      const testRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient");
      const { testGeneration3dPreviewWindowTransientContract } = await import(`${testRoot}/🧪️tests/🔬️contract/🟦️.ts`);
      testGeneration3dPreviewWindowTransientContract();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧬️schema/🟦️.ts`, `${testRoot}/🧪️tests/🔬️contract/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", ...["semio-s-artifact-procedural-3d-preview-eval", "semio-s-artifact-procedural-3d-mode-edit", "semio-s-artifact-procedural-3d-editor", "semio-s-artifact-procedural-3d-viewer"].flatMap((name) => ["-p", name]), "--lib", "preview_eval_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

/** 🧩️ Every package of the generation3d family, read from the workspace members. */
const familyRoot = resolve(import.meta.dir, "../..");
const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
if (process.argv[2] === "canonical-io") {
  console.log(`generation3d-io-authority checks=${testGeneration3dIoAuthorityFixture()}`);
  await receiveScriptProcessInvocation(process.env, invocation=>runArtifactRustTests("semio-s-artifact-procedural-generation3d", resolve(import.meta.dir, "../../../../../../.."), ["quick", "--locked", "--test", "io-round-trip"], invocation));
} else await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation3d", { packages: familyPackages, ...{ snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
  snapshotSqliteTestBudgetMs: 120000,
  twins: [
    { name: "generation3d-semantic-wire", run: assertGeneration3dSemanticWire },
    { name: "generation3d-mesh-selection", run: generation3dMeshSelectionSelfTests },
    { name: "generation3d-widget-input", run: generation3dWidgetInputSelfTests },
    { name: "generation3d-widget-creation", run: generation3dWidgetCreationSelfTests },
    { name: "generation3d-terminology", run: generation3dTerminologySelfTests },
    { name: "generation3d-snapshot-fixture-asset", run: generation3dSnapshotFixtureAssetSelfTests },
    { name: "generation3d-graph-keyboard", run: generation3dGraphKeyboardSelfTests },
  ],
}, commands: router });
