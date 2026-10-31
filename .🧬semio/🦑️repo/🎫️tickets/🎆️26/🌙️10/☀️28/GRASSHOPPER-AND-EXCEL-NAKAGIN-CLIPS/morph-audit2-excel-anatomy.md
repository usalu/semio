# Morph Audit 2: Excel / Anatomy (`Beat1_Anatomy`), read-only

Second round, against `morph-audit-excel-anatomy.md` (first audit, 15:39).

Inputs: planned beats `morph-excel-anatomy.md`, built `morph-excel-anatomy-done.md`, scene `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py` (last edit 15:53:00), frames `🗑️generated/morph/📊️excel/Beat1_Anatomy/{en,de}/` (mid frames read: en and de, all clauses; `index.md` read in both languages).

Stale check: cutoff = 15:53 minus 60 min = 14:53. Every frame on disk is from 15:45 or later, so no frame is stale and all were judged.

Caveat: the scene was last edited at 15:53, after the captures of workbook, sheets, function, locale, frames, recalc, trace, div0 and value (15:47 to 15:55, most at 15:47 to 15:53). Those frames may not show the final scene. Re-capture after the next scene edit.

Nothing was captured in this audit.

## 1. Per earlier beat (first audit verdict, now)

| # | Clause | First audit | Now | Evidence (frame base `🗑️generated/morph/📊️excel/Beat1_Anatomy/`) |
| --- | --- | --- | --- | --- |
| 1 | workbook | MISSING | BUILT | `en/workbook-04-mid.png`: yellow "140" chip lands on the title bar; `de/workbook-04-mid.png` same. Type-colour stripes on the title bar not visible. `en/workbook-03-mid.png`: grey tower ghost laid over the dimmed window, reads as an overlay, not a morph. `en/workbook-05-mid.png`: tower ghost still beside the window |
| 2 | sheets | MISSING | STILL WEAK | `en/sheets-03-mid.png`, `de/sheets-03-mid.png`: capsule silhouettes are grey, not the type colours (teal, orange, green, red). The orange Type B tab outline appears, but the capsule-to-tab link is not clear. Pale blob on cell B9 (`en/sheets-03`). `en/sheets-04-mid.png`: tabs highlight in yellow text (Indicate), Climate and Energy slide-in not visible |
| 3 | address | WEAK | FIXED (03), STILL WEAK (04) | `en/address-03-mid.png`: chip "2.50 m" is opaque and readable, a faint "2.50 m" ghost stays on the capsule dimension line. `en/address-04-mid.png`: the chip and the yellow Indicate highlight sit on top of the B2 value, so "2.50 m" shows twice. No tether line in any frame |
| 4 | range | WEAK | STILL WEAK | `en/range-03-mid.png`, `de/range-03-mid.png`: B2 and B3 chips cover cell values, the old value shows beside the chip. Height chip (B4) is grey and nearly invisible. `en/range-04-mid.png`: "2.50 m 2.50 m" and "4.00 m 4.00 m" double text in B2 to B4 |
| 5 | frames | BUILT | BUILT, with overlap | `en/frames-03-mid.png`: "2.30 m" blue and "3.80 m" red readable, origin ghosts on the capsule. `en/frames-04-mid.png`: the tokens land on B5 and B6 on top of the cell values ("2.30 m2.30 m"). Index for frames is absent (see section 2) |
| 6 | hatch | WEAK | STILL WEAK (03), BUILT (04) | `en/hatch-03-mid.png`, `de/hatch-03-mid.png`: green pill "8.7 m²" covers the B9 text but the remainder "m²" still shows beside it. `en/hatch-04-mid.png`: pill lands on the floor as a badge, B9 is clean. The pill itself is readable |
| 7 | precedence | BUILT, stale caption | FIXED | `en/precedence-05-mid.png`, `-06-mid.png`: caption "without brackets: one side missing" matches the window (B18 `=2*B2+B3`). `en/precedence-07-mid.png`: window moves to B19 `=2*(B2+B3)` and caption switches to "with brackets", in step. `en/precedence-04-mid.png`: the B3 chip floats over grid rows 14 to 16 (clutter). Stray red outline at A13 is not in the frames (rows 14 to 20 shown) |
| 8 | function | MISSING | BUILT | `en/function-04-mid.png`: "Bed 1.8 m²" grey, "Bathroom unit 1.2 m²" orange, "Storage 0.6 m²" tones match the floor hatches; "Σ 3.6 m²" chip readable. `en/function-05-mid.png`: "3.6 m²" pill sits over the G16 text (double value) |
| 9 | locale | MISSING | STILL WEAK | `en/locale-03-mid.png`, `en/locale-04-mid.png`, `de/locale-04-mid.png`: retext runs in a small grey chip ("=SUM1 (G12:G15;1,5)" mid-glyph) at the right. The formula bar and cell stay "=SUM(G12:G15" the whole time, so SUMME never appears in the sheet. Chip text is pale |
| 10 | recalc | WEAK | STILL WEAK | `en/recalc-04-mid.png`, `-05-mid.png`: the "Δ +0.8 m²" is still a text block (with B5, B9, B10 lines) at the stage. The green "+0.8 m²" chip overlaps "9.5 m²" in B9 (double text). The floor strip does grow, and a "9.5 m²" token lands on the floor |
| 11 | trace | MISSING | BUILT, with clutter | `en/trace-04-mid.png`: "3.80 m" chip rides the blue precedent arrow into B9, arrows coloured blue (precedent) and orange (dependent). `en/trace-06-mid.png`, `-07-mid.png`: "8.7 m²" chips land on top of B10, B15 and B16 text |
| 12a | div0 | MISSING | BUILT, with overlap at 03 | `en/div0-03-mid.png`: old window still fading under the new one (text ghosts "Interior length" over "Front wall"). `en/div0-04-mid.png`: "#DIV/0!" badge leaves B15 and lands on the capsule, red dashed "?" edge visible. `en/div0-05-mid.png`: badge on capsule, B15 error. Tooltip "The formula divides by zero" spills over the window edge at x 412 to 625 |
| 12b | name | WEAK | FIXED | `en/name-03-mid.png`: old window (DIV/0) fades out alone, no overlap. `en/name-04-mid.png`, `-05-mid.png`: new window with "#NAME?" in G16 and capsule badge, formula bar "=SUMM(G12:G15)". `en/name-06-mid.png`: formula bar reads "=SUM(G12:G15)" in place, error and tooltip still shown. The dropped M is not caught mid-letter in any frame |
| 12c | ref | BUILT, de chip clipped | FIXED | `en/ref-04-mid.png`: dashed red "?" edge, chip "deleted: interior length · undo", tooltip spills over the window edge. `en/ref-05-mid.png`: "#REF!" badge lands on the capsule. `de/ref-04-mid.png`: chip "gelöscht: Innenlänge · Rückgängig" fits, no longer clipped. Shards from the deleted row not visible |
| 12d | value | MISSING | BUILT, with overlap at 03 | `en/value-03-mid.png`: old and new windows overlap (ghost labels "Interior width", "Net floor area" over the new cells). `en/value-04-mid.png`, `-05-mid.png`: "#VALUE!" badge on B9 and on the capsule, "3.8 m" shard in B6, fix chip. `de/value-04-mid.png`: "#WERT!" badge and German caption correct |
| 12e | repair | BUILT | BUILT, cells scrambled mid-flight | `en/repair-03-mid.png`, `-04-mid.png`: "3.80 m" chip flies to the capsule length line and reads. The cells B9 to B13 scramble into unreadable glyphs at the same time |
| 13 | legend | WEAK | STILL WEAK (window to capsule), term rows FIXED | `en/legend-04-mid.png`, `de/legend-04` (not read; `de/legend-05` checked): term rows (workbook, worksheet, cell, reference, formula, function, Name Box, formula bar) sit right of the window, no tower overlap, labels readable. `en/legend-05-mid.png` and `de/legend-05-mid.png`: the window collapses into a grey parallelogram blob with a green dot, not a window or a capsule. `en/legend-06-mid.png`: capsule on the tower is readable afterwards |

