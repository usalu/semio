# XL-TOOLKIT report

Contract: `T/api.md` section 6 (Status: ready). Stills: `T/🗑️generated/xl-toolkit/{edit,trace,error,formulas,pop,chart,recalc}-{en,de}.png` with `layout-*.md` strict reports. Re-render: `python T/xl-toolkit/showcase.py [page ...]`.

## Delivered (X = `🎓️teaching/🏛️architecture/📊️excel/🎬️clip`)

| File | Content |
| --- | --- |
| `X/🎨️visuals/🐍️.py` | `ClipScene(Drafting, NakaginDrawing, Workbook, Scene)`: BIM interface `language/tr/number/step/present`, `source_files()` with Excel model + JSON, Nakagin model + visuals. `NakaginDrawing` import is guarded by the missing-module name (now resolves for real). |
| `X/🎨️visuals/📗️workbook/🐍️.py` | `Window`, `EngineView`, mixin `Workbook` (`xl_window`, `xl_ref_frame`, `xl_fill_handle`, `xl_trace`, `xl_error`, `xl_tabs`, `xl_bar_chart`, `xl_formula_text`, `xl_cell_pop`) and address / reference helpers. |
| `X/🎨️visuals/📗️workbook/🔣️.json` + `🧬️schema/🔣️.schema.json` | ribbon names en/de, reference colour order, UI strings, error labels and tooltips en/de (schema first). |
| `X/🧪️tests/🎭️presentation/🐍️.py` + `🔣️.cases.json` | 11 tests, all green. |
| `T/xl-toolkit/showcase.py` | stills from the real XL-engine Nakagin workbook (sheet Type A), en and de, strict layout check per still. |

## What the widgets do

- Window: green title bar with the file name, search pill, window buttons; ribbon tabs (en File Home Insert Page Layout Formulas Data Review View, de Datei Start Einfügen Seitenlayout Formeln Daten Überprüfen Ansicht) with the active tab underlined; Name Box with drop-down; cancel / enter / italic `fx`; formula bar; column letters and row numbers (selected ones green); gridlines; thick green active border with fill handle; tinted range with a white active cell; sheet tabs (active white, green underline, arrows, plus); status bar with Ready/Bereit, Average/Count/Sum (Mittelwert/Anzahl/Summe) of the selection, zoom.
- Edit mode (`editing=` or `.edit(f)`): coloured tokens in the cell and the formula bar with caret, one frame per distinct reference on the sheet (blue, red, purple, green, orange, teal, magenta, brown in order of first appearance); references to other sheets are coloured in the text but get no frame, as in Excel.
- Values: from the engine (`display`, bilingual labels, `localize` for German function names, `;`, decimal comma); error cells show the German label (#BEZUG!, #WERT!, #NV, #ÜBERLAUF!); left-aligned text overflows into empty cells then ends in an ellipsis, numbers that do not fit become `###`.
- Animation: `select` (glide), `bind_value` (live cell text from a `ValueTracker`), `recalculate(new_workbook)` (changed cells morph), `xl_bar_chart` with tracker values rebuilds when a value changes.
- Close-up: `xl_cell_pop` shows Address, Stored value, Displayed, Formula (coloured), Number format with a leader and ring on the cell.

## Verification

- Frames read with the Read tool in en and de; iterated until the window, frames, trace arrows, error tag and charts read as Excel. Cell text is 16.2 (rows below 0.34 units) or 19.2 at 1080p scale, the window fits the band y ∈ [−2.55, 2.55] above the caption.
- Strict layout (`T/layout_strict.py`) on every still: remaining findings are by design (tooltip and cell-pop card are opaque overlays on cells; reference frames and trace boxes lie on gridlines; 4 px proximity of descenders to the grid line in 0.33-unit rows). No clipped, off-frame or unreadable text.
- Tests (`python -m unittest X/🧪️tests/🎭️presentation/🐍️.py`): scene contract for every scene discovered by folder (narration en/de, unique keys, 14–24 clauses, title en/de, one `present_*`, step keys equal narration keys, literal imports resolve, render provenance closure, native Cairo lifecycle en+de) — passes vacuously now because no scene exists; toolkit tests against portable cases and the `openpyxl` oracle (column letters, ranges), reference tokenisation (no match in strings or in `LOG10(`), colours in order, bilingual UI and error strings, a window rendered on Cairo in both languages (lookup, select, edit, bind, trace, error, frame, fill handle, formula text, cell pop, chart), and a window over the real Nakagin workbook whose every cell string equals the engine's `shown` in en and de plus a recalculation check.

## Notes for scene authors

- Z-order is fixed (fill, tint, gridlines, text, frames, border) and kept by `select` / `recalculate` (`Glide` animates in place); the range tint can no longer grey the values. Test `test_selection_tint_stays_under_text` plays both animations and checks the order.
- Narrow windows: the title-bar search pill shrinks, then becomes a magnifier icon, then disappears, so the file name never sits under it (test `test_title_bar_keeps_the_file_name_readable`).
- Long formulas: the formula bar shrinks to 16.2, then wraps (operators, separators, never inside a reference) into an expanded panel over the grid with a chevron; colours stay per reference; in-cell edit text is dropped when it cannot fit. Test: `test_long_formula_wraps_inside_the_window` (120+ character German formula); still `long-{en,de}.png`.
- Tabs: localized by the engine's `tab_name` automatically; `xl_tabs(..., titles=sheet_titles(language, wb))` for standalone strips; compact tabs, all seven sheets fit at 12.4 units.
- `window.scene` is weak: standard `FadeIn`, `FadeOut`, `copy.deepcopy` work; drop scene `__deepcopy__` overrides.
- Glyphs: `-`, `*`, `=`, `,` sit on the baseline; `(` `)` use the theme serif face (round, the body face draws brackets). Test `test_formula_glyphs` renders `=SUM(B2:B4)-B5`.
- Overflow: left text and centred errors overflow into empty neighbours (opaque backing), centred text is scaled down if needed, left text gets an ellipsis, numbers `###`; error codes are never truncated.
- Create the window before animating it in; `window.cell(a1)` / `window.range(a1)` return groups usable with `Indicate`, `Circumscribe`; `window.rect(a1)` and `window.point(a1)` give screen coordinates for the Nakagin drawing leaders.
- Edit mode: `window.edit("=B5*B6")` then `FadeIn`, or type progressively with several `edit` calls on prefixes of the formula; `xl_formula_text(...)` shows the same colours elsewhere (for before/after copy displays).
- The stage height holds 10 rows at height 5.1; use `rows=(r1, r2)` and `cols="A:F"` to show the part of the sheet that matters, and a window of about 7 units next to the Nakagin drawing.
- The number format shown is the engine's cell format; custom row styling (bold header, fill) needs a `style` method on the view (the engine view has none yet).
