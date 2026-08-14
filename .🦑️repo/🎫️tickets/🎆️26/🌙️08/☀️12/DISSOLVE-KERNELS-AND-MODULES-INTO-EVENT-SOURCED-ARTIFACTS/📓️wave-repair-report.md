# Wave REPAIR — Report

## Ownership re-derivation (done independently, before touching anything)

- `/tmp/cc-socks/` at start of wave: `20304.sock` (Aug 13 19:10), `8850.sock` (Aug 13 18:08), `92028.sock` (Aug 13 22:26). None correspond to a UCAS session; by the time verification ran, `92028.sock` was already gone too — no socket ever pointed at either target crate's working session.
- `find ... -name "*.rs" -newermt "2026-08-12 00:00:00"` on both crates' Rust source trees showed exactly **one** file each with a recent mtime: `➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` (2026-08-13 22:18:55) and `📸️remodel/📦️packages/🦀️rust/📦️glue.rs` (2026-08-13 21:18:51) — both are the FIXALG-wave `semio_framework_number` repoint the brief said to leave alone, confirmed by reading `📦️glue.rs`'s own docstring (lines 24–41: "Wave FIXALG (same ticket) relocated `VecG`/`MatG`... and repointed both sites at `number::MatG`/`number::VecG`"). No other file in either crate had a post-12th mtime, i.e. no in-progress edits from any other session.
- Re-checked `/tmp/cc-socks/` and file mtimes again after finishing (before writing this report): only my own `touch` calls on `glue.rs` (required by the verification protocol) show up as recent; no other `.rs` file in either crate changed hands mid-wave.
- Conclusion: both crates were genuinely orphaned for the whole wave. Proceeded.

## `semio-s-plugin-mathematical` — before: 9 errors, after: 0

Census (from `cargo check -p semio-s-plugin-mathematical --all-targets`, `RUSTC_WRAPPER=""`):
- 1× E0432 `unresolved import crate::apps::mathematical::commands::document`
- 2× E0422 (`SetArtifact`, `SetPoints` not found), 1× E0422 (`SetLocale` not found)
- 5× E0433 (`node_graph_edit`/`set_directed`/`node_graph_viewport` not found in scope, repeated across call sites)

`📦️glue.rs` (`✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`, lines 465–481) mounts `apps::mathematical::commands` **flat** — 7 sibling modules (`set_artifact`, `set_algorithm`, `set_directed`, `node_graph_edit`, `node_graph_viewport`, `set_points`, `set_locale`), no grouping submodule named `document` or anything else. That settled every fix here: every stray reference was a flat-name that had lost its `use`, not a real grouping.

Fixes:
- `🎛️apps/➗️mathematical/🦀️component.rs:16` — `use crate::apps::mathematical::commands::document::set_artifact;` → `use crate::apps::mathematical::commands::set_artifact;` (only line referencing the nonexistent `document` grouping; every other import in the same file was already flat, confirming `document` was the sole abandoned-script leftover).
- `🎮️commands/📄️set-artifact/🦀️component.rs`, `📐️set-points/🦀️component.rs`, `🗣️set-locale/🦀️component.rs` — each command's own `#[cfg(test)] mod tests` referenced its own sibling struct (`SetArtifact`, `SetPoints`, `SetLocale`) without a `use super::*;`. Evidence: the struct is `pub struct SetArtifact { .. }` etc. defined directly in the **same file**, one module up from `tests` — `use super::*;` is the exact pattern already used in the sibling `🕸️set-algorithm/🦀️component.rs` test module, so this restores consistency rather than inventing a new pattern. rustc's own multi-candidate suggestion list for `SetLocale` was TRAP-1-shaped (`MathematicalCommand::SetLocale` / `MathematicalConfigMutation::SetLocale` / local `SetLocale`) — disambiguated by the construction site `SetLocale { value: "de-DE".into() }`, which only matches the local struct's single `value: String` field, not either enum variant.
- `🎮️commands/🕸️set-algorithm/🦀️component.rs` test module — missing `use crate::apps::mathematical::commands::{node_graph_edit, node_graph_viewport, set_directed};`. Note: the test module also defines a local helper `fn node_graph_edit(...)`; this does not collide with importing the module `node_graph_edit`, since fn (value namespace) and module (type namespace) are disjoint — verified by successful compile after the fix, not assumed.

Verification (mandatory exact form, run once each):
```
touch ➗️mathematical/📦️packages/🦀️rust/📦️glue.rs
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-mathematical --all-targets   → 0 errors
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test  -p semio-s-plugin-mathematical --lib            → 248 passed; 14 failed; 0 ignored; finished in 2.22s
```
The 14 failures are all inside `cas::*` / `polynomial::*` (integrate, limits, ode, sums, algebraic, finite, univariate) plus one `schema::mutations::…::insert_point_inverse_is_remove_point_at_same_index` — none in `apps::mathematical::commands::*` or anything this wave touched. These are explicitly out of scope ("our migrated `cas-internals`/`polynomial-internals`… all correct" / "do not touch `🧬️mutations/**` vocabulary"). No file under `cas`, `polynomial`, or `mutations` was edited this wave. Recording as this crate's new test baseline: **248 passed / 14 failed**, all pre-existing content failures unrelated to command-consolidation.

