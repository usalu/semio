# FEM 2D Editor Feature Matrix v2 (headless Chromium, http://127.0.0.1:6086/)

Driver on port 6197 (own process), scripts and screenshots in `🗑️generated/m2v2/` (shots are `<name>.png`, per-feature JSON in `results.jsonl`, raw script outputs `*.out`). Viewport 1600x1000. Boot to converged ~40-120 s (first boot 120 s, reloads 37-47 s). Latency is wall clock from action to the visible/aria effect. Tree/inspection reads are fast (1-5 s) when the app is healthy.

Test method notes (important for reading the rows):
- Canvas picking uses the hover result: the click must be preceded by a ~1.5 s hover on the target, otherwise the previously hovered entity gets selected (see R-S1).
- Selection through the canvas costs 4-7 s, marquee/lasso 17-19 s end to end (drag of ~4 s included), tree-row selection about 1-3 s.
- Entity edits through tree-row selection plus Inspection are fast (2-5 s) and re-solve in the background.

## Summary of FAILs (details below)

1. Undo is broken and wedges the app: Ctrl+Z and History panel Undo button do nothing (model keeps the edit, Redo stays disabled) and afterwards canvas/tree selection no longer updates (W1).
2. Playback crashes the plugin: Play/Pause animates 2 frames, then `resultAnimationTick` traps (`semio-plugin panic ... shared original alias must retire or transfer`), app wedged (W2). Space key does not start playback.
3. Selection mode radios Additive/Subtractive/Invertive have no effect (marquee and click both behave as Selective); only Shift (additive toggle) and Ctrl (subtractive) modifiers work (S2).
4. Delete key (canvas selection) and Delete/Backspace with tree selection do nothing; Backspace via canvas not verified (D2).
5. Inspection Delete of a node (with or without dependents) does nothing (n2, ridge): node removal never happens; element delete works (D3).
6. Transform gumball drag moves the node only partially (22 of 60 px), throws `transient publication generation is stale` errors and wedges the app (T1). Gumball option toggles (Uniform Scale/Scale Axes/Rotate/Move) change aria state but the gumball drawing does not change (T2).
7. Results: Mode=Modal/Buckling leaves the Mode combobox at "Static" and the Results window shows "Pending" without a result (R3); Deformation Scale edit in the Results panel has no visual effect in 45 s (R4); Step+ changes the frame, Step- not measurable.
8. Window Focus on the Model window hides the Results window (acts as maximize); Command > Reset panels does not restore a closed window (X2). Artifact panel overlays the Model window tab bar (Focus/Close buttons unreachable while Artifact is open) (X3).
9. Inspection Focus button has no visible effect (camera unchanged) (I9). No "Add Term" button in Combination inspection (I10).
10. Example switch No example -> Demo restores the model but Results stay "Pending 0/0" (E1), console `plugin.internal.zero-progress: runtime close cleanup faulted`.
11. Direct Select is not shown as pressed when the app boots (implicit default), only after clicking it (U1). Utilities tools toggle off when clicked again (informational).
12. Analysis row in the Artifact tree is not selectable (Inspection stays on previous entity) (A6).

## 1. Utilities

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| U1 Direct Select default armed | boot, open Utilities | button pressed | Utilities lists Direct Select, Marquee Select, Lasso Select, Pan, Transform; Direct Select NOT pressed on boot, canvas selection nonetheless works; after clicking it: `button "Direct Select" [pressed]` plus Method (Rectangle/Lasso) and Mode (Selective/Additive/Subtractive/Invertive) radiogroups appear. Clicking an active tool toggles it off (no radios). | <1 s | FAIL (default state not indicated) |
| U2 Direct click select | hover ridge (520,402) 1.5 s, click | `treeitem "ID ridge"` | ID ridge + X 4 + Y 7.6 + Focus + Delete | 2-7 s (first click after boot ~6 s) | PASS |
| U3 Marquee | tool Marquee, drag (380,370)-(660,660) | rectangle selects entities inside | "25 SELECTED" with list (Loads, Nodes, Elements, Supports) and first entity form; screenshot m1a.png | 4 s after mouseup (17-19 s end to end with slow drag) | PASS |
| U4 Lasso | tool Lasso, polygon (410,370),(640,370),(640,520),(410,520) | selects inside | "15 SELECTED" (a1lasso.png) | 18.9 s end to end | PASS |
| U5 Pan | tool Pan, drag +150,+100 (slow drag, 0.5 s per step) | view moves | model moved by exactly the drag, overlays hide during drag (a2pan_mid.png); a fast 6-step drag first time did not move (a1pan1.png); only Model window pans, Results has its own camera | live | PASS (drag must be slow). Console: 52x `[DEBUG] receiving quote lane=WindowConfig ...` as console.error per drag |
| U6 Transform gumball shown | tool Transform with a support selected | gumball | rings and 5 handles drawn at the selection (t4gumball.png); options panel Uniform Scale, Scale Axes, Rotate, Move all checked | 8 s | PASS |
| U7 Transform drag handle | drag red handle +60 px | node moves 60 px | node moved only +22 px (680->702, t5afterdrag.png), results moved with it; console `actor original-turn.semantic interactive-job.stale: typed-operation pending publication rejected a stale ...`, `typed-operation completion effects failed ... native-return-refused`; afterwards selection and Ctrl+Z dead (W1) | 15 s | FAIL |
| U8 Transform option toggles | click each of four checkboxes, then back | aria checked toggles and gumball changes | aria checked toggles for all four (t6.js); gumball drawing identical with Uniform Scale/Scale Axes/Rotate unchecked (t6_Rotate.png) | 3 s | FAIL (no visual effect) |

