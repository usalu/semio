# FEM 3D Editor Feature Matrix v2 (browser run, new build)

Run: headless Chromium via `page-driver.mjs` (port 6198) against `http://127.0.0.1:6087/`, viewport 1600x1000.
Scripts, outputs and screenshots (`m3v2/*.png`): `🗑️generated/m3v2/`. Previous report: `feature-matrix-3d.md`.
Boot to "Case: dead": 30-42 s. Latency = action until the observed effect, polled. "Re-solve" = time until the Results caption is
back to plain `Case: dead` (about 25 s per model edit on the Demo model).
The previously reported ordered-map panic and escrow exhaustion did NOT reproduce in this run (about 60 edits, 8 reloads).

## 1 Viewport

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| Pick without utility | click column (391,540) | selected or nothing | element `col_20_u` selected, no gumball (old build: nothing selected) | 0.3-2.4 s | PASS |
| Pick with Transform armed | Utilities > Transform, click column | selected + gumball | selected, gumball drawn, Results bar highlighted red | 2-2.5 s | PASS |
| Pick pixel precision | click 1-2 px beside a thin column | still picks | miss at x=391/390/392 after orbit, hit at 321 but not 320 (columns about 3 px wide, no pick tolerance) | n/a | note |
| Empty-click deselect | click ground | Summary shown | cleared | 1.4-2.6 s | PASS |
| Orbit, middle drag | middle drag | orbit | orbits | 0.2 s | PASS |
| Orbit, left drag (no utility) | left drag on ground, fast and slow (30 steps) | orbit | no camera change within 8 s (screenshots c5 and c6 identical) | none | FAIL (F1) |
| Orbit, left drag (Transform armed) | same | orbit or box select | no change | none | FAIL (F1) |
| Pan, shift+middle | drag | pan | pans | 0.1 s | PASS |
| Pan, right drag | right drag, fast and slow | pan | no change (armed and not armed) | none | FAIL (F1) |
| Wheel zoom | wheel +-300..400 | zoom | zooms (animates about 2 s) | 0.25-0.3 s | PASS |
| Gumball move handle | drag x arrow +50 px | geometry moves | foot node of `col_20_l` moved, column followed, Results updated | 0.2 s apply, re-solve 12-25 s | PASS (F13: guide line stays) |
| Gumball rotate handle, small drag (about 18 px) | drag Z ring on `sol1` | slab rotates | rotated | 0.2 s | PASS |
| Gumball rotate handle, larger drag (about 60 px) | drag Z ring on `sol1` | slab rotates | slab disappeared from Model view, Results `Case: dead · Analysis failed`; Ctrl+Z restored it | 0.2 s | FAIL (F2) |
| Gumball scale handle | drag x scale cube | slab scales | drag session started, slab slightly bigger; analysis then stuck (F3) | 0.2 s | PASS (weak) |
| Transform options Move, Move Planes, Rotate, Scale Axes, Uniform Scale | untick each checkbox | handle group hides | Move off hides arrows, Rotate off hides rings, each toggles back, `checked` correct | 0.1-0.2 s | PASS |

## 2 Delete, Undo/Redo, History

| Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|
| Delete key | click column to focus viewport, Delete | element removed | Elements 16 -> 15, Summary shown | 1.5-4.2 s | PASS (needs viewport focus: after clicking a Transform checkbox the key did nothing) |
| Backspace | same | removed | removed | 1.5 s | PASS |
| Inspection Delete | select element, Delete button | removed | removed | 1.9 s | PASS |
| Undo Ctrl+Z | after delete | restored | element back (selection restored after key delete, not after Inspection Delete) | 1.2-1.6 s | PASS |
| Redo Ctrl+Shift+Z | after undo | re-applied | 15 elements | 0.8 s | PASS |
| Redo Ctrl+Y | after undo | re-applied | 15 elements | 0.9 s | PASS |
| History panel Undo/Redo buttons | Artifact + History open | counts 16/15/16 | 3.9 s / 2.4 s / 2.4 s | | PASS |
| History Commit Checkpoint, Check In | click | visible effect | no visible feedback, no dialog | n/a | PASS (weak) |
| History Create Alternative | click | alternative created | console `module.vcs: no checkpoint for alternative`, `tree window refresh failed` | 1 s | FAIL (F7) |
| History command log | after edits | rows appear | Delete/Undo/Redo/Translate Selection/Set Active Example rows listed | | PASS |

## 3 Inspection edits (each applied within about 2-4 s, re-solve about 25 s)

