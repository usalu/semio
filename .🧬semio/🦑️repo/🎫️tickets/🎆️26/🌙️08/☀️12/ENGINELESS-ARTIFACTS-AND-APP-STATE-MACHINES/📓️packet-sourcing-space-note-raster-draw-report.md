# Packet — sourcing / space / note / raster / draw

Five artifact `⚙️engine` directories dissolved: `🪵️sourcing`, `🪐️space`, `🗒️note`, `🖨️raster`, `🖍️draw`.
Executed by the orchestrator directly for `🪐️space` and `🖨️raster`; `🪵️sourcing`, `🗒️note`, `🖍️draw`
delegated to three parallel background agents with the same rule set (see their prompts for the exact
briefs — region-map, io_registry shadow hazard, module-nesting-not-uniform warning, hard rules). This
file is filled in as each slice completes; per-plugin worklogs are in `scratch-<plugin>-worklog.txt`
in this folder.

## Summary table (fill-in in progress)

| plugin | engine dir gone | structural grep (0 required) | compiler | destinations | notes |
|---|---|---|---|---|---|
| 🪐️space | ✅ deleted | 0 (excl. `apps::space::engine::`/`base64::engine::`, both legitimate) | pre-existing red ×2 reproductions, NOT mine (see below) | schema (2 helpers), io (io_registry) | `SHomeEngine` deleted outright, 0 external refs; broken never-compiled `dsl::ArtifactEngine` test dropped with it |
| 🖨️raster | ✅ deleted | 0 (excl. `base64::engine::`/`semio_s_plugin_stdio::…::engine::`, both legitimate) | ✅ green, exit 0, `Finished` in 3m30s | schema (DocumentHelpers+Tree), io (SemioBridge+MediaExport+MediaImport+io_registry), apps (Io region) | `RasterEngine` deleted outright, 0 external refs; ~50 external call sites repointed; found+fixed 1 real bug (missing `base64::Engine` trait import) before going green |
| 🪵️sourcing | pending (agent running) | pending | pending | pending | delegated |
| 🗒️note | ✅ deleted | 0 (6 legitimate stdio-engine false positives) | ✅ green, exit 0, `Finished` in 6m39s | schema (DocumentHelpers), io (MediaExport+MediaImport+io_registry) | delegated; `NoteEngine` deleted outright, 0 external refs; 62 external call sites fixed; 9/9 tests preserved |
| 🖍️draw | ✅ deleted | 0 (1 legitimate stdio-engine false positive) | ✅ green, exit 0, `Finished` in 2m36s | schema (DocumentHelpers+Tree+SegmentGeometry+KernelResolve), io (SemioBridge+io_registry), apps (Io region) | delegated; `DrawEngine` deleted outright, 0 external refs; 33 external call-site files fixed; 34/34 tests preserved; `🔄️fsm` scope trap respected (verified untouched) |

## 🪐️space — done by orchestrator

**Source file**: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (138 lines).
Module mounted at `standards::v1::engine` directly (glue.rs:49-50) — NOT under `subsets::any::engine`.

**Regions found and destinations:**
- `🔖️DocumentHelpers` (`empty_shome_snapshot`) → `🧬️schema/🦀️component.rs` (new `🔖️DocumentHelpers` region). Rule 3.
- `🔖️SchemaRegistry` (`artifact_schema_registered`) → same new region in `🧬️schema/🦀️component.rs`. Rule 3 (closest fit; a schema-registry query helper, no better-matching bucket).
- `🔖️ArtifactEngine` (`struct SHomeEngine`, `impl SHomeEngine { new }`) → **DELETED OUTRIGHT**. Verified zero external references (`grep -rn "SHomeEngine\b" ✏️s/🔌️plugins/🪐️space` → only its own definition/test). Its sole consumer was the engine file's own test `engine_apply_updates_catalog_generation`, which called `SHomeEngine::apply(...)` via `use dsl::ArtifactEngine;` — **that trait has zero definitions/impls anywhere in shipped source** (`grep -rn "trait ArtifactEngine" ✏️s 🧰️framework` → 0), so this test could never have compiled; it is a pre-existing dead reference to the never-shipped trait this ticket repeals, not a surviving assertion. Not carried forward.
- `🚪️DerivedIoRegistry` (`io_registry` module: `ComposerEntry`/`composer_entry_of`/4 export composers) → `🚪️io/🦀️component.rs` (new `🚪️DerivedIoRegistry` region, alongside the pre-existing `🎹️DerivedComposition` region). Rule 5. Moved verbatim; all internal paths were already `crate::`-qualified, so nothing needed re-qualifying inside the moved body.

