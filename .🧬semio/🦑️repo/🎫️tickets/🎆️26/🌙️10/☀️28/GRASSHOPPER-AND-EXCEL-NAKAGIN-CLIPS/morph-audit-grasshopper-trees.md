# 🌳 Morph Audit: grasshopper / trees (`Beat5_DataTrees`)

Read-only audit by Haiku. Inputs: `morph-grasshopper-trees.md` (planned beats), `morph-grasshopper-trees-done.md` (built), scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py` (last edit 2026-10-29 16:24:44), frames `🗑️generated/morph/🦗️grasshopper/Beat5_DataTrees/{en,de}/`.

Freshness cutoff: scene edit minus 60 min = 15:24:44. Frames older than that are STALE and not judged.

Judged (newer than cutoff, mid frame `-03-mid.png`): en flat, tree, branch, path, item, flatten, mistake, result, recap; de flat, tree, branch, path, item, flatten, mistake, result, recap.

Stale (en only, not judged): flatten_one, graft_one, mapper_example, mapper, panel, perstorey, pertower, flip (flip at 15:22:40, 2 min before cutoff).

Frames are only the `-03-mid` midpoint per clause. The `index.md` files also list `-01-mid`, `-02-mid` and `-04-mid`, but those files are not on disk. No `recap-end.png` exists in either language.

## 1. Planned beats

Legend: BUILT = recognisable object in transformation, identity, colour and origin readable. WEAK = blob, unreadable, or a fade where a morph was planned. MISSING = no frame on disk. STALE = frame older than cutoff.

| # | Clause | en | de | Evidence and notes |
| --- | --- | --- | --- | --- |
| 1 | flat (c→a) | WEAK | WEAK | `en/flat-03-mid.png`, `de/flat-03-mid.png`: no capsule copy in flight. Only a small dot near row 21. No tether. |
| 2 | tree (c→a, many) | BUILT (marginal) | BUILT (marginal) | `en/tree-03-mid.png`, `de/tree-03-mid.png`: pale capsule-like trail from the tower to the fan. Tokens are about 8 px. Trunk and twigs appear by FadeIn. |
| 3 | branch (c→a) | WEAK | WEAK | `en/branch-03-mid.png`, `de/branch-03-mid.png`: the two leaves are placed, but no flying capsules are visible. Old "items" text ghosts behind the head. |
| 4 | path (c→a chain) | WEAK | WEAK | `en/path-03-mid.png`, `de/path-03-mid.png`: plate with coloured digits. No digit flights visible. Head text "path = address…" overlaps the `#21` chip (garbled). Tower is a cross-fade, not a recolour. |
| 5 | item (c→a→c) | WEAK | WEAK | `en/item-03-mid.png`, `de/item-03-mid.png`: the index token is a single yellow dot, not a `[0]` chip. Capsule 21 is only tinted in detail view. |
| 6 | panel | STALE (en) / MISSING (de) | MISSING | - |
| 7 | flatten_one (STALE en) / flatten | BUILT | BUILT | `en/flatten-03-mid.png`, `de/flatten-03-mid.png`: the 140 blue leaves converge into one row. Identity and colour carried. Head "items" ghost overlaps. |
| 8 | graft_one (STALE en) / graft | MISSING (graft) | MISSING | No frame for `graft`. |
| 9 | simplify_one / simplify | MISSING | MISSING | No frames. |
| 10 | mapper_example | STALE (en) | MISSING | - |
| 11 | mapper, perstorey, pertower | STALE (en) | MISSING | - |
| 12 | flip | STALE (en) | MISSING | - |
| 13 | match / mistake | match MISSING; mistake BUILT | match MISSING; mistake BUILT | `en/mistake-03-mid.png`, `de/mistake-03-mid.png`: teal and orange dots fly from the colour panel toward the tower, colour carried. Tokens are dots, not capsule-sized. They cross the "Custom Preview" label and the top-right "no error" chip. |
| 14 | fix / result | fix MISSING; result BUILT | fix MISSING; result BUILT | `en/result-03-mid.png`, `de/result-03-mid.png`: orange tokens fly to the tower and paint B orange. Tower A chip shows 76 teal. Trajectory crosses the Path Mapper, Graft Tree and Custom Preview components. |
| 15 | recap (a→c) | WEAK | WEAK | `en/recap-03-mid.png`, `de/recap-03-mid.png`: the legend is a fade-in over 47 FadeOuts. The running `#21` chip is a single yellow dot. No morph from the tower or diagram into the ten terms. |
| - | hook, why (not in table) | MISSING | MISSING | No frames. |

