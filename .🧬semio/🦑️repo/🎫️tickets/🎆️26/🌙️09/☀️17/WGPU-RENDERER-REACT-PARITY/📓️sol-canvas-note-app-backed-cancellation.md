# Canvas2d Draw and InkCanvas Note Cancellation Acceptance

## Accepted Policy

The actual React applications remain authoritative.

- Draw publishes one terminal `canvasPointerUp` receipt with `cancelled: true`. The unfinished drag changes neither the artifact nor its preview. A stale move, up, or duplicate cancel from that contact is inert. The next down starts a fresh gesture.
- Note persists accepted `inkApplyEvents` begin and live operations. Cancellation clears the local gesture and preview without publishing rollback or commit. A stale move, up, or duplicate cancel publishes nothing. The next down starts a fresh begin/live/commit sequence.

The schema-first `surface-behavior@1` fixture expresses these distinct terminal policies. Its strict Ajv oracle and the actual React Canvas2d and InkCanvas hosts use the same fixture. No Note rollback domain protocol was added.

## Fail-First Result and Production Repair

The mounted Canvas2d test was strengthened to send a move and pointer-up after pointer cancellation. It failed before the production edit: the host had three expected actions at cancellation but received five because it published both stale events.

`JsonLayersCanvasSession` now records that an active gesture was cancelled and rejects subsequent move/up input until the next down. Down clears the fence before normal admission. Cancellation without an active gesture does not arm the fence, and the existing pan terminal runs before the fence, preserving hover and pan behavior. The change adds no public API.

The sample-lane law independently requires the same cancelled-contact fence and proves that a new down is admitted after it.

## Actual Application Laws

The existing Draw `direct_drag_projects_without_editing_and_publishes_only_on_release` law now drives the real `DrawingApp` beyond the cancelled terminal. It sends a stale move and ordinary up, requires both to omit the Artifact result lane, and requires the document snapshot to remain byte-for-byte equal. A fresh down/move/up then publishes exactly one artifact result and moves the selected layer by `(30, 20)`.

The existing Note `cancellation_retains_accepted_begin_live_without_a_terminal_app_operation` law continues to drive the real `NoteApp`. It requires the accepted pre-cancel block at `(48, 52)`, asserts that live and commit are the fixture's blocked stale phases, then admits a fresh begin/live/commit and requires a second persisted block at `(88, 96)`. Cancellation therefore retains accepted content without inventing a rollback, while the next gesture remains usable.

These application laws are source-complete. The Draw law was launched, but its crate did not compile and the test never entered an assertion. No passing Rust result is claimed here.

## Validation Receipts

### Initial Contract and Mounted Hosts

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-canvas-ink/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧪️tests/🎬️surface-behavior/🟦️.ts' '../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx' '../../../../🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx' --silent=false --reporter=verbose --testNamePattern='validates exact Draw and Note cancellation policies|matches the Draw terminal receipt, stale continuation, and fresh-gesture policy|retains accepted Note begin/live events while cancellation blocks stale continuation'
```

Result: 3 files passed, 3 tests passed, 43 skipped; Vitest 38.64 seconds, Nx 46.0 seconds.

### Fail-First Stale Continuation

The strengthened mounted Draw law failed with `expected 3, received 5`. The two unexpected actions were the post-cancel move and up. This is the direct failing receipt that justified the session fence.

### Post-Repair Sample Lane and Mounted Host

The focused sample-lane and mounted-host command passed 2 files and 2 tests with 40 skipped. Vitest completed in 111.12 seconds and Nx in 2 minutes 7 seconds.

### Final React Receipt

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/sol-canvas-ink/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧪️tests/🎬️surface-behavior/🟦️.ts' '../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️gesture-sample-lane/🟦️.ts' '../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx' '../../../../🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx' --silent=false --reporter=verbose --testNamePattern='validates exact Draw and Note cancellation policies|turns a cancel mid-gesture|matches the Draw terminal receipt|retains accepted Note begin/live events while cancellation blocks stale continuation'
```

Result: 4 files passed, 4 tests passed, 47 skipped out of 51; Vitest 62.29 seconds, Nx 1 minute 11 seconds. The only diagnostics were the existing multiple-Three.js-instance warnings.

After the root full-suite snapshot reported the delayed modifier law red, the exact law passed on current source, and the complete gesture sample-lane file then passed 6 of 6 tests. Vitest took 13.62 seconds and Nx took 16.0 seconds. No additional Canvas production edit was needed: `pointerSample` already copies shift/control/meta/alt at sample admission, so later mutation of the browser-event carrier cannot change an owed move.

## Current Limits

The first WGPU attempt passed Cargo's test router an unsupported `--exact` argument and ran no test. A second focused attempt entered the shared native compilation and was interrupted after coordination identified the same `FromValue` derive failure already owned by the root integration run. It supplies no WGPU receipt.

The focused Draw application command ran for 118 minutes 8 seconds and then failed while compiling `semio-s-artifact-draw-drawing`, before the requested application law entered any assertion. Cargo reported E0433 at the SVG serializer's line 24 (`use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;`) and drawing IO facade line 137 (`export::svg::v1_1::any::drawing_document_to_svg(doc)`). This is compile-not-assertion evidence and does not contradict the cancellation policy. The exact diagnostics were sent to the stdio/Draw IO owner; this packet does not alter that ownership.

The focused Note application command ran for 15 minutes 44 seconds and also stopped before its requested assertion. `semio-s-artifact-stdio-contract` failed with E0433 at `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:1185:40`: the bound `S: dsl::ToValue` could not resolve `dsl`. The exact Cargo fingerprint is `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-stdio-contract/82a6c37f08e44b02/fingerprint/output-lib-semio_s_artifact_stdio_contract`; it was sent to the stdio owner. This is compile-not-assertion evidence.

This packet has no paired physical React/native/WASM application boot. The passing evidence covers the strict neutral fixture, actual mounted React hosts, production session fence, and sample lane. The strengthened actual Draw and Note application laws still require a native execution receipt. Root owns the full UI, renderer, native, WASM, and paired-shell integration gates.
