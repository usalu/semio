# r4 execution report: i-openings (Wave I, `🪟️opening-frames`)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `A` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`. Value type and conventions: `T/r4-api-opening-frames.md`.

## 1. Result

`opening_frames: BTreeMap<opening id, OpeningFrame>` is a field of `ModelInference`, an `InferredField` DAG storey → host (wall or curtain wall) → opening. Verified in an ISOLATED build (snapshot + diff + storey-levels + wall-layout + curtain-layout + opening-frames, real sources compiled in place): `cargo test --lib` 53 passed (16 are `opening_frames`, 37 are the f1/i-walls tests that compile in the slice), `wasm32-wasip2` check green, python oracle `check` and the platform's `oracle quick --case 🪟️infer-bim-1-opening-frames` green (2 scenarios). The whole-crate gate command could NOT run (see 6).

## 2. What it derives (per opening)

Resolved width/height/sill (override else type; window sill = type sill + opening sill, door/void sill = opening sill), point + tangent on the host axis at arc length `offset` (line and arc, `BulgeSeg`), local frame (origin on the location line at `base_z + sill`, `x` tangent, `y` left normal, both turned by `flip_facing`, right-handed), world frame (building origin, rotation, `site + building` datum), cut rectangle `(s_min,s_max,z_min,z_max)` in the host development (arc-length interval on arcs), `reveal_depth` = host thickness plus `face_front`/`face_back` (location-line offsets from `wall-layout::offsets_of`, swapped by `flip_facing`), door hand (type swing xor `flip_hand`), plan strokes (window glazing line; single/double door leaf lines and 90 degree swing arcs hinged on the +y face; void none), and validity `issues` (`HostMissing`, `TypeMissing`, `NonPositiveSize`, `OutsideHostExtent`, `BelowHostBase`, `AboveHostTop`, `OverlapsSibling`) with `overlaps` ids and `valid`.
Parametric: the host node resolves exactly like the walls (`wall_layout::{top_of, thickness_of, axis_length, offsets_of, constraint_storeys}`, `curtain_layout::curtain_layout_of`), so a storey height, wall top, base offset, type, location, axis edit (move, curve, shorten) or an opening/type edit re-derives exactly the dependent frames; an opening's `dep_input` includes its sibling cut rectangles (overlap), a host's the building origin/rotation.

## 3. Files

Created: `S/🧬️schema/💡️inferences/🪟️opening-frames/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs}`; fixtures `S/🧫️fixtures/💡️inferences/🪟️opening-frames/{🏡️placed,⚠️invalid}/{📸️snapshot/🔣️.json,💡️inference/🪟️opening-frames/🔣️.json}` (snapshots handwritten; tables WRITTEN by the oracle); oracle case `S/🧪️tests/🪟️infer-bim-1-opening-frames/{🥒️.feature,🐍️.py,🦀️.rs}` (scenarios `opening-frames-placed`, `opening-frames-invalid`; oracle id `bim-1-shapely-geometry` reused, no registration change); `T/r4-api-opening-frames.md`, this report; kept inputs `T/r4-i-openings-{facets,isolated,priv-setup}.ts`.
Updated (surgical): `A/📦️packages/🦀️rust/Cargo.toml` (+`semio-framework-geometry`, workspace); `A/🦀️.rs` (+`opening_frames` mount); `S/🧬️schema/💡️inferences/🦀️.rs` (+field, import, `infer` entry, `InferenceFieldSpec`); aggregate facets `🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto` (proto field 4; types `Vec3, Frame, OpeningCut, OpeningIssue, PlanRole, PlanShape, PlanStroke, OpeningFrame`, shared `Point2`/`Swing` only if absent); `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` (+`opening-frames` table in `encode_inference_projection_json`).
Peer files touched (blocking compile, trivial): `S/🧬️schema/💡️inferences/🧊️element-solids/{🧱️walls,🪟️curtain-walls}/🦀️.rs` build `HostExtent` literals; I added the two new fields `face_left`/`face_right` (from `offsets_of` / mullion depth / 2). They should rather call `opening_frames::host_extent(snapshot, host_id, own, target)`.

