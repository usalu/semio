# Audit: excel / energy (`Beat4_Energy`)

Read-only audit. Inputs: `audit-brief.md`, `plan.md` (row `excel_energy`), `agent-rules.md`, `scene-excel-energy.md`, `storyboard-excel-energy.md` (not re-read in full, the scene report supersedes it where they differ), scene file `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`, engine workbook `📊️excel/🎬️clip/🧱️model/🔣️.json`, Nakagin data `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json`, research files. Frames: all 23 keys viewed in `en/` and `de/` (1280x720). Nothing edited except this report.

## Summary verdict

Not publishable yet. The physics chain is right (every on-screen number reproduces from the engine and from an independent recomputation), but one narration clause is contradicted by its own frame (SUMPRODUCT "gives the same result", it gives 29.05 of 30.92 W/K), the air heat capacity cell shows "###", several labels collide (chain panel, compare x-axis, shape captions), the formula bar goes stale during the what-if, and the mistake/fix/repair clips do not show the cells they talk about. Degree-day input is unverified against JMA and conflicts with the research range.

Counts: 1 blocker, 13 major and 13 minor frame rows (several cover both languages), plus code, fact and didactic items below. Known-fixed items are listed separately.

## Frame defects

Severity: B = blocker, M = major, m = minor. "EN" frames still show pre-fix bracket rendering, so they must be recaptured after all fixes.

