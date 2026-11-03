# WP-19 EN 1997 Visual Vertical (Phase 1: scene) - status: WRITTEN, NEVER COMPILED

My own `:check` waited about 80 minutes in the capacity queue and was cancelled under the one-build-per-artifact rule; a `ready for rebuild` line is in `🗑️generated/build-en1997/latest.md`. Expect compile fixes. Public API matches EN 1993: `project(&SceneInput)`, `diagrams(document, report, focus)`, `handles(document, placements, selection)`.

Phase 2 is written the same way (never compiled): editor windows `🧾️properties`, `🧊️viewport`, `📈️diagrams`, `✅️checks` (copied from EN 1993, EN 1997 ids, `lookup_field_meta` for units) replace `📥️inputs` and `📊️results`; edit mode layout, editor `🦀️.rs` (render arms, config owner, interaction definition, window kinds), Cargo deps, tests and TS barrel updated. Viewer shims `🧊️viewport`, `✅️checks`, `📈️diagrams` replace `📊️report`. Commands keep `resolve_edit` until the norm agent delivers checklist step 5 (request filed). Not adapted: catalogue panel stays EN 1997 specific.

## Built
Crate `semio-s-artifact-norm-en1997-scene` at `🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene` (registered in the EN 1997 `Cargo.toml` members and workspace dependencies, and in the package list of `📦️packages/🦀️rust/📜️script.ts`). Pure projection `project(&SceneInput) -> StructuralScene`, `diagrams(&SceneInput) -> Vec<Diagram2d>`, `handles(&SceneInput) -> Vec<Handle>`.

| Module | Delivers |
|---|---|
| `🧮️mechanics` | Actions and eccentricity, effective strip (Annex D.1), Prandtl-Terzaghi mechanism (active wedge, two log-spiral fans, two passive wedges), Coulomb/at-rest/increased-active driving pressure, passive pressure, resultants and lever arms, slip slices, deterministic critical-circle grid search on the catalogue `bishop`, Boussinesq corner-method stress and oedometric settlement, pile grid and resistances. Reuses `checks::catalogue` (`n_q n_c n_gamma`, `ka_coulomb kp_coulomb k0`, `slopes::{Circle,bishop}`, `settlement::newmark_corner`). |
| `📍️layout` | Default rows (footings y=0, piles y=-16, uplift y=-32, walls y=16, slopes y=40), persisted `Placement` with yaw, ground block from all footprints, `Pose` mapping. |
| `🪨️ground` | One translucent `StrataColumn` per layer (soil class role, phi'/c'/gamma label, thickness dimension bound to `depthBottom`), groundwater plane (bound to `groundwaterLevel`). |
| `🦶️footings` | `FootingBody` with pedestal, effective-area plate on the base (hatched, centred on the resultant), resultant and V/H/M glyphs of the governing load case, five-part bearing failure mechanism (opacity and colour follow bearing utilisation), sliding plane, failure glyph and label on warning/fail, dimensions B, L, d, B'. |
| `🧱️walls` | Inverted-T wall, backfill, active and passive wedges, driving and passive pressure surfaces (40 kPa per metre drawing scale), resultant glyphs, surcharge area load, coefficient label, dimensions. |
| `⛰️slopes` | Embankment prism, critical Bishop circle (arc surface), 12 slice prisms coloured by local Fellenius demand (Pass/Warning/Fail bands), factor label, slip failure glyph when F<1. |
| `🏗️piles` | Pile cylinders on a 3D grid, cap, group entity, per-layer shaft resistance glyphs, base resistance, cap compression/tension. |
| `💧uplift` | Slab with pore-pressure area load and stabilising/destabilising glyphs. |
| `📉️settlement` | 21x13 settlement dish (x25 exaggeration, scalar legend in mm) superposed over all footings, label with s and limit per footing. |
| `📈️diagrams` | Earth pressure vs elevation per wall (driving, passive, net, resultant markers), CPT and SPT step profiles with groundwater and investigation depth and derived phi'/E_oed markers, bearing capacity factors (log axis) with footing operating points, load-settlement per footing (working point, limit, bearing resistance). All with table twins. |
| `🕹️handles` | Layer interfaces (write both adjacent fields), groundwater level, footing width/length/embedment/horizontal load tip/position, wall height/embedment/base width/stem thickness, slope height/angle, pile length/diameter/count/position. Convention: `s` is displacement from the anchor, field = base + k s, grid 0.05 m. |

## Tests
`🧪️tests/🔬️unit/🦀️.rs`: validity of compliant and non-compliant scenes, expected entity ids, determinism and incremental hashing (footing edit leaves layers, wall, slope, pile hashes untouched), golden scene bytes (`🧫️fixtures/🌍️compliant-scene.json`, regenerate with `SEMIO_UPDATE_GOLDEN=1`), layout rule, placements, handle laws (zero-drag identity, every handle referenced by an entity), diagram validity and twins, mechanics closed forms. Oracle `🔮️oracle/🐍️.py` (numpy, scipy, shapely): bearing factors, wedge areas, spiral radius and depth, Boussinesq rectangle stress by `scipy.integrate.dblquad`, slice area against the shapely disc-under-ground area. Feature file `🧪️tests/🥒️.feature`.

## Hand-offs
See `📌️important/requests.md` (vis-en1997 rows): Placement field and mutations, public `slices()`, field-meta rows, shared scene kit.

## Deviations and open items
- Mechanism, wedges and pressure diagrams are characteristic-value pictures; the checks keep the design values.
- Increased active pressure is drawn as the mean of K_a and K_0 (visual rule, not the DIN 4085 table).
- Only the governing load case (largest eccentricity) of a footing is drawn.
- Pile loads are taken as group totals; per-pile utilisation is the shared pile binding (no N_i distribution, no negative skin friction, no block-failure prism yet).
- Hydraulic flow nets, anchored walls (V97-7, V97-8) and the slip-circle heat-map need snapshot fields that do not exist yet.
