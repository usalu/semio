# Final verification: `Beat1_LocalGlobalExtrema` (📉️extrema) and `Beat2_GridRandomSampling` (🎲️sampling)

Read-only verification, round 3. No source file was edited. Generated crops and check logs are in `T/🗑️generated/final-verify/`.

## Method and disclosure

- Frames: all 29 extrema EN and DE frames, and all 33 sampling EN and DE frames (124 PNG), were read at full size by four reviewers, one per clip and language. Each reviewer read every PNG for its set. The reviewers reported findings in chat; the defect list below is the merge.
- The reviewers' disagreements were settled by my own reading of the frames: extrema `sweep`, `sweep_4` (EN), `scope` (EN), `constraint` (EN), sampling `compare_3` (DE), `random` zoom (EN), `limits_2` (EN), and the equation arithmetic. One reviewer claim was rejected: sampling `compare_3` (DE) does show its Δ rows, so there is no missing-gap defect there.
- Check scripts run (logs in `T/🗑️generated/final-verify/`):
  - `T/check-extrema-numbers.py`: no failures.
  - `T/check-extrema-r2-tables.py`: 48 checks `ok`, no failures. Caveat: it computes the balances with the unrounded η, so it passes the printed-arithmetic defect X1 below.
  - `T/verify-check-sampling.py`: printed per-design values (parts, bills, autonomy, shade) and the random draws; no failure lines. It prints values without pass/fail tokens, so it is evidence of the values, not a pass.
- The `…-layout.md` reports say "0 of 29" and "0 of 33 frames examined". They are not evidence and were not used.
- Motion, timing and audio were not judged (stills only).

## Verdicts

| Clip | Verdict | Blockers | Majors | Minors |
| --- | --- | --- | --- | --- |
| 📉️extrema, Beat1_LocalGlobalExtrema (29 keys × 2) | **FAIL** | 1 | 4 | 16 |
| 🎲️sampling, Beat2_GridRandomSampling (33 keys × 2) | **FAIL** | 0 | 2 | 17 |

Counts are per clip; the same defect in EN and DE is counted once. Minor items listed for the German set are included in the count.

Both clips are FAIL for specific, small fixes. Extrema has a printed equation that does not close (X1). Sampling has one narrated claim worded the wrong way and two legibility items.

## 📉️extrema: defects

Keys are frame keys. EN and DE both apply unless marked.

### Blocker

| ID | Keys | Location (approx px, 1280×720) | Defect | Fix |
| --- | --- | --- | --- | --- |
| X1 | sweep (EN and DE) | equation row 1, y ≈ 195–215, x ≈ 480–985 | Printed "1,877 kWh/a − 0.63 × 2,748 kWh/a ≈ 152 kWh/a". The printed inputs give 1,877 − 1,731 = 146, not 152. The "≈" hides a 6 kWh/a (4 %) gap. Cause: `scene line 268` prints η with `live(eta, INK, 2)`, and the sign rule (line 265) falls back to "≈" instead of fixing the precision. Model η = 0.6277 gives 152.1. | Print η with four decimals (0.6277): 1,877 − 0.6277 × 2,748 = 152.1 → 152, and print "=" only when the printed inputs reproduce the printed result. Make `check-extrema-r2-tables.py` check the printed inputs, not only the unrounded ones. |

### Major

