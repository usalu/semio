# Final verification: Beat3_FactorialExperiments (🧪️experiments) and Beat4_NormalizedIndex (⚖️index)

Read-only verification. No source file was edited. Only this report and generated files under `🗑️generated/final-verify/` were written.

## Verdicts

| Clip | Verdict | Blockers | Major | Minor | Frames visually verified |
| --- | --- | --- | --- | --- | --- |
| 🧪️experiments (`Beat3_FactorialExperiments`) | **FAIL (not certified)** | 0 | 1 open (M2 partial) | 2 | DE 27 of 35; DE 8 and EN 35 not visible in this session |
| ⚖️index (`Beat4_NormalizedIndex`) | **FAIL** | 0 | 3 | 7 | EN 42 of 42; DE 42 of 42 |

Summary: the index clip has three real major defects (a collision on `ruler_4`, a narrated claim that is false on the electricity ruler, and a derived line that does not reproduce from the printed weights). The experiments clip is clean on every DE frame I could see, but its verdict cannot be PASS: the leader-line part of prior major M2 is still open, and 43 of 70 frames could not be read in this session (see Method).

## Method and limits

- Frames read: `T/🗑️generated/frames/<Scene>/<Scene>-<en|de>-<key>.png`, all keys from NARRATION (35 experiments, 42 index). Per-frame reading was done on labelled 2×2 contact sheets built by `T/🗑️generated/final-verify/sheets.py` (`T/🗑️generated/final-verify/sheets/`), not on each full-size PNG and not on zoom crops. Small text (≈11–13 px at 1280) was judged from the sheet, so tiny-text findings are less certain than the others.
- Experiments coverage gap: the Read results for `…-en-*` (all 35) and `…-de-question`, `-question_2`, `-question_3`, `-factors`, `-factors_2`, `-factors_3`, `-plan`, `-plan_2` came back without image content ("media removed: request limit"). Those 43 frames are **not verified** here and no PASS is claimed for them. A re-read is needed.
- Index coverage: all 84 frames came back with content and were read.
- Layout reports: `…-layout.md` files are current (experiments 01:19, index 01:23, scene files 23:48 and 00:22). Experiments: "strict findings 0 in 0 of 35" in both languages. Index: strict finding only on `ruler_4` (EN and DE). The experiments report still shows 0 findings everywhere while visible touches exist elsewhere in prior audits; the report is not a sufficient check (prior M3, still open as a toolkit task).
- Scripts run (outputs in `🗑️generated/final-verify/`):
  - `verify-check-experiments.py` (`verify-check-experiments.py.out`): 28 of 29 OK. The one MISMATCH is the `difference` check, which still tests the old grouped string `½ x -278.9 - (-122.3)`. The frame now prints the three-row form and it is arithmetically correct (−278.9 − (−122.3) = −156.6; ½ × −156.6 = −78.3). The checker needs updating, not the scene.
  - `check-r2-index-scene.py` (`.out`): 79 OK, 1 MISMATCH (`glass_premium heating` 35.6). That row is never printed on screen, so it is not a defect. Exit code 1 comes from that row only.
  - `check-r2-index.py` (`.out`): 160 OK, 2 MISMATCH, both stale checker inputs: `glass_bare` uses rounded 725 (printed 725.1 gives 15.11 on screen, correct) and `shift_2` uses two-decimal endpoints (three-decimal endpoints give 0.71 on screen, correct).
  - `verify-check-index.py`: not present in the ticket folder; not run.
- Prior verdicts used: `verify-r2-experiments.md` (1 blocker, 3 major, 17 minor) and `verify-r2-index.md` (1 blocker, 4 major, 15 minor).

## 🧪️experiments: FAIL (not certified)

### Prior findings status (DE frames only; EN not visible)

| Prior | Status |
| --- | --- |
| B1 interaction formula not computing as printed | Resolved (`difference`, DE: "Differenz = Effekt bei 70 % − Effekt bei 30 %", "= −156,6 kWh/a", "A·B = ½ × [−156,6 kWh/a] = −78,3 kWh/a") |
| M1 plotted 0 m and 1 m values without room | Resolved (`curvature`, `confirm*`: hero rooms at 0,0 m = 439,7 and 1,0 m = 235,8) |
| M2 depth below energy clips | **Partly open**: headline chips per act and 20 px substituted formulas are present; leader lines from coloured terms to the drawn window, overhang or insulation are still absent (the verifier asked for them, and `scene-experiments.md` lists them as a remaining risk) |
| M3 layout reports blind | Open as a toolkit task (no seeded self-test); not a frame defect |
| m1–m17 | Visible in DE frames: resolved where checked (m4 caption wrap fixed, m8 "?" removed, m12 caption no longer repeats title, m15 "at this design" wording, m16 simulated 235,8 labelled). Remaining minors listed below |

