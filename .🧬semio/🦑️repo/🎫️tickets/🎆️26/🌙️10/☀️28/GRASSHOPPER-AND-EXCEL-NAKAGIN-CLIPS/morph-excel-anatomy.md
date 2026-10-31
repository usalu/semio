# Morph Storyboard: Excel / Anatomy (`Beat1_Anatomy`)

Scene `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`. Plan: `morph-plan.md`. Toolkit names (`morph`, `scatter`, `gather`, `split`, `token`, `travel`, `tether`, `unfold`, `retext`, `proxy`, `silhouette`) are from `morph-plan.md` section "Shared Morph Toolkit"; exact signatures follow `api.md` section 7 when it is `Status: ready`.

## Thread

One object is followed through the whole video: **the lit Type A capsule**. It leaves the tower as the "Type A" tab (beat 2), its width becomes cell B2 (3), its dimensions become B2:B4 (4), its interior sides become the tokens `B5` and `B6` (5), the result 8.74 flows back into the floor hatch (6), the formula tokens grow and lose the fourth edge (7), the fittings collapse into a SUM (8), a cell value drags the wall outward (9), precedents and dependents leave and return (10), errors break the same edge (11), and in the legend the window collapses back into the lit capsule on the tower (12). Colours are fixed: width blue, length red, height beige, area green, errors red-orange; the type colour of A (teal) is kept by the tab.

## Clauses (21, every one en+de, captions at most two lines)

`workbook` and `sheets` are merged into one clause and `range` and `bars` into one clause (pace item 7 of the quality review); all other keys stay. New texts:

| Key | en | de |
| --- | --- | --- |
| workbook (merged) | A workbook is the whole file, Nakagin.xlsx; its worksheets are the tabs: Program, one sheet per capsule type, Climate and Energy. | Eine Arbeitsmappe ist die ganze Datei, Nakagin.xlsx; ihre Arbeitsblätter sind die Register: Programm, je Kapseltyp ein Blatt, Klima und Energie. |
| range (merged) | A range joins cells: B2:B4 holds width, length and height. The Name Box shows the address, the formula bar the content. | Ein Bereich fasst Zellen zusammen: B2:B4 enthält Breite, Länge und Höhe. Das Namenfeld zeigt die Adresse, die Bearbeitungsleiste den Inhalt. |

All others keep their current NARRATION lines (hook, grid, address, constant, display, formula, frames, hatch, precedence, function, locale, recalc, trace, div0, name, ref, value, repair, legend). The German `value` line changes "wenn" to "wo" (sweep fix 2). The EN tab strip must contain Energy (sweep fix 1), checked in Phase C.

## Morph beats (12)

Direction: C→A concrete to abstract, A→C abstract to concrete. Durations include the staggering.