**Call site fixes:**
- Artifact root `component.rs`: `declaration()`'s `.composers(…)` call and the root's own shadowing `io_registry`'s `use … as v1` both repointed from `crate::artifacts::home::standards::v1::engine::io_registry` → `crate::artifacts::home::standards::v1::subsets::any::io::io_registry`.
- `📦️glue.rs`: removed the `#[path] pub mod engine;` mount under `standards::v1`, and removed the `pub mod engine { pub use super::standards::v1::engine::*; }` shim under `artifacts::home`. Verified **zero** other call sites anywhere in the plugin referenced `artifacts::home::engine::` — the only two references were the root's own `declaration()` and shadow module, both fixed above.

**Tests**: engine file had 2 `#[test]` fns. `empty_snapshot_uses_home_schema` moved verbatim to a new `#[cfg(test)] mod tests` in `🧬️schema/🦀️component.rs`. `engine_apply_updates_catalog_generation` dropped (dead code, see above — never compiled). **Assertion delta: -1 test, justified** (the dropped test's own subject, `SHomeEngine`, was deleted per the ticket's own D5a ruling, and the test never compiled in the first place).

**Files touched**: edited `🚪️io/🦀️component.rs`, `🧬️schema/🦀️component.rs`, artifact root `🦀️component.rs`, `📦️packages/🦀️rust/📦️glue.rs`; deleted the `⚙️engine` directory (1 file). No new files.

**Structural verification:**
```
$ find ✏️s/🔌️plugins/🪐️space -path "*🗿️artifacts*" -name "⚙️engine" -type d
(empty)
$ grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🪐️space --include="*.rs"
→ only `apps::space::engine::…` (18 hits, the app's own PRE-EXISTING, legitimate engine — a different,
  unrelated module the ticket explicitly keeps: "An app has an engine, and that engine is a state
  machine") and `base64::engine::…` (4 hits, external crate API, unrelated symbol collision). Zero hits
  on the artifact-tree engine.
```

**Compiler**: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-space --all-targets`.
A concurrent long-running check (started before my edits landed, finished after) surfaced **6 pre-existing
errors, none touching my changes**:
```
error[E0609]: no field `document` on type `&OsAppRegistration`
  --> …/🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs:79
error[E0609]/[E0560] (×4): CsvSnapshot has no field `headers`/`rows` (has `has_header`/`records` instead)
  --> …/🗿️artifacts/🏠️home/…/🚪️io/📥️import/…/csv/…component.rs:9
  --> …/🗿️artifacts/🏠️home/…/🚪️io/📤️export/…/csv/…component.rs:11
error[E0308]: `?` operator cannot convert serde_json::Value to JsonValue
  --> …/🗿️artifacts/🏠️home/…/🚪️io/📤️export/…/json/…component.rs:10
```
**Attribution**: none of these 5 files were touched by this packet. `git log --oneline -3` on the csv
deserializer/serializer and the catalogue panel show their last commits at flags 480/440/478, well before
this ticket's window — `CsvSnapshot`'s shape (`records`/`has_header` only) is owned by `🗄️stdio`'s own
csv artifact and has drifted out from under space's csv leaves; this is exactly the documented
"`semio-s-plugin-stdio` is currently RED and every plugin depends on it" situation, plus an independent
pre-existing `OsAppRegistration` field drift. **Not mine, upstream, unrelated to the engine dissolution.**
A second, later probe of `semio-s-plugin-stdio` itself independently confirms it is currently red for an
unrelated dangling-`#[path]`-mount reason (see 🖨️raster section below) — space's own check above pre-dates
that regression (it got past stdio compilation to its own 6 errors), so the two stdio-red states are not
even the same failure; stdio has been flapping through several distinct red states during this session,
consistent with `📓️io-registry-shadow-list.md`'s "verification is a timestamp, not a property" warning.

