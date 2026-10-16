/** 🚦️ Evaluates the source conditions of the five hub GIS inference gates on the current tree, plus the pre-relocation conditions of the two re-pointed gates (expected to fail). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const runtime = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🏃️runtime", "🦀️.rs"));
const gisBinding = readModuleTreeSource(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"));
const sqlite = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🪶️sqlite", "🦀️.rs"), "utf8");
const wal = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧾️wal", "🦀️.rs"), "utf8");
const mcp = readFileSync(join(repoRoot, "🧰️framework", "\u{1F6CD}️products", "💻️os", "🔨️modules", "🌉️mcp", "🔀️dispatch", "🦀️.rs"), "utf8");
const catalog = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "📇️catalog", "🦀️.rs"), "utf8");
const trusted = readFileSync(join(repoRoot, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog", "🦀️.rs"), "utf8");
const hubBin = readModuleTreeSource(join(repoRoot, "🌎️hub", "🏗️bootstrap", "🦀️.rs"));
const results: Array<[string, boolean, string]> = [];
const gate = (name: string, body: () => string | null) => { const failure = body(); results.push([name, failure === null, failure ?? ""]); };
const committerSymbols = ["RetainedGisMapApprovalCommitterV1", "RetainedGisMapDocumentStateV1", "DurableOwnedThreeStoreMapAssemblyV1", "durable_group_journal_sink", "InferenceWalVerifierV1", "parse_frontier_chain_hash", "GisMapApprovalCheckpointPublisherV1", "checkpoint_applied", "GisMapApprovalIngressAuthorityV1", "GisMapApprovalRequestOwnerV1", "GisMapPreparedApprovalV1", "GisMapAbandonedRequestTaskV1", "GisMapPreparedApprovalTaskV1", "resume_parked_requests", "resume_parked_prepared", "drive_abandoned_request", "drive_abandoned_prepared", "abandon_prepared_approval", "owners.fence.clone()", "RetainedGisMapDocumentStateV1::Publishing"];
const boundary = (storeConstruction: string, drawingSource: string, extra: boolean): string | null => {
  const retainedCommitter = runtime.slice(runtime.indexOf("pub struct RetainedGisMapApprovalCommitterV1"), runtime.indexOf("/// 🗺️ The server-materialized Map base"));
  const verify = retainedCommitter.slice(retainedCommitter.indexOf("async fn verify("), retainedCommitter.indexOf("async fn commit_retained("));
  const publishCall = verify.indexOf("self.publish_checkpoint(");
  const reconcileCall = verify.indexOf(".reconcile_committed_approval(");
  const checkpointAppliedCall = verify.indexOf(".checkpoint_applied(");
  const documentWriteRelease = verify.indexOf("drop(document_write)");
  const failed = !runtime.includes("const GIS_MAP_COMMITTER_CAPACITY: usize = 64;") || !storeConstruction.includes('artifact_kind: "s.gis.gismap"') || !storeConstruction.includes('artifact_kind: "s.stdio.semio"') || !storeConstruction.includes('slot: "drawing"') || !storeConstruction.includes('slot: "value"') || !drawingSource.includes("let drawing_id = snapshot.drawing.child_id.clone();") || !drawingSource.includes("let value_id = snapshot.value.child_id.clone();") || (extra && !retainedCommitter.includes("B::member_stores(&key, snapshot, actor)")) || retainedCommitter.includes("ArtifactHandle::submit") || publishCall < 0 || reconcileCall <= publishCall || !retainedCommitter.includes("document_write: GisMapDocumentWriteLeaseV1") || checkpointAppliedCall <= reconcileCall || documentWriteRelease <= checkpointAppliedCall;
  return failed ? "capacity, write authority, typed journal or publication-before-apply boundary drifted" : null;
};
gate("proposal-approval (after)", () => {
  if (runtime.includes("semio_s_artifact_") || !runtime.includes("impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B>")) return "runtime names an artifact crate";
  for (const symbol of ["impl ComposedMapCommitBindingV1 for GisMapInferenceBindingV1", "impl MapInferenceProposalBindingV1 for GisMapInferenceBindingV1", "gis_map_parent_stamped_one_item_preparation_factory"]) if (!gisBinding.includes(symbol)) return `binding missing ${symbol}`;
  for (const symbol of [...committerSymbols, "B::parent_preparation_factory(", "sole committed WAL event"]) if (!runtime.includes(symbol)) return `runtime missing ${symbol}`;
  const boundaryFailure = boundary(gisBinding.slice(gisBinding.indexOf("fn parent_store("), gisBinding.indexOf("pub fn gis_map_region_preview(")), gisBinding, true);
  if (boundaryFailure) return boundaryFailure;
  const registration = hubBin.slice(hubBin.indexOf("let inference_runtime = match gis_map_binding.as_ref()"), hubBin.indexOf("let inference_ready = inference_runtime.is_some()"));
  if (!registration.includes("RetainedGisMapApprovalCommitterV1::new") || !registration.includes("GisMapApprovalCheckpointPublisherV1Impl") || registration.includes("UnavailableGisMapApprovalCommitterV1")) return "production registration";
  return null;
});
gate("proposal-approval (pre-relocation conditions on the current tree)", () => {
  for (const symbol of [...committerSymbols, "gis_map_parent_stamped_one_item_preparation_factory", "sole committed WAL event"]) if (!runtime.includes(symbol)) return `runtime missing ${symbol}`;
  const retainedCommitter = runtime.slice(runtime.indexOf("pub struct RetainedGisMapApprovalCommitterV1"), runtime.indexOf("/// 🗺️ The server-materialized Map base"));
  return boundary(retainedCommitter.slice(retainedCommitter.indexOf("fn parent_store("), retainedCommitter.indexOf("fn derived_children(")), retainedCommitter, false);
});
const frozenRequired = (needle: string) => ["VerifiedGisMapArtifactBindingV1", "Arc<VerifiedTrustedCatalog>", "selected_document_open", needle, "executable_identity", "component_blake3", "descriptor_byte_sha256", "execution_protocol", "parent_dialect", "DocumentOpenSurfaceRoleV1::Editor", "grant.read", "grant.write", "grant.observe", "semio.hub.gis-map-frozen-binding/v1\\0"];
gate("frozen-binding (after)", () => {
  if (!readFileSync(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"), "utf8").includes("semio_s_artifact_gis_gismap::gis_map_inference_service()")) return "binding executable";
  const missing = frozenRequired("proposals.inference_service()").filter((needle) => !catalog.includes(needle));
  if (missing.length) return `catalog missing ${missing}`;
  if (!trusted.includes("pub fn selected_document_open") || !hubBin.includes("verified_catalog: Option<Arc<VerifiedTrustedCatalog>>") || !hubBin.includes("gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>")) return "startup retention";
  return null;
});
gate("frozen-binding (pre-relocation conditions on the current tree)", () => {
  const missing = frozenRequired("gis_map_inference_service").filter((needle) => !catalog.includes(needle));
  return missing.length ? `catalog missing ${missing}` : null;
});
gate("approval-undo (unchanged gate)", () => {
  const ok = runtime.includes("GisMapApprovalUndoTargetV1") && runtime.includes("undo_gis_map_approval") && runtime.includes("expected_current") && runtime.includes("checkpoint_applied") && sqlite.includes("inference_approval_undo_v1") && sqlite.includes("committed_witness_digest") && sqlite.includes("idempotency_key") && sqlite.includes("original_command") && wal.includes("approval_undo_witness_digest") && mcp.includes("HubInferenceApproval(HubInferenceApprovalUndoMemberV1)") && mcp.includes("HistoryUndoPort") && !runtime.includes("client_inverse") && !/pub struct HubInferenceApprovalUndoMemberV1 \{[^}]*\b(?:instance|inverse)/u.test(mcp);
  return ok ? null : "undo source boundary";
});
gate("retained-runtime (unchanged gate)", () => {
  const supervisor = runtime.slice(runtime.indexOf("struct InferenceRunKeyV1"), runtime.indexOf("/// 🏃️ The hub's only inference authority"));
  const worker = runtime.slice(runtime.indexOf("async fn finish_retained_gis_map_job"), runtime.indexOf("fn owner_page_receipt"));
  const ok = ["RetainedInferenceRunSupervisorV1", "InferenceRunKeyV1", "reap_finished_operations", "same_run", "signal_and_join_operations"].every((symbol) => supervisor.includes(symbol)) && ["spawn_blocking", "fresh_now_ms", "compare_frozen", "check_live_inference_author"].every((symbol) => worker.includes(symbol)) && !runtime.includes("operations.insert(job_id.to_owned(), control)") && !runtime.includes("handle.abort()") && sqlite.includes("pub fn heartbeat(") && sqlite.includes("lease_expires_at") && sqlite.includes("now >= lease") && sqlite.includes("current_epoch != run_epoch");
  return ok ? null : "supervisor/heartbeat/join";
});
gate("checkpoint-control (unchanged gate)", () => {
  for (const symbol of ["pub fn open_inherited", "write_inference_checkpoint_control_frame", "read_inference_checkpoint_control_frame", "GisInferenceCheckpointControlFrameV1", "INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES: usize = 256", "control.checkpoint(control.progress().0)", "progress_persisted(job_id, progress_cursor, completed, total)"]) if (!runtime.includes(symbol)) return `runtime missing ${symbol}`;
  if (!hubBin.includes('std::env::var_os("OS_HUB_TEST_INFERENCE_CHECKPOINT_FD")') || !hubBin.includes('descriptor.to_str() != Some("4")') || !hubBin.includes("mode != HubMode::Development")) return "integration fixture gate";
  return null;
});
for (const [name, ok, why] of results) console.log(ok ? "PASS" : "FAIL", name, why);