### Arithmetic and claims (DE frames; EN wording is the same by construction)

All printed chains hold as printed, one precision per line:

- Rays 204 = 122 + 82; saving_30: 3 × 122,4 × 1,000 = 367; 938,4 → 571,5 = −366,9; 312,8 → 190,5 = −122,3; 366,9 ÷ 3 = 122,3.
- saving_70: 7 × 122,4 × 0,977 = 837; 1.700,0 → 863,3 = −836,7; 566,7 → 287,8 = −278,9.
- interaction: 7 ÷ 3 = 2,3; 837 ÷ 367 = 2,3; −278,9 ÷ (−122,3) = 2,3.
- main: (566,7 + 287,8)/2 − (312,8 + 190,5)/2 = 427,25 − 251,65 = 175,6. main_2: 239,15 − 439,75 = −200,6.
- model: 339 + 88 − 100 − 39 = 288; model_2 mean 339,4 (unrounded, stated as such); model_3: 175,6 ÷ 2 = 87,8; −200,6 ÷ 2 = −100,3; −78,3 ÷ 2 = −39,2.
- confirm: 894,9 ÷ 3,0 = 298,3; 298,3 − 339,4 = −41,1; −41,1 / 339,4 = −12,1 %. confirm_3: 141,4 of 203,9 = 69 %.
- purpose_2 runs 316,5, 186,3, 564,8, 289,4 match the cards and table. purpose_4: −78,3 + 5,7 = −72,6; error 5,7; U-wall 0,29 → 0,11; cooling 285,9 → 289,4.
- answer: 279 and 122 kWh/a match −278,9 and −122,3 and the narration.
- Narration matches the frame in every DE beat checked, including the numbers in German form (decimal comma, dot thousands).

### Defects

| Key | Severity | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| E-M1 | Major (open, partial of M2) | keys `difference`…`answer` (DE frames read; EN not visible) | Substituted formulas are 20 px and coloured, but no leader lines link the coloured terms (A teal, B orange, A·B tan) to the drawn window, overhang or insulation. | Draw leader lines from each formula term to its part on the room (window, overhang, insulation) or to the card frame for means, in the same colour. |
| E-m1 | Minor | `saving_70` (DE; EN presumed the same) | One quantity, two precisions in one frame: "Direktsonne gestoppt … = 837 kWh/a" and "Kühllast 1.700,0 → 863,3 [−836,7]". 836,7 is the printed difference. | Print the difference as 837 in both lines, or print 836,7 in the chain line; one precision per quantity per frame. |
| E-m2 | Minor | `purpose_4` (DE; EN not visible) | The cyan reason line "U-Wand 0,29 → 0,11 … Kühlung 285,9 → 289,4 kWh/a" runs to within about 8 px of the panel's right edge, and its small text sits at the edge of readability. | Break the line after "Kühlung" or shorten "bei diesem Entwurf (70 %, 1,0 m)". |
| E-m3 | Minor (process) | `verify-check-experiments.py` | Checker still tests the old grouped `difference` string and reports MISMATCH against a corrected frame. | Update the checker string to the three-row form. |
| E-m4 | Note, unverified | `confirm_3` (DE) | In a full-size look I could not confirm whether a faint dashed box near the overhang (left of the sun rays) is a ghost tag. Not reported as a defect. | Check at full size during the re-read. |

### Not verified in this session

- EN frames 35 of 35 (`question` … `answer`).
- DE frames `question`, `question_2`, `question_3`, `factors`, `factors_2`, `factors_3`, `plan`, `plan_2` (8 keys). Their narration and numbers were checked with `verify-check-experiments.py`; their visuals were not.

## ⚖️index: FAIL

### Prior findings status (EN and DE frames read)

