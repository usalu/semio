# FEM 3D Editor: User-Facing Feature Inventory

Scope: the `✏️editor` subset of the FEM 3D artifact
`C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d\🏅️standards\🔖️1\🪆️subsets\🌐️any\✏️editor`
(Rust), plus the TypeScript schema, test and example files under the same subset and under
`C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d\📦️packages`. The `👁️viewer` subset is out of scope.

Read-only inventory. No source was changed.

## 0. How the UI is composed (read this first)

- The editor declares its UI in `📋️manifest/🦀️.rs` (actions, labels, keybindings, windows, panels,
  utility, interaction domain), renders windows in `🎭️modes/✏️edit/🪟️windows/*`, and renders panels in
  `📌️panels/*`. Labels are one `Fem3dLabels` struct in `🗣️terminology/🦀️.rs` (en + de per field).
- The shell (toolbar, command palette / "action pane", example switcher, utility picker, viewport
  raycasting, marquee, hover, camera orbit/pan/zoom, undo/redo keys, the Space/Delete keys, dock tabs)
  is NOT in this subtree. The FEM code only declares it. `🖥️play-app/🦀️.rs` comments name
  `ShellHost` (action pane and example switcher) and "React/wgpu shells" as the consumers.
- The 3D viewport is a `World3d` surface (canvas). Its DOM text is not produced here. For Playwright,
  assert viewport effects through the Inspection panel values, the Results caption text, and the Artifact
  tree rows, not through canvas pixels.
- Every control in the Inspection and Results panels is a tree/field row whose accessible name is the
  row label (set in `control_row` / `row` helpers). Button labels are the terminology strings below.
- Locale: `view_state.locale` selects `en` or `de`. The English string is listed first. Framework-owned
  strings (panel tab names "Artifact" and "Inspection" in English) are referenced by constant
  (`FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL`, `FRAMEWORK_PANEL_TAB_INSPECTION_LABEL`); their English text was not
  resolved in this tree.
- Keyboard: `mod` in the framework means Ctrl on Windows/Linux and Cmd on macOS (framework convention,
  not verified in this subtree).

## 1. Modes, windows, panels, utility

F01 Edit mode (default and only mode)
- Affordance: mode "Edit" / "Bearbeiten", icon `pencil`. `.mode(edit::MODE_ID, ...)` + `.default_mode_id`.
- Command: none. Effect: app opens in Edit mode. Source: `📋️manifest/🦀️.rs` ~line 33-34.

F02 Model window (World3d, undeformed structure)
- Affordance: window title "Model" / "Modell", surface name "Model view" / "Modellansicht", icon `fem-model`.
- Command: none (render only). Effect: nodes, bars, frames, meshed solids, supports and loads drawn.
  Source: `🧱️model/🦀️.rs` (`model_scene`, `render_with_progress`).

F03 Results window (World3d, results view)
- Affordance: window title "Results" / "Ergebnisse", surface name "Results view" / "Ergebnisansicht", icon `bar-chart-3`.
- Command: none (render only). Effect: the structure deformed or colored per the result display config (see section 8).
  Source: `📊️results/🦀️.rs`.

F04 Default two-window layout
- Affordance: horizontal split 0.5 / 0.5, Model left, Results right (`default_layout`).
- Effect: both windows open side by side. Splitting/closing is shell-owned (not verified here).

F05 Panel tab "Artifact" (outliner)
- Affordance: dock tab, group Workbench, body `fem3d.play.artifact`. Label: framework constant
  `FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL` (English not resolved), German "Artefakt".
- Command: row selection only (see F20-F30). Source: `📌️panels/🗿️artifact/🦀️.rs`.

F06 Panel tab "Inspection"
- Affordance: dock tab, group Details, body `fem3d.play.inspection`. Label: framework constant
  `FRAMEWORK_PANEL_TAB_INSPECTION_LABEL` (English not resolved), German "Inspektion".