| frame/key | lang | sev | defect | fix suggestion |
|---|---|---|---|---|
| ht | en, de | B | Narration says SUMPRODUCT "gives the same result straight from areas and U-values" (DE: "liefert dasselbe"). The card on the same frame shows `SUMPRODUCT(D3:D8,E3:E8) = 29.05 W/K` plus `attached_factor*D9*E9 = 1.88 W/K`; the sum of the card is 30.92 but SUMPRODUCT alone is not H_T. The workbook itself uses `=SUM(F3:F9)`. | EN: "SUMPRODUCT gives the exposed faces, 29.05 W/K. The attached back wall adds 1.88 W/K, and the total is H_T, 30.92 W/K." DE: "SUMMENPRODUKT liefert für die freien Flächen 29,05 W/K; die angebundene Rückwand kommt mit 1,88 W/K dazu." |
| climate | en, de | M | Cell B8 (air heat capacity, 0.34 Wh/(m³·K)) shows `###`. Column B of the A:B+ set is too narrow for the format. The scene report says `###` was fixed; it was not for this window. | Widen column B on the Climate window or shorten the format; re-check both languages. Consider also showing the value in the volume clip, since it is only legible here. |
| volume | en, de | M | The air-heat-capacity value (0.34) is referenced in the formula (`=air_heat_capacity*air_change*B10`) before the Climate clip introduces the names and before the value is readable (see climate `###`). | Move the climate clip before volume, or show the names with values in the volume clip. |
| trace | en | M | Chain panel: labels run into values ("Type A · window U2.33 W/[m²·K]", "Type A · U·A window3.09"). Label width 1.8 is too narrow. | Widen the panel or the label column; right-align values with a gutter. |
| trace | de | M | Same collision, worse: "Gebäudesumme182.901 kWh/a", "Typ A · Fenster U2,33 W/(m²·K)". | As above; shorten "Gebäudesumme" label or widen. |
| whatif | en, de | M | Formula bar still shows `2.33` while B14 displays 1.10. The formula bar is static text, not the cell's current content. | Bind the formula bar to the live value of the selected cell (or show "1.10" as the typed input). |
| frozen | en, de | M | Same stale formula bar (`2.33`) while the frame talks about D8. D8 is not selected, so the reader cannot see that D8 is a typed constant. | Select D8 and show its content (182900.76 typed) in the formula bar. |
| repaired | en, de | M | The formula is never shown being put back. The caption says "Put the formula back" but D8 is never selected and its formula `=SUM(D4:D7)` never appears. Stage is mostly empty. | Select Energy!D8, show `=SUM(D4:D7)` in the formula bar, then let the value update. |
| compare | en, de | M | Excel selection is D4:D7 (Total kWh/a) while the chart shows kWh/(m²·a) (column F). Selection and chart do not match. | `self.win.select("F4:F7")` (the per-area column). |
| compare | en, de | M | x-axis labels collide: "Type ABType BType CType D" (en), "Typ ABTyp BTyp CTyp D" (de). | Use "A B C D" with the chart title, enlarge chart width, or rotate labels. |
| fix | en, de | M | Caption: the factor "lives in one cell on the Climate sheet", but the frame stays on Type A and never shows Climate!B6. The viewer cannot see where the fix goes. | Briefly switch to the Climate window showing B6 (highlight), then back to the Type A arrow update. |
| mistake | en, de | M | Same gap: b is set to 1 but the cell is not shown; "b = 1.00" is a floating readout with no cell. | Show Climate!B6 changing in the window (or a reference frame on it). |
| hook | en, de | M | No "Teaching model · simplified, rounded" statement on the first frame, although the Nakagin tower is shown as the real building. The note first appears at `sheet`. Quality bar requires it once per clip, and the hook is where the claim is made. | Add the `_note()` element to `hook` (or say "teaching model" in the hook caption). |
| load | en, de | M | Heat-flow arrows sum to about 619 W (100+38+120+120+62+59+120) but Φ = 681 W. The air part (62 W, H_V·Δθ) is not drawn. Reader cannot reconcile the arrows with the readout. | Draw the ventilation pair (existing `_vent`) with a "62 W" label, or label "+ 62 W air" next to Φ. |
| shape | en, de | m | Count mismatch: "5 exposed faces" / "5 Außenflächen" next to 5.3 m²/m², but the 5.3 includes the window and the drawing has six surfaces (roof, floor, left, right, front, window). The label "roof, floor shared" shows only the hatched roof in the axon. | Label "5 exposed faces + window" or count the window; hatch the floor too or reword to "roof shared". |
| shape | en, de | m | Labels "5 exposed faces" and "roof, floor shared" (de: "5 Außenflächen", "Dach, Boden geteilt") sit on one line with no gap; right label reaches x≈1268 of 1280. | Stack them, or move the right label under its capsule. |
| sheet, dims, areas, exposed, uvalues, ua, ht, mistake, fix, volume, total, climate, load, annual, energy, compare, shape, trace, whatif, frozen, repaired, recap | en, de | m | Excel status bar sums are unformatted and mix units: "30.92126861" (ua, ht, total), "8.725" (exposed, area + U + U·A), "1443.73613" (annual, kWh/a + kWh/(m²·a)), "188267.7853" (energy). Real Excel formats the status sum with the cell format. Teaches a wrong habit. | Format the status-bar sum with the cell format, or hide it in the toolkit window. |
| ua | en, de | m | Arrow length = (HIDDEN + 0.08 + 0.13·U·A)·grow and width = 2 + 0.9·U·A: not proportional. Caption says "as long and thick as the U·A". Ratio 6.00 vs 1.88 is about 2.7 in length, 3.2 in U·A. | Make length proportional to U·A (scale only), or change caption to "longer and thicker for larger U·A". |
| volume | en, de | m | Air arrows are two opposite arrows that read as a double-headed "↕"; "cold air in, warm air out" is not legible. | Draw inflow (bottom) and outflow (top) with labels "in" / "out" or drop the caption's in/out claim. |
| volume, load, annual | en, de | m | Formula edit overlays extend beyond the Excel window into the stage (to x≈805, 722, 885) and cover drawing area. | Clip the overlay to the window width or shorten the edit text. |
| mistake | en, de | m | "3.75 W/K" label starts at x≈697, touching the Excel window edge at x≈690. | Shift label left or inwards. |
| compare | en | m | Unit label "kWh/[m²·a]" touches the Excel window edge at x≈915. | Shift chart right by a few px. |
| trace | en, de | m | Dashed sheet-icon arrow sits on the Type A count "36" (reads as "3…"). Blue trace arrow strikes through the per-capsule values in column C. Known cosmetic per scene report. | Start the sheet-icon arrow at the cell edge; route the dependent arrow beside the values. |
| whatif | en, de | m | Label "1.46 W/K" touches the capsule window arrow. | Offset label. |
| uvalues | en, de | m | Heat tint (green to red) has no legend. "Glass leaks most" is carried only by the red ring. | Small colour bar, or drop the tint and keep ring + chip. |
| recap | en, de | m | Legend has no row for b (attached factor), n (air change rate) or capsule count; caption says "mal Kapselzahl" / "times the capsule count" without a term. Legend has SUMPRODUCT, caption does not mention it. | Add b and n rows, or align caption and legend. |
| repaired | en, de | m | Stage almost empty (chain panel only). | Keep the tower or a small window of Energy!D8 for the repair. |

