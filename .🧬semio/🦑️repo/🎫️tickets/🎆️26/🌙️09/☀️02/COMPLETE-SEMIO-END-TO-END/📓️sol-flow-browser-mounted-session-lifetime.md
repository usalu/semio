# Mounted Flow Browser Session Lifetime

## Scope

This slice verifies two real mounted `FlowGraphCanvasHost` React consumers sharing one authored `FlowBrowserRuntime` over the repository's controlled serialized bridge. It is not native Wasm, a real browser, WebGPU, or a published generated-browser artifact acceptance claim.

## Implementation

- Extracted the canonical typed `MockFlowBridge` into adjacent test support and reused it from the existing JavaScript host suite and the React suite.
- Added exact operation-to-session attribution, bounded held-open replies, and terminal/global close observations to the bridge.
- Extended the language-neutral browser-runtime fixture and schema with the shared-runtime, ordered host-retirement, late-open-reply, live-peer command, and global-close invariants.
- Mounted hosts A and B against one `FlowBrowserRuntime`, retired A, proved B still sends `synchronizeDocumentJson` on its own session, retired B, and proved the global bridge closes only when the runtime owner closes.
- Held A's open reply, unmounted A before admission completed, opened and drove B, released A's late reply, and proved only A was closed while B remained usable until its own retirement.
- Registered `@semio-tech/framework-renderer-react:flow-browser-runtime-check` through the package `📜️script.ts` and launch registry.

## Evidence

- Registered exact gate: GREEN, 2 passed / 409 skipped / 411 discovered, duration 15.70s.
- Full renderer test-long after the implementation: GREEN, 585 tests across 11 files.
- Scoped `git diff --check`: GREEN.
- Plugin launch registry generation and generated-file check: GREEN.
- The contemporaneous renderer typecheck reached a single unrelated Directory schema diagnostic (`executionProtocol.appChannelVersion` widened to `number` instead of literal `14`); there were no diagnostics in the Flow runtime, reusable bridge, neutral fixture, or React tests.

## Remaining Acceptance

Native Wasm regeneration and a genuine browser/WebGPU execution remain owned by the parent lane. This evidence establishes the authored JavaScript runtime's multi-session ownership and React consumer lifetime over the controlled protocol bridge only.