- Command: patch/focus/delete commands (sections 5 and 6). Source: `📌️panels/🔍️inspection/🦀️.rs`.

F07 Panel tab "Results"
- Affordance: dock tab, group Display, label "Results" / "Ergebnisse" (native). Body `fem3d.play.results-panel`.
- Commands: `set-result-display`, `set-result-animation`, `set-analysis-settings`. Source: `📌️panels/📊️results/🦀️.rs`.

F08 Transform utility (armed by the shell utility picker)
- Affordance: utility "Transform" / "Transformieren", icon `move-3d`, group "Transform", offered for the Model window.
  Source: `📋️manifest/🦀️.rs` `.utility(...)`, `.window_kind_utilities(...)`.
- Command: none on arming (shell). Effect: when armed the Model window draws the gumball (see F22-F26) and
  shows the options rail (F09-F13). Armed state is read as `utility == "transform"`.

## 2. Transform options rail (Utility Options, shown while Transform is armed)

Group label is empty, default open. Each entry is a toggle, default pressed. Command for all five:
`set-transform-gumball-flag` (module `🎮️commands/🧭️gumball`, handler `set_transform_gumball_flag`).
Effect: the matching gumball handle is shown or hidden in the Model window.
Source: `🛠️options/🔄️transform/🦀️.rs`, config `Fem3dGumballConfig` in `🕹️interaction/🧭️gumball`.

F09 Toggle "Move" / "Verschieben" (icon `move-3d`), flag `move` (alias `moveAxes`). Test id `fem3d-transform-move`.
F10 Toggle "Move Planes" / "Ebenen verschieben" (icon `square`), flag `movePlanes`. Test id `fem3d-transform-move-planes`.
F11 Toggle "Rotate" / "Drehen" (icon `rotate-cw`), flag `rotate`. Test id `fem3d-transform-rotate`.
F12 Toggle "Scale Axes" / "Achsen skalieren" (icon `scaling`), flag `scaleAxes`. Test id `fem3d-transform-scale-axes`.
F13 Toggle "Uniform Scale" / "Uniform skalieren" (icon `expand`), flag `scaleUniform`. Test id `fem3d-transform-scale-uniform`.
- Note: an unknown flag is refused (`fem3d.gumball-flag.unknown`). The gumball is hidden when every flag is off
  (`Fem3dGumballConfig::any`). Flags are per Model window, never saved to the document.

## 3. Gumball / transform on the selection (viewport drag)

The gumball is drawn only when Transform is armed AND the selection moves geometry
(`fem3d_gumball_active`). Materials, sections, load cases and combinations have no geometry, so no gumball.
Drag handles stream `phase: "stream"` ticks into one open transaction (previewed in both windows, the
Results window re-solves live), `phase: "commit"` on release (one undo step), `phase: "abort"` on blur or
lost capture (no trace).

F14 Drag an axis arrow (Move) — command `translate-selection` (dx, dy, dz), route `🧭️gumball`.
- Effect: selected nodes and solids move; attached elements, supports and loads follow. Pivot = centroid.
- Affordance: viewport handle only (no DOM label).

F15 Drag a plane handle (Move Planes) — command `translate-selection`.
F16 Drag a rotation ring (Rotate) — command `rotate-selection` (ax, ay, az, angle), pivot = common centre.
F17 Drag an axis scale handle (Scale Axes) — command `scale-selection` (sx, sy, sz) about common centre.
F18 Drag the uniform scale handle (Scale Uniform) — command `scale-selection`.
F19 Release after a drag commits ONE history step; Ctrl/Cmd+Z reverts the whole gesture (undo, F75).
F20 Numeric entry for transform: NOT in fem3d panels. Only the generic action pane (shell) can send
  `translateSelection` (Dx, Dy, Dz, required), `rotateSelection` (Axis X/Y/Z default 0/0/1, Angle required),
  `scaleSelection` (Sx, Sy, Sz required). Labels: "Translate Selection"/"Auswahl verschieben",
  "Rotate Selection"/"Auswahl drehen", "Scale Selection"/"Auswahl skalieren". Commands take the selection
  when `ids` is empty.