## 2. Selection modes

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| S1 Click replaces selection | click ridge then p0_l2 | replace | works (hover before click). Without hover the previously hovered entity is selected (stale hit-test) | 4-7 s | PASS (R-S1 note) |
| S2a Shift+click | ridge, Shift+click p0_l2 | "2 SELECTED" | "2 SELECTED / Node: ridge / Node: p0_l2" (m7.js) | 6.4 s | PASS |
| S2b Ctrl+click | Ctrl+click selected node | removed from selection | removed (m6.js, 5.5 s); Ctrl on unselected node: no change | 5.5 s | PASS |
| S2c Radio Selective | radio, marquee | replace | replaces | | PASS |
| S2d Radio Additive | radio Additive; click or marquee another node | adds | replaces (only the new node selected), m4/m7/m10 outputs | 6 s click, 17 s marquee | FAIL |
| S2e Radio Subtractive | radio Subtractive; click/marquee on unselected node | no-op / removes | selects the node (behaves as Selective) | 6 s / 17 s | FAIL |
| S2f Radio Invertive | radio Invertive; click/marquee | toggles | replaces | 6 s / 17 s | FAIL |
| S3 Empty click deselect | click (300,300) | Summary | Summary returned (2.7 s); once the Inspection stayed stale for 60 s (s1) | 2.7-7.5 s | PASS (flaky once) |
| S4 "N Selected" | multi selection | header | `25 SELECTED`, `15 SELECTED`, `2 SELECTED` with entity list | 4 s | PASS |

## 3. Delete and history

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| D1 Inspection Delete element | select e11, Delete | removed | e11 removed (b6.out) | 2.4 s | PASS |
| D2 Delete key | select e10 in tree, Delete (focus blurred); also canvas-selected ridge, Delete | removed | not removed (80 s); canvas variant: ridge still selected after 40 s (b21.out) | n/a | FAIL |
| D2b Backspace | tree selected e10, Backspace | removed | not removed. Canvas-selected Backspace not verifiable (panel state toggling in my script) | n/a | FAIL (tree), NOT RUN (canvas) |
| D3 Inspection Delete node | select n2 (used by e3/e4) then ridge, Delete | node and dependents removed | node row stays, e3/e4 and e10/e11 stay (b6.out, b6b.out, b6_delnode.png) | n/a | FAIL |
| D4 Undo Ctrl+Z | edit element material, blur focus, Ctrl+Z | material back | no revert within 45 s; app wedged afterwards (selection no longer updates, health STUCK) (b10.out) | n/a | FAIL |
| D5 Redo Ctrl+Shift+Z / Ctrl+Y | after undo | redo | nothing to redo (Redo disabled, undo never happened) | n/a | FAIL (dependent on D4) |
| D6 History panel | open History | list, Undo/Redo | rows Actions: Undo, Redo (disabled), Commit Checkpoint, Check In, Create Alternative, Filter; Commands: `Change element "e5"`, `Set Active Example`. Undo button click: no revert, wedge (b11.out); Redo stays disabled | | FAIL |

