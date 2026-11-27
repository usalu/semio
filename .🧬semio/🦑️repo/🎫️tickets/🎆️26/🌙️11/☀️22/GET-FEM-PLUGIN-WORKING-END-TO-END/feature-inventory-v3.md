# FEM Plugin Feature Inventory v3 (2D and 3D, editor and viewer)

Read-only audit. No source or ticket file was changed except this file.

Sources:
- Editor and viewer source under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/{✏️editor,👁️viewer}`: manifests (`📋️manifest` for 3D, `🚀️ui-helpers-and-app-assembly` for 2D), keybindings, utilities, `🚪️io`, `📚️examples`.
- Prior inventories: `feature-inventory-2d.md`, `feature-inventory-3d.md` (same folder).
- Browser results: `feature-matrix-2d-v2.md` (cited as 2D-v2), `feature-matrix-3d-v2.md` (3D-v2), and `☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE/untested-features-3d.md` (3D-u, row number `#n`).

Result column: PASS, FAIL or NOT TESTED, then the source row. "PASS (weak)" means the browser check only saw a visible side effect, not the full expected state. Tests were run against the builds recorded in those files, so later builds may differ.

Trigger column: exact accessible name or key. Keys: `mod` is Ctrl on Windows and Linux, Cmd on macOS.

## 2D Editor