## 4. Selection (viewport, tree, inspector, keys)

Interaction domain `fem3d` (`fem3d_interaction_definition`): granularities Node "Node"/"Knoten",
Element, Solid, Support, Load, Material, Section, Load Case, Combination. Methods: Pick and Rectangle
(marquee). Modes: Single and Multiple. Merges: Replace, Additive, Subtractive, Invertive (modifier keys
mapping is host-owned, not verified here). Hover on channel `pointer`. Command for every row: framework
`interactionSelect` (the FEM code only builds the request; no `🎮️commands` directory owns it).

F21 Click-pick an entity in the Model or Results viewport
- Effect: the entity becomes the selection; the inspector and the tree mark it. Picks in either window share
  one selection. Pick granularity is that of the scene instance (node-keyed scene, `domain_granularity_id = node`).

F22 Marquee (Rectangle) select in the Model viewport: multi-select; inspector shows "N Selected"/"N Ausgewählt".
F23 Hover: the hovered entity is highlighted in tree (`highlighted`) and viewport. Effect only (no command).
F24 Click a row in the Artifact tree (F05) to select the entity; the granularity per row:
  node "circle-dot", element "minus", solid "box", support "anchor", load "arrow-down",
  load case "list", combination "link", material "layers", section "ruler".
  Row labels: node "N1 · (x, y, z)"; element "E1 · Bar N1 → N2" (German "Stab"; frame "Frame"/"Rahmenstab");
  solid "ID · name · N pts · Axis Z" (German "Punkte" as "Pkt."); support "ID · node · Tx Ty"; load "Dz -100 N @ N3"
  style; load case "name · n Loads · Self Weight"; combination "name · 1.35 LC1 + ..."; material "name · E 210 GPa";
  section "name · A m² · Iy m⁴". Empty section shows "(none)"/"(keine)". Section headers "Nodes (n)"/"Knoten",
  "Elements (n)"/"Elemente", "Solids (n)"/"Volumenkörper", "Supports (n)"/"Lager", "Load Cases (n)"/"Lastfälle",
  "Combinations (n)"/"Kombinationen", "Materials (n)"/"Materialien", "Sections (n)"/"Querschnitte", "Analysis"/"Analyse"
  (analysis row is read-only).
F25 Selecting a load case or combination in the tree expands its loads or terms (windowed child lists).
F26 Remove selection with Delete or Backspace (keys bound to `removeSelection`, F40).
F27 Deselect: not defined in fem3d (shell/host behaviour, not verified here).

## 5. Inspection panel: editing the selected entity

Summary (nothing selected): section "Summary"/"Übersicht" with rows "Schema"/"Schema" (FEM_3D_SCHEMA value),
"Nodes"/"Knoten", "Elements"/"Elemente", "Solids"/"Volumenkörper", "Materials"/"Materialien",
"Sections"/"Querschnitte", "Supports"/"Lager", "Load Cases"/"Lastfälle", "Combinations"/"Kombinationen" (read-only
counts). Section "Analysis"/"Analyse": "Modal Count"/"Anzahl Eigenformen", "Buckling Count"/"Anzahl Knickformen",
"Deformation Scale"/"Verformungsfaktor" (read-only, edit in F55-F57).
Every editable control dispatches on `Trigger::Change` with `{field, id, value}` to one `patch-*` command.
Section titles are the entity noun ("Node", "Element", "Solid", "Material", "Section", "Support", "Load",
"Load Case", "Combination"; German "Knoten", "Element", "Volumenkörper", "Material", "Querschnitt", "Lager",
"Last", "Lastfall", "Kombination").

F28 Node: X, Y, Z number inputs (step 0.1). Labels "X","Y","Z". Command `patch-node` (field x|y|z).
  ID row read-only ("ID"/"ID").
