# Morph Audit: Grasshopper / Anatomy (`Beat1_Anatomy`), read-only

Inputs: `morph-grasshopper-anatomy.md` (planned beats, storyboard), `morph-grasshopper-anatomy-done.md` (built), scene `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`, frames `T/🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/{en,de}/` with `index.md`. Nothing captured.

Frame coverage: 18 of 24 clauses have frames (script, box, flow, sliders, preview, panel, params, toggle, valuelist, plugin, item, series, count, tree, mistake, address, legend_tool, legend_data). Clauses hook, slider, parts, live, warning, error have none. en and de `index.md` are identical in animation types. Verdicts below use `en/` frames; `de/` was spot-checked (plugin-03, mistake-04, legend_tool-03), same layout and German captions fit.

## 1. Per planned beat

| # | Clause | Verdict | Evidence |
| --- | --- | --- | --- |
| 1 | script (capsule > .gh chip) | WEAK | `en/script-03-mid.png`: canvas with chip and four static capsule copies A to D; no capsule in flight, the lit tower origin is not in frame, no tether. |
| 2 | slider (width value > knob) | MISSING | No frames: clause `slider` not in `index.md`. |
| 3 | box (dimension lines > X, Y, Z wires) | WEAK | `en/box-03-mid.png`: value pills 2.5 / 4.0 / 2.5 at X/Y/Z in yellow/teal/green (identity readable), but viewport dimension lines do not detach; wires are FadeIn x6. |
| 3b | box (output grip > viewport box) | WEAK | `en/flow-05-mid.png`: one unlabelled green pill at the canvas/viewport edge; box already shown; no growth or tether. |
| 4 | flow (pulses > box edges) | WEAK | `en/flow-03-mid.png`: small dots on wires, no values; no edge growth in any frame. |
| 5 | sliders (knobs > edges, tethered) | WEAK | `en/sliders-03-mid.png`: teal and green pills float unlabelled between canvas and viewport, no tether; `en/sliders-05-mid.png`: end state (4.50 / 2.80 m), no knob flight. |
| 6 | preview (box > 25 m³ row) | WEAK | `en/preview-03-mid.png`: viewport already empty, chip "25 m³" placed above the Volume node, not on the Panel row as planned; no box in flight. |
| 7 | panel (label > rows) | BUILT | `en/panel-03-mid.png`: yellow "2.5" pill readable; collides with Panel row 0 and the Box top. |
| 8 | toggle (True knob > round window) | WEAK | `en/toggle-03-mid.png`: finished window and "Ø 1.30 m" chip; no knob in flight; amber outline ring on the side face reads as a blob. |
| 9 | valuelist (row > capsule window) | BUILT | `en/valuelist-03-mid.png`: "C" token on the window readable, amber chip, colour carried; ghost "Type B" and "index 0 > Type A" text overlaps the list. |
| 10 | plugin (ring > Window Brep > tint) | WEAK | `en/plugin-03-mid.png`: grey hexagon blob over the "LB Direct Sun Hours" node; ghost Value List rows and "index 2 > Type C" behind it. Viewport tint is built. |
| 11 | item > series (capsule > row > rows) | WEAK | `en/item-03-mid.png`: red row token in the Series panel, capsule flight not visible; `en/series-04-mid.png`: rows and yellow capsules present, scatter not visible as motion. |
| 12 | count (new rows > capsules) | WEAK | `en/count-03-mid.png` and `en/count-04-mid.png` are identical; the two new capsules are already on the tower; no token in flight. |
| 13 | tree (capsules > leaves, path chip) | WEAK | `en/tree-03-mid.png`: red dot on the "Capsule D" row text, path chips already present, segments not built. `en/tree-05-mid.png`: red pills over "branch = occupied facade group". |
| 14 | mistake ([1] > one per branch) | WEAK | `en/mistake-04-mid.png`: red dots spread across tree rows and the "expected: 1 capsule" note, crossing the Brep node; dots carry no "[1]" identity. |
| 15 | address (dots > one tag) | WEAK | `en/address-03-mid.png`: green chip "{0;0;0;3} index 1: one capsule" already placed; `en/address-04-mid.png`: large green-orange blob over the tree panel reaching into the viewport. |
| 16 | legend_tool (markers > rows) | WEAK | `en/legend_tool-03-mid.png`: numbered discs 1 to 8 still on the canvas, rows FadeIn; no disc in flight; `-04` not checked. |
| 17 | legend_data (brackets > rows) | WEAK | `en/legend_data-03-mid.png`: discs 1 to 6 still on the Panel, rows FadeIn; no flight visible. |

Counts: BUILT 2 (panel, valuelist), WEAK 15 (incl. 3b), MISSING 1 (slider).

## 2. Remaining fade-swaps and cuts (from `en/index.md`)

Every clause opens with `FadeOut` of the previous clause (scene cut, acceptable). Concrete/abstract swaps that should be morphs:

