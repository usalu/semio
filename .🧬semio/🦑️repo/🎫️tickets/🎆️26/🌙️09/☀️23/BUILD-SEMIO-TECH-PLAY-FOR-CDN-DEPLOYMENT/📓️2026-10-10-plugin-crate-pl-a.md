# 2026-10-10 pl-a: plugin crate duplicates, mounting, missing plugin-local symbols

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin` (`semio-framework-plugin`). Root `🦀️.rs` is abbreviated `root`.

## Result lines
- Baseline 451 errors, then 73, 6, 5, then 0.
- `cargo check -p semio-framework-plugin --lib` (native): Finished, 0 errors (after peers' concurrent edits).
- `--target wasm32-wasip2 --lib`: Finished, 0 errors after annotating the forwarded identity observer (`IdentityObserver` alias in `🚪️io/🛂️authority`: the kernel `EntityIdentityAuthority<'a, O>` became generic over its observer).
- `--target wasm32-wasip2 --lib --features component-guest` and `component-extension-guest`: Finished, 0 errors.
- `--lib --features artifact-app-testing` (native and wasm): 35 errors, all in testing fixtures and `🧪️tests/*` (history-edit-acceptance, node-drag-history `with_authoring_identity`, `document_store_owners_source_demands` wrappers missing imports at root ~9357, `identity` at ~8249). Not pl-a's class; the 75+ plugin dev-deps use this feature.
- `-p semio-framework-plugin-host`: 28 errors (was 51). pl-a mounted the missing `crate::reactor::original_actor_context` in the host packages root; the remaining ones are the job-session API (`take_outcome`, `try_new`, two `InteractiveJob::step` impls), `IssuedShardTurn: Copy`, `Budget.retained`/`TurnResult.retained_receipt`, and `⚡️effects` compute errors.
- `-p semio-framework-plugin-describe`: blocked by the host errors. `semio-framework-os` (host): not run.
- `GREEN os` NOT written.
- Logs: `.🧬semio/🦑️repo/⚡️cache/play-fleet/pl-a/check*.log`; older plugin revisions pulled from git history sit in `…/pl-a/hist/` (`root-<rev>.rs`). They are the reference for what the pre-merge side looked like.

## Fixes (all source edits, minimal anchors)
- Mounting: `🧵️retained-command/🦀️.rs` now mounts `🧬️context` (`ArtifactOwnedContextHandle`, `Context*` parts re-exported `pub(crate)`). `♻️frontier` and `🧩️composition/*` were already reachable by `include!`/`#[path]`.
- `runtime_lifecycle_grant()` restored in `plugin_runtime` with the original body from revision da607013f32 (copy = `RUNTIME_CLOSE_BYTES_PER_STEP`, capacity/release = envelope-decode maxima, depth 4096). The four in-app callers use `self.mounted_policy.maintenance` instead (the app owns its policy); the function is kept for the fixtures.
- `mounted_private_child_grant(0, bytes)` removed: `retire_typed_operation_run` now builds `original_plugin_turn_grant(policy.close, retained)`, quotes `typed_operation_retirement_demands`, funds-checks with `plugin_grant_funds` and drives `retire_typed_operation_unit(operation, grant)` (PluginLifecycleStep).
- `DOCUMENT_ARCHIVE_POLL_STEP_BYTES = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` next to `DOCUMENT_ARCHIVE_POLL_WALL_US`.
- `OriginalAuthoringEffect.publication`: `PublicationClaimHandle` (the async crate's claim) instead of the removed `Arc<ToolPublicationClaim>`.
- Snapshot read-return rotation: `snapshot_read_returns_retirement_demands` renamed to `snapshot_read_returns_demands` (what both ladders call). Added `next_snapshot_read_return_stage()` (first stage from the cursor whose pump or store holds a returned read) and made quote, step and `snapshot_read_returns_terminal_is_empty` all use it (config and interaction stores were previously ignored for terminality).
- `document_windows_retirement_demands(body, closing)` added in `🪟️window/🫧️transient/🔁️document-replacement`; dead `document_window_registry_demands` deleted.
- `ActiveMediaExport::retirement_demands(body)` added (mirrors `close_step` order).
- `PeerRosterPublication::{step_demands, close_demands}` added; `close_release` call replaced by `PresenceCommandCursor::close_step(grant)` (spr kernel already has the granted API; no new kernel method).
- `ToolLatestWinsRegistry`: methods were from the Weak-claim era while the struct was already the new one (strong `PublicationClaimHandle`, `retired_token/claim`, `token/claim_retirement`). Rewrote `new/can_begin/terminal_is_empty/begin(claim clone)/advance`, added `advance_demands()`. `advance` now returns `Result<RetainedCloneStep, Fault>` (tests and both ladders expect that); the four callers in `advance_latest_wins_admission_unit` adapted. A retired scope is decomposed into token and claim, each retired through `plugin_typed_owner_step`; stale-scope scan uses `claim.is_finished()`.
- `ArtifactInstanceOperationOwnerHandle`: gained the `inference` session map (`InstanceInferenceSessions` existed unused) with `with_inference_session`, `inference_is_empty`, `close_inference`.
- Duplicates deleted (stale side): second `ChildMemberRetirement::{retirement_demands, close_step}` (kept the ControlledRetirement-metadata pair), second `retirement_step`.
- `ArtifactApp` trait got `mounted_job_{maintenance,close}_demands(instance_id, body)` (defaults zero) and the three forwarding wrappers forward them (the editor/viewer traits already had them).
- `MountedTypedCommandFullOperation`: field `cancellation_retirement`; `retirement_demands`, `worker_step_demands`, `worker_retirement_pending/demands`, `retire_worker_step`; `retirement_step` rewritten around the granted frontier (close fault, worker session, window receipt, granted frontier, then view_state/window authorities admitted into the erased `publication_retirement` slot, captured child content, publication outcome, verb, actor lease). `terminal_is_empty` gained `actor_capture`; the lease file no longer reads the dead `terminal_outcome`.
- `ActiveArtifactStoreReplacement`: added `cancelled()`, `initializer_demands`, `member_open_demands`, `closure_demands`, `candidate_views_demands`, `retirement_demands`, and granted ports of `drive_member_open` / `drive_closure` (old `(max_items, max_bytes)` versions recovered from revision cede6fc227b; `StepContext::new` now takes the retained wallet and receipt). Maintenance ladder calls `active.cancelled()`.

## Decisions
- Newer side = the one consistent with grant-based protocol and the current job/store/value crates. Recovered stale sides from git history only as semantic reference.
- Anything that holds an erased retirement slot (`publication_retirement`) is closed by the granted frontier; new owners are only admitted into it.

## Open / handed over
- `impl RetireOwned for semio_framework_async::CancelToken` is missing in the async crate (exists only in unmounted `⏳️async/♻️cancel-return/🦀️.rs`, which also defines a duplicate `CancelTokenRetirement`). Told main; blocks `ToolLatestWinsRegistry`, the cancellation lease and `CancelToken` controlled retirement (about 17 E0277 at 11:2x).
- Test-only literals of `MountedTypedCommandFullOperation` (`🧪️tests/♻️publication-retirement-authority`) still list `terminal_outcome`.