| Feature | Steps | Observed | Result |
|---|---|---|---|
| Node X/Y/Z | `sc3` X=11, Y=3, Z=0.5 | Artifact row `(11.00, 3.00, 0.50)` | PASS |
| Node Focus | click Focus | camera re-aimed (1.7 s) | PASS |
| Element Roll | `col_20_l` 0.5 | 0.5 | PASS |
| Element Kind Frame -> Bar -> Frame | combobox | Roll row hides/shows, Roll resets to 0 | PASS |
| Element Start, End, Material | combobox | all applied (End n20_l2, Start n00_g, C30/37) | PASS |
| Element Section | only one section exists | not exercisable | NOT RUN |
| Solid Name, Height 0.4, Layers 2, Base 0.1, Mesh Size 0.8 | spin/text | all applied (verified after queue drained; readback after 1.5 s still showed old values) | PASS |
| Solid Axis X, Material | combobox | Artifact row `Axis X`, material changed | PASS |
| Support Node `ss_0` sc0 -> sc1 | combobox | applied | PASS |
| Support DOF toggle buttons Rx on, Tz off, Tz on | buttons (pressed state) | row `Tx Ty Tz Rx` / `Tx Ty Rx` / back; 1.4-1.9 s | PASS |
| Load case Name | `Dead Load X` | applied 1.7 s | PASS |
| Load case Self Weight toggle | button | label loses/gains `Self Weight` (1.4 s / 1.6 s) | PASS |
| Load, nodal (Value -7000) | spin | row `Tz -7000 N @ n20_l1` (3.1 s) | PASS |
| Load, area (Pressure 1800) | spin | row `1800 Pa @ sol1` (3.0 s) | PASS |
| Load, member | Demo has none; added `fb1_0` by action only | not edited | NOT RUN |
| Combination Name | `ULS2` | applied | PASS |
| Combination terms edit | | not exercised | NOT RUN |
| Material Name, Young's Modulus | `Steel S355`, 2e11 | row `Steel S355 · E 200 GPa` (2.0 s / 3.5 s) | PASS |
| Section Name, Area | `HEA 220`, 0.006 | row updated (2.5 s / 3.8 s) | PASS |
| Inspection settle lag | select row, read snapshot | Inspection pane still showed the previous entity after 1.8 s (stale reads) | note |

## 4 Model Actions pane

Pane lists: Clear Selection, Select All, Set Selection Mode, Set Granularity, Add Node/Bar/Frame/Material/Section/Support/Nodal Load/Member UDL/Area Load/Solid/Load Case/Combination, Set Self Weight, Set Analysis Settings, Remove Selection, Set Active Example, Set Result Display, Translate/Rotate/Scale Selection, Copy, Cut, Paste, Export/Import Document, Undo, Redo, Commit Checkpoint, Create Alternative, Switch Alternative, Checkout Checkpoint.
Arg ids are `framework.window.fem3dModel.action.<name>.arg.<arg>.input`, Execute is `...action.<name>.execute`. Clicking an action again closes its form; one form is open at a time. Entries are greyed while analysis is pending.
Model viewport while the pane is open: the pane (300-600 px) covers the left part and the rest of the Model window (600-800 px) shows empty beige with no grid (`m1.png`, `m6a.png`, `m12.png`): FAIL (F4).

| Action | Args filled | Entity appeared in tree | Latency (to tree) | Re-solve | Result |
|---|---|---|---|---|---|
| addMaterial (2x) | name, e, g | Materials 2 -> 3 -> 4 | 2.1 s | n/a | PASS |
| addNode | x3 y4 z0 | Nodes 16 -> 17 | 1.9 s | 25.8 s | PASS |
| addBar | n00_g -> n20_g, steel, hea200 | Elements 16 -> 17 | 1.8 s | 25.3 s | PASS |
| addFrame | n02_g -> n22_g, roll 0.2 | Elements 17 -> 18 | 2.0 s | 25.8 s | PASS |
| addSection | name, area, iy, iz, j | Sections 1 -> 2 | 1.7 s | 25.6 s | PASS |
| addSupport | n00_l2, `tx,ty,tz` | Supports 8 -> 9 | 1.7 s | 26.2 s | PASS |
| addNodalLoad | n20_l2, dof Tx (combobox), 1234, live | `Tx 1234 N @ n20_l2 Load` under Live Load | 1 s | 26 s | PASS |
| addMemberUdl | fb1_0, wz -2500, live | `wz -2500 N/m @ fb1_0` | 1.1 s | 28 s | PASS |
| addAreaLoad | sol1, 700, dead | `700 Pa @ sol1` | 1.1 s | 26 s | PASS |
| addLoadCase | name Wind | `Wind · 0 Loads` (form Self Weight default on, case shows no self weight) | 0.9 s | 27 s | PASS |
| addCombination | name SLS, terms `dead:1,live:1` and 9 other formats (`dead=1.35,live=1.5`, JSON, `1.35 dead`, ...) | nothing, no error or message | none in 8-12 s each | n/a | FAIL (F5) |
| addSolid | x20 y0 3x3x0.3, materialId `c30` (guess) | nothing, no error | none | n/a | FAIL (F5) |
| addSolid | same with materialId `concrete` | Solids 1 -> 2 | 3.7 s | stuck > 6 min (F3) | PASS |
| setSelfWeight | live, enabled | `Live Load · 4 Loads · Self Weight` | 1.8 s | 30 s | PASS |
| setAnalysisSettings | 4, 2, 150 | executed | n/a | 33 s | PASS |
| Copy, Paste… | selected `col_20_u`, Copy, Paste with empty anchor | Elements stayed 16; console `UnsupportedOwner: natural-file import requires its owned declared media and no wire` + `tree window refresh failed` | 1 s | | FAIL (F11) |
| Select All, Clear Selection, Set Selection Mode, Set Granularity, Translate/Rotate/Scale Selection, Set Result Display, Set Active Example, Cut, Export/Import | forms open (args present, e.g. translate dx,dy,dz) | not executed | | | NOT RUN |
| Command palette (Command hug) | open | tabs Layout, App, Appearance, Language; Layout has Set Driver, Reset panels, Set Layout, Fullscreen; Language has Set Terminology, Set Locale | | | PASS (menu) |
| Command palette Set Locale form | open | arg `command.os.os.setLocale.arg.locale.input` exists but has 0x0 size (panel clips it), Playwright "not visible" | | | FAIL (F6) |

