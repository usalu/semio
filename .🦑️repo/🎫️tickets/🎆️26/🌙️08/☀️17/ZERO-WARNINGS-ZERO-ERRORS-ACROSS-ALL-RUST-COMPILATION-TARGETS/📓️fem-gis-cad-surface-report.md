# fem / gis / cad / framework-surface — Warning Cleanup Report

Scope: drive the `(lib)` target of `semio-s-plugin-fem`, `semio-s-plugin-gis`, `semio-s-plugin-cad`,
`semio-framework-surface` to 0 warnings / 0 errors, per the parent ticket's assignment. `(lib test)`
targets were explicitly out of scope (blocked by another session's in-flight `Mutation::apply`/`::diff`
trait migration, plus one unrelated pre-existing missing-import bug noted below).

All four crates verified at **0 warnings / 0 errors** on their `(lib)` target via individual
`cargo check -p <crate>` runs (each reached a clean `Finished` with no warning/error line for the
crate itself). A workspace is under heavy **concurrent multi-session editing** (confirmed live during
this work — `semio-s-plugin-stdio`'s own warning count visibly dropped from 679 to ~109 over the
session purely from another session's parallel dead-code cleanup, and a transient stdio compile error
appeared/persisted near the end unrelated to anything touched here); combined multi-package
`cargo check` runs can fail-fast on that shared dependency, but each crate here was independently
confirmed clean when checked in isolation.

## semio-s-plugin-fem: 20 → 0 warnings

