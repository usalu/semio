# 📋️ Plan — Brep Kernel Dependency Free Runtime

Source of truth for scope: `.🧬semio/🦑️repo/✍️notes/semio_brep_kernel_audit_7ad363f.md` (§15 roadmap, §18 backlog). Evidence per subsystem: the `📓️explore-*.md` files in this folder.

Kernel root: `B = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep` (crate `semio-s-plugin-stdio`, module path `crate::artifacts::semio::standards::v1::subsets::brep`).

## Definition of done (audit §11.4)

1. No app imports `brepjs`/`brepjs-opencascade`/OCCT at runtime; they are `devDependencies` (oracles) only; `🔒️dependencies.json` regenerated so `productionReachable` is false.
2. Kernel core owns its neutral types (`Vec3`, `Aabb`, `ParamDomain`, `FaceGroup`, `MeshTransfer`, `PointClassification`); nothing in `✳️brep` imports `semio_framework_3d::engine`.
3. Every `BrepKernel` method carries capability metadata (`OpQuality`), no silent copies / ignored arguments / swallowed failures.
4. Transforms, primitives, sweeps, booleans (analytic surface classes), blends, offsets, draft are exact B-Rep — no tessellate→triangle-soup rebuild on the exact path.
5. Handles: ephemeral tokens separated from persistent labels; import clears the registry; arena GC; shell/compound handles; idempotent deconstruct.
6. One classifier on the BVH; p-curves produced by intersection and required by validation; closest parameter/UV exposed.
7. Artifact: lossless `Body ↔ SemioBrepSnapshot`; tessellation + mass-property inferences; viewer renders real tessellation; editor emits real mutations.
8. STEP: single implementation, p-curves/same_sense/units/ref_direction preserved.
9. Tests: language-agnostic feature/py/rs triplets per capability; differential corpus against the brepjs-generated fixtures (oracle only).
10. `launch.json` seed entries for the brep gates.

## Waves and file ownership

Workers edit ONLY the files listed in their row (plus new files under the paths they own). `⚙️engine/🦀️.rs` is shared: each worker edits only the `// #region` it is assigned, with small Edit hunks, never rewriting the file.

### Wave 1 — foundation (all parallel)

| id | worker slice | owns |
|---|---|---|
| W1-A | Neutral core types: define `Vec3`(=[f64;3] alias kept), `Aabb`, `ParamDomain`, `FaceGroup`, `EdgeGroup`, `FaceInfo`, `EdgeInfo`, `MeshTransfer` (+`edge_groups`, `face_infos`, `edge_infos`), `PointClassification`, `OpQuality` enum, in a new `B/🧬️schema/⚙️engine/🔖️contract/🦀️.rs`; delete them from `🧰️framework/🔨️modules/🧊️3d/⚙️engine/🦀️.rs`; repoint every importer (11 files, see explore-engine §5/§6) | contract file, framework-3d engine.rs, all `use semio_framework_3d::engine` sites outside `✳️brep` algorithm modules (bounding-volume, tessellation, classification, mesh-io, boolean, offset only change the `use` line) |
| W1-B | Exact affine transforms: `Trsf` → full affine (`Affine3` with mirror/non-uniform scale; analytic supports preserved under similarity, converted to NURBS otherwise), `Curve3/Curve2/Surface::transformed`, `Body::transform_solid` with `OpDelta` (modified labels), engine translate/rotate/scale/mirror/copy/patterns rewired; delete `transform_solid_mesh` | `📸️snapshot/➡️vector/🔢️matrix`, `transformed` impls in `➰️curve/🦀️.rs` + `🏄️surface/🦀️.rs` (append-only region), new `🔺️diff/🔁️transform/🦀️.rs`, engine `#region Transforms` |
| W1-C | Handle lifecycle: `Entity::{Shell,Compound}`, `GeometryHandle` ↔ `PersistentLabel` map, `import_*` reset registry, `Body::compact()` reachability GC from live handles, `deconstruct` idempotent by label, `dispose` reclaims, `retain` compacts; engine `registry_len`, `kind`, `deconstruct` | engine `#region Handles/Registry` + `deconstruct`/`import_*`/`dispose`/`retain`, `📸️snapshot/🕸️topology/🦀️.rs` GC section, `🏟️arena/🦀️.rs` |
| W1-D1 | NURBS math: exact rational derivatives (curve d1/d2, surface du/dv/duu/duv/dvv via de Boor derivative recurrence), periodic knot vectors, knot removal, general degree elevation, interpolation with end tangents + closed, error-bounded least-squares approximation; engine `interpolate_curve`/`approximate_curve`/`nurbs_surface_from_grid`/`coons_patch` made real | `➰️curve/🪢️bspline`, `➰️curve/🦀️.rs` NURBS arms, `🏄️surface/🦀️.rs` NURBS arms, `✂️curve-ops` interpolation/approximation fns, engine `#region Curves`/`#region Surfaces` |
| W1-D2 | Inverse evaluation: robust `closest_point`/`closest_uv` (subdivision/Bézier-clipping seeds, Newton with domain wrap, poles/apex, multi-solution), `Curve2` evaluation on surface, `Surface::isocurve`; engine `closest_parameter`, `closest_uv`, `closest_point` exposed | `🪡️surface-ops`, `✂️curve-ops` closest fns, `🏄️surface/🦀️.rs` isocurve region, engine `#region Evaluate` |
| W1-E | Exact primitives: sphere (two faces, seam + poles, p-curves), cylinder, cone (apex), torus as analytic surfaces with seam coedges and stored `Curve2` p-curves; keep `solid_from_triangle_soup` only for mesh import; `make_convex_hull` planar-merged faces | `🔺️diff/🧱️primitives/🦀️.rs`, engine `#region Primitives` |
| W1-F | One classifier: BVH traversal in `point_in_solid`, delete `classify_point_on_solid` from mass-properties, trim tests via stored p-curves, grazing/vertex rules; mass props on trimmed supports with error estimate; validation: shell closure, orientation consistency, sliver/degenerate, missing p-curve = error, self-intersection (face/face AABB+SSI probe) | `💡️inferences/🏷classification`, `🌳bounding-volume`, `📏mass-properties`, `✅validation-report` |
| W1-G | Tessellation: CDT (constrained Delaunay) replacing ear-clip+fan, stored p-curves for trims, seam/pole handling, crack-free shared edge samples, chordal+angular deviation control, `face_groups` keyed by persistent label, `edge_groups`/`face_infos`/`edge_infos` filled | `💡️inferences/🧩tessellation/🦀️.rs` |

