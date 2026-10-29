# Audit: grasshopper_anatomy (`Beat1_Anatomy`), read-only

Scope: all 24 keyframes in `en/` and `de/` (`🗑️generated/frames/🦗️grasshopper/Beat1_Anatomy/`), the scene `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`, `storyboard-grasshopper-anatomy.md`, `scene-grasshopper-anatomy.md`, `plan.md` row, `agent-rules.md`, `🏢️nakagin/🔣️.json`, `research-nakagin.md`, `research-grasshopper.md`. Two crops (Width floating param, Series wiring) checked at 3x in the scratchpad.

## Verdict

NOT READY. Two blockers and nine majors, most of them visible on screen in both languages. The narration arc is sound (24 clauses, slider to box, states, panel, params, toggle, list, plugin, item, list, tree, tree-as-list mistake, legend), and the Nakagin values (2.5 x 4.0 x 2.5 m, window 1.3 m, Tower A 9 / Tower B 11 capsule storeys, Series step 2.75 m) match the model. The layout report's claim of "no overlaps between text" is wrong: the strict checker flags a real text-over-text collision in the value list frame that the scene note does not mention.

## Defect table

Severity: blocker = illegible or contradicts the caption on screen; major = wrong or misleading teaching content or a quality-bar miss; minor = polish.

