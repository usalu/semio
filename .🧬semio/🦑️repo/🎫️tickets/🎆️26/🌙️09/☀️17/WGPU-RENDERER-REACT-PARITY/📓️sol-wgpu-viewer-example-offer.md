# Viewer Catalogue Offer Authority

The actual WGPU stdio WAV viewer boot failed when the shell dispatched `setActiveExample` for `?example=demo`. Its viewer descriptor declares neither an app nor a window action with that id. The strict plugin refusal is correct. The editor descriptor does declare the action; treating its declaration as viewer authority would be wrong.

React's canonical `ShellHelpers::appOffersRegisteredExamples` accepts any editor with a nonempty matching catalogue, but accepts a viewer only when its app actions or one window kind declares `setActiveExample`. `ShellHost` applies this predicate before computing options and treats empty options as an immediately ready initial document, with no example mutation. WGPU's `shell_example_rows` and `sync_session_chrome` currently filter only by dialect, so they incorrectly offer and announce the editor catalogue on an undeclared viewer.

The new Shell example-offer admission schema and six neutral vectors cover an editor, empty editor catalogue, undeclared viewer, app-declared viewer, window-declared viewer, and an empty declared viewer catalogue. The React helper and independent strict Ajv schema oracle passed the same six vectors through the existing React Nx test route (one selected law, 2705 filtered). The first schema attempt failed strict nested-type validation; explicit integer/array types repaired the authored schema. Logs: `🗑️generated/sol-2026-09-29/viewer-example-offer-browser.log` and `viewer-example-offer-browser-2.log`.

The native law `viewer_example_offer_matches_declared_action_authority` is authored in the existing WGPU chrome suite. Production repair remains pending its coordinated native fail-first execution. The intended repair applies the same predicate to matching option resolution, rows, boot/session announcement, and concrete picker selection. Generic plugin action admission remains strict.

The first coordinated native union did not execute the new law: its frame-worker generation prerequisite refused a newly reached `io/sqlite-snapshot` browser module that had no source ownership declaration. The parent integration lane owns that closure repair. A previous theme-only union had unrelated kernel re-export compile errors. Neither attempt is recorded as a native viewer red/green witness.

## Native Failure and Repair

The subsequent coordinated native run executed the law and failed on `viewer-default-document`: WGPU offered a row (`true`) while the shared authority requires no offer (`false`). The exact failed witness is in `🗑️generated/sol-theme-guidance-viewer-native-red-compiled.txt`.

The repaired Shell helper filters matching catalogue examples through the same role/action authority as React. Both chrome option resolution and painted rows call that helper, so boot and role announcement receive an empty selection for an undeclared viewer. A concrete picker command additionally must match an offered row before it can update selection or dispatch. The dialect-only picker law now explicitly declares viewer example authority where its old fixture is intended to test dialect matching rather than action admission. The generic plugin refusal remains unchanged. Native green is pending the shared focused union; fresh browser execution is owned by the parent and media lane.

## Coordinated Native Receipt

The executed union `🗑️generated/sol-native-theme-viewer-green-display-introduction-red.txt` ran 17 selected laws: 15 passed and two unrelated Display/introduction laws failed as intended. The viewer offer, admitted example picker, boot announcement and heap-first registry laws passed. This confirms the concrete production fixes; it does not establish full renderer parity. No duplicate Cargo process was started by this lane.

The coordinated follow-up `🗑️generated/sol-native-theme-display-introduction-green-graph-repaired.txt` executed 34 laws and passed all 34, including these viewer, admitted picker/boot and heap-first laws again.
