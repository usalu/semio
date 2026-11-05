# Layout Fleet Test Audit — 2026-11-04

Read-only review of current manifests and tests. No builds executed; commands below are declared owner routes, not reported passes.

## Highest-Value Gate

Run `repo run @semio-tech/layout-layout-rs:test`. Its package router explicitly includes `semio-s-artifact-layout-layout-editor-transform-tests` first, then editor, model, scene, flow, mutations, diff, SQLite, and export companions. This is the strongest existing end-to-end native behavioral gate. The root TypeScript baseline is separately `repo run @semio-tech/layout-js:test`.

The Rust manifest is `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📋️project.json`; its adjacent script lists the exact Cargo packages. The transform test package is at subset `✏️editor/🧪️tests/🧪️transform-tool-transactions/📦️packages/🦀️rust/Cargo.toml`, with library source two directories above, `🦀️.rs`.

## Existing Native Mounted Laws

Transform tests use the actual registered LayoutPlayApp and original fixture dispatch. Their settle loop drives maintenance, publication, result acknowledgements, history patches, and rendered scene queries. They verify streamed translation previews do not change persisted state; commit yields one parametric mutation and one history transaction; one undo restores the original document; English and German history labels; independent gestures receive independent transaction IDs; abort, blur, lost pointer capture, and retiring leave zero trace. A nested typing-edit test covers previous transaction publication while retaining the latest typing preview.

The editor unit companion declares public registered-app behavioral laws. Export unit companion declares original-caller export jobs, factory dispatch, caps, close ladder, and PDF laws. Reuse these ownership seams for regressions.

## Other Exact Declared Gates

- `repo run @semio-tech/layout-layout-rs:check`
- `repo run @semio-tech/layout-layout-rs:verify-layout-document-contract`
- `repo run @semio-tech/layout-layout-rs:verify-layout-frame-selection`
- `repo run @semio-tech/layout-layout-rs:verify-layout-window-ownership`
- `repo run @semio-tech/layout-layout-rs:test-snapshot-sqlite-native`
- `repo run @semio-tech/layout-layout-rs:test-media-adoption-source`
- `repo run @semio-tech/layout-layout-rs:verify-physical-codecs`

Source gates cannot substitute for behavioral/native execution. The media-adoption TypeScript test reads source trees and checks contract ownership and negative bindings; its plan-storage oracle additionally checks cancellation and offsets.

## Language-Neutral Fixtures and Independent Witnesses

The TypeScript runner registers schema/transport/layer order, mutation validation/structure/graph/frame selection, paragraph/font, frame planning, export fidelity and large-sheet, demo-session, drawing, flow, PDF stream, canonical JSON, and retained diff laws. `behavior.test.ts` extends this via `portable-oracles.mjs`, where three, lodash, fast-json-patch, fast-xml-parser, pdfjs, and native canvas remain behind local test interfaces.

The subset has committed mutation before/mutation/after vectors and a Rust mutation adapter under `🧪️tests/📐️mutate-layout-1`. Its historical prose says 25 variants but current KINDS contains 31; this prose is stale and should not be used as authoritative coverage. The adapter records a no-third-party-reader decision for native mutation semantics and applies explicit inverse/round-trip/observability laws. Other schema and interoperability features already use independent third-party witnesses.

## Remaining End-to-End Gap

Only `✏️s/🧪️tests/🏘️plugin-composition/🛍️layout-canvas-catalogue/🟦️.ts` was found as Layout-specific renderer integration in the inspected S/OS/Storybook test trees. It mounts a mocked Canvas2dHost and verifies drag/drop action envelopes; it does not drive the Layout controller, document mutation, history, persistence, or export. Its first test title claims an independent JSON Schema oracle, but its body only checks unique fixture IDs and presence of drag-over/drop cases.

A high-value browser smoke should activate the real installed Layout plugin, create/open its demo document, assert a painted page, insert a catalogue frame, select and transform it, undo/redo, save/reopen, and export PDF/SVG/PNG. Assert exported dimensions/content with existing independent decoders. Use the existing JSON fixture contract and established host route rather than adding direct runtime dependencies. Exercise cancellation while export is pending and a short connection interruption around persisted work.

## Likely Blockers to Resolve Before Claiming Completion

1. Passing TypeScript/source gates alone cannot establish that the Rust component compiles or the host loads the installed plugin.
2. Mocked renderer action forwarding cannot detect unavailable catalogue commands or failed native dispatch.
3. Shared workspace build admission and component publication may block native/browser checks; preserve owner refusals in ticket evidence and retry declared routes.
4. Mutation catalogue/schema/implementation drift needs its own current comparison; another agent owns the set-drawing-text investigation.
