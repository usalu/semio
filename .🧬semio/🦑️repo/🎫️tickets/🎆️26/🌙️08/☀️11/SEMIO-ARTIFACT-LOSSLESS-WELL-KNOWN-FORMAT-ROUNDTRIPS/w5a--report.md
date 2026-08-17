# W5a Report — 📐️cad plugin ad-hoc codec extraction

Agent: W5a, plugin `📐️cad` (crate `semio-s-plugin-cad`). Write scope: `✏️s/🔌️plugins/📐️cad/**` only.

## Baseline blocker (fixed, in-scope)

`cargo check -p semio-s-plugin-cad` did not even compile at the start of this session: the plugin's
own `📦️glue.rs` mounted `🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs`, a directory that no
longer exists (renamed to `📄️artifact` by an earlier, unrelated ticket — confirmed via
`git log`, the rename predates this session and is fully committed). Fixed with a 1-line `#[path=...]`
correction in `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs`. Without this the crate could not be
verified at all, so it was treated as in-scope (my own plugin's glue.rs, not a "hot" cross-plugin file).

## What was built

### 1. Real `semio/mesh` + `semio/brep` bridging in the engine (the ticket's core ask)

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, `export_solids_as`
(the function backing the play app's save/download STEP/OBJ/STL export, `brep:out` workflow port
media export, and `importCadFile`'s reverse path):

- **Obj/Stl**: now tessellate the live kernel solids into a real `SemioMeshSnapshot`
  (`semio_mesh_snapshot_from_solids`, new), then call stdio's own `SemioMeshToObj`/`SemioMeshToStl`
  serializers + `obj::engine::encode_obj`/`stl::engine::encode_stl_binary` — zero hand-rolled byte
  encoding, matching the plan's mandate exactly. Previously called the framework brep kernel's own
  `export_obj`/`export_stl` directly.
- **Step**: still SOURCES geometry from the framework kernel's native `export_step` (a real,
  working, geometry-exact AP214 writer one layer below this plugin — not ad-hoc plugin-level
  duplication, and out of this plugin's write scope to touch), but the BYTES actually returned now
  come from re-encoding that text through a real `semio/brep` round trip: kernel STEP text →
  `parse_part21` → `StepSnapshot` → `SemioBrepFromStep::deserialize` → `SemioBrepSnapshot` →
  `SemioBrepToStep::serialize` → `StepSnapshot` → `write_part21` → final text. This both validates
  the kernel's output against stdio's real AP214 entity-graph walk and produces the export from the
  same codec stdio/semio uses everywhere else (`semio_brep_snapshot_from_step_text`/
  `step_text_from_semio_brep_snapshot`, new).

**Real bug found and worked around** (framework code, out of write scope, reported not patched):
`🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs::write_step` builds its
`ADVANCED_BREP_SHAPE_REPRESENTATION` item list via `format!("({},)", items.join(", "))` —
unconditionally appending a trailing comma before the list's closing `)`, for 0, 1, or N items. That
is not valid ISO 10303-21 (a Part-21 list never permits a trailing comma before its close), and
stdio's own Part-21 tokenizer correctly rejects it (`UnexpectedChar { found: ')', expected: "value"
}`) rather than guessing — confirmed by direct `eprintln!` debug capture of the exact malformed byte
range (`(#162,)`) and independently re-verified with a standalone Python re-implementation of the
fix against the real captured text. Added `repair_step_trailing_comma_before_close_paren` (quote-aware:
a `,)` inside a real STEP string literal is left untouched) in the cad engine to repair this before
handing kernel-produced text to `parse_part21`, with its own dedicated unit test. See `foreign_breakage`
below — this is a framework bug, not a cad-plugin or stdio issue.

**Second real gap found** (also framework, also reported not patched): the framework kernel's own
`import_step` reader (`build_axis2_placement`) hard-requires all 3 `AXIS2_PLACEMENT_3D` references
to be non-null (`refs.len() < 3` → hard error), rejecting `SemioBrepToStep`'s own spec-valid `$`
(null) `ref_direction` output — ISO 10303-21 permits `ref_direction` to be derived/omitted.
`export_solids_as_step_round_trips_through_real_semio_brep_bridge`'s round-trip therefore reimports
through the SAME `semio/brep` bridge (not the framework kernel) to prove geometry-equivalence — see
that test's inline comment.

### 2. `mesh_to_obj_text` (the ~line-330 target in `⚙️engine/📥️geometry-import/🦀️component.rs`)

The hand-rolled `v`/`f`-only OBJ writer is deleted; `cad_object_from_mesh`'s only caller now goes
through the identical real stdio path (`SemioMeshSnapshot` → `SemioMeshToObj` → `obj::engine::encode_obj`),
sharing the exact same real encoder `export_solids_as` uses (not a second reimplementation). Kept the
function name/call site (the only consumer, `cad_object_from_mesh`, needed a text producer for the
kernel's `import_obj` reader — `BrepKernel` has no non-text mesh-import primitive) but its body is
now 100% stdio-backed.

### 3. Byte-reinterpret placeholder exporters/deserializers — cad's own `✳️any/🚪️io` tree

Explicitly named in the brief: **STL, IFC** (both directions, 4 files) — deleted the dead binary
`serialize`/`deserialize` functions outright. Verified dead first (zero callers anywhere outside
their own definition — `CadComposer::compose()` and `compose_export_*` in
`🏅️standards/🔖️1/🎹️composer/🦀️component.rs` only ever call the sibling `serialize_text`/
`deserialize_text`, which are `<CadSnapshot as ArtifactDsl>::print_dsl`/`parse_dsl` — an honest,
if non-functional-for-real-foreign-files, passthrough that is NOT the fabrication bug and was left
untouched).

**Found the identical bug (byte-reinterpret via `cad_to_wire`/`cad_from_wire`, i.e. reinterpreting
the CAD document's own opaque whole-document `ArtifactPack` bytes as fake `f32` vertex triples) in
OBJ, PNG, JSON, GLTF, STEP too** — all equally dead code, several already failing to compile outright
against stdio's evolved snapshot schemas (`ObjSnapshot` missing 5+ fields, `StlSnapshot` had no
`vertices` field at all, `PngSnapshot` had no `image` field, `JsonSnapshot.value` type mismatch,
`GltfSnapshot.document` type mismatch — all pre-existing breakage, confirmed via `git status`/
inspection, not introduced this session). Deleted the same dead pattern from all of them for
consistency (10 more files) — this doubles as fixing 19 pre-existing compile errors the crate had at
baseline (`cargo check` failed before I touched these files; see raw baseline evidence below).
`dwg`'s `ac1018` (explicitly frozen, not touched behaviorally) needed only the minimal 2-field
addition (`codepage`/`maintenance_version`, both `0` matching their own `#[serde(default)]`) to
satisfy `DwgSnapshot`'s grown shape — its dead `serialize()` body and `deserialize()` (which still
compiled) were left otherwise untouched, out of respect for "frozen legacy shim."

### 4. 11 orphaned JSON Schema files

`✏️s/🔌️plugins/📐️cad/🧬️schema/🔣️json/*.json` (action/attribute/display/expression/extension/
interaction/model/stat/transformation/typology/view.json, 1439 lines total) — deleted via `git rm`
after re-verifying zero real inbound references repo-wide (only 2 informal doc-comment mentions in
`🔨️modules/📐️geometry/🟦️component.ts`, updated to drop the dangling paths).

### 5. TS STEP writer/reader cluster — `🔨️modules/📐️geometry/🟦️component.ts`

Deleted the confirmed 1418–1545 range (`StepEntityWriter`, `parseStepEntityMap`, `stepEscape`/
`stepNumber`, `stepParseFirstString`/`stepParseDescriptivePayload`, `parseSpatialUdaPayloads`,
`mergeStepDataChunk`, `stepSpatialFileHeader`, `assembleStepFile`, `emitSpatialUdaProperty`) — but
widened the deletion to the FULL enclosing `// #region 🪜️StepRoundtrip` (1399–1571, 173 lines), since
`applySpatialAttributesFromUda`/`modelSpaceFromSpatialUda` (just past the recon's cited end line,
described there as "correctly out of scope") turned out to have their only real caller inside
brepjs's `importStepToModelSpace` — deleted in step 6 below — so they'd have become dead code too.
3389 → 3216 lines.

### 6. brepjs's STEP import/export entry points — `🔨️modules/📐️brepjs/🟦️component.ts`

Deleted the 5 entry-point methods on `BrepjsWasmEngine` (`exportModelSpaceToStep`, `exportModelToStep`,
`importStepBrepToModelSpace`, `importStepBimToModelSpace`, `importStepToModelSpace`) and the ~475-line
STEP-presentation-layer-import helper cluster they alone used (`validSolidsFromImportedShape` through
`mergeImportedBrepPart`, lines 1935–2409 pre-edit) — real brepjs `exportSTEP`/`importSTEP` (OpenCascade
kernel ops) and every boolean/fillet/sweep/etc geometry operation are untouched. Pruned now-unused
imports (`assembleStepFile`, `emitSpatialUdaProperty`, `hashSolidRecord`, `mergeStepDataChunk`,
`modelSpaceFromSpatialUda`, `parseSpatialUdaPayloads`, `parseStepEntityMap`, `stepParseFirstString`,
`stepEscape`, `stepSpatialFileHeader`, `StepEntityWriter`, `derivePropertyValue`, `importProfileFor`,
`kernelTypologyIds`, `typologyFromStepLayer`, `exportSTEP`/`importSTEP`, `cast`/`getBounds`/
`getVertices`/`iterTopo`/`outerWire`/`faceCenter`/`faceGeomType`/`vertexPosition`/`Solid`,
`ObjectRef`/`TypologyRef`). 4231 → 3320 lines.

**IMPORTANT — real feature/test-coverage loss, flagged prominently for the orchestrator**: this
cluster was NOT dead code from the TS side — it had extensive fixture-backed test coverage (real
`.stp` files under `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau`, asserting exact face/solid counts and
BIM-layer-to-typology assignments for the "concrete forest left" example) exercising a genuinely
working BIM/STEP presentation-layer import feature — including the generator script
(`CAD_GENERATE_STEP_FIXTURES=1`) that produces the `.model.json` play fixtures those STEP files back.
Deleted per the master plan's explicit, specific instruction ("brepjs's STEP import/export ENTRY
POINTS... only strip its STEP read/write surface", with the exact file named) and the ticket's
"delete outright, no legacy support" mandate, but this is real capability loss with **no replacement
built in this wave** — cad's own Rust-side STEP import (`import_step_object`) has no BIM
presentation-layer/typology-assignment equivalent. Also removed the corresponding tests in this
file's own embedded `describe` block (9 `it()` blocks/one fixture-loop, ~250 lines) AND in
`🔨️modules/🧪️tests/🟦️component.ts` (one `describe("...step roundtrip helpers")` block testing the
deleted pure functions directly) — kept every other test, including
`"concrete forest left play fixture roundtrips..."` (static-JSON-fixture based, untouched by any of
this). 3050 → 3025 lines (net; also edited the shared import list).

## Real tests added (`⚙️engine/🦀️component.rs`, new `#[cfg(test)] mod tests`)

- `export_solids_as_obj_uses_real_stdio_mesh_codec_not_hand_rolled_bytes` — real box tessellates to
  ≥8 vertices/≥12 faces via the new pipeline.
- `export_solids_as_stl_uses_real_stdio_mesh_codec` — decodes the base64 body, verifies a real binary
  STL header + triangle-count-consistent byte length.
- `export_solids_as_obj_none_for_a_solid_that_fails_to_tessellate` — disposed handle → `None`, never
  a fabricated triangle.
- `export_solids_as_step_round_trips_through_real_semio_brep_bridge` — **scenario (a)**, cad's own
  half: real box → kernel STEP → `semio/brep` (baseline topology) → `export_solids_as` (repaired +
  round-tripped STEP) → re-parsed via the SAME `semio/brep` bridge → solid/face/vertex counts match.
- `semio_brep_snapshot_from_step_text_carries_real_topology` — direct proof `SemioBrepSnapshot` has
  real non-empty `solids`/`faces`/`vertices`, and the reverse `SemioBrepToStep` round-trips to valid
  Part-21 text.
- `repair_step_trailing_comma_before_close_paren_is_quote_aware` — unit test for the framework-bug
  workaround, including the quote-safety case.

## stdio_gaps

- **cad→ifc**: no real bridge exists or was built (mesh/brep are the only semio subsets cad's
  geometry maps to; IFC belongs to the `model` subset, a spatial tree cad has no equivalent of). The
  fabricated placeholder was deleted, not replaced — reported as an absence, not silently patched.
- **cad→png**: same — no renderer (camera projection/rasterization) exists anywhere in this repo to
  produce real pixels from 3D CAD geometry. Fabricated placeholder deleted, not replaced.
- **`CadComposer::compose()`** (`🏅️standards/🔖️1/🎹️composer/🦀️component.rs`, ALL 8 registered
  dialects: dwg/gltf/ifc/json/obj/png/step/stl) only ever tries `deserialize_text` =
  `<CadSnapshot as ArtifactDsl>::parse_dsl(text)` — i.e. attempts to parse the foreign file's raw
  bytes AS IF they were cad's own internal DSL text. This will simply fail to parse for any real
  foreign-format file and fall through to the next dialect. This means the `s.cad`-artifact-level
  compose path (distinct from the real, working, kernel-based `export_solids_as`/`import_step_object`
  I rewired) has **no real per-format codec for any of its 8 dialects** today. Not fixed here (a much
  larger rebuild, not named in this wave's brief); flagged since it may be a blind spot for whoever
  consumes `CadComposer` directly.

## foreign_breakage (confirmed via `git status`, not touched)

1. **`🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs::write_step`** — the trailing-comma
   Part-21 list bug described above. Framework file, out of write scope; worked around defensively
   inside the cad engine (see §1), not patched at the source.
2. **`🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs::build_axis2_placement`** — rejects
   spec-valid null `ref_direction`. Same file, same reasoning.
3. **`✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs`** stale panel mount — fixed (§ Baseline
   blocker), in my own write scope.
4. **Live concurrent session, `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/**`,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/**`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**`,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/**` and
   `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/**`** — confirmed via repeated `git status`
   polling across this entire session (unstaged modifications growing from ~6 to 19 framework files
   over roughly an hour); caused transient `semio-framework-os-kernel`/`semio-framework-os-kernel-dsl-derive`
   compile failures (`E0308`/`E0277`/proc-macro-derive-root errors, three distinct error signatures
   across separate polls — a real in-progress refactor, not a stale-cache artifact) that repeatedly
   blocked/queued my own `cargo check`/`cargo test` runs, unrelated to any cad-plugin code. Never
   touched. Polled ~10 times over roughly an hour (Monitor-based waits, not sleep loops) rather than
   chasing; final clean `cargo check` (EXIT=0) and `cargo test --lib` (130 passed, 0 failed, EXIT=0)
   runs both landed once that session's edits stabilized. Full timeline in
   `w5a--foreign-breakage-timeline.txt`.

## Exit checklist

`cargo check -p semio-s-plugin-cad 2>&1 | tail -40` — see `w5a--cargo-check.txt` (raw, real, EXIT=0,
10 pre-existing warnings unrelated to this session's edits, zero errors).

`cargo test -p semio-s-plugin-cad --lib 2>&1 | tail -30` — see `w5a--cargo-test.txt`. **130 passed;
0 failed; 0 ignored — EXIT=0**, real final run (fully green, including all 5 new tests: mesh/obj,
mesh/stl, obj-tessellation-absence, the scenario-(a) semio/brep STEP round trip, and the direct
brep-snapshot topology test).

LOC deleted (grep/`wc -l` before→after, real):
- `🔨️modules/📐️brepjs/🟦️component.ts`: 4231 → 3320 (-911)
- `🔨️modules/📐️geometry/🟦️component.ts`: 3389 → 3216 (-173, +2 comment edits)
- `🔨️modules/🧪️tests/🟦️component.ts`: 3050 → 3025 (-25, +1 import-list edit)
- 11× `🧬️schema/🔣️json/*.json`: -1439 (full delete)
- 14× cad io leaf files (stl/ifc/obj/png/json/gltf × import+export): dead binary fn removed from each
- `git diff HEAD --numstat` over the whole plugin: **389 insertions(+), 2796 deletions(-)** across 32
  files (net **-2407 lines**).

Files touched (created none; deleted 11; modified the rest): see the numstat list in
`w5a--cargo-check.txt`'s header or `git status --porcelain -- "✏️s/🔌️plugins/📐️cad"`.
