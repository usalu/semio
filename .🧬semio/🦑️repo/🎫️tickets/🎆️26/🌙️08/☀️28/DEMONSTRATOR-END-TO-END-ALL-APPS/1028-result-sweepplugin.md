# sweepplugin result

STATUS: RED (stopped at CHECKPOINT 22:55). PARTIAL (native --lib --tests). Framework-workspace ui batches fw_fix1..3 GREEN after fixes; full re-sweep (fw_fix3, root_aa, root_ab, fw_aa, fw_ab) running via 1028-tools/sweepplugin/all.sh; wasm32-wasip2 --lib sweep not yet run; plugin-mounted-owner (standalone workspace, excluded from root) not yet checked.

Tools: C/1028-tools/sweepplugin/{run.sh,all.sh,errs.py,batch_*}. Note: repo cargo refuses >~32 KB argv (build source closure) -> batches of <=70 `-p`; crates live in two workspaces (Cargo.toml root members, 🧰️framework/Cargo.toml).

## Fixed (crate: cause -> fix)
- ui-scene-mesh (test): `Mesh3dSlot` private in pub(super) field -> `pub(super) struct`.
- ui-retained-tree-tree (test): stale `use crate::tree::NodeFlags` in reconcile unit test -> removed (already via `super::*`).
- ui-runtime-reconcile-primitives (test): `SurfaceTreeRetireCursor.owner` private -> `pub(crate)`.
- ui-contract-conformance (test): external test read private `handle`/`root_key` -> added public observers `UiDocumentLease::slot_epoch`, `UiResidentPermit::root_slot_epoch`; test uses them. (SHARED CHANGE: ui-contract public API +2 read-only fns)
- ui-runtime-presentation (test native_resource_custody): imports from facade lacking mesh-message exports -> import from `semio_framework_ui_runtime_presentation`.
- ui-prepared-staging (test): close steps now `Result<InteractiveJobCloseStep,_>`, `Refused` gone -> unwrap/drop arm; added `Close` alias import in abandoned test.
- ui-prepared-laws (test): `extern crate self` alias shadowed the real dev-dep, missing ui-scene dev-dep, stale APIs -> removed alias, added dev-dep, moved fixture `packet`/`render_mailbox_is_empty` into `ui-prepared::testing` (PreparedRenderMailboxSlot now pub(super)), unwrap close steps, import ValueError/ValueRefusalKind.
- ui-select-behavior (test): `arm_select_scroll`/`clear_select_scroll` (pure over HashMap) moved from wgpu select target into the behavior crate (still re-exported via `controls::select`).
- ui-wgpu-draw-execution (test): gpu_device `BufferSlice::slice` missing -> added relative sub-slice to ui-wgpu-device (SHARED CHANGE: device crate +1 pub fn); missing testkit features -> dev-deps on draw-contract/raster-owner/pipelines/raster with `testkit`.
- ui-wgpu-render (test): readback test used removed gpu_device::Instance API -> `platform::prepare_headless`; `queue.submit` now takes CommandBuffer -> Result; `physical_scissor_rect` from `crate::`; dev-deps testkit (draw-contract, raster-owner, draw-execution).
- ui-backend-metal: feature-unification artifact resolved once the above compiled (no edit).
- plugin-window-transient E0119: macro line already removed by a peer (verified file has derive only).

## Remaining / not done
- store-engine-command (puzzlehub/puzzleplugin store bound work) and plugin-mounted-owner (standalone workspace; dev-deps fine; needs `--manifest-path` its own Cargo.toml) unchecked.
- wasm32-wasip2 lib sweep pending.

## Second round fixes (root workspace, after first root_aa run)
- plugin-retained-command: oracle struct literal lacked `original_command_admitted` -> field `pub(super)`, oracle sets false; `mounted/frontier` used tuple `source_identity` -> `binding` from `fixture_source_caller()`.
- plugin-testing-fixtures: missing dep `semio-framework-value-snapshot-projection` (SnapshotProjection derive) -> added.
- plugin-host: `into_fault` trait not in scope -> `use semio_framework::FaultFrom`.
- plugin-actor-host (test): `Mutations.envelopes` is PagedBytes -> `PagedBytes::try_from_slice`.
- plugin-plugin-program (test): `view_state` is Option<ViewModel> in binding -> `.as_ref().unwrap()`.
- plugin-decl-tool-contract (test): missing dev-deps async/trace/serde_json -> added (unverified).
- prepared-laws + prepared `testing`: verified GREEN (fw_fix3 rc 0).
- plugin-window-transient E0119: already fixed by a peer; no edit by me.
- I made NO store bound edits (StoreState/StoreMutationFor).

## Open (not mine / not done)
- plugin-runtime-ops (peer editing typed-operations + staged.rs): `handle_intent_frame` arity (args, original_fault) + `StagedWindowContextCapture: Copy`.
- decl-views / decl-app tests: `store::os_store::snapshot_read` no longer exports SnapshotReadRegistryHandle/AliasRetirement/snapshot_registry_alias_close_step (live in value crate `semio_framework_value_snapshot_read`); fix = re-export in `store/🦀️.rs` `mod snapshot_read` (store owner).
- publication-mounted test: impl of ErasedMemberStoreOneItemPublication lacks `admit_original_origins`, `close_original_step`.
- decl-surface test: `SnapshotBuilder::from_snapshot` bounds; plugin-operation test: `ToolCancellationHandle::default`; child-group-wire-laws test (needs snapshot-projection dep too probably); store-engine-command (store owners).
- plugin-mounted-owner is excluded from the root workspace: check with `--manifest-path` of its own Cargo.toml (its dev-deps exist; root `-p` ignores them) and remove it from batch_root_aa.
- wasm32-wasip2 --lib sweep not started.

## Next step
`cd /Users/ueli/Documents/semio && C-tools/sweepplugin/run.sh nat root_aa` (then root_ab, fw_aa, fw_ab; wasm: edit run.sh mode). A root_ab check (my last edits) was still queued when stopped; all.sh parent killed so no later batches start.