Files touched:
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- 2d/3d JSON import (`.../🚪️io/📥️import/🧩️deserializers/.../🔣️json/.../🦀️component.rs`) and export
  (`.../🚪️io/📤️export/🧵️serializers/.../🔣️json/.../🦀️component.rs`) leaves
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️标准/.../✏️editor/🎮️commands/🏋️add-load-case/🦀️component.rs`
- same dir's `🏋️add-combination/🦀️component.rs` and `🏋️set-self-weight/🦀️component.rs`

What was fixed:
- **Hidden lifetimes** (2): `fn compose(sources: &[ComposeSource])` → `&[ComposeSource<'_>]` in both
  2d and 3d `io` `ArtifactComposition` impls (`ComposeSource<'a>` genuinely has a lifetime param).
- **Unused imports** (2): `semio_framework_plugin::ArtifactAnalyzer as _;` in both 2d/3d `io`
  modules — confirmed dead by grep (the macro-generated `Fem2dAnalyzer`/`Fem3dAnalyzer::analyze` call
  doesn't need the trait in local scope; the generator wraps it internally).
- **Genuinely dead code, deleted** (real, crate-wide-grep-confirmed zero call sites, not even in
  tests):
  - `FEM3D_EXAMPLE_DSL` const in fem3d's editor `component.rs` — unlike its live fem2d sibling
    (`FEM2D_EXAMPLE_DSL`, still wired into `set-active-example` + tests), this one was truly
    unreferenced; its own doc comment falsely claimed 3 live call sites. Deleted the const and
    corrected the now-stale doc comment on `create_fem3d_app` that referenced it.
  - `index_of<T: HasId>` helper in both 2d/3d diff/text `component.rs` files — superseded by direct
    `.id()` calls everywhere, zero real callers.
  - Hand-rolled `json_value_to_serde`/`serde_value_to_json` (or `serde_to_json_value`) bridge pairs in
    all four fem 2d/3d JSON import/export leaves — superseded by `JsonSnapshot::to_serde_value()` /
    `JsonSnapshot::from_value()` methods that the real `deserialize`/`serialize` functions actually
    call. Same pattern the parent ticket had already confirmed for the **pptx** plugin; re-confirmed
    independently here rather than assumed.
  - Duplicated `resolve_load_case`/`add_load_mutation`/`next_load_id` helper trio, copy-pasted into
    `add-load-case`/`add-combination`/`set-self-weight` command files but only genuinely used by their
    siblings `add-nodal-load`/`add-member-udl`/`add-area-load` (confirmed via grep + reading each
    `handle()`). Deleted the dead copies and trimmed the now-unused `add_load`/`create_load_case`/
    `FemLoad`/`FemLoadCase` imports each file no longer needed.

## semio-s-plugin-gis: 67 → 0 warnings

Files touched:
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/...` and `🗺️gismap/...` — `🚪️io/🦀️component.rs`,
  JSON import/export leaves, plus mechanical fixes across `🧬️schema`/`✏️editor`/`📦️glue.rs` from
  `cargo fix`.

What was fixed:
- Ran `cargo fix --lib -p semio-s-plugin-gis --allow-dirty` first: cleared 55 of 67 warnings
  mechanically (unnecessary qualifications, unused imports, one unused-extern-crate in `glue.rs`).
- Remaining 12, same shape as fem's: 2 hidden lifetimes (`ComposeSource<'_>`) + 2 unused
  `ArtifactAnalyzer as _` imports (gisterrain/gismap `io` modules) + 8 dead
  `json_value_to_serde`/`serde_value_to_json` bridge functions across the 4 JSON leaves (gisterrain
  import/export, gismap import/export) — identical superseded-by-`to_serde_value`/`from_value`
  pattern as fem, independently re-confirmed via grep before deleting.

## semio-s-plugin-cad: 53 → 0 warnings

Files touched (many): `🗿️artifacts/📐️cad/🦀️component.rs`, `.../🚪️io/🦀️component.rs`,
`.../🚪️io/🗺️geometry-import/🦀️component.rs`, `.../🧬️schema/💡️inferences/🦀️component.rs` (largest),
`.../✏️editor/🦀️component.rs`, `.../✏️editor/🎭️modes/✏️edit/🦀️component.rs`,
`.../✏️editor/📌️panels/📄️artifact/🦀️component.rs`, `.../✏️editor/📌️panels/🔍️inspection/🦀️component.rs`,
`.../✏️editor/⚙️engine/🕹️interaction/🦀️component.rs`.

This crate's 53 warnings were mostly (46) `private_interfaces`: dozens of `pub fn`s taking/returning
`CadObject`/`CadGeometry`/`CadPrimitiveSlot`, all three of which are declared `pub(crate)` in
`🚪️io/🗺️geometry-import/🦀️component.rs`. Confirmed via crate-wide grep that **no crate outside
`semio-s-plugin-cad`** (only its own `🧩️extensions/*` siblings, none of which reference these
specific functions/types) calls any of the flagged items, so the correct fix — matching the types'
own already-declared `pub(crate)` scope — was narrowing each flagged `pub fn`/field/method to
`pub(crate)`, not widening the types to `pub`.

That visibility narrowing had a real second-order effect: `pub` items are exempt from the `dead_code`
lint regardless of actual use, so narrowing to `pub(crate)` unmasked **17 more warnings** — real,
previously-hidden dead code. Triaged each per the established methodology (grep whole crate including
`#[cfg(test)] mod tests` blocks):
- **Genuinely dead everywhere, deleted**: `CadWorkingScene::objects_for`/`geometry_for` (zero callers,
  not even tests) and `cad_object_from_solid_handle` in geometry-import (zero callers; its sibling
  `cad_object_from_mesh` is the one actually used) — plus the now-unused `mesh_from_indexed` import
  that only fed it.
- **Used only by tests, `#[cfg(test)]`-gated (not deleted)**: a whole "derive energy objects from
  shape geometry" pipeline (`run_derive_from_geometry` + its private helpers `dominant_axis_of`/
  `axis_normal_component`/`classify_rule_matches`/`fuse_solids`/`FaceMeta`/`next_object_id`/
  `FROM_GEOMETRY_CLASSIFY_RULES`), a building→structure typology mapper (`apply_from_building` +
  `BUILDING_TO_STRUCTURE`), `apply_typology_fallback`, a Jack `QueryableGraph` topology-query engine
  (`CadTopologyGraph::new` + `run_construct_query`), and the inspection panel's
  `object_inspector_group`/`primitive_inspector_group` builders. Every one of these has an explicit
  doc comment or inline test comment already documenting it as real, working, tested logic that lost
  its production call site in a prior refactor (e.g. `derive_transformation_populates_energy_pane`'s
  own comment: "`apply_transformation_mutations` is a documented no-op pending the child-dispatch
  seam ... this instead exercises the real derive algorithm directly"). Matches the ticket's
  established `#[cfg(test)]`-gate-don't-delete pattern exactly. Also gated the imports
  (`CadPrimitiveSlot`, `HashMap`, `typology_label`, `TYPOLOGY_CATALOG`, `CadObject` in the inspection
  file, `UiSelectItem`/`UiSelectNode`/`ui_inspector_mixed_text`/`ui_inspector_mixed_toggle`) that only
  those now-gated functions used, to avoid new unused-import warnings.
- Plus 1 hidden lifetime (`ComposeSource<'_>`), 1 unused `ArtifactAnalyzer as _` import, and 1 "unused
  doc comment" (a `///` doc comment sitting on a `thread_local!` block, not an item it can document —
  converted to a plain `//` comment).

**Verification caveat**: `cargo check -p semio-s-plugin-cad --tests` surfaces 3 pre-existing
`E0425 cannot find function run_derive_from_geometry` errors in
`✏️editor/🦀️component.rs`'s own `mod tests` (lines ~2034/2055/2060). Traced this carefully: no `use`
statement anywhere in that file (not `use super::*`, not the explicit `inferences::{...}` import list,
not `testkit`) ever brings `run_derive_from_geometry` into unqualified scope — this is independent of
visibility (`pub` vs `pub(crate)`) and independent of my `#[cfg(test)]` gating, so it predates this
session's edits and is unrelated pre-existing `(lib test)` breakage, not something introduced here.
Left untouched per the ticket's explicit "(lib test) out of scope" instruction — noted here for
whoever picks up `(lib test)` cleanup next.

## semio-framework-surface: 13 → 0 warnings

All 13 warnings were in one file: `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs` (the
raster-paint `RasterHost`/`RasterSession`).

- **2 `hidden_glob_reexports`** ("private item shadows public glob re-export"): the file does
  `pub use infinite_canvas::{self as canvas, *};` then separately `use canvas::camera::{Camera,
  Viewport};` and defines its own private `struct CameraJson`. Both `Camera` and `CameraJson` collide
  by name with items the glob also tries to publicly re-export from deep inside `infinite_canvas`
  (`board::Camera` via the board-ports re-export chain, `board::CameraJson`) — **confirmed via the
  compiler's own `hidden_glob_reexports` notes**, and confirmed these are genuinely different,
  incompatible types (an initial attempt to just drop the private `Camera` import and rely on the
  glob failed with 9 `E0308 mismatched types: expected Camera, found semio_framework_os_infinite::
  Camera` — reverted). Real fix: renamed the local private `CameraJson` → `DocumentCameraJson`
  (distinct name, no more collision — and since none of its fields were ever read either, and neither
  was the `DocumentJson.camera` field pointing to it, deleted the whole struct along with the other
  dead fields below rather than just renaming a corpse); for `Camera`, made the local import an
  explicit `pub use canvas::camera::Camera;` — an intentional public re-export always wins over a glob
  with no ambiguity, which resolves the warning honestly (this file's `Camera` really is the correct
  canvas-camera type, not the board one).
- **10 "field is never read"**: these are `#[derive(Deserialize)]`-only wire-shape DTOs
  (`LayerNodeJson`, `MaskJson`, `AdjustmentParamsJson`, the old `CameraJson`, `DocumentJson`,
  `LayerNode::Adjustment`) where `parse_layer`/`parse_document` explicitly discard several fields via
  `..`. Removed every field confirmed unread anywhere in the crate: `name`/`clip_to_below`/`filters`
  from `LayerNodeJson::Pixel`, `name`/`clip_to_below` from `::Group`, `name`/`transform` from
  `::Adjustment`, `linked` from `MaskJson`, `hue`/`saturation`/`levels_black`/`levels_white` from
  `AdjustmentParamsJson` (kept `brightness`/`contrast`, which `append_layer_node` does read),
  `id`/`camera`/`brush_size`/`brush_opacity` from `DocumentJson` (kept `schema`/`layers`), and `id`
  from the internal `LayerNode::Adjustment` (kept on the JSON-wire `LayerNodeJson::Adjustment` where
  it's still legitimately pattern-matched-out, just no longer carried into the internal enum). Since
  `filters: Vec<FilterJson>` itself was unread, and every one of `FilterJson`'s own 3 fields was
  independently flagged unread too, deleted `FilterJson` entirely (whole struct, zero remaining
  references). No `#[serde(deny_unknown_fields)]` anywhere in this file, so trimming these fields
  doesn't change what JSON documents are accepted — extra keys are silently ignored either way,
  identical practical behavior to before (where the code parsed them and then discarded via `..`).
- **2 "function never used"**: `apply_brightness_contrast`/`apply_blur_box` — real pixel-processing
  algorithms, thoroughly exercised by 6 tests (`apply_brightness_contrast_shifts_brightness`,
  `..._clamps_extremes`, `apply_blur_box_zero_radius_is_noop`, `..._preserves_uniform_image`,
  `..._smooths_a_sharp_edge`), but with no production call site — `append_layer_node`'s
  `"brightnessContrast"` adjustment-kind branch computes `brightness`/`contrast` and then explicitly
  `let _ = (b, c, opacity, blend);`s them away rather than calling `apply_brightness_contrast`. Same
  documented-gap shape as cad's derive pipeline; `#[cfg(test)]`-gated both rather than deleting
  tested, working code.

## Not touched / out of scope
- `semio-s-plugin-stdio` (dependency of fem/gis/cad): still has a large, actively-shrinking warning
  count (was 679 at ticket start per `📓️progress.md`, observed as low as ~109 during this session,
  purely from another session's concurrent work — not touched here) and, near the end of this
  session, one transient compile error (`E0560`, a DWG struct field mismatch) from that same
  concurrent editing. Neither is in this ticket's assigned scope.
- cad's `(lib test)` pre-existing `E0425` (see above) — out of scope per ticket instructions, flagged
  for whoever owns `(lib test)` cleanup.
- Did not touch any file under `.🦑️repo` or run any git command.
