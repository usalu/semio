# W4 — `gis` composes stdio mesh (terrain) / drawing+image+value (map)

**ucas-status: complete — 171/171 tests, 0 compile errors, reproduced stable across two consecutive full runs**

## Pre-flight

`git status --porcelain -- ✏️s/🔌️plugins/🌍️gis`: one pre-existing uncommitted change was present at start
(`🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, 2 insertions/2 deletions — not mine,
not attributable to any live session via `git log`). It lived in the same file this migration heavily
edits; built on top of it rather than reverting — final compile/test is clean either way. See
`## Concurrent-churn observations`.

Baseline `cargo check -p semio-s-plugin-gis --all-targets` (before any edit): **3 pre-existing errors**
(`E0433`/`E0432`, `cannot find "modules" in crate` — a stale `use crate::modules::terrain::{...}` import
in `💡️inferences/🦀️component.rs` and in `🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs`, both
explicitly flagged in-file as "KNOWN PRE-EXISTING… reported by another session"). Fixed opportunistically
(one-line path corrections to the types' real home, `crate::artifacts::gisterrain::schema`) since both
files needed touching for this migration anyway and the fix is unambiguous — not counted as "my bug",
but not left broken either.

## What changed

### `gisterrain` composes `s.stdio.semio.mesh`

- `🗿️artifacts/🏔️gisterrain/🦀️component.rs`: deleted `mesh_artifact_kind()` (the duplicate `3d.mesh`
  `ArtifactKindSpec`) and its test — the second, gis-owned copy of the duplicate lowpoly already
  removed. Added `gis_terrain_mesh_child_handle`/`gis_terrain_mesh_content_key`/
  `gis_terrain_snapshot_with_derived_mesh` (content-addressed off `(exaggeration,
  imported_features_json)`, mirroring lowpoly's `mesh_child_handle`) and
  `gis_terrain_mesh_from_snapshot` — a REAL converter building an actual `SemioMeshSnapshot` (one flat
  quad, 2 triangles, sized by the imported-overlay bounding box, honestly flat-at-z=0 since gis3d has
  no DEM tessellator yet — the exact same documented gap `gis3d_scene_media` already carried).
- `🧬️schema/📸️snapshot/🦀️component.rs`: `GisTerrainSnapshot` gained `mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>`
  (`#[child(kind = "s.stdio.semio.mesh")]`). Dropped `#[derive(dsl::DslRecord)]` (no `DslField` impl for
  `ArtifactChild<S>`), hand-rolled `ArtifactDsl`/`ArtifactPack` + text (`exaggeration=`/
  `importedFeaturesJson=`/`mesh=` lines, tolerant of the fixture's own `origin`/`position` sidecar
  lines) + binary (LE f64 + length-prefixed string + child-opt) codecs, matching lowpoly/cad's
  hex/bracket convention exactly.
- `🧬️schema/🦀️component.rs` (`GisTerrainArtifact`): mirrored `mesh` field; `to_snapshot`/`from_snapshot`/
  `set_snapshot`/`empty_gis_terrain_snapshot` re-derive it so it never drifts from the two source fields.
- `🧬️mutations/🦀️component.rs`'s `apply_gis_terrain_mutation` and `🔺️diff/📝️text/🦀️component.rs`'s
  `GisTerrainDiff::apply` both re-derive `mesh` via `gis_terrain_snapshot_with_derived_mesh` after every
  mutation/diff-apply — the single source of truth, never independently mutable.
- `🎛️apps/🧊️3d/🦀️component.rs`: removed the `mesh_artifact_kind` import and
  `.artifact_kind(mesh_artifact_kind())` registration; updated `the_manifest_stitches_every_taxonomy_node`
  to assert `3d.mesh` is no longer independently registered (the media-port `kind_id`/`schema` string tags
  referencing the canonical kind by id are untouched — that's correct reuse, not a re-declaration).

### `gismap` composes `s.stdio.semio.drawing` + `s.stdio.semio.image` + `s.stdio.semio.value`

Kept `positions`/`routes`/`regions: Vec<MapFeature>` **inline** on `GisMapSnapshot` — this is gis's own
id-keyed feature vocabulary (not a duplicated stdio type), the same call cad made keeping `nodes`/
`references_by_model_definition_id` inline while composing `model`/`drawing` children alongside them
(`📐️cad`'s `📸️snapshot/🦀️component.rs` module doc, precedent cited in-file). All 12 existing mutation
triads (create/delete/replace-data/reorder × positions/routes/regions) are untouched and still fully
real/granular — no teardown, no no-op stubs needed, because the composed children never held that data
in the first place.

- `🗿️artifacts/🗺️gismap/🦀️component.rs`, new `🔖️Composition` region: `GisMapDrawingChild`/
  `GisMapImageChild`/`GisMapValueChild` type aliases; `gis_map_drawing_child_handle`/
  `gis_map_value_child_handle` (content-addressed, same shape as terrain's); real bidirectional
  `semio_value_from_serde_json`/`serde_json_from_semio_value` (`serde_json::Value` ↔ `SemioValue`,
  direct structural mapping, `Bytes`/`Ref` never produced going forward — mirrors stdio's own
  `json`-subset converter, written locally since that one bridges stdio's OWN `JsonValue` AST, not
  `serde_json::Value`); `gis_map_value_from_descriptor_json`/`gis_map_descriptor_json_from_value` (the
  value child's real content ↔ the existing `{positions,routes,regions}` descriptor JSON, losslessly);
  `gis_map_snapshot_with_derived_children` — the single re-derivation point every constructor/mutator
  funnels through. `drawing`'s actual content is `gis_map_snapshot_to_drawing` (already existed, unchanged
  — positions→markers, routes/regions→open/closed polylines). `image` stays honestly `Option::None`
  always — gis has no raster-basemap capture path today (the app's `render_mode` toggle picks a
  rendering STYLE of the same vector data, not a second raster document); the slot is real and typed,
  documented as forward-declared, not a stub.
- `🧬️schema/📸️snapshot/🦀️component.rs`: `GisMapSnapshot` gained `drawing: GisMapDrawingChild` (always
  present), `image: Option<GisMapImageChild>`, `value: GisMapValueChild` (always present). Dropped
  `#[derive(dsl::DslRecord)]`; hand-rolled `ArtifactDsl`/`ArtifactPack` — `positions`/`routes`/`regions`
  round-trip via JSON-then-hex (`enc_json`/`dec_json`, cad's convention for its own structured fields),
  the three children via the hex/bracket child-handle codec, in both text and binary.
- `🧬️schema/🦀️component.rs` (`GisMapArtifact`): gained `image` (carried verbatim — real, not derivable
  from anything this plugin owns, so dropping it silently would be a genuine, undocumented loss the
  moment a future basemap path populates it); `to_snapshot` re-derives `drawing`/`value` via
  `gis_map_snapshot_with_derived_children`; `empty_gis_map_snapshot`/`gis_map_document_from_descriptor_json`/
  `gis2d_document_json_from_dwg` all route their constructed snapshots through the same helper.
- `🧬️mutations/🦀️component.rs`'s `apply_gis_map_mutation` and `🔺️diff/📝️text/🦀️component.rs`'s
  `GisMapDiff::apply` re-derive `drawing`/`value` after every mutation/diff-apply, same pattern as terrain.

### Fixture regeneration (both plugins)

Both `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` files were written by the OLD
`dsl::DslRecord`-derived grammar and could not parse under the new hand-rolled codec. Regenerated for
real via the temporary `#[cfg(test)] mod debug_fixture_regen` technique (constructed a representative
snapshot from the OLD fixture's own data, dumped real `print_dsl()` output via
`cargo test … dump_fixture_dsl -- --nocapture`, captured it, wrote it as the new fixture, removed the
temp module — verified `grep -rn debug_fixture_regen` returns nothing). Terrain's fixture keeps its
`origin`/`position` sidecar lines verbatim (read separately by `terrain_fixture_text::parse_descriptor`,
never part of `GisTerrainSnapshot`'s own codec) — the new parser skips unrecognized lines instead of
erroring, matching what the old derive-generated grammar already did implicitly.

## Struct-literal fallout (per `📌️important.md` §4)

Grep-swept `GisTerrainSnapshot {`/`GisMapSnapshot {` across the whole plugin (not just the touched
files) after each field addition; fixed every non-trivial literal (`..Default::default()` where the
composed-child value doesn't matter to the test's own assertion; explicit
`gis_terrain_snapshot_with_derived_mesh(...)`/`gis_map_snapshot_with_derived_children(...)` wraps where
a test's `assert_eq!` on the WHOLE snapshot — inverse-law/diff-absorb-law tests — needed the composed
child to actually match the literal's own content, not `Default`'s empty one). This was the actual root
cause of 8 of the 8 failures hit on the first full test pass (see below) — not a hypothetical, a real bug
caught and fixed.

## Verification

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-gis --all-targets   → 0 errors (baseline had 3, all fixed)
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-gis --no-fail-fast
→ run 1: 171 tests run, 163 passed, 8 failed
→ (fixed: 5 gismap inverse/absorb-law literals, 2 gisterrain inverse/absorb-law literals, 1 manifest assertion)
→ run 2 (after fix): 171 tests run, 171 passed, 0 failed
→ run 3 (reproduction, no changes): 171 tests run, 171 passed, 0 failed
```

Every failure was independently traced to its exact cause (not assumed pre-existing, not deferred) —
all 8 were introduced by this migration's own composed-child re-derivation and fixed in the same pass,
per §6.5 of the migration recipe ("if a failure IS something your migration introduced, fix it, don't
defer it"). Zero failures were classified as pre-existing.

`grep -rn '"3d.mesh"'` across the plugin: only media-port `kind_id`/`Media.payload.schema` string tags
remain (gis3d's `scene:out` port + its tests) — these REFERENCE the canonical stdio-owned interchange
kind by id, they don't re-declare it. `grep -rn "mesh_artifact_kind\|ArtifactKindSpec {"`: zero hits for
the duplicate declaration; the only `ArtifactKindSpec {` construction left in the plugin is gismap's own
unrelated `2d.map` kind.

## sharedFileRequests

None. Every change is contained inside `✏️s/🔌️plugins/🌍️gis/**`; no `📦️glue.rs`/`📦️index.ts` edits were
needed (no new mounted files, no new taxonomy nodes — every new function/type lives in an already-mounted
`component.rs`).

## Concurrent-churn observations

The one pre-existing uncommitted 2-line change noted in Pre-flight lived inside
`🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, a file this migration rewrites
substantially (added the `image` field, `to_snapshot`/`from_snapshot`/`set_snapshot` rewiring, etc.).
`git log -3 -- <file>` showed nothing landed on it during this session's work; no live session identified
itself as owning it. Final compile/test state is clean, so whatever that 2-line change was, it composed
without conflict. No other concurrent churn observed — every `cargo check`/`nextest` run during this pass
was already clean on the first attempt with no stdio/framework errors surfacing.

## Honest gaps (documented in-code, not hidden)

- `gis_terrain_mesh_from_snapshot` produces a flat placeholder quad (z=0 always) — gis3d has no DEM
  heightfield tessellator, a pre-existing gap this migration didn't need to and didn't attempt to close
  (matches `gis3d_scene_media`'s own long-standing doc comment). `exaggeration` still round-trips as real
  document state and still keys the mesh's content-addressed handle.
- gismap's `image` composed child is always `None` — gis has no raster-basemap capture path today. Real,
  typed slot, not a stub; documented as forward-declared.
- The pre-existing terrain/mesh `las`/`ply`/`obj`/`stl`/`gltf`/`dwg` import deserializers under
  `🚪️io/📥️import/🧩️deserializers/**` were already honest stubs (`Ok(GisTerrainSnapshot::default())`)
  before this migration and remain so — populating them with real point-cloud/mesh parsing is a
  separate, much larger scope this ticket did not ask for.

ucas-status: complete
