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
| 🪐️space | ✅ deleted | 0 (excl. `apps::space::engine::`/`base64::engine::`, both legitimate) | pre-existing red, NOT mine (see below) | schema (2 helpers), io (io_registry) | `SHomeEngine` deleted outright, 0 external refs; broken never-compiled `dsl::ArtifactEngine` test dropped with it |
| 🖨️raster | ✅ deleted | 0 (excl. `base64::engine::`/`semio_s_plugin_stdio::…::engine::`, both legitimate) | blocked on stdio (upstream, see below) | schema (DocumentHelpers+Tree), io (SemioBridge+MediaExport+MediaImport+io_registry), apps (Io region) | `RasterEngine` deleted outright, 0 external refs; ~50 external call sites across the plugin repointed |
| 🪵️sourcing | pending (agent running) | pending | pending | pending | delegated |
| 🗒️note | pending (agent running) | pending | pending | pending | delegated |
| 🖍️draw | pending (agent running) | pending | pending | pending | delegated |

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
**Attribution**: this is `semio-s-plugin-stdio` itself, a dangling `#[path]` mount left behind by another
session's in-flight mutation-vocabulary rename under `✳️mesh` — exactly the documented pattern in this
ticket's own `📓️packet-manifest.md` ("A rename landed on directories, left the `#[path]` mount behind —
third instance of this pattern today"). `semio-s-plugin-raster` was never actually compiled by this run;
the compiler never got past its dependency, `semio-s-plugin-stdio`. Not touched, not fixed here, per the
explicit instruction not to patch stdio. A background poll (`scratch-stdio-poll.txt`) is retrying
`cargo check -p semio-s-plugin-stdio` every ~45s to catch it going green so raster can be re-verified;
results will be appended below once it resolves or the packet closes, whichever first.

## 🪵️sourcing / 🗒️note / 🖍️draw

Delegated to three parallel background agents (dispatched with the full rule set, the shadow-io_registry
hazard, the module-nesting-is-not-uniform warning, and — for sourcing specifically — the pre-derived
region map covering its three dependent extension crates). Results pending; appended below on completion.