Counts for the earlier beats: FIXED 4 (address, precedence, name, ref), BUILT 8 (frames, hatch 04, function, trace, div0, value, repair, workbook), STILL WEAK 6 (sheets, range, hatch 03, recalc, locale, legend window morph). Of the 7 earlier MISSING beats, 5 are now BUILT (workbook, function, trace, div0, value) and 2 are STILL WEAK (sheets, locale).

## 2. Remaining fade-swaps and cuts

`en/index.md` and `de/index.md` (both written 16:07 to 16:08) list only address, range, hatch and precedence. They are stale for the other 13 clauses captured at 15:47 to 15:56, so the animation types of those clauses cannot be checked from the index. From the index (same in en and de):

- address-01, address-02 FadeOut and FadeIn of the window; address-04 FadeIn of the frame plus Indicate on B2 (double value, see table).
- range-01, range-02 FadeOut and FadeIn of the window; range-04 FadeOut of chips while cells still show.
- hatch-01, hatch-02 FadeOut and FadeIn around the result; hatch-03 FadeOut plus Indicate on B9.
- precedence-01, -02, -03 whole-window FadeOut and FadeIn; precedence-05 and -08 FadeOut.

Not verifiable from the index: frames (FadeIn of frame cells, per the first audit), legend-03 (FadeIn of term rows; the rows now look framed but the birth is not visible), sheets, locale, workbook, function, trace, div0, name, ref, value, repair.

Whole-window swaps (old window FadeOut, new window FadeIn) remain between div0 and value, and between name windows. They are acceptable as window changes only where a part morph shows identity; in value-03 and div0-03 they overlap (see section 4).

