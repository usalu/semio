# 1028 result: playbook

Status: RED (frontier moved) - no error in playbook-owned crates; remaining errors are in shared framework crates owned by others. Last runs chk22/chk23.

## Frontier now (wasm32-wasip2, --keep-going)
- playbook plugin (`-p semio-hub-playbook-component`, 🌎️hub): hub manifests load again (`metadata` rc=0, playbook-meta.txt). Remaining errors, all shared:
  - semio-framework-os-history: E0063 missing `range_start` in `OriginalWireSourceIdentity` (📡️spr/📜️history/…/🛬️decode/🗂️file/🗜️deflate/🦀️.rs:19) - same family as replication-wire-core (describe).
  - semio-framework-mesh-engine: E0119 conflicting `RetireOwned` for `decode::State` (🏗️mesh-engine/…/🌲️canonical/📥️decode/🦀️.rs:36, derive + manual impl).
  - semio-framework-manifest-view-model: 5 errors, E0277 `dyn NativeValueSource` unsized / `I` unsized in a `NativeReceive` derive (needs `I:NativeValueSource+?Sized` style fix, like part21).
- procedural (`--workspace`, ✏️s/…/🌀️procedural): only semio-framework-replication-mutation-retained (E0308 x2; puzzleplugin owns replication-mutation).

## Fixed
- semio-s-artifact-stdio-contract-part21 (OWNED): wasm32-wasip2 and native lib now compile (warnings only).
  - 🫴️receiving/🦀️.rs: `semio_framework_value::native_decoding` no longer exists -> import NativeReceive/NativeReceiver/NativeValueSource from `semio_framework_value_native_owned::native_decoding`; `copied_bytes` shorthand -> `copied_bytes:copy_bytes`; `I:NativeValueSource` -> `I:NativeValueSource+?Sized` (derive instantiates `dyn NativeValueSource`).
  - 🚦️controlled/🛬️input/🦀️.rs: `use crate::is_part21_binary;`.
  - 🔤️lexer-and-parse/🦀️.rs:182: `high: Option<u32>` (inference picked i32).
  - Not verified: `--tests` build (blocked by semio-framework-replication-edit, not mine).
  - Leftover warnings: unused import `value_and_canonical_codecs::*` in root 🦀️.rs, unused parens in 🔢️values/🦀️.rs:100, derive-origin unreachable warnings.
