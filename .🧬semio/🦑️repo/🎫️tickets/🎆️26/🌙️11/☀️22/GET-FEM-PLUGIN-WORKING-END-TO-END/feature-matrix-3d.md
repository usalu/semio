# FEM 3D Editor Feature Matrix (browser run, PARTIAL)

Run: headless Chromium via `page-driver.mjs` on port 6198 against `http://127.0.0.1:6087/`. Scripts and
screenshots (prefix `m3-`) are under `🗑️generated/m3/` and `🗑️generated/`.
Coverage is partial: from about 03:03 on, `/@vite/client` on 6087 returned the SPA `index.html`
(content-type text/html, browser reports 404), the app never booted again and every reload/goto ended with an
empty body. Items marked NOT RUN were therefore not tested.
Latency = time from the action to the observed effect (polled), measured while the app was slow.

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| Boot / first solve | open page, wait | "Case: dead" in Results | reached | 46-117 s | PASS |
| Utilities menu content | click "Utilities" in Model window | selection utility offered | only "Transform" offered, no Direct Select (N1) | <1 s | PASS (N1) |
| Transform utility arm + options rail | Utilities > Transform | 5 toggles (Move, Move Planes, Rotate, Scale Axes, Uniform Scale) | all five shown, checked | <1 s | PASS |
| Viewport pick without utility | click column, no utility | nothing selected | nothing selected | 15 s | PASS |
| Viewport pick with Transform armed | click column (391,600) | selected, inspector Element, gumball, tree row and Results highlight | col_20_l selected, gumball drawn, tree row highlighted, Results bar red | 12-24 s | PASS |
| Empty-click deselect | click empty ground | cleared, Summary shown | cleared | 19-26 s | PASS |
| Orbit, left drag (Transform armed) | left-drag on ground | camera orbits | no change in 30 s | no effect | FAIL (F1) |
| Orbit, middle drag | middle-drag | camera orbits | view rotated | 0.2-1 s | PASS |
| Pan, shift+middle drag | drag | pans | view moved | 0.1-0.5 s | PASS |
| Zoom, wheel | wheel -400 | zoom | zoomed | 0.2-1.3 s | PASS |
| Right drag | right-drag on ground | pan or orbit | no change in 30-45 s | no effect | FAIL (F1) |
| Artifact tree selection (node, element, solid, support, load case) | click rows | Inspection kind changes | all selected | 10-20 s | PASS |
| Artifact tree, Load Cases children | view tree | child loads listed directly | large empty gap under cases, children appear later | n/a | FAIL (F4, visual) |
| Artifact tree, Materials and Sections groups | view tree bottom | rows visible | groups collapsed by default, not expanded or tested | n/a | NOT RUN |
| Inspection Node edit X | sc3 X=11 | tree row 11.00, gumball moves | applied | about 20 s | PASS |
| Inspection Element Roll | fb1_0 Roll=0.5 | Roll 0.5 | applied | 23 s | PASS |
| Inspection Element Kind Bar then Frame | custom combobox | Roll row hides for Bar, shows for Frame | works | 18 s / 32 s | PASS |
| Inspection Element Start, End, Material, Section | pick option | value changes | all applied | 12-23 s | PASS |
| Inspection Solid Name, Height, Layers, Base, Mesh Size, Material | edit each | tree and inspector update | all applied | 7-33 s | PASS |
| Inspection Solid Axis X then Z | custom combobox | Axis changes | applied | 15-18 s | PASS |
| Inspection Support Node select | sc3 to sc2 | tree shows sc2 | applied | 18 s | PASS |
| Inspection Support DOF toggles | click Tz, Rx | toggle | rendered as buttons, not clicked (test script assumed checkboxes) | n/a | NOT RUN |
| Inspection Load Case Name | rename | tree and inspector update | applied | quick | PASS (weak) |
| Load Case Self Weight toggle | click | label gains/loses "Self Weight" | state ambiguous (script overlapped a previous run) | n/a | NOT RUN |
| Loads (nodal, member, area), Combination, Material, Section edits | select and edit | patch commands | not reached | n/a | NOT RUN |
| Focus | select node, Focus | orbit re-aims | not reached | n/a | NOT RUN |
| Delete (Inspection), Delete key, Backspace | delete entity | entity and dependents removed | not reached (f1.js written, server broke) | n/a | NOT RUN |
| Undo/redo (keys, History panel) | after edit | restored | not reached | n/a | NOT RUN |
| Gumball drag (move/rotate/scale) and option toggles | drag handles | geometry changes | not reached | n/a | NOT RUN |
| Add * actions (Actions palette, Command palette) | run each | entity added | not reached | n/a | NOT RUN |
| Results panel (source, mode, index, playback, analysis settings) | edit | caption/render updates | not reached | n/a | NOT RUN |
| 3D results rendering | after first solve | solid coloured, bars shown | solid coloured (cyan/yellow) in Results window; frame deformation not visible | n/a | PASS (weak) |
| Results re-solve after edits | edit model | back to "Case: dead" | caption stayed on "Preparing analysis", "Generating mesh", "Preparing result view" for 60 s+ after edits, settled once | 60 s+ | observed |
| Example combobox, Editor/Viewer role, window Focus/Close/Fullscreen | use controls | works | not reached | n/a | NOT RUN |
| Settings, language DE/EN, customization | open Settings | switch | not reached | n/a | NOT RUN |