F29 Element (bar or frame): Kind select (options "Bar"/"Stab", "Frame"/"Rahmenstab"; value bar|frame);
  Start select, End select (node ids, max 24 options); Material select (names); Section select (names);
  Roll number input (frame only, step 0.05, label "Roll"/"Rollwinkel"). Command `patch-element`
  (fields kind, start, end, materialId, sectionId, roll).
F30 Material: Name text input (label "Name"); Young's Modulus number (label "Young's Modulus"/"Elastizitätsmodul",
  step 1e9); Shear Modulus (label "Shear Modulus"/"Schubmodul", step 1e9); Poisson Ratio slider 0 to 0.49 step 0.01
  (label "Poisson Ratio"/"Querdehnzahl"); Density (label "Density"/"Dichte", step 10). Command `patch-material`
  (name, e, g, nu, rho).
F31 Section: Name text; Area ("Area"/"Fläche", step 0.0001); Second Moment Iy ("Second Moment Iy"/
  "Flächenträgheitsmoment Iy", step 1e-6); Second Moment Iz; Torsion Constant ("Torsion Constant"/"Torsionskonstante",
  step 1e-6). Command `patch-section` (name, area, iy, iz, j).
F32 Support: Node select (field nodeId); six DOF toggles labelled Tx, Ty, Tz, Rx, Ry, Rz (ISO symbols, same in
  every locale). Command `patch-support` (nodeId, tx..rz as booleans).
F33 Solid: Name text; Axis select (X, Y, Z; keys x|y|z); Base (label "Base"/"Basis", baseZ, step 0.1); Height
  ("Height"/"Höhe"); Layers ("Layers"/"Schichten", step 1); Mesh Size ("Mesh Size"/"Netzweite", step 0.1); Material
  select. Read-only rows: Outline ("Outline"/"Umriss", "N pts"/"N Pkt.") and Holes ("Holes"/"Öffnungen", count).
  Command `patch-solid` (name, axis, baseZ, height, layers, meshSize, materialId).
F34 Load, nodal: read-only ID and Case rows; Node select (nodeId); Dof select Tx..Rz (dof); Value number (step 100).
  Command `patch-load` (nodeId, dof, value).
F35 Load, member UDL: read-only ID and Case; Element select (elementId); wx, wy, wz number inputs (step 100).
  Command `patch-load` (elementId, wx, wy, wz).
F36 Load, area: read-only ID and Case; Solid select (solidId); Pressure (step 100). Command `patch-load`.
  Note: the load's case cannot be reassigned in this panel (read-only row).
F37 Load case: ID read-only; Name text (`patch-load-case` name); "Self Weight"/"Eigengewicht" toggle
  (`patch-load-case` selfWeight). Section "Loads"/"Lasten": one pick button per load (label
  "<id> · <kind> · <detail>"), activating it selects that load (interactionSelect, F24).
F38 Combination: ID read-only; Name text (`patch-combination` name); Terms section "Terms"/"Terme":
  one number input per term labelled with the case name (step 0.05; field `term:<caseId>`);
  "Add Term"/"Term hinzufügen" select (options: cases not yet in the combination; field `addTerm`, case id as value);
  "Delete Term"/"Löschen Term" select (options: current terms; field `removeTerm`).
  Note: the Add Term select's own value is always "" (intentional, a one-shot picker).
F39 Focus button (Actions section): "Focus"/"Fokussieren" for node, element, solid, support, load.
  Command `focus-entity` (id) — re-aims the Model orbit at the entity point, keeps its offset (F56).
  Not offered for material, section, load case or combination.
F40 Delete button (Actions section): "Delete"/"Löschen" for every selected entity. Command `remove-selection`
  (ids). Effect: removes the entity; dependents are removed by the delete mutations (verify cascade in a browser).
  Loads are removed as `RemoveLoad`. Multi-selection: the "N Selected"/"N Ausgewählt" section lists all ids, and
  fields below belong to the first.