- `slider` (no frames): knob and value are not captured at all.
- `box-03`: six `FadeIn` for the wires before the PartMorphs; `box-04`: three FadeOut without a box output morph.
- `flow-03`: `FadeIn` + `Succession` for the pulses; no morph to the box edges.
- `preview-03`: `Transform` + `PartMorph` is planned, but the frame shows only the end.
- `toggle-03`: `FadeIn` x3 for the window chips beside the one `Morph`.
- `valuelist-03`: `FadeIn` x5 for the list rows beside the one `PartMorph`.
- `plugin-03`: `FadeOut` x6 and `FadeIn` x4 around one `Morph`; the ring-to-hexagon change is a fade.
- `item-03`: `FadeIn` x9 and `FadeOut` for the list panel; only one `PartMorph` (capsule to row).
- `series-05`: nine `FadeOut` after the swarm; the capsule copies fade instead of landing.
- `count-05`: `FadeOut` x2 after the morph; the new rows are not seen leaving the list.
- `tree-03`: `FadeIn` x5 for the tree chips beside two `PartMorph`s.
- `mistake-03`: `FadeIn` x4 for the index dots; `split` is not visible.
- `address-03`: `FadeIn` x2 for the tag beside `Create`; the gather is not a morph.
- `legend_tool-03` and `legend_data-03`: `FadeIn` x9 / x2 for the rows; only a LaggedStart for the discs.

## 3. Direction and chain

- Concrete > abstract in frames: panel, item, series, tree; planned but not visible: script, box, legend_tool, legend_data.
- Abstract > concrete in frames: valuelist (C token on window), mistake (dots), address (tag); planned but not visible: slider, sliders, flow, toggle, plugin ring, count.
- Both directions occur. Chain visible in frames: lit capsule on tower (`item-03`) > red row token (`item-03`) > rows and yellow capsules (`series-04`) > red dot on leaf "Capsule D" (`tree-03`) > path chip `{0;0;0;3}` > `[1]` dots (`mistake-04`) > one tag and lit capsule (`address-03`). Chain links 1 to 3 (script, slider, box) and the legend end are not visible.
- Storyboard path `{0;0;3;1}` is stale; frames show `{0;0;0;3}`. Done file's "35 branches with [1]" is correct against the frame text ("3 of 41 branches shown").

## 4. Collisions and clutter

- `item-03` (en and de): ghost of previous plugin text ("Package Manager", "Food4Rhino", "less sun / more sun") and "window" Brep node overlapping the Count slider; dark wedge on the tower top. The wedge persists in `series-03/04` and `count-03/04`.
- `valuelist-03` and `plugin-03`: ghost Value List rows overlap the canvas nodes.
- `panel-03`: "2.5" pill covers Panel row 0.
- `tree-03` and `tree-05`: red dots and pills over text rows and the panel header.
- `mistake-04` (en and de): dots cross tree rows and the "expected" note; de "Fenster" label overlaps the Brep node.
- `address-04`: green-orange blob over the tree panel and into the viewport; red "[1] exists" strip still behind.
- `legend_tool-03`: ghost "Series" and "Capsules" text under the canvas.

## Verdict: FIX

Fix list:

1. Capture `slider` (en and de); beat 2 has no frames.
2. Re-capture mid frames at alpha 0.4 to 0.6 for script, box, flow, sliders, preview, toggle, count, legend_tool and legend_data; if no token is in flight, implement the planned flight (currently fades or static end states).
3. `box`: fade out the viewport dimension lines only after their token has left; morph the three wires instead of FadeIn.
4. `flow`: give the pulse tokens their values (2.5 / 4.0 / 2.5) and grow the box edges from the corner; add the output-grip token with a tether (`flow-05`).
5. `sliders`: label the two pills (4.5, 2.8) and add dashed tethers from knob to edge.
6. `preview`: place "25 m³" on the Panel row as the storyboard says; keep the box visible during the collapse.
7. `toggle`: morph the True knob to the window ring; remove the stray amber outline ring.
8. `plugin`: replace the grey hexagon blob with a labelled Window Brep node; hide the Value List ghost.
9. `item` / `series`: fade out the plugin residue before the item; remove the dark wedge on the tower top; move the Window Brep node off the Count slider.
10. `count`: the two new rows must visibly leave the list and land on the tower (`count-03` equals `count-04`).
11. `tree`: move the red dot off "Capsule D" (`tree-03`) and the pills off the header (`tree-05`); build the path segments or drop that claim.
12. `mistake`: dots must read as "[1]" tokens and avoid tree row text; hide "expected: 1 capsule" during the flight.
13. `address`: replace the green-orange blob (`address-04`) with a small chip docking onto the capsule, kept out of the tree panel.
14. `legend_tool` / `legend_data`: capture the discs in flight; remove ghost text.
15. `panel`: move the "2.5" pill off Panel row 0.
16. Update the storyboard: path `{0;0;3;1}` > `{0;0;0;3}`; preview target row; the [1] count is 35 branches with index 1 (of 41).

## Summary

1. 18 beats audited (17 storyboard beats plus 3b): BUILT 2 (panel, valuelist), WEAK 15, MISSING 1 (slider, no frames).
2. Built and readable: panel label and the valuelist C token on the window; params (not in the storyboard) also shows the 2.5 carried into the floating Number.
3. Most flights are not in the sampled mid frames (count, sliders, toggle, legend); the rest are blobs or ghost text (plugin, address, mistake).
4. Both directions occur; the item > series > tree > `[1]` > address chain is visible, the script > slider > box chain start and the legend end are not.
5. Verdict FIX; fix list above, full report at `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/morph-audit-grasshopper-anatomy.md`.
