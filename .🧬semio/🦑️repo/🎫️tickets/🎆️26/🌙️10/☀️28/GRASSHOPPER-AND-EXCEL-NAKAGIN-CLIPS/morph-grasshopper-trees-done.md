# Morph Done: grasshopper / trees (`Beat5_DataTrees`)

Scene `G/🌳️trees/🐍️.py`. Storyboard `morph-grasshopper-trees.md`. Implemented with scene-local Manim-only helpers (`_Trip` animation, `_flight`, `_birth`, `_regroup`, `_gather`, `_paint`, `_centre`) because the toolkit was not ready when the work started.

## Beats implemented

- flat: capsule 21 flies to the framed Panel row (c to a).
- tree: all 140 capsules fly in list order to their 140 leaves, tower colour to leaf colour (c to a).
- branch: the two capsules of the first facade group fly to their two leaves.
- path: each segment (building, tower, level, facade) is born from its tower object and lands in the path text, segment colour.
- item: index token flies from capsule 21 to the `[1]` chip, then the full address chip flies back to the ringed capsule (c to a to c).
- panel: capsules of the two compared storeys collapse into their count chips.
- flatten_one, flatten, graft_one, graft, simplify_one, simplify, mapper, pertower, flip: leaves of the old diagram fly to the same capsules (matched by slot) in the new diagram.
- mapper_example: letters A, B, C, D fly from tower, tower, storey and facade into the binding row.
- perstorey, pertower: lit capsules gather into the count chips.
- mistake, result: colour tokens leave the swatches and land on the 140 capsules (wrong stripes, then the matched towers).
- recap: the running chip flies home to the ringed capsule 21.

New data: `_diagram` returns `group.leaves[slot] = (point, tone)`.

## Frames checked (en, `T/🗑️generated/morph/🦗️grasshopper/Beat5_DataTrees/en`)

tree, flatten, mapper_example, pertower, flip, result mid frames viewed: tokens are readable, arcs show the origin, colours carry (teal/orange towers, periwinkle leaves, yellow ring). Other mids (flat, branch, path, item, panel, flatten_one, graft_one, mapper, perstorey, mistake, recap) were rendered without errors; not each viewed.

## Foreign-change fix

`flip` caption was 3 lines in both languages and crashed `caption_bar`; shortened en and de (transcripts `🗣️transcript.*.md` may need `sync-transcripts.py`).

## Open items

- German morph run and `T/capture.py` keyframes for layout not done.
- Presentation contract tests not run.
- Migrate `_Trip` helpers to the toolkit `morph()/scatter()` (part-wise morphs, api.md section 7) once stable.
- Mid-flight cross-fades of `_swap` still overlap texts (existing behaviour).
- Only the first codex overwrite wiped my edits once; they were reapplied in one atomic script (`scratchpad/apply.py`).

## Round 2: toolkit migration

- `flat`, `tree`, `branch` now use `_morph.scatter(copies of the tower capsules, item tokens, pairs_fn=_morph.capsule_pairs, dip=0.15)` through `_hatch` / `_birth` (capsule shell becomes the leaf ring, window becomes the dot, tower colour carries to the leaf tone). The tokens are dropped by `_settle()` after the clause; the diagram leaves lie underneath. Copies are used so the tower itself stays intact.
- `_Trip` / `_flight` / `_regroup` / `_gather` / `_paint` stay for the nested morphs (path digits, item chip, panel / perstorey / pertower gathers, regroup of leaves, swatch painting) because the toolkit forbids Swarms inside `AnimationGroup` / `Succession` and has no leaf-to-leaf equivalent.
- Verified after writing: all edits present in the file (grep `_hatch`, `_regroup`, `_Trip`). One self-inflicted problem: my `r+` rewrite left a stale tail after the file got shorter (syntax error); removed with the Edit tool. Codex did not overwrite this round.
- Captures en and de (flat, tree, branch, path, item, flatten, mistake, result, recap) ran without errors; `tree` and `flat` mid frames viewed: real capsule shapes travel to the leaves. `capture.py` keyframes flat, tree, branch, path, item en and de: 0 strict findings (soft caption-touch notes are the tower base, as before).
- Still open: German/English mid frames of every other clause not individually viewed; presentation contract tests not run; migrate remaining flights to the toolkit if a non-nested pattern becomes possible.

