# Scene: grasshopper_anatomy (`Beat1_Anatomy`)

File: `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py` (class `Beat1_Anatomy`, title "Anatomy of a Grasshopper Script" / "Aufbau eines Grasshopper-Skripts"). Storyboard: `storyboard-grasshopper-anatomy.md` (superseded in details by this file). Audit: `audit-grasshopper-anatomy.md`.

## What was built

- Stage: ribbon on top, Grasshopper canvas sheet left, Rhino viewport right. The viewport shows the live Nakagin drawing: a plain box from the Center Box output (sizes read from the engine solution each frame), a finished capsule type A to D, or Tower A / Tower B with the addressed capsules outlined.
- Colour linkage: width yellow, length teal, height green. The slider ring, the wire into X/Y/Z, the dimension line and its label share the colour; the floating Width parameter and its Panel use the width colour.
- Every number comes from the engine or `🏢️nakagin`: slider settings, Box sizes, volume 25 m³, panel rows, series 0 … 22 / 27.5, tree paths with capsule types, the 27 branches of Tower A, window diameters per type, type names.
- Live values: `ValueTracker` for width, length, height, Count and the Value List index, drawn with `self.live`.
- Engine addition (anatomy section of `T/gh-engine/build_model.py`, regenerated into `🧱️model/🔣️.json`): toggle, value list, warning and error objects and views; the floating Number parameter `fl` now receives the Width slider by a wire (it reads 2.5, no longer the interior width 2.3). Cases file: `anatomy-floating-param` now expects 2.5. G tests 57 OK, schema test OK.

## Audit fixes (blockers, majors, minors)

- Perspective header collision: the type note is two lines at the bottom right of the viewport.
- Floating Width param: wired from the slider, reads 2.5.
- Nickname: the body shows "Center Box" in clause 4 and "Box" from clause 5 on; the callouts say "Name: Center Box" and "Nickname: Box".
- Center Box inputs: Width, Length and Height sliders are wired into X, Y, Z from the start (no unwired defaults).
- Warning: Panel `2.5` + `tall` into Z gives orange with one Box and one null (output panel shows both); the caption says so; the viewport draws the one box. Error: panel `tall` alone, red, empty viewport, "first problem" marker in red.
- German legends speak German terms first (Skript, Komponente, ...), English second; English legend speaks both. Legends are numbered markers tied to a mini canvas (script chip, Center Box, floating param, ports, slider, Panel, Ladybug ribbon tile) and to the tree Panel (item, list, tree, branch, path, index with brackets).
- Mistake tag counts what is shown: "[1] fits 27 branches · 27 capsules" (27 lit capsules on the tower, "3 of 27 branches shown" under the Panel). Address tag: path {0;0;3;1}, index 1.
- Localized slider names in the state views; Series wording "one capsule on each of nine storeys"; German wording fixes (Arbeitsfläche, Auswahlmöglichkeiten); "normally flow from left to right"; preview clause ends with the eye struck through, empty viewport and "25 m³" in the Panel; flow pulses the three wires in their colours; Ø chip beside the window; sun tint only on the window; teaching-model note fades after the hook; slider and Panel positions moved inside the sheet.
- Value List ends on type C (Ø 1.6 m) so the window change is visible.

## Clause list (24)

hook, script, slider, box, parts, flow, sliders, live, preview, warning, error, panel, params, toggle, valuelist, plugin, item, series, count, tree, mistake, address, legend_tool, legend_data.

Notes: Ladybug stays qualitative (ribbon tab with `LB Sunpath` / `LB Direct Sun Hours`, shown as LS/LD tiles, tinted window, no sun-hour numbers). Storey height 2.75 m is the teaching-model value. The slider "rounding" of the plan row is shown as the decimals setting (Rhino 8 labels are `[CHECK]` in research). The Package Manager has no menu path on screen (`[CHECK]` in research). The error bubble keeps the real English Grasshopper text in the German frames; the German caption explains it. Capsule counts follow the model (140 = 70 branches × 2; Tower A 27 branches).

