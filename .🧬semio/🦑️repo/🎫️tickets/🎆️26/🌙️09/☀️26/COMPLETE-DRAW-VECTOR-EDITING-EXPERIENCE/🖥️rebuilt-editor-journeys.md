# Rebuilt Editor End-User Journeys

This continues the unchanged complete image-editing goal. The preceding cache migration is verified progress (native 576, TS 611); it does not prove the current browser or native UI experience. Current app channel mismatch withholds Draw from the preview catalog. The genuine composition build is running at 20810; preserve its handle through terminal status before materialization/activation. No matching current tab or live preview existed when inspected.

After verified rebuild and activation, inspect the actual current Demo and create an editable document through visible controls. Verify the following against actual published UI state, rendered geometry and captured runtime diagnostics, retaining screenshots and evidence inside this ticket:

- Layers: create/name, ordering, nested groups, hide/lock ancestry, opacity and blend, fill/stroke and gradient editing; selected layer and inspector agree.
- Paths: direct painted picking through holes/concavities, anchor/control selection and drag, point marquee, typed node edits/conversions/deletion, modifiers, keyboard nudge, editable state after undo/redo.
- Selection and transforms: topmost paint, groups and locked descendants, marquee/lasso merge modes, translate/resize/rotate handles, cancellation and one transaction per completed gesture.
- Algorithms: Boolean operands/holes, decoded-image tracing with holes, parameter edits and bake, clear malformed/unsupported input behavior, progress and cancellation during expensive work.
- Images and files: visible image placement and affine sampling; actual import/open and editable SVG round trips; complete PNG/SVG/PDF downloads with current authored appearance and groups; private interaction metadata excluded from exports.
- History and users: no artifact publication while dragging or on cancellation, one undoable edit per committed gesture, source replacement without stale selection publication, shared versus local state and real multiuser acceptance.
- Usability: default utility chrome matches active tool, utility menu fits the viewport, desktop/multidevice controls, English/German and keyboard/accessibility/customization; useful empty/busy/error states.
- Native parity: current native path/affine/image/text rendering and meaningful runtime evidence, beyond state-only native tests.

Existing automated tests and previous browser evidence are located in acceptance.md. Only current mounted evidence can discharge these journeys. Do not call the goal complete without each applicable requirement proven.

## Actual Current React Session

Preview 82877 owns a real Vite listener at 127.0.0.1:6064 (PID 34411); the serve freshness check confirms one component against source and activation receipt. The in-app browser opened the Demo editor and rendered its vector image. A real canvas click at the red frame selected Red Frame and mounted its layer appearance, affine transform, arrangement and path-node inspector. Console error/warn query returned no entries at this point. This verifies current rendered point selection/inspector entry, not every inspector command.

The Utilities palette has a 300-pixel plane while its loose utility row runs beyond 600 pixels: Direct Select and later tools are horizontally clipped on opening. The Drawing row also extends beyond the same plane. All initial utility leaves are unpressed despite the canvas using selectDirect as its domain default. Layers is not exposed as an initially visible window in this session. Screenshots and measured visible DOM evidence are under generated. These usability gaps remain under investigation; no full editing acceptance is claimed.

## Palette Fix And Test-First Evidence

A neutral bounded-palette schema/fixture now covers 150/300/480 pixel panes, up/down directions, disabled tools, exact window/generation dispatch, and EN/DE mutually exclusive selection choices. The first corrected run 60788 failed six disabled-tool cases (11 total); UtilityTree had omitted disabled from its ToggleGroup items. After passing it through and wrapping stacked groups within their pane, 18475 terminated exit 0: 11 passed. Actual browser geometry then confirmed every Drawing/Combine/selection/path tool stays inside the 300-pixel plane; generated/draw-palette-wrapped.png is its screenshot.

Real Rectangle drags committed geometry, selected the created shape and returned to Direct Select. Inspector Name blur persisted Acceptance Rectangle. Convert to Path removed that command and exposed Path Nodes. Selection method/merge options still needed the same wrapping treatment; corrected red run 92370 terminated exit 1 with the two EN/DE cases missing radiogroup semantics (11 others passed). The method and merge controls now have named radio groups and bounded wrapping; verification  is running. Console freshness logs confirm current transform admission; no error/warn entries were observed during these interactions.

Earlier harness runs 40093 and 14316 failed on a wrong fixture import and unsupported wrapper getAllByRole, respectively; neither is behavioral test evidence. Those harness errors were corrected before the reported red runs.

## Selection Options And Layer Entry Corrections

The selection palette final run 42119 terminated exit 0: 13 passed, including independent AJV validation, exact tool dispatch, disabled controls, and EN/DE named radio groups whose merge choice updates the scoped store. This is component semantics evidence; actual final wrapping still needs the reloaded browser.

Correction to the initial layer-entry concern: pressing the existing Artifact workbench tab opens the real layer tree (Semio Emblem children plus both created rectangles). The panel existed behind collapsed chrome; no missing app declaration is inferred. Two actual drags created two separate rectangles. Inspector Name blur renamed the second to Acceptance Rectangle, then Convert to Path retained its name/appearance and mounted an actual Anchor 1 coordinate row. generated/draw-path-and-layers.png records this real session. The default-utility mismatch on first mount remains; a committed shape correctly arms Direct Select in both canvas and chrome.

## Current Browser Selection Acceptance

After reloading current source and explicitly arming Direct Select, the browser exposed named Method/Mode radiogroups and all six radio choices. Read-only DOM measurements confirmed every choice and both groups are wholly inside the actual 300-pixel utility plane. A real Additive press changed its checked state alone; a real Lasso press was then sent to the current window. The final palette screenshot is generated/draw-selection-palette-final.png. Actual final method settlement and final regression handle 55411 still need recording.

Broader regression run 76729 terminated exit 1: 39 passed, one assertion failed, 662 filtered skips. The failure was an existing raw class-order substring for the utility options wrapper. It now checks that exact final options item’s individual classes, including bounded width, rather than adjacency/order. The rerun includes both the 40 utility/ribbon regressions and all 13 palette tests. Filtered skips are not a full suite witness.

Undo was invoked through the actual action rail, but inspector selection changed while panels were opened/closed. No controlled before/after path-undo witness was captured; undo/redo acceptance remains unverified in this browser stage. The full goal stays active.

## Verified Stage Handoff

Final 55411 is terminal exit 0: 64 passed, 651 filtered skips, two files. Lasso click settled both Lasso Select and its method radio. Keyboard ArrowLeft settled Marquee Select/Rectangle, and ArrowRight settled Subtractive. Console error/warn query was empty. All current controls/groups measured inside the 300-pixel pane; generated/draw-selection-keyboard-final.png records the state. The 150/480 widths and EN/DE are neutral/component semantics evidence only; no browser resize/native appearance claim is made.

Preview owner 82877 remains live; final lsof confirms PID 34411 on 6064. All other stage build/test handles are terminal. Next work needs first-mount utility agreement, a controlled history round trip, actual SVG/PNG/PDF file journeys, native parity, and the scene precision/area/fairness limitations in acceptance. Registry admission requires owner description before projection; this stage emitted its actual verified pair but did not repair that zero-touch ordering gap. Full goal remains active.
