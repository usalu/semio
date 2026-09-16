# Process Concrete Forest Example (2026-09-16)

## Objective

Ship the hexagonal-cut concrete forest piece (`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau`) as a bundled `process3d`
example whose timeline applies every machine of the concrete catalog (`🧩️extensions/🧱️concrete` / `schema::ConcreteCatalog`:
diamond saw, wall saw, wire saw, core drill, rotary hammer, anchor setter, surface grinder) as a real, kernel-replayed
step — the way `timber-beam-joinery` applies the wood catalog.

## What existed

- `process3d` persisted a stock as one inline `WorkingSolid`; only `Box`/`Cylinder`/`Sphere` replay across sessions.
  `ImportedSolid` is a kernel-session handle (dead in the fresh session every `processed_mesh` opens) and `ImportedMesh`
  has no B-Rep, so neither could carry a real piece into an example.
- The piece exists as an exact one-solid STEP export (57 planar faces, 126 straight edges written as degree-1/3
  B-splines, `AXIS2_PLACEMENT_3D` with `$` ref directions), and the brep kernel reads STEP (`Brep::import_step_sync`).

## What was built

### `WorkingSolid::Reference { reference_id }` + shipped reference solids (`process3d/🦀️.rs`)

A durable, cross-session solid: the artifact ships exact STEP text under `🖼️assets/🌲️concrete-forest/📐️.stp`
(`REFERENCE_SOLIDS`, `reference_solid(id)`, `ReferenceSolid::import` / `brep_snapshot`), the replay
(`💡️inferences::solid_for_spec`) rebuilds it into real kernel topology on every session, `stock_extent` reads the
authored extent for capability rules, and the composed `stock_solid` child is content-addressed by the STEP text
(`working_solid_child_handle`) so no 57-face snapshot JSON is materialised inside the guest per mutation. Every codec
(pack tag `5`, retained cursor, mutation binary copy/observe/retirement), the editor's byte envelope, the inspection
panel (`stock_kind_reference` label) and the catalogue's validation context learned the variant.

### The example (`📚️examples/🌲️concrete-forest`, `🖼️assets/🌲️concrete-forest/🗣️.dsl.semio`)

Regenerated from `concrete_forest_scene()` (`📝️text/🧪️tests/🔬️unit/🦀️.rs`) via the real
`process_working_scene_to_snapshot` + `print_dsl` — never hand-transcribed. Stock: the piece, posed `(-5, -2, 0)`.
Steps (piece coordinates: slab `z ∈ [2.735, 3.0]`, beams `[2.285, 2.735]`, columns at `(2.7, 2.338)`/`(8.1, 2.338)`):

| step | machine / capability | measure |
|---|---|---|
| Wall Saw Slab Cut | `wallSaw` / `wallCut` | box 4.5 mm × 6 m × 0.32 m at `x = 5.4`, 0.32 m from the top |
| Diamond Saw Beam Relief | `diamondSaw` / `crosscut` | box 4 mm × 0.5 m × 0.125 m into the beam underside on the same line |
| Wire Saw Column Cut | `wireSaw` / `wireCut` | 11 mm blade through the right column at `z = 1.2` |
| Core Sample | `coreDrill` / `core` | Ø102 blind core, 0.2 m into the slab at `(6.0, 3.6)` |
| Anchor Hole | `rotaryHammer` / `anchorHole` | Ø20 × 0.16 m at `(4.5, 1.2)` |
| Set Anchor | `anchorSetter` / `anchor` | Ø16 × 0.19 m, seated 10 mm into the hole floor, 30 mm proud |
| Grind Surface Patch | `surfaceGrinder` / `grind` | Ø250 pad, 5 mm off the slab top at `(3.5, 3.5)` |

Registered as `crate::examples::concrete_forest` (id `concrete-forest`, `PROCESS3D_EXAMPLE_CONCRETE_FOREST`),
handled by `setActiveExample`, listed in the action's `exampleId` select, and — new for this plugin — published
through `.editor_with_examples(…, examples())` so the react shell's example picker offers `demo` and
`concrete-forest`.

