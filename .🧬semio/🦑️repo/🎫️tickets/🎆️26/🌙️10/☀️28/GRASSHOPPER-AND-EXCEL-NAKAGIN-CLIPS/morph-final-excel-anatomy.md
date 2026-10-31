# Final Morph Review: Excel / Anatomy (`Beat1_Anatomy`)

Judged from the scene code (`📊️excel/🎬️clip/🦴️anatomy/🐍️.py`), storyboard, done-log and the en mid frames (hook, workbook, sheets, range, frames, hatch, recalc, div0, legend). German frames were not read.

## Per clause

| Clause | Verdict | Why |
| --- | --- | --- |
| hook | ACCEPTABLE | Tower with one lit capsule and leader; establishes the concrete origin, no morph needed. |
| workbook | WEAK | Tower `proxy` morphs into a "140" token on the title bar, but the mid frame (workbook-04) shows only a yellow smear on the title bar; the viewer cannot read "this building became the file", and the narration says nothing about 140. |
| sheets | GOOD | Four capsule copies fly part-wise (`capsule_pairs`) onto the tabs in type colours; a real identity transfer (sheets-03 shows it). |
| grid | ACCEPTABLE | Circumscribe of headers; no concrete counterpart needed. |
| address | GOOD | Blue width dimension becomes the B2 value, cell veiled while flying, frame persists. |
| range | GOOD | Three dimension tokens (blue/red/height tone) fly to B2:B4 in order; colours carry. Mid frame is slightly crowded around B2. |
| bars | ACCEPTABLE | Callouts to Name Box and formula bar; a pure pointing clause. |
| constant | ACCEPTABLE | Selection hop only, no object change. |
| display | WEAK | Value versus display is explained by a pop card with rows; no morph of 8.7 m² into its exact stored number or back to the floor. Idea is abstract-only. |
| formula | ACCEPTABLE | Formula text appears in the cell; the floor becomes grey interior, but the formula is not born from the floor. |
| frames | GOOD | Interior width and length dimension tokens fly into B5 and B6, frames match the dimension colours (frames-04 confirms). |
| hatch | GOOD | Green B9 token flies onto the floor and lands as the area badge with hatch; the cell is veiled, so one value only. A-to-C back-link is visible. |
| precedence | ACCEPTABLE | Edge tokens and a "( )" glyph fly to the capsule edges; the bracket is the only anonymous token, but the missing fourth edge is a real concrete-abstract link. |
| function | GOOD | Fitting area chips (hatch tones) gather into the G16 total. |
| locale | GOOD | In-place `retext` of the formula text EN to DE. |
| recalc | GOOD | Interior widens continuously; the new floor strip morphs into the "Δ +0.8 m²" chip (recalc-05 shows it born from the strip). |
| trace | ACCEPTABLE | Value tokens travel along the precedent and dependent arrows; the idea is explained by travel more than by identity. |
| div0 | GOOD | Error code leaves its cell and lands on the capsule next to the "?" length (div0-04); the empty length is shown dashed. |
| name | GOOD | `=SUMM` retexts to `=SUM`, plus error code to capsule. |
| ref | ACCEPTABLE | Deleted row label and value tokens fall away; simple tokens, no real shard effect, but the event is shown. |
| value | ACCEPTABLE | Error ships to the capsule; the unit-in-cell cause is only in the fix chip. |
| repair | GOOD | Retyped length flies from B6 to the length line; cells recalculate. |
| legend | ACCEPTABLE | Window recedes, Type A tab flies as a token into the lit capsule on the tower (closing the sheet-to-capsule arc); the term rows are born from framed window parts. The mid frame (legend-06) shows a very small token and an almost empty stage; the landing is easy to miss. |

## Overall

Both directions are present and chained: tower to file (C to A), capsules to tabs, dimensions to B2/B3/B4 and B5/B6, then area to floor (A to C), errors to capsule, repaired length back, tab back into the tower. Part-wise morphs with persistent partners (veiled cells, dimension labels, ghosts) follow the general lessons. The remaining gaps are in the two opening clauses and display, which are secondary ideas.

**Verdict: SHIP** (morph-driven where ideas are explained; minor polish only). Optional polish, highest impact first:

1. `workbook` (code lines 345-347): replace the proxy-to-"140" morph by the four type stripes or a "Nakagin.xlsx" file token growing out of the tower so the identity (building to file) is readable; at minimum make the "140" chip opaque and keep the tower as a ghost.
2. `legend` (lines 519-521): enlarge the flying tab token (about 1.5x) and linger on the lit capsule so the closing arc reads at 640x360.
3. `display` (lines 387-396): morph the floor badge "8.7 m²" into the exact stored number "8.74 m²" inside the cell pop to show format versus value.
4. `precedence` (lines 428-432): replace the anonymous "( )" token by the real bracket glyph copied from the formula bar.
5. `ref` (lines 494-497): shatter the deleted row's tokens into shards falling to `#REF!`.