| # | Clause | Concrete object (Nakagin / drawing) | Abstract object | Dir. | Carries over | Time | Narration while it happens |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | workbook | all 140 capsules of the tower, `proxy()` per tower with type colours | the window title "Nakagin.xlsx" with the chip "140" | C→A, `gather` (many to one) | type colours become the four thin colour stripes of the title bar, count 140 | 1.6 s | "A workbook is the whole file, Nakagin.xlsx" |
| 2 | workbook | one capsule of each type (A teal, B orange, C green, D red), `silhouette()` lifted from the tower | the tabs Type A, Type B, Type C, Type D; Program ← the tower outline, Climate and Energy tabs slide in from the right with a ghost dot | C→A, `scatter` with LaggedStart (lag 0.25) | type colour of each tab underline; the lit Type A capsule stays as ghost over the tab (tether) | 2.0 s | "its worksheets are the tabs: Program, one sheet per capsule type, Climate and Energy" |
| 3 | address | the width dimension line of the Type A capsule (blue arrow with `2.50 m`) | cell B2 with value `2.50 m` | C→A, `morph` + `tether` | blue colour, the number and unit text; a blue frame stays on B2 and the dimension line keeps a faint copy on the capsule | 1.4 s | "B2 is the width of the capsule, shown with its unit" |
| 4 | range | the three dimension lines width (blue), length (red), height (beige) around the capsule | the cells B2, B3, B4 stacked in the window with the range border B2:B4 | C→A, `scatter` in the order width, length, height | colour per quantity, order of the lines is the order of the rows, units | 1.6 s | "B2:B4 holds width, length and height" |
| 5 | formula | the interior dimension lines of the floor (blue width, red length) | the reference tokens `B5` (blue) and `B6` (red) inside the typed formula `=B5*B6` in the cell and the formula bar | C→A, `morph` per line, then `travel` of each token to its cell, where it settles as the cell frame | the blue and red colour, the order, the numbers 2.30 m and 3.80 m shown beside the tokens | 1.8 s (formula), 1.4 s (frames) | "The floor area is interior width times interior length: B5 times B6" and "Excel colours every reference and frames its cell" |
| 6 | hatch | the grey floor outline turns into the green hatched floor | the result 8.74 in cell B9 | A→C, `morph` of the result number (green `8.7 m²` token) out of B9 into the hatch and the area badge; `split` into hatch lines | green colour, the value 8.74 m² as badge number, position order of hatch lines | 1.6 s | "The result is the floor of the capsule, drawn as a hatched surface" |
| 7 | precedence | the capsule outline with the perimeter edges: two blue sides (width), two red sides (length) | the formula tokens `2*B2` and `+B3` in the formula bar and the result `9.00 m` | C↔A both ways: tokens `B2` and `B3` `travel` onto the edges (token → edges), the missing fourth edge grows out of the bracket (A→C: bracket token `(`...`)` slides in around `B2+B3` and the dashed grey edge becomes solid red), the number `9.00 m` retexts to `13.00 m` | blue for B2 edges, red for B3 edges, bracket token green, the result number glyphs via `retext` | 3.0 s | "without brackets Excel multiplies first, so one side is missing; brackets fix it" |
| 8 | function | the three fitting hatches (bath, kitchen, bed) with their tones and areas on the floor | the cells G12, G13, G14 (G15 empty) and the result in G16 | C→A→A: `gather` three hatch rectangles into the range G12:G15 (cells lit) then collapse into `=SUM(G12:G15)` and the total `3.6 m²` in G16 | each hatch tone becomes the tone of its cell value, the areas add up 1.8 + 1.2 + 0.6 | 2.2 s | "SUM adds a range: the fittings give their total area" |
| 9 | locale | the English token `SUM(` `,` `.` | the German token `SUMME(` `;` `,` | A→A, `retext` glyph by glyph (SUM → SUMME, comma → semicolon, point → comma) | the cell references, the colour of each reference | 1.6 s | "German Excel writes SUMME, separates arguments with a semicolon and decimals with a comma" |
| 10 | recalc | the interior walls of the capsule move outward to the outer walls, the floor hatch grows | the cell B5 ticks 2.30 → 2.50 m, B9 8.7 → 9.5 m², B10 18.4 → 20.0 m³; the `Δ +0.8 m²` chip | A→C: `tether` lines from B5/B9/B10 to the moving dimension line and the growing hatch; the delta chip `morph`s from the hatch strip that grew | blue (width), green (area), the numbers; the delta chip is made from the hatch strip colour | 3.0 s | "every dependent cell recalculates and the floor follows" |
| 11 | trace | the width and length dimension lines (precedents) and the floor lifting into the capsule volume (dependent) | blue arrows from B5:B6 to B9 and orange arrows from B9 to B10 and B15 | C→A→C: the two dimension lines `travel` along the precedent arrows into B9; the floor hatch lifts into a prism and `morph`s into the volume cell B10, and the demand token `148 kWh/(m²·a)` flies to B15 and returns to the capsule as a faint glow | precedent blue / dependent orange, the numbers | 2.5 s | "Trace arrows show precedents and dependents" |
| 12 | div0, name, ref, value, repair | the interior length edge on the floor of the capsule | cell B6 and the error badges `#DIV/0!`, `#NAME?`, `#REF!`, `#VALUE!` | div0 (A→C): the emptied B6 turns the red length line into a dashed red `?` edge, the `#DIV/0!` tag in B15 travels out of the cell onto the capsule as a badge. name (A→A): the letter M of SUMM drops out (`retext` SUMM → SUM) and the cell fills. ref (C→A): the deleted row strikes through, the row cells fall as shards and the length line leaves a ghost with a dashed trail where the source was. value (C→A): the `m` unit splits off the number token `3.80 m` and ejects as a shard. repair (A→C): the retyped number flies from B6 back to the capsule and becomes the red length line, the error badges dissolve and the numbers return | red-orange for errors, red for the length line, numbers 3.80 m, 8.7 m², 18.4 m³, 3.6 m² | 5 x about 1.3 s | one beat per clause ("An empty interior length ...", "SUMM does not exist", "Delete a source row ...", "interior length was typed with its unit", "Type the number again") |
| 13 | legend | the eight parts of the window (title, tab, B9, B5, formula, function, Name Box, formula bar) | the eight terms | A→C→A: each term chip is born from its part (`morph` of a framed part to the term chip), then the whole window `gather`s into the lit Type A capsule on the tower, which pulses once | the part colour becomes the chip colour; the lit capsule is the one from the hook | 3.5 s | "Remember the terms: workbook, worksheet, cell, ..." and "Next video: references" |

