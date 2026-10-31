# Morph Done: grasshopper_anatomy (`Beat1_Anatomy`)

File: `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`. Storyboard: `morph-grasshopper-anatomy.md`.

## Implemented (toolkit `🔀️morph`)

Token-to-token hops (`_hop`) use part-wise `pairs=` so the label travels with the pill; capsule-to-token morphs (`_cap_to`, `_to_cap`, scatter `pairs_fn`) use the two biggest drawn parts of the capsule copy (shell to ring, second face to dot).

| clause | morph | direction |
| --- | --- | --- |
| script | capsule copy (shell, face) flies into the `Nakagin_Capsule.gh` chip (box, label) | concrete > abstract |
| slider | width value token (yellow) from the slider to the viewport dimension | abstract > concrete |
| box | three dimension tokens (yellow, teal, green) from the viewport into the X, Y, Z grips | concrete > abstract |
| flow | a `Box` token leaves the Center Box output grip and lands on the viewport box | abstract > concrete |
| sliders | length 5.0 (teal) and height 3.0 (green) tokens fly from the sliders to the box dimensions while it grows | abstract > concrete |
| preview | the box collapses into the `25 m³` token that lands on the Panel | concrete > abstract |
| panel | the width dimension value flies into the Panel row | concrete > abstract |
| params | the Panel value flies into the floating Width parameter | abstract > abstract (identity of the 2.5) |
| toggle | the True token becomes the round window ring on the capsule | abstract > concrete |
| valuelist | a type C token flies from the Value List to the capsule window | abstract > concrete |
| plugin | the window ring becomes the Ladybug Window Brep parameter | concrete > abstract |
| item | capsule copy of Tower B level 0 becomes the first Series row | concrete > abstract |
| series | nine capsule copies scatter into the nine list rows | concrete > abstract |
| count | the two new list rows (9, 10) become capsule copies on Tower A | abstract > concrete |
| tree | the two capsules of the chosen branch become tree leaves; four segment tokens gather into the path chip | concrete > abstract |
| mistake | the `[1]` token splits into one dot per capsule that has an index 1 (35 on Tower A) | abstract > concrete |
| address | the dots gather into the one path-and-index tag on the capsule | abstract > concrete |
| legend_tool | the eight numbered markers on the canvas fly into the number discs of their legend rows | concrete > abstract |
| legend_data | the six numbered markers on the Panel fly into the number discs of the data legend | concrete > abstract |

Chain: capsule > Series row > leaf > path chip > `[1]` > many dots > one tag on the capsule > numbered marker > legend row.

## Verification

- Morph frames: `T/🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/en` and `/de`, full run of all 18 listed clauses without error (first full runs after the part-wise change; two earlier runs crashed on empty capsule parts and were fixed by choosing the two biggest drawn parts). Mid frames read in en: script, item, count, legend_tool, plugin, toggle, box (earlier run); de: legend_data, toggle, tree (earlier run).
- Captions: every clause scanned with `caption_bar` in en and de: 0 wrapped to 3 lines (scan script `scratchpad/caps.py`). Earlier fixes: series en/de, valuelist de, mistake de.
- Quick presentation contract tests (narration, dispatch, chapters, imports, provenance, language): 6 of 6 OK (run before the last edits; not repeated).

## Open items

- `T/capture.py` layout keyframes (en and de) were not rerun; the mid frames show no overlaps except the intended flying discs.
- `_morph.capsule_pairs` (announced late) is not used; hand-picked pairs work.
- Mid-flight silhouettes of single-part sources (e.g. the plugin ring) are plain blobs; the colour and destination carry the identity.
- Beat 9 (valuelist) flies one type token only (C); the live type follow-through is unchanged.

## Audit round (morph-audit-grasshopper-anatomy.md)

- Slider beat captured (`slider` added to the key list; `slider-03-mid.png` shows the yellow 2.5 token landing on the dimension label). `index.md` of en and de lists every clause with each morph's own play.
- Capsule morphs now use `_morph.capsule_pairs` (script, item, series, tree leaves, count); the hand-written part picker is gone.
- Plugin: the window ring morphs into a labelled amber `Window` cell token (no grey blob), which fades as the real parameter node appears.
- Clutter: the tower capital (dark wedge) is no longer drawn in the viewport towers (live and static drawings match); tree leaf tokens land left of the Panel rows instead of on the text; the "[1] exists" and "expected" tags appear only after the dots have flown; the panel and params value tokens start left of the Panel instead of on row 0.
- Storyboard values updated to the model (path {0;0;0;3}, `[1]` in 35 of 41 branches, Tower B nine levels, Tower A eleven levels).
- Frames: full en and de run of 19 clauses, no error; read in en: slider, plugin, tree, mistake, address.
- Still open: the address gather of 35 dots shows a green/orange blob over the tower in mid-flight (convergence of the silhouettes); `capture.py` layout keyframes were not rerun; legend and count flights are short and mostly overlap their end state.

