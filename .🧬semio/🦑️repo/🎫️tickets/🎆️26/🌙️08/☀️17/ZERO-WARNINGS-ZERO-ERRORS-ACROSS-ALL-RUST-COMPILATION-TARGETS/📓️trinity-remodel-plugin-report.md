# Trinity + Remodel Plugin — Warning Triage Report

Scope: `semio-s-plugin-trinity` and `semio-s-plugin-remodel`, `(lib)` target only. `(lib test)` on
both crates still fails to compile with pre-existing errors from another session's in-flight
`Mutation::apply`/`::diff` trait-signature migration — untouched, out of scope per the ticket brief.

## semio-s-plugin-trinity

**29 warnings -> 0 warnings, 0 errors** (verified via `cargo check -p semio-s-plugin-trinity
--message-format=short`, re-run after each batch; final run shows no `semio-s-plugin-trinity`
warning-summary line at all, i.e. zero).

### Fixed: hidden-lifetime + unused-import (4 warnings, 2 files)
Both artifact IO leaves had the same `rust-2018-idioms` pattern: `ComposeSource` used without its
elided lifetime, and a dead `use semio_framework_plugin::ArtifactAnalyzer as _;` (the actual
`analyze()` calls resolve as inherent associated functions on `JackAnalyzer`/`RewriteAnalyzer`, not
via the trait, confirmed by no other use of `ArtifactAnalyzer` in either file).
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

### Fixed: dead_code, self-contained command-leaf duplicates (25 warnings, 6 files)
This crate wires each editor command as its own `#[path] mod component;` leaf
(`✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs`), and per its own comment "every command
file is self-contained (its own private copy of any shared helpers)". Several leaves carry a full
copy of helper functions but only call a subset of them — the rest were genuinely dead **in that
specific file**, confirmed by grepping the whole crate (not just the file) for real callers,
including `#[cfg(test)]` blocks, before touching anything:
- `jack/…/commands/🔎️run-query/🦀️component.rs` — deleted `fixture_dsl_for_preset` (never called
  anywhere in the crate, all 3 sibling copies dead too). Kept `preset_query` in this file: it's the
  one real copy re-exported crate-wide as `commands::query::preset_query` and consumed by the
  catalogue panel (`✏️editor/📌️panels/📚️catalogue/🦀️component.rs`).
- `jack/…/commands/🔎️load-example-query/🦀️component.rs` — deleted `preset_query` and
  `fixture_dsl_for_preset` (own unused copies; `run_jack_query`/`error_result_json` are used here).
- `jack/…/commands/🔎️format-document/🦀️component.rs` — the whole helper block
  (`run_jack_query`/`preset_query`/`error_result_json`/`fixture_dsl_for_preset`) was unused
  boilerplate never called by `format_document`; deleted all 4, plus now-unused imports
  (`JackSnapshot`, `TrinityGraphMutation`, `serde_json::json`).
- `♻️rewrite/…/commands/📜️node-graph-edit/🦀️component.rs` — deleted `add_rule_clause` and
  `patch_fixture_nodes` (unused own copies; real homes are the two files below). Removed
  now-unused `PropertyValue`/`ParameterKind` imports.
