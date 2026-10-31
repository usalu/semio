# Physical Casing Implementation

The saved reconstruction uses the actual native triangle-surface body dimensions **2.60 × 4.20 × 2.50 m**, with a **0.25 m three-dimensional fillet** on all edges and corners. The documented **2.50 × 4.00 × 2.50 m** dimensions remain explicit nominal study, program and energy controls. The casing is symmetric about the existing reference centre: X ±1.30, Y −0.10…4.10, Z 0…2.50. All 33 confirmed pose/family corrections, world centres, phases, IDs and program assignments are preserved.

## Current Final Physical Gate

All **135 physical tests pass** after the conformal panel correction. The complete broad AST/JSON input map is identical at start and end; the hardened eleven-domain acceptance also verifies exact current physical inputs. No rendering-only drift needed an exception for this run. Physical manifest SHA-256: d1423bd15d7f3785fb515d710199611f86fe1aec0d9358e9709fb3c800768ffa.

The model AST is 87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084, identical to the parent-owned fresh 52-test structural proof. Thus **187 physical and structural checks pass on the same model**. Native replay remains on HOLD for final rendering acceptance.

| Domain | Tests | Seconds | Result |
|---|---:|---:|---|
| shared | 59 | 79.688 | PASS |
| 🚶️circulation | 18 | 21.781 | PASS |
| 📦️casing | 22 | 41.281 | PASS |
| 🧱️panel | 3 | 19.953 | PASS |
| 💊️detail | 9 | 10.813 | PASS |
| 🪟️window | 2 | 0.484 | PASS |
| 🧭️poses | 2 | 4.844 | PASS |
| 📍️survey | 6 | 0.000 | PASS |
| 🪟️unit | 8 | 44.578 | PASS |
| 🔧️mounts | 4 | 12.625 | PASS |
| 💡️light | 2 | 4.515 | PASS |

Strict schema aggregation passes: authored model accepted with 1,397 source-bearing leaves; 29 invalid shared models, casing 5, conformal panel 3, window unit 4, mount 3 and pose 4 malformed examples rejected. The complete-domain verifier rejects empty, missing, duplicate, wrong-count, unknown and failed suites; Python and AJV agree on eight examples.

## Project-Owned Interfaces

- capsule_casing returns physical fabrication bounds, deriving allowances once from the nominal and physical reference quantities.
- capsule_parameters_from_casing converts explicitly physical slider dimensions into nominal model parameters once, retaining supplied interior and window controls.
- capsule_interior returns authoritative usable bounds. Present fields are honored even when equal to reference values; missing dictionary fields derive the declared nominal-to-interior clearances.
- capsule_interior_profile exposes the actual faceted clear X/Z lining profile used by fixture containment and support clipping.
- capsule_entry returns the physical boundary Plane at base Z 0; the authored sill remains at finish Z 0.20.
- capsule_window returns the actual fixed-glass midplane Circle for every physical family.

Each interface accepts project CapsuleType values, keys or explicit parameter dictionaries as appropriate, and exports only project-owned types. The default usable interior is independently 2.30 × 3.80 × 2.10 m, from Z 0.20 to Z 2.30. The threshold joins the actual wall to the finished floor. The removed entry_standoff field no longer compensates for nominal-body gaps.

capsule_solid is the explicit nominal study envelope. slot_solid is a conservative physical collision envelope: analytic support tangents enclose the actual casing rather than underbounding it with an inscribed corner polygon. The old underbound was independently measured at 4.433 mm.

## Physical Material and Interior

Six planar panels, twelve cylindrical edge patches and eight triangulated spherical corners reproduce the recoverable native body. Real back and side door cuts continue through the lower rolled material at the unchanged Z 0.20 sill. Steel inner rounding follows the sourced 1.2 mm skin thickness. Sharp and sub-skin-radius controls use finite wall lofts without division by zero or negative inner radii.

