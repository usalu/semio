# WP-03 and WP-04: Tessellation Kernel, Projection Framework, Scene Diff, Text Alternative

Agent A3/A4. Crate `semio_s_plugin_norm_structural_scene` at `✏️s/🔌️plugins/📕️norm/🎬️structural-scene` (CRATE below).

## Status

CODE COMPLETE, NOT COMPILED OR RUN. `repo run @semio-s/plugin-norm-structural-scene:test` failed after a ~1 h capacity queue with "Unknown project in the closure": the crate has no `📋️project.json`/`📜️script.ts`. Handed to the integrator via `📌️important/requests.md`. Expect first-compile fixes (borrow/inference details); logic was validated by Python ports (see end).

## What was built

### `CRATE/🧮️tessellation` (pure, deterministic, LOD-bounded)

| Module | Content |
|---|---|
| `🦀️.rs` | `MeshData` (positions, normals, indices, optional scalar slot = relative arc position, `aabb`, `volume`, `surface_area`, `is_watertight`, `merge`, `placed`, `impostor`), `MeshBuilder` (triangle soup welded and smooth-shaded below a crease angle), `Lod` (Lod0 box impostor, Lod1 coarse, Lod2, Lod3 filleted), `Tolerance` (chord sagitta), `segments()`, `MeshKey`, trait `Tessellate` implemented for `Geometry`, `geometry_bounds` |
| `📐️vector` | Copyable `[f64;3]` helpers (add, cross, Rodrigues rotation, minimal rotation between directions, perpendicular basis) |
| `🔺️triangulate` | Polygon with holes: Eberly hole bridging (duplicate-aware via local-inside test) plus ear clipping with distance-tolerant containment |
| `🧱️section-outline` | `regions()` (circles by chord tolerance, root fillets from optional parameter `r` for I/H, U, T, L; RHS outer `r` and inner `r - t`), `properties()` (area, centroid, ixx, iyy, ixy by Green), builders `plate_girder`, `deck_slab`, `composite_floor`, `rc_rectangle` (rebar layouts) |
| `📏️extrude` | Straight, tapered (linear blend of equal-topology outlines), curved (arc, clamped B-spline) and cambered members with parallel-transported frames, end eccentricities, capped ends, composite rebar bars at Lod2+, `tube`, `prism`, `sweep`, `straight_mesh` |
| `🔩️fasteners` | Bolt (head, washers, nut), stud, screw (4 head shapes), nail, dowel, anchor, fillet and butt weld, rebar with bend, stirrup loops and cages, tendon tube; `offsets()` for Grid, Row, Along patterns (bounded by `Lod::max_instances`) |
| `🪨️solids` | Prism, box, cylinder, wedge, mesh, strata column (per-stratum role parts), footing with pedestal, layered wall assembly, `wall_with_openings` (polygon-with-holes extrusion), `pile` |
| `🏹️glyphs` | Point/impact arrows, moment arc, line (profile-sampled arrow row), area (grid inside polygon), pressure zones, inertial masses, explosion shells, supports (fixed, pinned, roller, spring coil, elastic bed slab, pile head); nominal metre size, host rescales to screen space |
| `🌐️shells` | `revolve` (any closed profile, partial sweeps capped), shell bands with real thickness (mitred offsets), grid patches with thickness and rims, plates with holes and stiffeners, double-sided surfaces, `cylindrical_shell`, `silo_shell` |

Oracle `CRATE/🔮️oracles/🐍️.py`: rebuilds each profile from its parameters with shapely rectangle booleans and rectangle-decomposition moments (parallel-axis theorem), checks mesh volume = area x length, watertightness (numpy edge pairing) and bounds; cross-checks with `trimesh` when importable. Invoked from the Rust test `geometry_oracle_agrees_with_third_party_libraries` through `python_oracle::run_ok`.

Language-agnostic features: `🧮️tessellation/🧪️tests/🥒️.feature`, `🎯️projection/🧪️tests/🥒️.feature`.

### `CRATE/🎯️projection`, `CRATE/🔺️diff`