| Prior | Status |
| --- | --- |
| B1 cost and carbon with no drawn meaning | Resolved: the value strip under the room carries a mini stacked bar on every frame (kWh layers, CHF parts, kg layers, % mark) |
| M1 `range_2` narration false on screen | **Still open** (wording fixed, but the claim is still false, see I-M2) |
| M2 winter arrows on the wrong surface; no heating in `next` | Resolved: red winter arrows on the façade wall and window (`ruler_4`, `next`); "winter … 183 kWh/a" next to them |
| M3 budget typed in the scene | Resolved: `🔣️.json` contains `budget_chf: 21000`; the frames show "Budget … 21.000 CHF" |
| M4 heating derivation, standards, daylight physics | Largely resolved: derivation lines present; footer "degree-hours method · EN ISO 13790 style" on every frame; daylight rays labelled "direct sun · not in DF" |
| m1 span 4.669 | Resolved (`shift_2`: "÷ 4.669 kWh/m²a") |
| m2 hyphen and unitless ticks | Resolved in the visible frames ("+2 kWh/[m²·a]" tick) |
| m3 unit notation brackets | **Still open** (see I-m3) |
| m9 one-word last lines | Resolved in the visible frames |
| m10 DE terms | Mostly resolved ("Fixposten", "Graue Emissionen", "Tageslichtversorgt"); "Heizen" still appears (I-m4) |
| m11 sweep winner strip colours | Resolved (`sweep`, `sweep_2`: coloured segments) |

### Arithmetic

Recomputed and holding as printed: all ruler endpoints (10.437, 10.923, 13.407, 13.732, 15.106) and normalized ticks; the cost parts (Punched 9.000 + 5.400 + 1.728 + 3.300 + 0 = 19.428; Ribbon 9.000 + 3.600 + 1.152 + 6.600 + 192 = 20.544; Premium 22.080); the carbon parts (Punched 2.520, Ribbon 3.840, Premium 5.388); the daylight factor 5,60 % (0,6 · 16,8 · 75 ÷ (180 · 0,75)); the units_2 sums 21.964, 23.848, 24.105, 24.398, 26.625 (each design includes the 2.00/3.00/… % value); units_3 sums 37,7, 38,3, 39,5, 40,4, 44,5 (kCHF and t) and the ranking change; the index products (`index_3`, `winner_2`, `next`); the limits split (Shaded 21.475 and Bare 21.091 excluded, the three allowed designs re-normalised 0,32 / 0,60 / 0,90); `range_3` rank changes; the close-call and switch shares (4 % and 39 %).

Recomputed and **not** holding as printed: see I-M1 (`ruler_4`), I-M2 (`range_2`), I-M3 (`sensitivity`, `sensitivity_3`) and I-m2 (`next`).

### Defects

