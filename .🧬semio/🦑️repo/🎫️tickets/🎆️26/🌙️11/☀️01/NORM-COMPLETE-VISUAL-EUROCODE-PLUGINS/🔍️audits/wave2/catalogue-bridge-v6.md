# Catalogue bridge v6 (wave 2)

Status: UNCOMPILED. No cargo, rustc or build was run. Python only (`🧰️scripts/catalogue-bridge-v6/law_check.py`, `demo_gen.py en1997`, `geometry_gen.py --check` green for all six codes). `repo` MCP not connected; work is filed by path. Rebuild lines appended to `build-en1995` and `build-en1997` `latest.md`.

## 1. EN 1995 through the shared forms: NOT done (blocked on scope), gap list for the owner
I did not switch EN 1995 to `forms::draw` and did not delete its case scene. Reasons, read from the code:
- `🎬️scene/🗂️cases/🦀️.rs::project_row` is the renderer of two things: the catalogue cases and the typed physical entities (`🧱️entities`: tapered and apex members, notches, holes, fastener patterns with explicit points, CLT panels with char layers, floors with plate modes and creep surfaces are "drawn as the case row they stand for"). `cases::geo` is also read by `📈️diagrams` and `🕹️handles`. Together `🗂️cases` 366 + `🧩️special` 321 + the case parts of `🕹️handles` (`fastener_pattern_handles`, `case_row_handles`, ~140 lines) + `📍️layout` (`extent`, `ROW_CASES`, `CASE_GAP`) + about 35 scene tests (fire residual and char, floor modes, CLT laminate, explicit fastener grips, minimum-spacing colouring, jointed beam, crowd, creep, golden) depend on it.
- The v7 owner changed these files today (typed child rows, laminate from layer rows, explicit positions, hatch direction), and a half switch would leave both engines. A one-shot rewrite of about 1000 lines plus tests with no compiler is a net regression risk, so I stopped at the analysis. No file of that scene was edited.

What the shared forms lack, as generic form features (with the EN 1995 consumer), in the order I recommend:
| # | generic feature | consumer | notes |
|---|---|---|---|
| 1 | `Part` hatch/grain (`Hatch::Horizontal` along, `Vertical` across the length) on Member, Plate, Block | all timber members | `Appearance.hatch` already exists and v7 added `Vertical`; a form `grain` aux role (empty key = along) |
| 2 | Member profile: taper angle and apex (prism from an elevation polygon, apex zone) | T-31..T-33, typed tapered/apex members | `cases::elevation` is pure geometry, move it to the kit |
| 3 | void/inset parts: notch box, hole cylinder, residual section and char/front bands as inset prisms of the section by per-face depths | T-45 notch, T-18 hole, T-101..T-107 fire | the char depth is a domain computation (`facts::fire_after`): pass the depths in as data (`draw_with(.., &Extras)`), keep `facts` in EN 1995 |
| 4 | Laminate form from series layer rows (thickness, orientation) | T-97, T-107, CLT panels | `SolidShape::Laminate` and `GrainLayer.grain` exist; series input `t1..tn` like the ground `layers` |
| 5 | Fastener pattern: rows x columns with a1/a2, end and edge distance, explicit point list, per-instance colour field and legend | T-70..T-83, typed patterns | extends the v5 Fastener row; the field values (minimum-spacing demand) come from the domain as data |
| 6 | Deformation overlay on a part (first vertical mode, plate heat map) | floors, footbridges (T-60..T-65, T-120) | data from `modes::first_vertical`, memoised in EN 1995 |
| 7 | Stacked parts with slip rows (jointed beam), spaced column, truss, bracing, diaphragm, Miner damage tower, SLT deck, crowd | T-90..T-96, T-120..T-122 | domain illustrations; keep as a small EN 1995 extension that emits `Part`s, not a second engine |
Then: `📍️layout` takes `forms::origins`/`extent`, `🕹️handles` delegates to `forms::handles` and keeps only the typed-entity grips, `cases::geo` moves next to the diagrams that need it, the bespoke `Geo`/`project_row` go, tests move to `forms` tests and the law test (`assert_case_dimension_laws::<En1995Cases>` already runs for the declared rows).
EN 1995 geometry declarations (70 rows) are already complete for the forms; the consumer list above needs the extra aux roles (`grain`, `taper`, `layers`, `rows`, `spacing2`) added to `forms::aux_roles` and to `📐️geometry/🔣️.json`, validated by `geometry_gen.py`.