Known, already fixed, listed only: brackets `( )` rendered as `[ ]` in EN (`=SUM[F3:F9]`, `kWh/[m²·a]`, `W/[m²·K]`, `=[B11+B12]*…`; DE already shows parentheses, so EN frames predate the fix and must be recaptured); window fades on unselected cells; underscores in names and captions (`H_T`, `attached_factor`, `theta_i`); sheet tab labels.

## Fact findings

Recomputed independently (plain Python, same formulas as the sheet) and checked against the engine (`evaluate`, `dependents`):

| quantity | on screen | recomputed | verdict |
|---|---|---|---|
| Surfaces A (m²) roof, floor, left, right, front, window, back | 10.0, 10.0, 10.0, 10.0, 4.9, 1.3, 6.3 | 10.00, 10.00, 10.00, 10.00, 4.92, 1.33, 6.25 | OK (6.25 shown as 6.3 by rounding) |
| U·A roof, floor, walls, front, window, back | 5.00, 6.00, 6.00, 6.00, 2.95, 3.09, 1.88 | 5.00, 6.00, 6.00, 6.00, 2.95, 3.09, 1.875 | OK; back wall factor b = 0.5 applied |
| H_T = Σ U·A | 30.92 W/K | 30.921 | OK |
| SUMPRODUCT card (exposed faces incl. window) | 29.05 W/K | 29.046 | OK value, but narration claims it equals H_T (blocker above) |
| H_V = 0.34·n·V, V = 18.35 m³ (interior), n = 0.5 1/h | 3.12 W/K | 3.120 | OK; units Wh/(m³·K)·1/h·m³ = W/K |
| H = H_T + H_V | 34.04 | 34.041 | OK |
| Φ = H·(θi − θe), θi 20, θe 0 | 681 W | 680.8 W | OK |
| HDD18 (Climate B5, Σ months (18 − mean)·days) | 1,586 K·d (1585.7) | 1585.7 | arithmetic OK; INPUT UNVERIFIED (see below) |
| Q = H·HDD·24/1000 | 1,296 kWh/a | 1,295.5 | OK; units W/K·K·d·h/d ÷ 1000 = kWh ✓ |
| Q/A (net floor 8.74 m²) A, B, D; C | 148; 153 | 148.2; 153.4 | OK |
| Building total (140 capsules, 36/35/34/35) | 182,901 kWh/a | 182,900.8 | OK; counts sum to 140 ✓ |
| Shape: exposed shell per floor (6 surfaces incl. window) | 5.3 m²/m² | 5.29 | OK (count label, see shape) |
| Shape: roof + floor set to U = 0 | 3.0 m²/m², 100 kWh/(m²·a) | 3.00; 100.3 | OK |
| Mistake b = 1 | ΔH_T +1.88; back arrow 3.75 | +1.875; 3.75 | OK |
| What-if U_window 1.10 | window U·A 1.46; H_T 29.29; Q 1,233; total 173,115 | 1.460; 29.29; 1,233.4; 173,115 | OK (engine) |
| Dependents of window U | 39 | 39 (engine `dependents` closure) | OK |
| Load arrows (W) | 100, 38, 120, 120, 62, 59, 120 | 100, 37.5, 120, 120, 61.9, 59.1, 120 | OK per face; ventilation 62 W not drawn (see load) |

Degree days: the 1585.7 K·d reproduces from the twelve monthly means in `🧱️model` / `🏢️nakagin` (Jan 5.4 … Dec 7.7). I could not verify those means: Wikipedia and JMA pages fetched in this session did not return the climate table. `research-nakagin.md` §4.4 gives "about 1,000 to 1,300 K·d" as low-confidence and says to compute from JMA daily data before publishing. Note that the monthly-mean method is a lower estimate for a convex function (Jensen), so the daily-data value is likely ≥ 1,586 if the means are right; the research range looks low, not the method. Action: verify the 12 means against JMA normals 1991–2020 (Kitanomaru, station 47662) and show source and period on the Climate sheet. The sheet currently shows only "Climate Tokyo".