F41 Actions section title "Actions"/"Aktionen".

## 6. Adding entities and other document actions (shell action pane / command palette)

These actions are `in_palette` (default). The fem3d code has NO dedicated button or panel control for them.
They are reachable only through the shell's generic action pane, which builds a form from the `action_args`
declared in the manifest. Labels are `LocalizedLabel::native` English / German.

F42 Add Node / "Knoten hinzufügen": args X, Y, Z (required numbers). Command `add-node`. Effect: a node is
  appended and appears in the Nodes section.
F43 Add Bar / "Stab hinzufügen": command `add-bar` (start, end, materialId, sectionId). NO args declared in
  the manifest (see U1). Not usable from the generic form.
F44 Add Frame / "Rahmen hinzufügen": command `add-frame` (start, end, materialId, sectionId, roll). NO args
  declared (U1).
F45 Add Material / "Material hinzufügen": command `add-material` (name, e, g). NO args declared (U1).
F46 Add Section / "Querschnitt hinzufügen": command `add-section` (name, area, iy, iz, j). NO args declared (U1).
F47 Add Support / "Lager hinzufügen": command `add-support` (nodeId, fixed). NO args declared (U1).
F48 Add Nodal Load / "Knotenlast hinzufügen": command `add-nodal-load`. Manifest declares only `caseId`
  (U2) so nodeId, dof and value cannot be supplied.
F49 Add Member UDL / "Streckenlast hinzufügen": command `add-member-udl`. Manifest declares only `caseId` (U2).
F50 Add Area Load / "Flächenlast hinzufügen": args solidId (required), pressure (required), caseId. Command
  `add-area-load`.
F51 Add Solid / "Volumenkörper hinzufügen": args X, Y, Width, Depth, Height (required), Material (required),
  Base Z (default 0), Layers (default 1), Mesh Size (default 0.5), Axis (x|y|z, default z). Command `add-solid`.
F52 Add Load Case / "Lastfall hinzufügen": args Name (required), Self Weight toggle (default off). Command
  `add-load-case`.
F53 Add Combination / "Kombination hinzufügen": args Name (required), Terms (required, JSON list of
  [caseId, factor]). Command `add-combination`. Malformed Terms silently do nothing (U5).
F54 Set Self Weight / "Eigengewicht festlegen": args caseId (required), enabled (required). Command
  `set-self-weight`. Also available as the Self Weight toggle (F37).
F55 Set Analysis Settings / "Analyseeinstellungen festlegen": optional modalCount, bucklingCount,
  deformationScale. Command `set-analysis-settings`. Also edited in the Results panel Analysis section (F69-F71).
F56 Focus Entity / "Element fokussieren": in_palette false. Command `focus-entity`, see F39.
F57 Patch Node / Element / Material / Section / Support / Solid / Load / Load Case / Combination
  ("Knoten ändern", "Element ändern", ...): in_palette false, bound only from the Inspection panel
  (F28-F38). Commands `patch-node`, `patch-element`, `patch-material`, `patch-section`, `patch-support`,
  `patch-solid`, `patch-load`, `patch-load-case`, `patch-combination`.
F58 Remove Selection / "Auswahl entfernen": destructive. Args `ids` (list). Keys Delete and Backspace (F40, F26).
  Command `remove-selection`.
F59 Set Active Example / "Aktives Beispiel festlegen": destructive. Arg Example (select): "Concrete Forest"/"Betonwald"
  (`concrete_forest`), "Default"/"Standard" (`demo`), "House"/"Haus" (`house`). Replaces the whole model with the
  bundled example. Command `set-active-example`. Offered by the shell example switcher (see U7).
F60 Set Camera / "Kamera festlegen": audience `Chrome` only. Command `set-camera` (Viewport3dOrbit: position,
  target, zoom, up; validated). There is NO fem3d control for it. Orbit, pan and zoom gestures are host-owned
  (see F61 in section 7).

