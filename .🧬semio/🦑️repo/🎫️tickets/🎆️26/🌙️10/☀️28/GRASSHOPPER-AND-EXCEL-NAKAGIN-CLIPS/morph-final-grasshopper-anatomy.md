# Final Morph Review: grasshopper anatomy (`Beat1_Anatomy`)

Scene: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`. Frames reviewed: `🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/en` (only warning, error, panel, item, count, tree frames exist in this capture; the other clauses are judged from code).

## Tree path sanity check (tree / address clauses)

Verified against the current model (`_gh.aggregate_tree()` evaluated live, nakagin has uncommitted edits): 75 branches, every path has 4 levels `{BUILDING;TOWER;STORY;FACADE}`, ragged (1 or 2 capsules per branch). Tower A (`path[:2] == (0, 0)`) has 42 branches, 34 with at least 2 items, so `chosen_path` (first branch with >= 2 items, e.g. `{0;0;1;0}`) and the `[1]` ambiguity story still hold. The frames show `{0;0;1;0}` in panel, viewport mark and chip consistently. `slot.path` is `(building, tower, storey, facade)`, matching narration "occupied facade group". Verdict: the clauses are still correct; no code change needed. Gap: nowhere is the four-level meaning of the path named on screen (see polish 3).

## Per clause

| clause | rating | why |
| --- | --- | --- |
| hook | ACCEPTABLE | Fade-in of the staged scene; the lit capsule of the whole tower is the identity anchor for the next clause. |
| script | GOOD | The lit capsule enlarges, then its shell and face fly into the `Nakagin_Capsule.gh` chip: concrete becomes the abstract file. |
| slider | GOOD | The value token travels from the slider to the viewport width dimension while the box grows. |
| box | GOOD | Three coloured dimension lines morph into the three coloured wires, and tokens fly into X/Y/Z grips: real identity of dimension = wire = input. |
| parts | ACCEPTABLE | Fade swap to the nickname component plus callouts; explains by labels, not morph. Fine for naming. |
| flow | GOOD | Pulses along wires, then a `Box` token leaves the output grip and lands on the viewport box. |
| sliders | GOOD | Length and height tokens fly to the growing dimensions. |
| live | ACCEPTABLE | Tracker-driven live update (the slider IS the cause); a readout is enough. |
| preview | GOOD | Box collapses into the volume token, then the token becomes a dashed volume ghost: hidden geometry versus data is shown, not told. |
| warning / error | GOOD | The text in the Panel retexts into 2.5 in place (`retext`), then component, bubble and output transform to the repaired state; the repair is one visible event. |
| panel | ACCEPTABLE | The dimension value flies into the Panel row; fine, though only one small token in a busy scene. |
| params | ACCEPTABLE | Value flies Panel to floating parameter; abstract to abstract, but the ring on input X makes the port-versus-free distinction. |
| toggle | GOOD | True token becomes the round window ring on the capsule. |
| valuelist | GOOD | Type C token flies from the list to the capsule window while the viewport capsule follows the index. |
| plugin | ACCEPTABLE | The window ring becomes the Window Brep parameter (identity kept); install steps and tint bar are labels. |
| item | GOOD | The first Tower B capsule copy flies part-wise (shell to ring, face to dot) into the `0 m` list item. Exactly the capsule-to-data-item morph the owner asked for. |
| series | GOOD | Every capsule level of Tower B scatters into its own list row (lagged, capsule pairs). |
| count | GOOD | New list rows hop to the new capsule levels of Tower A while the tower lights up more levels; abstract to concrete in reverse. |
| tree | GOOD | The branch's capsules fly into the tree Panel rows (frame tree-04 shows two coloured dots landing on rows B and C), and the path segments gather into the `{0;0;1;0}` chip. |
| mistake | GOOD | The `[1]` token splits into one red ring per capsule that is index 1 in some branch: "one index, many capsules" is literal. |
| address | GOOD | The rings fly into one green `path [1]` tag at exactly one capsule. Strong identity close. |
| legend_tool / legend_data | GOOD | The numbered markers on the canvas fly into the legend discs; the glossary is connected to the objects it names. |

## Verdict: SHIP

The video is morph-driven wherever an idea is explained: capsule to chip, item, list, tree rows, path chip, ambiguous index and legend discs all keep identity. Remaining polish is minor, in priority order:

1. `tree` clause explain notes (`explain`, around `_at(1.1, 0.7)` with `anchor="l"`): "branch = occupied facade group" is clipped by the viewport edge in tree-04, and the viewport mark "occupied facade group" (`path_marks`, offset `-.6, -.22`) is cut at the tower. Shorten the string or move `explain` left by about 0.3 and flip that mark's `dx`.
2. `count` clause (`count_hops`, offset `+0.55`): hop targets and the `ΔZ` marks overlap near the top of Tower A in count-04 (labels "28 m" stack on "25.2 m"). Offset hop targets to the right by about 0.9 or hop only the new rows with fading marks.
3. Name the path levels on screen: after the `gather` into `path_chip`, morph the chip into four small labelled slots `building · tower · storey · facade` (or add this text to `explain`, `Pfad = Gebäude;Turm;Geschoss;Fassade`), since the model proves four levels and the video currently only says "address".
4. Capture hygiene: the frame folder lacks the other clause frames (script to legend), so those ratings rest on code; re-run the capture before final render.
5. `parts` and `live` remain fade-only; acceptable, but a Name/Nickname token pair morphing out of the component would raise parts to GOOD.
