# Atomic Multi-Point Editing

## Behavior Implemented

The existing semantic `editPath` command accepts a schema-defined `translate` operation containing typed `{index, point}` references and one path-local delta. Rust and TypeScript implementations compute the union of selected coordinates and attached tangents before changing geometry. Duplicate references and overlapping anchor/handle selections therefore move each coordinate once. Quadratic controls shared by two selected anchors also move once. Contour boundaries are respected; arc radii, rotation and flags remain unchanged.

Validation occurs against the source geometry, and output is separate from the source. Missing indices, nonexistent handles, close-segment anchors, empty selection, invalid deltas and nonfinite results are refused. A late invalid target or overflowing coordinate cannot partially update the document. The existing path geometry mutation supplies one undoable document edit.

This is the semantic foundation for multiple-point canvas dragging and keyboard nudging. It does **not** yet expose persistent point selection in the canvas. The broader editor goal remains active.

## Test Evidence

- Added the JSON schema branch before implementation and ten language-neutral cases covering cubic tangents, duplicate handles, shared quadratic controls, interior anchors, disjoint contours, arcs and invalid edits.
- Ran `bun nx run @semio-tech/draw-js:test --excludeTaskDependencies` before implementation: 113 passed, one failed with `Missing path node` on the new translation case. Log: `🗑️generated/tests-point-translation-red.txt`.
- Ran the same target after both implementations were authored: 114 passed, 18,174 assertions across 22 files, plus the existing 44-case Ajv field-patch oracle. Log: `🗑️generated/tests-point-translation-current.txt`.
- The new tests compare authored expected geometry with independent Three.js `Vector2` translations, validate the command with Ajv, verify selection order independence, inverse translation and source preservation.
- Native tests include the same fixture cases, first-party value roundtrips, command argument decoding, DSL text/binary roundtrips and one-step undo/redo through the real artifact wrapper. Full native run **71498** is active; no native pass claimed yet. Log: `🗑️generated/tests-native-point-translation.txt`.

## Selection Integration Findings

Current retained node picking identifies only the node being dragged. `DrawingInteractionSnapshot` retains layer IDs, while the framework owns the `strokes` domain. Node overlays currently distinguish anchors and controls but do not indicate persistent point selection.

Point selection must use a framework interaction domain and read the request-owned interaction view. Adding a second private selection vector would create conflicting authorities. Point references also need an explicit invalidation/remapping rule: a plain segment index can silently refer to a different anchor after insertion, deletion, reversal or a concurrent geometry edit. This must be resolved before the new command is connected to persistent selection. The current atomic translation references address the source geometry for one command only and do not claim durable identity.

Remaining integration: reference identity/invalidation, point domain topology, click/modified-click and marquee semantics, selected-point rendering, multi-point preview, keyboard nudge/delete, cancellation, concurrency and bounded-work ownership.

## Other Validation Progress

- Prior describe/materialize run **79298** finished successfully (61m 13s). Its component preceded the inspector row edits.
- Prior native node-row run **73342** finished successfully: one test passed, 379 skipped (50m 32s total, 0.434s test execution). It does not validate the gradient-row test.
- Fresh inspector component build **93519** remains active. Log: `🗑️generated/build-draw-inspector-rows.txt`. After completion, explicitly activate the component and verify cap/join values plus node/gradient row layout in the browser.
- Stable preview **76268**, browser 1/tab 5 at `http://127.0.0.1:6065/`, remains available. Current UI still uses the older component; no refreshed-row browser verification claimed.

## Files Changed In This Checkpoint

- Geometry editing `🧬️schema/🔣️.json`, `🦀️.rs`, `🟦️.ts`.
- New geometry editing fixture `🧫️fixtures/↔️points/🔣️.json`.
- Geometry editing Rust/TypeScript unit tests.
- Editor argument-bridge tests and artifact history test.
- This note and the ticket acceptance ledger.

## Additional Browser Findings

Opening Actions while Orange Wedge is selected shows `1 layer · 0 selected`; the inspector simultaneously displays the selected layer's coordinates. Source inspection confirms `window_engagements` counts only root layers and hardcodes zero selected; `window_engagements_with_request_context` ignores its interaction view. The selection status must read the framework-owned domain, and the status/input placeholder also need localization. This is an observed remaining usability defect, not yet repaired.

The public Edit Path action currently presents free-text Path and Node Edit fields. The inspector provides the usual node operations, but this generic action form alone is not a usable multiple-point editor. Canvas/keyboard integration remains mandatory.

## Activation Check

Activation run 63816 returned a cached Nx result. Since activation changes the runtime receipt, that output did not establish a fresh receipt publication. Reran the same registered target with `--skip-nx-cache` (run 95840): terminal success, one completed component activated with a changed receipt, 7.2s total. Log: `🗑️generated/activate-stroke-enums-uncached.txt`. Use uncached activation when explicitly publishing an already materialized component. The newer inspector build is still separate and must be activated after it completes.

## Stroke Browser Result

The completed earlier component is now activated and its cap/join controls are verified in the actual browser, including Round/Bevel changes and separate one-step undos. Temporary projection logs showed `butt, miter, round, bevel, miter, butt`; browser errors were empty. The temporary source diagnostic was removed. See [stroke validation](🖊️stroke-enums.md). The inspector-row component and native translation suite remain active.
