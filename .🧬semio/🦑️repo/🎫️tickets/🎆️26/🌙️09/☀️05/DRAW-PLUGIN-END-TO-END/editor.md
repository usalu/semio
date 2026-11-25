# Draw Editor Verification

## Fixes

- Removed eight byte-identical duplicate package entries from the Draw artifact `Cargo.lock`. Cargo previously refused to parse it. Repo Cargo metadata completed successfully with offline and locked arguments, and the dependency preparation selected zero version updates.
- The browser keyboard path now sends the paste action and target window through the clipboard queue. The dispatch funnel reads the fragment when the queued action gets its turn. This prevents a held fragment from overriding the selection of a newer copy or cut. The fixture starts with a prior selection and checks both subsequent writes and pastes in submission order.
- The native Demo-archive law now observes both effects returned by the dispatch and those emitted by subsequent work. It uses the shared fixture settlement and close procedures, so result-page loans are retired before acknowledgement and terminal completion is drained. It asserts that exactly one load is published.
- Draw's `setActiveExample` now clears both its layer and point selections via the local interaction lane as well as publishing the requested file load. The reported zero layers with one selection came from the current guest interaction view, while the archive loader replaced the document store. Both renderer routes submit the registered Draw command, so the clear applies across the normal example switches. The command law covers registered examples and the empty drawing.
- Draw's native document and document member catalogs now declare the same whole-document conflict target as its existing app operation route. The catalog also installs the original edit message producer and the artifact's persisted operation receiver so restored and newly created stores can use the same native history wire.
- Updated the route completeness test with the existing export command's publication contracts and replaced the duplicate retained limits in the browser-grant layer test with the mounted component policy. The inspector event binding fixture now retires materialized built nodes separately from UI values.
- The vector port test drives mounted scene preparation and reads it with the current fixture render identity. A command-only view cannot address the mounted scene.

## Checks

- Renderer window action panel, 22 passed using the owner Vitest configuration.
- Draw publication authority passed after the selection-lane change, with 69 Ajv cases, 38 routes, five hostile mutations, and the Scheduler oracle.
- The earlier English port 6064 journey recorded 3,024 boot pixels and no logged errors, then timed out waiting for the pasted layer. Its copy step had only submitted the shortcut. The coordinator's trace confirmed that the copy completed with no clipboard effect, and paste entered without a fragment.
- The canonical English browser report verified zero selection after both the Blank and Demo switches. Selecting the restored layer and running copy, paste, and cut produced the expected layer counts of one, two, and one. The subsequent rectangle, undo, redo, and persisted revision were observed without browser errors.
- The complete Draw JavaScript owner script passed. Its 15 Bun test invocations counted 715 passes, and the strict TypeScript, image admission and publication checks completed.
- The current WASI build compiled the Draw command crate containing the example-selection fix. The linked guest is still being verified separately.
- The repo-managed native test stopped making compiler progress after 312 units, with all 18 Cargo wrappers asleep and no active fleet action for that build. The local-only retry waited at memory admission. Pinned native Cargo built the three suites with one job and a private target in the ticket, with the repaired lockfile enforced offline.
- The exact Demo archive law passed with the registered fixture context. The initial native run reported one inspector failure and one original export-read cleanup failure. The full editor rerun includes the fixture, history and shared document loader changes. The host suite's first compile failed on one ambiguous fixture conversion, which the publication owner corrected before the rerun.
- The native retest combines the three editor suites and eight sibling owners in one locked, offline Cargo build. It uses the pinned toolchain, a serial compiler frontend, and the standard 128 MiB test stack. The host capacity warden terminated the intervening nonserial exact export build.

## Reproduction

- Run `repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p semio-s-artifact-draw-drawing-editor-tests -p semio-s-artifact-draw-drawing-editor-tests-2 -p semio-s-artifact-draw-drawing-editor-interaction-tests` from the Draw artifact workspace.
- The normal Draw JS target waited in the repository graph scanner. The ticket's `draw-native-dev.ts` launcher invokes the same declared package `test` script with its issued repo process envelope and uses the same route for the renderer's `test long window-action-panel`.