## 7. Camera (orbit, pan, zoom, focus)

F61 Orbit, pan, zoom in the Model or Results viewport: host-owned gestures. Each window stores its own camera
  (`Fem3dModelWindowConfig.camera`, `Fem3dResultsWindowConfig.camera`). The gesture dispatches `set-camera`
  (chrome audience) with the window's own orbit; it is not a document change.
F62 Focus Entity (F39): re-aims the addressed window's orbit at the entity's point (node position, element
  midpoint, solid centroid, support/nodal-load node, member-load element midpoint, area-load solid top),
  keeping the eye offset and zoom.

## 8. Results panel (Display, Playback, Analysis) and Results window

Display section "Display"/"Anzeige" (command `set-result-display`, fields sourceId, mode, modeIndex, per window):
F63 Source select "Source"/"Quelle": options = every load case then every combination (label = name, value = id;
  max 24). See U4 for a live-window issue.
F64 Mode select "Mode"/"Modus": "Static"/"Statisch" (static), "Modal" (modal), "Buckling"/"Knicken" (buckling).
  Buckling is validated against the chosen source by `set-result-display`.
F65 Mode Index number input "Mode Index"/"Modenindex" (step 1; limited to modal count for modal).

Playback section "Playback"/"Wiedergabe" (command `set-result-animation`, field-based; ResultAnimationTick is the timer):
F66 Hint row "Focus the Results window for live playback readouts"/"Ergebnisfenster fokussieren, um die Wiedergabe live zu sehen"
  (only when no results window is focused; the transport then is a bare play/pause toggle).
F67 Phase slider "Phase" (0..1, step 0.01, field `phase`). Follows the running clock while playing.
F68 Transport row (label "Playing"/"Läuft"): "Step −"/"Schritt −" (back, icon skip-back, field phaseStep negative),
  Play "Play"/"Abspielen" or Pause "Pause" (icon play/pause, field playing true/false), "Step +"/"Schritt +"
  (forward, icon skip-forward). Space bar toggles play/pause (F79).
F69 Speed slider "Speed"/"Geschwindigkeit" (field speed, step 0.05, bounded).
F70 Loop select "Loop"/"Schleife": "Loop"/"Schleife" (loop), "Ping-Pong"/"Ping-Pong" (pingPong), "Once"/"Einmal" (once).
F71 Waveform select "Waveform"/"Wellenform": "Ramp"/"Rampe" (ramp), "Sine"/"Sinus" (sine).

Analysis section "Analysis"/"Analyse" (command `set-analysis-settings`):
F72 Modal Count number input "Modal Count"/"Anzahl Eigenformen" (step 1; field modalCount).
F73 Buckling Count number input "Buckling Count"/"Anzahl Knickformen" (step 1; field bucklingCount).
F74 Deformation Scale number input "Deformation Scale"/"Verformungsfaktor" (step 10; field deformationScale).

Results window captions (DOM text above the scene, from `with_caption`):
- Static: "Case: <case id>" (German "Lastfall"). While playing, appended " · Phase 0.25 · ▶ 1 Hz"
  (phase and speed text). The case id is shown, not its name.
- Modal: "Mode Shape: 1: 12.345 Hz" (German "Eigenform").
- Buckling: "Buckling mode 1: Factor 3.456" (German "Knickform", "Faktor").
- Pending or progress captions while a solve runs: "Generating mesh"/"Netz wird erzeugt",
  "Assembling analysis"/"Berechnung wird aufgebaut", "Solving loads"/"Lasten werden berechnet",
  "Computing modes"/"Eigenformen werden berechnet", "Validating results"/"Ergebnisse werden geprüft",
  "Preparing result view"/"Ergebnisansicht wird vorbereitet", "Analysis queued"/"Berechnung vorgemerkt",
  "Preparing analysis"/"Berechnung wird vorbereitet", "Analysis cancelled"/"Berechnung abgebrochen",
  "Analysis failed"/"Berechnung fehlgeschlagen".
