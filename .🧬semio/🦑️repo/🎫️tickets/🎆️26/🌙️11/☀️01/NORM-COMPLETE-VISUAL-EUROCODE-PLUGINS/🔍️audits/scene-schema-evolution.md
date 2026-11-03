# Scene Schema Evolution (A1/A2 owner pass)

Status: written, NOT compiled (the integrator builds; request appended to `📌️important/requests.md`). Twins regenerated: `wp01-gen-twins.py --check` (96 types) and `wp02-gen-twins.py --check` green.

Root: `✏️s/🔌️plugins/📕️norm/🎬️structural-scene`.

## Changes (schema first, then twins, tessellation, viewport)
| Gap workaround in the verticals | Schema change | Where |
|---|---|---|
| EN 1992 square stirrups (`hole_clearance` side, one spacing) | `Pattern::Cage { outline: Ring2, zones: Vec<LinkZone{start,spacing,count}>, reference: Vec3 }` | `🧩️entities` |
| Evenly spread bars, bolts, piles | `Pattern::Points { offsets }` | `🧩️entities` |
| Per-instance / per-station / per-vertex tint, several scalar fields | `ColourChannel::Fields { active, fields: Vec<ScalarField{id,unit,samples}> }`, `FieldSamples::{Vertex,Station,Instance}` replaces the data-less `Scalar { field_id }` | `🎨️appearance` |
| Sinusoidal 1-D mode shapes, no alpha_cr, no playback | `DeformationOverlay.{surface: Vec<SurfaceSample>, motion: Motion, load_factor}` | `🎨️appearance` |
| Wall openings drawn evenly or not at all (EN 1991, 1996) | `WallAssembly.openings: Vec<WallCut{offset,width,sill,height}>` | `🧩️entities` |
| CLT / glulam layers (EN 1995) | `SolidShape::Laminate { frame, footprint, layers: Vec<GrainLayer{thickness,role,grain,label}> }` | `🧩️entities` |
| Weld runs per edge (EN 1993, 1999) | `FastenerKind::WeldRun { throat, weld, path, pitch: Option<WeldPitch> }` | `🧩️entities` |
| Section polygons with holes (EN 1999) | already present: `SectionShape::Polygon { polygon: Polygon2{outer,holes} }`; no schema change | - |
| Spline tendons | already present: `Tendon { profile: Path3::Spline }`; P(x) now expressible as a `Station` field | - |
| Animated parameters | `Motion` covers overlay playback only. Parameter tracks (fire time, creep age) are NOT modelled yet | open |

Validation: cage zones ascending and non-empty, reference non-zero; cuts inside the wall and disjoint; laminates non-empty; weld pitch positive; field ids unique and `active` present; station samples strictly ascending; `load_factor` positive; a mode overlay needs `samples` or `surface`.

## Consumers updated
- Tessellation `🔩️fasteners` (cage from outline and zones with rounded corners, points, `weld_run` with pitch), `🪨️solids` (`wall_with_openings` now accepts sill 0 as notch and openings touching the wall ends; assembly cuts per segment; laminate parts), tests extended with analytic volumes (walls with door and window, laminate 6 m2 x 0.11 m, weld runs 2 * a^2/2 per metre, intermittent 6 x 0.2 m).
- Viewport `🧱️parts` (wall pieces around cuts, laminate prisms, cage loops as edge cylinders, points, weld runs), `🎨️colour` (active field mean via the ramp), entity bounds for `Laminate`.
- Scenes: all 11 `DeformationOverlay` literals (prefix fields), EN 1992 `members::links` (real rectangular outline, no `hole_clearance`) and its test, EN 1991 parapet (`openings: Vec::new()`), EN 1997 settlement (`ColourChannel::Fields` with per-vertex settlements).
- Tests added in `🧪️tests/🔬️unit`: cages/points/weld runs, wall cuts and laminates, scalar fields and mode surfaces (round trip through the value codec plus path-addressed violations). Cross-checks against third-party code: the existing canonical-JSON-vs-serde_json test covers wire shapes; volumes above are closed forms.

## Remaining schema gaps found (not done)
1. Parameter tracks (animated time/age sliders), per-step snapshots.
2. Viewport per-vertex field ramps (`FieldSamples::Vertex/Station` currently reduced to the mean for flat shading; the `🎞️lowering` station path still reads `UtilisationBinding.per_station`). Consume `Fields` there next.
3. `Motion` playback in the viewport (the overlay is still drawn static with `deformation_scale`).
4. Plate/shell eigenmode application: `surface` samples are not yet applied by the viewport or tessellation.
5. Tendon force/stress and friction losses are only expressible as a coloured field, not as a typed `TendonProfile`.
6. Shell boundary conditions, torsional centres and similar are snapshot gaps, not scene ones.
7. A golden scene fixture for the new variants is still the empty file; fill it after the first green run.

## Notified
Owners via `📌️important/requests.md` (section "Scene-schema owner (A1/A2)"): EN 1992, 1991, 1995, 1996, 1993/1999, 1997, 1990/1998 plus the integrator.
