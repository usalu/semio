# Morph Done: Excel / Anatomy (`Beat1_Anatomy`)

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`. Storyboard: `morph-excel-anatomy.md`. Frames: `🗑️generated/morph/📊️excel/Beat1_Anatomy/en/` (full run, 72 frames, no error) and `.../de/` (run still going when this was written; no traceback so far).

## Implemented (12 morph beats)

New scene helper `_clause(key, *plays)` replaces `step` where a clause needs several sequential plays (swarms play alone); `capture_morph.py` now detects the clause from the caption swap, so frames are named by clause.

| Clause | Beat | Dir. |
| --- | --- | --- |
| workbook | tower `proxy()` flies into a "140" number token on the title bar | C to A |
| sheets | one `silhouette()` capsule per type (A to D) flies to its tab (`scatter`, type colours carry) | C to A |
| address | blue width dimension "2.50 m" becomes the B2 chip | C to A |
| range | width, length, height dimension tokens `scatter` into B2, B3, B4 in order and colour | C to A |
| frames | interior width/length tokens (blue, red) fly to the cells B5, B6 | C to A |
| hatch | green B9 result token flies onto the floor and becomes the area badge | A to C |
| function | fitting area tokens (hatch tones) `gather` into the G16 total | C to A |
| locale | `retext` SUM(G12:G15,1.5) to the German form, EN tag to DE tag | A to A |
| trace | B5/B6 tokens travel along the precedent arrows into B9; result token travels to the dependents | A to A |
| div0, name, ref, value | the localized error code leaves its cell and lands on the capsule as a badge | A to C |
| repair | the retyped length flies from B6 back to the capsule's length line | A to C |

Frames read (en mid): workbook, sheets, address, range, formula, frames, hatch, function, locale, trace, div0, value, repair. Colours carry; tokens are readable but pale mid-flight (the cell chip fill is light, text only partly visible at the midpoint).

## Round 2 (coordinator follow-up)

- Built: precedence (tokens `B2`/`B3` fly from the formula bar onto the front and side edges, a bracket token flies to the missing fourth edge, then the perimeter turns right), recalc (a `+0.8 m²` token leaves the grown floor strip and lands on B9), name (`=SUMM(...)` retexts to `=SUM(...)`, the dropped letter vanishes), ref (the deleted row's label and value tokens fall away), legend (the window collapses into the lit Type A capsule on the tower, which pulses). legend also now uses `=SUM(G12:G15)` as sample.
- Pale tokens: every token flight now uses part-wise morphs (`_fly`, `_swarm`: chip and text travel as pairs), so chip and text stay at full opacity in flight (checked in precedence, recalc, repair frames).
- Captured only the changed keys (address range frames hatch precedence recalc name ref repair legend), en and de, no errors; frames in `🗑️generated/morph/📊️excel/Beat1_Anatomy/{en,de}/`. Read en: precedence, recalc, name, ref, legend; de frames exist but were not read.
- Not finished: `T/capture.py` keyframes (en+de) and the X presentation contract tests were started twice but made no progress under the current machine load (hours per run); I stopped my own processes. Run them when the machine is quieter: `capture.py 📊️excel 🦴️anatomy Beat1_Anatomy en|de` and `python 🧪️tests/🎭️presentation/🐍️.py` in the excel clip folder. The scene parses and plays end to end in `capture_morph`.
- Mid-frame `ref` shards and the `( )` bracket glyph (now in mono, previously drawn as square brackets) were not re-read after the last edit.

## Round 3 (audit `morph-audit-excel-anatomy.md`: FIX)

- Re-captured all 17 morph clauses en+de (workbook, sheets, address, range, frames, hatch, function, locale, precedence, recalc, trace, div0, name, ref, value, repair, legend), then address, range, hatch, precedence once more after the last edits. Frames in `🗑️generated/morph/📊️excel/Beat1_Anatomy/{en,de}/`. Read en: workbook, sheets, address, range, hatch, function, precedence, recalc, trace, name, legend; de: ref (chip no longer clipped). Remaining en/de mid frames not read one by one: div0, value, repair, German of the others.
- Chips: tokens now sit on an opaque dark chip and fly with `ReplacementTransform` (text stays readable), with a faint ghost of the origin left behind until the clause ends (address, range, frames, ship of error codes, repair, precedence, bracket).
- hatch: no ghost, opaque pill covers B9 text while flying: one value only.
- precedence: the cell selection hops to B19 only after the bracket landed and the capsule state is "right", so caption, window and capsule agree.
- recalc: the `+0.8 m²` token is born from the grown floor strip (part-wise `pairs=` morph strip to chip), then flies to B9.
- name: old window fades out completely first, then window, error and `=SUMM` text appear; tooltip fades before the in-place retext to `=SUM`.
- legend: term squares are born from the framed window parts (TransformFromCopy); window parts morph part-wise (`pairs=`: title to capsule body, Type A tab to capsule window) into a capsule at the stage, which then morphs into the lit capsule on the tower; the tower appears only after the term rows left.
- ref (de): chip shortened to "gelöscht: Innenlänge · Rückgängig".
- Still open: the window-to-capsule morph reads as a pale body next to the tower (no window detail), the ghost chips of the precedence tokens linger until the clause ends, `capture.py` keyframes en+de and the X contract tests were not run (machine load).

## Round 4 (audit 2: FIX, 15 items)

Scene changes: sheets (capsule copies fly part-wise with `capsule_pairs` to item tokens in the type colours on the tabs; four type stripes on the title bar), cell text is veiled while its token flies (address, range, frames, hatch, function, trace: no doubled text), recalc (the Δ chip is born from the grown strip as a shape and then reads "Δ +0.8 m²"; the Δ text line is gone; no yellow Indicate), locale (formula bar covered by a full-size copy that retexts EN to DE in place), div0/ref/value (old window fades out completely before the new one), tooltips kept inside the stage, legend (window shrinks into a thumbnail at the capsule position, then morphs with the toolkit's automatic part matching into the capsule, which flies to the lit capsule on the tower).

Captures (`capture_morph`, all clauses en and de once, then re-runs for the clauses changed afterwards: workbook sheets function legend en+de, recalc en+de). Frames read: en sheets, workbook, range, address, hatch, frames, function, locale, recalc (old code), trace, div0, value, repair, ref, legend; de recalc (old code), locale, name, value. Unread: de frames of the other clauses.

Open:
- legend: the window-to-capsule step still shows a grey slab in the earlier frames (explicit pairs); the final scene uses automatic matching but that frame was not re-read.
- recalc frames in `en` and `de` before the last re-run show the old yellow Indicate chip; the re-run frames (when finished) replace them.
- repair: the toolkit's cell scramble in `window.recalculate` is unchanged.
- `capture.py` keyframes and the X contract tests are still not run.
- Parallel runs of the same language clash (font cache, ParseError / PermissionError): run one capture per language at a time.

Final status (round 4): all re-runs finished without errors. en recalc re-read: the "Δ +0.8 m²" chip is born from the grown floor strip and lands readable, no yellow flash. Legend: frame `legend-06-mid` still shows the grey slab morphing into the capsule even with automatic part matching, so that step remains open (needs a toolkit-side window-to-capsule recipe or a simpler cut: thumbnail, then fade into the tower capsule). `capture.py` keyframes and X contract tests are still not run.

## Legend decision applied (final)

Per the coordinator: the window is no longer morphed into a capsule. It recedes (scale down and fade) while the Type A tab detaches as a teal item token and flies part-wise (`capsule_pairs` reversed: ring to shell, dot to window) into the lit Type A capsule on the tower, which then pulses. Captured en and de, mid frames `legend-05` and `-06` read: window gone cleanly (no leftover chrome), token readable, landing on the lit capsule. This closes the sheet to capsule-type arc. Remaining open: `capture.py` keyframes and X contract tests (not run), and the toolkit's cell scramble in `repair`.

## Lessons pass (12 general lessons of `morph-review-excel-references.md`)

| # | Lesson | Anatomy scene |
| --- | --- | --- |
| 1 | one actor, no side-card duplicates | OK: the token is the cell value or dimension itself; cards (pop) stay captions |
| 2 | persist the bound pair | FIXED: error badges on the capsule, the "Δ +0.8 m²" chip and the origin ghosts now stay through the narration hold and fade only after it (`_after`, `_ghosts`, removed with `scene.remove`); area badge, dimension labels and cell values already persist |
| 3 | move the number, then tint | OK for hatch (pill flies, badge and hatch at landing); floor tint is set slightly before the flight (minor) |
| 4 | no tracker sweep as morph | OK: the only ValueTracker is the continuous interior width in recalc |
| 5 | show the event | partly: the deleted row's label and value fall away (ref); delete/shatter is simple tokens, not a shard effect |
| 6 | structure first, then morph | OK: window swaps are separate plays before every morph (div0, ref, value, name, function, trace, legend) |
| 7 | legible morphs | OK: opaque dark chips with full text and colour, one flying group per clause, at most four flyers (sheets) |
| 8 | check at reading size | mid frames are 640x360; text read there; not re-read at 1280x720 |
| 9 | both directions, chained | OK: C to A (tower, capsules to tabs, dimensions to cells) and A to C (result to hatch, error codes, repair, legend tab to lit capsule) |
| 10 | match narration | OK: each key event happens inside its clause (hold after the plays) |
| 11 | back-link | OK: hatch, repair, error badges, legend (tab flies back into the lit capsule) |
| 12 | toolkit hygiene | OK: `capsule_pairs`, `pairs=`, `gather`/`scatter`, real copies; the bracket token is the only anonymous glyph token |

Captured after the pass (en and de, changed keys): address, precedence, recalc, div0, name, ref, value, repair. Read: error badges stay on the capsule during the clause and are gone by `repair`; with the toolkit's in-place retype `repair` now shows readable cells with a green flash (en and de). Still not run: `capture.py` keyframes and the X contract tests.

## Final polish (morph-final-excel-anatomy.md: SHIP)

- workbook: the tower silhouette copy (outline kept) flies into a "Nakagin.xlsx" file token on the title bar, then a "140 capsules" / "140 Kapseln" chip is copied off it (`TransformFromCopy`); the yellow Circumscribe is gone; the file token fades, the chip stays until the tabs.
- display: the displayed "8.7 m²" (from B9, whose text is veiled) glyph-morphs (`retext`) into the stored value "8.74" at the Stored row of the card and back into "8.7 m²" at the cell.
- precedence: the bracket is now the real `()` glyph pair (formula font, green) taken from the formula bar and flown to the missing edge.
- Captured workbook, display, precedence en+de without errors; read en workbook-04/05, display-04, precedence-06: file token lands on the title bar, the card shows 8.74 with the morphing chip, bracket glyph on the capsule edge. German frames written, not individually read. Still not run: `capture.py` keyframes and the X contract tests.

Bracket tweak: the precedence bracket is now a bold `(  )` glyph at 2.4x formula size, full opacity, on a filled dark chip with a green outline (opaque plate, ReplacementTransform keeps it at full opacity in flight). Recaptured en+de and read mid frames 05 and 06 in both languages: the chip reads as a bracket pair leaving the formula bar and landing on the capsule's missing edge.
