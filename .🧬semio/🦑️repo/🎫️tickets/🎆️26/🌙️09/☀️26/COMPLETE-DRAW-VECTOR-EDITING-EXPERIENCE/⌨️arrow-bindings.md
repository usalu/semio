# Arrow Shortcut Admission and Browser Journeys

## Runtime Reproduction

The current rebuilt component was activated successfully and opened at the stable ticket preview on port 6065. Direct Select selected Orange Wedge. Edit Nodes then picked its top-right anchor: dragging screen (469,225) to (496,238) changed only that corner. Command-Z restored the original four-anchor outline. In Edit Nodes, dragging blank screen (351,214) to (490,249) selected both top anchors. Backspace removed both together, leaving the two bottom anchors and their connecting stroke. One Command-Z restored the original outline. Browser error logs were empty for these gestures. Screenshots are `🗑️generated/node-drag-oct02.png`, `node-marquee-oct02.png` and `node-delete-oct02.png`.

Shift-ArrowRight had no effect despite canvas focus and the two-point selection. Inspection found the manifest declares `shift+right` and the renderer compares exact DOM `KeyboardEvent.key`, which is `ArrowRight`. The wgpu native event mapping also emits `ArrowRight`. All eight Draw nudge bindings now use canonical arrow tokens, with corresponding shared fixture keys and explicit DOM event names. No compatibility aliases were introduced.

## Regression Evidence

An initial Bun test attempt imported the entire React helper into the non-Vite Draw test runner and failed module initialization on a Vite worker URL export. The DOM oracle was moved to the existing renderer input contract suite, where Vite resolves that dependency. This failed attempt is `tests-arrow-bindings-red.txt` and is not treated as evidence of the desired red case.

The proper red run 88691 (`tests-arrow-bindings-source-red.txt`) passed 284 tests and failed the existing Draw manifest binding guard with the corrected shared fixture. After fixing production bindings, run 48137 (`arrow-bindings-green.txt`) passed all 285 Draw tests. Renderer run 53967 (`arrow-bindings-renderer.txt`) passed 44 tests, including independently dispatched Testing Library/jsdom DOM arrow events for all eight bindings and modifier mismatch refusal. Native nudge/history behavior previously passed in the complete 479-test suite; the changed binding manifest will receive fresh native/component execution.

Component describe/materialize run 98128 is live (`arrow-bindings-build.txt`) and must be polled before any new component build. Stable preview handle 75879 remains running with HMR disabled. Browser binding `browser` is browser 2; tab binding `tab` is tab 1, marked for handoff. A temporary `[DEBUG] App shortcut` line is installed in ShellHost to confirm runtime shortcut admission after activation; remove it after verification and reload the preview. Full goal and ticket remain active.


## Completed Verification — 2026-10-02

Component run 98128 exited 0: all ten describe/materialize tasks in 1m20s. Explicit activation 26863 exited 0. The fresh native run 45974 passed all 479 tests, zero skipped (`tests-native-arrow-bindings.txt`). The neutral keyboard fixture now has a separate schema requiring canonical DOM key names and arrow chords. Draw run 8086 passed 285 tests / 157459 assertions after its validation/rejection checks were added (`tests-arrow-bindings-schema.txt`). Renderer test targets explicitly include the imported Draw fixture in their Nx inputs so fixture changes invalidate the test cache.

After reloading the corrected component, the same marquee selected Orange Wedge anchors 3 and 4. Shift-ArrowRight changed their X coordinates from 36.25 and 1.25 to **46.25 and 11.25**. The bottom anchors stayed at 1.25 and 36.25, and the layer Position X stayed zero. The canvas visibly changed only the selected top edge. Console evidence recorded `[DEBUG] App shortcut ArrowRight true nudgeSelectionRightFast` (`arrow-shortcut-console.json`). One Command-Z restored all anchor X coordinates `[1.25,36.25,36.25,1.25]`; the console also recorded the undo dispatch. Screenshot `node-nudge-oct02.png` shows the verified two-anchor edit. This is actual running-browser verification, beyond command-unit tests. The temporary ShellHost debug line was removed and the preview reloaded. No permanent diagnostic was retained.

Folding Layer and Arrange exposes the complete four-anchor numeric inspector. Converting Solid to Linear Gradient rendered the gradient and the Start/End coordinate controls, two color stops, opacity and position controls (`gradient-inspector-oct02.png`). An Add Color Stop attempt was interrupted by component activation resetting the page and is **not** counted as a browser pass. The earlier independent insertion pixel oracle remains the test evidence for that operation.

Stable preview 75879 remains live at 6065 with HMR disabled. Browser 2/tab 1 is retained for continued acceptance work. Current goal progress is substantive: fixed canvas constant paint and dead arrow chords, green suites and verified node drag/marquee/deletion/nudge/undo. PNG production, the larger import/export/algorithm workflow, responsiveness, accessibility and multiuser acceptance are still incomplete. Do not complete the goal or close the ticket.
