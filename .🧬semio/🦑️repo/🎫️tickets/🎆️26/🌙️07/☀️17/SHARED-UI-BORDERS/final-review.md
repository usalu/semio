# Final Independent Review Follow-up

## React Dock
Reviewer found semantic text inputs beside invisible resize hit regions were still intercepted. Lead extended the control-ownership predicate to textbox/combobox/spinbutton/checkbox/radio/switch roles and all editable attribute forms except explicit false. Pointer regressions cover textbox, combobox, empty contenteditable and plaintext-only. The final 713-test integration run includes these cases.

## Native Dock
Reviewer found notch baseline paint reversed its inward direction when a cap existed. Lead changed baseline paint to the same inward direction as its corresponding top/bottom boundary. Added top/bottom chip/body containment assertions alongside the existing plain-rectangle inward test. The full Rust integration test remains unexecuted due the repository source-capture path-length blocker, not claimed passing. The dependency-free native kernel checks passed before this one-line integration correction; review found no other concrete native issues.

## Regression Assertions
Updated obsolete tests that required double-shadow seams, broad child-border erasure, physical gutters, and clipping on the menu's silhouette host. Kept assertions for focus rings, active outlines, actual mounted corner resizing and portal ownership. Added a real zero-measurement menu fallback: retain authored pointer anchoring until dimensions are available rather than clamping a zero-sized jsdom/hidden rect to the origin.

## Final Browser Evidence
Chromium 156 with actual compiled canonical CSS and real controls verifies protected Select overlay sizing (frame approximately 77.375×22.391, paint 77×22), shell-owned reachable 400×160 popup over neighboring content, Select shell portal ownership and dismissal, hover precedence and system Highlight under forced colors plus reduced motion. Normal and forced-color screenshots were reviewed before generated-output cleanup. No claims about native GPU execution are made.
