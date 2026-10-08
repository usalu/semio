# r4 execution report: i-walls (Wave I, `🧱️wall-layout` and `🪞️curtain-layout`)

`T` = ticket folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`. Value type and conventions for consumers: `T/r4-api-wall-layout.md`.

## 1. Result

`🧱️wall-layout` is complete: per wall the resolved base/top/height (f1 semantics and datum unchanged), thickness (sum of the layers), location-line face offsets, per-layer interface offsets, left/right face curves (parallel lines, concentric arcs via the bulge segment kernel), joins on one storey (miter for nodes of 2, 3, 4 and more wall ends, butt for T ends against the near face, cross for X), the join-trimmed footprint loop (bulged, exact area), face lengths, centreline length, gross left/right side areas, volume and the join graph (`joins`, symmetric on both walls). Curtain walls got a sibling field `🪞️curtain-layout` (base/top/height, axis length, area, panel grid). Everything is wired into `ModelInference` (`wall_layout`, `curtain_layout`), all four facets, a language-agnostic fixture set, a shapely/IfcOpenShell oracle, and unit tests.

Definitions (also in the API doc): layers run from the left (interior) face to the right (exterior) face along the axis; `Interior` puts the axis on the left face, `Exterior` on the right face, `Center` on the mid plane, `CoreCenter` on the middle of the `Core` layers (else `Structure`, else mid plane). Joins: ends within 1 micrometre form a node mitered pairwise around it (neighbours ordered by outward angle, miter point further than 4 thicknesses falls back to a square end), an end on another axis butts against the near face (the end edge of the footprint follows that face when it is an arc), axes crossing in both interiors form a cross that trims nothing and reports the overlap area. Only walls of the same storey join.

DAG and honesty: wall nodes keep only storeys as parents (own storey, then the `TopConstraint::Storey` target); joined neighbours are inputs, never parents (joins are symmetric, parents would cycle). `dep_input` = `{own: {wall, layers}, neighbours: {id: {wall, layers}}}` with the neighbour set computed by the same function `compute` uses, so a wall that starts or stops joining, or a neighbour edit, changes the hash and an unrelated wall does not (tested against the cache and the pure recompute). `SCHEMA_VERSION` of the field is now 2.

## 2. Files

Created:
- `I/🧱️wall-layout/🔗️joins/🦀️.rs` (pure band/join/trim/footprint geometry over the framework `BulgeSeg`, `triangulate`; no snapshot access).
- `I/🪞️curtain-layout/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs}`.
- Fixtures `S/🧫️fixtures/💡️inferences/🧱️wall-layout/{l-miter-equal,t-butt,x-cross,location-lines,arc-tangent}/🔣️.json` (`before` snapshot plus hand-derived expected layout fields, subset-compared) and the oracle case data `S/🧫️fixtures/💡️inferences/🧱️wall-joins/{📸️snapshot,💡️inference/🧱️wall-layout,💡️inference/🪜️storey-levels}/🔣️.json` (30 walls on 2 storeys: L, unequal L, 60 degree L, T from both sides, X, nodes of 3 and 4, tangent arc to line, right-angle arc to line, line stem onto an arc, four location lines, a wall on another storey).
- Oracle case `S/🧪️tests/🧱️infer-bim-1-wall-joins/{🥒️.feature,🐍️.py,🦀️.rs}` (`@oracle-bim-1-shapely-geometry`, scenario `wall-joins`).
- `T/r4-api-wall-layout.md`, this report.

Updated (surgical): `I/🧱️wall-layout/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs}` (rewritten values, geometry, field, tests); aggregate `I/{🦀️.rs,🟦️.ts,🔣️.json,🛰️.proto,🔗️.graphql}` (field `curtain_layout`, `WallLayout` and its types in every facet); aggregate unit tests `I/🧪️tests/🔬️unit/🦀️.rs` (recursive oracle comparison, house plus joins tables); crate root mount `🏢️model/🦀️.rs` (`curtain_layout` module); levels oracle `S/🧪️tests/🪜️infer-bim-1-levels-and-wall-heights/{🐍️.py,🥒️.feature}` (now imports the plan geometry of the joins oracle by path, recursive compare); registration rationale of `bim-1-shapely-geometry` in `S/🔮️oracles/🔣️.json`; regenerated tables `S/🧫️fixtures/💡️inferences/🏠️house/💡️inference/🧱️wall-layout/🔣️.json`. The IfcOpenShell oracle `🧊️infer-bim-1-wall-solids/🐍️.py` was already moved by a peer to extrude the committed join-trimmed footprint; it agrees with my tables (see 3).

Before to after of `WallLayout`: `{base_z, top_z, height, thickness, length, side_area, footprint_area (length x thickness), volume}` to the field set of the API doc (`footprint_area`/`volume` are now join-trimmed; `side_area` kept as centreline length x height; `Copy` dropped). `layout_of` gained the wall id.

## 3. Commands and results

| Command | Result |
|---|---|
| `bimmini` scratch crate (`scratchpad/bimmini`: the real snapshot, diff and my three inference leaves mounted by `#[path]`, same dependencies) `cargo test --lib` through the gate | 39 passed, 0 failed (13 wall-layout tests, 5 curtain-layout tests, storey-levels, diff, and the oracle-table test below) |
| same crate, test `oracle_tables::the_subject_reproduces_the_third_party_oracle_tables` | subject equals the committed shapely tables for house (7 walls, incl. arc joins) and wall-joins (30 walls) at 1e-9, whole structure (loops, face curves, joins, layer offsets) |
| same crate `cargo check --target wasm32-wasip2` | exit 0 |
| real crate `cargo check -p semio-s-artifact-bim-model` (native, and `--target wasm32-wasip2`) | exit 0 both (after the stdio / editor peers settled; the earlier runs failed in peers' in-flight code) |
| real crate `cargo test --lib` | NOT RUN to completion: the lib-test target fails to compile in peers' in-progress code (`stair-runs`, `spaces`, `quantities` tests: `super::storey_levels` unresolved; `viewer` tests; `io` tests; `render/window-config`), none of it in my files. My leaves' tests were run in the scratch crate above (identical source files) |
| python `🧱️infer-bim-1-wall-joins/🐍️.py check` / levels oracle `write`, then `check`, shapely 2.1.2 | `oracle agrees`; the audit measured validity, areas, face lengths, non-overlap of joined footprints, cross overlap, `buffer(mitre)` identity of 2-wall nodes, `difference` identity of butts, GEOS `offset_curve` corners of every straight join |
| IfcOpenShell `🧊️infer-bim-1-wall-solids/🐍️.py check` | `oracle agrees` (6 of 7 house walls, 26 of 30 joins walls: kernel volume of the extruded join-trimmed footprint equals `footprint_area * height`) |
| harness `oracle quick --case` for `🧱️infer-bim-1-wall-joins`, `🪜️infer-bim-1-levels-and-wall-heights`, `🧊️infer-bim-1-wall-solids` | 1/1, 2/2, 2/2 passed |

Findings while testing (all fixed): the end edge of a butt against a curved face was a chord (5.8e-5 m2 off the shapely `difference` identity), now an arc; `curtain-layout` needed `super::super::wall_layout` (caught by the scratch build).

## 4. Open issues

1. `cargo test --lib` of the real crate could not be completed (peers' test code, see above); re-run `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib` (the aggregate test `the_subject_reproduces_the_third_party_oracle_tables`, 13 wall-layout and 5 curtain-layout tests).
2. The `subject` and `parity` platform roles for `🧱️infer-bim-1-wall-joins` (Rust adapter `🦀️.rs`, `sut`-gated) were not run; the oracle role is green.
3. Cross overlap uses triangulation plus convex clipping of the untrimmed bodies (exact for lines, 1e-6 flattening for arcs); a cross trims nothing by design. Several through walls at one T end: the lowest id wins. Walls of different phases or without vertical overlap still join if on one storey. The miter limit (4 thicknesses) and the tolerance (1 micrometre) are constants in `🔗️joins`.
4. `wall-solids` projection in the IO text (f1) still keys on `Axis::Line`; walls whose cut edge is curved (a line stem butting an arc) are then projected although their loop has a bulge. House is unaffected.
5. Scratch: `T/🗑️generated/i-walls` was deleted; the scratch crates live in the session scratchpad only.