- Source-boundary budget raises REVERTED: plugin-job (my three lines restored to 262144/32/4000; peer's new dependency line left), decl-views was reset by puzzleplugin.
- Hub manifest-load error: could not reproduce any more; `cargo metadata` for 🌎️hub exits 0 (ifc agent's fix).

## Files changed
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🫴️receiving/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🚦️controlled/🛬️input/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📦️packages/🦀️rust/Cargo.toml (revert only)

## SHARED CHANGE
none (part21 is stdio-owned artifact code).

Logs: C/🗑️generated/1028/playbook-{chk22,chk23,p21d,p21e,meta}.txt

## Update (chk26/chk27, 19:35)
Fixed (owned by me per coordinator):
- semio-framework-os-history: `range_start:0` added to `OriginalWireSourceIdentity` in 📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🦀️.rs (payload pages start at 0, mirrors `PagedBytes` in replication codec).
- semio-framework-manifest-view-model: root cause was the leaf macro in ui-locale (SHARED CHANGE): 🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧾️value/🦀️.rs `axis_value!` impls `NativeScalarValue<I>`/`NativeReceive<I>` now `I:NativeValueSource+?Sized`.
Both no longer appear in the frontier. mesh-engine (gis) and one-item (path already fixed by a peer) also cleared.

Remaining (all in the store layer, puzzlehub's in-flight split/migration; not touched):
- hub chk26: semio-framework-os-store-one-item-batch (28 errors: missing `MutationMessageConcat` in os_spr, duplicate fields `original_registry`/`registry_retirement`, `SnapshotProjection::Output==S` mismatches, private `advance_preparation`), os-store-codec-registry (1), os-store-presence (1).
- procedural chk27: semio-framework-os-store-replay: `settlement.step()` now returns `(bool,RetainedCloneProgress)` (E0308 at ⏩️replay-owned-state-operations/🦀️.rs:503; caller must account the progress through its signature, design decision) and `OperationMessageSettlement::new` is private (🎮️operation/🦀️.rs:19, edited 19:21).
No playbook-owned crate has any error.

## Update 2 (chk30, store layer) - one-item-batch, codec-registry, presence DONE
All three crates now compile (wasm32-wasip2). Fixes (store = 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store):
- os-store-one-item-batch:
  - 📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs: removed the duplicated `original_registry`/`registry_retirement` fields; moved `close_original_step` (with doc line) out of the `ArtifactStoreBatchStage` impl into the `ArtifactStoreBatchPublication` impl, where its fields (`group_preparation`, `transaction*`, `lane_*`, `preparation`) and `begin_close` live. The stage struct itself was NOT missing fields; the method was misplaced by the carve.
  - 📬️paged-one-item/🦀️.rs: impl at line 284 needs `S:SnapshotProjection<Output=S>` like the struct.
  - 📬️paged-one-item/🧬️mutation-apply/🦀️.rs: dropped the second (dead) `parent_copy`/`context`/`progress` helper trio in the duplicate impl block; removed `MutationMessageConcat` from the os_spr import list (it is imported from `protocol`); `mutation_apply_preparation_factory` where-clause `S:SnapshotProjection<Output=S>`.
- os-store-presence: 🫧️ephemeral/📤️ephemeral-one-item-publication/🦀️.rs `advance_preparation` `pub(super)` -> `pub` (SHARED CHANGE: visibility; presence is a sibling crate after the split).
- os-store-codec-registry: 📦️codec/📸️native-snapshot/writing.rs `declared_binary_prefix_byte(..)?` now `Option<u8>`: `.ok_or_else(invalid)?` (offset beyond the declared prefix is an invariant refusal). (The `retirement::RetireOwned` trait path in receiving was fixed by puzzleplugin.)

## Frontier now (hub chk30)
- semio-framework-os-store-engine-stage (puzzlehub carve, ⚙️engine/🗄️stage): 13 errors, all in ⚙️engine/📥️lifted/🗄️durable-group/🦀️.rs: unresolved import `bound_DurableBoundPlanningV1` / missing `planning::DurableBoundPlanningV1`, and missing generic bounds (`P: Clone`, `P: FromValue`, `Mutation: Clone`, `Mutation: OpBinary`) on the lifted impls. Also seen earlier: engine-core `EditReplay::is_finished/finish` need `P: Clone, Mutation: ColdMutation<P>` bounds (⚙️engine/🧱️store-components/…/🖱️store-cursor-operations/🦀️.rs:140,142). Not touched (puzzlehub).

## CHECKPOINT 23:05 - STATUS: RED (stopped at usage checkpoint; all my edits compile-atomic)
Update 3 (plugin layer, non-bound errors):
- semio-framework-plugin-instance-jobs (🧵️instance/👷️jobs/📥️staging): `⏯️tool-run` now uses `semio_framework_plugin_reactor_app::tasks_extension_continuation` (the crate `plugin_reactor_tasks` is not a dep); `🔒️close-ladder` imports `ToolCancellationLease` (plugin_operation) and `SpaceMember` (os_kernel); `🚪️ingress` imports `SpaceMember`, `semio_framework_job::RetainedCloneProgress`, `bounded_ownership::plugin_grant_funds`. Verified only by reading the compiler's own suggestions; not yet re-compiled.
Files changed in this update: the three staging `🦀️.rs` files above.

Last known frontier (chk31/chk33, before the instance-jobs edit):
- hub (playbook plugin): semio-framework-plugin-reactor-context (36 errors in ⚛️reactor/🌉️call/🎭️exports/🦀️.rs: reactor/puzzleplugin); semio-framework-artifact-flow-flow-document (1: `ProducedMemberSnapshotOpen` removed in the carve -> `OriginalRecordMemberSnapshotOpen<S>`, flow's); instance-jobs tool-run (fixed now).
- procedural: semio-framework-plugin-reactor-wit-bridge: `poll_kernel_output` now takes `events:&mut Option<Event>`, bridge passes `Vec<Event>` (🪢️bridge/🦀️.rs:91; reactor/puzzleplugin mid-edit), plus instance-jobs (fixed now).

NEXT STEP (after peers land reactor + flow): run both, detached with .rc marker, e.g.
`cd /Users/ueli/Documents/semio && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 --keep-going -p semio-hub-playbook-component`
`... --manifest-path ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml --target wasm32-wasip2 --keep-going --workspace`
Two checks of mine may still be queued/running: logs C/🗑️generated/1028/playbook-chk34.txt (hub) and playbook-chk35.txt (procedural), .rc appears when done; they verify the instance-jobs edit.