## Round 3: real objects in flight (audit fix) and lessons pass

- `_Trip` now flies the real object (copy) instead of a dot: capsule copies (flat, branch, panel/perstorey/pertower gathers), the actual path digit (path), the `[i]` chip and the whole address chip (item, there and back), the A-D letters (mapper_example), the old leaf dot (regroup flatten/graft/simplify/mapper/pertower/flip), the swatch square (mistake, result, routed through a waypoint below the GH sheet so it does not cross Custom Preview, Path Mapper or Graft Tree), and a copy of the running `#21` chip (recap). The toolkit `scatter` + `capsule_pairs` stays for `tree` (140 capsules, ring and dot per leaf). `flat` and `branch` use real capsule copies via `_Trip` because the single-item toolkit morph was invisible at mid-flight; leaf-to-leaf stays `_Trip` with the real leaf dot (a dot to a dot, no toolkit nesting needed).
- Recap: old pipeline fades first, then the legend (Succession); path: the `#21` chip enters only in stage 4 (no more overlap with the head text).
- Fixed a `KeyError` in the mistake and fix diagrams (paint tables are keyed by capsule slot, not branch position); the leaf colour now reads the item's slot tag.
- Storyboard values updated (branches and address are read from the engine; last capture: 75 branches, `{0;0;3;3}[0]`).
- Full capture en and de (no key filter): 24 mid frames each, no errors. Viewed (en): flat, branch, path, item, mistake, recap, plus the earlier tree, flatten, pertower, result, flip; real capsules / swatches / chips visible in flight. Item mid frame falls on the boundary between its two flights (nothing in the air at exactly 50 %).

### Lessons pass (T/morph-review-excel-references.md section 3)

1 one actor: capsule 21 and its chip run through the video (ok). 2 bound pair persists: tree leaves and the ring stay; no extra tether added (open). 3 number before state: `mistake`/`result` now play the flights first, then the tint and tags (Succession). 4 no tracker sweeps: none used. 5 actual event: swatches physically paint the capsules. 6 structure first: window swaps happen at clause start, morphs run beside them except tree/flat (open). 7 legible: swatches/capsule copies keep their colour and size, flights off the GH sheet; `tree` has 140 fliers by design. 8 mid frames read at 640x360 only (open: 1280x720 not captured). 9 both directions: yes (flat/tree/path C to A, mistake/result/recap A to C). 10 narration timing: not verified frame by frame (open). 11 back-links: counts gather capsules into chips, results return to the tower, recap chip returns to capsule 21. 12 toolkit hygiene: `capsule_pairs` for tree; copies of real mobjects everywhere, no anonymous tokens.

## Round 4: final polish (T/morph-final-grasshopper-trees.md)

1. tree: the tower is dimmed in its tower colours while 140 teal/orange capsule copies leave it (the tower visibly gives its capsules); leaves and trunk take the tower colour (`leaf_color` = tower tone). Frame checked en and de: teal/orange copies in flight, dimmed tower, coloured fan.
2. flat: one large capsule copy (1.8 x its width, no lag) flies into the Panel row; capsule 21 is dimmed on the tower meanwhile. Checked (de): the capsule is visible landing at the row.
3. item: the index chip and the address chip fly overlapping (lag 0.3) over 2.4 s on the same detail drawing. Checked (de): the chip sits on the ringed capsule, no empty frame.
4. head and thread texts retext in place (`Transform` of the same mobject) instead of cross-fading; no double text. Side effect: mid frames show glyphs in transition ("7# branches"), as intended for a retext.
5. recap: the live diagram and the Panel shrink into the first legend rows (scale 0.15, move, fade) instead of fading out; why: two copies of the short-list chip fly over the long path row.
- ONE full capture en and ONE de: 24 mid frames each, no errors. Viewed: tree (en, de), recap (en), item (de), flat (de).
- Open: recap legend terms do not fly from the objects they name; `why`/`fix` do not fan the grafted colours; layout keyframes (`capture.py`) and presentation tests not re-run after this round.
