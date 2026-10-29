# 📋 Audit: excel / 📋️program / `Beat3_Program` (read-only)

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`
Frames: `T/🗑️generated/frames/📊️excel/Beat3_Program/{en,de}/*.png` (24 clauses x 2 languages, all 48 PNGs read).
Engine values checked by dumping the workbook (`🧱️model`) and the Nakagin type sheets; history checked against `T/research-nakagin.md`.

## Verdict

Not ready to publish. The content chain is correct (counts 36/35/34/35, 140, 1223.6 m², 472.0 + 751.6, 139/FALSE and 140/TRUE all match the engine), and German Excel names and separators are right. Two blockers stop it: the Program "Total" column shows `###` in the chart and update clauses, and the volume clause draws a 2.50 m height while the formula uses the 2.10 m interior height. Several majors follow: clipped headers, an invisible trailing-space typo, and the error red colliding with Type D.

Counts: 2 blockers, 9 majors, 9 minors in frame defects; 4 fact findings; 6 didactic findings.

## Frame defects

Severity: blocker / major / minor. Frames are the same in en and de unless stated.

| frame/key | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| chart | en, de | blocker | Program D5:D8 and E8 render as `###` (D4 is fine, same text length). The "Total" column of the chart clause is unreadable, so "Column D multiplies count by area" cannot be checked on screen. | Widen Program columns D and E in the model (`2,569.6 m³` needs about 12 characters); re-check whether the non-selected cells morph at the same width as D4. |
| update | en, de | blocker | Same `###` in D4:D8 and E8, so "the total stays" (D8 = 1223.6 m²) is not visible. | Same fix as chart. |
| volume | en, de | blocker | Right panel dimension reads "2.50 m" (nominal outer height, `Type A!B4`). The formula in B10 is `=B9*B7`, which uses the interior height 2.10 m (`B7`). The drawing contradicts the caption ("clear height"). | Dimension the interior height 2.10 m (pass the interior height into `capsule_axon` or label a separate clear-height dimension). |
| volume | en, de | major | No volume is drawn: the storyboard asked for a translucent interior box of 18.4 m³; the axon is an empty capsule shell. Also no reference frames on B9/B7 although the formula is shown in red and blue. | Draw the interior box (2.3 x 3.8 x 2.1 m) tinted in DATA, and add `xl_ref_frame` for B9 and B7 on `vol_win`. |
| hook, rest, pull, chart, update, countif (de) | de | major | Program B3 header "Anzahl" clipped to "Anza...". | Widen Program column B (German header). |
| countifs, towers | de | major | Header "Nettofläche" in F10 clipped to "Nettoflä...". | Widen column F, or wrap header text. |
| countifs | en | major | Header "Type A" in B10 clipped to "Type...". Also F10 "Net area" is a header over an empty column at this stage (cosmetic). | Widen column B. Hide or fill F10 consistently with the clause. |
| typo | en, de | major | Trailing space in M3 ("Type B ") is invisible. The storyboard asked for a visible marker; without it the learner cannot see why the capsule turns grey. | Draw a visible marker (e.g. a red dot or `·` after the text) on the cell and in the formula bar text. |
| types, countif, rest, typo, fix, recap, update | en, de | major | Colour collision: Type D is `P_RED`, the same colour as the error frames and FALSE/FALSCH badges (`FALSE_RED = _art.P_RED`, `TYPE_COLORS[3] = P_RED`). A learner reads red as "wrong" when Type D is red on the tower. | Give errors a non-type colour (for example a red with a cross glyph, or a dark-red border), or move Type D off red. |
| towers | en, de | major | Tower B is drawn teal (`ARCH`), the same colour as Type A in countif/rest/swap. Tower A is yellow (`DATA`), the colour of bed dimensions and the 140 counter. | Use neutral tower tints (for example light grey and dark grey) or outlines, so the type colours stay unique. |
| update | en, de | major | Trace arrows (`xl_trace` from B4 and B6) pass through the values "37", "33" and the cell text "8.7 m²", so the numbers are struck through. | Route trace arrows under the cell row, or draw them above the value and shorten them to the column edge. |
| sumproduct | en, de | major | Operand ranges B4:B7 and C4:C7 are off-window (window is rows 14–18) and not framed. The storyboard asked for both ranges framed in two colours; the frame shows only the formula text. | Use a window that contains rows 4–7 and 15–16, or add reference frames for B4:B7 and C4:C7 (they are off-window now, so the frame loop skips them). |
| hook | de | major | Caption and chip are fine; the "140" counter is correct. (No defect beyond the B3 header above.) | none |
| recap | en, de | minor | Legend chips are INK, not in their frame colours as the storyboard required; the connection to the colour frames is lost. | Colour each term chip in its frame colour (COUNTIF, XLOOKUP, ...). |
| countifs | en | minor | Formula bar text runs past the window edge (known toolkit issue, not dwelt on). | Assigned to toolkit. |
| towers | de | minor | Formula bar `=SUMMEWENN(...)` runs past the window edge (known toolkit issue). | Assigned to toolkit. |
| pull | en, de | minor | Count column B4:B7 blanks out after countifs (values 36/35/34/35 vanish) and returns in chart. The count state breaks continuity. | Keep B4:B7 in `program_book` keep list for the pull step. |
| free | en, de | minor | G16 (fitting area) is referenced in `=B9-G16` but off-window, so no frame; occupants row (B17) is shown but not highlighted although the narration names it. | Add a frame on G16 by extending the window to G, and highlight B17 at the occupants sentence. |
| update | en, de | minor | "+1" and "-1" delta chips float above the chart, far from the changed counts B4/B6. | Place the chips next to B4 and B6. |
| rest | en, de | minor | Total "140" is drawn in type-A teal (counter colour from the first clause), although it is the total of all four types. | Use a neutral colour for the total. |
| sheet, fill, free | en, de | minor | Legend chips (50 % fill) look brown-grey while the bathroom outline is peach. Colour linkage is hard to match at this size. | Use the same colour for chip and outline (full opacity chip or outline in the chip colour). |
| net | en, de | minor | "3.80 m" dimension label and "=B5*B6" card nearly touch on the right. | Move the formula card 0.3 m lower. |
| fill | en, de | minor | Legend "Bathroom unit 1.2 m²" lies close to the right edge (about 27 px margin at 1280 wide). | Shorten legend text or reduce spacing. |
| volume | de | minor | Number format popup shows `0.0 "m³"` with a decimal point in the German UI (German Excel shows `0,0 "m³"`). | Localise the format string in the popup. |
| typo | de | minor | Label "Leerzeichen am Ende" touches the chip "139 ≠ 140". | Move the label left or chip right. |
| check | en, de | minor | Check cell B18 is off-window during typo, so the FALSE/FALSCH is shown only on the tower chip, not in the sheet. | Include row 18 in the typo window. |
| update | en, de | minor | Chart title and the Type C bar value are correct, but "323.4" and "288.4" use one decimal while the tile labels in sumproduct use two decimals (8.74). Mixed precision. | Use one precision for all area displays. |
| countif, rest | en, de | minor | Criterion frame on A4 is red (reference colour `DATA`/red), the same colour as Type D and the error. | Covered by the colour-collision fix above. |
| layout.md strict | en, de | info | 21 strict findings in `hook`, `sheet`, `area`, `fill`, `free`, `table`, `countif`, `countifs`, `towers` are mostly reference-frame and dimension lines crossing cell text (by design per the scene notes). Only the `area` chip findings are real overlaps of frames with the "0,90 m" and "1,8 m²" chips. | Review the two `area` chip frames; accept the rest. |

Frames with no defect beyond those listed: program (en, de), swap (en, de), xlookup (en, de), check (en, de), fix (en, de), recap (en, de, apart from the chip colours).

## Known and already assigned (listed, not dwelt on)

- Structured table references missing (engine): formulas use `$M$2:$M$141` and `$I$2:$I$141`, not `Capsules[Type]`; the list is a plain range, not an Excel table. Plan row "Excel table" and the storyboard's `Capsules[#All]` are not realised. Narration does not claim "table", so no spoken error.
- Long formulas overflow the formula bar (toolkit): countifs (en), towers (de).

## Fact findings

1. hook (en, de): "140 capsules of four types" is stated as a fact of the building. The research (section 3.2) says the four types are a teaching grouping of six IFC variants, and only the 140 total is historical (Wikipedia, High). Fix: "In this teaching model, the 140 capsules are four types", or state the grouping in the program clause (which says "teaching model" once).
2. Counts per type 36/35/34/35 are the model values (authoritative per brief). Research section 3.4 still lists 40/36/44/20 and towers 63/77, while the model gives 36/35/34/35 and towers 54/86. The clip follows the model (correct per brief); the research file is stale and should be updated, not the clip.
3. Volume: interior 2.3 x 3.8 x 2.1 m, volume 18.354 m³ (stored), display 18.4 m³ (format `0.0`). Matches research (Archiweb, medium) and the formula. The unit is a label, not in the number. Correct.
4. Type C: window 1.6 m; the historical circular window is 1.3 m (High). The Type C sheet says "large window", but it is not labelled as a teaching variant on screen. Minor; mark "teaching variant" in the sheet title, as Type D's title already does.
5. Type D title "turned 5 degrees on its plane" is not drawn. Minor; either draw it or drop the phrase from the sheet.
6. Excel facts checked and correct: COUNTIF/COUNTIFS exact-match behaviour; trailing space not counted (34 + 35 + 35 + 35 = 139, so the check shows 139 and FALSE); XLOOKUP exact match default; SUMPRODUCT; SUMIF; check formula. German names: ZÄHLENWENN, ZÄHLENWENNS, XVERWEIS, SUMMENPRODUKT, SUMMEWENN, SUMME, UND, ZEILEN, WENN; separator `;`; decimal comma; tabs Programm, Typ A, Klima, Energie; status Bereit; FALSCH/WAHR; `#N/A` = `#NV` (not shown here). All correct.
7. Terminology mismatch: narration says "clear height" (en) and "lichte Höhe" (de), while the sheet labels read "Interior height" and "Innenhöhe". Learners see a different term than they hear. Fix the sheet label or the narration to one term.
8. The popup "Cell B10" (Stored value / Displayed / Formula / Number format) is a teaching callout, not a native Excel panel. Keep it, but do not present it as Excel UI.
9. Historical lines in this clip: none beyond the hook. Physics: U·A and H_V are not used here; no issue.

## Didactic findings

1. Smallest example first: yes (the bed, 0.9 x 2.0 m, one formula). Good.
2. Typical mistake (trailing space) is shown and fixed, but the cause is not visible (blocker-level for the lesson, major above). Fix the marker.
3. Arc: list, type sheets, one formula, fill, net, volume, four types, table, COUNTIF, fill, COUNTIFS, cross-sheet, XLOOKUP, SUMPRODUCT, chart, towers, swap, update, check, typo, fix, recap. Coherent.
4. Missing from the plan row or storyboard: COUNT vs COUNTIF misconception (storyboard list item 1 of misconceptions), pasted values freeze the result, table columns addressed by name (structured refs, known engine gap), XLOOKUP #N/A when the key is missing. Minor overall; the plan row itself is covered except the structured table.
5. Typical misconception "the tower total is a sum of the capsules counted by eye": the towers clause handles it with SUMIF; good.
6. Narration pacing: captions are at most two lines; the recap de caption is two lines. Good.
7. Teaching model is stated once (program clause). The hook claim (fact 1) needs the same qualifier.

## Fix priority

1. Program columns B, D, E, F widths (fixes ### and clipped headers at once).
2. Volume dimension to interior height 2.10 m, and draw the interior box.
3. Visible trailing-space marker; separate the error red from Type D; neutral tower tints.
4. Trace arrows not through values; sumproduct operand frames in window.
5. Hook fact qualifier; sheet term "clear height" vs "Interior height".

## Summary

- 2 blockers: Program D/E shows `###` in chart and update; volume dimension shows 2.50 m where the formula uses 2.10 m.
- 9 majors: clipped headers (B3 de, B10 en, F10 de), invisible trailing space, error red = Type D red, tower colours clash with type colours, trace arrows strike values, sumproduct operands off-window, no volume box.
- Engine values and German Excel names are correct; the hook claim of four historical types needs a teaching-model qualifier.
- Known/assigned items listed only: structured references, formula bar overflow.
- Report: `T/audit-excel-program.md`.