## 3. Direction and chains

- Concrete to abstract present: workbook (tower to "140"), sheets (capsules to tabs, silhouette in grey), address, range, frames (dimension tokens to B2, B2:B4, B5, B6), function (floor hatches to G16 total), trace (dimension token along the arrow).
- Abstract to concrete present: hatch (B9 result to floor badge), precedence (tokens and bracket to the edges), div0, ref and value (error badges from the cell to the capsule), repair (token back to the length line), legend (window to capsule, weak).
- Both directions occur. Locale is A to A (retext).
- Chain (a), capsule to tab to B2 to B2:B4 to tokens to floor: tab link is only partly visible (sheets-03 shows the Type B capsule, the Type A tab link is not seen); B2, B2:B4, B5/B6 and floor are visible. Partly intact.
- Chain (b), length line to B6 token to precedent arrow to `?` edge to repaired line to term chip: trace-04 (chip along the arrow), div0-03/ref-04 (`?` edge), repair-03 (chip to length line), legend-04 (term row) are all visible. Intact.
- Chain (c), fittings to G12:G14 to SUM to total: function-04 shows tones carried to the total pill and the Σ chip. Intact, but the fittings do not visibly leave their floor hatches; the link is by colour only.

## 4. Collisions and clutter

- `en/hatch-03`, `de/hatch-03`: pill over B9, "m²" remainder visible.
- `en/range-03`, `de/range-03`, `en/range-04`: chips over B2 to B3 cell values, double text.
- `en/address-04`: chip plus Indicate highlight over B2 value.
- `en/frames-04`: tokens over B5 and B6 values ("2.30 m2.30 m").
- `en/recalc-04`, `-05`: "+0.8 m²" chip over the B9 "9.5 m²" text; text block of Δ lines at the stage corner.
- `en/trace-06`, `-07`: "8.7 m²" chips over B10, B15 and B16 text.
- `en/function-05`: "3.6 m²" pill over G16 text.
- `en/sheets-03`: pale blob over cell B9.
- `en/workbook-03`, `-05`: tower ghost over the dimmed window.
- `en/div0-03`, `en/value-03`: old and new windows overlap, ghost text readable through the new window.
- `en/repair-03`, `-04` (and `de`): cells B9 to B13 scrambled into glyphs mid-flight.
- `en/precedence-04`: B3 chip floats over grid rows 14 to 16.
- `en/div0-04`, `en/ref-04`: tooltip spills over the window edge.
- `en/locale-03`, `-04`: pale grey chip, low contrast.

## Verdict: FIX

Fix list (exact, short):

1. `sheets`: colour each silhouette by its type (A teal, B orange, C green, D red) and slide Climate and Energy tabs in; remove the B9 blob.
2. `locale`: retext the formula bar and the cell (G16) in place, and raise the chip text contrast.
3. `legend`: give the window-to-capsule morph a readable silhouette (keep the window frame and tab shapes while shrinking to the capsule outline) in `en/legend-05` and `de/legend-05`.
4. `recalc`: replace the Δ text block with a chip born from the grown hatch strip, and keep the chip off the B9 text.
5. `range` and `address`: hide the cell value while its chip flies (no double text); raise the B4 height chip fill; add the tether line from the dimension line to the cell.
6. `frames`: hide the B5 and B6 values while their tokens settle (no "2.30 m2.30 m").
7. `hatch`: hide the rest of the B9 text ("m²") during the pill flight.
8. `trace`: land dependent chips beside the cell, not on its text (B10, B15, B16).
9. `function`: move the G16 pill off the cell value.
10. `workbook`: add the type-colour stripes to the title bar and fade the tower ghost out before it crosses the window.
11. `precedence`: move the B3 chip off grid rows 14 to 16 in `en/precedence-04`.
12. `repair`: keep B9 to B13 readable during the glyph scramble, or do not scramble cells not in the morph.
13. `div0`, `value`: fade the old window out completely before the new one (no overlap in `div0-03`, `value-03`).
14. Tooltips: keep the error tooltip inside the stage (`div0-04`, `ref-04`).
15. Re-capture all clauses en and de after the 15:53 scene edit (workbook, sheets, function, locale, frames, recalc, trace, div0, value). Regenerate `index.md` so it covers all clauses, not only address, range, hatch and precedence.

## Summary

1. Frames exist for all 17 clauses in en and de; no MISSING beat is left.
2. Fixed since round 1: address opacity, precedence stale caption, name cross-fade, ref German clip.
3. Still weak: sheets (grey capsules), range and hatch (double text), recalc (text delta), locale (chip only), legend (window-to-capsule blob).
4. Fade-swaps remain in address, range, hatch and precedence; the index does not cover the other 13 clauses, so their swaps are unverified.
5. Verdict FIX. Fix list above; the scene edit at 15:53 post-dates the workbook to value captures, so those need re-capture.
