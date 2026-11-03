# Catalogue bridge v5 (wave 2): hand-curated geometry for every design case

Status: UNCOMPILED. Nothing built or run except the Python validator/generator (`geometry_gen.py --check` is green for all six codes). All Rust is unverified by a compiler. The `repo` MCP failed to connect (CONNECTION_CLOSED), so no ticket tool calls were possible; work is filed under this ticket by path.

## 1. What changed
- Heuristic gone: `🧰️scripts/catalogue-bridge-v4/geometry_gen.py` (keyword form classifier, priority lists, overrides) is deleted. `🧰️scripts/catalogue-bridge-v5/geometry_gen.py` only validates and writes: one row per registry entry and no extra ids; the form is a `CaseForm` variant; every `aux` role belongs to `forms::aux_roles(form)` (parsed from the Rust source); every key is a declared input, dimension and length roles are length inputs (Metre or Millimetre), `layers` is a series input, count/angle/ratio roles and the load are number inputs. `--check` compares the committed `🦀️.rs`.
- The per-artifact `📐️geometry/🔣️.json` (30 + 98 + 174 + 70 + 40 + 71 = 483 entries) is now the hand-maintained source. Each of the 483 rows was written by hand against the clause title and its declared inputs and units (one-off curation script and converter deleted afterwards, no migration left).
- Schema (`⚖️compliance/🧰️design-case`): `CaseGeometry` gained `aux: &[AuxRow]` (`(role, key)`, empty key = draw the role nominally) with `aux_key(role)` and `keys()`; `CaseForm` gained Symbol, Fastener, Weld, Anchor, Opening, Footing, Pile, RetainingWall, SheetWall, Slope, SoilBlock, GroundAnchor, Seepage, Well.

## 2. New forms (projection-kit `🗂️design-case/🧊️forms`, with grips)
`forms/🦀️.rs` (dispatcher, `Meas`, `Part` with context/opacity, `Plan` grips, `signature`, `aux_roles`, `aux_is_scalar`, `grid_for`), `🌍️ground/🦀️.rs`, `🔩️joints/🦀️.rs`.
| form | draws | aux roles |
|---|---|---|
| Fastener | two lapped plates, row of count bolts with heads | thickness2, diameter, count, spacing |
| Weld | base plate, upright, two fillet beads (45 deg boxes, throat) | throat |
| Anchor | concrete block, headed anchor embedded | embed, diameter |
| Opening | slab as four strips around the opening | open_w, open_l |
| Footing | pad + column stub at embed on translucent soil layers, groundwater sheet | embed, layers, water |
| Pile | pile or rows x columns group under a cap in soil | rows, columns, spacing, layers, water |
| RetainingWall | stem, base slab with toe/heel, backfill, groundwater in the backfill | base, base_t, toe, water |
| SheetWall | embedded wall, retained soil, excavation, prop, groundwater | embed, prop, water |
| Slope | wedge slope + crest on ground, soft layer, water level | angle or ratio, soft, level |
| SoilBlock | translucent block, layer interface sheets, groundwater, test rod | layers, water, rod |
| GroundAnchor | wall, inclined free tendon and fixed bore in soil | free, angle |
| Seepage | soil layer between two water levels differing by the head, cut-off | embed, layer |
| Well | aquifer cylinder, casing, three stepped drawdown discs (log curve) | radius, level |
| Symbol | criteria sign on a plinth (rule/checklist cases, no number) | |
Soil and water are context parts (opacity 0.3/0.55, role colour, no utilisation, no grips); the body keeps the utilisation colour and the grips. Each size grip snaps to a drag step that follows the dimension (`grid_for`: 5 cm, 1 cm, 1 mm); `MIN_SIZE` is now 2 mm so mm-sized dimensions (throat, bolt diameter) respond. No catalogue entry maps to Weld today (EN 1993 has no case catalogue); the form is available.