## 4. Tests (all in `S/🧬️schema/💡️inferences/🪟️opening-frames/🧪️tests/🔬️unit/🦀️.rs`)

Window on a line host (type size, sill, position, local + world frame incl. rotated building and datum, glazing stroke); override size and window sill rule; flip_facing (frame turned, right-handed, origin stays on the axis); door on the floor and hand (type swing, `flip_hand`, single and double leaves, affine maps local points to world); arc host (point on the circle, tangent, left normal towards the centre, arc length, arc-length cut, flipped door); storeys above, curtain-wall host (mullion depth); faces per location line and facing; all seven validity issues by name on the invalid model; storey height lowered makes a window cross the wall top and raising heals it (and lifts the storey above); host axis moved, shortened and curved via the same `ModelDiff` patch `set-wall-axis` emits (frames follow, door leaves the extent); opening offset slid then overlap follows; DAG parents (opening to host, host to storeys, orphan has none, topological order); host extent equals `wall-layout` height/length/thickness/base; determinism, empty model, one frame per opening; `infer_field_after_diff` gating (material edit untouched, window type edit re-derives only its openings); third-party oracle tables reproduced (1e-9) for both cases.

## 5. Commands and results

| Command | Result |
|---|---|
| `python 🐍️.py write`, then `check` over `S/🧫️fixtures/💡️inferences/🪟️opening-frames` (shapely 2.1.2, numpy) | `oracle agrees` (shapely audits: walked axis point and tangent within 1e-5 of the closed form, cut box area, containment in the host rectangle, sibling intersections; numpy: right-handedness) |
| `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🪟️infer-bim-1-opening-frames` | `executed=2 passed=2 failed=0` |
| `gate.sh i-openings -- cargo test --manifest-path <private>/Cargo.toml -p semio-s-artifact-bim-model --lib` (isolated lib root, real sources through a junction; recipe in `T/r4-i-openings-isolated.ts` + `T/r4-i-openings-priv-setup.ts`) | `53 passed; 0 failed` (16 `opening_frames`) |
| same with `--target wasm32-wasip2 cargo check` | exit 0; no warnings from the module |
| `gate.sh i-openings -- cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib` | NOT RUN to a verdict: see 6 |

## 6. Open issues

1. Whole-crate gate blocked by others (not mine): the crate depends on `semio-s-artifact-stdio-ifc` (added by x-ifc), which pulls `semio-s-artifact-stdio-contract` / `-step` that do not compile (`MutationDiff::apply` now takes 3 arguments; diff-only mutations ticket in flight), and `semio-framework-os-infinite` fails on `IconName::CloudDownload` (the gitignored generated icon enum predates the committed `🌧️cloud-download.svg`; needs the icon generator run). Slots with cached artifacts still pass; fresh ones do not. The private workspace trick (drop the ifc dependency, as m-frame did) gets past it; the main crate still showed foreign errors (editor/viewer/io export/bodies/curtain-layout `use super::wall_layout`, `mutations` descriptor panic) when last tried, so re-run the gate command after the peers finish.
2. `curtain-layout` (i-walls) imports `super::wall_layout` where its mount makes that path `super::super::wall_layout`; in my isolated lib I worked around it with a private `pub use`, the real file needs the fix.
3. `Point2` has no `Default`, so `OpeningFrame` implements `Default` by hand; if the model generator derives `Default` for `Point2` this can shrink.
4. Subject adapter `🦀️.rs` and the platform `subject`/`parity` roles were not run (they need the whole crate).
5. Window sill semantics are a decision (`Opening.sill` is a plain `f64`, so "type default" = 0 added to the type sill); the mutation leaves `create-opening`/`set-opening` should default `sill` to 0 for windows.
6. Curtain-wall hosts only provide a plain cut (no grid avoidance); plan hinge sits on the +y face, glazing on the location line.

## 7. Scratch

`T/🗑️generated/i-openings/` (logs, private workspace with junctions) deleted (junctions removed first). No temp files left in `/tmp`.