## Keyframe review

All 24 frames in en and de were captured after the last change and looked at; the final full pass was followed by one preview-only re-capture (Panel width). Layout checker: 35 (en) and 36 (de) strict findings in 13 of 24 frames, all justified:
- 34 of them: dimension labels drawn by `capsule_axon` on the viewport (the viewport grid is one stroked object under the label), and the one chip leader that crosses the window.
- de `params`: the red leader of the input-X tag crosses a wire (intended).
No text overlaps text, no clipped or tiny text, nothing in the caption band.

## Tests

- Engine: `🧪️tests/🐍️.py` 57 OK; `node 🧪️tests/🟨️.cjs` OK.
- Presentation (`🧪️tests/🎭️presentation/🐍️.py`, previous full run): 11 of 12 OK, `Beat1_Anatomy` lifecycle OK in en and de; the one error is `test_every_engine_view_builds` for `capsule/types` (toolkit/capsule scene, not anatomy). Re-run after this revision: see the end of this file.

### Re-run after the audit revision

Presentation contract tests (narration bilingual and unique, dispatch and step keys, imports, declared chapters, provenance, explicit language): 6 of 6 OK. Engine tests 57 OK. The slow native-lifecycle test was not repeated; the full capture of all 24 frames in en and de ran through the real animation code (`capture.py`) without error. The layout counts above come from the last full pass; the final preview-only re-capture overwrote `layout.md` with that single frame.

## Quality round (quality-grasshopper-anatomy.md)

Implemented: whole-tower hook ("1 of 140 capsules") that becomes the dimensioned capsule in `script`, with a strip "one definition: 4 types, 140 placements" (item 4 folded into `script` to stay at 24 clauses); `flow` shows arrowheads on the three wires plus pulses; `sliders` ends with length 5.0 / height 3.0 (box grows, then returns); `live` ends at width 3.0 with an "X = 3.0 m" readout (then returns); warning and error each continue with a repair beat (text replaced by 2.5, component grey, box back; extra snapshots `warning_fixed`, `error_fixed`, saved by `capture.py` through `Scene.snapshot`); plugin shows a Ladybug component with a Window parameter and a "less sun / more sun" colour bar, qualitative; larger tower (no podium) with labels "0 → 0.0 m" and "8 → 22.0 m" tied to the first and last list row, Series ports explained "S start 0 · N step · C count"; the path {0;0;3;1} is mapped to "storey 3", "facade 1 (E)" and finally "{0;0;3;1} [1]" at the capsule; mistake/address are a pair (27 capsules vs "expected: 1 capsule", then the wrong tag struck out and the single capsule lit, caption adds that the Panel shows both); wording: "Center Box, nickname Box", "height 0 m of the first storey", "Schieberegler" for the Anzahl slider; legend caption voices the first two rows and says "and six more" (all rows are lit in sequence on screen).

Not done: voicing all eight legend rows in the caption (caption limit of two lines).

Review: all 24 keyframes plus the two repair snapshots were captured in en and de and looked at; layout counts of the last full pass 38 (en) and 41 (de), justified as before (dimension labels over the viewport grid, the intended strike line through the wrong tag, leaders crossing wires). Quick G contract tests: 6 of 6 OK; engine tests OK.

## Sweep fixes (sweep-grasshopper-anatomy.md)

German balloons keep the real English text with the tag "Grasshopper-Meldung (englisch)" and a German explanation; German captions use "Schieberegler" and "Reiter"; the Panel text "2.5" stays with a decimal point (Grasshopper parses text that way) and the captions say "der Text 2.5" / "statt tall"; the series and count rows carry the unit m; viewport title "Perspektive" in de (user-chosen labels localized, built-in component names English). All 24 frames recaptured in en and de after the last full change; the German balloon-tag frames were recaptured once more after moving the tag clear of the "erstes Problem" marker.