| ID | Keys | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| M1 | sweep, sweep_2, sweep_3, sweep_4 | derivation head row, y ≈ 140–180, x ≈ 480–1040 | Term labels sit under the wrong values. "Losses" is under 43 and "Gains" under 135 (sweep). "Gains" is under 43, and "Abführbar = Kühlbedarf" under 135 and 681 (sweep_2). "Tageslichtquotient / Autonomie / Lampen an" sit under 311, 407 and 164 (sweep_4). A reader takes 43 as losses and 681 as cooling demand. The EN sweep frame shows the same ("Losses", "− Useful η ×", "Gains" beneath 43 / 503 / 135). | Put each term name inside its own derivation line; keep the sum row free of sub-labels. |
| M2 | sweep_4 (summer and winter panels), local, constraint, constraint_2, feasible, feasible_2 | section panels, x ≈ 130–215, y ≈ 215–300 | At 0 h of direct sun the summer panel still draws a yellow sun symbol with a starburst at the glazing and a yellow fan of lines inside the room (crop `en-sweep_4-summer-zoom.png`). The label reads "direct sun 0 h". The caption says "only diffuse light reaches the glass". This is the prior B1, partly open: the rays inside the room are gone, the sun icon and fan are not. | Draw diffuse light as a soft pale band with no sun symbol and no spikes, or label the burst "diffuse sky light". Keep the sun icon only when hours > 0. |
| M3 | every solstice panel (response, sweep to sweep_5, local, local_2, global, constraint, feasible) | caption under each panel, x ≈ 60–180 | "44 % of best", "57 % of best", "21 % of best" give no measured quantity and no reference. The prior M3 is only partly fixed: the DE reviewer notes the reference is named but not what is measured. | "sun on glass, 44 % of the best façade" (or the chosen name), or drop the percentage. |
| M4 | negate_2 | formula box y ≈ 140–175 and chart chip y ≈ 320–352 | The same quantity is printed as S = 449.8 kWh/a in the formula and as S 450 kWh/a in the chip (EN major, DE minor). | Print one value in both places, for example 449.8 kWh/a (449.8 × 0.25 = 112.45, printed 112.5 with the tariff shown). |

### Minor

