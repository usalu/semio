# Scene report: excel / energy (`Beat4_Energy`)

File: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`. Storyboard: `storyboard-excel-energy.md` (superseded by the clause list below where they differ). Audit: `audit-excel-energy.md` (all blocker and major items addressed, see below). Helper scripts in `excel-energy/`; frames in `🗑️generated/frames/📊️excel/Beat4_Energy/{en,de}/`.

## What was built

One scene, 23 clauses, built from the XL engine workbook (`workbook(language=...)`, `evaluate`, `apply`, `dependents`, `calculate`), the Nakagin model (`capsule_type`, `slots`, `value`, `variant`, `climate`) and the toolkit (`xl_window`, `xl_ref_frame`, `xl_trace`, `xl_bar_chart`, `xl_formula_text`, `capsule_axon`, `tower_axon`).

Where numbers come from: every displayed number is an engine value (`evaluate`) or a model value. The what-if scenarios are the named model variants `window_upgrade` (window U 1.1), `back_wall_exposed` (b 1.0) and `shared_roof_floor` (roof and floor U 0): `variant(name)["overrides"]` gives the values, the workbook's defined names (`u_window`, `attached_factor`, `u_roof`, `u_floor`) give the cells, and `apply` set-cell events produce the changed workbook. The scene types no scenario number. Not done: the variants are not bound inside the workbook JSON itself (no cells were added or edited there); they are applied as events on the unmodified workbook. The earlier claim that "no number is typed" was false until this change (1.1, 1.0, 0.0 were typed).

Live mechanism: every numeric cell of each window is bound to a blended workbook state (`vs.a`, `vs.b`, `vs.t`); a change is evaluated once by the engine and a ValueTracker blends old and new values (the chain is affine, so the blend is exact). Cells, heat-flow arrows, readouts, chain panel and chart follow together. `window.recalculate` points the window at the new workbook so the formula bar follows (what-if, frozen, repaired, mistake, fix).

## Clauses

1 hook (tower, type A lit, teaching-model note). 2 sheet (Type A window and capsule). 3 dims. 4 areas (surfaces framed in face colours, faces tinted). 5 exposed (back view with bolts and hatch, b with basis chip). 6 uvalues (U column, heat tint with legend, window chip "benchmark"). 7 ua (arrows, length and width grow with U·A). 8 ht: H_T = `SUM(F3:F9)` as in the workbook and `SUMPRODUCT(D3:D9,E3:E9,G3:G9)` over all seven surfaces with the temperature factor column G (1 for six exposed surfaces, b for the back wall, 1.88 W/K), both 30.92 W/K from the engine. 9 climate (moved before the mistake and the H_V clause): Climate window with all defined names and live values, source and period (JMA normals 1991-2020) and the HDD18 method from `climate()`. 10 mistake: Climate!B6 changes to 1.00 in the window, back arrow grows, delta H_T. 11 fix: B6 back. 12 volume: H_V formula with names, own blue air arrow. 13 total: H_T and H_V cells and stacked bar. 14 load: arrows in W including the air arrow (H_V times delta theta), so the arrows sum to Φ. 15 annual: formula, unit chain, floor plan. 16 energy: Energy window, tower tinted by type, counts. 17 compare: window cols D:F with F4:F7 selected (the column the chart shows), type badges aligned to the rows, bar chart with A B C D labels. 18 shape: five faces plus window against shared roof and floor (variant), shell per floor area and kWh/(m²·a). 19 trace: dependents of the window U (engine closure, 39 cells), arrows, two-line chain panel. 20 whatif: window U 2.33 to 1.10 with every cell, arrow, panel value and total following; formula bar follows. 21 frozen: building total as a typed constant, D8 selected, stale value, dashed frame, true row sum. 22 repaired: D8 selected, formula `=SUM(D4:D7)` back, values match. 23 recap: legend with symbol, name and unit including b, n, capsule count and SUMPRODUCT.

## Audit items

Fixed: ht blocker (narration and card now say what the formula sums); Climate `###` (window width); names before values (climate clause moved before use); chain panel collisions (two-line rows, narrower labels); formula bar follows what-if, frozen, repaired, mistake, fix; compare selects F and labels A-D; Climate!B6 shown in mistake and fix; formula returns to D8 in repaired; "teaching model" note in the hook; H_V·Δθ drawn as its own arrow; ua arrows scale with U·A; climate source and period shown; recap legend extended; heat tint legend; shape labels; German wording (H_T und H_V zusammen, würden den Bedarf deutlich senken, Richtwert).
Not changed (toolkit): the status-bar sum is unformatted; formula-edit overlays extend beyond the window; dashed sheet-icon trace arrow starts over the B4 count and the blue trace arrow crosses the per-capsule values (as Excel draws it).
Open (data, not scene): the twelve monthly means are the model's JMA-normals values taken through the Wikipedia climate table; they were not re-verified against JMA here.

## Keyframe review

All 23 keys in en and de were read (1280x720) in the final capture. Strict layout: en 112, de 130 findings; none off frame except the small grey note line of the ht clause which sits 8 px from the edge (flagged "near frame edge", readable). The remaining findings are by design of the toolkit window (grid lines crossing cell text, reference frames on grid lines, formula-edit overlays, opaque overlays) and trace arrows crossing cells.

## Test result

See the appended section below (`python -m unittest X/🧪️tests/🎭️presentation/🐍️.py`).

## Quality round (final)

24 clauses (new: `degreedays`, between climate and mistake). Added: degree-day chart (twelve Tokyo monthly means from `climate()` against the 18 °C limit, shortfall hatched, sum from the engine 1,586 K·d); interior volume box with V and n; annual demand as multiplying tiles (H x degree days x 24 h/d, then divided by floor area); compare clause explains that A, B and D share one shell and fittings do not change the loss, only C differs by its 1.6 m window (type row drawn from the model); the b mistake shows the Type A formula `=D9*E9*attached_factor` next to Climate!B6; arrow lengths and widths proportional to U·A with a 1 W/K key (hidden faces start outside the body on a dashed lead); after the what-if a small tower with the live building total and percent change (182,901 to 173,115 kWh/a, -5.4 %, from the engine); hook shows the tower with the temperatures and the teaching-model note; recap shows the tower with the result chips (Type A 1,296 to 1,233 kWh/a, 148 to 141 kWh/(m²·a), building 182,901 to 173,115). Status-bar sums use the cell format (toolkit edit in `🎨️visuals/📗️workbook/🐍️.py`); parentheses in stage text use the serif face.
Final captures: en 114, de 131 strict findings; the only near-edge flag is the grey G note of the ht clause (8 px). `python -m unittest X/🧪️tests/🎭️presentation/🐍️.py`: 15 tests, OK.
Note: the reviewer's "-9 %" is not what the engine gives; the live counter shows -5.4 %.