Laws: `concrete_forest_example_fixture_is_the_authored_scene`, `concrete_forest_example_applies_every_concrete_machine`,
`concrete_forest_example_replays_every_step_on_the_kernel` (stock volume 14.0998 m³ = the GLB's own signed volume; every
cut removes and the anchor adds material), plus the example leaf's own laws.

### Kernel fixes the piece forced (stdio `🧊️brep`)

1. **STEP reader — optional placement slots.** `AXIS2_PLACEMENT_3D` read its three slots positionally-blind
   (`parse_refs`) and demanded three references; ISO 10303-42 makes `axis` and `ref_direction` optional and every
   Rhino/ST-Developer export writes `$` for the ref direction. Now `parse_slots` keeps positions, absent `axis` is `+Z`,
   absent `ref_direction` is the standard's `first_proj_axis`.
2. **STEP reader — straight B-splines are lines.** A B-spline whose control polygon is collinear is read as the
   `Curve3::Line` between its edge vertices. Before, the NURBS carrier made the piece's volume read 17.70 m³ against a
   true 14.0998 m³ (the GLB oracle) — the kernel measures and imprints analytic lines exactly.
3. **STEP reader — p-curves.** Imported faces had no coedge p-curves at all (the kernel's own primitives always attach
   them); every boolean classification samples loops through p-curves, so even a STEP-round-tripped box could not be
   drilled (`no interior UV sample found`). `attach_pcurves` projects each edge onto its face surface.
4. **Validator — shared vertices are not self-intersections.** The self-intersection probe sampled face vertices and
   flagged every pair of non-adjacent faces sharing a vertex (valence ≥ 4: notch corners, hexagonal columns meeting
   beams) as "within 0 of each other" — 99 warnings on the untouched piece, and every boolean result failed its own
   validation. Samples on topologically shared vertices are exempt now; AABBs and samples are computed once per face
   instead of per pair.
5. **Validator — sliver probe cost.** `SLIVER_PROBE_TOL` was `1e-3`, which drove the adaptive quadrature to depth 6 on
   every cylindrical face: `DegenerateFaces` took 8 s of an 11 s validation (debug) after a few bores. It is `1e-2` now
   (the verdict is `area < 1e-14`), and `segments_for_chord_deviation` caps the angular step at `π/4` so a coarse
   tolerance can never collapse a small circle to two points (an 8 mm bore measured area 0 = "sliver").

Laws: `real_export_reads_optional_placement_slots_and_straight_bsplines_as_lines`, `read_faces_carry_pcurves_and_validate_clean`,
`read_solids_take_booleans`, `coarse_chord_tolerance_keeps_small_circular_faces_measurable`.

## Kernel gaps found and left (documented, not worked around silently)

- A **through** bore/box across a non-rectangular planar prism fails the boolean (`orientation-inconsistent` /
  `hole-loop-winding-inverted`): reproduced on a `convex_hull` parallelogram/trapezoid slab, not on a box or a rotated
  box. The example's core is a blind core sample because of this.
- Fusing a cylinder **coincident with a hole wall** (dowel-style) fails on this piece and on a plain slab box
  (`non-manifold-edge` / leftover coincident faces); fusing over a hole loop hits `split_face_by_chain does not support
  faces with inner loops yet`. The example drills the hole 2 mm wider than the anchor and seats the anchor 10 mm into
  the hole floor.
- The boolean is **translation-sensitive**: the same seven steps replay with the piece posed at `(-5, -2, 0)` and at
  the origin, but the core step fails at `(-5.4, -2.3385, 0)`.
- Validation still costs ~1 s per boolean on the processed piece (debug): `DegenerateFaces` ≈ 1.0 s, seven `cheap`
  phases ≈ 50 ms each.

## Verification

Native: `cargo test -p semio-s-artifact-process-process3d --lib` — 304 ok / 32 red (all in the e2e ticket's pre-existing
classes: render-harness serialisation, `🌉️wasm` retirement factory, `brep:out` export kind, dispatch-effect laws) plus
`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` still running after 15 min of CPU
(killed; a 200 000-turn maintenance loop). `cargo test -p semio-s-artifact-stdio-semio --lib -- …step …mass_properties
…validation_report …boolean …primitives` — 78 ok.