| ID | Keys | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| m-a | sweep_3 | "2,549 − 1.00 × 214 ≈ 2,334" (EN); "≈ 286 CHF/a" beside 1,146 kWh/a (DE) | Printed inputs give 2,335, not 2,334. 1,146 × 0.25 = 286.5, printed 286. | "=" with 2,335, or print the unrounded values; CHF/a 287 or 286.5. |
| m-b | sweep_4, local | chart maximum tag "maximum 282°" (sweep_4); bar tags (global, local) | The maximum has no kWh/a value on that frame. Bar axis has no ticks (prior m7). | Add "1,146 kWh/a" to the tag; add ticks at 0, 500, 1,000 kWh/a to the bar column. |
| m-c | global_2, maximize | change column, x ≈ 955–1040 | Bare numbers (−268, +96, −29; +138, +300, +12) with the unit only on the total row. | Put "kWh/a" in the column header. |
| m-d | global_3, feasible_2 | chip "882 − 681 = +201 kWh/a · +29.5 % · +50 CHF/a" | "+29.5 %" has no base, and North / South are not named in the subtraction. | "North 360° − South 180°: +201 kWh/a (+29.5 % of South)". |
| m-e | constraint family | chip "θ 360° · 882 kWh/a", x 718–910, y 316–348 | The dashed 270° limit line starts at the chip's bottom edge (crop checked in the EN constraint frame). The reviewers disagree on whether it crosses the chip; it touches it. | Start the limit line 6 px below the chip, or move the chip. |
| m-f | local, compare_2 | "start" tag, x ≈ 755–800, y ≈ 350–366 | Tag touches the readout chip above it. | Move the tag beside its ring. |
| m-g | global, constraint_2, feasible, feasible_2 | legend | Legend glyph does not match the plotted marker (square versus circle; the orange X is not in the constraint_2 legend). | Match the glyphs. |
| m-h | sweep_4 (summer and winter) | teal and red arrows leaving the room | Heat-flow arrows have no legend. | Label them "heat removed" and "heat lost" once. |
| m-i | scope | overhang curve, x ≈ 600–720, y ≈ 480–505; colour bar "832 … 882 kWh/a" | The teal overhang curve has visible corners and a plateau that the caption does not explain (the model's hourly sun sampling). The colour bar's meaning is not stated. | Name the cause in the caption or a note ("hourly sun samples"); label the colour bar "kWh/a per window share". |
| m-j | response_2, sweep | "Goal: minimize E" and "E" before E is defined; "Sun 19°" in fixed without "altitude" | Term used before definition; "Sun 19°" is ambiguous. | "E = electricity"; "Sun altitude 19°". |
| m-k | compare, compare_2 | chip "S 180° · 681 kWh/a" | S means South here and saving elsewhere. | One symbol set per clip. |
| m-l | negate | label "S = 0: east 90°" x ≈ 575–645, y ≈ 485–512 | Label sits near South; its guide touches "90°". | Move the label to the 90° origin. |
| m-m | German, all frames | chips and equations | Lowercase chip starts ("unzulässig", "zulässig", "ausgeschlossen", "lokales/globales Minimum", "kleiner ist besser", "beste"). ASCII hyphen as minus ("882 - 681", "E bei 90° - E bei θ", "1.877 kWh/a - 0,63"). English "WWR", "d", "t" in section chips. "O" on the compass versus "Ost" in tags. | "Unzulässig…", "Zulässig", "Ausgeschlossen", "Bestes Ergebnis"; use "−"; "Fensteranteil", "Vordach", "Dämmung"; one word for east. |
| m-n | sweep_5, sweep, response_2, argument (captions) | caption second lines | One-word or two-word orphans: DE "sie begann." (sweep_5); EN "electricity falls.", "teaching model.", "in kWh/a.". | Rebreak so line two has at least three words. |
| m-o | sweep, sweep_2, sweep_3, sweep_4 | whole frame | Two panels, a chart, chips, equation block and bar at once. Denser than the energy clip (depth noted, not a separate defect). | Move the bill and one equation to a separate beat if time allows. |
| m-p | local, local_2 | neighbour tables | Neighbours are numbers only; no drawn cause (prior m11). | Optional: one mini plan per neighbour. |

Prior extrema items: B1 partly open (M2); B2 open in part (M1, X1); M1 chip-on-line touch (m-e); M2 fixed (arithmetic now shows 0.432 kW and 25.3 %, 164 closes); M3 open (M3); M4 changed (the scope now shows θ* 329° at 870 kWh/a; wiggle m-i); m1 brackets fixed; m2 partly open (m-m); m3 partly (m-m); m4 start ring open in compare_2 (m-f); m5 fixed; m6 fixed in most frames; m7 open (m-b); m8 fixed; m9 fixed (no climate claim); m10 fixed; m11 open (m-p).

## 🎲️sampling: defects

No blockers. The prior B1 (limits_4 derivation) is resolved: the reviewers read 135.0 ÷ 3.5 = 38.6, 841.8 ÷ 3.0 = 280.6, 0.432 kW × 1,500 h/a × 27.3 % = 176.7, sum 495.9, bill 124. Each closes as printed.

### Major

| ID | Keys | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| S-M1 | limits_2 (EN caption), limits_3 (EN card) | caption band y ≈ 640–685, x ≈ 330–950 | "A finite sample proves no global optimum." Read literally, the sentence says the sample proves that no optimum exists, which is false. The intent is "cannot prove". The DE card "Sie beweist kein Optimum" has the same problem (minor in DE). | EN: "A finite sample cannot prove the global optimum." DE: "Eine Stichprobe kann das globale Optimum nicht beweisen." Check the two-line limit. |
| S-M2 | run, run_2, run_3, reveal_2, limits_3, limits_4 | room labels "Daylit ≥ 2 % to 3.8 m" (≈ x 747–905, y 344–354); meter ends "0 %" / "100 %" and "Glass in shade" (y ≈ 381–402); grey subtitle "Simplified annual teaching model…" (x 117–550, y 577–588, limits_3 and limits_4); plane tick labels | Measured cap heights are about 11–13 px (EN reviewer) and 9–11 px for the grey note (DE reviewer). The prior threshold is 14 px. The daylight range and meter ends carry claims. | Set these labels to at least 14 px cap height or shorten them; lighten the grey subtitle and move it clear of the axis title. |

### Minor

| ID | Keys | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| S-m1 | limits_2, limits | chip "≤ 519.6 kWh/a" beside "Dense grid → 494.8 kWh/a" | 494.8 × 1.05 = 519.54, printed 519.6 (the model value 494.84 gives 519.58). A reader who checks from the screen gets 519.5 (prior m3). | Print 494.84 for the optimum wherever it appears, or 519.5 for the threshold. |
| S-m2 | gaps_3 (EN), gaps_2, gaps_3, reveal_2, parameters (DE) | caption line two | Orphans: EN "coverage."; DE "Simulationen.", "Abdeckung.", "Bereichs.", "Südfassade?". | Rebreak so line two has at least three words. |
| S-m3 | random (EN) | badge "9" at about x 231–259, y 414–445 | Badge 9 sits on the outline of thumbnail 2 (zoom `en-random-zoom.png`). The EN reviewer rated this major; my zoom shows a touch, so minor. | Place each badge outside every thumbnail box with a short leader. |
| S-m4 | random_3 (DE) | samples 3 and 4 (x ≈ 285–345, y ≈ 295–345) | Two points merge into one unnumbered blob. | Number them as in random, or shrink the pair. |
| S-m5 | reveal_2, gaps_3 | star (66 %, 0.99 m) and triangle (68 %, 1.00 m) at x 497–530, y 258–290; gaps_3 green ring over the orange pair ring | Markers merge (prior m9, partly open). | Label the optimum and the LHS best with short leaders; shrink the pair ring to r 0.18. |
| S-m6 | gaps | chip "WWR 52 % · d 0.68 m" (x 905–1124, y 46–80) | The box centre is 0.675 m, printed 0.68 (a tie; prior m6, open). | Print the box edges, or the centre to three decimals (0.675 m). |
| S-m7 | gaps | dashed box x 320–534, y 232–438 | Thumbnails at WWR 35 % and 70 % straddle the box edge; the caption says "edges excluded" (prior m7, partly open). | Draw the thumbnails above the outline, or shrink the outline to the interior. |
| S-m8 | cells | chip "largest empty box" x 299–465, y 296–330 | The chip sits on the dashed top edge; box edges run through draws at 14 % and 76 % (prior m8, partly open). DE caption "9 von 81 Zellen" is narrated, but no 81-cell grid is drawn, so 81 cannot be counted. | Move the chip above the box; draw a faint 9 × 9 cell grid, or drop "81". |
| S-m9 | run, run_2, run_3, reveal_2, limits_4 | bar total (e.g. "522.1 / 494.8 / 495.9 kWh/a") at x 676–822, y 440–470; "0 %" meter end | The total and the meter end label touch. | Move the meter about 40 px right, or lower the total. |
| S-m10 | run, run_2 | top chip "d 0.60 m" against plane chip "d 0.6 m" | Two precisions for one overhang on one frame (prior m5, open here). | "d 0.60 m" everywhere on those frames. |
| S-m11 | compare, compare_2, compare_3 | tile bars x ≈ 915–1245; Δ bars in compare_2 | No scale on the tile bars; Δ bars are slivers (about 3–9 px of 127 px) (prior m2, partly open). Work-plane line in the tiles is unlabelled (prior m10). | Add ticks "0 – 846 kWh/a" to the tile bars and "0 – 67.0 kWh/a" to Δ bars; label the work-plane line once. |
| S-m12 | run_2, run_3, reveal_2, limits_4 | equation panel x 815–1262, y 450–580 | About 25 numerals on one screen, including the bill row (prior m12, open). Lower depth than the energy benchmark in legibility. | Move the bill row to its own beat. |
| S-m13 | gaps_2, gaps_3 (DE) and others | chips "Zug 3 / Zug 4" against captions "Ziehung"; chips "nicht simuliert" (budget, cells, evaluate, gaps, grid, grid_2, lhs, marginals, marginals_2, random) and "größte leere Box" (cells) start lowercase | Mixed terms for one draw; lowercase chip starts. | "Ziehung 3/4", "Die beste Ziehung", "Nicht simuliert", "Größte leere Box". |
| S-m14 | run_3, reveal_2, limits_4 (DE) | equation line x 815–1262, y 565–580 | "Rechnung" differs from "Stromrechnung" on the bar. | "Stromrechnung". |
| S-m15 | reveal_2 (DE); compare tiles | chip "LHS-Bester"; "Latin-Hypercube" in tile labels | English method names in German chips. Acceptable only if the scene accepts them in both languages. | Decide once; if accepted, use it in both languages with the same spelling. |
| S-m16 | growth, growth_2, growth_3 | E/S/W/N headers and 10/20/30 cm labels, y ≈ 385–560 | Labels appear with no matrix cells, so the still looks half-built. Uncertain whether this is the intended build state. | Show the matrix cells, or hide the labels until the cells appear. |
| S-m17 | evaluate, evaluate_2, reveal, reveal_2 | LHS and random points | Points are coloured only; their kWh/a values are not printed. The colour depends on the ramp (uncertain against the owner's bar). | Optional: label the best three points with their values. |

Prior sampling items: B1 resolved; M1 resolved; M2 resolved; M3 resolved (points 1–9 numbered and matched to the table; badge touch is S-m3); M4 resolved for totals (integer ramp ticks 495 and 846 beside one-decimal totals remain, minor); m1 resolved; m2 partly (S-m11); m3 open (S-m1); m4 resolved; m5 open on run and run_2 (S-m10); m6 open (S-m6); m7 partly (S-m7); m8 partly (S-m8); m9 partly (S-m5); m10 partly (S-m11); m11 resolved; m12 open (S-m12); m13 resolved; m14 resolved; m15 resolved.

## Rejected reviewer claims

- Sampling `compare_3` (DE): "Δ rows and Δ bars are gone". Rejected. The rows "Δ 4,4 kWh/a · 0,9 %", "Δ 67,0 kWh/a · 13,5 %" and "Δ 1,1 kWh/a · 0,2 %" and the banner "Δ-Balken: volle Länge = 67,0 kWh/a" are all present.
- Extrema `sweep_4` (EN): "beams at 0 h" in the sense of rays entering the room. Narrowed. No rays enter the room; the sun icon and fan at the glazing remain (M2).
- Sampling `random` (EN): "major" badge collision. Downgraded to minor (S-m3).

## Owner's bar, per clip

- **📉️extrema: FAIL.** Every plotted value has its room drawing and its meaning (plan, section, bar, meter). The printed arithmetic fails in one place (X1). The term labels in the derivation attach to the wrong values (M1). The sun icon at 0 h (M2) and the bare "% of best" (M3) remain. Depth is between energy clips and this one: derivations for heating, cooling and lighting exist now, but the derivation row is harder to read than the energy benchmark.
- **🎲️sampling: FAIL.** All values are shown where they belong, and the arithmetic holds as printed, except the wording of S-M1 and legibility (S-M2). Density is higher than the energy benchmark (S-m12). Most round 2 blockers and majors are resolved.

## Files

- Frames: `T/🗑️generated/frames/Beat1_LocalGlobalExtrema/`, `T/🗑️generated/frames/Beat2_GridRandomSampling/`
- Scenes: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py`, `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py`
- Checks and logs: `T/🗑️generated/final-verify/check-extrema-numbers.out`, `check-extrema-r2-tables.out`, `verify-check-sampling.out`
- Zoom crops: `T/🗑️generated/final-verify/en-sweep_4-summer-zoom.png`, `en-random-zoom.png`, `en-limits_2-zoom.png`; crop script `crop-final.py`