| Key | Severity | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| I-M1 | Major | `ruler_4` (EN and DE) | The derivation rows collide: the top line "20.87 W/K × 70,000 Kh/a = 1,461 kWh/a loss − 1,107 free heat = 354 kWh/a" overprints the label row "Heat losses − Utilisation × Gains = Heating demand" (DE: "Wärmeverlust − Ausnutzung × Gewinne = Heizbedarf"). The sun disc and its rays cover "354.1 kWh/a" in the derivation (EN) and "354,1 kWh/a" (DE). The strict layout report confirms both (overlap and "object under", "filled shape under … 30 %"). | Move the formula block up or down so no line overprints another; move the sun icon out of the derivation area (or to the room margin); re-run the strict layout check on `ruler_4` until 0. |
| I-M2 | Major | `range_2` (EN "Premium: worst cost, best electricity; all other values shrink."; DE "…alle anderen Werte werden kleiner.") | On the electricity ruler the other values grow, not shrink. Old values 0,00 / 0,10 / 0,64 / 0,71 become 0,01 / 0,11 / 0,64 / 0,71 (Shaded, Ribbon, Super, Punched) because the best value moves from 10,437 to 10,407 and the span grows to 4,699. Only the cost ruler shrinks. | Make the claim true: "Premium is worst on cost and best on electricity; the cost values shrink, the electricity values move slightly." (DE analogue), or print the cost ruler next to the electricity ruler and narrate only what it shows. |
| I-M3 | Major | `sensitivity` (EN and DE, Punched 0,28), `sensitivity_3` (EN and DE, index line `0.229 × 0.706 + 0.600 × 0.000 + 0.114 × 1.000 + 0.057 × 0.000 = 0.28`) | The printed weights are 23 %, 60 %, 11 %, 6 %. With those, Punched gives 0,23 × 0,71 + 0,11 × 1,00 = 0,27, not 0,28. The index line uses unprinted weights 0,229 / 0,114 / 0,057 and the unrounded 0,706. The rule that each derived line holds as printed fails. (The ranking is unchanged.) | Print the weights with the precision the line needs (0,229 / 0,600 / 0,114 / 0,057 in the strip, or the index line in the same 23 / 60 / 11 / 6 %), and use one precision for the normalized value (0,71) in the same frame. |
| I-m1 | Minor | `next` (EN "winter loss 183 kWh/a", DE "Winter: Verlust 183 kWh/a") | 183 kWh/a is the net heating demand (the next derivation calls it "heat"). The loss is the larger 1,461-type value in `ruler_4`. The label says "loss". | Label it "winter heating demand 183 kWh/a" (DE "Heizbedarf"). |
| I-m2 | Minor | `next` (EN and DE, "0.505 × 10 % = 0.050") | 0,505 × 10 % = 0,0505, which does not print as 0,050 as written; it holds only from the unrounded 0,5046. | Print the carbon normalized value with three decimals that reproduce it (0,505 → 0,5046 is a four-decimal value) or write "0.0505 → 0.050". |
| I-m3 | Minor (open from r2 m3) | Unit notation on chips, rulers and parts, e.g. `units` "13.73 kWh/[m²·a]", `ruler` header "kWh/[m²·a]", `cost` "CHF/[m²·cm]", `pointer` axis; DE "kWh/[m²·a]" | Bracketed units from the bundled font are used on screen. Chips elsewhere use "kWh/m²a" without the dot. Two notations in one clip. | Use one notation across every frame, e.g. "kWh/(m²·a)" drawn as "kWh/m²a" everywhere (the toolkit `INTENSITY_UNIT`), and apply it to every plain-text unit string. |
| I-m4 | Minor | `ruler_4` (DE derivation head "Heizen") | Toolkit word "Heizen" is used where other frames say "Heizung" / "Heizbedarf". | Use "Heizung" in the derivation head, as in the chips. |
| I-m5 | Minor | `variants_2` (EN "Only window share, overhang and insulation differ."; DE "Nur Fensteranteil, Vordach und Dämmung unterscheiden sich.") | The table lists those three parameters, but the drawings differ in window geometry as well (ribbon band, glass wall, punched openings). | Narrate "Only window share, overhang and insulation are compared" or show the glass type in the table. |
| I-m6 | Minor | `ruler_4` (EN and DE) | Heat loss is printed "1,461 kWh/a" on the formula line and "1.460,7 kWh/a" in the derivation line of the same frame. One quantity, two precisions. | Use 1,461 in both (or 1.460,7 in both). |
| I-m7 | Minor (terminology) | `weights_3` (DE "stärkster Zug 40 %", "schwächster Zug 10 %") | "Zug" is an unclear rendering of "pull"; the EN is "pull". | Use "stärkster Einfluss 40 %" / "schwächster Einfluss 10 %", or "Gewicht" wording that matches the caption. |

### Visually verified and clean (no finding)

Layout and text of every other EN and DE frame read on the sheets: no clipping, no unreadable text, no empty or cluttered screen beyond the minor items above. The room and value strip move with the design on show. Captions are one or two lines and match their narration.

## Counts

- 🧪️experiments: blockers 0; major 1 (E-M1); minor 3 (E-m1 precision, E-m2 edge, E-m3 checker; E-m4 is an unverified note). Verdict FAIL (not certified): 43 of 70 frames not verified in this session.
- ⚖️index: blockers 0; major 3 (I-M1, I-M2, I-M3); minor 7 (I-m1 to I-m7). Verdict FAIL.

## Files

- This report: `T/final-verify-experiments-index.md` (this file).
- Contact sheets: `T/🗑️generated/final-verify/sheets/` (`experiments-{en,de}-01..09.png`, `index-{en,de}-01..11.png`).
- Check outputs: `T/🗑️generated/final-verify/verify-check-experiments.py.out`, `check-r2-index-scene.py.out`, `check-r2-index.py.out`.
- Narration dump: `T/🗑️generated/final-verify/narration.txt`; dump and sheet scripts: `T/🗑️generated/final-verify/dump-narration.py`, `sheets.py`.