## `semio-s-plugin-remodel` — before: 41 errors, after: 0

Census (from `cargo check -p semio-s-plugin-remodel --all-targets`):
- 6× E0432 unresolved imports of grouping modules `calibration`, `ingest`, `params`, `reset`, `shell`, `view` under `commands`
- 35× E0433 (flat command modules — `add_gcp`, `place_gcp_observation`, `remove_gcp`, `calibrate_cameras`, `import_frame_payload`, `import_video_bytes_payload`, `import_video_done`, `add_stream`, `set_stream_sync`, `remove_stream`, `set_sfm_params`, `set_geo_params`, `set_mesh_params`, `set_feature_params`, `set_match_params`, `set_dense_params`, `set_motion_params`, `retry_stage`, `clear_result` ×2, `clear_sparse`, `clear_dense`, `clear_mesh_result`, `clear_tracks`, `clear_geo_products`, `import_video`, `export_qc_report`, `set_camera`, `set_layer_visibility`, `set_frame_cursor`, `set_report_table`, `set_active_utility`, `set_locale` — not found in scope)

`📦️glue.rs` (`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs`, lines 774–855) mounts `apps::remodel::commands` **flat** — 38 sibling modules, no grouping submodules `calibration`/`ingest`/`params`/`reset`/`shell`/`view` exist anywhere in it. Confirmed by grep: zero hits for any of those six names as `pub mod` anywhere in glue.rs. `🎛️apps/📸️remodel/🦀️component.rs` itself carries a doc comment directly above the flat import block (lines 188–189, untouched by this wave): *"`app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*` payload module is imported here under its own flat name."* — that comment, plus the flat glue mount, is the decisive evidence the six-module grouping was the abandoned UCAS script's unfinished intent, never realized in glue.rs, and the correct repair is to flatten every reference back out, not to invent the six submodules.