Count: 13 beats (beat 12 groups five error morphs, one per clause). Concrete → abstract: 1, 2, 3, 4, 5, 7 (first half), 8, 11. Abstract → concrete: 6, 7 (bracket), 10, 11 (return), 12 (div0, repair), 13 (window to capsule). Chains: (a) capsule → tab (2) → cell B2 (3) → B2:B4 (4) → tokens (5) → floor hatch (6): the same Type A capsule; (b) interior length line → token `B6` (5) → precedent arrow (11) → `?` edge (12) → repaired line (12) → term chip "reference" (13); (c) fittings → G12:G14 → SUM → total chip.

## Replaced fade-swaps and cuts

| Today | Becomes |
| --- | --- |
| `workbook`: FadeOut tower + FadeIn window | beat 1 (tower gathers into the title) |
| `sheets`: Indicate on tabs | beat 2 (capsules become tabs) |
| `address`, `range`: select + FadeIn frame | beats 3 and 4 (dimension lines become cells) |
| `formula`/`frames`: window edit + frames FadeIn | beat 5 (interior lines become tokens, tokens settle as frames) |
| `hatch`: FadeOut frames + Indicate | beat 6 (result number becomes the hatch) |
| `precedence`: FadeOut/FadeIn window2 + flip | beat 7 (tokens travel onto edges, bracket slides in) |
| `function`/`locale`: FadeIn window3 / formulas / tags | beats 8 and 9 (hatches gather into SUM; glyph retext) |
| `trace`: FadeIn arrows | beat 11 (dimensions and floor travel along the arrows) |
| `div0 name ref value`: FadeOut/FadeIn window and tooltip | beat 12 (cell emptied turns the edge into a `?` line, shards, drop-out letter) |
| `legend`: FadeIn rows + targets | beat 13 (terms are born from their parts; window collapses into the capsule) |

## Implementation notes (for Phase B)

- Sources: `self.tower_axon(...).capsules[key]` (`silhouette()`), the live capsule group `cap.dims[name]`, `cap.at(x, y, z)`; window parts `window.cell(a1)`, `window.cell(a1).label`, `window.tabs[...]`, `window.frames`, formula tokens `xl_formula_text(...).refs`.
- Heavy tower: always `proxy()` for beat 1, per-capsule `silhouette()` only for the four typed capsules of beat 2.
- Identity ghost: leave a faint copy (ghost 0.25) of each concrete source until the abstract object is done, and a `tether` between the two (removed at the end of the clause).
- The live capsule redraws from `self.s`; every morph that ends in the capsule must set the state first and animate the morphing token to the position of the new part (`cap.at(...)`); mid-frame readability needs a path arc of 0.3 to 0.6.
- Edit windows are the engine output (`_xl.apply`), numbers from `values`, nothing typed.
- No edit of `🏢️nakagin/**`. Counts from the model: 140 capsules (A 36, B 35, C 34, D 35).

## Status

Phase A done. Phase B is blocked on `api.md` section 7 (`Status: ready`) and the module `🎓️teaching/🏛️architecture/🔀️morph/🐍️.py`, neither of which exists on disk yet.