## 4. Inspection edits (fresh page per feature; tree-row selection)

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| I1 Node X | select ridge, X=5 | node moves, results re-solve | ridge to x=5, Converged 40/512, Ty 1692 N (e1_undo.png) | 2-5 s edit, ~20 s re-solve | PASS |
| I2 Node Y | ridge, Y=9 | tree (4.00, 9.00) | updated | 4.9 s | PASS |
| I3 Element start/end/material/section/kind | e5, comboboxes | tree/inspection update | start=rc2, end=rc3, material C30/37, section Timber Floor Beam, kind Beam->Bar (options Bar, Beam) all updated | 2.1-3.3 s | PASS |
| I4 Material | E=2e11; name | tree E 200 GPa | updated | 2.7 s | PASS |
| I5 Section | area 0.002 | tree A 0.002 | updated | 2.7 s | PASS |
| I6 Support DOF toggle | s2 Tx toggle | tree s2 Tx Ty | updated | 1.6 s | PASS |
| I7 Region name, mesh size, material | r1 | updated | name Slab B (1.7 s), mesh 0.5 persists (11.7 s incl. reselect), material C24 (2.1 s) | | PASS |
| I8 Loads | area load 800->900 Pa; nodal Ty -12000 -> -9000 | tree update | updated (3.2 s each) | | PASS |
| I8b Load case | name; self weight toggle | tree update | name Live B (2.2 s), Self Weight tag removed (2.3 s) | | PASS |
| I8c Combination factor/name | dead 1.35->1.2, name SLS | tree | updated (3.9 s, 1.6 s) | | PASS |
| I9 Focus button | select e5, Focus | view recentres | no visible camera change (b5_focus_before/after.png) | 6 s | FAIL |
| I10 Add Term | combination Inspection | button exists | only Delete button in the combination form; term factors are inline inputs, no "Add Term" control | | FAIL (absent) |

Health after these single edits was responsive (1.9-2.9 s probe) also 45 s after node Y/section edits. Rapid consecutive edits (section rename then material rename seconds later) left Results "Pending" forever, Inspection stalled and the material rename took >60 s to appear (ed1/ed2.png): re-solve overlap wedge.

## 5. Actions palette (Model window Actions -> Viewport panel)

Every action opens an args form (ids `framework.window.fem2dModel.action.<id>.arg.<arg>.input`), Execute enabled after filling. Palette actions: Clear Selection, Select All, Set Selection Mode, Set Granularity, Add Node/Bar/Beam/Material/Section/Support/Nodal Load/Member UDL/Area Load/Region/Load Case/Combination, Set Self Weight, Set Analysis Settings, Remove Selection, Set Active Example, Set Result Display, Translate/Rotate/Scale Selection, Copy, Cut, Paste, Export/Import Document, Undo, Redo, Commit Checkpoint, Create Alternative, Switch Alternative, Checkout Checkpoint.

| Feature | Observed | Latency | Result |
|---|---|---|---|
| Add Node (2,2) | Nodes 12->13, new id `n-e669...` | 1.0 s | PASS |
| Add Bar / Add Beam | Elements +1 each | 2.5 s / 4.0 s | PASS |
| Add Material | Materials +1 | 36 s (slow) | PASS (slow) |
| Add Section | first run no change in 60 s right after Add Material (queued), retry on fresh page added | 3.4 s (retry) | PASS |
| Add Support, Nodal Load, Member UDL, Area Load | verified rows | 2.7 s, 3.9 s, 4.7 s, 1.2 s | PASS |
| Add Region | Regions +1 | 31 s (slow) | PASS (slow) |
| Add Load Case, Add Combination, Set Self Weight | counts/tags updated | 1.8 s, 3.9 s, 4.1 s | PASS |
| Set Analysis Settings (4,2,100) | Results deformation visibly shrinks (b22_analysis.png) ; Artifact summary row not visible | n/a | PASS (visual) |
| Remove Selection, Translate/Rotate/Scale, Copy/Cut/Paste, Set Result Display, Set Selection Mode, Set Granularity, Export/Import | forms present (ids inventoried in f2.js output), not executed | | NOT RUN |
| Command menu | Command button opens menu with categories Layout, App, Appearance, Language and commands Set Driver, Reset panels, Set Layout, Fullscreen | | PASS |
| Command > Fullscreen | `document.fullscreenElement` set | 2.5 s | PASS |
| Command > Reset panels | opens Settings Driver row "Reset panels"; does not restore a closed window (X2) | | FAIL |

## 6. Artifact tree

All sections populated: Nodes (12), Elements (9), Regions (1), Supports (4), Load Cases (2) with child Loads, Combinations (1) with Terms, Materials (3), Sections (4), Analysis. Materials, Sections, Analysis are collapsed by default and expand with the chevron (x=9). Row click selects within 1.3-2.5 s and shows the matching Inspection form (Region, Support, Load area/nodal, Load Case, Combination (Term rows select the combination), Material, Section, Element, Node). Visual defect: Section rows draw the icon text overlapping the label (`ile:CHS 76...`, g4.png). Analysis row does not change the Inspection (A6). Result: PASS with A6 FAIL.

