# Packet Report — `✏️s/🔌️plugins/🎬️sequence` artifact-tree `⚙️engine` rehome

Target: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (1288 LOC, single file).

## Region → destination map applied

| Region (old file) | Destination | Notes |
|---|---|---|
| `SequenceEngine` struct + impl (`🔖️ArtifactEngine`) | **Deleted outright** | Zero construction sites repo-wide (only its own `struct`/`impl`). Matches rule 1 exactly. |
| `io_registry` module (`🚪️DerivedIoRegistry`: `entries()`, `rebuild_native_snapshot`, `compose_export_{csv,md,json}`, dialect consts) | `🧬️.../🚪️io/🦀️component.rs` | Appended as a new `//#region 🚪️DerivedIoRegistry`, verbatim body (only the wrapping module moved; internal `crate::artifacts::sequence::io::...` paths are logical, unaffected by the physical move). |
| `sequence_example_json()` (`🔖️Example`) | `🧬️.../🧬️schema/🦀️component.rs` | Pure helper over the document (rule 3) — zero external callers found repo-wide, so it colocates with `default_snapshot()`/the schema descriptor rather than with any single consumer. Added `use store::ArtifactDsl;` + `default_snapshot`/`SequenceSnapshot` to the file's top imports. |
| `register()`, `register_pilot_languages()`, `register_artifact_schema()`, `register_artifact_inferences()` (`🔖️Register`, `🔖️SchemaRegistry`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region 🔌️Registration`) | Matches the validated `block/◻2d` exemplar exactly — `block2d`'s `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()` all landed in `🎛️apps/◻2d/🦀️component.rs`, not the schema file. `register()`'s codec-registration call simplified from `register_document_codec_for_app::<crate::apps::sequence::SequencePlayApp>` to the now-local `SequencePlayApp`. |
| `sequence_io()`, `next_available_step_id()` (`🔖️Io`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region 🔖️Io`) | `*_io() -> AppIo` is rule 4 verbatim. `next_available_step_id` has exactly one consumer (`SequencePlayApp::import_media`, in this same file) — colocated per the old file's own stated rule ("a helper with exactly one consumer lives in that consumer's component file"). |
| `SequenceCamera`↔`DagCamera` conversions (`🔖️Camera`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region 🔖️Camera`) | Both consumers (`SequenceHost::rebuild_dag`/`sync_from_dag`, and the wasm bridge) now live in the app tree. |
| `SequenceCoreError` (`⚠️Errors`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region ⚠️ Errors`) | `SequenceHost`'s own error type; no external name references found, moves with its owner. |
| `SequenceHost` struct + impl, plus every helper it alone uses (`is_control_kind`, `control_slots`, `default_control_slot`, `slot_key`, `parse_serial_suffix`, `max_serial_in_snapshot`, port/channel builders, DAG node sizing, `ensure_imperative_modules_for_tests`) (`🔖️Host`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region 🔖️Host`) | **Not covered by the map's 7 numbered rules** — this is a stateful, mutable UI-editing host (add/remove/connect/reorganize steps, DAG rebuild), not a pure derived-compute fn (rule 2) or a pure document helper (rule 3). Every one of its 14 pre-existing call sites (`grep`, see below) is inside `🎛️apps/🎬️sequence/*` (commands, windows, panels, the wasm bridge) — textbook "behaviour belongs to the app that edits the artifact." Placed in the app's own top-level `component.rs` per the same "shared compute lives in the closest common ancestor across taxonomy nodes" principle the map already applies to registration wiring. |
| `host_from_snapshot`, `ops_from_host_mutation` (`🔖️HostHelpers`) | `🎛️apps/🎬️sequence/🦀️component.rs` (new `//#region 🔖️HostHelpers`) | Same reasoning as Host. |
| Tests (`🧪️Tests`, 46 `#[test]` fns / 86 assertions) | `🎛️apps/🎬️sequence/🦀️component.rs`'s existing `mod tests`, new `//#region 🔖️HostTests` | Extended the existing test file (no new test file created), zero name collisions with the 15 pre-existing tests there. |

## Naming collision handled

`apps/🎬️sequence/🦀️component.rs` already binds bare `Value` to `serde_json::Value` (`use serde_json::{json, Value};`, used by `import_media`). The engine file's `neural_engine::Value` (atoms for step params) would have collided. Imported it as `use neural_engine::{..., Value as NeuralValue};` and rewrote every bare `Value` in the moved Host code/tests (1 fn signature, 6 test literals) to `NeuralValue`.

## Call sites updated (14 + 2 doc/module-wiring)

- `✏️s/🔌️plugins/🎬️sequence/🦀️component.rs:6` — `crate::artifacts::sequence::engine::register()` → `crate::apps::sequence::register()`
- `🎛️apps/🎬️sequence/📌️panels/📄️artifact/🦀️component.rs:5`, `📌️panels/🛍️catalogue/🦀️component.rs:6` — `engine::{control_slots, is_control_kind}` → `crate::apps::sequence::{control_slots, is_control_kind}`
- `🎛️apps/🎬️sequence/🌉️wasm/🦀️component.rs:6` — `engine::{sequence_camera_from_dag, SequenceHost}` → `crate::apps::sequence::{...}`
- `🎛️apps/🎬️sequence/🎭️modes/✏️edit/🪟️windows/{📜️script,🧬️compiled,📽️main}/🦀️component.rs` — `engine::host_from_snapshot` → `crate::apps::sequence::host_from_snapshot`
- `🎛️apps/🎬️sequence/🎮️commands/🪜️step/🦀️component.rs` — `engine::{host_from_snapshot, ops_from_host_mutation}` → local app path
- `🎛️apps/🎬️sequence/🎮️commands/{🏃️run,🔄️layout,🔗️connection,🕸️node-graph}/🦀️component.rs` — same pattern for whichever of `host_from_snapshot`/`ops_from_host_mutation` each uses
- `🗿️artifacts/🎬️sequence/🦀️component.rs:254` — `standards::v1::engine::io_registry as v1` → `standards::v1::subsets::any::io::io_registry as v1` (the "thin wrapper" call site flagged by the packet instructions)
- `🗿️artifacts/🎬️sequence/🦀️component.rs` — 3 internal call sites inside `SequencePlayApp` (`fn io()`, `import_media`, the manifest's `.io(...)`) updated to call the now-local `sequence_io()`/`next_available_step_id()`
- `🗿️.../🧬️schema/🧬️mutations/🦀️component.rs:142` — see Deviation below
- `📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod engine;` (`#[path]` pointed at the now-deleted file) and the `pub mod engine { pub use super::standards::v1::engine::*; }` shim
- Docstrings: `🎛️apps/🎬️sequence/🦀️component.rs` module doc, `🗿️artifacts/🎬️sequence/🦀️component.rs`'s `SequenceCamera` doc, `🚪️io/🦀️component.rs`'s file doc — all updated from "the artifact's `⚙️engine`" to the correct new location.

## Deviation from the map (the interesting part)

`🗿️.../🧬️schema/🧬️mutations/🦀️component.rs`'s test `snapshot_mutations_capture_move_and_connect` built its "after" snapshot via `crate::artifacts::sequence::engine::SequenceHost::default()` + `.add_step(...)`. Since `SequenceHost` now lives in `crate::apps::sequence` (an app), leaving this call site as-is would make an **artifact schema test depend on an app** — the exact thing the codebase's own docstrings (block2d's `register()` doc: "an artifact must never depend on an app") forbid. Rewrote the test to build `before`/`after` `SequenceSnapshot`s by hand (push one `SequenceStep`) and call `sequence_snapshot_mutations(&before, &after)` directly — same function under test, same single assertion, zero `SequenceHost` dependency. Assertion count for that file unchanged (9 before, 9 after; 11 `#[test]` before, 11 after).

## Structural verification

- `grep -rn "sequence::engine\|standards::v1::engine" ✏️s/🔌️plugins/🎬️sequence` → **0** hits.
- `find ... -type d -name "⚙️engine"` under the artifact tree → **gone**. (An unrelated, pre-existing, empty `🎛️apps/🎬️sequence/⚙️engine/` directory with zero files remains on disk — last touched by commit `6f693a1` from 2026-06-04, not part of git, not the artifact-tree target, untouched by this packet.)
- `grep -rn "SequenceEngine" ✏️s/🔌️plugins/🎬️sequence` → **0** hits (deleted, not just unreferenced).
- Every destination file exists and contains the moved code (verified by direct `Read` after each edit).
- Assertion count: engine file had **46 `#[test]` / 86 `assert*!`** (via `git show HEAD:<path>`). `apps/🎬️sequence/🦀️component.rs` went from **15 tests / 26 asserts** (pre-existing) to **61 tests / 112 asserts** — deltas are exactly +46/+86. `schema/mutations/component.rs` stayed at **11 tests / 9 asserts** (the one rewritten test kept its single assertion). **Equal or higher, confirmed by exact arithmetic, not eyeballing.**

## Compiler verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-sequence --all-targets` — **red**, 12 errors (both `lib` and `lib test` targets; same 12 errors each). Independently confirmed `semio-s-plugin-stdio` itself is green (`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets` → `Finished ... in 39.66s`, exit 0, 0 errors — ran this myself, not inferred).

**All 12 errors are outside this packet's edited regions**, confirmed via `git diff` (I only touched the files/lines listed above) and `git log` (every offending file/line was last touched by a commit at or before HEAD, i.e. already in this state before I started):

1. `🎛️apps/🎬️sequence/🦀️component.rs:884` (×2, lib+test) — `SequenceMutation::StepsAdd` variant doesn't exist on the 8-variant enum. This line is `import_media`'s pre-existing body (I only touched the adjacent `next_available_step_id` call two lines above it) — verified via `git diff` this line is untouched by me.
2. `🚪️io/📤️export/🧵️serializers/.../📊️csv/.../component.rs:11`, `.../📝️md/.../component.rs:10`, `🚪️io/📥️import/🧩️deserializers/.../📊️csv/.../component.rs:9`, `.../📝️md/.../component.rs:9` — `CsvSnapshot`/`MdSnapshot` (types owned by `semio-s-plugin-stdio`) no longer have `headers`/`rows`/`body` fields (now `has_header`/`records`/`blocks`). Files never touched by this packet.
3. `🚪️io/📤️export/.../🔣️json/.../component.rs:10`, `🚪️io/📥️import/.../🔣️json/.../component.rs:9,18` — `serde_json::Value` vs `semio_s_plugin_stdio::...::JsonValue` mismatch. Files never touched by this packet.
4. `🧬️schema/🧬️mutations/🦀️component.rs:95,99,114` (×2 each, lib+test) — `.diff()`/`.inverse()` not found on `&SequenceMutation`; rustc suggests `use crate::dsl::Mutation;`. These lines (`apply_sequence_mutation`, `inverse_sequence_mutation`, the test `round_trip` helper) are untouched by me — I only edited the one test function ~40 lines below them (confirmed via `git diff`, 8 insertions/4 deletions, all inside `snapshot_mutations_capture_move_and_connect`).

All four point at the same upstream cause: the concurrent mutation-vocabulary/stdio-shape refactor has propagated into `semio-s-plugin-stdio` itself (now green) but **not yet into `semio-s-plugin-sequence`'s own pre-existing stdio-integration leaf files and `#[derive(dsl::Mutations)]` usage**, which is a separate, much larger fix (updating every plugin's stdio-shaped serializers and re-deriving/re-importing the mutation trait) outside this packet's scope (`⚙️engine` rehome only). Labeling honestly per the packet instructions: **refactor complete, structurally verified, NOT compiler-green — 12 pre-existing errors, all outside the edited regions, none inside `⚙️engine` or on any line this packet touched.**

`cargo test -p semio-s-plugin-sequence` was also run; it cannot get past the same compile errors (test binaries build on top of the failing `lib`/`lib test` targets), so no test executed either — consistent with the check result, not a separate finding.

## Files touched

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — **deleted** (directory removed).
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — added `io_registry` region, fixed file doc.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — added `sequence_example_json()` + imports.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — rewrote one test to drop its `SequenceHost` (app) dependency.
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🦀️component.rs` — the big one: new imports, new regions (`Io`, `Registration`, `Camera`, `Errors`, `Host`, `HostHelpers`), 3 internal call-site fixes, `NeuralValue` rename, 46 tests appended.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️component.rs` — `io_registry` wrapper path fix, `SequenceCamera` doc fix.
- `✏️s/🔌️plugins/🎬️sequence/🦀️component.rs` — `register()` call site.
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/📌️panels/{📄️artifact,🛍️catalogue}/🦀️component.rs`, `🌉️wasm/🦀️component.rs`, `🎮️commands/{🪜️step,🏃️run,🔄️layout,🔗️connection,🕸️node-graph}/🦀️component.rs`, `🎭️modes/✏️edit/🪟️windows/{📜️script,🧬️compiled,📽️main}/🦀️component.rs` — import path fixes.
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs` — removed `engine` mod declaration + shim.