Summary: BUILT 4 of 15 planned beats (tree marginal, flatten, mistake, result). WEAK 6 (flat, branch, path, item, recap, and match/mistake dots). MISSING or STALE for the rest.

## 2. Remaining fade-swaps where a morph would show identity

- `path`: `_swap("tower", view)` cross-fades the full tower with its tinted copy. The tower is the same object and should recolour, not fade (`index.md` path-03: AnimationGroup, AnimationGroup, Succession).
- `item`: `_swap("tower", right_capsule)` fades capsule 21 into its magnified detail view. Identity is lost during the swap.
- `tree`: `FadeIn(full_diag)` makes trunk and twigs appear by fade. Acceptable, since they are abstract, but the plan's "twigs grow" is not built.
- `flat`: `_swap("pipe", panel21)` fades the Panel row in over the old panel. The row is abstract, so the fade is acceptable.
- `recap`: 47 FadeOuts and one FadeIn of the legend. The legend terms should be morphs from their tree/diagram objects, not fades.

## 3. Directions and chain

- Concrete to abstract: tree (BUILT), flat, branch, path, item (WEAK).
- Abstract to concrete: result (BUILT), mistake (BUILT), recap (WEAK).
- Abstract to abstract: flatten (BUILT).
- Both directions occur. Capsule 21's yellow ring is present in every judged mid frame, so the object persists. The address chain breaks at the path digits (WEAK) and the index chip (WEAK), so the link from row 21 to the tower is not morphed.

## 4. Collisions and clutter

- `path-03` en and de: the head "path = address…" overlaps the `#21` thread chip. The text is garbled. FIX.
- `mistake-03` en and de: the dots cross the "Custom Preview" label and the top-right "no error" chip. FIX.
- `result-03` en and de: the orange flight crosses Path Mapper, Graft Tree and Custom Preview. FIX.
- `recap-03` en and de: the legend sits over the old pipeline, which is still faintly visible. Labels overlap ("Kapseln" over "Flatten", "Element" over the teal/orange panel). Unreadable. FIX.
- `branch-03`, `flatten-03`, `tree-03`: head text overlaps the faded previous head ("items", "Elemente"). Minor.
- `en/recap-03`: labels carry a German gloss in brackets ("Tree [Datenbaum]"). Confirm the English version should show this.

## 5. Document drift (not frame defects)

- `morph-grasshopper-trees.md` says 70 branches. Frames show 74 (`en/tree-03-mid.png`, `de/tree-03-mid.png`). The engine count is 74, so update the storyboard.
- `morph-grasshopper-trees.md` gives the example address `{0;0;3;1}[1]`. Frames show `{0;0;3;3}[0]` (`en/item-03-mid.png`). Update the storyboard.
- `index.md` (en and de) lists `-01`, `-02` and `-04` mids that are not on disk. Regenerate the index after recapture.

## Verdict: FIX

Fix list:
1. `_Trip` always draws a `Dot`. Accept a mobject factory so flights carry the capsule copy, a `[i]` chip or a path chip. This fixes flat, item and recap.
2. Add a visible tether from the source (ghost) to the abstract copy for flat, branch and path.
3. `path`: hide the `#21` thread chip until stage 4, or shorten the head so it does not reach x = -0.35.
4. `mistake` and `result`: route flights below the GH components (lower arc or fixed waypoint) so they do not cross Custom Preview, Path Mapper or Graft Tree.
5. `recap`: remove the old pipeline before the legend FadeIn. Make the running chip a chip, not a dot. Morph each term row from its tree or diagram object.
6. Replace the `path` and `item` tower/capsule `_swap` with a recolour or zoom morph that keeps identity.
7. Capture every flight at its own midpoint (not the clause midpoint), since the clause midpoint can miss Succession stages.
8. Recapture the stale en frames (flatten_one, graft_one, mapper_example, mapper, panel, perstorey, pertower, flip) and capture all missing frames: hook, graft, simplify_one, simplify, match, why, fix, and every de clause from flatten_one onward. Add `recap-end`.
9. Decide whether en recap should show German glosses.
10. Update the storyboard counts (74 branches) and the example address (`{0;0;3;3}[0]`). Regenerate both `index.md` files.
