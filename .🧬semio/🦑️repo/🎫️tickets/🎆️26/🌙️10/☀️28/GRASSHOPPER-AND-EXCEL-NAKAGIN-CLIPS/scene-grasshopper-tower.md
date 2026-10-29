# Scene report: grasshopper_tower, `Beat3_Tower`

File: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`. Storyboard: `storyboard-grasshopper-tower.md`. Audit: `audit-grasshopper-tower.md` (all blockers, majors and the coordinator decisions are handled below). Helper scripts: `gh-tower/` (probes, patch scripts, `scene_v1.py.txt` = first version). Frames: `🗑️generated/frames/🦗️grasshopper/Beat3_Tower/{en,de}`.

## What was built (22 clauses)

hook, parts, core, shaft, series, facades, axes, items, orient, types, count, nowire, mistake, fix, capital, ontop, podium, why, towers, total, swap, recap. English and German authored natively; every German clause fits two caption lines.

- Layout: canvas left, Rhino `Perspective` viewport right; five coloured groups (Core blue, Planes orange, Capsules green, Capital pink, Podium violet) around the objects of each part. The same colours outline the drawn parts in `parts` and `recap` (cores, capitals, podium outlined, capsules tinted green).
- Everything on the canvas is solved by the engine: `canvas("tower")` for core, series, facades, orient, count, cull, capital and podium; small custom definitions built with `define()` and solved by `evaluate()` for the type pattern (Panel to List Item with a Series index, output panel computed) and the swap (Boolean Toggle, Multiplication, Addition, Subtraction; the toggle is flipped by an engine input override False to True and gives B 35 to 0 and D 35 to 70).
- Counts: the sweep runs from `high // 3` to `high` storeys, where `high` is the number of full storeys in the engine's `Tree Statistics` lengths (10). The formula reads storeys, facades (branches of `Rotate`) and items (planes per branch) from the engine: `k × 4 × 2`. No typed numbers: capital animation targets are the slider maxima of the engine definition, the core sweep goes to `high // 2`, plane values come from the engine planes, podium height from the model (2 storeys × 2.75 m).
- Typical mistake and fix (coordinator decision 4): with the Cull Index list emptied by an engine override, `Tree Statistics` gives 88 capsules for tower B (sum of branch lengths) and two red crosses mark the two west planes of the top storey at the capital level; the fix clause restores the `cullp` indices and the sum reads 86.
- Towers clause shows `A 9 × 3 × 2 = 54`, `B 11 × 4 × 2 = 88`, `Cull Index −2`, `B = 86` (facades from the model's `facade_storeys`, items from the engine); towers A and B are labelled in the viewport. Slider names are "Capsule storeys A/B" (de "Kapselgeschosse A/B").
- Terminology (decision 5): the canvas label "Capsule storeys" is "Kapselgeschosse" in German; captions use the same word; "storeys" is used for building levels only. German lists use "; " between decimal numbers (`-12,475; 2,75; 5,5 m`).
- Units are on all sliders and on the plane origin (`m`); `Capital base Z = 33 m` is the engine value of `tot` (base of the capital), not the centre.
- Swap: the engine-evaluated type counts change and the drawing recolours type B capsules with the colour of type D (the Nakagin drawing has no per-type geometry parameter); the caption says "changes". A colour legend A to D is on the canvas in `types` and `swap`.
- Model is authoritative (decision 1): 54 / 86, podium 5.5 m, type counts 36/35/34/35; the research values 63/77 and 7.5 m are not used.
- "Teaching model" is said once, in `recap`.

## Audit items not done, and why

- Storyboard clause 20 legend of ten terms: replaced by the five coloured groups with their names (recap shows them next to the outlined tower).
- Empty band between title and stage on every frame: part of the shared ClipScene layout, not changed.
- Group frames reach the canvas edge in a few frames, and "1.00" tick labels sit close to wire grips: kept (toolkit geometry).
- Capital clause still draws the capital without the core under it (the core would hide the dimension line); the `ontop` clause shows it on the rising tower.
- List Item output nickname "i" and Tree Statistics outputs P, L, C come from the engine catalogue; to check in Rhino 8 before filming (audit fact 7).

## Keyframe review

All 22 frames in en and de were looked at (the final capture of `swap`, `total`, `recap` after the last layout fix; the others from the capture before it with only the toggle label changed). Remaining strict findings (22 en, 25 de, in 11 of 22 frames) are slider value text against its own glyph strokes, wire grips touching component edges, and dimension labels or axis letters over the viewport drawing; none hides information.

## Test result

See the line at the end (presentation test run after the last change).

Presentation tests after the last change (`🧪️tests/🎭️presentation/🐍️.py`): the bilingual lifecycle of `Beat3_Tower` (22 clauses, en and de) rendered without error and the ten test results printed before the log stopped were all passes. The log has no final "Ran N tests" line (the run was cut off during `Beat4_Casting`), so the full-suite summary, including the earlier toolkit failure `CanvasWidgets.test_states_and_panels_come_from_the_solution`, was not re-confirmed.

## Quality round (quality-grasshopper-tower.md)

Done: mistake draws the two phantom capsules (red, crossed) and fix removes them while the indices 6 and 7 wire into Cull Index; nowire sweeps the slider inside the clause so it differs from count; core and shaft grow from a smaller start with the equation `13 × 2.75 m = 35.75 m`; the capital is drawn on a core stub; items shows the 2.75 m pitch dimension and the caption says "to each side"; types numbers the eight planes 0 to 7 (same numbering as the Cull Index); why is now an argument drawn on the building (podium outlined as context, cards "5 components, one extrusion" vs "49 components, 140 copies") instead of a bar chart; towers highlights tower A (blue) with its storeys and facades, total tints both towers; swap is B to C with a real geometry change (window Ø 1.30 m to Ø 1.60 m drawn on two capsules, counts B 0 and C 69 from the engine, the tower recoloured); hook names the Nakagin Capsule Tower; German podium and count captions shortened or fixed. Still 22 clauses. Not done: mini plan of one storey with planes 0 to 7, Capsule storeys vs "capsule floors" wording (kept "storeys" per coordinator), a count formula built from three small panels.

Review: all 22 frames read in en and de after the last change (full captures, then the changed keys re-captured). GH model tests (`🧪️tests/🐍️.py`): 57 pass. The presentation test suite was not re-run after this round.

## Final sweep (sweep-grasshopper-tower.md)

Done: `nowire` now shows slider, Tree Statistics and formula at the same value (5 storeys, "same wires, other value"); German labels of custom nicknames (Capsule per slot to Kapsel pro Slot, sliders, plane panel header Ebene); swap panels read "Type B: 0" and "Type C: 69" (de "Typ B", "Typ C"), values from the engine; the sum text sits inside the group; wires moved off slider maximum labels in count, capital and ontop; the podium group no longer reaches the viewport. All 22 keys re-captured in en and de after these edits (the last layout tweak to the swap panels re-captured for swap only); changed frames checked. Remaining strict findings are the known slider text, grip and viewport annotation findings.
