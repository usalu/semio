# Verification: excel_anatomy (`Beat1_Anatomy`)

Fresh captures: `🗑️generated/frames/📊️excel/Beat1_Anatomy/en` and `de` (23 PNGs each, both runs logged "captured 23 beats"). All 46 PNGs viewed. Audit: `audit-excel-anatomy.md`.

Verdict: **NOT READY.** Most audit defects are fixed. Two precedence items are still open, the legend pointers are still incomplete, and the English precedence caption contradicts the drawing.

## Audit blockers and majors

| Audit item | en | de | Frame evidence |
| --- | --- | --- | --- |
| Blocker: parentheses drawn as brackets | FIXED at 720p | FIXED at 720p | `precedence`, `function`, `locale` show round `(` `)` in the formula bar. The 1080p check is still open. |
| Ghost tab bar `grid` to `legend` | FIXED | FIXED | `grid`, `bars`: no empty bar |
| `range` values greyed under the tint | FIXED | FIXED | `range`: values at full contrast |
| `bars` labels on ghost bar, Name Box leader | FIXED | FIXED | `bars`: labels above the title |
| `display` card covers column B | FIXED | FIXED | `display`: B values visible |
| `display` format `0.00 "m"` (German) | n/a | FIXED | `display` de: `0,00 "m"` |
| Colour linkage width/length | FIXED | FIXED | `range`, `frames`: outer and interior dimensions use the same pair |
| `hatch` area label on floor | FIXED | FIXED | `hatch`: "8.7 m²" badge |
| `precedence`: wrong formula never shown | NOT FIXED | NOT FIXED | Only the right result (13.00 m) and the wrong 9.00 m row are shown; `=2*B2+B3` never appears in the formula bar |
| `function`: fitting labels and total | FIXED | FIXED | `function`: Bed, Bathroom unit, Storage, Σ 3.6 m² |
| `recalc`: stale formula bar, outer shell | FIXED | FIXED | `recalc`: bar shows `=B5*B6`, 9.5 m² and 20.0 m³ consistent |
| `trace`: dependents not visible | FIXED | FIXED | `trace`: dependent arrows reach rows 14 and 15 |
| `div0`: red mark, tooltip covers values | FIXED | FIXED | `div0`: tooltip below the table, empty interior length marked |
| `name`: `#NAME?` clipped | FIXED | FIXED | `name`: full "#NAME?" |
| `ref`: tooltip covers values | FIXED | FIXED | `ref`: values visible |
| `ref`: removed surface row never mentioned | NOT FIXED | NOT FIXED | Row 6 deletion also removes "Right side wall" (C6:F6); no caption or frame mentions it |
| `value`: tooltip covers Volume | FIXED | FIXED | `value`: values visible |
| `repair`: retyping not animated | PARTLY | PARTLY | Still frame only; the animation was not played, so this is not verified |
| `legend`: badge 6 has no pointer on the window | NOT FIXED | NOT FIXED | `legend`: "6" appears only in the list, not on the SUM token |
| `legend`: badge 2 on the tab | PARTLY | PARTLY | `legend`: badge 2 sits in the status bar below the "Type A" tab, not on it |
| `legend`: badge 7 partly covers Name Box | PARTLY | PARTLY | Badge overlaps the right side of the Name Box; "B9" stays readable |
| `hook`: chip overlaps podium | FIXED | FIXED | `hook`: chip above the caption, clear of the podium |
| `constant`: text cell A2 not selected | NOT FIXED | NOT FIXED | `constant`: only B2 selected; caption names "Width" but no frame shows A2 |
| `de` `div0`/`value`: "Innenlänge" wording | n/a | FIXED | `div0` de: "Eine leere Innenlänge"; `value` de: "die Innenlänge" |

## Audit fact and didactic findings

| ID | Status | Notes |
| --- | --- | --- |
| F1 German format code | FIXED | `display` de: `0,00 "m"` |
| F2 status bar values | pass | `range` en: "Average: 3.00 m / Count: 3 / Sum: 9.00 m"; de: "Mittelwert: 3,00 m / Anzahl: 3 / Summe: 9,00 m" |
| F3 error tooltips | still CHECK | Custom teaching strings; confirm real M365 texts or label them as teaching text |
| F5 `SUMME`, `;`, `1,5` | pass | `locale`, `function` de |
| F9 error behaviour | pass | `#DIV/0!`, `#NAME?`, `#REF!`/`#BEZUG!`, `#VALUE!`/`#WERT!` |
| F12 German wording | FIXED | "Dezimalstellen", "Merken wir uns" |

## New findings

| Severity | Lang | Frame | Defect | Fix |
| --- | --- | --- | --- | --- |
| major | en | `precedence` | Caption reads "Brackets fix it: 2*[B2+B3]" with square brackets, while the formula bar shows `=2*(B2+B3)` and the German caption uses `(`. Square brackets are invalid Excel grouping, so the caption contradicts the drawing. | Use `2*(B2+B3)` in the English caption. |
| major | en+de | `precedence` | The wrong formula is still never shown (see audit). | Show `=2*B2+B3` with a "forgot the brackets" callout for about 1.5 s, then the right formula. |
| major | en+de | `legend` | Badge 6 has no pointer on the window (see audit). | Point 6 at the SUM token in G16. |
| major | en+de | `ref` | Side effect of deleting row 6 (surface row removed) is not mentioned. | Name the removed surface row in the caption, or use a helper row so no surface row is lost. |
| minor | en+de | `legend` | Badge 2 sits below the Type A tab, not on it. | Move it onto the tab centre. |
| minor | en+de | `constant` | Caption names text "Width" but no frame shows A2. | Select A2 in the same clause, or drop "text such as Width" from the caption. |
| minor | en+de | `function` | Arguments of SUM are not annotated (storyboard). | Label `G12:G15` as the argument. |

## Remaining checks

- Parenthesis glyphs: the blocker is not reproduced at 720p. Confirm at 1080p before filming.
- Repair and hatch sweep are still frames only; the animations were not played.
- F3 error tooltips need confirmation against the real M365 text.

## Must change before READY

1. Fix the English precedence caption to `2*(B2+B3)`.
2. Show the wrong formula `=2*B2+B3` in the precedence clause (en and de).
3. Point legend badge 6 at the SUM token and move badge 2 onto the Type A tab (en and de).
4. Name or handle the removed surface row in the `ref` clause (en and de).
