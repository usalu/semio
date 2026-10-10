#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { brepExtensionRetirementOracle, geometryInferenceOracle, channelIdentityOracle } from "../../🧪️tests/🔬️extension-guest-standalone/🟦️.ts";
import { runRepositoryCargoTests, runRepositoryExactCargoLaws, runBun, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}`);
    console.log(`geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}`);
    console.log(`channel-identity-oracle cases=${channelIdentityOracle(import.meta.dir)}`);
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-s-plugin-flow-extension-brep"], this.repoRoot, this.invocation.control, rest);
  }
}

class MeshOracleScript extends BundleScript {
  async run(segments:string[]):Promise<void> {
    if (segments.length) throw new Error("test-mesh-oracle takes no arguments");
    runBun(["test","../../🥽️mesh/🧪️tests/🔬️unit/🟦️.ts"],import.meta.dir);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("canonical-architecture accepts only --oracle-only");
    console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}`);
    console.log(`geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}`);
    console.log(`channel-identity-oracle cases=${channelIdentityOracle(import.meta.dir)}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3600000), lawMilliseconds: 600000 }, cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" }, nativeEnv: { RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-s-plugin-flow-extension-brep", target: { kind: "lib" }, laws: [
        "bundle_identity_matches_catalogue_fixture",
        "extension_guest_retires_actual_session_geometry_and_inflight_tessellation",
        "extension_bundle_extends_flow_and_evaluates_box",
        "named_geometry_inference_owns_worker_invocation_and_dependency_identity",
        "named_geometry_inference_standard_gateway_uses_registered_extension_context",
        "named_geometry_inference_resumes_with_progress_matches_output_and_cancels",
        "mesh_output_jobs_bound_work_and_cancel_every_output_phase",
        "mesh_output_encoding_cancels_through_the_existing_graph_owner",
        "mesh_mirror_uses_the_existing_retained_modeling_owner",
        "mesh_merge_uses_retained_pair_and_corner_work",
        "mesh_subdivision_retains_concave_triangulation_and_centroid_work",
        "mesh_existing_synchronous_routes_record_runtime_costs",
        "mesh_analysis_retains_topology_tessellation_and_measurement",
        "mesh_generation_refuses_expansion_before_reconstruction",
        "mesh_expensive_routes_step_through_the_existing_operator_owner",
        "mesh_from_brep_retains_tessellation_import_normals_and_output",
        "polygon_attributes_share_portable_fixture_and_preserve_owned_assets",
        "mesh_output_metadata_retains_bounded_values_and_cancellation",
        "mesh_json_export_retains_polygon_metadata_without_preview_work",
        "mesh_affine_operator_owns_matrix_contract_and_retained_output",
      ] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, progress(event) { console.log(`brep-extension-retirement ${event.stage}: ${event.law ?? ""}`); } });
    console.log(`brep-extension-retirement receipts=${receipts.length}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("test-mesh-oracle", MeshOracleScript).register("package", PackageScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