- Errors: "Analysis error"/"Analysefehler: <msg>", "Modal analysis error"/"Fehler der Eigenformanalyse",
  "Buckling analysis error"/"Fehler der Knickanalyse", "No load case defined"/"Kein Lastfall definiert",
  "Result not found"/"Ergebnis nicht gefunden: <case>". A failed solve still shows the undeformed structure.
- Static solids are colored by nodal-averaged von Mises stress (no legend in fem3d code).

## 9. Keyboard shortcuts (manifest `.keybinding`)

F75 Ctrl/Cmd+Z: undo (framework "undo"). Each committed gumball gesture is one step.
F76 Ctrl/Cmd+Shift+Z: redo (framework "redo").
F77 Delete: `removeSelection` (same as F40).
F78 Backspace: `removeSelection`.
F79 Space: `setResultAnimation` with no args, which toggles play/pause of the focused Results window
  (`set_playing(!playing)`). Source: `🎮️commands/⏯️set-result-animation` (fem2d implementation).

## 10. Undo, redo, history

F80 Undo and redo through the shell (keys F75-F76, or shell buttons, not verified here). Every document
  command above is one history step; gumball drags are one step per release.

## 11. Language switching (en / de)

F81 Language: `view_state.locale` (framework-owned switch). All FEM strings are `Fem3dLabels` en/de pairs
  and `LocalizedLabel::native`. Locale is not a fem3d control. Tests should run both locales.
  Labels that are identical in both locales by design are listed in `FEM3D_LABELS_IDENTICAL_BY_DESIGN`
  (schema, element, material, term, id, name, x, y, z, start, wx, wy, wz, modal, phase, pause, ping_pong).

## 12. Customization

F82 Per-window view state: Model and Results cameras, gumball handle toggles, result source, result mode,
  mode index and playback settings are window configuration, persisted per window (not document content).
  `ConfigSpec::default()` — no app-level config knob exists in fem3d.
