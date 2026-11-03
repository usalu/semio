# WP-18 EN 1996 Visual Vertical

Status: phase 1 (scene) and phase 2 (editor and viewer windows) WRITTEN. Build status: see "Verification" (updated when the queued `check` run exits).

Root: `NORM/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any` (`SUBSET`). Rollout checklist followed: `📌️important/artifact-rollout-checklist.md`. Scripts: `🧰️scripts/vis-en1996-rollout.py` (phase 2 copy and wiring).

## 1. Scene crate `semio-s-artifact-norm-en1996-scene` (`SUBSET/🎬️scene`)

Registered: member in `🪨️en1996/Cargo.toml`, `[workspace.dependencies.semio-s-artifact-norm-en1996-scene]` and `[workspace.dependencies.semio-s-plugin-norm-structural-scene]`. The en1996 `📜️script.ts` has no `packages` list, nothing to add there.
Deps: `-model`, `-schema` (for `checks::masonry`: `AnnexParams`, `f_k_pa`, `phi_i`, `phi_m`, `fire_min_thickness_m`, `numerical_models::{Edges, yield_line_pressure, MnSection}`), `norm-compliance`, `norm-structural-scene`.
Public API (same shape as EN 1993): `project(&SceneInput) -> StructuralScene`, `diagrams(document, report, focus: Option<&str>)`, `handles(document, placements, selection)`, `Layout`, `Placement`, `ENTITY_KINDS`, `entity_kind_of`, `entity_dependencies`, `profile::{effects, profile, governing, lateral}`, `loads::{spreading, yield_regions}`, `bond::overlap_need`.

| Module | Content |
|---|---|
| `🧰️kit` | entity builder with nested paths (`walls[id=W].openings[id=O]`), mesh builder (cuboids in wall coordinates), utilisation bindings from the report (`UnverifiedBasis` maps to warning), layers, legends, registry (wall, opening, concentrated load) |
| `📍️layout` | default rule: walls along `+X` with a 3 m gap, thickness towards `+Y`; openings spread evenly between the piers; persisted placements (origin, yaw) win; `solid_rects` cuts the openings out of any height band |
| `🧮️profile` | EN 1990 combinations and the vertical state of a wall (N at top, middle, bottom, e_top, e_bottom, e_init, e_hm, e_k, e_mk, Φ per station, utilisation per station, σ for shear); `Lateral` (m1, m2, edges from the held sides, w_Ed); computed through the same public `masonry` functions as the checks |
| `🧱️walls` | per wall: group entity (handles, governing binding, clause badge), three utilisation zones (bottom, middle, top) as cut solid meshes bound to the compression checks per station, window plates (glass) and door plates, concrete lintels (bearing 0.25 m each side), floor plate bearing on the wall with the bearing depth, base support and held-edge supports by `supportSides`, dimensions for length, height and thickness bound to the leaves, label, failure glyphs per failing check (crushing, sliding, buckling) |
| `🧩️bond` | explicit running bond: courses of `hu + tbed`, half-unit offset, perpends, units cut by openings, mortar bed and perpend joints as inset geometry; overlap law `max(0.4 hu, 45 mm)` colours an invalid bond in error style; hidden `bond` layer; unit budget 20 000 per projection, beyond it only the joint lines |
| `🏋️loads` | resultant N_Ed at the top and bottom with the eccentricity (declared + bearing + rotation, at least 0.05 t) and a dimension; concentrated loads with the 60° spreading zone to mid-height (width = bearing + h/√3); wind pressure glyph; indicative yield-line regions (two, three or four held sides) bound to the M-31 yield-line check |
| `🔩️reinforcement` | vertical bars and bed-joint layers of reinforced walls, diameters from the declared steel areas, patterns along paths |
| `📈️diagrams` | `ecc` (e over height with limits 0.05 t and 0.2 t; top and bottom markers drag the declared eccentricities), `phi` (Φ top, middle, bottom with the checks), `capacity` (N_Ed against N_Rd along the height), `slenderness` (Φ_m over h_ef/t for e/t 0.05 to 0.3, the wall curve, limit 27, marker drags the wall height through ρ_n), `fk` (f_k over f_b for f_m 2.5 to 20 and the wall; marker drags f_b through the shape factor), `shear` (f_vk line, uncapped line, f_vlt cap, σ_d marker), `lateral-alpha` (α₁ over h/ℓ), `lateral-capacity` (w_Rd over h/ℓ against w_Ed; both drag the wall height), `fire` (t_min over the fire class for three μ_fi, wall thickness, marker drags the required class with `Choice` snapping), `mn` (plane-section M–N curve of reinforced walls with the wind moment); every diagram has a table twin and EN+DE texts |
| `🕹️handles` | wall length, height, thickness, held sides (count stepper); opening width, height, sill; floor bearing depth and span; eccentricity at top and bottom; concentrated bearing length; all with snap grids, limits, EN+DE accessible names; ids listed on the owning scene entities; `s` is the coordinate along the axis measured from the wall origin, bindings are affine in `s` |

## 2. Tests (language-agnostic first)

