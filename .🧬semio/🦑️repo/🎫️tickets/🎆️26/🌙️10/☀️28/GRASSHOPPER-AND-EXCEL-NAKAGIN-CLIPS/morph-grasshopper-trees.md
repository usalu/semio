# Morph Storyboard: grasshopper / trees (`Beat5_DataTrees`)

Scene `G/🌳️trees/🐍️.py`, 24 clauses kept (hook, flat, tree, branch, path, item, panel, flatten_one, flatten, graft_one, graft, simplify_one, simplify, mapper_example, mapper, perstorey, pertower, flip, match, mistake, why, fix, result, recap). Thread: capsule 21 (`{0;0;3;3}[0]` at the last capture (read from the engine), ringed) plus its family (facade pair, storey, tower). Counts always from the engine (read from the engine (last capture: 75 branches, 140 items, tower A 76, tower B 64)).

Rule used everywhere: the concrete source stays on the tower as a dim ghost (opacity 0.25) while the abstract copy is alive, so the viewer sees where it was born; the copy keeps the capsule's colour and a thin tether runs from leaf to capsule while it matters. Results always travel back: the abstract item flies to its capsule, which lights in the result colour.

## Beats

| # | Clause | Dir | Concrete (exact object) | Abstract | Carries over | Dur | Narration while it happens |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | flat | c→a | ringed capsule 21 on tower A, storey 3 | Panel row "21 Brep" with the yellow frame | yellow ring colour, index 21 | 1.4 s | "Item 21 is one capsule of 140." |
| 2 | tree | c→a (many) | all 140 capsules, tower colours A teal / B orange | 140 leaves of the tree fan, staggered storey by storey, then the twigs grow | tower colour, count 140, list order (bottom storey first) | 2.4 s | "A data tree keeps the address..." |
| 3 | branch | c→a | the facade pair `(0,0,0,0)` (two capsules) | the two leaves of the first branch become big, joined by the stem | capsule colour, count 2, order | 1.4 s | "One occupied facade group is one branch." |
| 4 | path | c→a chain | whole tower outline → "0"; tower B silhouette → "1"; storey 4 slab → "4"; facade pair → "2" | path segment tokens fly into the dark path plate, each in its segment colour; the diagram route lights as each lands | segment colour, value, order of the address | 7.0 s (existing) | "...building, tower, capsule level and facade." |
| 5 | item | c→a→c | capsule 21 → index token `[1]` | `[1]` snaps onto the path chip `{0;0;3;1}`; the complete chip then flies back and rings the capsule on the tower | orange index colour, then yellow ring | 1.4 s | "Path and index together identify the selected capsule." |
| 6 | panel | c→a | the storey capsule groups of tower B (8 and 6) | the numbers 8 and 6 in the length chips; ghost groups stay lit | storey colour, counts from engine | 3.6 s | "Tree Statistics reveals the different branch lengths." |
| 7 | flatten_one, flatten | a→a (+c) | storey 3 facade branches (tower: storey lit); then all 140 | the leaves of each branch fall sideways into one row (gather), the panel list grows; the tower sweeps in list order | leaf colour and order; total 6 then 140 | 1.6 s / 2.6 s | "Flatten ... joins its facade branches into one list." |
| 8 | graft_one, graft | a→a | storey 3 capsules | one leaf at a time splits off its branch and gets its own twig (split); each new twig keeps its capsule tether | leaf colour, index becomes last path segment | 1.6 s | "...every capsule gets its own branch." |
| 9 | simplify_one, simplify | a→a | leading path segment `0` | the shared `0` token peels off the path chips and fades into the building outline (a→c: "only one building") | segment colour | 1.6 s | "Simplify removes the leading 0..." |
| 10 | mapper_example | c→a→c | path segments of capsule 21's address | letters A=0 B=0 C=3 D=1 born from the highlighted tower objects; D is crushed (dropped), the remaining A,B,C slide into the new address and storey 3 with its capsules lights green | segment colours, D pink | 2.6 s | "The facade letter is dropped; the remaining address selects a storey." |
| 11 | mapper, perstorey, pertower | a→a, c→a | facade twigs of every storey; the 8 capsules of storey 4; capsules of tower A (76) and B (64) | twigs merge into one storey branch (storey branches, count from the engine); lit capsules gather into the chip number 8, 6, then 76 and 64; ghosts stay | storey / tower colour, counts from engine | 1.6 s, 4.0 s, 3.2 s | "...20 branches... 76 capsules in A and 64 in B." |
| 12 | flip | a→a | the leaves of one branch | leaves swap rows and columns: leaves rotate onto the new branches, short branches receive hollow null dots (not capsules) | leaf colour, null = hollow | 3.6 s | "Short branches receive null placeholders." |
| 13 | match, mistake | a→c | colour swatches teal / orange from the Custom Preview colour Panel | each swatch splits into capsule-sized tokens that fly onto the capsules in the branch order; the wrong stripes follow item index (alternating) | swatch colour, branch order | 1.6 s each | "...the short colour list repeats in every branch." |
| 14 | fix, result | a→c (many) | Path Mapper output `{0;0}`, `{0;1}`; teal / orange swatches | the two grafted colour items fly to their towers: 76 capsules turn teal, 64 turn orange; counts chips show the numbers born from the lit capsules | tower colour, counts from engine | 3.2 s, 1.6 s | "Matching paths send the tower colours to the measured totals." |
| 15 | recap | a→c | running chip `#21 = ...` | the chip flies to the ringed capsule 21 and the 10 terms point back at the tower | yellow, address | 2.0 s | "Ten terms, one address." |

Directions: concrete→abstract in beats 1, 2, 3, 4, 5, 6, 10, 11; abstract→concrete in 5, 10, 13, 14, 15; abstract→abstract in 7, 8, 9, 12. Chains: capsule 21 → row 21 (1) → leaf (2) → path chip (4,5) → ring on the tower (5) → letters (10) → `#21` chip → storey branch (11) → ring (15).

## Replacing fade-swaps

`_swap` cross-fades remain only for pure canvas component changes. Tower <-> diagram changes of the same data become morphs: (2) leaves are born from tower capsule proxies, (7)(8)(9)(11)(12) move the existing leaf dots of the old diagram into the new diagram (matching items keyed by capsule slot), (13)(14) paint by flying swatches.

## Implementation notes

- Leaf identity: `_diagram` records `group.leaves[(path, index)] = point` and the tag slot, so any two diagrams of the same capsules can be matched by slot.
- Capsule sources use `silhouette()`/`proxy()` of `drawing.capsules[key]` (never the full faces). If `T/api.md` section 7 is not `ready`, the scene uses scene-local Manim-only helpers with the same names and signature (`_born`, `_gather`, `_scatter`), to be swapped for the toolkit.