## Weak-item round

- box: three coloured dimension lines (yellow, teal, green) morph into the three wires (static curves, swapped for the live wires after the morph); the value tokens still hop into X, Y, Z. Frame `box-03-mid`: tokens at the grips and the wire curves arriving.
- address: the 35 dots are half size, gathered with lag 0.06, path arc 0.9, 1.2 s each (about 3.4 s total). Frame `address-04-mid`: staggered streaks along arcs; a green smear at the tag is still visible where the first dots arrive.
- count: tokens fly 1.6 s with path arc 1.0 into the two new capsules; the capsule copies sit on the already lit capsules, so the landing is readable only as arrival, not as a new object (`count-04-mid`).
- legends: number discs fly 1.3 s with arc 0.8, lag 0.15 (3.2 s total) while the originals dim to 25 percent; `legend_tool-04-mid` shows eight discs in flight, none on the canvas end state.
- Full en and de run of box, count, address, legend_tool, legend_data: no error. English frames read; German frames captured, not read.
- Open: the green smear at the address tag, and count's landing is subtle because the tower is already lit.

## Second audit round (full runs)

- Full runs without key filter, en and de, after the toolkit's new automatic part matching: 24 clauses each, 38 mid frames each, no error (`🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/{en,de}`; `index.md` lists every clause).
- count: the flight is its own `play` of 1.6 s with two labelled number pills (`27.5 m`, `24.75 m`, in the German run `29,0 m`, `26,1 m`) travelling from the list rows to the top of Tower A; mid frame now shows both pills over the tower (the earlier item-kind dot was too small to see).
- address: the 35 dots are open red rings; they travel on arcs (lag 0.05) to the tag position and fade, then the tag fades in; no smear in `address-04-mid` (en, de). The red "[1] exists" strip sits lower (y -1.4) and no longer touches the chip.
- legend_tool, legend_data: the number discs fly in their own 3.2 s `play` (1.3 s each, lag 0.15, arc 0.8); the originals on the canvas are hidden when their copy leaves; mid frames show discs between canvas and legend and part of the rows already filled.
- box: the viewport dimension lines are switched off after the value tokens land and come back before `sliders` (live viewport; the swap is instant, not a fade).
- Frames read: en script, sliders, series, count, address, legend_tool, legend_data, box, flow, preview, panel, params; de address, legend_tool, count. The other frames were captured but not read one by one.
- Still weak: script (the capsule flight is not visible in the sampled mid frame; the chip is already complete), sliders (the two flying pills show no label mid-flight), flow (the Box output token is a small dark pill), dimension-line detach is an instant switch rather than a morph.

## Last three (script, sliders, flow)

- Captured en and de (keys script, slider, box, flow, sliders, preview, panel, params, valuelist), no error.
- sliders: the token hops no longer pass explicit pairs, so the toolkit matches the glyph parts itself and the labels travel: `sliders-03-mid` shows two pills "5.0" (teal) and "3.0" (green) in en and de.
- flow: the Box token is 1.25 times the normal size, in the viewport box colour, labelled "Box", own 1.4 s play; `flow-05-mid` shows it above the box with its label (a faint ghost trails it).
- script: the capsule to chip flight is its own 1.4 s play (`script-04-mid`). The flying capsule is tiny (a tower capsule at 0.065 scale) and shows only as a faint grey streak beside the chip at the midpoint, so the flight is sampled but still subtle.

## Lessons pass (morph-review-excel-references.md, section 3)

