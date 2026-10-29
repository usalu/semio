# Scene: Excel / Anatomy (`Beat1_Anatomy`)

File: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`. Storyboard: `T/storyboard-excel-anatomy.md` (its cell addresses are superseded by the binding below). Audit: `T/audit-excel-anatomy.md`. Probe: `T/excel-anatomy/probe.py`. Keyframes: `T/🗑️generated/frames/📊️excel/Beat1_Anatomy/{en,de}/`.

## Binding to the workbook (Type A sheet)

B2:B4 outer width / length / height, B5:B7 interior width / length / height, B9 net floor area `=B5*B6`, B10 volume, B15 `=B14/B9`, C11:G16 fittings with `G16 =SUM(G12:G15)`. No cell was added to the workbook JSON (and `build_workbook.py` is not run any more): the scratch cells A18:B19 of the precedence demo and every error state are made with engine events (`apply`: set-cell, clear-cell, delete-row). The scene uses `workbook(language=lang)`, so German labels come from the engine, and tab names, error codes, formulas and display strings from `shown`/`display`/`localize`; nothing is typed in the scene.

## Round 2: audit findings fixed

| Audit finding | Fix |
| --- | --- |
| ghost tab bar, separate strip, `_Titled` wrapper | removed; the window shows all seven tabs (German from the toolkit), `Indicate` on `window.tabs` |
| windows pop in | windows fade in and out (`FadeIn`/`FadeOut` on the window) |
| formula bar vs cell (2.3 vs 2.80) | the recalculation selects B9 (formula `=B5*B6` is true during the ramp); B5, B9, B10 are bound to the engine recalculation of the same width |
| capsule wider than B2 | the outer shell is always drawn from B2:B4; only the interior floor follows B5 (ramp from the model value to the outer width, so the floor never leaves the shell) |
| one colour per quantity | width blue, length red, height beige on cell frames, outer lines, interior lines and formula tokens; area green |
| tooltips and cards covering values | error tooltip moved under the capsule; cell card moved beside the window (German card shows `0,00 "m"`); error windows show only the relevant rows |
| dependents invisible | trace window shows rows 4 to 16: range arrow B5:B6 to B9 and dependent arrows to B10, B15 |
| red mark on the wrong line | the interior length is marked (dashed red floor outline and "?" line) for #DIV/0!, #REF!, #VALUE!; B16 hidden in the #DIV/0! window |
| typical mistake not named | new clause text names `2*B2+B3`; the formula bar and outline swap from wrong (three sides, red, 9.00 m) to right (four sides, green, 13.00 m) inside the clause |
| repair not animated | `window.recalculate(book)` morphs the cell back to the number, the errors resolve |
| legend pointers | badges on title bar, Type A tab, a cell, the B5 reference frame, the formula, the Name Box and the formula bar; the function has its own example `=SUM(G12:G15)` / `=SUMME(G12:G15)` |
| hatch / fittings labels | area badge on the floor, fitting names and areas in the hatch colours with the total |
| status bar values | formatted with the number format of B2 while the range is shown (hook on `xl_display`) |
| captions | "interior length" in div0 and value; German wording "Dezimalstellen", "wenn", "Merken wir uns" |
| constant | selection hops A2 then B2 inside the clause |
| hook chip | tower raised, chip clear of the podium |

## Keyframe review

All 23 PNGs per language of the second capture were read (en and de). No clipped or overlapping text remains except what is listed.

Open, outside the scene:
- The toolkit's range-selection tint greys the values in B3:B4 (still faint in `range`). Needs a toolkit fix (text above the tint or outline-only selection).
- The error tooltip, the cell card and `#VALUE!`-type cascades are drawn by the toolkit; the tooltip is now placed under the capsule, so they cover no taught value.
- Parenthesis glyphs now read as round brackets in the window; confirm at 1080p in the final render.
- The Name Box and formula-bar callout leaders cross the title bar and the ribbon labels (thin lines, label above the window).
- German tooltips and `#BEZUG!`/`#WERT!` texts come from the engine and toolkit (secondary sources, see `T/xl-engine.md`).

Layout report: `en` 24 strict findings in 12 of 23 frames, `de` 51 in 17 of 23; all remaining findings are grid lines through cell text, header gridlines, selection tints (window chrome) and the cell-card overlay.

## Tests

Series presentation test and model test results are in the final message.