## Diagnoses of FAILs and defects

F1 Right-drag and left-drag (Transform armed) do not move the camera.
Evidence: screenshots `cam-right-drag.png`, `cam-left-drag-orbit.png` unchanged (page hash identical for 30-45 s) while
middle-drag, shift+middle and wheel change the view in under 1.5 s. Left-drag is plausibly owned by the armed Transform
utility, right-drag has no effect at all. Left-drag orbit with no utility armed was not tested.

F2 Plugin panic during viewport interaction (intermittent, seen in 2 of 3 sessions).
Console: `[semio-plugin panic] panicked at semio-framework-value-ordered-0.1.0\..\..\🦀️.rs:105:57: ordered-map root must be explicitly retired before drop`,
then `wasm trap: wasm unreachable instruction executed` in `semio_hub_fem_component.core.wasm`
(`tree window refresh failed`, `typed-operation completion effects failed`). Afterwards the Artifact tree no longer
refreshes and picks no longer select until reload. First seen in one run that did pick, deselect, left-drag, right-drag,
middle-drag, wheel in sequence; repeating the gestures alone did not reproduce it; a second occurrence followed a viewport
pick click right after a reload (`c5.js`).

F3 Plugin runtime error after many edits on one page: `[actor] original-turn.semantic OwnershipLimit: original return escrow table has no free slot for a new receiving scope`
plus `typed-operation progress refresh failed` / `completion effects failed`, after roughly 15 consecutive inspection edits
(during the load-case edit run). The following Self Weight toggle showed no effect within 90 s.

F4 Artifact tree, Load Cases section: loads under a case leave a large empty gap (`m3-t2.png`); children appear only after a
later re-render (`m3-e7-lc.png`).

F5 Dev server 6087 broke again at about 03:03: `/@vite/client` is answered with the SPA `index.html` (text/html, 3975 bytes), the module
request fails (404 in the browser), body stays empty on `reload` and `goto` (`m3/g3.js`, `m3/g4.js`). Blocked all remaining tests.

## Notes

N1 The 3D Utilities menu offers only Transform; there is no pure selection utility, selection shares the Transform gumball.
2D offers "Direct Select".
N2 The Model window does not highlight the picked bar while the gumball is shown (Results window does, red bar).
N3 With Artifact and Inspection panels open, panels overlap the viewports and the Artifact panel covers the "Results" dock tab.
N4 Shared `🗑️generated/` is racy: another agent overwrote `c6.js`; own files are now under `🗑️generated/m3/`.