1. One actor: capsule copies, value tokens and number discs are the same objects as in the drawing; no side cards. OK.
2. Bound pair: the final tag (`{0;0;0;3} [1]`) stays on the capsule until the legend; value pills hand over to a lasting partner (panel row, floating Number, dimension). The short-lived pills (slider, panel, params, count, flow) fade after landing; the partner stays. Partly OK.
3. Number moves, state follows: slider, sliders, panel, params, count move the value before the partner shows it; the box and window states are live and change at the same time. Partly OK.
4. Tracker sweeps: fixed in `series` (Count set to 9 at once, the nine capsules fly into the nine rows; no 2 s sweep). Remaining sweeps are continuous quantities (width, length, height, Value List index) and `count` 9 to 11 (two items, 1.6 s).
5. Actual event: warning and error still swap whole states by fade (no token replaces the text `tall` by `2.5`); open.
6. Structure before morph: `item`, `tree`, `legend_*` still start the morph beside fades of the previous stage; the legends and `address`/`count` flights now run as their own plays after the swap. Partly OK.
7. Legibility: value tokens keep their labels in flight (`sliders`, `flow`, `count` pills); flock rings are small by design; frames read at 640x360 only.
8. Frames at 1280x720: not captured (capture_morph is fixed at the low size); open.
9. Both directions, chained: `script` lifts the lit capsule out of the tower (4x, yellow outline, `script-03-mid`) and morphs it into the chip; capsule to Series row to leaf to path chip to `[1]` rings to one tag on the capsule; count rows back to the tower. OK.
10. Narration match: each clause plays its morph under its own caption; the series caption states a step of 2.90 m while the slider currently shows 2.80 (model value changed by the codex session after the caption was written): needs the transcript owner.
11. Back-link: count to lit capsules on the tower and the `[1]` token to 35 capsules exist; the `25 m³` goes only to the Panel (no return); open.
12. Hygiene: `pairs=` via `capsule_pairs`, automatic part matching for tokens, gather/split with arcs; path tokens carry the concept text.

## Final round (narration figures, repair event, structure, back-link)

- Narration figures: `_figures()` fills every model number into the NARRATION templates (slider value and domain, length/height targets, width goal, volume, window diameter, storey step, capsule levels per tower), decimal comma in German; the series caption can no longer disagree with the slider (`2.80` in both). The code uses `NB`/`NA` (capsule levels of Tower B and A from the model) instead of 9 and 11. The caption scan (`caption_bar`, en and de) found no clause wrapping to 3 lines. Numbers not taken from the model are labels of the demo itself (the texts `tall` and `2.5` typed into the Panel, path `{0}`).
- warning and error: the actual event. `warning-04-mid` / `error-04-mid` (TransformMatchingShapes): the text `tall` in the Panel morphs in place into `2.5`; then (`-05-mid`) the component turns from orange or red to grey, the balloon (and in German the gloss and the "first problem" marker) fade and the output Panel changes from `Box, null` to `Box, Box`. The repaired state replaces the staged group in place (no whole-state fade); a sweep removes staged leftovers (first run left a ghost of the repaired state in later clauses; fixed, `count-04-mid` clean).
- Structure before morph: `item` and `tree` finish their stage fades first, then the capsule flight is its own play (1.2 s); `legend_*` and `address` were already so.
- Back-link: at the end of `preview` the `25 m³` token morphs back into the viewport as a faint dashed volume outline (`preview-04-mid`): the data still describes the box. It fades with the next stage.
- Frames read this round: en warning-04/05, preview-04, count-04, item-03, panel-03, tree-03; de error-05. The frames of the earlier slider/sliders/series captures were overwritten by the later partial runs and not recaptured after the figures change.
- Open: the eight number-less captions are unchanged; `tree` shows a different chosen branch now ({0;0;1;0}) because the model changed (derived, nothing typed).

## Last polish (final review)

- tree: the explain notes start 0.7 right of the sheet centre line and moved down (no clipping at the viewport edge), "branch = facade group" / "Ast = Fassadengruppe" shortened, the viewport mark reads "facade group" / "Fassadengruppe" (`tree-end` en, `tree-07-mid` de).
- path levels: after the four segments gather into the path chip `{0;0;1;0}` (moved up to y 1.55), the labels BUILDING; TOWER; STORY; FACADE (de GEBÄUDE; TURM; GESCHOSS; FASSADE) appear one by one in a 2x2 grid under it (first single-row layout was too wide, the German one ran into the viewport); they leave with the chip in `address`.
- count: the two flying pills land 0.95 right of their capsules and 0.4 apart vertically, so "28 m" and "25.2 m" no longer stack (`count-04-mid`); the tower marks stay on the left.
- Captures: one full run en and de (44 mid frames each, no error, folder `🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/{en,de}`), then tree, mistake and address recaptured after the grid fix (en and de, no error; the folder now holds those three clauses only, the full run is copied to `🗑️generated/morph-prev/full-r14`). Frames read: en count-04, tree-07 and tree-end, de tree-end, tree-07.