F83 Result display, playback and gumball flags are per window. Panel bindings address the focused window
  (`windowId` in every control's arguments).

## 13. Examples

F84 Example switch: see F59 (the `setActiveExample` action; `exampleId`). Example labels in the picker:
  "Concrete Forest"/"Betonwald", "Default"/"Standard", "House"/"Haus". The example `📚️examples/🎬️demo-session`
  ("Demo Session"/"Demo-Sitzung") is not in the picker (U7).

## 14. Unwired and suspect items

Checked by reading the declarations and handlers. Each item needs a Playwright check to confirm.

U1 (unreachable from UI) Five mutations are declared without `action_args`, but their commands need payload fields:
  `addBar`, `addFrame`, `addMaterial`, `addSection`, `addSupport` (manifest `📋️manifest/🦀️.rs` lines 49-53).
  The shell form has no fields to send, and no fem3d panel control binds these actions.
  Command payloads: `➖️add-bar/🦀️.rs` (start, end, material_id, section_id), `🖼️add-frame/🦀️.rs` (+ roll),
  `🧱️add-material/🦀️.rs` (name, e, g), `📐️add-section/🦀️.rs` (name, area, iy, iz, j),
  `🛡️add-support/🦀️.rs` (node_id, fixed).

U2 (incomplete args) `addNodalLoad` declares only `caseId` (manifest line 55) but `📍️add-nodal-load` needs
  node_id, dof, value. `addMemberUdl` declares only `caseId` (manifest line 57) but `📏️add-member-udl` needs
  element_id, wx, wy, wz.

U3 (unwired field) `setResultAnimation` accepts `reverse` (fem2d `set-result-animation` apply_field, file
  `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\🏅️standards\🔖️1\🪆️subsets\🌐️any\✏️editor\🎮️commands\⏯️set-result-animation\🦀️.rs`
  line 151), but the fem3d manifest declares no `reverse` arg (manifest line ~136-146) and the Results panel
  renders no reverse control.

U4 (suspect, live-window path) The Results panel Source select offers plain case and combination ids
  (`📌️panels/📊️results/🦀️.rs` `result_sources`, ~line 172), and `set-result-display` stores the raw value with
  no identity check (`👁️set-result-display/🦀️.rs` line 33). The live (operation) render path resolves the
  source with `borrow_result_source_identity`, which expects an encoded identity (`🏷️result-source/🦀️.rs` lines 51-58)
  (`🎭️modes/.../📊️results/🦀️.rs` line 33 `result_request`, and line 332 caption fallback to the FIRST case).
  Expected symptoms when a non-first source is picked in a live window: a fault in the results window, or
  a caption naming the first case. The static (non-live) path accepts plain ids. Verify in a browser.

U5 (silent failure) `add-combination` returns an empty emit on malformed Terms (`🔗️add-combination/🦀️.rs` line 34).
  The user sees no error and no change.

U6 (silent truncation) Node, element, material, section and solid selects are capped at 24 options
  (`📌️panels/🔍️inspection/🦀️.rs` line 90 `SELECT_ITEMS_MAX`, applied at line 191); the Results Source select at 24
  (`📌️panels/📊️results/🦀️.rs` line 160 `RESULT_SOURCE_OPTIONS`). Entries past 24 are dropped with no "+N" row.

U7 (not offered) `📚️examples/🎬️demo-session` (label "Demo Session"/"Demo-Sitzung") is not in the
  `setActiveExample` option list (manifest ~line 104-105). It is only a test/demo module.

U8 (no fem3d control, by design) `setCamera` audience is Chrome (manifest line 234). Orbit, pan and zoom have no
  fem3d button; they are host gestures.

U9 (no fem3d control, by design) `setSelfWeight` and the `addLoadCase` self-weight arg are palette-only; the
  Self Weight toggle in the Inspection panel (`patch-load-case` selfWeight) is the in-panel path.

U10 (no numeric gumball entry) Translate, Rotate and Scale numeric inputs exist only as generic action args
  (F20). The viewport gumball is the only gesture UI in fem3d.

U11 (read-only by design) The Analysis tree row in the Artifact tab (`📌️panels/🗿️artifact/🦀️.rs`) is not
  selectable and not editable. Analysis settings are edited only in the Results panel.

U12 (no reassignment) Load case of a load is read-only in the Inspection panel (`load.case` row); no field moves a
  load to another case.

U13 (no cancel control) Analysis cancellation exists as a stage caption ("Analysis cancelled") but no fem3d
  control triggers cancel. Interactive jobs are "Migrated" in the manifest.

U14 (duplicates, not unwired) `set-self-weight` and `set-analysis-settings` have both a palette action and a
  panel binding. `focus-entity` is bound only from the Inspection panel (in_palette false).

## 15. Command coverage

All 35 `Fem3dCommand` variants (`🦀️.rs` `app_commands!`) map to a manifest action. Commands reachable from a fem3d
panel or window: `patch-*` (9), `remove-selection`, `focus-entity`, `set-result-display`, `set-result-animation`,
`result-animation-tick` (timer), `set-analysis-settings`, `set-transform-gumball-flag`, `translate-selection`,
`rotate-selection`, `scale-selection` (viewport), `set-camera` (viewport). Palette-only: the add-* actions,
`set-self-weight`, `set-active-example`. Palette-only and broken: U1, U2.

## 16. Test-driving hints (Playwright)

- Use `en` and `de` locales for each feature.
- Inspection and Results controls: find by row label (the strings above).
- Transform toggles: test ids `fem3d-transform-*`.
- Results captions and the Artifact row labels are DOM text; use them as assertions for the viewport effects.
- Viewport gestures (gumball drags, marquee, orbit): drive with canvas coordinates; assert via panel values and
  captions. Gesture abort is checked by blurring during a drag.
