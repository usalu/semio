# Scene: grasshopper / trees (`Beat5_DataTrees`)

File `G/🌳️trees/🐍️.py` (G = `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip`), title `{en: "Data Trees on the Tower", de: "Datenbäume am Turm"}`, 22 clauses, `construct` → `self.present("trees")` → `present_trees`. Storyboard: `storyboard-grasshopper-trees.md` (clause texts were tightened to two caption lines; the `recap` clause ends with "Ten terms, one address." / "Zehn Begriffe.").

## What was built

- Every number comes from the engines: the tree is `canvas("trees")` evaluated by the GH engine (`src` 70 branches / 140 items, `fl`, `gr`, `sm`, `pm` (20 storeys), `pt` (54 / 86), `fp` (2 × 70), `cp1` 70/70, `cp2` 54/86). Capsule addresses, towers, facades and storeys come from `🏢️nakagin` (`slots()`, `towers()`), the tower drawing from `NakaginDrawing.tower_axon`. The model JSON was not changed (all views the clip needs already exist in `canvases.trees`), so `build_model.py` was not re-run.
- Stage: left the tool (light Grasshopper canvas strip with real components Panel, Param Viewer, Tree Statistics, Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, Custom Preview, wired with tree wires) and below it the tree diagram; right the Nakagin tower. Both are on screen in every clause except the path/item zoom (slice of tower B) and the recap.
- Tree diagram (`_diagram`): drawn from any engine `Tree` as a real tree (trunk = building, boughs = towers, branches = storeys, twigs = facades, leaves = capsules), with level labels, prefix highlighting (`lit`), leaf colours, 140 leaves compressed as a fan. Segment colours are fixed: building white, tower cyan, storey yellow, facade green, item index orange.
- Path anatomy (clause 5): path `{0;1;4;2}` coloured per segment on a dark plate, one word per segment appears in sequence, the diagram lights the route and the tower is magnified in two steps (whole tower B, then a slice of B around storey 4 seen from the south with the facade pair outlined). Clause 6 lights the right capsule `[1]`.
- Tree operations (9–15): each clause swaps component, Panel rows (path headers + indexed items from `panel_rows`), count line and diagram for the evaluated result; the tower shows the meaning (sweep in list order for Flatten, per-storey / per-tower lighting with counts 6, 8, 6 and 54, 86, left / right capsules 70 each for Flip Matrix).
- Matching (16–21): Custom Preview with the 2-item colour list; the mistake paints left capsules teal and right orange in both towers (tag read from `cp1`), `why` shows two Param Viewers (depth 4 against depth 1), `fix` uses Path Mapper `{A;B;C;D}(i) → {A;B}` + Graft Tree (`cp2`), `result` paints 54 teal / 86 orange from `cp2`.
- Recap: ten terms (en / de) with mini trees computed by the engine (`Tree.flatten`, `graft`, `simplify`, `map`, `flip`) and the Nakagin result line.

## Clauses

hook, flat, tree, branch, path, item, panel, stats, flatten, graft, simplify, mapper, perstorey, pertower, flip, match, mistake, why, fix, result, rule, recap.

## Keyframe review notes (en and de, every PNG viewed)

- Found and fixed during review: invisible white text on the light canvas (path, mapping rule, labels) → dark plates and dark text; overlapping path labels; chips colliding with the tower (now one row along the top edge); the south facade of tower B hidden behind other capsules (zoom now draws a slice of tower B from the south); "1 branches" plural; Param Viewers overlapping the source parameter; colour list overlapping the parameter caption; German recap caption wrapping to three lines (shortened).
- Remaining strict finding: none after the last capture (see layout reports `T/🗑️generated/frames/🦗️grasshopper/Beat5_DataTrees/{en,de}/layout.md`).
- German keeps the Grasshopper UI names English (Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, Custom Preview, Tree Statistics, Param Viewer); captions, levels, chips and the recap are German; the Panel strings `teal` / `orange` and `Brep` are engine data.
- Traps: `capture.py` runs the whole scene for any key (about 4 minutes per language); an empty string in `self._say` crashes Manim `Text`; German captions wrap to three lines at about 130 characters.

## Tests

See the result block at the end of this file.

## Audit round (T/audit-grasshopper-trees.md): fixes