**Re-run after stdio recovered** (background poll confirmed `cargo check -p semio-s-plugin-stdio
--all-targets` → `Finished` in 10m07s, exit 0): re-ran `cargo check -p semio-s-plugin-space --all-targets`
a second time. **Identical 6 errors, same files, same line numbers, same error codes** (E0308/E0560/E0609)
— stable and reproduced twice independently of stdio's state, confirming these are genuinely space-local/
stdio-schema-drift pre-existing issues, not a stdio compile failure and not caused by this packet's edits.

## 🖨️raster — done by orchestrator

**Source file**: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (786 lines).
Module mounted at `standards::v1::engine` directly (glue.rs:38-39) — NOT under `subsets::any::engine`.

**Regions found and destinations:**
- `🔖️Constants` (`SEMIO_RASTER_EXAMPLE_TEXT`) + `🔖️DocumentHelpers` (`create_raster_id`, `empty_raster_snapshot`, `🔖️Tree` sub-region: `layer_node_id/layer_name/layer_visible/layer_opacity/layer_blend_mode/layer_transform/find_layer/locate_layer/flatten_raster_layers`, `create_pixel_layer/create_group_layer/create_adjustment_layer/create_layer_of_kind`, `empty_raster_document`, `semio_fixture_snapshot`, `semio_example_document`, `semio_example_json`, `clone_layer`) → `🧬️schema/🦀️component.rs`, new `🔖️DocumentHelpers` region. Rule 3. `create_pixel_layer` was promoted `fn` → `pub fn` (needed cross-module by the io-side `MediaImport` functions, which build named/sized pixel layers rather than generic ones).
- `🔖️SemioBridge` (dialect consts, `ensure_stdio_semio_and_png_registered`, `semio_io_key`, `semio_transform_from_raster`, `draw_node_for_raster_layer`, `drawing_snapshot_from_raster`, `drawing_snapshot_from_dwg`, `dispatch_drawing_to_svg`, `semio_image_from_png_bytes`, `png_bytes_from_semio_image`, `canonicalize_png_bytes`) + `🔖️MediaExport` (`raster_document_json_to_svg`) + `🔖️MediaImport` (`raster_document_json_from_dwg`, `raster_image_layer_and_asset`) + `🚪️DerivedIoRegistry` (`io_registry` module) → `🚪️io/🦀️component.rs`. Rule 5 (sniff/codec dispatch). `canonicalize_png_bytes` promoted `fn` → `pub fn` (needed cross-module by the app's `raster_composite_media`).
- `🔖️Io` (`raster_io`, `raster_image_in_port`, `raster_image_out_port`, `raster_composite_media`) → `🎛️apps/🖨️raster/🦀️component.rs`, new `🔖️Io` region (right after the existing `🔖️RasterPlayApp` region, whose `io()`/`export_media` trait methods call straight into it). Rule 4.
- `🔖️ArtifactEngine` (`struct RasterEngine`, `impl RasterEngine { new, into_snapshot }`) → **DELETED OUTRIGHT**. Verified zero external references.

**Call site fixes (the bulk of this packet's work — raster's engine functions were used ~50 times across the plugin):**
- Artifact root `component.rs`: `declaration()`'s `.composers(…)` and the shadow `io_registry`'s `use … as v1`, both `…standards::v1::engine::io_registry` → `…standards::v1::subsets::any::io::io_registry`.
- Every `crate::artifacts::raster::engine::X` call site plugin-wide: mechanically remapped to `crate::artifacts::raster::schema::X` (34 sites across 30 files — mutation/diff/snapshot leaves under `🧬️schema/🧬️mutations/**`, `🧬️schema/📸️snapshot/**`, `🧬️schema/🔺️diff/**`, io deserializer leaves under `🚪️io/📥️import/…`, and app panels/commands), **except** the two symbols that actually moved to `io::` instead: `raster_document_json_from_dwg` (1 site, the dwg deserializer) and `raster_image_layer_and_asset` (1 site, the app's `import_media`) — both hand-fixed after the bulk remap, verified by re-grepping for `schema::raster_document_json_from_dwg`/`schema::raster_image_layer_and_asset`/`schema::raster_document_json_to_svg`/`schema::canonicalize_png_bytes` → 0 hits (would indicate a missed remap).
- `🎛️apps/🖨️raster/🦀️component.rs`: import line rewritten (`raster_composite_media`/`raster_io` are now defined locally in this same file, so dropped from the `use`; `semio_example_json` repointed to `schema::`); added `use base64::Engine as _;` (needed by the relocated `raster_composite_media` body, previously supplied by the engine file's own top-level `use base64::Engine;`).
- `📦️glue.rs`: removed the `#[path] pub mod engine;` mount under `standards::v1` and the `pub mod engine { pub use super::standards::v1::engine::*; }` shim under `artifacts::raster`.

**Tests**: engine file had 5 `#[test]` fns. `imports_dwg_polyline_into_raster_document` + `imports_empty_dwg_into_blank_raster_document` → moved to `🧬️schema/🦀️component.rs`'s new test module (calling `crate::artifacts::raster::io::raster_document_json_from_dwg`). `raster_image_layer_and_asset_builds_a_pixel_layer_and_matching_asset` → moved to a new `🧪️Tests` region in `🚪️io/🦀️component.rs`. `raster_io_declares_image_in_and_image_out` + `raster_composite_media_exports_structured_2d_image_payload` → moved to the app's existing `mod tests` block. **Assertion delta: 0 — all 5 tests carried forward verbatim**, only their `use`/qualification updated to match new locations.

**Files touched**: edited `🧬️schema/🦀️component.rs`, `🚪️io/🦀️component.rs`, artifact root `🦀️component.rs`, `🎛️apps/🖨️raster/🦀️component.rs`, `📦️packages/🦀️rust/📦️glue.rs`, plus 29 leaf files across `🧬️schema/🧬️mutations/**`, `🧬️schema/📸️snapshot/**`, `🧬️schema/🔺️diff/**`, `🚪️io/📥️import/…` deserializers, and 3 app panel/command files (mechanical `engine::` → `schema::`/`io::` path fixes only, no logic changes). Deleted the `⚙️engine` directory (1 file). No new files.

**Structural verification:**
```
$ find ✏️s/🔌️plugins/🖨️raster -path "*🗿️artifacts*" -name "⚙️engine" -type d
(empty)
$ grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🖨️raster --include="*.rs"
→ 14 hits total: all `base64::engine::general_purpose::…` (external crate API) or
  `semio_s_plugin_stdio::artifacts::{semio,png}::…::engine::…` (stdio's OWN engine modules — a
  different, out-of-scope plugin; raster legitimately calls into stdio's real png/semio codec engines
  as a cross-plugin dependency, not a struct instantiation). Zero hits on raster's own artifact engine.
```

**Compiler**: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-raster --all-targets`
→ **blocked before reaching raster at all**:
```
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`:
No such file or directory (os error 2)
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:7024:37
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```
**Attribution**: this was `semio-s-plugin-stdio` itself, a dangling `#[path]` mount left behind by another
session's in-flight mutation-vocabulary rename under `✳️mesh` — exactly the documented pattern in this
ticket's own `📓️packet-manifest.md` ("A rename landed on directories, left the `#[path]` mount behind —
third instance of this pattern today"). `semio-s-plugin-raster` was never actually compiled by that run;
the compiler never got past its dependency, `semio-s-plugin-stdio`. Not touched, not fixed here, per the
explicit instruction not to patch stdio.

**Re-run after stdio recovered** (background poll confirmed `cargo check -p semio-s-plugin-stdio
--all-targets` → `Finished` in 10m07s, exit 0): re-ran raster's own check.
- **First re-run**: 1 real error, MINE — `error[E0599]: no method named 'decode' found for struct
  'GeneralPurpose'` at `🧬️schema/🦀️component.rs:379` (inside the relocated `semio_fixture_snapshot`,
  which calls `base64::engine::general_purpose::STANDARD.decode(...)`) — the engine file's own top-level
  `use base64::Engine;` didn't travel with the relocated body. Fixed: added `use base64::Engine as _;` to
  `🧬️schema/🦀️component.rs`'s top-of-file imports (the `🚪️io/🦀️component.rs` and app-file copies of this
  same relocated code already had their own `Engine` import, added correctly the first time).
- **Second re-run**: **green.**
```
warning: `semio-s-plugin-raster` (lib) generated 36 warnings (run `cargo fix --lib -p semio-s-plugin-raster` to apply 31 suggestions)
warning: `semio-s-plugin-raster` (lib test) generated 39 warnings (34 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 3m 30s
EXIT:0
```
`grep -c "^error"` on the full log → **0**. All 36/39 warnings are pre-existing (unused-import/dead-code/
unused-variable style, in both touched and untouched files) — none are new compile errors. **Net result:
`semio-s-plugin-raster --all-targets` compiles clean.**

## 🗒️note — delegated agent, STATUS: green (full worklog: `scratch-note-worklog.txt`)

**Source file**: `…/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (1109 lines), module
mounted at `standards::v1::engine` (verified via `📦️glue.rs`, not assumed).

**Destinations:**
- `NoteEngine` (struct+impl) → **DELETED OUTRIGHT** (zero external refs, confirmed by grep before deleting).
- Pure document helpers (`create_note_id`, `semio_example_snapshot/json`, `empty_note_snapshot`, block
  tree helpers `block_id/name/kind/visible/locked/icon`, `block_tree_row_id`, `find_block(_location)`,
  `flatten_blocks`, `create_block_by_kind`, `remove_block_from_tree`, `reid_block_tree`, `clone_block`,
  `offset_block_tree`, `insert_after/block`, `update/mutate_block_in_tree`, `block_bounds`,
  `patch_block_field`) → `🧬️schema/🦀️component.rs`, new `🔖️DocumentHelpers` region.
- MediaExport/MediaImport bridge (`note_document_bounds`, `note_document_to_svg`,
  `note_document_json_to_svg`, `note_document_json_from_dwg`, `ensure_semio_drawing_bridge_registered`,
  etc.) + the real `io_registry` → `🚪️io/🦀️component.rs`, added alongside its pre-existing
  `derived_composition` module (not replaced).
- **Nothing moved to `💡️inferences/`** (no derived-compute helper existed in the engine file — note's
  only inference already lives elsewhere) and **nothing moved to `🎛️apps/`** (no `AppIo` builder, no
  `register*()` wiring existed; `declaration()`/`pilot_languages()` were already relocated by an earlier
  pass and correctly left alone). Reported as an honest "nothing to do here" rather than inventing a
  destination to match the map.

**Shadow trap**: root's shadowing `io_registry` present and handled — both the `declaration()` composers
call and the shadow's own `use … as v1` repointed by full canonical path
(`standards::v1::subsets::any::io::io_registry`), never via the ambiguous `crate::artifacts::note::io::`
shim.

**Call sites**: 62 files referenced `crate::artifacts::note::engine::X`; all fixed — 57 to `schema::`, 5
hand-corrected to `io::` (including one file whose `use {flatten_blocks, note_document_bounds}` had to be
*split* across both modules since the two symbols now live in different places).

**Tests**: 9 before (`git show HEAD:<engine-file> | grep -c '#\[test\]'`) → 9 after (1 in schema, 8 in
io). **Delta: 0.**

**Compiler**: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-note --all-targets`
→ **exit 0, `Finished` in 6m39s, zero `^error` lines.** Self-reported run history in full: run 1 was red
with 2 lib + 4 lib-test errors (`NoteTextParagraph`/`NoteTextRun` not imported after relocation) — these
were the agent's own bugs, fixed rather than attributed away; run 2 was killed by the harness mid-lock-wait
(exit 144, shared-target contention, not a real failure); run 3 is the green one reported above. All
remaining warnings proven pre-existing via `git show HEAD` on the same lines. Net new warnings introduced: 0.

**Files touched**: 0 created, 68 edited (glue.rs, artifact root, schema/component.rs, io/component.rs,
and 64 call-site leaf files), 1 file + 1 dir deleted (the engine component + its directory).

## 🖍️draw — delegated agent, STATUS: green (full worklog: `scratch-draw-worklog.txt`)

**Source file**: `…/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (2054 lines, the
largest of the five), module mounted at `standards::v1::engine` directly (verified via `📦️glue.rs`).
`✏️s/🔌️plugins/🖍️draw/🔄️fsm/**` (a sibling state-machine crate pair belonging to a different workstream
of this same ticket) was explicitly out of scope and confirmed untouched (`git status --porcelain` empty,
both its `Cargo.toml`s intact).

**Destinations:**
- `DrawEngine` (struct+impl) → **DELETED OUTRIGHT** (0 external refs, confirmed by grep before deleting).
- Pure document helpers/types (scene node types, id/tree/kind helpers, layer constructors, path-segment
  geometry — hex/rgba, transform↔matrix, curve sampling/flattening, boolean/trace layer resolution,
  kernel segment conversion, `artifact_schema_registered`) → `🧬️schema/🦀️component.rs`, new
  `🔖️DocumentHelpers` region (with `🔖️SceneTypes`/`🔖️Tree`/`🔖️SegmentGeometry`/`🔖️KernelResolve`
  sub-regions) + a new `🧪️Tests` region (the destination file had none before).
- `io_registry` + the semio/drawing↔svg bridge (`ensure_semio_drawing_bridge_registered` — confirmed
  present, contrary to an earlier report's suspicion that draw might be missing this pattern) →
  `🚪️io/🦀️component.rs`, added alongside its pre-existing `🎹️DerivedComposition` region (not replaced).
- `draw_io`/`draw_vector_out_port`/`draw_vector_media` (`AppIo`-returning) → `🎛️apps/🖍️draw/🦀️component.rs`,
  new `🔖️Io` region. The empty `🎛️apps/🖍️draw/⚙️engine` stub was confirmed still empty and left alone.
- Nothing routed to `💡️inferences/` (draw's only such compute, `compute_draw_topology`, already lives
  there and was untouched) — reported explicitly rather than assumed.

**Shadow trap**: confirmed real and handled at all three affected sites (root `declaration()`, the root's
own shadow module's `as v1` alias, and the real `io_registry`'s own internal call into the relocated
`draw_document_to_semio_drawing`) — all fully re-qualified onto `standards::v1::subsets::any::io::io_registry`
/`crate::artifacts::draw::io::`, never left as a bare/ambiguous name.

**Call sites**: 33 files outside the engine referenced `crate::artifacts::draw::engine::X`; all rewritten
to `crate::artifacts::draw::schema::X` (5 app panel/command files, 6 io deserializer leaves, 20 schema
mutation/snapshot/diff leaves, plus the app root itself, which now calls its own local `draw_io()`/
`draw_vector_media()` directly instead of importing them).

**Tests**: 34 before (`git show HEAD:<engine-file> | grep -c '#\[test\]'`) → 34 after (33 in schema, 1 in
io — the semio/svg bridge test). **Delta: 0.** Cross-checked plugin-wide test count too (133 in the
working tree vs 167 in `git grep HEAD`, where HEAD still double-counts the pre-relocation engine file
alongside the already-auto-committed destinations — 167 = 133 + the same 34, confirming no loss).

**Compiler**: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-draw --all-targets`.
Run 1 was red on `semio-s-plugin-stdio` (E0308, live churn — `git status --porcelain` on stdio showed an
uncommitted `M` + a batch of newly `A`dded files at that moment, i.e. another session mid-edit; draw's own
crate never got compiled). Run 2, after stdio settled, was green — `Finished` in 2m36s, 0 errors — and the
agent additionally found and fixed 2 warnings genuinely introduced by its own relocation (`unnecessary
qualification`: `std::collections::BTreeMap::new()` → `BTreeMap::new()`, redundant once inside a file that
already imports `BTreeMap` at its top) rather than leaving them. Run 3 re-confirmed clean after that
cleanup. `grep -c "^error"` on the full log → 0 in every green run.

**Files touched**: 0 created, 37 edited (glue.rs, artifact root, schema/component.rs, io/component.rs,
app root, 32 call-site leaf files), 1 file + 1 dir deleted.

**Self-reported gaps** (agent's own honesty section): `cargo test` was not run (only `--all-targets`
check, per the brief) — type-correctness of the relocated tests is proven, runtime behavior of the
relocated bridge test is not independently re-verified beyond compiling.

## 🪵️sourcing

Delegated to a background agent (dispatched with the full rule set, the shadow-io_registry hazard, the
module-nesting-is-not-uniform warning, and the pre-derived region map covering its three dependent
extension crates). Resumed once to wait out its own compiles after `semio-s-plugin-stdio` went green
mid-packet. Results pending; appended below on completion.
