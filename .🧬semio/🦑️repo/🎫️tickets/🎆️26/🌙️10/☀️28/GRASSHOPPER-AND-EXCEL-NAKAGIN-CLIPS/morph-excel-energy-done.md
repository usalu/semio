# Morph done: excel / energy (`Beat4_Energy`)

Scene: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`. Storyboard: `morph-excel-energy.md`. Frames on disk: `🗑️generated/morph/📊️excel/Beat4_Energy/{en,de}/` (640x360, mid frame of every animation `<key>-<n>-mid.png`, 39 mid frames per language from the full run; later key runs overwrote only their keys). Only what is listed under "Frames read" was looked at.

## Mechanism

`step(...)` has up to three phases (`animations`, `then=` after `between()`, `after=`); swarms keep their own timing (`seconds=None`). Morph sources are still copies of the live drawings (`_static`, `_arrows`, `_face_flyers`, `_stack_shape`, tower dots) handed over to the live drawing after the clause. The toolkit default matches real parts automatically (new version, all frames below are from after that change). `capture_morph.py` forwards `**kwargs` to `step`.

## Beats built (14 of 14 stored beats, as captured)

| Clause | Beat | Direction |
| --- | --- | --- |
| sheet | tower capsule morphs into the big capsule; a second copy morphs part-wise (`capsule_pairs`) into the tab chip | C to C and A |
| dims | three dimension lines fly into B2:B4, ghost stays on the capsule | C to A |
| areas | seven faces (hidden ones as tinted squares) unfold into the area chips in D3:D9; the tint arrives after the chips land (`after`); chips stay until the window changes | C to A, then A to C |
| ua | U·A chips grow into the seven arrow shafts | A to C |
| ht | arrows pile up into the H_T token | C to A |
| degreedays | hatched shortfalls gather into the Σ token | C to A |
| mistake / fix | b chip flies to the back-wall arrow and back to B6 (0.50); the chart fades first, then the capsule appears | A to C and back |
| volume | volume box gathers into the V chip, blue H_V arrow grows | C to A, A to C |
| total | H_T and H_V chips morph into the bar segments | A to C |
| load | nine opaque bar slices scatter into the eight arrows and the Φ chip in B13 | A to C |
| annual | floor plan morphs into the floor-area tile; the parked Σ token morphs into the "degree days" tile | C to A, chain from degreedays |
| energy | 140 type-coloured dots scatter to the left of the four count chips, chips fade in | C to A |
| compare | the four capsules morph part-wise into item tokens with the engine value; chart fades in beneath | C to A |
| shape | faces of both capsules unfold into area-proportional rectangles above the capsules (net rows at y 3.1 and 2.65) | C to A |
| whatif | 140 opaque seeds from the U cell scatter onto the small tower; they turn green with the live change | A to C |
| recap | the 140 dots scatter onto the legend tower, the tower fades in | A to C |

## Data note on compare

On disk the engine gives the same value for all four types: 148.2 kWh/(m²·a) (`excel-energy/probe-compare.py`, window diameter 1.3 m for all types), so bars, value labels and narration agree. If the model gets a larger window for C the bars will show it by their labels and heights, the narration clause would then need a new sentence.

## Frames read

Full run (all clauses, no key filter), en and de: read as contact sheets of all mid frames (en: all 39; de: second sheet of 15 plus single frames sheet, ht, mistake, compare, load, recap). Then keyed runs after the later changes: en/de `areas exposed ua` (en areas read), en/de `mistake fix whatif frozen repaired` (en mistake, fix, whatif, de frozen, repaired, mistake read).
Observed: Σ chip parked at the right now fully on stage; mistake capsule appears after the chart has faded; shape net clear of both capsules; recap dots land in the tower; energy dots no longer cover the C chip.

## Lessons pass (T/morph-review-excel-references.md, 12 lessons)

Changed: lesson 2 (area chips persist until the window changes, no longer cleared in `exposed`) and lesson 3 (tint after the chips land, via `after=`). Already satisfied: 1, 4 (only continuous ValueTracker blends), 6 (structure before morph via phases), 7 (at most seven flying objects except the 140-dot swarms), 9 (both directions), 11 (whatif and recap return to the tower), 12 (default part matching, no anonymous path tokens). Not done: 5 (frozen total shown as a state change, no event morph), 8 (no 1280x720 capture), 10 (narration timing against key words not checked), and the `recalculate` retyping from the XL toolkit message is only seen in the mistake, fix, whatif, frozen and repaired frames above.

## Open

- Recap legend symbols do not themselves morph into the tower (only the dots do).
- `compare` does not morph capsule silhouettes into the bar tops; they become item tokens and the chart fades in.
- Mid frames at 640x360 only; `capture.py` layout keyframes not re-run.

## Final polish (after SHIP review)

Done: `compare` now shows the chart from the start of the clause and the four capsules morph part-wise into value tokens that land on the bar tops (the chart no longer fades in after them); `annual` morphs the parked H_T chip (kept from `ht`, parked right through climate to load) into the H tile and the parked Σ chip into the "degree days" tile; `recap` legend symbols fly from the dependency panel (U, H_T, H_V, Q, Q/A) or the small tower (the rest) to their rows; the dead window-diameter colour branch is removed; the storyboard line about "only C stands higher" now states equal bars (148 kWh/(m²·a)).
Not done: the "24 h/d" tile has no earlier object to come from, it still fades in with the tile row; recap symbol origins are approximate (one source point per group), not the exact place each symbol first appeared; the SUMPRODUCT symbol overlaps the "Building:" result line while it flies.
Captured: en and de keys `annual energy compare whatif recap` (21 mid frames each, no errors). Frames read: en annual-04, compare-03/04, recap-03/04, energy-04; de annual-03, compare-04, recap-03. whatif frames of this run were not opened (whatif code unchanged in this round).