Other facts:
- Nakagin geometry: 2.5 × 4.0 × 2.5 m outer, 1.3 m round window, four bolts, 140 capsules. Consistent with `🏢️nakagin/🔣️.json` and the fetched Wikipedia text. The fetched Wikipedia text is contradictory on bolting ("not bolted to a central core" vs "by only four high-tension bolts … cantilevered from the shaft"). The scene's "bolted to the core" matches the shaft attachment; keep, or say "attached to the central shaft by four bolts".
- Window U 2.33 is labelled "sourced" and the chip reads "window sourced" / "Fenster belegt". It is a 2019 Tokyo guideline example value, a benchmark, not a measured value for this window. Suggest chip "benchmark" / "Richtwert".
- Back-wall factor b = 0.5 "assumed" (temperature reduction towards the core) is consistent with the JSON and research; OK as a teaching assumption.
- Energy physics clarity: H_T uses outer-dimension envelope areas, H_V and the floor area use interior dimensions (18.35 m³, 8.74 m²) while the floor U·A uses 10 m². Acceptable for a teaching model, but not stated on screen.
- Two temperature bases: the load uses θi − θe = 20 K, the annual demand uses HDD base 18 °C. Not explained; one line would do (heating limit below setpoint because of internal and solar gains).
- Type D: same numbers as A and B (148). The Type D sheet subtitle says "side window, turned 5 degrees", which the calculation does not model. The clip does not say so.
- Scene vs quality bar: the scene report says "No number is typed in the scene", but the scene types `self.u1 = 1.1` (line 290), `edit(... "B6", 1.0)` (line 295) and `edit(... "B11"/"B12", 0.0)` (line 296). These what-if scenarios should come from `🧱️model/🔣️.json` or the nakagin energy inputs. Also the formula strings in the scene (`=air_heat_capacity*air_change*B10`, `=(B11+B12)*…`) duplicate workbook formulas and can drift.

German fact and language checks:
- Excel UI names in DE are correct: Datei, Start, Einfügen, Seitenlayout, Formeln, Daten, Freigeben; status bar Bereit, Anzahl, Summe, Mittelwert. Functions SUMME, SUMMENPRODUKT; separator `;` in SUMMENPRODUKT(D3:D8;E3:E8). Decimal comma and thousand point (1.296 kWh/a) correct.
- German wording: "H_T plus H_V" in total caption (English word, DE: "H_T und H_V zusammen"); shape caption "sparten viel" (DE: "würden den Bedarf deutlich senken"); chip "belegt" (DE: "Quelle belegt" or "gesichert"); "Summe der Produkte" is correct. Defined names (`theta_i`, `heating_degree_days`) stay English on the stage. Acceptable, but say so once.

## Didactic findings

- Arc holds: sheet, dims, areas, exposed, U-values, U·A, H_T, typical mistake and fix, H_V, total, climate, load, annual, energy, compare, shape, trace, what-if, typical mistake (constant), repair, recap. Every key is a single visible change, captions are at most two lines, and both languages carry the same structure.
- Typical mistake 1 (b = 1) is shown and fixed, but the cell is never shown (fix, mistake). Typical mistake 2 (typed constant) is shown well (dashed D8, "rows add up to"), but the repair is not shown (repaired).
- Names used before they are explained: `air_heat_capacity`, `air_change` appear in the volume formula; their values appear in the climate clip, where the cell is `###`.
- The "hook" lacks the teaching-model statement (see above).
- Plan gap: the plan row says the what-if "updates every dependent cell and the hatched surfaces". The what-if updates the window arrow and cells, but no hatched surface changes. Either add the window-hatch change or amend the plan.
- Excel's Sum in the status bar teaches the wrong habit: it adds area, U-value and U·A (exposed, 8.725) and kWh/a with kWh/(m²·a) (annual). Remove or format.
- Arrow length/width is not to scale (see ua); the "as long and thick as U·A" promise is only approximately kept.
- Recap legend and narration disagree (b, n, capsule count, SUMPRODUCT).
- Type C is the only difference the compare clip explains; Type D is identical to A and B without saying why.

## Test and tooling notes

- No files were created or modified except this report. The engine was imported read-only to count dependents (39) and recompute values; no `__pycache__` was newly created that I could find.
- Layout report (`🗑️generated/frames/…/en/layout.md`) lists 129 strict findings (en) and 140 (de); most are by design of the toolkit window (grid lines through cell text, reference frames on grid lines). I did not re-triage them; the findings above are the visible ones.
- Recapture EN after the bracket fix, and rerun the layout check after the label and width changes.