- Blocker: every operation now shows the RESULT structure. Count line, Panel rows and tree diagram are built from the evaluated result tree: Path Mapper per storey 20 branches (levels building, tower, storey), per tower 2 branches, Flatten 1 branch of 140, Graft 140 branches (five-index paths), Simplify without the building level (the towers are drawn as roots, no dangling apex). The diagram legend now follows the levels that exist after the operation.
- Flatten, Graft and Simplify are introduced on the smallest example (tower A, storey 0 = 3 facade branches) and then run on the whole tower in the same clause.
- Canvas inset from the frame edge (left edge at 28 px); components moved right, no clipped nubs.
- `fix`: Graft Tree is wired from the colour Panel; Path Mapper `T` comes from the capsules and `M` from a Panel holding the mapping text (Tree Statistics outputs P/L/C and the lexical rule `{A;B;C;D}(i) -> {A;B;C}(i)` kept as decided). The mapping Panel in `mapper`, `perstorey`, `pertower`, `fix` shows the rule without spaces so it is not cut.
- German Panel shows `türkis` / `orange`; one meaning per colour: building white, tower beige, storey green, facade pink, item index periwinkle; teal and orange are only the preview materials; tower B violet; highlight outline yellow; "no error" chip white.
- `perstorey` lights an eight-capsule storey (tower B storey 4) next to the six-capsule top storey (storey 10) with chips; `pertower` lights A, then B, then both.
- Chips name the tower (`B, storey 10: 3 facades`); `panel` shows `Param Viewer: 70 branches`; German grammar fixes applied (`welche Kapsel`, `mit seinem Pfad`, `wird zu`, `zeigt, warum`, `Die linken Kapseln ...`, `führen zu stillen Fehlern`, `Datenbaum` in the recap, `mit Graft Tree verteilen`).
- Storyboard F8: clause 13 says 20 branches (9 + 11), not 30.

Remaining strict findings (6 per language, frames `mapper`, `perstorey`, `pertower`): the wire from the mapping Panel (output on its right) to the Path Mapper `M` input (on the left) loops behind the component, as it does in Grasshopper when a Panel sits upstream-left; justified. Captures: 0 other findings in en and de; all 44 PNGs of the last run viewed in the loop.

Tests: the presentation test passed for `Beat5_DataTrees` before this round; the only failures (`Beat3_Tower`, unhashable dict key in `_solve`) are another agent's scene. Not re-run after this round (about 17 minutes); the scene was re-rendered natively by the captures in both languages without errors.

## Quality round (T/quality-grasshopper-trees.md): implemented

24 clauses now (hook, flat, tree, branch, path, item, panel, flatten_one, flatten, graft_one, graft, simplify_one, simplify, mapper_example, mapper, perstorey, pertower, flip, match, mistake, why, fix, result, recap). The `stats` clause was merged into `panel`, the `rule` clause was removed (its sentence lives in `why`).

1. One capsule as story thread: capsule 21 (engine slot 21, tower A, storey 3, facade 1, address `{0;0;3;1}[1]`; capsule 57 would sit in the middle of tower B where the facade slice is hard to read). It is ringed on the tower in every frame, a running chip `#21 = <path>[index]` follows it through every operation (`{0}[21]` after Flatten, `{0;0;3;1;1}[0]` after Graft, `{0;3;1}[1]` after Simplify, `{0;0;3}[3]` after the storey mapping, `{1}[10]` after Flip) and the recap ends on `capsule 21 = {0;0;3;1}[1]`.
2. `why`: the two path sets side by side in path colours (`{0;0;0;0}` x70 against `{0}` x1), the Param Viewers, and two small towers "expected" against "actual", a red "different paths" chip; the caption names the repeating of the short list.
3. `mapper_example` (new): A=0, B=0, C=3, D=1 bound on capsule 21's path, D struck through, `{0;0;3;1}[1] -> {0;0;3}[3]`, storey 3 lit green with its six capsules; `mapper` then shows the 20 branches.
4. Recap with the tower on screen, results per term computed by the engine for capsule 21, German "Zehn Begriffe, eine Adresse."
5. Flip Matrix with reason and before/after: left / right colours on the tower and a 70-branch diagram first, then the Flip Matrix component and the two-fan result.
6. Tree Statistics: one Panel per output (P paths, L lengths, C count 70) with separate wires; caption "70 branches of two items".
7. Colour swatches next to the colour names in the Custom Preview colour Panels.
8. "Read both paths first" moved to `why` only (no duplicate).
9. One colour per tower throughout (A teal, B orange; path segments, tree, tower, chips); yellow ring only for capsule 21 and in the one-storey states; `result` paints from the engine.
10. `fix` in two sub-steps: Path Mapper output first (towers dimmed in their colours, paths `{0;0}`, `{0;1}`), then the Graft and colours with "equal paths" and the meeting line.

Review: all 24 frames x en/de viewed in this round (the en frames of `flat`..`recap` in full, de in full); layout report: 0 findings except the intentional strike-through on `D=1` (`mapper_example`) and a wire crossing the index digit of the colour Panel (`fix`, `result`), both justified. The G model tests pass (57 tests); the contract script `T/gh-trees/contract.py` confirms 24 unique bilingual clauses, all used, `present_trees` present. The full presentation test (about 17 minutes for all scenes) was not re-run.
