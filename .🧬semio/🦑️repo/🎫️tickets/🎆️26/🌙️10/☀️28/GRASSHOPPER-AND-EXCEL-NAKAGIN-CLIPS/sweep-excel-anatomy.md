# 🦴 Final Sweep: Excel Anatomy (Beat1_Anatomy)

Series `📊️excel`, scene `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`, class `Beat1_Anatomy` (scene file mtime 2026-10-29 04:35).
Frames read: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/🗑️generated/frames/📊️excel/Beat1_Anatomy/{en,de}/`, all 23 clauses in both languages, in narration order.

Freshness: all PNGs are within 30 minutes of the scene file. The oldest are the EN `workbook`, `sheets`, `grid`, `address`, `range`, `bars`, `constant` frames (04:22 to 04:23, about 13 minutes before the scene edit). If the 04:35 edit touched the EN workbook, these need a re-capture (see fix 1).

## Verdict: FIX

### Fixes

1. **EN, workbook tab strip has no Energy sheet (major, caption contradicts picture).**
   - Frames: `en/sheets.png` (caption "...Climate and Energy."). The tab strip in every EN frame from `workbook` to `legend` reads Program, Type A, Type B, Type C, Type D, Climate, then "+". No Energy tab.
   - The DE frames show `Energie` in the same strip (`de/sheets.png`), so the EN workbook is missing the sheet, or its tab is hidden.
   - Fix: check the EN workbook sheet list (`_xl.workbook(language="en")`). If Energy is missing, add it as DE has it. If it exists but is hidden, show it. Then re-capture EN frames `workbook` through `legend`.

2. **DE `value`, German grammar (major for the brief's category).**
   - Frame: `de/value.png`. Caption: "Text, wenn eine Zahl erwartet wird, ergibt #WERT! ..."
   - "wenn" (if/when) is wrong for a place-like sense. Use "wo" (where).
   - Fix in the NARRATION `value` DE line: "Text, wo eine Zahl erwartet wird, ergibt #WERT! und im Englischen #VALUE!: Hier wurde die Innenlänge mit Einheit getippt." Re-capture `de/value`.

No other blockers or majors found.

### Checked and fine

- Both languages: captions and tooltips do not clip or overlap, no `###` on a shown value, no empty or static stages.
- Numbers (checked by arithmetic against the Type A inputs): 2.30 x 3.80 = 8.74, shown 8.7 m²; volume 18.4 m³; perimeter 9.00 / 13.00 m; fittings 1.8 + 1.2 + 0.6 = 3.6 m²; widened interior 2.50 x 3.80 = 9.5 m², volume 20.0 m³, delta +0.8 m²; #DIV/0! state heating load 618 W (30.92 x 20); demand per area 148 kWh/(m²·a) = 1,296 / 8.74.
- DE UI: German ribbon, status bar, Name Box and formula bar labels, `Zelle B9` popup, decimal commas, `SUMME` with `;` and `1,5`, `#BEZUG!`, `#WERT!`, `#NAME?`, `Behebung:` chips.
- DE contains English only as the deliberate `EN` / `DE` tags and the English `=SUM(G12:G15,1.5)` in the `locale` frame, which is the comparison the clause makes. Excel function names are allowed.
- Frame-to-caption checks: `frames`, `hatch`, `precedence`, `function`, `recalc`, `trace`, `div0`, `name`, `ref`, `repair`, `legend` match their captions in both languages.

### Not reported (minor, cosmetic, or unverified)

- `en/trace`, `de/trace`: the orange arrow tip at B15 touches the first digit of "148 kWh/(m²·a)".
- `en/bars`, `de/bars`: the "Name Box" / "Namenfeld" leader runs through the title text "Nakagin.xlsx".
- `en/hatch`, `de/hatch`: the "8.7 m²" badge touches the window opening.
- `en/ref`, `de/ref`: Transmission H_T reads 24.92 W/K, against 30.92 W/K in every other state. This is an error-state side effect of the deleted row, and I did not check it against the Nakagin model, so it is not a verdict item. Worth one look against the model if time allows.
