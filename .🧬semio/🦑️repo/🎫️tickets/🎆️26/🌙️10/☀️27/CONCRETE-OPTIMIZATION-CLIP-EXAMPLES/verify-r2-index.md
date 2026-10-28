# Verify R2: Beat4_NormalizedIndex (⚖️index)

**Verdict: FAIL**

Defects: 1 blocker, 4 major, 15 minor.

Scope: all 42 English and all 42 German frames read one by one (keys from `NARRATION` in `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py`), zoomed crops in `🗑️generated/verify-index/`, the layout reports, `check-r2-index-scene.py` and `check-r2-index.py` (outputs `🗑️generated/verify-index/scene-check.out` and `check-index.out`), the scene code, `model.md` and `audit-r2-index.md`. Frames (22:20 to 22:30) are newer than the scene file (22:19), so no re-capture was needed. The missing persistent title is ignored as instructed.

## 1. Defects

| Key | Severity | Where | Defect | Concrete fix |
| --- | --- | --- | --- | --- |
| B1 | blocker | EN and DE: room chips on most frames (`value_strip`, lower right); tables and plots `criteria`, `criteria_2`, `best`, `matrix`, `weights`, `weights_2`, `weights_3`, `record`, `limits`, `limits_2`, `pointer` | Cost (CHF) and embodied carbon (kg CO₂e) are printed with no physical meaning on screen in these frames. The room shows only a chip such as "19,428 CHF" or "2,520 kg CO₂e". Their parts (fixed 9,000, wall, insulation, windows, overhang; masonry, insulation, glass, overhang) are drawn only in `cost`, `cost_2`, `carbon` and in the Premium build-up of `range`/`range_2`. Electricity has the same gap in `criteria`, `best`, `matrix`, `weights*`: the room shows lamps and arrows, not the heating/cooling/lighting split. This breaks the owner rule "values always visualized at the same time by their meaning". | Add a compact build-up strip to the persistent room panel of the design on show: five CHF parts from `block_cost` and four kg layers from `block_carbon` (same colours), plus the three kWh parts. Keep it on every frame, so each printed cost, carbon or kWh value has its parts beside it. Then the chips no longer float without meaning. |
| M1 | major | EN `range_2` (narration l. 645, `screen_range` l. 1479–1480); DE `range_2` | Narration "Best and worst move, so all other normalized values shrink" is false as drawn. On the cost ruler the best (Punched, 0.00) does not move; only the worst moves (Shaded to Premium). The electricity best does move (Premium 10.41 is below Shaded 10.44, printed only as the room chip in `range` to `range_3`), but no electricity ruler is shown. The electricity change is silent (old D17 still open). | Either draw the electricity ruler for six designs in `range_2` (Premium 0.00, Shaded 0.01, Ribbon 0.11, Super 0.64, Punched 0.71, Bare 1.00 with 10.41 and 15.11) and narrate it, or change the claim to "The worst moves to Premium; all other normalized values shrink" (DE "Der schlechteste Wert wandert zur Premium-Glaswand; alle anderen normierten Werte schrumpfen."). Test the sentence against the ruler values. |
| M2 | major | EN and DE `ruler_4` (header l. 1219; heat arrows by `demand_arrows`, l. 772–788); EN and DE `next` (`screen_answer`, l. 1581) | The winter heat-loss arrows are red and leave through the right (rear) wall of the room section, while the header says "heat leaves through wall and window". The façade (wall U 0.19, window U 0.9, the only surfaces with those values) has no arrows. Cooling arrows (teal) also leave the rear wall. In `next` the winner prints "Heating 183 kWh/a ÷ COP 3.5 = 52" and the room shows only teal arrows, so the heating value has no drawn meaning on that frame. | Draw the heat-loss arrows from the façade wall and window (count bound to heating demand), keep the cooling arrows on the façade inflow or label them. In `next` add the winter arrows (or an inset of the winter section) so that heating 52 kWh/a has its meaning in the same frame. |
| M3 | major | Scene l. 33 `BUDGET_CHF = 21000`; used at l. 1595 and 1607 (`screen_limits`); EN and DE `limits`, `limits_2` | The 21,000 CHF budget is typed in the scene, not in `🧮️model/🔣️.json` (only `budget: 50` for the algorithms exists there). The owner rule is "every number on screen comes from the model functions, never typed by hand". The narration says "The client's budget is 21,000 CHF". | Add `constraints.facade_cost_cap_chf` with bilingual `note` to `🔣️.json` and its schema, read it through a model function, delete `BUDGET_CHF`, and add a test that the cap excludes exactly Shaded and Bare. |
| M4 | major | Depth vs energy clips; `next`, `ruler_4`, `ruler`, `daylight`, footer (`prepare`) | Below the energy-clip bar in three ways. (a) Heating demand 354 kWh/a (Punched), 136 (Shaded), 183 (Ribbon) is printed without derivation from U values, degree hours or gains. (b) The energy clips cite their standard (DIN V 18599-5, DIN 4108-2, VDI 2078) in the corner or caption; the index clip has one generic footer "EN ISO 13790 style". (c) Daylight is shown with direct sun rays (see m5) and no diffuse-sky picture. | For each criterion add a source line in the energy-clip style (the model's own standard basis) and a one-line derivation of the demand printed on the ruler frame (e.g. "heating demand = Σ U·A·ΔT − gains·utilization = 354 kWh/a" bound to the model terms). Use a diffuse sky dome for daylight (see m5). |
| m1 | minor | `screen_rulers` `step_note` (l. 1179); EN and DE `shift_2` | Chip "÷ 4.668 kWh/(m²·a)" (DE "÷ 4,668"). The printed endpoints are 15.106 and 10.437, and 15.106 − 10.437 = 4.669. The exact span 4.66875 rounds to 4.669. | Print 4.669 (use the same rounded endpoints for the span). Extend `check-r2-index.py` to check the span from the printed endpoints. |
| m2 | minor | EN and DE `shift`, `shift_2` | Chip "- 10.437 kWh/[m²·a]" (DE "- 10,437") uses hyphen-minus. The ticks "+2 +4 +6" have no unit. (Old D14/D15 open.) | Use the true minus "−10.437 kWh/(m²·a)" and put the unit on the first tick "+2 kWh/(m²·a)". |
| m3 | minor | Unit formatter, every frame (EN and DE), e.g. `block_electricity` l. 892, `cost` chip "CHF/[m²·cm]" | Units are written "kWh/[m²·a]", "CHF/[m²·cm]", "kg CO₂e/[m²·cm]" on every screen. The model unit is "kWh/(m²·a)". (Old D15 open.) | Change the unit formatter to parentheses, e.g. "kWh/(m²·a)" and "CHF/(m²·cm)", and update the check script. |
| m4 | minor | Dimension "d x m" in `build_panel` `dimension()` (l. 758–771); EN and DE `ruler`, `ruler_2`, `shift`, `shift_2`, `cost_2`, `carbon`, `index_2`, `index_3`, `winner`, `winner_2`, `sensitivity`, `sensitivity_2`, `sensitivity_3`, `range`, `range_2`, `range_3`, `record`, `pointer`, `next` | Sun rays are drawn over the label "d 1.2 m" / "d 0.6 m" (the "1.2" and "0.6" are crossed). In `range_3` the Ribbon/Fensterband row frame (right edge x≈717) runs into the "d" label. | Put a background plate behind the dimension label (z-order above the rays), or move the dimension under the overhang. Keep the row frame clear of the room panel (stop it at x≈700). |
| m5 | minor | `block_daylight` and the room sun in EN and DE `daylight`, `criteria_2` | The daylight factor is a diffuse-sky metric (DF uses the sky angle θ), but the room draws direct sun rays through the window. The sky angle and lamps are bound, the sun rays are not. (Old D02 partly open.) | Draw the sky dome (θ) and the 2 % isoline in the section instead of beam rays, or label the rays "direct sun, not in DF". |
| m6 | minor | `block_daylight` l. 981 "Daylit (≥ 2 %) to 4.9 m of 6 m depth" (EN and DE `daylight`) | The number comes from the toolkit `daylit_depth` (linear falloff from the window to zero at the back wall), not from the model. The assumption is not stated on screen. | Derive it from a model function, or print "linear falloff assumed" beside the number. |
| m7 | minor | EN and DE `sensitivity_2`, `sensitivity_3`, `range_3`; `sensitivity_2` "was 1st" tag | Unlabelled marks (old D24 open): grey dashed rank frame with bracket ticks at x≈716 (sensitivity_2 and _3); orange double arrow at x≈590, y≈215–255 in `range_3` with no label (only the footnote "[orange: changed]"). The "was 1st" tag floats between the bars and the index (x≈472–555), not on the row. | Label the double arrow "rank change" in place; remove the bracket ticks or label them; put the "was 1st" tag next to row 2's rank number. |
| m8 | minor | `weights`, `weights_2`, `weights_3`, `matrix`, `sensitivity` (EN and DE) | Column chips are not centred on their strip segments (old D21 open). In `weights_2` the offsets are about −46, −72, −61, −11 px (Electricity chip centre ≈269 vs segment ≈315; Cost ≈375 vs ≈447). The "10 %" label sits outside the strip while the other values sit inside. | Centre each chip on its segment (or the column in `matrix`) and put the "10 %" inside the carbon segment or label all four outside consistently. |
| m9 | minor | Caption band, EN and DE (`present` captions) | Single-word second lines (orphans) in many frames. DE: `units` "Einheit.", `ruler_2` "bedeutet.", `ruler_3` "Effizienz.", `ruler_5` "Strombedarf.", `cost` "wird null.", `cost_2` "und Vordach.", `carbon` "je Schicht.", `index_3` "Index.", `range_2` "schrumpfen.", `range_3` "blieben.", `record` "jeder Entscheidung.", `sweep_2` "entscheidest.". EN: `ruler_4` "heat.", `carbon` "layer.", `index_3` "index.", `record` "decision.", `limits` "weights.". (Old D25/D29 partly open.) | Set explicit line breaks in the caption text (both languages) so that no second line holds one word. |
| m10 | minor | DE terms: `cost` chip and `cost_2` caption; `units_3` caption and legend; `range`, `record` | "Feste Positionen" (cost chip) vs "Festes" (narration `cost_2`); "Emissionen" and "t CO₂e" in `units_3` where "Graue Emissionen" is used everywhere else (old D28 partly open); "Premium-Glaswand" hyphen (EN "Premium glass wall"; old D open); "Tageshell" is an unusual term for "daylit". | Use one DE term per concept: "Fixposten" in the chip and narration; "Graue Emissionen" in `units_3`; "Tageslicht im Raum" or "tageslichtversorgt" for "daylit". |
| m11 | minor | `sweep`, `sweep_2` (EN and DE) | The winner strip at y≈190 is one uniform green line, so the three winner segments (Shaded 0–4 %, Ribbon 4–39 %, Punched 39–100 %) are not distinguishable on the strip. The first top thumbnail (0–4 %) is unlabelled (old D23 open). | Colour each winner segment of the strip in its design colour and label the first thumbnail "Shaded" or move the thumbnail row. |
| m12 | minor | `index`, `index_2` (EN and DE) | The value label "0.70" (Super-insulated) sits on the room ground hatch line (x≈640–690, y≈404–422). | Move the index value column to x≈650 or lift the labels 6 px above the ground line. |
| m13 | minor | `record` (EN and DE) and `next` | In `record` the "Compared variants" labels are staggered (Ribbon and Super-insulated dropped below their thumbnails). In `next` the left column has an empty band (y≈300–400) between the index line and the energy bar. | Put all five labels on one baseline under their thumbnails; move the energy bar up or fill the gap with the index explanation. |
| m14 | minor | `variants`, `variants_2` (EN and DE) | Thumbnails are about 50 px. Punched and Super-insulated punched windows look alike (old D10 open); the wall thickness that separates them is not visible at this size. | Enlarge the thumbnails to at least 80 px or show the insulation thickness in the spec column (already in `variants_2`; add a drawn section tick). |
| m15 | minor | `next` (EN and DE) | "0.55 × 30 % = 0.17" holds only if 0.165 is rounded half-up (exact 0.5452 × 0.30 = 0.1636 gives 0.16). | Print three decimals in the card (0.55 × 30 % = 0.165) or state the rounding. |

Counts: blocker 1, major 4, minor 15 (m1–m15).

## 2. Audit R2 status (`audit-r2-index.md`)

| Defect | Status now |
| --- | --- |
| D01 carbon has no drawn meaning (blocker) | Resolved: `carbon` beat with four layer bars and tinted façade layers (`materials_overlay`). The matrix still shows 2,520 as text (see B1). |
| D02 daylight sun rays, lamps not bound (blocker) | Partly: lamps now dim with autonomy (Bare 18.69 %, Ribbon 30.8 %) and the daylit floor is drawn; direct sun rays remain (m5). |
| D03 no room on 11 frames (blocker) | Room on all 42 frames. Cost and carbon still without meaning in those frames: B1. |
| D04 725 ÷ 48 = 15.11 (major) | Resolved: "725.1 kWh/a ÷ 48 m² = 15.11" reproduces (checker mismatch is a stale checker input, see §4). |
| D05 cap from the winner (major) | Partly: `ceil` rule and `CAP_STEP` removed; budget is authored in the scene, not the model (M3). |
| D06 index arithmetic (major) | Resolved: `index_3` and `next` print the four products. |
| D07 heating and lighting arithmetic, winter arrows (major) | Partly: arithmetic printed (ruler_4, next); winter arrows on the wrong surface, no winter arrows in `next` (M2). |
| D08 cost parts (major) | Resolved: `cost` and `cost_2` stacked build-up with the apportioned values. |
| D09 room too small (major) | Resolved: room about 425 px wide, 12–14 px labels readable. |
| D10 sparse cards (major) | Partly: table of five rows with a room; thumbnails still about 50 px (m14). |
| D11 shift_2 not reproducible (minor) | Resolved for 0.71 (3-decimal endpoints give 0.7057); the span chip is still off (m1). |
| D12 close-call bands (minor) | Resolved: bands 0–10 % and 36–43 % (read at 2.6 px per percent from the crop). |
| D13 feasible set not re-normalized (minor) | Resolved: "normalized over the allowed designs", 0.32 / 0.60 / 0.90. |
| D14 scale labels under baseline (minor) | Resolved: labels removed; `next` prints 524 kWh/a = 10.92. Shift ticks still unitless (m2). |
| D15 notation (minor) | Open: brackets and hyphen-minus (m2, m3). |
| D16 Pareto dominated (minor) | Resolved: grey points "dominated by Ribbon window"; axis "Electricity intensity, kWh/(m²·a)". |
| D17 six-design electricity (minor) | Open: M1. |
| D18 weakest chip (minor) | Resolved: "weakest = highest normalized value: Daylight 0.72". |
| D19 limits leader through "nicht erlaubt" (minor) | Resolved: label sits inside the zone in EN and DE. |
| D20 criteria_2 card wrap (minor) | Resolved: "2.00 %" on one line; no overlap with arrows. |
| D21 chips not centred (minor) | Open: m8. |
| D22 range gutter and stacked ticks (minor) | Resolved: chips stacked; "1" and "1.00" not stacked. |
| D23 sweep chips, unlabelled thumbnail, "rank" (minor) | Partly: "rank" moved; first thumbnail unlabelled (m11). |
| D24 unlabelled marks (minor) | Partly: `weights_3` labelled; sensitivity ticks and range_3 arrow open (m7). |
| D25 secondary text, orphans (minor) | Partly: orphans remain (m9). |
| D26 "1 m" dimension, "*" box (minor) | Resolved: no "1 m" dimension; "*" box removed; overhang depth dimensioned in winner, ruler, index_2, next. |
| D27 DE room-bar verbs (minor) | Resolved: Heizung, Kühlung, Beleuchtung. |
| D28 DE carbon term (minor) | Partly: "Graue Emissionen" in headers and chips; `units_3` still "Emissionen" (m10). |
| D29 DE sweep_2 "Rang" and orphan (minor) | Partly: "Rang" moved; orphan "entscheidest." remains (m9). |
| D30 footer (minor) | Resolved: footer line on every frame, EN and DE. |

## 3. Answers to the seven checks

1. **Physical meaning at the same moment**: fails for cost and carbon (B1); heating arrows on the wrong surface and no heating meaning in `next` (M2).
2. **Numbers, arithmetic, units, claims**: every model-derived number on screen reproduces (`check-r2-index-scene.py`: 79 of 80 checks OK; the one mismatch is the Premium heating demand, which is never shown; `check-r2-index.py`: 160 of 162 OK). Units and weights add to 100 % (40/30/20/10, 23/60/11/6, 25 × 4). Arithmetic holds as printed except m1 and m15. Narrated claims: `range_2` false on screen (M1); "heat leaves through wall and window" drawn on the rear wall (M2); "client's budget" is a stated scenario value (M3).
3. **Overlaps, clipping, legibility**: no clipped text or overlapping labels beyond the minor items (m4, m7, m12, m13). The tiny scale bar is gone. The room is not cluttered except for the sun rays crossing labels.
4. **Narration and visual sync**: mismatches at `range_2` (M1) and `ruler_4` (M2); the "was 1st" tag is detached from its row (m7). All other captions match their frames.
5. **German**: complete (42 of 42), numbers formatted correctly (13,73; 19.428 CHF; 1.500 h/a; 21.000 CHF; 0,36), grammatical noun labels. Remaining: m1 (÷ 4,668), m2 (hyphen and unitless ticks), m9 (orphans), m10 (terms).
6. **Depth vs energy clips**: partly there (every block has written equations, footer on every frame, room on every frame); below the bar in heating derivation, standards and daylight physics (M4).
7. **Audit R2 defects**: see §2. 16 of 30 resolved, 14 partly or open; the cost and carbon meaning part of D01 and D03 is not closed (B1).

## 4. Checker notes (not scene defects)

- `check-r2-index.py` reports "glass_bare printed arithmetic 15.10": it uses the rounded total 725. The screen prints 725.1, and 725.1 ÷ 48 = 15.11 holds. Update the input.
- `check-r2-index.py` reports "shift_2 Punched normalized from printed values 0.70": it uses two-decimal endpoints. The screen prints three decimals and 0.71 holds. Update the input.
- `check-r2-index.py` does not check the span chip (m1); add it.
- `check-r2-index-scene.py` reports "glass_premium heating … 35.6": Premium heating is never printed, so it is not a defect. Exclude or ignore it.

Files: this report `T/verify-r2-index.md`; outputs `T/🗑️generated/verify-index/scene-check.out`, `check-index.out`; zoom crops and two crop scripts in `T/🗑️generated/verify-index/`.
