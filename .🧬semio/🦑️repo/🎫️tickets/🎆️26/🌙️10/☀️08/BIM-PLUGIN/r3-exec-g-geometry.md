# 📐️ r3 Execution Report — `g-geometry` (Wave F)

Companion files: `r3-geometry-inventory.md` (what existed, gaps, decisions), `r3-geometry-api.md` (reference for the inference agents).
Input scripts kept: `r3-g-geometry-fixtures.py` (closed-form fixture generator), `r3-g-geometry-schemas.py` (fixture schemas).

## What was built (all additive; no existing public API changed)

`semio-framework-geometry` (`🧰️framework/🔨️modules/📐️geometry`), new domain folders mounted from `📦️packages/🦀️rust/🦀️.rs`:

| Folder | Content |
|---|---|
| `➗️vector` | f64 helpers, `Xyz`, `LENGTH_EPS`/`ANGLE_EPS` |
| `🌙️bulge` | `BulgeSeg` (centre/radius/sweep/length/point/tangent/closest/bounds/offset/split/retarget/flatten/area terms), `intersect` (line/arc/arc, bounded or infinite), `nearest_intersection`, `corner_join`, `band_loop` |
| `➰️loops` | bulged loops: exact area/centroid/perimeter/bounds/winding/containment, flatten, mitered `offset`, `self_intersections` |
| `🔺️triangulation` | hole bridging + ear clipping, crack-free (collinear vertices preserved) |
| `🧭️placement` | `Affine3`, `ZPlane` |
| `🕸️mesh` | f64 `TriMesh` (measures, weld, crease normals, f32 export) and builders `extrude`, `extrude_loops`, `prism_between`, `sweep_profile`, `extrude_between_faces` (walls with openings, slanted ends, curved walls) |
| `🔪️section` | `section_plane`, `section_z`, `chain` |

`semio-framework-2d` (`🧰️framework/🔨️modules/◻️2d/🧱️regions`): `Region`, `region_boolean`, `offset_regions` (miter/bevel/round, budget + cancel via `control`), `regions_from_path`.

Tests/fixtures: per-domain `🧫️fixtures/<emoji>/🔣️.json` (+ `🧬️schema/🔣️.json`), unit tests `🧪️tests/🔬️unit/🦀️.rs` in every domain, oracle suites in
`📐️geometry/🧪️tests/🏙️aec-oracles/{🦀️.rs,🟦️.ts,🐍️.py}` (+ `🧪️tests/🎚️config/🟦️.ts`), Cargo: dev-deps `serde_json`, `parry3d 0.17` (kurbo already present), second `[[test]] aec_geometry_oracles`.

## Before → after

Before: no bulge segments, no arc offsets/intersections beyond bounded line-circle, no bulged loops, no triangulation, no f64 mesh/solid
builders, no plane section on meshes, no polygon offset. After: all of the above, plus the wall-with-openings solid builder.

## Commands and exact results (repo root, root `Cargo.toml` workspace, via `🚦️gate.sh g-geometry`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-geometry` | lib **134 passed**, `aec_geometry_oracles` **9 passed** (kurbo + parry3d), `first_party_geometry` 5 passed (pre-existing), doc 0 |
| `cargo test -p semio-framework-2d --lib regions::` | **5 passed** |
| `cargo check -p semio-framework-geometry -p semio-framework-2d --target wasm32-wasip2` | OK, no warnings in the new files |
| `node_modules/.bin/vitest run --config 🧰️framework/🔨️modules/📐️geometry/🧪️tests/🎚️config/🟦️.ts` | **26 passed** (`three`: EllipseCurve/Path/ShapeUtils earcut/ExtrudeGeometry) |
| `.venv/Scripts/python.exe -m pytest -p no:cacheprovider 🧰️…/📐️geometry/🧪️tests/🏙️aec-oracles/🐍️.py` | **82 passed** (`shapely` 2.1.2: buffer joins, set ops, constrained Delaunay, polygon formulas; jsonschema validates every fixture against its schema) |

Oracle coverage: kurbo (arc length, bounds, nearest point, segment area, offset distance, intersections on-curve + line hits, loop area / perimeter / bounds / centroid via moments / winding on ~1000 probes), parry3d (volume, centroid, area, AABB of every solid incl. 60 random transformed slabs with holes; plane-cut length), three (arcs, loop area/perimeter, earcut area and triangle count, extrusion volume/area), shapely (offsets, region booleans/offsets, loop offsets, triangle counts, prism formulas, wall elevation formulas).

## Findings worth knowing

- Triangulating a developed curved face without Steiner points leaves long triangles that cut through the cylinder; `extrude_between_faces` therefore slices the coarse triangulation at `s` grid lines (linear size, 7.6k triangles at 1 mm tolerance for a 7.9 m curved wall).
- `parry3d` TriMesh section/split needs shared vertices: `TriMesh::welded()` exists for that (and for exporters).
- Winding of points exactly on a chord needs a consistent tie-break (documented: as if nudged towards +Y).

## Deviations and open issues

- **Build contention:** the shared `.cargo` build dir deadlocked on `proc-macro2` file locks with ~20 foreign cargo processes (no rustc running). To get unblocked I ran my cargo calls with a private intermediate dir (`CARGO_BUILD_BUILD_DIR` under the session scratchpad, since deleted); `CARGO_TARGET_DIR` was never set. I killed only my own two blocked cargo processes.
- No rustfmt on this toolchain (`rustfmt` component missing), files are hand-formatted to the repo style; clippy not run.
- The TS/Python oracle suites are run directly (vitest config / pytest). They are not yet registered as Nx targets, `launch.json` entries or in the BIM subset `🔮️oracles` registry (framework module level, not subset level; `.vscode/launch.json` is mid-rewrite by others). Registration is left to the oracle/tooling owners.
- `Cargo.lock` gained `parry3d` and `serde_json` under `semio-framework-geometry` (dev-only).
- `loops::offset` is a raw mitered offset (not self-intersection-cleaned); the robust path is `flatten` + `regions::offset_regions`.
- Not provided (out of scope of generic primitives): mesh booleans (use `semio-framework-3d` B-Rep when truly needed), stairs/roof-shape recipes (compose `prism_between`, `extrude`, `sweep_profile`), curtain-wall grids (compose `extrude` / `sweep_profile` per mullion/panel).
