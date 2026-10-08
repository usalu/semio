# r4 execution report: x-examples (Wave X, BIM model examples)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `A` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`.

## 1. Result

Two handcrafted buildings (`house`, `office`) are committed as snapshot JSON plus DSL text, registered in the editor's `examples()` and mounted in the artifact root, together with a shared check kit and 20 example tests (demo included). Every committed value is authored (no elevations, heights, areas or other derived value). Both snapshots validate against the snapshot JSON Schema with `jsonschema` (0 errors), decode strictly, round-trip through text and pack, are closed under references, infer with valid opening frames, compliant stairs, closed rooms and no diagnostics at all (no error, no warning, no clash), and are rebuilt from an empty snapshot by replaying their records through the central mutation applier. No create kind is missing; the Wave M leaves are all in.

## 2. The buildings

| | House (`🏡️house`) | Office (`🏢️office`) |
|---|---|---|
| Site | Bern 46.948 N / 7.4474 E, 542 m, 22.5 x 30 m plot, building origin (6, 9), plinth 0.30 m | Zurich 47.391 N / 8.488 E, 408 m, 56 x 48 m plot, origin (12, 14) |
| Storeys | basement -1 (2.6 m), ground 0 (2.8), upper 1 (2.7), attic 2 (2.4) | ground 0 (4.0), 1 to 3 (3.8 each), roof level 4 (1.2, parapet and flat roof) |
| Walls | 30: 6 exterior walls per floor with one arc bay wall (bulge 0.5, 3.48 m long) on basement, ground and upper; spine, one continuous cross wall (X join with the spine), partitions with T and L joins; 4 attic knee walls (unconnected 0.9 m) | 60: 24 infill panels between the columns (top 0.86 m below the storey top, under the edge beam), 32 core walls (two stair cores per storey), 4 roof parapets |
| Openings | 35: 25 windows (5 types, size overrides), 9 doors (5 types), 1 cased void | 41: 24 ribbon windows, 17 doors (entrance in the curtain wall, 16 core doors) |
| Horizontal | 4 slabs (foundation, floor with cellar stair hole, floor with U-stair hole, timber attic floor); gable roof (35 degrees, 0.5 m overhang) and flat bay roof | 4 slabs (ground, 3 floors with 2 stair shafts each), flat roof |
| Vertical circulation | U-stair ground to upper (15 risers), straight cellar stair (13 risers), 2 railings | 6 straight stairs (two cores x storeys 0 to 2, 19 or 20 risers) |
| Frame | steel IPE 240 beam + CHS column under the bearing kitchen wall | 6 x 4 grid, 96 columns (500 and 400 mm), 152 beams (primary and secondary) |
| Façade | masonry cavity walls | 8 curtain walls (south, north, per storey) |
| Types | 10 materials, 6 wall, 3 slab, 2 roof, 1 column, 1 beam, 5 window, 5 door types with real layer stacks (listed interior to exterior, exterior on the right of the CCW axes) | 8 materials, 3 wall, 2 slab, 1 roof, 2 column, 2 beam, 1 window, 2 door types |
| Data | 12 spaces (Bounded seeds), 6 grid lines, 6 property sets, 6 classifications (Uniclass 2015 EF_25_10, DIN 276) | 28 spaces (22 Explicit outlines, 6 Bounded core seeds), 10 grid lines, 4 property sets, 5 classifications (DIN 276) |

Conventions used (checked against the inference docs): opening `offset` is the centre along the host; a window's real sill is type sill plus the opening's sill (kept 0); stair `start` is the middle of the foot edge, `UTurn` folds to the left of the travel direction; layers run interior to exterior and the exterior is on the right of a CCW perimeter; slab and floor stack hang down from the storey elevation, a beam's top is the storey top plus the signed `top_offset` (the solids, bodies and IFC code all add it), so the beams carry the negative slab thickness above them (-0.30 house, -0.36 office) and the column under the house beam stops at -0.54.

## 3. Files

Created (kept inputs in `T`): `r4-x-examples-gen.ts` (writes both snapshot JSONs, verifies references, opening margins and overlaps, space numbers and levels before writing), `r4-x-examples-gen-rust.ts` (writes the Rust and TS sources below and mounts them in the root, idempotent), `r4-x-examples-mirror.ts` (verification harness, see section 5), `r4-x-examples-plan-check.py` (shapely plan sanity oracle).

Created under `S`: `🖼️assets/🏡️house/{📸️snapshot.json,🗣️.dsl.semio}`, `🖼️assets/🏢️office/{…}` (the DSL texts are blessed from the JSON with `BIM_BLESS=1 cargo test bless_the_`), `📚️examples/🏡️house/{🦀️.rs,🟦️.ts,🧪️tests/🧩️example/🦀️.rs}`, `📚️examples/🏢️office/{…}`, `📚️examples/🧰️checks/🦀️.rs` (shared kit: `Asset`, `dangling`, `infer`, `replay`, `assert_replay`).

Updated (surgical): `A/🦀️.rs` (examples mount block: `checks`, `house`, `office`), `S/✏️editor/🦀️.rs` (`examples()` returns demo, house, office; the editor agent rewrote the file once and the edit had to be re-applied), `S/📚️examples/🎬️demo/🦀️.rs` (`ASSET_DIR`, `SNAPSHOT_JSON`) and the demo tests (a `shared` module running the same checks on the demo).

