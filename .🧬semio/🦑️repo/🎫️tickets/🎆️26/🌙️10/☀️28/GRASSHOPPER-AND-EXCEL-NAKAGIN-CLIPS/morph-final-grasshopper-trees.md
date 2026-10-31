# Final Morph Review: grasshopper / trees (`Beat5_DataTrees`)

Judged from the scene code (`G/🌳️trees/🐍️.py`), the storyboard, the done file and the 640x360 midpoints in `T/🗑️generated/morph/🦗️grasshopper/Beat5_DataTrees/{en,de}/`. Several `en` mids (hook, flat, branch, item, recap) are missing or stale (en 19:46 vs de 19:05); `de` was used for those. Item and flat mids fall at gaps or show no flier.

## Verdict per clause

| Clause | Rating | Why |
| --- | --- | --- |
| hook | ACCEPTABLE | Fade-in of the whole stage; the premise ("which one sits where?") has no morph, fine for an opener. |
| flat | WEAK | The prime "capsule becomes the list item": one copy flies to the framed Panel row, but no flier is visible in the mid frame, the tower keeps the capsule in place (no ghost/dim) and the row stays a text "Brep"; the number 21 is not born from the capsule. |
| tree | ACCEPTABLE | Real capsule shapes (`capsule_pairs`, shell to ring, window to dot) fly from the tower to the leaves: the right idea. But the tower is not dimmed while 140 copies leave, capsules are grey while the tree trunk is teal/orange and leaves periwinkle (colour does not carry as the storyboard promises), and the fliers read as flat shards at 640x360. |
| branch | GOOD | A big, real capsule copy visibly flies to the two leaves of the first branch; the lit pair on the tower stays as origin. |
| path | GOOD | Each path digit is born from its tower object (tower, storey slab, facade) and lands in the dark path text in segment colour; route lights in the tree. Flaw: the head text cross-fades through the old count (garbled "path = address..." overlap). |
| item | WEAK | Idea is right (index chip out, whole address chip back to the ring) but the two flights are sequential in 1.4 s, the mid frame falls on the gap (nothing airborne), the swapped head text overlaps the old one, and the right capsule is a different magnified drawing. |
| panel | ACCEPTABLE | Lit storey capsules gather into count chips; good identity. Param Viewer / Tree Statistics windows are fade swaps, the stats panel numbers are not born from the capsules. |
| flatten_one / flatten | ACCEPTABLE | Real leaf dots regroup into one row (`_regroup`); the tower only retints (sweep tint is unused), and the Panel list just swaps. Abstract-to-abstract is fine here. |
| graft_one / graft | ACCEPTABLE | Leaves regroup into per-item branches; same remark as flatten. |
| simplify_one / simplify | ACCEPTABLE | Leaves regroup, but the leading `0` is not peeled off a path chip; the path text just swaps in the Panel. |
| mapper_example | GOOD | Letters A, B, C, D are born from the highlighted tower and fly into the binding row ("C=3" large token visible), storey lights green. Flaw: cross-fade clutter with the old `#21` chip and Panel (double text). |
| mapper | ACCEPTABLE | Facade leaves regroup into storey leaves; rule row is a swap. |
| perstorey / pertower | GOOD | Capsules of a storey / tower collapse into the count chips, tower groups stay lit. |
| flip | ACCEPTABLE | Leaves cross to the new branches, tower recoloured by index; the null dots are not morphed from a capsule leaving. |
| match | ACCEPTABLE | Swap only; the swatches are the setup for the painters in the next clauses. |
| mistake | GOOD | Swatches physically leave the colour Panel and land on the capsules in branch order; wrong stripes arrive with the flight. Strongest abstract-to-concrete morph. |
| why | WEAK | Viewer windows and a side-by-side mini tower pair are swapped in; no object travels (acceptable for a diagnostic card, but not morph-driven). |
| fix | ACCEPTABLE | Pipeline parts appear stepwise; the grafted colour list does not visibly fan out to the two branches. |
| result | GOOD | Swatches from the fixed Graft/Custom Preview chain paint 76 teal and 64 orange capsules; tower dimmed first, then re-lit. |
| recap | WEAK | The old window and diagram simply fade out, then a ten-row legend of mini trees fades in; only the `#21` chip flies home. The finale is a fade swap, not a morph back into the real objects. |

## Overall

Both directions exist and chain well (capsule 21 to row to leaf to path chip to ring to letters to `#21` chip to ring; swatches back to capsules). The owner's prime example is present in `tree` and `branch`, but it is not yet as strict as the owner wants: the concrete source never ghosts, the colour does not carry (capsule grey, leaf periwinkle, trunk teal/orange), and `flat` and `item` (the sentences that define "item" and "index") are weak or off-beat. The remaining fade swaps of head/thread texts garble labels in several clauses.

## Verdict: POLISH

1. **tree / `_birth`, `_hatch` (code L170-187, step at L457):** tint the tower by tower colour (A teal, B orange) just before the birth and dim every source capsule to ~0.25 as its copy leaves (ghost, not intact); give the leaf rings the capsule's tower colour (pass `leaf_color` = tower tone for the full tree diagram) so colour carries; make copies larger/front-face (not silhouette shards) and keep the item tokens on screen ~0.4 s before the diagram fades in (stagger by storey).
2. **flat / `_hatch` single-item path (L172, step at L453):** fly one clearly visible, large capsule copy (drop `lag=0.2`), then retext the capsule into the number `21` and let it land as the Panel row "21 Brep" (digit flies into the row index, ring stays on the tower); dim capsule 21 on the tower while it travels.
3. **item (step at L516):** run `index_out` and `chip_back` overlapping (LaggedStart 0.3) over ~2.4 s so nothing is empty at 50 %; keep the same tower drawing (do not swap to a second magnified drawing mid-flight).
4. **`_swap` of `head` and `thread` (L236, used in path, item, mapper_example, flatten...):** replace the cross-fade by a `Transform`/retext of the same text mobject (toolkit text morph) so the old and new strings never overlap; this removes the double text in path, mapper_example and item mids.
5. **recap (`_recap`, L684):** morph instead of fade: shrink the live tree diagram into the first mini tree, let each term label fly from the object it names (leaf = Item, branch lines = Branch, path chip = Path, index chip = Index) and keep the tower with its tints; at least turn `FadeOut(gone)` into a shrink-to-legend of the actual diagram and Panel.

Optional after these: `why` and `fix` could fan the grafted colour items from the Graft Tree output to the two branches before the repaint.
