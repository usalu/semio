# FEM 2D Editor Feature Inventory

Source: Haiku read-only audit of `◻️2d/.../✏️editor` (Rust; the TS package is a re-export). English labels from `🗣️terminology/🦀️.rs`, German in `native_de`.

## Shell and Windows
- Edit mode "Edit"; windows "Model" (Structure view) and "Results" (Results view), 0.5/0.5 (`🚀️ui-helpers-and-app-assembly/🦀️.rs:40-47`).
- Panels: "Artifact" (workbench tree), "Inspection" (details), "Results" (display). Example combobox "Example" (Demo). Surface role Editor/Viewer.
- Language en/de via shell.

## Utilities (`:214-218`)
- Direct Select (`selectDirect`), Marquee Select (`selectMarquee`, <4 px = click), Lasso Select (`selectLasso`), Pan (`transformMove`, host side, `canvas-gesture/🦀️.rs:322`), Transform (`transform`, gumball when selection non-empty).

## Transform Options (`🛠️options/🔄️transform/🦀️.rs:44-68`)
- Toggles Move, Rotate, Scale Axes, Uniform Scale → `setTransformGumballFlag`.

## Selection (`🕹️interaction/🦀️.rs:349`)
- Click pick (hit radii node 8, support 11, load 6, element 6 px), Shift additive, Ctrl/Cmd subtractive, Shift+Ctrl invertive, marquee/lasso with same merges, hover highlight, empty click clears, artifact tree row click selects, inspection load-pick selects, multi-select shows "N Selected", Delete/Backspace and inspection "Delete" → `removeSelection`.

## Gumball (`🕹️interaction/🧭️gumball/🦀️.rs:99-142`)
- Translate → `translateSelection`, rotate → `rotateSelection`, scale → `scaleSelection`; pointer cancel leaves selection. No numeric entry or snapping in editor code.

## View
- Pan host side; wheel zoom host side via `setCamera` (Chrome audience, `:284`); `setCamera` rejects zoom ≤ 0; Inspection "Focus" → `focusEntity` (node, element, region, support, load).

## Actions (palette)
- Add Node, Add Bar, Add Beam, Add Material, Add Section, Add Support, Add Nodal Load, Add Member UDL, Add Area Load, Add Region, Add Load Case, Add Combination, Set Self Weight.

## Inspection (`📌️panels/🔍️inspection/🦀️.rs`)
- Node X/Y (`patchNode`); Element Kind/Start/End/Material/Section (`patchElement`); Material Name/Young's Modulus/Poisson Ratio slider/Density (`patchMaterial`); Section Name/Area/Second Moment of Area (`patchSection`); Support Node/Tx/Ty/Rz (`patchSupport`); Region Name/Thickness slider/Mesh Size/Material (`patchRegion`); Loads nodal/UDL/area (`patchLoad`); Load Case Name/Self Weight (`patchLoadCase`); Combination Name, term factors (0 removes), "Add Term" select (`patchCombination`); empty selection shows summary counts and analysis rows.

## Artifact Tree (`📌️panels/🗿️artifact/🦀️.rs`)
- Sections Nodes, Elements, Regions, Supports, Load Cases, Combinations, Materials, Sections, Analysis with counts; row click selects.

## Results (`📌️panels/📊️results/🦀️.rs`)
- Display Source/Mode (Static, Modal, Buckling)/Mode Index → `setResultDisplay`; Playback Phase slider, Step −, Play/Pause, Step +, Speed, Loop (Loop, Ping-Pong, Once), Waveform (Ramp, Sine) → `setResultAnimation`; Analysis Modal Count, Buckling Count, Deformation Scale → `setAnalysisSettings`.

## Keyboard
- mod+z / mod+shift+z (manifest `:248-249`, host undo/redo), Space play/pause, Delete/Backspace remove.

## Unwired Suspects
- `setSelfWeight` duplicates the inspection `patchLoadCase selfWeight` path (`:125-126`).
- Add actions only reachable through the action palette.
- `removeTerm` in `🩹️patch-combination/🦀️.rs:61` has no UI sender.
- Undo/redo depend on the host.
- Presence is `NoPresence` (`🎛️play-app-editor/🦀️.rs:32`).