## 4. Tests (all in `semio-s-artifact-bim-model --lib`, `examples::`)

Per building: documented counts (storeys, levels, walls per storey, arcs, window/door/void counts, slabs, holes, roofs, stairs, spaces, types, properties); text is the codec fixed point of the JSON and equals it; pack round trip; JSON Schema validation by `jsonschema` 4.26 through the repository `.venv` (skipped with a message when no `.venv` exists); referential integrity and unique levels and space numbers (`dangling`); inference: storey elevations and absolute elevations, wall heights per storey, wall thickness and length (arc length 3.477356 m from the bulge formula), stair riser counts, valid opening frames, compliant stairs, closed rooms, no diagnostic error; the parametric law (raise one storey height and every storey and wall above it moves while the storey below stays); quantities and rooms against hand calculations (slab gross, net and volume including hole and circular segment, wall side and opening area, column and beam volumes, kind counts, WC and stair-core room areas); mutation replay (house: 157 mutations, office: 448, demo: 11) equal to the committed snapshot.

Third-party validation: `jsonschema` (schema), `shapely` through `r4-x-examples-plan-check.py` (0 wall overlaps above 0.06 m2, 0 column-wall clashes, every hole inside its slab and clear of its storey walls, for both buildings).

## 5. Commands and results

| Command | Result |
|---|---|
| `bun r4-x-examples-gen.ts` | both snapshots written, generator verification clean |
| `.venv/Scripts/python.exe -I r3-o-oracles-validate-snapshot.py` on both snapshots | `errors: 0` twice |
| `.venv/Scripts/python.exe -I r4-x-examples-plan-check.py <snapshot>` | `issues 0` twice |
| `cargo test --manifest-path <mirror>/Cargo.toml -p semio-s-artifact-bim-model --lib examples::` (gate slot) | 20 passed, 0 failed (house 7, office 7, demo 6) |
| `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib` (the gate, last run) | 3123 passed, 8 failed: 3 `editor::bim` and 5 `viewer::bim` tests of the UI agents ("registered fixture did not reach its exact terminal-empty witness", a window isolation law); all 20 `examples::` tests pass |

Why a mirror: for most of the session the shared crate did not compile its test target because of peers' in-flight work (stdio crates not yet migrated to `ApplyCapability`, a stale generated icon enum, missing `mod tests;` files in viewer and editor, a broken `use` block in `set-column` and `set-beam`). `r4-x-examples-mirror.ts` copies the artifact into `T/🗑️generated/x-examples/mirror`, strips every `cfg(test)` module mount except the examples, rebases the mutation leaf descriptor owners (the derive checks them against the path) and builds a one-member private cargo workspace over it; the examples compile and run unchanged. It is a verification harness only; the committed code is the real tree.

## 6. Open items

- The 8 failing gate tests belong to u-editor and u-viewer (UI fixtures); re-run the gate after they land.
- Peer fixes I had to make: `set-column` and `set-beam` `🦠️mutation/🦀️.rs` carried stray lines `Point2` and `TopConstraint` after the `use` block (written at 10:36 by an unknown script, the owner had already reported), which broke every build; I deleted exactly those lines.
- The disk filled up (0 bytes free, "No space left on device") during the session, probably through the cargo caches under `⚡️cache/cargo` (many stale `target-fleet-*` dirs); it recovered by itself. My private mirror workspace adds one more dependency build to whichever gate slot it runs in; delete those slot dirs when space matters.
- `✏️s/Cargo.lock` is sometimes unwritable (a process holds it mapped, os error 1224); the mirror uses its own copy.
- Findings for the peers: the IFC and solids code read `Beam.top_offset` as signed (top = storey top + offset) while the `create-beam` payload doc says "hangs below by its authored offset"; the doc should say signed. Two collinear walls that end on the axis of a through wall (a split cross wall) report `ClashWallWall` with the through wall; the examples use one continuous wall instead.
- Properties and classifications are replayed through `set-element-property` and `set-element-classification` now that they exist; nothing is seeded.
- Taxonomy: the new names `🏡️house`, `🏢️office` (assets and examples) and `🧰️checks` are not registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`members-of-assets` and the examples members); Wave R should run `verify taxonomy` and register them.
- Attic gable ends: the E and W knee walls stop at 0.9 m; the gable triangle above them is the roof solid's job (no roof-trimmed wall exists). The curtain walls sit 0.45 m outside the column grid, the slabs reach 0.3 m beyond it.
- The office has five storeys (four office floors plus the roof level 4) because the roof and the parapet need a storey of their own; the house attic plays the same role.
- No `🥒️.feature` scenario was added for the examples; the language-agnostic evidence is the JSON Schema validation, the JSON snapshots and the mutation replay. The existing `mutate-model-1-any` feature already round-trips the demo DSL.
- The classifications use DIN 276 codes and Uniclass 2015 `EF_25_10`; one classification per element is all the snapshot allows.

## 7. Scratch

Deleted: `T/🗑️generated/x-examples/` (logs, mirror, private workspace). Kept: the four `r4-x-examples-*` inputs and this report.