## 2. EN 1997 SLOPE demo values (done)
Cause: the geometry rows bind `h`, `beta_deg`, `gwl_level` (SLOPE-01/-01B/-02) and `z` (SLOPE-03, `soft`) but the fixture cases did not carry them. Added in `🧰️scripts/demo-values/values_en1997.py` `EXTRA` (not normative):
| entry | input | value | note |
|---|---|---|---|
| SLOPE-01 | `h`, `beta_deg`, `gwl_level` | 6 m, 30 deg, 2 m | the case takes the undrained arc branch (`arc_deg`, `w`, `x_w`), which returns before `h`/`beta_deg`/`gwl_level` are read: the result is unchanged |
| SLOPE-01B, SLOPE-02 | `gwl_level` | 2 m above the toe | enters `slices` (pore force, saturated weight); `gamma_sat` falls back to `gamma`; no error path |
| SLOPE-03 | `z` | 2.5 m | read only together with `c_k`, which the demo does not set: the evaluation is unchanged, the soft layer is drawn |
`demo_gen.py en1997` regenerated `🌱️demo/🔣️.json`, `🌱️demo/🦀️.rs` and the demo document (520 value rows, 514 before; `validate.py en1997`: 0 issues). Dimension law: the slope wedge height follows `h` exactly (wedge height and crest box), `beta_deg` changes the wedge base (scalar role), `gwl_level` moves the water sheet, `z` is the thickness of the first soil layer (box z size, exact). Not touched: SLS-01B `b`, HYD-02 `d_embed`, DEW-01 `r_influence`, EN 1990 E90.27 `span`, EN 1992 P-01 opening (method multiplexed or evaluator-switching, see v5 section 4). SLOPE-03 has no height input, so its crest height stays nominal.

## 3. Clamps against the dimension law (done)
Method: `law_check.py` replays the builder clamps on the six demo documents (483 rows), and each form was read against `assert_case_dimension_laws` and the `full()` toy rows of the forms unit test. Demo documents hit: SLS-01 footing (top-up soil layer appears or disappears when `h1` scales: part count changes), HYD-01/-02/-03 (`upstream` water was `1 + delta_h`, no solid carried `delta_h`), EN 1995 T-74 (spacing without a count draws one fastener, spacing has no effect). The toy rows hit more (all removed):
| form | clamp | fix |
|---|---|---|
| SoilBlock | `water.min(depth)`, `rod.min(depth)`, interface drawn only `if top > -depth` | no clamp; interface at `top.max(-depth)` always drawn |
| Seepage | `embed.min(layer)`; upstream box `base_level + head` | no clamp; `upstream` (base level) plus own `head` sheet of exactly `delta_h` |
| SheetWall | `prop.min(retained)` | none |
| Well | `radius.min(0.5 influence)`, `level.min(thickness)`, `surface.min(thickness)` | none (the `ln` ratio keeps its `1e-6` floor) |
| Opening | holes `.min(0.9 w/l)` | strips keep `MIN_RIM` (2 cm), half extent grows to hole + rims, so the hole always follows its input |
| Anchor | declared height `.max(embed)` | none |
| Footing, Pile | `strata` topped up only when the layers are thinner than the depth | one bottom layer always (`MIN_BOTTOM_LAYER` 0.5 m), part count independent of the inputs; `bottom` is the stack sum |
| Fastener | spacing role with default count 1 | default count 2 when a spacing is declared |
Kept deliberately (scalar roles only change the signature, no exact factor claimed): count clamp 1..12 with rounding, angle clamp 0.05..1.5 rad, retaining-wall `toe` clamp inside the base, base floor `t + 0.2`, `MIN_SIZE`/`MAX_SIZE`. Python twin: 0 findings for all six demo documents after the fixes; counts at 1 or 12 and angles above 69 degrees would not pass the law but occur in no demo row (`law_check.py` reports them).
New forms unit tests: seepage head sheet equals the head, opening wider than the slab, fastener pair; `the_dimensions_of_every_form_follow_their_driving_inputs` covers the removed clamps through its toy rows.

## Files
Edited: `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/{🦀️.rs,🌍️ground/🦀️.rs,🔩️joints/🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; EN 1997 `🌱️demo/{🔣️.json,🦀️.rs}` and `🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json`; ticket: `🧰️scripts/demo-values/values_en1997.py`, new `🧰️scripts/catalogue-bridge-v6/law_check.py`, `📌️important/requests.md`, `build-en1995`/`build-en1997` `latest.md`.