| frame/key | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| valuelist | de | blocker | Viewport header "Perspective" is overprinted by "Typ D · Gefächerte Wohnkapsel · Ø 1,3 m". Unreadable. | Move the type note out of the header band (bottom of viewport, or below the header with its own row); keep header clear. Shorten "Gefächerte Wohnkapsel" if needed. |
| valuelist | en | major | Same collision: "Perspective" overlaps "Type D · Splayed residence · Ø 1.3 m" (strict checker: 83x18 px). | Same fix as de. |
| params | en, de | major | Floating Number "Width" (hexagon) feeds a Panel that reads 2.3 (de "2,3"), with no wire from the Width slider (2.5) and X at 2.5. 2.3 is the sourced interior width, not the outer width used everywhere else. The storyboard asked for 2.50 on both. | Wire the floating param from the Width slider so the Panel reads 2.5, or use the model outer width. Check the engine view "params" default. |
| parts | en, de | major | On the component body the text reads "Center Box", while the leader says "Nickname: Box". In Grasshopper 1 the body shows the nickname ("Box"); the full name "Center Box" is only the name/tooltip. The frame teaches the wrong thing. Storyboard said "Center Box is shown as Box". | Render the body as "Box"; keep "Name: Center Box" leader to the tooltip/title area. |
| box | en, de | major (verify) | Center Box solves with Y and Z unwired; the viewport shows Y 1.00 m, Z 1.00 m from nowhere. In Grasshopper 1 an unconnected required input usually gives a red "Input parameter Y failed to collect data" (the scene already uses that message in `error`). Storyboard asked for default sliders for Y and Z; not drawn. | Add default sliders for Y and Z (1.00 m / or the first storey values) before the solve, or show the red state. Verify the required/optional behaviour of Center Box in Rhino 8 first. |
| warning | en, de | major | Caption says "the rest still works", but the Box output reads `null` and the viewport keeps the old box. The frame contradicts the clause and teaches "orange = still usable"; storyboard misconception "grey downstream is not proof" is not addressed. | Make the caption match the frame ("the output is empty: null"), or keep a valid partial result (e.g. a list where only one item fails). Show the viewport box as empty when the output is null. |
| warning | de | minor | Sliders read "Width" and "Length" (engine view labels), while every other German frame says "Breite" and "Länge". | Pass the localized LABELS to the warning/error engine views. |
| mistake | en, de | major | Tag "[1] fits 27 branches" / "[1] passt zu 27 Ästen", but the panel shows 3 of the 27 branches and the drawing lights 27 capsules without count. The viewer cannot check the number on screen. | Say "in all 27 branches of Tower A" and show "3 of 27 shown" next to the panel, or count the lit capsules on the tower. |
| legend_tool | en | major | Caption says "in English with the German word", then says only the English terms (script, component, ...). No German word is spoken; the German column is only on screen. | Speak the pairs: "script, Skript; component, Komponente; ...", or drop "with the German word". |
| legend_tool | de | major | Caption "Die Begriffe des Canvas, englisch mit dem deutschen Wort: Script, Component, ..." lists English terms only. A German listener hears no German term. | "Die Begriffe des Canvas: Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel, Plugin." English stays on screen. |
| legend_data | de | major | Caption "Die Begriffe der Daten: Item, List, Tree, Branch, Path und Index." speaks English only; the German column (Element, Liste, Datenbaum, Ast, Pfad, Index) is never said. | "Die Begriffe der Daten: Element, Liste, Datenbaum, Ast, Pfad und Index." |
| legend_data | en | minor | Same pattern: German equivalents are on screen only. | Optional: "item, Element; list, Liste; ..." |
| hook, sliders, box, flow, live, toggle | en, de | major | Colour linkage not implemented: every dimension line is one dark orange (#A85500), not width yellow, length cyan, height green. Wires are grey, not the colour of the dimension they drive. Quality bar: "wires and reference frames share the colour of the drawn element". | Give each dimension its storyboard colour and tint the wire and slider outline with it. |
| legend_tool, legend_data, tree, address | en, de | minor | Storyboard asked each legend row to ping its canvas object (slider, Center Box, port, panel, ribbon tab, branch, path, capsules). No leaders or pings; the tables float over empty canvas. | Add leaders/pings from each row to the object. |
| flow | en, de | minor | Only the X wire pulses. Storyboard asked for three wires pulsing in their colours. | Pulse all three wires in sequence. |
| preview | en, de | minor | The still shows the preview eye ON; the "hidden" state the caption describes is not in any keyframe. Volume panel "25" has no unit next to it (m³ sits under the panel). | Add an off-state keyframe; put the unit beside the value. |
| hook | en, de | minor | Dimension labels "4.00 m" and "2.50 m" sit on the capsule outline. | Offset labels from the outline. |
| toggle | en, de | minor | "Ø 1.30 m" chip sits over the window circle; the leader crosses the window. | Move the chip beside the window. |
| plugin | en, de | minor | Whole front face is tinted olive; caption speaks about the window face. | Tint only the window area. |
| item, series | en, de | minor | Storey-height slider chip starts at the canvas sheet's left edge (x about 30 to 37 px at 1280x720), touching or leaving the sheet. | Shift the slider right by about 0.3 units. |
| error | de | minor | "erstes Problem" label collides with the green frame edge around "tall". Green marks the problem, which reads as "ok" (red/orange is the state colour). | Use red/orange for the first problem marker; move label clear of the frame. |
| error | en, de | minor | Error bubble is the real Grasshopper text, in English in the German frame. Acceptable by design, but the German narration never says so. | Keep; mention in the README, or add a German gloss in the caption. |
| hook ... legend_data | en, de | minor | Teaching model note "Teaching model · simplified, rounded" stays on every keyframe. Storyboard asked for clause 1 only. | Decide: fade it after the hook, or keep it as a deliberate persistent note and update the storyboard. |
| count | de | minor | Slider label "Count" stays English in the German frame. | Localize (e.g. "Anzahl") if the slider is user-named. |
| valuelist | en, de | minor | Caption "the window changes with the type": A, B and D are all Ø 1.3 m; only C (Ø 1.6 m, not on screen) differs. The visible change is small. | Caption: "the capsule takes its type's window and entrance", or show Ø per type in the type note. |
| address | en, de | minor | Tag "{0;0;3;1}[1]" is teaching notation; the Grasshopper Panel shows path and index separately. | Label "path {0;0;3;1}, index [1]" or add a short note. |
| script | de | minor | "Zeichenfläche ist der Canvas": research recommends "Arbeitsfläche (Canvas)" for the same term. | "Ihre Arbeitsfläche ist der Canvas." |
| valuelist | de | minor | "Eine Werteliste bietet feste Auswahlen" is unidiomatic. | "bietet feste Auswahlmöglichkeiten". |
| flow | en | minor | "always flowing from left to right" is overstated: wires can run right to left. | "normally flow from left to right". |
| all | en, de | minor | Strict layout findings (32 en, 33 de) were dismissed as false positives in the scene note. The valuelist overlap is real and not in that dismissal. | Re-run the checker after fixes and list each finding with its verdict. |

## Fact findings (narration and on-screen Grasshopper/Excel/Nakagin facts)

Verified in the frames against the model or research:

- Nakagin: capsule 2.5 x 4.0 x 2.5 m outer (hook, sliders, live), window Ø 1.3 m (toggle, type A/B/D), Tower A 9 capsule storeys and Tower B 11 (count clause) match `🏢️nakagin/🔣️.json` and research §3 "Tower A 9 capsule floors, Tower B 11". Facades 0 N, 1 E, 2 S for Tower A match the model (no west facade on Tower A).
- Volume 25 m³ = 2.5 x 4.0 x 2.5 (preview) correct.
- Series S, N, C inputs; Storey height wired to N (Step), Count to C: confirmed on the crop. Series output S correct.
- Center Box inputs B, X, Y, Z and output B; name "Center Box", nickname "Box" [K]. The on-canvas text is wrong (see parts).
- Warning text "Data conversion failed from Text to Number" and error text "Input parameter Z failed to collect data" are the real Grasshopper 1 messages [K].

Need checking (not verified here):

- [CHECK] box: whether Center Box Y and Z without a wire fail (red) or solve. See box row.
- [CHECK] warning: the research §2.1 quotes McNeel "put up an error and leave that slot blank", while the scene shows an orange warning. Confirm the level of the conversion message in Rhino 8 before the clip says "orange" for a failed cast.
- [CHECK] Number Slider rounding: the plan row asks for "rounding"; the clip only shows decimals. Research §1.1 has the Rhino 8 rounding labels as [CHECK]. Either add rounding or drop it from the plan row.
- [CHECK] Package Manager menu location in Rhino 8 (research §1, [CHECK]). The clip shows "Package Manager" as a step without a menu path.
- [K] Ladybug components LB Sunpath and LB Direct Sun Hours: names are real, shown abbreviated as LS/LD. Fine as abbreviations, but say so in the notes.
- Storey height 2.75 m: this is the repo IFC pitch, a teaching value. Research §5 says the real storey height is not established (heights imply about 4.3 m). The clip labels it "Storey height" with no qualifier except the persistent note. Acceptable, but say "teaching value" in the scene notes.
- "nine capsules on the tower" (series, de "neun Kapseln am Turm"): the drawing lights one capsule per storey on facade E, not nine of the two-per-facade items. "nine capsule storeys" is more exact.

Model cross-check (outside this clip, affects the tree clips and this clip's "two items per branch"):

- `🏢️nakagin/🔣️.json` gives 2 capsules per facade slot. Tower A: 9 storeys x 3 facades x 2 = 54. Tower B: 10 storeys x 4 facades x 2 + 1 storey x 1 facade x 2 = 82 (exception at storey 10). Total 136, but the model note says "140 in total".
- Research §3 gives 7 capsules per floor: Tower A 63, Tower B 77, total 140.
- The clip's "27 branches x 2 items" implies 54 capsules on Tower A, which disagrees with the research (63). Escalate to the model owner before the tree clips are rendered.

## Didactic findings

- Arc: hook on the real capsule, smallest example (slider into box), states, panel, params, toggle, list, plugin, item, list, tree, mistake, address, legend. Typical mistakes are shown and fixed (red text-to-number, tree read as a list). 24 clauses, within 14 to 24.
- Misconception 1 from the storyboard ("a wire does not create geometry; the text tall is not a height") is not taught anywhere. Add it to the error clause or the legend.
- Misconception "grey downstream is not proof of correctness" and "orange is a warning, the rest still works" are not handled. The warning frame currently shows the opposite (null output). See the warning row.
- Misconception "{0} and {0;0} are different structures": only {0} is shown. The tree clause could show the contrast.
- "Enabled would switch the component off" (storyboard clause 9) is missing from the preview narration.
- Plan row: "Number Slider (domain, rounding, value)" is shown as domain and decimals only.
- Legend clauses (22, 23) are the least visual: floating tables on an empty sheet with no pings. The storyboard asked for pings to the canvas objects.
- Duration: step times sum to about 50 s of animation (1.2 to 3.2 s per clause). The plan row asks for 3 to 6 minutes. The narration will set the real timing, so verify the rendered length; if it is under 3 minutes, add holds on the Nakagin drawing.
- German narration is complete for all 24 clauses, but the two legend clauses do not speak their German terms (see table).
- The layout report's claim "no overlaps between text" is false (valuelist, both languages). Re-run after fixes.

## Frames reviewed

en: hook, script, slider, box, parts, flow, sliders, live, preview, warning, error, panel, params, toggle, valuelist, plugin, item, series, count, tree, mistake, address, legend_tool, legend_data (all 24).
de: the same 24 keys (all 24).