- `♻️rewrite/…/commands/📜️add-rule-clause-command/🦀️component.rs` — kept only `add_rule_clause`
  (the function this leaf's `add_rule_clause_command` actually calls); deleted the other 8 unused
  helper items (`RuleClauseRef`, `parse_fixture_json`, `apply_semantic_layout_edit`,
  `parse_clause_ref`, `remove_at`, `delete_rule_clause`,
  `apply_rewrite_node_graph_edit_operations`, `patch_fixture_nodes`) and pruned imports down to
  what `add_rule_clause`/`add_rule_clause_command` need (`Graph`, `JackSnapshot`, `Value` dropped).
- `♻️rewrite/…/commands/📜️patch-nodes/🦀️component.rs` — mirror image: kept only
  `patch_fixture_nodes` (what `patch_nodes` calls), deleted the same 8-item unused cluster and
  pruned `PropertyValue`/`ParameterKind`/`Rhs`/`Value` imports.

All deletions confirmed dead by crate-wide grep (zero call sites anywhere, including
`#[cfg(test)]`) before removal — nothing here was a same-crate-test-only false positive.

## semio-s-plugin-remodel

**18 warnings -> 0 warnings, 0 errors** (verified via `cargo check -p semio-s-plugin-remodel
--message-format=short`; final run shows no `semio-s-plugin-remodel` warning-summary line).

### Fixed: hidden-lifetime + unused-import (2 warnings, 1 file)
Same `ComposeSource`/`ArtifactAnalyzer as _` pattern as trinity.
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

### Fixed: dead_code, real test-only helper — gated, not deleted (1 warning)
- Same file: `semio_mesh_to_mesh_data` (the documented real inverse of `mesh_data_to_semio_mesh`)
  is only called from `#[cfg(test)] mod tests` in this same file (confirmed by grep — the only
  call sites are inside the test at line ~245). A plain `cargo check` doesn't compile
  `#[cfg(test)]` code, so this legitimately warns dead in this compilation despite being real,
  needed test fixture. Per the established idiom: added `#[cfg(test)]` to the function itself
  rather than deleting it.

### Fixed: dead_code, self-contained command-leaf duplicates (3 warnings, 3 files)
Same "self-contained per-leaf helper" shape as trinity's rewrite commands:
- `…/commands/🧹️clear-result/🦀️component.rs` — deleted unused own copy of `placeholder_result`
  (kept `empty_result`, which `handle` actually calls); dropped now-unused `mesh_from_kind` import.
- `…/commands/🧹️clear-mesh-result/🦀️component.rs` — same: deleted unused `placeholder_result`,
  dropped `mesh_from_kind` import.
- `…/commands/🧹️reset-placeholder-mesh/🦀️component.rs` — mirror image: deleted unused
  `empty_result` (this leaf's `handle` calls `placeholder_result` instead), dropped now-unused
  `MeshData` import.
  Verified via crate-wide grep that `placeholder_result`/`empty_result` have zero cross-file
  callers — each leaf's copy stands alone.

### Fixed: dead_code, superseded hand-rolled converters (2 warnings, 2 files)
- `…/io/📥️import/🧩️deserializers/…/json/🔖️rfc8259/✳️any/🦀️component.rs` — `json_value_to_serde`
  was a hand-rolled `JsonValue -> serde_json::Value` structural converter, fully superseded by
  `JsonSnapshot::to_serde_value()` (what `deserialize()` actually calls). Zero callers anywhere in
  the crate (only self-recursive). Deleted, plus the now-unused `JsonValue`/`std::str::FromStr`
  imports.
- `…/io/📤️export/🧵️serializers/…/json/🔖️rfc8259/✳️any/🦀️component.rs` — mirror: `serde_to_json_value`
  fully superseded by `JsonSnapshot::from_value()` (what `serialize()` actually calls). Deleted,
  plus now-unused `JsonValue`/`JsonMember` imports.

### Fixed: dead_code, genuinely orphaned function (1 warning)
- `…/✏️editor/⚙️engine/🎥️video/🦀️component.rs` — `visual_sample_entry_box` (a minimal ISO-BMFF
  `VisualSampleEntry` box builder). Its own doc comment claimed it backs `mjpg`/`hvc1` muxing, but
  zero call sites exist anywhere in the crate (not even in tests) — `write_mp4_mjpeg` and friends
  build `Mp4Track`/`Mp4Codec::default()` directly through stdio's mp4 engine instead. Deleted along
  with its doc comment.

### Fixed: dead_code, orphaned duplicate of a real per-tick helper cluster (9 warnings, 2 files)
Remodel has THREE near-identical "VideoImportScratch" helper clusters (`VideoImportScratch` struct,
`BLUR_GATE_ROLLING_WINDOW`/`BLUR_GATE_MIN_SAMPLES` consts, `local_sharpness_score`,
`local_rolling_median`, `blur_gate_reject`, `rebuild_video_import_scratch`, `batch_stream_id`), one
per command leaf. Grepped crate-wide to find the one real, fully-wired copy:
`…/commands/📥️import-video-frame-payload/🦀️component.rs` (`ImportVideoFramePayload::handle` calls
all seven pieces) — that file had zero warnings and was left untouched.
- `…/commands/📥️import-frame-payload/🦀️component.rs` — this leaf's `handle` only ever calls
  `batch_stream_id` (kept); the rest of the cluster (`VideoImportScratch`,
  `BLUR_GATE_ROLLING_WINDOW`, `BLUR_GATE_MIN_SAMPLES`, `local_sharpness_score`,
  `local_rolling_median`, `blur_gate_reject`, `rebuild_video_import_scratch`) was an unused
  duplicate copy — deleted, plus now-unused `std::collections::VecDeque` import. This also
  surfaced a second-order warning: `use …images as remodel_image` became unused on the `(lib)`
  target because its only remaining use is inside `#[cfg(test)]` fixture functions
  (`checker_image`/`checker_data_url*`) — gated the import itself with `#[cfg(test)]` (same
  test-only idiom as `semio_mesh_to_mesh_data` above), rather than deleting real test fixtures.
- `…/commands/📥️import-video-bytes-payload/🦀️component.rs` — this leaf's `handle` uses
  `VideoImportScratch`/`local_sharpness_score`/`blur_gate_reject` directly but builds its own
  stream id inline (`next_remodel_id("stream")`) rather than via `batch_stream_id`, and never
  rebuilds scratch from persisted frames (`rebuild_video_import_scratch`) since it processes a
  whole video in one pure call. Deleted `rebuild_video_import_scratch` and `batch_stream_id` only;
  dropped now-unused `decode_still_image` import (was only used inside the deleted
  `rebuild_video_import_scratch`).

## Verification
Both crates re-checked after all edits with `cargo check -p <crate> --message-format=short`: no
`semio-s-plugin-trinity`/`semio-s-plugin-remodel` warning-summary line in either final run (i.e.
zero warnings), `grep -c "^error"` = 0 in both runs. `(lib test)` target on both crates was not
touched and still fails on the pre-existing cross-cutting `Mutation::apply`/`::diff` migration
noted in `📓️progress.md` — out of scope per the ticket brief.

## Files touched
Trinity (8 files):
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️run-query/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️load-example-query/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️format-document/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️node-graph-edit/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️add-rule-clause-command/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️patch-nodes/🦀️component.rs`

Remodel (9 files):
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-mesh-result/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️reset-placeholder-mesh/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-frame-payload/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-bytes-payload/🦀️component.rs`

All edits were auto-committed by the repo's live auto-commit process during this session (most
recently commit `1d71198c19` at time of writing); verified via `git log --oneline` and content
checks (e.g. `RuleClauseRef` no longer present in `patch-nodes/🦀️component.rs`) that nothing was
lost, and via `cargo check` that both crates still compile clean.

## Left alone (not in scope)
- `(lib test)` target on both crates: still fails on the pre-existing, cross-cutting
  `Mutation::apply`/`::diff` -> `MutationApplyResult<T>` trait migration described in
  `📓️progress.md` — another session's in-flight work, not attempted.
- `semio-s-plugin-stdio` (a dependency of both crates): warning count dropped from ~702 to ~324
  warnings over the course of this session purely from other concurrent sessions' work on that
  crate — not touched by this report's work, called out here only so the numbers in the raw
  `cargo check` transcripts aren't mistaken for something done here.

---

# Second wave: dag, mathematical, architect, vcs

Same scope rule: `(lib)` target only; `(lib test)` migration errors left untouched; no
`#[allow(...)]`. All four confirmed via `cargo check -p <crate> --message-format=short`, and each
was **re-verified a second time** after the whole batch was done (see "Re-verification" below) to
guard against the heavy concurrent-editing noise this session kept surfacing (see "Concurrent-edit
noise" below) — every crate showed 0 warnings, 0 errors both times.

## semio-s-plugin-dag: 2 -> 0 warnings, 0 errors
Only the by-now-familiar `derived_composition` IO-leaf pattern (seen in trinity/remodel/mathematical/
architect/vcs alike — looks like one shared codegen template across all these artifact plugins):
`ComposeSource` missing its elided lifetime, and a dead `use semio_framework_plugin::ArtifactAnalyzer
as _;` (the real `DagAnalyzer::analyze(...)` call resolves as an inherent associated function, not
via the trait — confirmed via grep, zero other `ArtifactAnalyzer` references in the file).
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

## semio-s-plugin-mathematical: 3 -> 0 warnings, 0 errors
Same IO-leaf lifetime/import pair, plus one new warning shape not seen in the first wave:
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
  — `ComposeSource` lifetime + dead `ArtifactAnalyzer as _`, same as dag.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
  — **"private item shadows public glob re-export"**: line 3 had an explicit
  `use crate::artifacts::mathematical::schema::diff::MathematicalDiff;`, but line 14 of the same
  file already does `pub use crate::artifacts::mathematical::schema::diff::*;`, which re-exports the
  same `MathematicalDiff` (confirmed it's genuinely defined in that `diff` module via grep). The
  explicit private import was pure redundant shadowing of the public glob re-export — deleted the
  explicit `use` line; `impl MathematicalDiff` later in the file still resolves fine via the glob
  (Rust `use` visibility isn't order-dependent within a module).

## semio-s-plugin-architect: 5 -> 0 warnings, 0 errors
**Flagged as an active-goal crate ("Architect" program/adjacency-matrix editing, due 2026-12-31) —
treated with the same conservative bar as norm: every fix below is a pure lint/import correction or
a confirmed-dead-with-zero-callers deletion, nothing that touches scaffolding logic or in-progress
feature shape.**
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
  — same `ComposeSource` lifetime + dead `ArtifactAnalyzer as _` pair.
- `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — `json_value_to_serde`,
  a hand-rolled `JsonValue -> serde_json::Value` converter fully superseded by
  `JsonSnapshot::to_serde_value()` (what `deserialize()` actually calls). Zero callers crate-wide
  (only self-recursive). Deleted, plus now-dead `JsonValue`/`std::str::FromStr` imports. Exact same
  shape as remodel's equivalent file from the first wave.
- `…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — mirror:
  `serde_to_json_value` superseded by `JsonSnapshot::from_value()`. Deleted, plus now-dead
  `JsonValue`/`JsonMember` imports.
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️component.rs` — **"unused doc comment"** at
  `PROGRAM_BENCHMARKS_SCRATCH`'s `thread_local! { ... }` block: the 13-line doc comment describing
  the scratch cache's contract (staleness gap, `EngineRep` semantics, mirrors
  `➗️mathematical::MATH_SCRATCH`/`📕️norm::EN1990_QK_SCRATCH`) was written *before* the
  `thread_local!` macro invocation itself, which doesn't have a doc-carrying slot there — the
  `thread_local!` macro only forwards `#[$attr:meta]` written *inside* the braces, immediately
  before the `static` item. This is a pure placement fix, not a content change: moved the exact same
  doc comment inside the braces, directly above `static PROGRAM_BENCHMARKS_SCRATCH`, where it now
  actually attaches and documents the item as originally intended. No logic touched.

## semio-s-plugin-vcs: 6 -> 0 warnings, 0 errors
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — same
  `ComposeSource` lifetime + dead `ArtifactAnalyzer as _` pair.
- Self-contained command-leaf duplicate helpers (same shape as trinity's rewrite commands and
  remodel's clear-result family): `patch-snapshot`/`text-edit`/`edit` each carry their own private
  copy of `vcs_patch_operation_for_field`, `vcs_demo_projection_diff_operations`, and
  `text_edit_operations`, but each leaf's own `handle` only calls a subset:
  - `…/✏️editor/🎮️commands/🩹️patch-snapshot/🦀️component.rs` — `handle` only calls
    `vcs_patch_operation_for_field` (kept); deleted this file's own unused copies of
    `vcs_demo_projection_diff_operations` and `text_edit_operations` (verified transitively dead:
    the latter's only caller in this file, `handle`, doesn't call it).
  - `…/✏️editor/🎮️commands/🩹️text-edit/🦀️component.rs` — `handle` calls `text_edit_operations`
    (which calls `vcs_demo_projection_diff_operations`, both kept); deleted this file's own unused
    copy of `vcs_patch_operation_for_field`.
  - `…/✏️editor/🎮️commands/🩹️edit/🦀️component.rs` — identical to `text-edit`: `handle` is a
    line-for-line alias calling `text_edit_operations`; deleted the unused
    `vcs_patch_operation_for_field` copy.
  Confirmed via crate-wide grep before deleting anything that none of these three functions has a
  cross-file caller — each leaf's copy stands alone, consistent with every other plugin's
  "self-contained command leaf" convention seen this session.

## Concurrent-edit noise hit while checking this batch (not caused by this work, not fixed)
Two transient `cargo check` failures on `semio-s-plugin-stdio` (a shared dependency, never touched
by this report) surfaced mid-session, both confirmed via `git diff --stat` showing live, uncommitted
changes on the exact failing file from another session, and both gone on retry:
1. `dag`'s first check hit `E0425 cannot find type CanonicalR2004Section` in stdio's DWG IO file —
   another session mid-edit on that type; retry succeeded clean.
2. `dag`'s re-verification pass hit `E0432 unresolved imports … dec_paragraph_bin … enc_slide_bin …`
   in stdio's PPTX diff/mutations files — this is the exact hand-rolled binary pptx codec deletion
   `📓️progress.md` already documented as genuinely dead and removed earlier this session; another
   session was mid-way through deleting the encoder/decoder functions from the `diff` module but
   hadn't yet updated the `mutations` file's imports of them. `git diff --stat` on that file showed
   live in-progress edits (951 and 162 lines respectively); retry succeeded clean once that session's
   edit finished landing.
Neither incident involved this report's target crates or files — noted here only so the raw
transcripts aren't mistaken for a bug introduced by this pass.

## Re-verification (after full batch)
Re-ran `cargo check -p <crate> --message-format=short` for all four crates a second time after
finishing the whole batch (partly to wait out the stdio noise above). All four: no
`semio-s-plugin-{dag,mathematical,architect,vcs}` warning-summary line in the output (i.e. zero
warnings), `grep -c "^error"` = 0. `semio-s-plugin-stdio`'s own warning count kept dropping across
these re-runs (322 -> ~108) purely from other concurrent sessions' unrelated work on that crate —
not this report's doing.

## Files touched (second wave)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️component.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️text-edit/🦀️component.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️edit/🦀️component.rs`

## Left alone (second wave, not in scope)
- `(lib test)` on all four crates: not attempted, same migration-blocked reasoning as the first
  wave.
- Note for whoever reads this next: this session observed the ticket's own infrastructure directory
  move live, mid-task — `.🦑️repo/` was renamed to `.🧬semio/🦑️repo/` by another concurrent session
  partway through this second wave (a repo-wide restructure, confirmed via `git status` showing it
  as a staged rename, not a deletion). This report file itself had to be re-located there to append
  this section. All six crates' code edits (first + second wave) were verified still intact and
  correctly compiling after that move.