### Wave 2 — exact modelling (after W1-B, D1, D2, E)

| id | slice | owns |
|---|---|---|
| W2-A | Intersections: exact SSI for all analytic pairs (plane/cyl/cone/sphere/torus incl. cyl-cyl, cyl-sphere, torus cases → circles/lines/ellipses or NURBS-fitted), marching + p-curves on both supports, curve/surface exact for analytic | `🔺️diff/✂️intersect` |
| W2-B | Boolean exact pipeline: SSI → edge/face splitting (Euler ops, imprint on non-planar) → cell classification (W1-F classifier) → select/stitch → sew → `OpDelta`; section/split exact; mesh fallback only behind `OpQuality::MeshDerived` opt-in | `🔀️boolean`, `🔺️euler` (imprint extension), `🧵️sew` |
| W2-C | Exact sweeps: extrude (planes/cylinders/NURBS extrusion surfaces from each profile edge, caps, p-curves), revolve (cyl/cone/sphere/torus/NURBS-of-revolution), loft/sweep/pipe (NURBS skinning, frames, guide honoured, `smooth` honoured), helix exact | `➡️sweep` |
| W2-D | Blends + offsets + draft: rolling-ball fillet on planar/planar, planar/cyl, cyl/cyl edge pairs; chamfer (asymmetric) via cutting planes; analytic offset surfaces, shell with open faces, thicken, draft about neutral plane with face-chain propagation | `🎨️blend`, `↔️offset` |

### Wave 3 — artifact, viewer, editor, STEP, flow (after W1-A/C/G)

| id | slice | owns |
|---|---|---|
| W3-A | Lossless `Body ↔ SemioBrepSnapshot` (coedges, p-curves, tolerances, labels, generations, knots); inferences `tessellation` + `mass_properties`; viewer main window uploads real `MeshTransfer` with labels; editor `set-vertex` → `MoveVertex`, operation→mutation batch compiler | `📸️snapshot/🦀️.rs`, `💡️inferences/🦀️.rs`, `👁️viewer/**`, `✏️editor/**`, `🧬️mutations/🦀️.rs` |
| W3-B | STEP: dissolve `⚙️engine/📄️step`, route engine export/import through `🚪️io` AP214; add p-curves (`PCURVE`/`SEAM_CURVE`), `same_sense`, `ref_direction`, units, diagnostics | `⚙️engine/📄️step`, `🚪️io/**`, engine `#region IO` |
| W3-C | Flow nodes: `OpQuality` metadata per node, no silent fallbacks, tests for each family | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/**` |

### Wave 4 — JS runtime removal (after W1-A/G, W3-A)

| id | slice | owns |
|---|---|---|
| W4-A | `SpatialKernel` first-party implementation: route the 18 OCCT-backed methods through the first-party Rust kernel over the existing JS→Rust bridge (`🧰️framework/🔨️modules/🧊️3d/🟦️.ts` flow_core pattern), STEP via `🚪️io`; `brepjs`/`brepjs-opencascade` → `devDependencies`; kernel id `semio-brep`; vitest keeps OCCT as oracle | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/**`, `✏️s/🔌️plugins/📐️cad/⚙️engine/🧱️brepjs`, cad `package.json`, `🔒️dependencies.json` (regenerate via script) |
| W4-B | Differential corpus: feature/py/rs triplets comparing first-party results with the brepjs-generated fixtures (`🧫️fixtures`), property tests (A∪A=A, transform∘inverse, tessellation convergence), `launch.json` seed entries | `B/🧪️tests/**`, `B/🏭️generator/**`, `.vscode/🧩️launch.seed.jsonc` |

## Rules for workers

- Test-driven, exact, no shims/adapters/deprecations; fix callee not call site; docstrings start with an emoji; no comments inside definitions.
- Verify by running: `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --message-format short` and `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-stdio --lib -- brep::<module>` in the FOREGROUND (never background). Report failures verbatim; never claim a pass you did not run.
- Concurrent peers edit the same tree; ignore unrelated churn; never run git write commands; never close/reopen the ticket.
- Every worker writes `📓️w<id>-<slug>.md` in this ticket folder (what changed, how verified, what remains) and keeps tool output under `🗑️generated/`.