The floor build-up bears on the actual steel floor and supports the finished surface. Trusses use member- and skin-aware cover, including at radius zero. Internal geometry is checked against the full three-dimensional rounded-body SDF. Lining inner radius remains positive; its outer radius is the inner radius plus lining thickness, including explicitly fixed small interiors inside wider casings.

Reference fittings map between usable bounds with floor/ceiling anchors; studies preserve authored primary footprint dimensions and counts. The shared reference mounts and cavity clipping provide actual bearing/contact instead of raising fixtures or concealing penetrations. Both entry jambs and the head use the same finish datum.

## Conformal Panel Coverage

The former radial outer-loop sampling omitted exact flat-panel rectangle corners and mismatched the lining quarter bands, leaving real baseline slits even while each plate was individually closed. The corrected cap sectors retain every exact outer-profile vertex and the unchanged N-angle aperture polygon independently. Outer-profile edge-fraction subdivisions correspond between lofted wall boundaries; only true outer and inner perimeters receive depth walls. No internal radial partition walls or resampled aperture create new gaps.

Lining end profiles use the same exact quarter vertices as their adjacent bands. The rectangular door decomposition canonicalizes clipping vertices before boundary cancellation using the existing material-weld identity, preserving original profile coordinates. Three new portable tests prove exact corners, unchanged aperture vertices, analytic area and loft volume, material corner rays, coherent manifold edges and every lining patch across both LODs, all six families and sharp/sub-skin/default radii. Poses, dimensions and aperture radii remain unchanged.

## Window Evidence and Unit

Native planar surfaces establish front glazing recess 0.065666667 m, side recess 0.065123725 m and reveal depth 0.066666667 m. Actual meshes and capsule_window share these explicit quantities. The opening is 1.30 m diameter with a finite 1.2 mm steel reveal/lip. The unsupported 80 mm exterior radial enlargement is removed; a separate interior countertrim remains explicitly assumed.

Direct component-relative GLUON fits support the authored common P/S longitudinal offset 0.60 m, with approximately 0.10–0.15 m uncertainty. The conflicting native kit value 0.950000008 m is retained as reference evidence, never a runtime fallback.

The PDF/WRI-backed finite window mechanism is implemented independently of body geometry: fixed outer pane, named fan guides and controls, radial folded paper, inferred inward-opening inner sash and hardware. See window-unit-plan.md for exact depth intervals, source/inference distinctions and standalone state limits. Furnished reference views remain closed; opening inspections do not claim clearance from movable pillows.

## Verification Chronology

Portable schema and source-bearing cases preceded implementation. Red cases caught nominal physical bounds, the inscribed collision envelope, oversized exterior ring, old glass recess, fixed-interior fitting overflow, negative lining radius, constant-extrusion overfill, incorrect jamb datum and sharp-radius failures.

The earlier 16-case full-body gate passed in 8.554 s. The expanded lining simplicity and physical-control factory checks then passed. The window-unit suite passed all 8 tests in 56.148 s, covering positive closed material, actual contacts, both LODs and mechanism states.

The final shared-model run subsequently exposed internal partition faces in the back door panel. The actual aperture material union now omits those internal faces. Its two original failing tests passed in 13.869 s. A stronger portable full/exterior × five radii × six-family manifold/containment regression also caught the sharp-wall variant and passed after correction: 2 tests in 25.564 s. See capsule-material-topology.md for the exact red/green chronology.

Final current shared, circulation and all physical-domain results are recorded from validate-physical-domains.py and verified by accept-physical-evidence.py with explicit semantic start/end/current maps. No barrier roles, material tests or clearance tolerances were removed to obtain a pass.

The final coherent-winding regression passed in 14.497 s across all six families, both LODs and five radius controls. Strict AJV aggregation also passed: 29 invalid shared models rejected across 1,397 source-bearing leaves; casing 5, mechanism 4, mounts 3 and pose fixture 4 invalid examples rejected. The integrated current-source physical acceptance is green as recorded above.