## 5 Artifact tree

| Feature | Observed | Result |
|---|---|---|
| Groups Nodes, Elements, Solids, Supports, Load Cases, Combinations, Materials, Sections, Analysis | all present with counts; Materials/Sections collapsed by default, expand on click | PASS |
| Load Cases children | loads listed under each case; no permanent gap. Rows are virtualized: after toggling groups the children are absent for several seconds (`a4.png`) and then appear | PASS (F14 transient) |
| Row click selects | node, element, solid, support, load case, combination, material, section, load (via the case's Loads list): Inspection switches in 1.7-2.1 s | PASS |
| Analysis row click | Inspection unchanged | PASS (weak, probably no inspector) |
| Visual | Section row shows literal text `ruler` over its icon (`a9.png`); Poisson ratio value clipped as `0.3` | FAIL (F12, cosmetic) |

## 6 Results

| Feature | Steps | Observed | Result |
|---|---|---|---|
| Results panel contents | Results hug button | Source, Mode, Mode Index, Phase, Playback (Step -, Play/Pause, Step +), Speed, Loop, Waveform, Analysis (Modal Count, Buckling Count, Deformation Scale) | PASS |
| Source options | open combobox | Dead Load, Live Load, ULS (renamed entries tracked) | PASS |
| Mode options | | Static, Modal, Buckling | PASS |
| Mode Modal | select Modal | caption `Mode 1: 0.000 Hz` in 1.3 s (0 Hz on the edited model) | PASS (weak) |
| Source Live Load | select | caption stayed `Case: dead`, no visual change in 30 s | FAIL (F10) |
| Mode Index 1 | fill 1, Enter | caption stayed `Mode 1` | FAIL (F10) |
| Display panel state | after selecting Modal | comboboxes still showed `Dead Load X` / `Static` | FAIL (F10) |
| Playback Play/Pause | click | `aria-pressed` null, no visible effect; next boot printed a panic (F9) | FAIL (F9) |
| Analysis Modal Count 4 | fill | value kept 4, re-solve 33 s | PASS |
| 3D results rendering | boot | slab coloured (cyan/yellow), frame shown; deformation visible after edits | PASS |
| Re-solve after edit | edit model | `Case: dead · Preparing analysis`, then plain `Case: dead` after 25-33 s per edit on Demo | PASS (slow) |
| Re-solve stability | many edits, gumball rotate/scale, undo; addSolid | caption stuck on `Preparing analysis` 400 s+ and 15 min+ in two sessions, `Preparing result view` 6 min+ after addSolid; Results geometry stale; reload resets to Demo | FAIL (F3) |

## 7 Example, role, window, settings, language

| Feature | Steps | Observed | Latency | Result |
|---|---|---|---|---|
| Example combobox | open | No example, Demo, Concrete Forest, House | | PASS |
| Example House | select | geometry of both windows replaced in 20 s; analysis `Preparing analysis` for 260 s+; switching back to Demo needed 186 s until `Case: dead` | 20 s / 260 s+ | PASS (slow) |
| Example Concrete Forest | | not run | | NOT RUN |
| Role Viewer | click Viewer | Viewer layout (Model only, Results and Inspection removed, Viewer pressed) after 4.2 s | 4.2 s | PASS |
| Role Editor after Viewer | click Editor | Editor button `disabled`, cannot return until reload | | FAIL (F8) |
| Window Focus (Model) | click Focus | other window hidden | 0.1 s | PASS |
| Window Focus again | | both windows back | | PASS |
| Top Fullscreen | click | browser fullscreen on, button becomes `Exit Fullscreen`, off again | 3 s | PASS |
| Window Close (Results, Model) | click Close | window removed; banner "Drag windows from Display in the navbar, or restore a saved layout"; Display panel lists Windows Results/Model | 2.5 s | PASS |
| Window reopen via Display | | not exercised (reload restores) | | NOT RUN |
| Settings panel | open | Driver (Reset panels, Merge policy, Terminology, Language, Driver, Layout, Appearance), General, App; tabs General/Theme/Hotkeys/Default apps/Conflicts | | PASS |
| Language EN -> DE -> EN | Settings > Language | whole UI (nav, trees, captions `Lastfall: dead`, tooltips) switches to German and back | 3 s | PASS |
| Theme/Appearance | | not exercised | | NOT RUN |

## FAIL diagnoses

F1 Left drag (no utility and Transform armed) and right drag do not move the camera. Middle drag, shift+middle, wheel work. Evidence `m3v2/c4-*.png`, `c5.png` vs `c6-*.png`: identical camera after fast and slow (30 steps x 30 ms) drags.
F2 Rotate gumball drag of about 60 px on `sol1` (ring at (578,712) -> (540,762)): slab vanishes from Model view, Results shows `Case: dead · Analysis failed` and a stale red slab (`g6-rot.png`, `g7.png`). Ctrl+Z restores it (`u1.png`). An 18 px drag works (`g8-rot-small.png`).
F3 Re-solve wedges or is very slow. After gumball move/scale + undo/redo cycles the caption stays `Case: dead · Preparing analysis` for 400 s+ (`w1.png`, `w3.png`); in another run 15 min+ after Delete/Undo/Redo (`m1.png`); after `addSolid` (concrete) it stays at `Preparing result view` 6 min+ with Results geometry stale (`mm.png`). Reload resets to Demo. A normal single edit re-solve is 25-33 s. Actions are greyed while analysis is pending.
F4 Actions pane in the Model window hides the viewport (pane plus empty area, no grid in the remaining 200 px): `m1.png`, `m6a.png`, `m12.png`. With Command/Settings/Artifact panels the viewport stays drawn.
F5 `addCombination` has no effect for 10 different `terms` strings and gives no error; `addSolid` with an unknown `materialId` is silently ignored (no validation message). Scripts `mf.js`, `mh.js`, `mk.js`.
F6 Command palette > Language > Set Locale: the arg input exists in the DOM with zero size (`n5.out`), unusable. Settings > Language works.
F7 History > Create Alternative: console `[actor] original-turn.semantic module.vcs: no checkpoint for alternative` and `[os-shell] tree window refresh failed ... native-return-refused` (`h3.out`).
F8 Role: after switching to Viewer the Editor button (`playground.navbar.roles.editor`) is `disabled`, no way back without reload (`w8.png`).
F9 Boot after Play/Pause had been toggled: `[semio-plugin panic] ... semio-framework-value-allocation-return-0.1.0 ... 152:62: shared original alias must retire or transfer`, then `scheduled dispatch of "resultAnimationTick" failed ... wasm trap: unreachable` (also once `plugin.internal.zero-progress: runtime close cleanup faulted for instance 1`). The Play/Pause button never shows pressed.
F10 Results Display panel: Source and Mode selections are not reflected in the combobox text (stay `Dead Load X`, `Static` after Modal); Source=Live Load changes neither caption nor geometry; Mode Index=1 does not change the `Mode 1` caption (`t4.out`, `t4-idx1.png`).
F11 Copy then Paste (empty args): console `UnsupportedOwner: natural-file import requires its owned declared media and no wire`, no element added (`y2.out`).
F12 Cosmetic: literal `ruler` text over the section icon in the Artifact tree; Poisson ratio number clipped.
F13 After a gumball move/scale drag the red (move/scale) or orange guide line stays across the viewport (`g2.png`, `g9-scale.png`, `g6-rot.png`).
F14 Transient: after toggling Materials/Sections groups the Load Cases children are missing for several seconds, then reappear (`a4.png`, `a5.png`); rows are virtualized so lookups need scrolling.

## Not tested

Member load edit, combination term edit, Section combobox on an element (only one section), Select All / Clear Selection / Set Selection Mode / Set Granularity / Translate / Rotate / Scale Selection / Set Result Display / Set Active Example / Cut / Export / Import actions (forms listed only), Concrete Forest example, Theme/Appearance, window reopen via Display, playback readouts in the Results window.