## 3. Forms per code (counts)
EN 1990: Gauge 17, Symbol 8, Load 2, Column 1, Block 1, Member 1. EN 1991: Gauge 23, Load 21, Plate 15, Block 13, Member 8, Column 7, Cylinder 6, Symbol 3, Wall 1, Connection 1. EN 1992: Member 58, Gauge 39, Plate 18, Column 17, Anchor 14, Cylinder 11, Wall 5, Block 4, Symbol 4, Footing 2, Opening 1, Connection 1. EN 1995: Member 29, Fastener 14, Plate 12, Column 6, Gauge 4, Symbol 3, Block 1, Opening 1. EN 1996: Wall 28, Gauge 6, Block 2, Cylinder 2, Plate 1, Member 1. EN 1997: SoilBlock 14, Pile 10, RetainingWall 8, Slope 7, Footing 6, GroundAnchor 4, Gauge 4, SheetWall 3, Well 3, Seepage 3, Block 3, Cylinder 2, Load 1, Member 1, Column 1, Symbol 1.
Examples of fixes: T-71/T-72/T-73 are Fastener (diameter d, t1, t2, steel plate) instead of Member; T-18 holes are an Opening (cross-section with the hole diameter); T-123 Fastener with n bolts; D-03/B-0x bars are Cylinders of diameter phi; FA-xx are Anchors (hef = embed, d = diameter); DT-12 and P-06 are Footings; P-01 is a slab with opening (opening_w, opening_a); LQ-06 prestressed tank is a Wall under pressure; R-05/A-09..A-13/BR-08 columns take the section as plan and l0 as height; EN 1991 silo entries are Cylinders of diameter dc; bridge/footbridge entries are Plates of deck width and span; EN 1996 walls take t_m as thickness (not as width of a Block). Pure rule cases without any number (19 Symbols in all) or, when the clause is about a physical element, its element (roof Plate, collision Column, crowd Plate); pure-factor cases are a Gauge of the factor.

## 4. EN 1997: all 71 catalogue entries in 3D
The bespoke `Body`/`keys` code of the en1997 case scene is gone: `🎬️scene/🗂️cases` now draws through `forms::draw::<En1997Cases>`; `Out::push_entity` (kit), packed default origins (`forms::origins`, no slot overlap), `handles::cases` = `forms::handles`, kit `rows_of` filter = `cases::declared`. Soil layers come from the series inputs (`h1`, `p1_h1`), the water table from z_w/delta_h/gwl_level, wall geometry from t_stem/t_base/a_toe/b, slope from h/beta_deg or n_slope with t_soft, anchors from l_f/d/theta_deg/t_w/l_free, wells from r_well/h_well/h_aquifer. Scene tests rewritten (footing, pile, wall, slope, pile group, nominal Symbol, handle tags, placement, case + dimension laws).
Entries whose declared driving inputs have no demo value (method-multiplexed or unset, drawn nominal, no grip until set): SLOPE-01 (h, gwl_level), SLOPE-01B/-02 (gwl_level), SLOPE-03 (z), SLS-01B (b), HYD-02 (d_embed), DEW-01 (r_influence); EN 1992 P-01 (opening_w/a); EN 1990 E90.27 (span). Not drawn: the slip circle of SLOPE-0x (centre/radius are entity inputs), seepage flow lines.

## 5. Dimension laws
`test_support::assert_case_dimension_laws::<Catalogue>(&rows)`: for every case row with geometry and every driving input it sets (width, length, height, aux roles, load) the input is scaled by 1.25; length roles must change one scalar of the drawn solids (box/cylinder/wedge sizes, frame offsets, half extent, top: `forms::signature`) by exactly 1.25, counts/angles/ratios and the edge-relative roles (`aux_is_scalar`: toe, water of walls, prop, well level) must change the signature, the load bar must scale by 1.25; clamped values (under 1 cm, over 40 m, bar beyond 0.1..6 m) are skipped, and at least one input must have been driven. Called from the case law tests of EN 1990, 1991, 1992, 1995, 1996, 1997 and from the forms unit tests (all 14 new forms plus the old ones). The catalogue tests also assert that every aux role belongs to `forms::aux_roles(form)` and every key (`CaseGeometry::keys`) is a declared input.

## 6. Open / risks
- Nothing compiled. Expect small type errors in the new forms code (const-fn byte-string patterns in `aux_is_scalar`, frame axes of the weld bead and ground anchor, `Wedge` orientation: legs along x and z, extruded along y from the frame origin).
- EN 1995 keeps its own case scene (groups, notches, plates); the Fastener/Opening declarations feed the registry and the dimension law, not that drawing. EN 1995 v6 owner: switching to `forms::draw` would give T-71..T-83 the fastener row.
- Demo cases are only perturbed when they carry the driving input; the first run decides whether a form builder clamps a demo value (retaining-wall toe/base, opening hole 90 percent) so the law fails; fix the builder or move the role to `aux_is_scalar`.
- Goldens of the case scenes (all codes) change with the new forms; EN 1997 scene golden (empty file) and packs still need regeneration after the rebuild.
- Series layers: only `h1`..`h8` style inputs; `PILE-05` uses `h1`, `PILE-11` `p1_h1`, `SLS-01` `h1`, `G-01` `h1`.