| ID | Feature (action id / panel control / window) | Trigger in UI | Expected effect | Last browser result |
|---|---|---|---|---|
| E2D-01 | Editor role (boot) | Role button "Editor" | Editing UI shown | PASS (2D-v2 E2) |
| E2D-02 | Model window (structure view) | Window "Model" at boot | Structure canvas drawn | PASS (2D-v2 R8) |
| E2D-03 | Results window | Window "Results" at boot | Static results drawn | PASS (2D-v2 R8) |
| E2D-04 | Artifact panel open | Panel "Artifact" | Workbench tree shown | PASS (2D-v2 §6) |
| E2D-05 | Artifact panel does not cover window chrome | Open "Artifact" | Model tab bar, Focus and Close reachable | FAIL (2D-v2 X3) |
| E2D-06 | Inspection panel | Tree row or canvas pick | Entity form shown | PASS (2D-v2 I1) |
| E2D-07 | Results panel (bottom bar) | Bottom bar "Results" | Display, Playback, Analysis controls | PASS (2D-v2 R1) |
| E2D-08 | History panel | "History" | Actions and Commands lists | PASS (2D-v2 D6) |
| E2D-09 | Settings panel | Settings | Driver, General, App sections | PASS (2D-v2 L1) |
| E2D-10 | Window Close | Close button on window | Window removed | PASS (2D-v2 X1) |
| E2D-11 | Window Focus (Model) | Focus button on window | Window maximized, other hidden | FAIL (2D-v2 X2, restore) |
| E2D-12 | Command > Reset panels | Command > Layout > "Reset panels" | Closed window restored | FAIL (2D-v2 §5, X2) |
| E2D-13 | Command > Fullscreen | Command > Layout > "Fullscreen" | Document fullscreen | PASS (2D-v2 X4) |
| E2D-14 | Command menu | "Command" button | Layout, App, Appearance, Language tabs | PASS (2D-v2 §5) |
| E2D-15 | Language EN to DE | Settings > Language > "Deutsch" | All labels German | PASS (2D-v2 L2) |
| E2D-16 | Theme / Appearance | Settings > Driver > Appearance | UI theme changes | NOT TESTED (no 2D row) |
| E2D-17 | Wheel zoom | Mouse wheel over Model | View zooms | PASS (2D-v2 L3) |
| E2D-18 | Example "Demo" at boot | Example combobox "Demo" | Demo model loaded | PASS (2D-v2 §4 boot) |
| E2D-19 | Example "No example" then "Demo" | Example combobox | Model restored, Results recover | FAIL (2D-v2 E1) |
| E2D-20 | Role Viewer and back to Editor | "Viewer" then "Editor" | Viewer layout, then editing again | PASS (2D-v2 E2) |
| E2D-21 | Utility Direct Select armed at boot | Utilities > "Direct Select" (pressed state) | Shown pressed at boot | FAIL (2D-v2 U1) |
| E2D-22 | Direct Select click | Utilities > "Direct Select", click node | Node selected | PASS (2D-v2 U2) |
| E2D-23 | Marquee Select (`selectMarquee`) | Utilities > "Marquee Select", drag | Entities in rectangle selected | PASS (2D-v2 U3) |
| E2D-24 | Lasso Select (`selectLasso`) | Utilities > "Lasso Select", draw polygon | Entities in polygon selected | PASS (2D-v2 U4) |
| E2D-25 | Pan (`transformMove`) | Utilities > "Pan", slow drag | Model moves by drag | PASS (2D-v2 U5) |
| E2D-26 | Transform utility, gumball shown | Utilities > "Transform" with selection | Gumball drawn | PASS (2D-v2 U6) |
| E2D-27 | Gumball translate (`translateSelection`) | Drag red handle on gumball | Node moves by drag | FAIL (2D-v2 U7, T1) |
| E2D-28 | Gumball rotate (`rotateSelection`) | Drag rotate handle | Selection rotates | NOT TESTED |
| E2D-29 | Gumball scale (`scaleSelection`) | Drag scale handle | Selection scales | NOT TESTED |
| E2D-30 | Transform options Move, Rotate, Scale Axes, Uniform Scale | Checkboxes of those names | Matching gumball handles hide | FAIL (2D-v2 U8, aria toggles only) |
| E2D-31 | Gumball pointer cancel | Pointer cancel during drag | Selection kept | NOT TESTED |
| E2D-32 | Click pick replaces selection | Click entity | Entity selected alone | PASS (2D-v2 S1) |
| E2D-33 | Shift+click additive | `shift` + click | Entity added | PASS (2D-v2 S2a) |
| E2D-34 | Ctrl+click subtractive | `mod` + click selected entity | Entity removed | PASS (2D-v2 S2b) |
| E2D-35 | Shift+Ctrl invertive | `shift`+`mod` + click | Entity toggled | NOT TESTED |
| E2D-36 | Mode radio "Selective" | Radio "Selective" then marquee | Replaces selection | PASS (2D-v2 S2c) |
| E2D-37 | Mode radio "Additive" | Radio "Additive" then click or marquee | Adds to selection | FAIL (2D-v2 S2d) |
| E2D-38 | Mode radio "Subtractive" | Radio "Subtractive" then click or marquee | Removes from selection | FAIL (2D-v2 S2e) |
| E2D-39 | Mode radio "Invertive" | Radio "Invertive" then click or marquee | Toggles selection | FAIL (2D-v2 S2f) |
| E2D-40 | Hover highlight | Hover 1.5 s on entity | Entity highlighted | NOT TESTED (used only as precondition) |
| E2D-41 | Empty click clears selection | Click empty canvas | Summary shown | PASS (2D-v2 S3) |
| E2D-42 | Marquee under 4 px acts as click | Tiny drag | Click behaviour | NOT TESTED |
| E2D-43 | "N SELECTED" header | Multi-select | Count and entity list | PASS (2D-v2 S4) |
| E2D-44 | Artifact tree row select (Node, Element, Region, Support, Load, Load Case, Combination, Material, Section) | Click tree row | Inspection shows entity | PASS (2D-v2 §6) |
| E2D-45 | Artifact tree Analysis row select | Click "Analysis" row | Inspection switches | FAIL (2D-v2 A6) |
| E2D-46 | Artifact tree Section row label | View tree | Label clear of icon | FAIL (2D-v2 §6, cosmetic overlap) |
| E2D-47 | Delete key removes selection (canvas) | Canvas select then `delete` | Entity removed | FAIL (2D-v2 D2) |
| E2D-48 | Delete key removes selection (tree) | Tree row select then `delete` | Entity removed | FAIL (2D-v2 D2) |
| E2D-49 | Backspace removes selection (tree) | Tree row select then `backspace` | Entity removed | FAIL (2D-v2 D2b) |
| E2D-50 | Backspace removes selection (canvas) | Canvas select then `backspace` | Entity removed | NOT TESTED (2D-v2 D2b) |
| E2D-51 | Inspection "Delete" on element (`removeSelection`) | Inspection "Delete" | Element removed | PASS (2D-v2 D1) |
| E2D-52 | Inspection "Delete" on node | Inspection "Delete" on n2 | Node and dependents removed | FAIL (2D-v2 D3) |
| E2D-53 | Inspection "Delete" on support, load, region, material, section, combination, load case | Inspection "Delete" | Entity removed | NOT TESTED |
| E2D-54 | Remove combination term (factor 0) | Term factor 0 | Term removed | NOT TESTED |
| E2D-55 | Inspection "Focus" (`focus-entity`) | Inspection "Focus" on element | Camera recentres | FAIL (2D-v2 I9) |
| E2D-56 | Node X (`patch-node` x) | Inspection "X" | Node moves, Results re-solve | PASS (2D-v2 I1) |
| E2D-57 | Node Y (`patch-node` y) | Inspection "Y" | Node moves | PASS (2D-v2 I2) |
| E2D-58 | Element Kind (`patch-element` kind) | Inspection "Kind" combobox | Bar or Beam | PASS (2D-v2 I3) |
| E2D-59 | Element Start / End (`patch-element`) | Inspection "Start", "End" comboboxes | Node reassigned | PASS (2D-v2 I3) |
| E2D-60 | Element Material (`patch-element` materialId) | Inspection "Material" combobox | Material changed | PASS (2D-v2 I3) |
| E2D-61 | Element Section (`patch-element` sectionId) | Inspection "Section" combobox | Section changed | PASS (2D-v2 I3) |
| E2D-62 | Material Name (`patch-material` name) | Inspection "Name" | Name changed | PASS (2D-v2 I4) |
| E2D-63 | Material Young's Modulus (`patch-material` e) | Inspection "Young's Modulus" | Value changed | PASS (2D-v2 I4) |
| E2D-64 | Material Poisson Ratio slider | Inspection "Poisson Ratio" slider | Value changed | NOT TESTED |
| E2D-65 | Material Density (`patch-material` rho) | Inspection "Density" | Value changed | NOT TESTED |
| E2D-66 | Section Name (`patch-section` name) | Inspection "Name" | Name changed | NOT TESTED |
| E2D-67 | Section Area (`patch-section` area) | Inspection "Area" | Value changed | PASS (2D-v2 I5) |
| E2D-68 | Section Second Moment of Area (`patch-section`) | Inspection "Second Moment of Area" | Value changed | NOT TESTED |
| E2D-69 | Support Node (`patch-support` nodeId) | Inspection "Node" combobox | Support moves | NOT TESTED |
| E2D-70 | Support Tx (`patch-support` tx) | Inspection "Tx" toggle | Restraint toggled | PASS (2D-v2 I6) |
| E2D-71 | Support Ty, Rz (`patch-support`) | Inspection "Ty", "Rz" toggles | Restraint toggled | NOT TESTED |
| E2D-72 | Region Name (`patch-region` name) | Inspection "Name" | Name changed | PASS (2D-v2 I7) |
| E2D-73 | Region Thickness slider (`patch-region`) | Inspection "Thickness" slider | Value changed | NOT TESTED |
| E2D-74 | Region Mesh Size (`patch-region` meshSize) | Inspection "Mesh Size" | Value changed | PASS (2D-v2 I7) |
| E2D-75 | Region Material (`patch-region` materialId) | Inspection "Material" | Material changed | PASS (2D-v2 I7) |
| E2D-76 | Nodal load value (`patch-load` nodal) | Inspection load value | Load changed | PASS (2D-v2 I8) |
| E2D-77 | Member UDL (`patch-load` udl) | Inspection UDL field | Load changed | NOT TESTED |
| E2D-78 | Area load pressure (`patch-load` area) | Inspection pressure field | Load changed | PASS (2D-v2 I8) |
| E2D-79 | Load Case Name (`patch-load-case` name) | Inspection "Name" | Name changed | PASS (2D-v2 I8b) |
| E2D-80 | Load Case Self Weight (`patch-load-case` selfWeight) | Inspection "Self Weight" toggle | Tag removed or added | PASS (2D-v2 I8b) |
| E2D-81 | Combination Name (`patch-combination` name) | Inspection "Name" | Name changed | PASS (2D-v2 I8c) |
| E2D-82 | Combination term factor (`patch-combination`) | Inspection term factor | Factor changed | PASS (2D-v2 I8c) |
| E2D-83 | Combination "Add Term" | Inspection "Add Term" | Term added | FAIL (2D-v2 I10, control absent) |
| E2D-84 | Summary (nothing selected) | Empty selection | Counts and analysis rows | PASS (2D-v2 S3) |
| E2D-85 | Artifact tree groups and counts | Artifact panel | Sections with counts | PASS (2D-v2 §6) |
| E2D-86 | Artifact tree Materials, Sections, Analysis expand | Chevron | Child rows shown | PASS (2D-v2 §6) |
| E2D-87 | Action "Add Node" (`add-node`) | Actions palette, Model > Actions | Node count +1 | PASS (2D-v2 §5) |
| E2D-88 | Action "Add Bar" (`add-bar`) | Actions palette | Element count +1 | PASS (2D-v2 §5) |
| E2D-89 | Action "Add Beam" (`add-beam`) | Actions palette | Element count +1 | PASS (2D-v2 §5) |
| E2D-90 | Action "Add Material" (`add-material`) | Actions palette | Material count +1 (slow, 36 s) | PASS (2D-v2 §5) |
| E2D-91 | Action "Add Section" (`add-section`) | Actions palette | Section count +1 (retry needed) | PASS (2D-v2 §5) |
| E2D-92 | Action "Add Support" (`add-support`) | Actions palette | Support count +1 | PASS (2D-v2 §5) |
| E2D-93 | Action "Add Nodal Load" (`add-nodal-load`) | Actions palette | Load added | PASS (2D-v2 §5) |
| E2D-94 | Action "Add Member UDL" (`add-member-udl`) | Actions palette | Load added | PASS (2D-v2 §5) |
| E2D-95 | Action "Add Area Load" (`add-area-load`) | Actions palette | Load added | PASS (2D-v2 §5) |
| E2D-96 | Action "Add Region" (`add-region`) | Actions palette | Region count +1 (slow, 31 s) | PASS (2D-v2 §5) |
| E2D-97 | Action "Add Load Case" (`add-load-case`) | Actions palette | Load case added | PASS (2D-v2 §5) |
| E2D-98 | Action "Add Combination" (`add-combination`) | Actions palette | Combination added | PASS (2D-v2 §5) |
| E2D-99 | Action "Set Self Weight" (`set-self-weight`) | Actions palette | Self Weight tag set | PASS (2D-v2 §5) |
| E2D-100 | Action "Set Analysis Settings" (`set-analysis-settings`) | Actions palette, 4, 2, 100 | Deformation visibly shrinks | PASS (2D-v2 §5, visual) |
| E2D-101 | Action "Clear Selection" | Actions palette | Selection cleared | NOT TESTED |
| E2D-102 | Action "Select All" | Actions palette | All selected | NOT TESTED |
| E2D-103 | Action "Set Selection Mode" | Actions palette | Mode set | NOT TESTED |
| E2D-104 | Action "Set Granularity" | Actions palette | Granularity set | NOT TESTED |
| E2D-105 | Action "Remove Selection" (`remove-selection`) | Actions palette | Selection removed | NOT TESTED |
| E2D-106 | Actions "Translate Selection", "Rotate Selection", "Scale Selection" | Actions palette | Selection moves | NOT TESTED |
| E2D-107 | Actions "Copy", "Cut", "Paste" | Actions palette | Clipboard operations | NOT TESTED |
| E2D-108 | Action "Set Result Display" (`set-result-display`) | Actions palette | Display changes | NOT TESTED (panel path only, E2D-112 onward) |
| E2D-109 | Action "Set Active Example" (`set-active-example`) | Actions palette or Example combobox | Model replaced | FAIL (2D-v2 E1, via combobox) |
| E2D-110 | Export Document (`.semio-archive`) | Actions palette "Export Document" | File downloads | NOT TESTED |
| E2D-111 | Import Document | Actions palette "Import Document" | Archive restored | NOT TESTED |
| E2D-112 | Undo via keys (`mod+z`) | `mod` + `z` after edit | Edit reverted | FAIL (2D-v2 D4) |
| E2D-113 | Redo via keys (`mod+shift+z`) | `mod` + `shift` + `z` | Edit re-applied | FAIL (2D-v2 D5) |
| E2D-114 | Redo via keys (`mod+y`) | `mod` + `y` | Edit re-applied | FAIL (2D-v2 D5) |
| E2D-115 | Action "Undo" and History "Undo" button | History panel "Undo" | Edit reverted | FAIL (2D-v2 D6) |
| E2D-116 | History panel "Redo" | History panel "Redo" | Edit re-applied | FAIL (2D-v2 D5, disabled) |
| E2D-117 | Action "Commit Checkpoint" | Actions palette | Checkpoint created | NOT TESTED |
| E2D-118 | Action "Create Alternative" | Actions palette | Alternative created | NOT TESTED |
| E2D-119 | Action "Switch Alternative" | Actions palette | Alternative switched | NOT TESTED |
| E2D-120 | Action "Checkout Checkpoint" | Actions palette | Checkpoint restored | NOT TESTED |
| E2D-121 | Results Display Source "Live Load" (`set-result-display` sourceId) | Results panel "Source" combobox | Results change | PASS (2D-v2 R2) |
| E2D-122 | Results Display Source "Dead Load", "ULS" | Results panel "Source" combobox | Results change | NOT TESTED |
| E2D-123 | Results Display Mode "Modal" (`set-result-display` mode) | Results panel "Mode" combobox | Mode shape shown | FAIL (2D-v2 R3) |
| E2D-124 | Results Display Mode "Buckling" | Results panel "Mode" combobox | Buckling mode shown | NOT TESTED |
| E2D-125 | Results Mode Index | Results panel "Mode Index" | Mode index changes shape | NOT TESTED |
| E2D-126 | Deformation Scale (`set-analysis-settings` deformationScale) | Results panel "Deformation Scale" | Deformation magnified | FAIL (2D-v2 R4) |
| E2D-127 | Playback Step + (`set-result-animation` phaseStep) | Results panel "Step +" | Frame advances | PASS (2D-v2 R5) |
| E2D-128 | Playback Step - (`set-result-animation` phaseStep) | Results panel "Step -" | Frame goes back | NOT TESTED (not measurable) |
| E2D-129 | Playback Play / Pause (`set-result-animation` playing) | Results panel "Play" / "Pause" | Frames animate | FAIL (2D-v2 R6, wasm panic) |
| E2D-130 | Playback Space toggle (`setResultAnimation`) | `space` over Results window | Playback toggles | FAIL (2D-v2 R7) |
| E2D-131 | Playback Speed slider | Results panel "Speed" | Speed changes | NOT TESTED |
| E2D-132 | Playback Loop (Loop, Ping-Pong, Once) | Results panel "Loop" combobox | Loop mode | NOT TESTED |
| E2D-133 | Playback Waveform (Ramp, Sine) | Results panel "Waveform" combobox | Waveform | NOT TESTED |
| E2D-134 | Playback Phase slider | Results panel "Phase" | Phase set | NOT TESTED |
| E2D-135 | Analysis Modal Count, Buckling Count fields | Results panel "Modal Count", "Buckling Count" | Count changes | NOT TESTED (palette path PASS, E2D-100) |
| E2D-136 | Results window render (static deformation, reactions, caption) | Boot | Caption not overlapping | PASS (2D-v2 R8) |
| E2D-137 | Re-solve after edit | Any model edit | Results update | PASS (2D-v2 R9) |
| E2D-138 | Viewer read-only model window (2D viewer app) | Role "Viewer" | Structure shown, no editing | PASS (2D-v2 E2, layout only) |

