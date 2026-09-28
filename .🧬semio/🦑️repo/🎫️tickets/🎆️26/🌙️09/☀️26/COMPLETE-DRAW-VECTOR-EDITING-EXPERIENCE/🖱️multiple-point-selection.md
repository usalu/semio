# Multiple Point Selection and Combined Dragging

The preceding goal turn was progress: it added keyboard commands and verified their TypeScript contracts. This turn extends the actual node gesture workflow and revalidates the live build handles.

## Implemented Behavior

- Plain press on an already selected point retains the selected drag group. Plain press on a different point replaces it.
- Shift-click toggles membership. Control/Command-click adds membership. Modified blank clicks preserve point selection; plain blank clicks clear it while retaining layer selection.
- The framework-owned point domain now declares multiple selection. The retained gesture stores the intended selected IDs as gesture input; normal framework selection publication remains authoritative.
- Preview and release use the same world-displacement path math. Points on differently transformed paths can move together; adjacent handles are translated once.
- Release uses the same atomic planner as keyboard movement, rebinding every surviving point reference. Cancellation leaves geometry and selected IDs unchanged.
- Removed the single-segment preview representation; there is no compatibility adapter.

## Tests and Limits

Nine neutral pick cases have a schema, Rust/TypeScript implementations, Ajv checks, and Immer output comparison. A native integration scenario covers two paths at different scales, repeated Shift toggles, modified blank clicks, combined preview, release/cancel, rebinding, undo/redo and final blank-click layer preservation. Native tests are authored; no pass is claimed yet.

A new neutral affine case reproduced overflow caused by including a huge translation origin in direction inversion. Both point-displacement implementations now invert only the affine basis and independently validate every matrix value. TypeScript regression evidence is in `🗑️generated/tests-point-origin-red.txt` and `🗑️generated/tests-point-workflows-current.txt`.

The synchronous movement planner still has its explicit 4,096-item admission limit. The gesture has bounded point-reference/byte capacity. Large-document resumable preparation, marquee/lasso point selection, deletion, and broader collaboration/history behavior remain acceptance gaps.

## Native and Component Runs

Native session **71498** terminated with exit 1 after 51m 2s: the compiled test could not resolve `blake3`. The current manifest already contains the development dependency, added after that process began. No source compile error besides the missing test dependency was reported by that run. Fresh native session **78443** was launched after termination against the current manifest and point-workflow sources; log `🗑️generated/tests-native-point-workflows.txt`.

Component session **93519** remains live. Do not restart it. Inspect the completed component/descriptor for current source coverage before activation. Stable preview session **76268** remains live.

## Independent Browser Export Check

The existing preview exposes SVG in a form misleadingly labelled Export PDF. The source label is now Export Drawing / Zeichnung exportieren. The first SVG Execute attempt produced no observed download within 15 seconds and no console error. This is not yet evidence of successful export or a proven application failure; scoped host diagnostics are being used to distinguish effect delivery from browser download handling.

The diagnostic replay confirmed `[DEBUG] Draw export delivery semio.svg image/svg+xml 1423` and a DOM download anchor named `semio.svg` pointing to a blob URL. The in-app browser emitted no completed download. Its documented `downloadMedia` fallback refused the hidden anchor. No matching file appeared in the normal Downloads folder. This proves effect delivery and link creation, not file delivery. The temporary host log was removed from source. Evidence is in `🗑️generated/export-delivery-trace.json` and `🗑️generated/export-form-current.png`; the source label correction awaits a rebuilt descriptor.

## Final TypeScript Evidence

`bun nx run @semio-tech/draw-js:test --excludeTaskDependencies` passed **121 tests, 18,318 assertions**, plus 44 field-patch oracle cases and the 36-command publication audit. Session **11325** exited 0; authoritative log `🗑️generated/tests-point-workflows-current.txt`. Native **78443** and component **93519** were polled and confirmed live after this result. No native or rebuilt-browser pass is claimed for this stage.

## Native Follow-Up

The fresh native run (`tests-native-point-workflows.txt`) compiled and ran 99 of 396 tests before fail-fast: 96 passed; two new nudge tests counted pending selection operations in their nudge receipt, and the combined-drag test queried point selection before the initial layer selection publication had settled. Each new fixture now completes the framework's typed publication/acknowledgment protocol after its reserved selection admission. A full `--no-fail-fast` rerun is in progress (`tests-native-selection-settled.txt`). This is not yet evidence that the changed workflows pass.

The rerun 9208 terminated before Draw compilation because `semio-framework-os-kernel` has twelve current compilation errors: incompatible retained-close trait methods, mismatched values, missing `RetainedCloneClose::push`, and a lifetime error. The same initial-selection publication fix also applies to the existing single-node drag and point-click fixtures, and is now present there. These native changes remain unverified. No unrelated kernel source was changed.
