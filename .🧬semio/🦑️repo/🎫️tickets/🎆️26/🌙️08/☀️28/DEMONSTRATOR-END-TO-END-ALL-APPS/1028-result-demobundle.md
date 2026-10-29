# 1028 result demobundle

STATUS: RED (frontier: foreign `semio-framework-os-store-replay`; chk8). Guest crate `semio-hub-demonstrator-component` (hub `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust`) not yet reaching its own code: blocked by peer churn in framework value/replication/ui-locale crates (E0277 `dyn NativeValueSource` unsized, NativeRecordKey API, store-link-schema). Owned by the value-native-receive sweep, not demonstrator glue.

## Done: plugin-job split (assignment from coordinator)
`semio-framework-plugin-job` 286801 B -> 246211 B / 14 files / 2723 lines (gate 262144/32/4000, census verified).
New sibling crate `semio-framework-plugin-job-receiving` (40651 B, 5 files, 202 lines) at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust`, lib root `👷️job/🛬️receiving/🦀️.rs`, holding modules original_pack_failure, original_pack_receiving (moved to `📥️pack/🦀️.rs` with its `🧪️tests`), original_value_copies, original_transaction_prepare.
Consumers updated (no shims): job `🏛️reserved/📌️checkpoint/🦀️.rs` (import), reactor custody manifest `🦀️.rs` (import), Cargo deps in `👷️job/📦️packages/🦀️rust/Cargo.toml` and `⚛️reactor/🏃️runtime/🧰️ops/📦️packages/🦀️rust/Cargo.toml`.
Not compile-verified (dependency crates fail from peer edits; chk3-5 logs). Static review: all items pub, no crate:: refs, deps mirrored from job.

## SHARED CHANGE
- `🧰️framework/Cargo.toml`: exclude entry for the new crate.
- `📋️build-owners.json`: owner `@semio-tech/plugin-job-receiving-rs`.
- new `📋️project.json` in the new crate.
- `semio-framework-plugin-job` no longer exports original_pack_failure/receiving/value_copies/transaction_prepare; use `semio_framework_plugin_job_receiving`.

## Files changed
job `🦀️.rs`, `🏛️reserved/📌️checkpoint/🦀️.rs`, job Cargo.toml, runtime-ops Cargo.toml, reactor custody manifest `🦀️.rs`, moved receiving tree, new crate files.

## Next
Re-run `repo cargo ... check --target wasm32-wasip2 -p semio-hub-demonstrator-component` after peers' value-native-receive migration compiles (chk6 log: C/🗑️generated/1028/demobundle-chk6.txt).

## Resume run (chk7, chk8)
Fixed (foreign crates, trivially local, orphan-rule / type-migration leftovers):
- `semio-framework-3d-mesh-quality`: inherent `impl HalfedgeMesh` outside the defining crate (E0116) -> free fn `mesh_quality_report(&HalfedgeMesh)`; callers updated: procedural `analysis-check` lib + unit test, quality unit test. SHARED CHANGE (API: `HalfedgeMesh::quality_report()` removed).
- `semio-framework-os-pack` feature `record-codec-laws`: `🧪️tests/🧬️record-codec-laws/🦀️.rs` built `Bytes64(Vec<u8>)`, now `RecordBytes` (`try_from_slice` / `default`).
Remaining (foreign, owner = store/replay executor, peer mid-edit):
- `semio-framework-os-store-replay` (`🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs`): L503 `settlement.step(..)?` now returns `(bool, RetainedCloneProgress)` (E0308); L600 `OperationMessageSettlement::new` is private in `🔁️replay/🎮️operation` (E0624).
No demonstrator-owned or composition errors reached yet. The plugin-job split crates compile checks are queued behind these.

## Free-fix iteration (chk9, chk10)
STATUS: RED, frontier = framework os store-engine split in flight (peers editing within 10 min, so skipped per rule).
- chk9: `semio-framework-os-store-engine-live`: E0034 ambiguous `foreign_step_source` (trait `ColdMutation` in `📡️replication/🎮️mutation/❄️reference` vs `protocol::Mutation` in `🎮️mutation/🦀️.rs`); mutation crate changed minutes before.
- chk10: `store-engine-branch` (37 errors: `ArtifactStore::generation/content_revision` bounds), `store/🧩️composition/🗄️durable-group` (E0117/E0116/E0592 orphan impls + duplicate `originals_empty`/`retire_originals_step`/`advance_binding`/`take_bound`, files `🪪️three-member-decision-operations-codecs`, `🛠️map-assembly-recovery-completion`, `📒️group-decision-journal-record-and-snapshot`), `store-initialization-replay` (38). All from the peer store-engine carve; `🌿️branch/🦀️.rs` modified within 10 min.
No demonstrator/composition errors reached. Re-run chk after the store-engine split settles; plugin-job split crates still unverified by compiler.

## CHECKPOINT 22:55 state
STATUS: RED, stopped at checkpoint. Last verified frontier (chk11): non-demonstrator errors only.
- Fixed after chk11: `semio-framework-os-kernel-neural-engine-model` E0119 (explicit `impl OriginalDomainProjection for Dictionary` in `🧠️neural/⚙️engine/📤️projection/🦀️.rs` conflicted with blanket `impl<T:DslField> FieldProjectionSource` now provided by `📝️dsl`). Removed the stale module file and its `mod dictionary_projection` line in `🧱️model/🦀️.rs`. Behavior note: projection_identity for Dictionary now defaults to the object address instead of `original_root_identity()`; restore via a DslField identity hook if a law needs it.
- Remaining at chk11 (not fixed): `plugin-instance-children` (17 errors, `M: ReferenceSpaceMember` bound missing on `impl<M: SpaceMember + MemberFactory>` in `🧵️instance/🪆️children/📥️staging/🧱️components/🦀️.rs` L53; owner puzzleplugin bounds); `plugin-reactor-wit-bridge` (E0308 `🪢️bridge/🦀️.rs:91`: `poll_kernel_output` now takes `&mut Option<Event>`, bridge passes `Vec<Event>`; file was being edited by a peer).
- chk12 (`C/🗑️generated/1028/demobundle-chk12.txt`, marker `demobundle-chk12.rc`) was started to verify the neural fix and is left running; read its `could not compile` lines first.
Next step: `cd /Users/ueli/Documents/semio && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-demonstrator-component`, then the native check of `semio-framework-plugin-job`, `-job-receiving`, `-runtime-ops` (framework workspace, `--tests`).