Import and export formats (2D editor): `.semio-archive` (E2D-110, E2D-111). The `🚪️io` folder holds internal protocol grammars (text `.semio` grammar, binary snapshot, diff and mutations); no user control was found for them.

Examples (2D): only `🎬️demo` ("Demo"). See E2D-18 and E2D-19.

## 3D Editor

| ID | Feature (action id / panel control / window) | Trigger in UI | Expected effect | Last browser result |
|---|---|---|---|---|
| E3D-01 | Edit mode (default) | Boot | Editor UI | PASS (3D-v2 §1) |
| E3D-02 | Model window | Window "Model" | Structure drawn | PASS (3D-v2 §1) |
| E3D-03 | Results window | Window "Results" | Deformed or coloured structure | PASS (3D-v2 §6) |
| E3D-04 | Default two-window layout | Boot | Model left, Results right | PASS (3D-v2 §6, boot) |
| E3D-05 | Artifact panel | Tab "Artifact" | Outliner tree | PASS (3D-v2 §5) |
| E3D-06 | Inspection panel | Tab "Inspection" or tree row | Entity form | PASS (3D-v2 §3) |
| E3D-07 | Results panel | Panel "Results" | Display, Playback, Analysis controls | PASS (3D-v2 §6) |
| E3D-08 | Utility Transform armed | Utilities > "Transform" | Gumball and option rail shown | PASS (3D-v2 §1) |
| E3D-09 | Option "Move" (`set-transform-gumball-flag` move) | Checkbox "Move" | Arrow handles hide and show | PASS (3D-v2 §1) |
| E3D-10 | Option "Move Planes" (`movePlanes`) | Checkbox "Move Planes" | Plane handles hide and show | PASS (3D-v2 §1, lumped) |
| E3D-11 | Option "Rotate" (`rotate`) | Checkbox "Rotate" | Rings hide and show | PASS (3D-v2 §1) |
| E3D-12 | Option "Scale Axes" (`scaleAxes`) | Checkbox "Scale Axes" | Axis scale handles hide and show | PASS (3D-v2 §1, lumped) |
| E3D-13 | Option "Uniform Scale" (`scaleUniform`) | Checkbox "Uniform Scale" | Uniform handle hides and shows | PASS (3D-v2 §1, lumped) |
| E3D-14 | Pick entity without utility | Click column in Model | Entity selected | PASS (3D-v2 §1) |
| E3D-15 | Pick entity with Transform armed | Click column | Selected, gumball drawn | PASS (3D-v2 §1) |
| E3D-16 | Gumball move (`translate-selection`) | Drag axis arrow | Geometry moves, Results update | PASS (3D-v2 §1) |
| E3D-17 | Gumball plane handle (`translate-selection`) | Drag plane handle (Move Planes) | Geometry moves on plane | NOT TESTED |
| E3D-18 | Gumball rotate, small drag (`rotate-selection`) | Drag Z ring about 18 px | Solid rotates | PASS (3D-v2 §1) |
| E3D-19 | Gumball rotate, large drag | Drag Z ring about 60 px | Solid rotates | FAIL (3D-v2 §1, F2: slab vanishes) |
| E3D-20 | Gumball scale axis (`scale-selection`) | Drag scale cube | Solid scales | PASS (3D-v2 §1, weak) |
| E3D-21 | Gumball uniform scale (`scale-selection`) | Drag uniform handle | Solid scales uniformly | NOT TESTED |
| E3D-22 | Gumball release is one undo step | `mod+z` after rotate | Gesture reverted | PASS (3D-v2 §1, F19) |
| E3D-23 | Gumball guide line cleared after drag | View after drag | Guide line removed | FAIL (3D-v2 F13) |
| E3D-24 | Marquee select (Rectangle) in Model | Drag rectangle | Entities inside selected | NOT TESTED |
| E3D-25 | Hover highlight | Hover entity | Highlight in tree and viewport | NOT TESTED |
| E3D-26 | Action "Select All" (`select-all`) | Actions pane "Select All" | All selected | FAIL (3D-u #4, F-A) |
| E3D-27 | Action "Clear Selection" | Actions pane "Clear Selection" | Selection cleared | PASS (3D-u #5) |
| E3D-28 | "N Selected" header | Multi-select | Count shown | NOT TESTED |
| E3D-29 | Modifier merges Additive, Subtractive, Invertive | `shift` / `mod` + click | Merge applied | NOT TESTED |
| E3D-30 | Empty click deselect | Click ground | Summary shown | PASS (3D-v2 §1) |
| E3D-31 | Action "Set Selection Mode" (`set-selection-mode`) | Actions pane, domain `fem3d`, Multiple | Mode set | FAIL (3D-u #6, F-B) |
| E3D-32 | Action "Set Granularity" (`set-granularity`) | Actions pane, domain `fem3d`, element | Granularity set | NOT TESTED (3D-u #7, not verified) |
| E3D-33 | Camera orbit, middle drag (`set-camera`) | Middle drag in Model | Orbit | PASS (3D-v2 §1) |
| E3D-34 | Camera orbit, left drag | Left drag on ground | Orbit | FAIL (3D-v2 F1) |
| E3D-35 | Camera orbit, left drag with Transform armed | Left drag | Orbit | FAIL (3D-v2 F1) |
| E3D-36 | Camera pan, shift + middle drag | `shift` + middle drag | Pan | PASS (3D-v2 §1) |
| E3D-37 | Camera pan, right drag | Right drag | Pan | FAIL (3D-v2 F1) |
| E3D-38 | Camera zoom, wheel | Mouse wheel | Zoom | PASS (3D-v2 §1) |
| E3D-39 | Focus Entity for node (`focus-entity`) | Inspection "Focus" on node | Camera re-aimed | PASS (3D-v2 §3) |
| E3D-40 | Focus Entity for element, solid, support, load | Inspection "Focus" | Camera re-aimed | NOT TESTED |
| E3D-41 | Window Focus (Model) and back | Window "Focus", then again | Other window hidden, then restored | PASS (3D-v2 §7) |
| E3D-42 | Delete key removes selection | Viewport focused, `delete` | Element removed | PASS (3D-v2 §2) |
| E3D-43 | Backspace removes selection | Viewport focused, `backspace` | Element removed | PASS (3D-v2 §2) |
| E3D-44 | Inspection "Delete" (`remove-selection`) | Inspection "Delete" | Element removed | PASS (3D-v2 §2) |
| E3D-45 | Undo (`mod+z`) | After delete | Element restored | PASS (3D-v2 §2) |
| E3D-46 | Redo (`mod+shift+z`) | After undo | Re-applied | PASS (3D-v2 §2) |
| E3D-47 | Redo (`mod+y`) | After undo | Re-applied | PASS (3D-v2 §2) |
| E3D-48 | History Undo and Redo buttons | History panel | Counts change | PASS (3D-v2 §2) |
| E3D-49 | Action "Commit Checkpoint" | History or Actions pane | Checkpoint created | PASS (3D-v2 §2, weak) |
| E3D-50 | Action "Check In" | History panel | Checked in | PASS (3D-v2 §2, weak) |
| E3D-51 | Action "Create Alternative" | History panel | Alternative created | FAIL (3D-v2 F7) |
| E3D-52 | Action "Switch Alternative" | Actions pane | Alternative switched | NOT TESTED |
| E3D-53 | Action "Checkout Checkpoint" | Actions pane | Checkpoint restored | NOT TESTED |
| E3D-54 | History command log rows | After edits | Rows listed | PASS (3D-v2 §2) |
| E3D-55 | Summary (nothing selected) | Empty selection | Counts (Schema, Nodes, ...) | PASS (3D-v2 §1) |
| E3D-56 | Node X, Y, Z (`patch-node`) | Inspection "X", "Y", "Z" | Node moves | PASS (3D-v2 §3) |
| E3D-57 | Element Kind Frame and Bar (`patch-element` kind) | Inspection "Kind" combobox | Roll row shows and hides | PASS (3D-v2 §3) |
| E3D-58 | Element Roll (`patch-element` roll) | Inspection "Roll" | Roll changed | PASS (3D-v2 §3) |
| E3D-59 | Element Start, End (`patch-element`) | Inspection "Start", "End" | Node reassigned | PASS (3D-v2 §3) |
| E3D-60 | Element Material (`patch-element` materialId) | Inspection "Material" | Changed | PASS (3D-v2 §3) |
| E3D-61 | Element Section (`patch-element` sectionId) | Inspection "Section" | Changed | PASS (3D-u #3) |
| E3D-62 | Material Name (`patch-material` name) | Inspection "Name" | Renamed | PASS (3D-v2 §3) |
| E3D-63 | Material Young's Modulus (`patch-material` e) | Inspection "Young's Modulus" | Changed | PASS (3D-v2 §3) |
| E3D-64 | Material Shear Modulus (`patch-material` g) | Inspection "Shear Modulus" | Changed | NOT TESTED |
| E3D-65 | Material Poisson Ratio slider (`patch-material` nu) | Inspection "Poisson Ratio" | Changed | NOT TESTED |
| E3D-66 | Material Density (`patch-material` rho) | Inspection "Density" | Changed | NOT TESTED |
| E3D-67 | Section Name (`patch-section` name) | Inspection "Name" | Renamed | PASS (3D-v2 §3) |
| E3D-68 | Section Area (`patch-section` area) | Inspection "Area" | Changed | PASS (3D-v2 §3) |
| E3D-69 | Section Second Moment Iy (`patch-section` iy) | Inspection "Second Moment Iy" | Changed | NOT TESTED |
| E3D-70 | Section Second Moment Iz (`patch-section` iz) | Inspection "Second Moment Iz" | Changed | NOT TESTED |
| E3D-71 | Section Torsion Constant (`patch-section` j) | Inspection "Torsion Constant" | Changed | NOT TESTED |
| E3D-72 | Support Node (`patch-support` nodeId) | Inspection "Node" combobox | Support moves | PASS (3D-v2 §3) |
| E3D-73 | Support DOF toggles Tx to Rz (`patch-support`) | Inspection buttons Tx, Ty, Tz, Rx, Ry, Rz | Restraint toggled | PASS (3D-v2 §3) |
| E3D-74 | Solid Name (`patch-solid` name) | Inspection "Name" | Renamed | PASS (3D-v2 §3) |
| E3D-75 | Solid Axis (`patch-solid` axis) | Inspection "Axis" combobox | Axis changed | PASS (3D-v2 §3) |
| E3D-76 | Solid Base (`patch-solid` baseZ) | Inspection "Base" | Changed | PASS (3D-v2 §3) |
| E3D-77 | Solid Height (`patch-solid` height) | Inspection "Height" | Changed | PASS (3D-v2 §3) |
| E3D-78 | Solid Layers (`patch-solid` layers) | Inspection "Layers" | Changed | PASS (3D-v2 §3) |
| E3D-79 | Solid Mesh Size (`patch-solid` meshSize) | Inspection "Mesh Size" | Changed | PASS (3D-v2 §3) |
| E3D-80 | Solid Material (`patch-solid` materialId) | Inspection "Material" | Changed | PASS (3D-v2 §3) |
| E3D-81 | Solid Outline and Holes read-only rows | Inspection | Read-only values | NOT TESTED |
| E3D-82 | Nodal load value (`patch-load` value) | Inspection "Value" | Changed | PASS (3D-v2 §3) |
| E3D-83 | Nodal load node and DOF (`patch-load` nodeId, dof) | Inspection "Node", "Dof" | Changed | NOT TESTED |
| E3D-84 | Member load wz (`patch-load` wz) | Inspection "wz" | Changed | PASS (3D-u #1) |
| E3D-85 | Member load wx, wy (`patch-load`) | Inspection "wx", "wy" | Changed | NOT TESTED |
| E3D-86 | Area load pressure (`patch-load` value) | Inspection "Pressure" | Changed | PASS (3D-v2 §3) |
| E3D-87 | Load Case Name (`patch-load-case` name) | Inspection "Name" | Renamed | PASS (3D-v2 §3) |
| E3D-88 | Load Case Self Weight (`patch-load-case` selfWeight) | Inspection "Self Weight" toggle | Tag toggled | PASS (3D-v2 §3) |
| E3D-89 | Load pick buttons in Load Case | Inspection "Loads" list | Selects that load | PASS (3D-v2 §5) |
| E3D-90 | Combination Name (`patch-combination` name) | Inspection "Name" | Renamed | PASS (3D-v2 §3) |
| E3D-91 | Combination term factor (`patch-combination` term) | Inspection term number input | Factor changed | PASS (3D-u #2) |
| E3D-92 | Combination "Add Term" (`patch-combination` addTerm) | Inspection "Add Term" | Term added | NOT TESTED |
| E3D-93 | Combination "Delete Term" (`patch-combination` removeTerm) | Inspection "Delete Term" | Term removed | NOT TESTED |
| E3D-94 | Action "Add Node" (`add-node`) | Actions pane | Node appears in tree | PASS (3D-v2 §4) |
| E3D-95 | Action "Add Bar" (`add-bar`) | Actions pane | Element appears | PASS (3D-v2 §4) |
| E3D-96 | Action "Add Frame" (`add-frame`) | Actions pane | Element appears | PASS (3D-v2 §4) |
| E3D-97 | Action "Add Material" (`add-material`) | Actions pane | Material appears | PASS (3D-v2 §4) |
| E3D-98 | Action "Add Section" (`add-section`) | Actions pane | Section appears | PASS (3D-v2 §4) |
| E3D-99 | Action "Add Support" (`add-support`) | Actions pane | Support appears | PASS (3D-v2 §4) |
| E3D-100 | Action "Add Nodal Load" (`add-nodal-load`) | Actions pane | Load appears | PASS (3D-v2 §4) |
| E3D-101 | Action "Add Member UDL" (`add-member-udl`) | Actions pane | Load appears | PASS (3D-v2 §4) |
| E3D-102 | Action "Add Area Load" (`add-area-load`) | Actions pane | Load appears | PASS (3D-v2 §4) |
| E3D-103 | Action "Add Solid" (`add-solid`), known material | Actions pane | Solid appears | PASS (3D-v2 §4) |
| E3D-104 | Action "Add Solid", unknown material | Actions pane, materialId `c30` | Error message | FAIL (3D-v2 F5, silent) |
| E3D-105 | Action "Add Load Case" (`add-load-case`) | Actions pane | Load case appears | PASS (3D-v2 §4) |
| E3D-106 | Action "Add Combination" (`add-combination`) | Actions pane, terms in all tried formats | Combination appears | FAIL (3D-v2 F5, silent) |
| E3D-107 | Action "Set Self Weight" (`set-self-weight`) | Actions pane | Self Weight set | PASS (3D-v2 §4) |
| E3D-108 | Action "Set Analysis Settings" (`set-analysis-settings`) | Actions pane | Settings applied | PASS (3D-v2 §4) |
| E3D-109 | Action "Remove Selection" (palette) | Actions pane | Selection removed | NOT TESTED (key path PASS, E3D-42) |
| E3D-110 | Action "Translate Selection" (`translate-selection`) | Actions pane, dx 0.5 | Element moves | PASS (3D-u #8, weak) |
| E3D-111 | Action "Rotate Selection" (`rotate-selection`) | Actions pane, axis Z, angle 3 | Element rotates | PASS (3D-u #9) |
| E3D-112 | Action "Scale Selection" (`scale-selection`) | Actions pane, sx 1.3 | Element scales | PASS (3D-u #10, weak) |
| E3D-113 | Action "Set Result Display" (`set-result-display`) | Actions pane, Live Load, Modal | Caption and display change | FAIL (3D-u #11, F-C) |
| E3D-114 | Action "Set Active Example" Demo and House | Actions pane or Example combobox | Model replaced | PASS (3D-v2 §7) |
| E3D-115 | Action "Set Active Example" Concrete Forest | Actions pane or Example combobox | Model replaced and analysed | NOT TESTED (3D-u #12, not verified) |
| E3D-116 | Action "Copy" and "Paste" | Actions pane | Element pasted | FAIL (3D-v2 F11) |
| E3D-117 | Action "Cut" | Actions pane, element selected | Element to clipboard | FAIL (3D-u #13, F-E) |
| E3D-118 | Export Document (`.semio-archive`) | Actions pane "Export Document" | Archive downloads | PASS (3D-u #14, clean page) |
| E3D-119 | Import Document, probe JSON | Actions pane "Import Document" | Probe rejected with message | PASS (3D-u #15) |
| E3D-120 | Import Document, exported archive | Actions pane "Import Document" | Archive restored, edits still work | FAIL (3D-u #15, F-F) |
| E3D-121 | Action "Undo" and "Redo" (palette) | Actions pane | Edit reverted or re-applied | NOT TESTED (key path PASS) |
| E3D-122 | Action "Select All" (palette) | Actions pane | All selected | FAIL (3D-u #4) |
| E3D-123 | Action "Set Camera" (`set-camera`) | No fem3d control (host gesture) | Camera set | NOT TESTED |
| E3D-124 | Artifact tree groups and counts | Artifact panel | Nine groups with counts | PASS (3D-v2 §5) |
| E3D-125 | Artifact tree Materials, Sections expand | Click group | Children shown | PASS (3D-v2 §5) |
| E3D-126 | Artifact tree Load Cases children (virtualized) | Expand Load Cases | Loads shown after a short delay | PASS (3D-v2 §5, F14 transient) |
| E3D-127 | Artifact tree row select (node, element, solid, support, load case, combination, material, section, load) | Click row | Inspection switches | PASS (3D-v2 §5) |
| E3D-128 | Artifact tree Analysis row | Click "Analysis" | Inspection unchanged (read-only) | PASS (3D-v2 §5, weak) |
| E3D-129 | Artifact tree Section row icon label | View tree | Label clear of icon | FAIL (3D-v2 F12, cosmetic) |
| E3D-130 | Poisson ratio value in Inspection | View Poisson ratio | Value fully visible | FAIL (3D-v2 F12, cosmetic) |
| E3D-131 | Results Source "Dead Load" (`set-result-display` sourceId) | Results panel "Source" | Dead Load shown | PASS (3D-v2 §6) |
| E3D-132 | Results Source "Live Load" | Results panel "Source" combobox | Live Load results | FAIL (3D-v2 F10) |
| E3D-133 | Results Mode "Static" | Results panel "Mode" | Static shown | PASS (3D-v2 §6) |
| E3D-134 | Results Mode "Modal" | Results panel "Mode" | Mode shape caption and combobox | FAIL (3D-v2 F10, combobox stays "Static") |
| E3D-135 | Results Mode "Buckling" | Results panel "Mode" | Buckling caption | NOT TESTED |
| E3D-136 | Results Mode Index (`set-result-display` modeIndex) | Results panel "Mode Index" | Caption changes | FAIL (3D-v2 F10) |
| E3D-137 | Playback Phase slider (`set-result-animation` phase) | Results panel "Phase" | Phase set | NOT TESTED |
| E3D-138 | Playback Step + (`set-result-animation` phaseStep) | Results panel "Step +" | Phase advances | FAIL (3D-u #19) |
| E3D-139 | Playback Step - (`set-result-animation` phaseStep) | Results panel "Step -" | Phase goes back | NOT TESTED |
| E3D-140 | Playback Play / Pause (`set-result-animation` playing) | Results panel "Play" / "Pause" | Animates, then pauses | FAIL (3D-v2 F9, 3D-u #19 panic) |
| E3D-141 | Playback Space toggle | `space` over Results | Playback toggles | NOT TESTED |
| E3D-142 | Playback Speed slider | Results panel "Speed" | Speed set | NOT TESTED |
| E3D-143 | Playback Loop (Loop, Ping-Pong, Once) | Results panel "Loop" | Loop mode | NOT TESTED |
| E3D-144 | Playback Waveform (Ramp, Sine) | Results panel "Waveform" | Waveform | NOT TESTED |
| E3D-145 | Playback readouts in Results caption | Play in Results window | Phase and Hz readouts update, freeze on pause | FAIL (3D-u #19, F-G) |
| E3D-146 | Analysis Modal Count (`set-analysis-settings` modalCount) | Results panel "Modal Count" | Count changes | PASS (3D-v2 §6) |
| E3D-147 | Analysis Buckling Count | Results panel "Buckling Count" | Count changes | NOT TESTED |
| E3D-148 | Deformation Scale (`set-analysis-settings` deformationScale) | Results panel "Deformation Scale" | Deformation magnified | NOT TESTED |
| E3D-149 | Results caption, static "Case: dead" | Results window | Caption shown | PASS (3D-v2 §6) |
| E3D-150 | Results caption, buckling | Buckling mode | "Buckling mode 1: Factor" caption | NOT TESTED |
| E3D-151 | Results caption, progress and error states | Force analysis error | Error caption | NOT TESTED |
| E3D-152 | Re-solve after single edit | Any edit | Caption returns to plain case | PASS (3D-v2 §6) |
| E3D-153 | Re-solve stability after many edits | Gumball and undo cycles, addSolid | Results re-solve | FAIL (3D-v2 F3) |
| E3D-154 | Example combobox listing | Example combobox | Lists No example, Demo, Concrete Forest, House | PASS (3D-v2 §7) |
| E3D-155 | Example "Demo" | Example combobox | Demo model | PASS (3D-v2 §7) |
| E3D-156 | Example "House" | Example combobox | House model (slow, 260 s analysis) | PASS (3D-v2 §7, slow) |
| E3D-157 | Example "Concrete Forest" | Example combobox | Concrete Forest model | NOT TESTED (3D-u #16) |
| E3D-158 | Role "Viewer" | Role button "Viewer" | Model only, Results and Inspection removed | PASS (3D-v2 §7) |
| E3D-159 | Role "Editor" after Viewer | Role button "Editor" | Editing UI returns | FAIL (3D-v2 F8, disabled) |
| E3D-160 | Window Close | Close on window | Window removed | PASS (3D-v2 §7) |
| E3D-161 | Window reopen via Display | Drag Display windows handle | Window returns | PASS (3D-u #18, drag handle only) |
| E3D-162 | Window Focus (Model) | Window "Focus" | Other window hidden | PASS (3D-v2 §7) |
| E3D-163 | Top Fullscreen | Top "Fullscreen" button | Browser fullscreen on and off | PASS (3D-v2 §7) |
| E3D-164 | Settings panel | Settings | Driver, General, App sections | PASS (3D-v2 §7) |
| E3D-165 | Language EN to DE and back | Settings > Language | Whole UI switches | PASS (3D-v2 §7) |
| E3D-166 | Theme / Appearance | Settings > Driver > Appearance, System, Dark | UI dark, then restored | PASS (3D-u #17) |
| E3D-167 | Per-window view state (camera, gumball flags, result settings) | Switch windows | State kept per window | NOT TESTED |
| E3D-168 | Command menu tabs (Layout, App, Appearance, Language) | "Command" button | Tabs shown | PASS (3D-v2 §4) |
| E3D-169 | Command > Set Locale form | Command > Language > "Set Locale" | Locale picker usable | FAIL (3D-v2 F6, zero-size input) |

Import and export formats (3D editor): `.semio-archive` (E3D-118 export, E3D-119 and E3D-120 import). The `🚪️io` folder holds internal protocol grammars; no user control was found for them.

Examples (3D): `🌲️concrete-forest` ("Concrete Forest"), `🎬️demo` ("Demo"), `🏠️house` ("House"). See E3D-154 to E3D-157.

## 2D Viewer

| ID | Feature | Trigger in UI | Expected effect | Last browser result |
|---|---|---|---|---|
| V2D-01 | Viewer role layout | Role button "Viewer" | Model window only, editor chrome removed | PASS (2D-v2 E2) |
| V2D-02 | Return to Editor | Role button "Editor" | Editing UI back | PASS (2D-v2 E2) |
| V2D-03 | Declares no actions or utilities | Viewer role | No Actions palette | PASS (2D-v2 E2, palette removed) |
| V2D-04 | Read-only undeformed structure | Model window in viewer | Structure drawn, no edits | NOT TESTED |
| V2D-05 | Mesh-edge preview overlay | Viewer with mesh | Muted edges drawn | NOT TESTED |
| V2D-06 | Viewer camera pan and zoom | Wheel and drag in Viewer | View moves | NOT TESTED |
| V2D-07 | Editor-only windows absent | Viewer role | Results and Inspection not shown | NOT TESTED |

## 3D Viewer

| ID | Feature | Trigger in UI | Expected effect | Last browser result |
|---|---|---|---|---|
| V3D-01 | Viewer role layout | Role button "Viewer" | Model only, Results and Inspection removed | PASS (3D-v2 §7) |
| V3D-02 | Return to Editor | Role button "Editor" | Editing UI back | FAIL (3D-v2 F8, Editor disabled) |
| V3D-03 | Declares no actions or utilities | Viewer role | No Actions pane | NOT TESTED |
| V3D-04 | Read-only Model window render (meshes, supports, loads) | Viewer role | Scene drawn, no edits | NOT TESTED |
| V3D-05 | Viewer camera orbit, pan and zoom | Mouse in Viewer | Camera moves | NOT TESTED |
| V3D-06 | Viewer live results captions | Viewer with analysis | Caption shown | NOT TESTED |

## Counts

| App | Total | PASS | FAIL | NOT TESTED |
|---|---|---|---|---|
| 2D Editor | 138 | 69 | 28 | 41 |
| 3D Editor | 169 | 105 | 26 | 38 |
| 2D Viewer | 7 | 3 | 0 | 4 |
| 3D Viewer | 6 | 1 | 1 | 4 |

Notes on the matrices behind these counts:
- 2D-v2 and 3D-v2 record "Viewer" role checks through the editor. No standalone viewer app was run in the browser.
- The 3D-u rows ("Playback readouts", "Set Result Display") and 3D-v2 rows overlap for Play, Step + and Mode. Each row above uses the most recent result.