## 7. Results

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| R1 Results panel | open Results (bottom bar) | controls | Display: Source (Dead Load/Live Load/ULS), Mode (Static/Modal/Buckling), Mode Index, Playback: Phase, Step -, Play / Pause, Step +, Speed, Loop (Loop/Ping-Pong/Once), Waveform (Ramp/Sine); Analysis: Modal Count, Buckling Count, Deformation Scale | 3 s | PASS |
| R2 Source change | Live Load | results change | changed | 1.7 s | PASS |
| R3 Mode Modal/Buckling | pick Modal | mode shape | combobox keeps "Static"; Results show "Pending" with deformation removed, no mode shape after 60 s (b15_modal.png, b22_modal60.png) | | FAIL |
| R4 Deformation scale 1000 | edit | magnified | no visual change in 45 s | | FAIL |
| R5 Step + | click | frame changes | changed | 2.3 s | PASS |
| R6 Play/Pause | click Play, sample 8 frames | animates | 2 distinct frames then plugin panic `semio-plugin panic panicked at semio-framework-value-allocation-return-...: shared original alias must retire or transfer` and `scheduled dispatch of "resultAnimationTick" failed: wasm trap: wasm unreachable instruction executed`; app wedged afterwards | | FAIL |
| R7 Space key | Space over Results window | toggles playback | no frames change | | FAIL |
| R8 Results window render | static boot | deformation (pink), reactions, loads, status caption not overlapping | OK: caption "Converged 36 / 512 Residual 7.460e-9" now sits above the model, no overlap with load/reaction labels (boot.png, r1.png); no contour/legend present for static Dead Load | | PASS (no legend shown) |
| R9 Re-solve after edit | X edit | updates | "Converged 40/512" and reaction labels updated in ~20 s | | PASS |
| R10 Panels visual | Results panel section headers (Display, Playback, Analysis) are white bars with black text, inconsistent with the rest | | note |

## 8. Misc

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| E1 Example combobox | No example then Demo | cleared, restored | Nodes 0, then Demo restored model; Results stuck "Pending 0 / 0" afterwards (b17_demo_back.png); console `plugin.internal.zero-progress: runtime close cleanup faulted for instance` | 4.9 s | FAIL (Results) |
| E2 Editor/Viewer | click Viewer, then Editor | Viewer pressed, editing off | Viewer pressed; Actions palette removed; Viewer button disabled while Inspection panel open (u3 snapshot) | 4 s | PASS |
| X1 Window Close | close window | removed | tab removed | 4 s | PASS |
| X2 Window Focus | Model Focus | maximize | Results window removed from view (only Model stays); Reset panels does not restore a closed window | 4 s | FAIL (restore) |
| X3 Layout | Artifact open | window chrome reachable | Artifact panel (x 0-300, y 0-230) overlays the Model tab bar, Focus/Close buttons cannot be clicked | | FAIL |
| X4 Top Fullscreen | Command > Fullscreen | document fullscreen | PASS | | PASS |
| L1 Settings | open | panel | Driver (Reset panels, Merge policy, Terminology, Language, Driver, Layout, Appearance), General, App sections | | PASS |
| L2 Language EN/DE | Settings > Language > Deutsch | labels German | all labels German (Dokument, Inspektion, Aktionen, Hilfsmittel, Anzeige, Ergebnisse, Befehl, Einstellungen, Verlauf, Aufgaben, Vollbild, Betrachter; results caption Konvergiert/Restkraft) no English left; truncation: "Panels zurückset", "ngen" in bottom bar (b20_de.png); switched back to English | 5 s | PASS |
| L3 Wheel zoom | wheel -600 over Model | zoom | view changed | 0.6 s | PASS |

## Diagnoses and evidence

- W1 Undo/wedge: `b10.out` (Ctrl+Z: material stays C30/37 for 45 s, health STUCK), `b11.out` (History Undo button same), `h1.png` (History lists `Change node "ridge"`, Redo disabled). Log on h1: `tree window refresh failed Error: original-turn.native-return-refused: semantic: interactive-job.stale`. After the wedge: tree row click highlights but Inspection never updates, Results "Pending" (ed2.png).
- W2 Playback panic: console text in driver log of b14.js (`semio-plugin panic panicked at semio-framework-value-allocation-return-#.#.#\..\..\shared\origin...`, `shared original alias must retire or transfer`, `scheduled dispatch of "resultAnimationTick" failed Error: wasm trap: wasm unreachable`).
- T1 gumball: `t5afterdrag.png` and log lines `transient publication generation is stale`, `typed-operation progress refresh failed ... native-return-refused`.
- Console noise on every Pan/drag/window interaction: dozens of `error [DEBUG] receiving quote lane=WindowConfig retained=RetainedCloneGrant ...` (and lane=Artifact) as console.error.
- Node delete: `b6_delnode.png` shows Delete row highlighted but node n2 still in tree (12 nodes).
- Mode radios: m2add.png shows Additive checked with only the new node selected.