- `SUBSET/🎬️scene/🧪️tests/🥒️.feature`: openings and zones, utilisation colouring, bond, 60° spreading, yield-line regions, handles, diagrams and oracle.
- `🔮️oracles/🐍️.py` (stdlib, independent): mesh volumes of the zones against `(L h − Σ openings) t`, vertex bounds, unit volume, spreading widths, Φ_m of Annex G at every sample of the slenderness curves, f_k power-law slopes and caps and the f_m exponent, the tabulated fire thickness for clay group 1, the isotropic square yield load 24 m / l², table twin lengths. Spawned through `norm::python_oracle::run_ok`.
- `🧪️tests/🔬️unit/🦀️.rs`: valid scene for six fixtures, unique ids and layers, determinism, zone volumes, opening spans, zone ratios equal the report utilisation of the combined and the mid-height check, every binding names a report check, profile reproduces N_Rd of the report, failing wall style, layout and placement, bond and unit budget, spreading law, yield regions tile the wall, supports by held sides, reinforcement, handle coverage and anchors write the current value, length drag snapping, registry and dependencies, diagrams validate with table twins, Φ and eccentricity markers, f_k marker and drag, lateral capacity equals the M-31 check limit, fire marker, focus, golden scene, oracle, 1 000-wall timing.
- Golden `🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` is empty until the first run with `SEMIO_UPDATE_GOLDEN=1`.

## 3. Editor and viewer windows (phase 2)

- Editor: deleted `🪟️windows/{📥️inputs,📊️results}` (Rust, TS twins, tests, window-ownership test); added `{🧊️viewport,📈️diagrams,🧾️properties,✅️checks}` (Rust, tests, TS twins) from the EN 1993 reference; edit mode layout (properties 22 %, model 50 %, checks above diagrams 28 %) and its test; `✏️editor/🦀️.rs`: imports, `ENTITY_KIND_SCENE_KINDS`, `ChecksWindowConfigOwner` registered and used as `ResultsWindowConfigOwner`, render arms (focus = entity of the selected check, diagrams fall back to the worst entity), `viewport_interaction_definition`, four window kind definitions; `Cargo.toml` deps (`-scene`, `norm-structural-scene`); TS twin exports; root unit test body keys and focused window.
- Viewer: deleted `📊️report`; added read-only `{🧊️viewport,📈️diagrams,✅️checks}` (no controller, no editor import); `view` mode layout 50/25/25 and its test; `👁️viewer/🦀️.rs` render arms and window kinds; `Cargo.toml` deps; TS twin exports.
- Field meta: every handle and marker write path already has a `FieldSpec` row (`lengthM`, `heightM`, `thicknessM`, `supportSides`, `openings[].widthM/heightM/sillHeightM`, `slabBearingDepthM`, `eccentricityTopM/BottomM`, `fBPa`, `fireReiMin`, `loadCases[].slabSpanM`, `concentrated[].bearingLengthM`).
- Commands stay on `EDIT_RULES.resolve`: the `[id=…]` selectors of handle paths resolve through the app-surface `address`, no new `resolve` function was needed; the existing `change-*` kinds cover every handle leaf.

## 4. Deviations and honest gaps

Spec V-M1 … V-M9 coverage against the current snapshot:

| Spec | Delivered | Gap |
|---|---|---|
| V-M1 wall 3D | zones, openings, lintels, floors, bond with perpend offset, invalid overlap colouring, mortar as inset geometry, budget LOD | ring beams, pier highlighting, header/English/Flemish bonds, textured planes beyond distance, LOD instancing (no bond kind in the snapshot; the unit mesh is capped at 20 000 units) |
| V-M2 eccentricity and Φ | `ecc`, `phi`, `capacity`, `slenderness` | buckling shape graphic and restraint icons |
| V-M3 lateral | yield-line regions by held sides, wind pressure, utilisation on the regions, `lateral-alpha`, `lateral-capacity` | moment contours, arching thrust line; the regions are indicative (ridge at mid-height, corner offset from m2/m1), not the solver optimum |
| V-M4 shear walls | `shear` diagram, sliding glyph | compressed zone block, flange widths, overturn line (no flanges, no in-plane actions in the snapshot) |
| V-M5 concentrated load | 60° zone, bearing length handle, binding to the §6.1.3 check | local crushing region, β annotation; load position is not in the snapshot (spread evenly) |
| V-M6 reinforced | bars and layers, `mn` curve with operating point | anchorage length, strain diagram |
| V-M7 cavity wall | not delivered | no cavity, leaf or tie data in the snapshot |
| V-M8 fire | `fire` table chart, marker bound to `fireReiMin` | plaster equivalent thickness |
| V-M9 parameters | `fk` (formula curves), `slenderness`, `shear` | table points next to the formula, f_k surface over f_b and f_m |

Further honest notes:
- `🧮️profile` replicates `design_effects` and `compression` of the private check code; a law test compares N_Rd of the report. Request logged to export a public profile.
- Opening and concentrated-load positions are layout rules, not document data; request logged.
- Handles for storey heights, tie spacing and flange widths do not exist because the fields do not.
- The placement is not persisted; the editor and viewer pass `&[]`.
- `UnverifiedBasis` maps to the warning status in the kit.
- The window-ownership fixture of `🪟️results/🎚️config` and the hub component manifests still list the removed EN 1996 windows (requests logged).

## 5. Requests

Appended to `📌️important/requests.md` under "WP-18 EN 1996 visual vertical": Placement fields and mutation, opening and load positions, missing snapshot entities (cavity, ties, flanges, confinement, bond kind, storey heights), public `wall_profile`, `fire_state`, results-config fixture and hub manifests.

## 6. Verification

- `repo run '@semio-tech/norm-en1996-rs:check'` started once (log `🗑️generated/vis-en1996/check1.txt`); see the result line below.
- Test command after `check` is green: `SEMIO_UPDATE_GOLDEN=1 repo run '@semio-tech/norm-en1996-rs:test'` once to write the golden, review the diff, then without the variable.

Result: not compiled. Per the new rule visual agents start no builds; the norm agent builds (see `🗑️generated/build-en1996/latest.md`, "ready for rebuild" line written).