Fixes (all "flatten the stray grouped import back to `commands::X`" or "add the missing flat sibling `use`" — no grouping was invented):
- `🎛️apps/📸️remodel/🦀️component.rs:14` — removed `use crate::apps::remodel::commands::{calibration, ingest, params, reset, shell, view};` (the six fake groupings, unused after the flatten below).
- Same file, former lines 191–196 — `use calibration::{...}` / `use ingest::{...}` / `use params::{...}` / `use reset::{...}` / `use shell::{...}` / `use view::{...}` → six `use crate::apps::remodel::commands::{...};` statements, same member lists, now importing directly from the real flat module.
- `📌️panels/🗂️media/🦀️component.rs:4` — `commands::shell::REMODEL_MEDIA_ACCEPT` → `commands::import_frames::REMODEL_MEDIA_ACCEPT`. Evidence: `REMODEL_MEDIA_ACCEPT` is `pub const` in **both** `🎮️commands/🐚️import-frames` and `🎮️commands/🐚️import-video` (identical string value in both — a pre-existing duplication, left untouched); chose `import_frames` because the panel's drop zone dispatches `"importFramePayload"` (matches import-frames' role), not a video-only action.
- `📌️panels/🗂️media/🦀️component.rs` test mod, and `🎮️commands/🚀️run-reconstruction/🦀️component.rs` test mod — both called `commands::ingest::testkit_import_checker_stream`. That function does not live under any `ingest` module; it is `pub(crate) fn testkit_import_checker_stream` defined directly inside `🎮️commands/📥️import-frame-payload/🦀️component.rs` (also duplicated verbatim inside `import-video-bytes-payload` and `import-video-frame-payload`, each `pub(crate)`, each usable from its own module — a pre-existing triplication, left untouched). Repointed both call sites to `commands::import_frame_payload::testkit_import_checker_stream`.
- `🎮️commands/🚀️run-reconstruction/🦀️component.rs` — `RemodelCommand::RetryStage(super::retry_stage::RetryStage {...})`: `retry_stage` is a **sibling** of `run_reconstruction` under `commands`, not a child of it, so `super::retry_stage` never resolved (`super` here is `commands::run_reconstruction`'s own parent, `commands`, but `retry_stage` was never `pub` reachable that way inside the test submodule without an explicit `use`). Added `use crate::apps::remodel::commands::retry_stage;` and changed the call site to plain `retry_stage::RetryStage`. Left `super::RunReconstruction` alone — that one is correctly this module's own struct, one level up from `tests`.
- `⚙️set-ingest-params`, `🎯️edit-calibration`, `👁️set-selection`, `🧹️reset-placeholder-mesh`, `🐚️import-frames` — each `mod tests` already had `use super::*;` (for its own struct) but was missing `use crate::apps::remodel::commands::{…sibling command modules referenced in that file's tests…};`. Added the precise sibling list evidenced by grep of `<name>::` construction sites inside each file (e.g. `set-ingest-params` tests construct `set_dense_params::SetDenseParams{..}`, `set_feature_params::SetFeatureParams{..}`, etc. — every name that appears as `X::Y{ .. }` needed `X` imported).
- `🎮️commands/📥️import-frame-payload/🦀️component.rs` — two distinct issues at the same call site's neighborhood:
  1. `testkit_import_checker_stream`'s body wrote `import_frame_payload::ImportFramePayload{..}` — a **self**-reference (this file *is* module `import_frame_payload`; `ImportFramePayload` is defined earlier in the same file). Fixed by dropping the redundant `import_frame_payload::` prefix rather than importing the module into itself.
  2. `handle()`'s video-mime branch calls `import_video_bytes_payload::handle(&import_video_bytes_payload::ImportVideoBytesPayload{..}, doc, cfg)` — a genuine cross-module call (production code, not test-only). Added `use crate::apps::remodel::commands::import_video_bytes_payload;` at file top.
- `🎮️commands/📥️import-video-bytes-payload/🦀️component.rs` — its own copy of `testkit_import_checker_stream` calls `import_frame_payload::ImportFramePayload{..}`, which **is** a genuine cross-module reference here (this file is `import_video_bytes_payload`, not `import_frame_payload`). Added `use crate::apps::remodel::commands::import_frame_payload;` at file top.
- `🎮️commands/📥️import-video-frame-payload/🦀️component.rs` — same cross-module `import_frame_payload::ImportFramePayload{..}` self-vs-sibling distinction as above (this file is `import_video_frame_payload`, a third module, so the reference is genuinely cross-module): added the same top-level `use crate::apps::remodel::commands::import_frame_payload;`. Its `mod tests` separately referenced `import_video_bytes_payload::`, `import_video_done::`, `add_stream::`, `set_stream_sync::`, `remove_stream::` — added `use crate::apps::remodel::commands::{add_stream, import_video_bytes_payload, import_video_done, remove_stream, set_stream_sync};` to the test module (its `set_ingest_params::SetIngestParams` call site was already written fully-qualified as `crate::apps::remodel::commands::set_ingest_params::SetIngestParams`, left as-is).

Nothing was guessed at: every fix traces to either (a) `📦️glue.rs`'s actual flat mount list, (b) a `pub` item's actual defining file found by grep, or (c) rustc's own single-candidate suggestion. No module or type was invented.

Verification (mandatory exact form, run once each):
```
touch 📸️remodel/📦️packages/🦀️rust/📦️glue.rs
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-remodel --all-targets   → 0 errors
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test  -p semio-s-plugin-remodel --lib            → 487 passed; 2 failed; 0 ignored; finished in 44.57s
```
The 2 failures:
- `apps::remodel::engine::images::tests::jpeg_decode_never_panics_on_truncated_input` — panics inside `⚙️engine/🖼️images/🦀️component.rs:975` (decode-truncation assertion).
- `apps::remodel::engine::reconstruction::tests::long::video_in_yields_watertight_mesh_out` — panics inside `⚙️engine/🏭️reconstruction/🦀️component.rs:1460` ("expected a non-empty mesh, got 0 triangles").

Both are inside the photogrammetry engine's own algorithm code (`⚙️engine/*`), untouched by this wave and outside its scope (command-consolidation only). Recording as this crate's new test baseline: **487 passed / 2 failed**, both pre-existing content failures unrelated to command-consolidation.

## Refused-to-guess-at items

None. Every error in both crates traced to an honest destination (glue.rs's real flat mount, a `pub`/`pub(crate)` item's real defining file, or an unambiguous rustc suggestion disambiguated by the construction site's field shape). No TRAP-2-style `apps::` vs `artifacts::` type collisions were encountered in either crate this wave — both crates' command payload structs are unique per name.

## Files touched

- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/📄️set-artifact/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/📐️set-points/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/🗣️set-locale/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/🕸️set-algorithm/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/📌️panels/🗂️media/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/⚙️set-ingest-params/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🎯️edit-calibration/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/👁️set-selection/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🧹️reset-placeholder-mesh/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🐚️import-frames/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🚀️run-reconstruction/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/📥️import-frame-payload/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/📥️import-video-bytes-payload/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/📥️import-video-frame-payload/🦀️component.rs`

(`glue.rs` in both crates was only `touch`ed to defeat cargo's cache per the mandatory verification protocol — not edited.)
