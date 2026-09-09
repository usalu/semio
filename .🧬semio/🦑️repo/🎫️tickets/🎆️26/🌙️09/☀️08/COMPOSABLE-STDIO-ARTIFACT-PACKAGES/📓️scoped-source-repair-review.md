# Scoped Source Repair Review

Read-only review on 2026-09-09. No native command was launched and no implementation file was modified.

## Result

The settled Object, Kit, and Drawing repairs follow their declared schema/grammar and do not relax fixtures. The Norm surface fixture correctly retires registered apps before surfacing a result. The shared viewer helper has a current compiler defect, described below. Two focused Semio/Norm regression assertions are still absent; add them before treating those source repairs as fully acceptance-covered.

The latest Norm surface receipt is blocked before test execution by seven `E0277: u8 is not a future` errors in `semio-framework-os-kernel`; it is not evidence about the Norm changes. The receipt is `🗑️generated/norm-surface-render-test-final.txt`, lines 15-22. No Semio focused runtime receipt was available at this review point.

Norm started focused Nx session 59206 after these three Semio source repairs. Its intended receipt is `🗑️generated/stdio-semio-schema-defaults-focused.txt`; it was still live and zero bytes when checked, so this review makes no test-pass claim.

## Confirmed Source Semantics

### Semio Snapshot Decoders

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs:67-79` now keeps `schema` and `transform` required, while absent `brep`, `mesh`, and `properties` decode as `None`. That matches the Object JSON Schema, whose required list contains only `schema` and `transform`.

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs:128-142` keeps `schema` required, defaults absent list fields to empty, and decodes absent `properties` as `None`. This matches the Kit JSON Schema, whose only required property is `schema`.

Both decoders still decode and reject malformed values when a field is present. They do not change fixtures, children remain handles, and the repairs are constrained to the manual schema boundary.

### Drawing Binary Keyword

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:19` now aliases `OP_KEYWORDS` to the canonical kebab-case `super::KINDS`. Decode reconstructs the same grammar keyword used by `parse_op` at lines 71-75; ordinal layout remains unchanged. This is the smallest schema-first repair for the former `rotateNode` diagnostic. The existing exhaustive `op_binary_roundtrip_law` at `💾️binary/🧪️tests/🔬️unit/🦀️.rs:6-11` is the appropriate runtime regression target, but has not been rerun in a settled receipt.

### Viewer Helper and Fixture Ownership

The fixture lifecycle ordering in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6734-6758` is sound in intent: it records the handler result, closes through the exact state machine, and only then exposes an error or assertion. The close helper (`:6462-6482`) rejects `AwaitingInput` and `Blocked`, and requires the terminal-empty witness.

It is not currently compilable. The live Flow receipt `🗑️generated/native-library-recovery-semio-s-artifact-flow-flow.txt`, lines 101 onward, reports `E0308` at helper line 6751: `ViewerApp::handle` requires `ArtifactView<V::Snapshot>` and `ConfigView<V::Config>`, while `app.cache.as_ref()` leaves `ArtifactView<Arc<V::Snapshot>>` and `ConfigView<Arc<V::Config>>`. The minimal repair is to construct the views from `snapshot.as_ref()` and `config.as_ref()`; do not weaken the helper or change fixtures. Runtime claims about the helper remain invalid until a fresh compiled receipt passes.

The Norm integration fixture otherwise follows the same pattern: it projects and retires every component tree, records each app result, closes the app, then panics on the recorded result (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🖥️app-surface/🦀️.rs:28-81`). Its retained cohort also closes every real plugin-created runtime and applies the helper to all fifteen viewer types (`:134-214`). The lifecycle assertions remain valuable once the `Arc::as_ref` type gap is repaired; registry-less constructor checks are outside that registered-fixture contract, and `VcsArtifactApp::Drop` only cancels its tool scope (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21431-21435`).

### Norm Report and Test Stack

`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:95-113` gives each report row a sibling-unique index key, so repeated/default labels can no longer collide. The index reflects computed-check order, which is the report's row identity for this stateless rendering tree.

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:216-221` sets `RUST_MIN_STACK=67108864` only for the `surface_render` Cargo test child. It does not alter application runtime configuration or other test targets.

## Required Follow-ups

1. Add an Object decoder regression in `📦️object/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` that decodes a value containing only the schema-required `schema` and `transform`, and asserts all three optional handles are `None`.
2. Add a Kit decoder regression in `🧰️kit/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` that decodes a value containing only `schema`, and asserts each collection is empty and `properties` is `None`.
3. Add a `render_report` test in `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🔬️unit/🦀️.rs` with at least two checks that share a clause/default-like presentation. Project the rendered tree and assert two distinct row keys. The current report test at lines 97-111 validates one table row only and cannot detect the repaired duplicate-ID failure.
4. After the shared kernel `E0277` owner repairs that compiler blocker, rerun the existing focused Drawing binary law and the ordinary Semio representative gate; retain those receipts separately from this source review.

## Formatter Incident Attribution

`📓️formatter-incident.md` records an unintended `cargo fmt --all --` invocation, PIDs 59812/59816/59887, termination, and later exact-file rustfmt on the three Semio sources. A current process snapshot contains no live `cargo fmt` or `rustfmt` process. The three owned source files still have the shared 10:10:54 modification time described as preceding the incident.

The report does not preserve the repository-wide or narrowed modification-time query it relies on. Current metadata therefore cannot independently establish that the interrupted workspace-wide formatter changed no other Rust file, particularly while other agents are editing the same workspace. The evidence supports no broad revert: preserve concurrent work, retain the incident record, and use a future path-scoped diff/mtime receipt if a clean attribution is needed.
