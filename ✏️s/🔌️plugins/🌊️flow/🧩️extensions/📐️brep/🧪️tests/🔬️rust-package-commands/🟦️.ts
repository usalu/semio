import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/framework/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/framework/process/cargo-laws";
import { brepExtensionRetirementOracle, geometryInferenceOracle, channelIdentityOracle } from "../🔬️extension-guest-standalone/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/framework/process/cargo-test";

/** 🏃️ Runs the `CanonicalArchitectureScript` command under the caller's roots and invocation. */
export async function runCanonicalArchitectureScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("canonical-architecture accepts only --oracle-only");
  console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(context.root)}`);
  console.log(`geometry-inference-oracle cases=${geometryInferenceOracle(context.root)}`);
  console.log(`channel-identity-oracle cases=${channelIdentityOracle(context.root)}`);
  if (segments.includes("--oracle-only")) return;
  const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3600000), lawMilliseconds: 600000 },cwd: context.repoRoot,env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },nativeEnv: { RUST_MIN_STACK: "268435456" },groups: [{ package: "semio-s-plugin-flow-brep", target: { kind: "lib" }, laws: [
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
    ] }],progress(event) { console.log(`brep-extension-retirement ${event.stage}: ${event.law ?? ""}`); }});
  console.log(`brep-extension-retirement receipts=${receipts.length}`);
}