- `StructuralProjection` trait (spec section 3.9) with two additions the cache needs: `entities(&doc)` (document order) and `fingerprint(&doc, &EntityRef)` (canonical text of the entity subtree). `EntityEvaluation` is owned and domain-neutral (`CheckSample { check_id, utilisation, status }`), so the scene crate does not depend on the compliance crate.
- `SceneBuilder` + `EntityDraft`: append-only drafts (`member/plate/shell/solid/fastener/support/load/surface/group`), `named`, `detail`, `labelled`, `layer`, `role`, `style`, `opacity`, `hatch`, `deformation`, `pick`, `handle`, `utilise` (worst check, per-station samples, utilisation channel, stale style), `add` (seals the content hash, registers the default kind layer, grows the fragment bounds). `assemble()` concatenates fragments (global first, then document order), applies layer visibility, unions bounds, writes the description and validates the whole scene (a11y names in en and de, unique ids, hashes).
- `FragmentCache` keyed by `fnv64(entity fingerprint, evaluation, context, transitive dependency fingerprints and evaluations)`; `DependencyIndex` with transitive closures both ways (cycle safe); `SceneProjector::{project, project_delta}`; `ChangedSet` limits re-fingerprinting to the reverse-dependency closure, content keys still guard correctness, evaluation changes are hashed every pass (so they need no `ChangedSet` entry).
- `diff(a, b)` / `apply(delta, a)` (`DiffError`): longest order-preserving run stays, everything else is an upsert; `Add { index, entity }` is an upsert at its final index (moves an existing entity), keeping one patch per id as `SceneDelta::validate` requires.
- Text alternative (`🗣️description`): `describe(&scene)` (counts per kind, governing utilisation with element name, failing and stale counts, EN and DE) and `describe_entities(&scene)` (one line per entity); `assemble` stores `describe` in `scene.description`.
- Mock projection fixture `CRATE/🧪️tests/🧫️fixtures/🎭️mock-projection/🦀️.rs` (members, loads, dimensions, ground, legend; stored or chained layout).

## Laws tested (`🎯️projection/🧪️tests/🦀️.rs`, `🔺️diff/🧪️tests/🦀️.rs`)

diff(a, a) empty; apply(diff(a, b), a) = b; incremental = full over 80 xorshift-random mutation steps (add, remove, edit, utilisation) in both stored and chained layout; minimal delta (editing m3 of 8 changes exactly m3, l3, dim-m3-length with 3 re-projections and 14 reuses); evaluation-only change patches only the entity; chained layout propagates exactly through the dependency closure; reorder = at most two upserts; base mismatch and unknown id are rejected; bilingual descriptions.

## Deviations from the spec

1. `SceneProjector::project*` return `Result<_, Vec<Violation>>` (scene validation can fail).
2. `entities`/`fingerprint` added to the trait; `EntityEvaluation` is owned and neutral (see above).
3. `MeshData.aabb` is a method, not a field; `Tessellate::key` has no level (`MeshKey::at(lod)` appends it).
4. `ProjectionContext` gained `document_seq` (excluded from cache keys).
5. `Add` patches are upserts (documented in `🔺️diff/🦀️.rs`).

## Known gaps and requests (also in `📌️important/requests.md`)

- `Pattern::Cage` has no loop size: stirrups use `hole_clearance` (default 0.2 m) as square side. Request to add `width`, `height`.
- `SolidShape::WallAssembly` has no openings; the kernel supports them (`wall_with_openings`).
- Tendon `duct` is validated, not drawn. No mitre scaling at kinks of polyline axes. Taper requires equal-topology outlines (error otherwise).
- Python `trimesh` and `sectionproperties` are pinned in the root `pyproject.toml` but not installed in `.venv`; the oracle uses shapely and numpy (installed) and activates `trimesh` automatically when present. `sectionproperties` is not used (an FE-mesh adapter belongs in the shared harness).
- Pre-compile verification: the polygon triangulation, hole bridging, fillet generation, sweep with transported frames, revolve and the diff algorithm were ported line by line to Python and fuzzed (400 random hole layouts, 150 watertight prisms, 5000 random diffs, bent bar, stirrup, arc member, silo) before the Rust build could run; two real defects were found and fixed this way (bridge triangle orientation, duplicate-vertex bridge selection, plus exact fillet endpoints and collinear-tolerant containment).
