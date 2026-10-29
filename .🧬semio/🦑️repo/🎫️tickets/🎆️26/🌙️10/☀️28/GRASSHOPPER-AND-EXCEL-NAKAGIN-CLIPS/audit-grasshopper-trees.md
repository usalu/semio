# Audit: grasshopper / trees (`Beat5_DataTrees`)

Scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py`, 22 clauses, frames `🗑️generated/frames/🦗️grasshopper/Beat5_DataTrees/{en,de}/*.png` (44 PNG, all viewed). Facts checked against `🏢️nakagin/🔣️.json` (authoritative), `research-grasshopper.md` §3, §4, §7, `research-nakagin.md` §3, and `storyboard-grasshopper-trees.md`.

## Verdict

Not ready to film. One blocker (the Path Mapper count line contradicts the result on screen, in both languages) and several majors: the canvas sits 5 px from the frame edge, the fix step has an unwired Graft input and a suspicious Path Mapper wire, the German Panel still shows `teal`/`orange` next to `türkis`, the colour meanings collide (yellow and orange), the Simplify diagram leaves a dangling root, and "eight capsules" is claimed but never shown. Every number in the model-driven counts checks out (70, 140, 20, 54, 86, 6/8, 3 vs 4 facades).

Counts and values verified against the model: Tower A 9 storeys x 3 facades = 27 branches = 54 capsules; Tower B 11 storeys x 4 facades, storey 10 with 3 = 43 branches = 86 capsules; 70 branches, 140 items, 20 storeys; facade index 2 = South (`facades` in the model). Operation results: Flatten 1x140, Graft 140x1 with 5-index paths, Simplify 70x2 with 3-index paths (only the leading 0 is common), Flip 70x2 to 2x70 — all correct.

## Defects per frame

Severity: blocker = wrong fact or count on screen; major = visible defect or unclear teaching point; minor = polish.

| frame/key | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| `mapper` (and `perstorey` context) | en, de | blocker | After Path Mapper `{A;B;C;D}(i) -> {A;B;C}(i)` the count line still reads `70 branches · 140 items` / `70 Äste · 140 Elemente`, the Panel shows the merged `{0;0;0}` with items 0 to 4, and the tree diagram still has the facade level (70 twigs). `perstorey` then reads `20 Äste`. The coordinator concern is confirmed. | Derive the count line and diagram from the evaluated result: 20 branches, 140 items, storey level as the leaf level, labels adjusted. Or label the count as "input". |
| every canvas frame | en, de | major | The canvas rectangle starts at x = 5 px (the 16 px gutter is not kept). In `en` the Capsules component's left nub is cut at x = 0 (`hook`, `flat`, `mapper`, all EN frames). `layout.md` reports 0 findings, so the layout check does not test canvas margins. | Inset the canvas to x >= 16 px, move the components right, re-check clipping, and add the canvas margin to the layout check. |
| `fix` | en, de | major | Graft Tree input `T` has no wire. The colour list that the caption says is grafted is not on the canvas. | Add the 2-item colour list (Panel) wired to Graft Tree `T`. |
| `fix` | en, de | major | The Path Mapper `M` input appears wired from Capsules: the dashed wire loops into both `T` and `M` (crop of the EN frame, top left of the component). `M` takes the mapping text, not geometry. | Wire only `T`; put the mapping text in `M` (text or Panel). Verify with a zoom on the canvas. |
| `simplify` | en, de | major | After the building node is removed, the two tower boughs meet in a teal V whose apex (about x 347, y 566) is a dangling point with no node. | Remove the apex: towers become the roots, or draw the removed building as a labelled, faded node. |
| `match`, `mistake`, `result` (Panel and chips) | de | major | Panel items stay English (`teal`, `orange`) while the chips say `türkis` / `orange`. The same colour carries two names on one screen. | Localise the colour names in the German version, or show colour swatches instead of text in both languages. |
| `pertower`, `perstorey`, `stats`, `flatten`, `flip`, `result`, `match`, `mistake` | en, de | major | Colour meanings collide. Yellow is the storey colour in the diagram and Panel legend, but also the lit highlight (`stats`, `flatten`, `tree`) and Tower B in `pertower` (`tower B: 86` in yellow). Orange is the item-index colour, but also the Tower B chip (`match`, `result`), and the "no error" chip in `mistake` uses the Grasshopper warning colour. | One meaning per colour: give Tower B its own hue, keep yellow = storey and orange = item index, use a neutral or green chip for "no error". |
| `perstorey` | en, de | major | Caption: "eight where it has four". Eight is never shown; only the top storey (6 capsules) is lit. The storyboard promised a cycle over A storey 0, B storey 4 and the top storey with their counts. | Light one 8-capsule storey (B storey 4, chip `8 Kapseln`) beside the 6-capsule top storey. |
| `flatten`, `graft`, `simplify`, `perstorey`, `pertower`, `flip`, `fix`, `result`, `rule` | en, de | major | The tree diagram level labels (building / tower / storey / facade / capsules) disappear in these frames, while labels stay in `tree`, `branch`, `path`, `item`, `panel`, `stats`, `mapper`, `match`, `mistake`, `why`. The viewer loses the level names exactly where the level changes. | Keep the level legend on every diagram frame; relabel to the levels that exist after the operation (e.g. tower / storey / capsule after mapping). |
| `flatten`, `graft` | en, de | major | Flatten and Graft are introduced on the full 140 capsules. The smallest example (one branch of two items, as in `branch`) is not used. | Show Graft on `{0;0;0;0}` first (`{0;0;0;0;0}`, `{0;0;0;0;1}`), then the 140 case; show Flatten on a one-branch example before the full tree. |
| `pertower` | en, de | minor | Only Tower B is lit (`tower B: 86`); Tower A (54, cyan) is never lit at this step, as the storyboard requires. | Light Tower A cyan, then B. |
| `stats` | en, de | minor | Chips `storey 9: 4 facades` / `Geschoss 9: 4 Fassaden` do not name the tower. | `Turm B, Geschoss 10: 3 Fassaden` and `Turm B, Geschoss 9: 4 Fassaden`. |
| `stats` | en, de | minor | The output wired to the `70` box is labelled `P` (paths) on the Tree Statistics component, and the wires cross, so the source is unclear. | Verify the output names (see fact F2) and route the count output to the box. |
| `panel` | en, de | minor | Caption says the Param Viewer "draws the same tree as 70 branches", but it shows four rows `{0}` to `{3}` and no component name. | Show the branch count on the Param Viewer, and name the component. |
| `why` | en, de | minor | Storyboard path columns (`{0;1;4;2}` vs `{0}`) are missing. The viewers show `{0}`, `{1}`, `{2}` and `{0}` only. | Show the first path of each tree next to its Param Viewer. |
| `flip` | en, de | minor | Only the right (item 1) capsules are lit; the left-then-right sequence from the storyboard is absent. The tree nodes after the flip are drawn white (building colour) although they are branches. | Light left capsules first, then right; use the branch colour for the two nodes. |
| `simplify` | en, de | minor | Storyboard example panel `{1;4;2}` (from `{0;1;4;2}`) is not shown; the panel shows `{0;0;0}`. The example path is lost until the recap. | Show `{1;4;2}` in the panel (the first Tower B branch) so the clip's example carries through. |
| `mapper` | en, de | minor | Storyboard says the dropped `D` is struck through; not drawn. | Strike through `D` in the source rule. |
| `mistake` | en, de | minor | Storyboard's silent-wrong tag is not drawn; the chip is orange. | Neutral chip plus the tag on the component. |
| `rule` | en, de | minor | Storyboard's check badge and side-by-side Param Viewer / Custom Preview are missing. The green chip `read both paths first` (light text on mid-green) is low contrast. | Add the check mark; darken the chip. |
| `recap` | en, de | minor | No tower thumbnail beside the legend (storyboard). | Optional; else update storyboard. |
| `path` | en, de | minor | The path plate covers the Capsules component's output stub. | Move the plate up or right. |
| `hook` | de | minor | "aber welche sitzt wo?" reads colloquially. | "aber welche Kapsel sitzt wo?" |
| `panel` | de | minor | "listet jeden Ast mit Pfad" | "listet jeden Ast mit seinem Pfad" |
| `mapper` | de | minor | "Die Regel {A;B;C;D}(i) nach {A;B;C}" | "Die Regel {A;B;C;D}(i) wird zu {A;B;C}" |
| `fix` | de | minor | "die Farbliste graften" is an anglicism. | "die Farbliste mit Graft Tree je Ast aufteilen" or keep it if the team uses "graften" elsewhere. |
| `why` | de | minor | "Der Param Viewer zeigt warum:" | "Der Param Viewer zeigt, warum:" |
| `mistake` | de | minor | "Links wird türkis, rechts orange, in beiden Türmen." | "Die linken Kapseln werden türkis, die rechten orange, in beiden Türmen." |
| `rule` | de | minor | "verschiedene Strukturen überraschen still" is unidiomatic. | "verschiedene Strukturen führen zu stillen Fehlern" (EN "silent surprises"). |
| `recap` | de | minor | Caption says "Baum", legend row says "Datenbaum". | Use "Datenbaum" in the caption. |

## Fact findings

- **F1 (major, check).** The Path Mapper rule uses letters and `(i)`: `{A;B;C;D}(i) -> {A;B;C}(i)`. Nothing in `research-grasshopper.md` §3.3 gives this notation. The research describes index-based mappings and the constants `item_count`, `path_count`, `path_index`. Check the Grasshopper 1 Path Mapper expression syntax (McNeel docs) before filming. If the letters are teaching notation, say so on screen, or write the component's real expression (numeric indices) and keep the letters as the teaching label.
- **F2 (minor, check).** Tree Statistics outputs are labelled `P`, `L`, `C`. Confirm the Grasshopper 1 output names and which output carries the branch count (the wire to the `70` box is attached to `P`).
- **F3 (minor, check).** Input labels not in the research: Simplify Tree second input `F`, Flip Matrix `D`, Flatten Tree `P`, Custom Preview `G`/`M`, and the name "Param Viewer" (research §3.3 gives "Parameter Viewer"). Confirm against the component docs.
- **F4 (ok).** Facade indices: 0 North, 1 East, 2 South, 3 West (model `facades`). `{0;1;4;2}` = building, Tower B, storey 4, South facade. German `Süden` is correct.
- **F5 (ok).** Tower and storey counts from the model: A 9 storeys, B 11 (top storey has 3 facades), 70 branches, 140 capsules; 54 + 86 = 140; mapped `{B;T;S}` gives 20 branches, 6 or 8 capsules each.
- **F6 (ok).** Flatten 1 x 140 (path `{0}`); Graft 140 x 1, path length 5, last index = old item index (research §3.3); Simplify removes only the leading 0 (Towers differ, so no more common prefix), 70 x 2, path length 3; Flip Matrix 70 x 2 becomes 2 x 70 (research §3.3, uniform lengths).
- **F7 (ok).** Short list against a tree repeats in each branch with no error (research §3.4 McNeel rule). The mistake and fix captions match the rule.
- **F8 (storyboard error).** Storyboard clause 13 says "30 branches"; the correct value is 20 (9 + 11 storeys). The frame shows 20, which is right. Fix the storyboard.
- **F9 (research staleness).** `research-nakagin.md` §3.3-3.4 gives Tower A 11 and Tower B 13 storeys (Wikipedia-based). The model (authoritative) has 9 and 11, and the clip follows the model. Mark the research as superseded for this clip.
- **F10 (German terms).** `Ast`, `Pfad`, `Element`, `Index`, `Geschoss`, `Fassade`, `Datenbaum`, `Turm` are used consistently. Only the Panel colour strings are inconsistent (see table).

## Didactic findings

- **D1 (major).** The smallest-example rule is broken for Flatten, Graft and Simplify (introduced on 140 items). Misconceptions 1 (Flatten vs Simplify) and 2 (Graft keeps grouping, it does not) need the one-branch example. Use `branch` as the base for Graft and Simplify, and Flatten on `{0}: [a, b, c]`.
- **D2 (major).** Misconception 5 (short list repeats, no error) is shown well in `mistake`/`fix`/`result`. Misconception 3 (`{0}` vs `{0;0}`) is not shown; the storyboard lists it at clause 4-5 but the clip uses depth 4 vs depth 1 in `why`, which is a different point.
- **D3 (major).** The "check counts" lesson for Path Mapper (misconception 6 in research §3.5, storyboard item 3) is undermined by the blocker in `mapper`: the clip shows a count that does not match the result. Once the count is fixed, add one explicit count check (`20 x 6/8` = 140).
- **D4 (ok).** The arc holds: tree and path, branch and item, panel and Param Viewer, operations, path mapping to storeys and towers, flip, matching with the mistake, cause (`why`), fix and result, rule and recap. The teaching-model statement is in `hook` in both languages.
- **D5 (ok).** All captions fit two lines in both languages (the German recap and mistake captions are two lines at about 130 characters).
- **D6 (ok).** Plan row coverage: Panel and Param Viewer, Flatten, Graft, Simplify, Path Mapper twice, Flip Matrix, counts per tower and storey, branch matching. Partly missing: the highlighted per-storey counts on the tower (only the top storey is shown; see table).

## Frame coverage

All 22 keyframes viewed in `en` and in `de`: hook, flat, tree, branch, path, item, panel, stats, flatten, graft, simplify, mapper, perstorey, pertower, flip, match, mistake, why, fix, result, rule, recap. Canvas crops for `fix` and `stats` checked at 3x.
