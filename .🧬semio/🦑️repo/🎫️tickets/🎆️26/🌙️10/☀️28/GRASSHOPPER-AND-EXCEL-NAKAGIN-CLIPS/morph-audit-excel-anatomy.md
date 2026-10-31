# Morph Audit: Excel / Anatomy (`Beat1_Anatomy`), read-only

Inputs: `morph-excel-anatomy.md` (planned beats), `morph-excel-anatomy-done.md` (built), scene `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`, frames `T/🗑️generated/morph/📊️excel/Beat1_Anatomy/{en,de}/` with `index.md`.

Frame base: `T/🗑️generated/morph/📊️excel/Beat1_Anatomy/en/` (abbreviated `en/` below, `de/` for German).

Frames on disk cover only clauses address, range, frames, hatch, precedence, recalc, name, ref, repair, legend. Clauses workbook, sheets, function, locale, trace, div0 and value have no frames in `en/` or `de/`. The done file reports a full run of 72 frames, which is no longer on disk (the later partial capture overwrote the folder). Nothing was captured in this audit.

## 1. Per planned beat

| # | Clause | Verdict | Evidence |
| --- | --- | --- | --- |
| 1 | workbook (tower to title "140") | MISSING | no `workbook-*` frames |
| 2 | sheets (capsules to tabs) | MISSING | no `sheets-*` frames; the capsule-to-tab chain cannot be checked |
| 3 | address (dimension to B2) | WEAK | `en/address-03-mid.png`: chip "2.50 m" in flight is pale; no tether or ghost visible |
| 4 | range (dimensions to B2:B4) | WEAK | `en/range-03-mid.png`: B3 and B4 tokens faded; height token nearly invisible |
| 5 | frames (interior lines to B5, B6) | BUILT | `en/frames-03-mid.png`: "2.30 m" blue and "3.80 m" red readable, colours carried. The formula `=B5*B6` already shows in the formula bar before the tokens land |
| 6 | hatch (B9 result to floor) | WEAK | `en/hatch-03-mid.png`, `de/hatch-03-mid.png`: green "8.7 m²" pill sits on top of the B9 cell text, so the value appears twice. `en/hatch-04-mid.png`: badge on floor, no visible origin ghost |
| 7 | precedence (tokens to edges, bracket, 13.00 m) | BUILT, with fixes | `en/precedence-04-mid.png`: B2/B3 chips readable at edges. `en/precedence-06-mid.png`: green "( )" readable. Stale annotation "without brackets: one side missing / =2*B2+B3 = 9.00 m" still on the capsule in `precedence-05` and `-06`, while the window already shows B19 `=2*(B2+B3)` |
| 8 | function (fittings to G16) | MISSING | no `function-*` frames |
| 9 | locale (SUM to SUMME, retext) | MISSING | no `locale-*` frames |
| 10 | recalc (wall grows, delta chip) | WEAK | `en/recalc-03-mid.png`: wall move and "Δ +0.8 m²" readable, but the delta chip is text-only; no token flight from the hatch strip is visible |
| 11 | trace (precedents and dependents) | MISSING | no `trace-*` frames |
| 12a | div0 (B6 empty, edge to `?`, #DIV/0! badge) | MISSING | no `div0-*` frames |
| 12b | name (SUMM to SUM) | WEAK | `en/name-03-mid.png`: old window (DIV/0) cross-fades under the new window, text overlaps (e.g. "Interior length" over "Fitting area"). `en/name-06-mid.png`: #NAME? tooltip and error still shown while the formula reads SUM. The retext is not visible as a morph |
| 12c | ref (row deleted, `?` edge, #REF! badge) | BUILT, with fix | `en/ref-04-mid.png`: dashed red `?` edge and chip. `en/ref-05-mid.png`: "#REF!" badge in flight to the capsule, readable. Shards from the deleted row are not visible. `de/ref-04-mid.png`: chip "gelöscht: Innenlänge · Rückgängig stellt sie her" runs under the window edge and is clipped |
| 12d | value (unit typed, `3.80 m` shard) | MISSING | no `value-*` frames |
| 12e | repair (retyped 3.80 m back to edge) | BUILT | `en/repair-03-mid.png`: "3.80 m" token readable mid-flight, landing on the length line |
| 13 | legend (window into lit capsule, terms from parts) | WEAK | `en/legend-05-mid.png`: the morphing window is a grey blob with a yellow outline, not readable as window or capsule. `en/legend-04-mid.png`: term rows (x about 2.3 to 6.8) overlap the tower at right, labels "function" and "Name Box" are cut by the tower. Term chips are FadeIn (`legend-03`), not born from their parts |

Counts: BUILT 3 (frames, precedence with fixes, repair), ref BUILT with a DE clip fix, WEAK 6 (address, range, hatch, recalc, name, legend), MISSING 7 (workbook, sheets, function, locale, trace, div0, value).

## 2. Remaining fade-swaps and cuts (from `en/index.md`)

Where a morph would show identity:

- `legend-03` LaggedStart of FadeIn(term row) and FadeIn(target): term chips should be born from their framed parts.
- `legend-04` FadeOut(panel) and FadeIn(sky tower): the window-to-capsule return is split between a fade and a morph (`legend-05`).
- `frames-04` FadeIn(cell frames): the tokens should settle into the frames, not fade in beside them.
- `address-04` FadeIn(frame) on B2 and `range-02`/`frames-02` FadeIn: the B2 frame should grow from the dimension line.
- `name-01` to `name-03` FadeOut(window6) and FadeIn(window7): a cross-fade where the SUMM letter should drop out of the formula.
- `precedence-01` to `-03`, `recalc-01` to `-03`, `ref-01` to `-03`, `repair-01`, `-02`: whole-window FadeOut and FadeIn swaps. These are scene cuts between concrete and abstract only where a part should carry identity; acceptable as window changes if the part morph is clean.
- `hatch-01`, `hatch-02`: FadeOut and FadeIn around the result token.
- Not verifiable without frames: `workbook`, `sheets`, `function`, `trace`, `div0`, `value`.

## 3. Direction and chains

- Concrete to abstract present: address, range, frames, ref (error badge), and sheets/function/workbook once re-captured.
- Abstract to concrete present: hatch, precedence bracket, repair, legend (window to capsule, `legend-05`).
- Both directions occur. Chain (a) capsule to tab to B2 to B2:B4 to tokens to floor: links 3 to 6 are visible, but the tab link (beat 2) cannot be checked. Chain (b) length line to B6 token to precedent to `?` edge to repaired line: links to `?` and repaired line are visible; the trace link (beat 11) is missing. Chain (c) fittings to G12:G14 to SUM to total: all missing.
- No frame shows the origin ghost or tether described in the storyboard. No identity tether was found in any frame.

## 4. Collisions and clutter

- `hatch-03` (en and de): green pill over B9 text.
- `precedence-04`: stray red outline on the window at A13 (left of the Annual heating demand row).
- `precedence-05` and `-06`: stale capsule annotation contradicts the window.
- `name-03`: old and new windows overlap.
- `legend-04`: term rows overlap the tower.
- `de/ref-04`: chip clipped by the window.

## Verdict: FIX

Fix list:

1. Re-capture the missing clauses in both languages: workbook, sheets, function, locale, trace, div0, value. Use `capture_morph` for `Beat1_Anatomy` (`en` and `de`). Without them, chains (a) and (c) cannot be verified.
2. `hatch`: fade the B9 cell label (or hide it) for the duration of the flight, so the pill is the only "8.7 m²".
3. `precedence`: when `perim` becomes "right", replace the "without brackets" caption and the "=2*B2+B3" line with the "right" text before the bracket flies in. Remove the stray red frame at A13.
4. `name`: fade out window6 completely before window7 appears, or morph the SUMM letter in place. Hide the #NAME? tooltip before the retext.
5. `legend`: move the term rows clear of the tower (for example left of x 4.0, or scale the tower down further). Give the window-to-capsule morph a visible silhouette (scale the body to the capsule size, add a ghost). Replace `legend-03` FadeIn of rows with a morph from each framed part.
6. `recalc`: morph the "Δ +0.8 m²" chip from the grown hatch strip (currently text-only).
7. `address` and `range`: keep chip text opaque in flight (raise the chip fill) and add a faint tether from each dimension line to its cell.
8. `frames`: replace FadeIn(frames) with the tokens settling into the frames.
9. `ref` (de): shorten the "gelöscht: ..." chip or place it clear of the window.
10. Add a faint origin ghost and tether to the morphing beats per the storyboard (`Identity ghost` note), so origin is visible mid-frame.

## Summary

1. Frames exist for 10 of 17 clauses; 7 are MISSING (workbook, sheets, function, locale, trace, div0, value) and must be re-captured.
2. Built and readable: frames, precedence (with a stale caption), ref (en), repair.
3. Weak: address, range, hatch (double value), recalc (delta chip), name (cross-fade), legend (blob and tower overlap).
4. Remaining fade-swaps: legend term rows, frames FadeIn, and whole-window swaps; both directions and the chain are present but chains (a) and (c) are not yet visible.
5. Verdict FIX. Full fix list above; written to `T/morph-audit-excel-anatomy.md`.
