# P9-A3 Framework Browser WebGPU Surface Adapter

## Verdict

**GREEN under the authorized dependency-free P9-A3 gates.** The framework WebGPU target now exposes an owned, generation-bearing surface ABI over the accepted A1 message/page/control port. The composed JavaScript boundary consumes the accepted A2 browser-host imports, owns canvas lookup and raw-handle registration, and shares only bytes with Rust. The previous public browser-canvas constructor is gone. `web-sys` and `wasm-bindgen` are removed from this crate's manifest after their source consumers were migrated; `wgpu` and `bytemuck` deliberately remain.

No root manifest, lockfile, shared script, launch configuration, OS renderer, A2 host source, Cargo workspace state, Nx state, Wasm output, or browser state was changed or invoked.

## Owned Contract

The new public edge is:

- `SurfaceId`, a non-zero owned raw selector matching the shim's `data-raw-handle` registration;
- `CanvasMetrics { width, height, scale_factor }`, including a valid zero-sized parked state;
- non-zero, checked `SurfaceGeneration`;
- `GpuOutcome::{Created, Resized, FrameAccepted, Lost, Recovered, Dropped, Cancelled, Rejected}`;
- `WebGpuSurfaceAdapter<P: AbiPort>` and the primitive-only `LinearMemoryWebGpuSurfacePort` generated-import implementation;
- A1 `AbiRequest`, `AbiPage`, and `AbiControl::{Cancel, Close, Acknowledge}` records only.

The language-neutral source of truth is `🧬️schema/🔣️surface-port.json`, with ten executable create/resize/frame/loss/recovery/drop records in `🧪️fixtures/📒️surface-port.tsv` and eight exact maximum/maximum-plus-one laws in `🧪️fixtures/📐️surface-port-limits.tsv`. The binary ledger is version 1, fixed little-endian, and bounded before state mutation.

The fixed admissions are eight surface sessions, four in-flight frames, eight in-flight outcome pages, eight controls, 4,096 frame bytes, 8,192 outcome bytes, and 64 semantic work units per callback. The JavaScript callback watchdog rejects a step at eight milliseconds before host state mutation. Rust honours cancellation, interruption, deadline, and byte-credit refusal without cursor or owner movement.

## Lifecycle And Ownership

Create validates the JavaScript/A2-owned canvas and adapter status before admitting a surface. A failed missing/bad/unsupported replacement leaves the last valid surface and metrics intact. Resize preserves zero-size parking. Frames are checked against surface identity, generation, loss state, payload size, and global frame/page capacity before the frame owner is admitted. A frame remains in flight until its exact outcome page is acknowledged, cancelled, or closed.

Surface and device loss are explicit outcomes. Recovery retains the valid surface metrics, increments the generation with checked arithmetic, and produces the owned admission needed to reconstruct device resources. Old-generation frames are rejected. Drop refuses while a frame owner remains and removes the surface exactly after those owners are handed back. Adapter close retires one retained resource per admitted step and ends in an idempotent terminal-empty state.

The renderer's existing `GraphicsBackend` frame/resource behavior is unchanged: `apply_resources`, render replay, surface-error mapping, readback, and resident-resource recovery semantics were not rewritten. Its public `from_outcome` construction edge accepts only an admitted owned create/recover outcome. `GpuContext` uses `wgpu`'s owned numeric web raw-handle selector; no browser SDK type crosses the Rust API or source boundary.

The JavaScript module `🟨️webgpu-surface.js` imports and composes A2's `createBrowserHostImports`, binds both ports to the same linear memory, registers/removes the canvas raw selector only after accepted outcomes, retains pages until ACK/close, returns an exact rejected frame byte owner, and contains no third-party import.

## Exact Dependency And Source Delta

| Census | Before | After |
| --- | ---: | ---: |
| Direct rows among `bytemuck`, `wgpu`, `web-sys`, `wasm-bindgen` | 4 | 2 |
| Direct `web-sys` rows | 1 | 0 |
| Direct `wasm-bindgen` rows | 1 | 0 |
| Direct `wgpu` rows | 1 | 1 |
| Direct `bytemuck` rows | 1 | 1 |
| Rust source browser-SDK deny matches | 4 | 0 |
| Public `HtmlCanvasElement` constructor edges | 1 | 0 |
| Direct constructor consumers outside the crate | 0 | 0 |

The four retired Rust-source matches were the public `WebGpuBackend::new(HtmlCanvasElement)`, private `GpuContext::new(HtmlCanvasElement)`, and two explanatory browser-SDK comments. The fifth target-wide pre-change census match was the manifest's `web-sys` row. Current Rust source has zero `HtmlCanvasElement`, `JsValue`, bindgen attribute, `wasm_bindgen`, `web_sys`, `js_sys`, `Promise`, or closure bridge match.

Exactly two manifest rows were removed. `wgpu` remains the device/render implementation and still owns browser transitives; `bytemuck` remains the vertex/uniform byte-cast implementation. Their replacement belongs to P9-C, not P9-A3.

## Executed Gates

| Gate | Result |
| --- | --- |
| Direct dependency-free `rustc` crate-root harness with `-D warnings` | GREEN — 28 passed, 0 failed; includes 14 accepted A1 laws and 14 A3 laws |
| Direct dependency-free optimized crate-root harness (`-C opt-level=3`, `-D warnings`) | GREEN — 28 passed, 0 failed |
| Rust hostile matrix | GREEN — create/resize/frame/drop; zero-size; missing/bad/unsupported host; surface/device loss; stale generation; deterministic recovery; cancellation; interrupted send/callback; exact frame max/+1; actual page max/+1; session/frame/page/control admission; lost/stale/duplicate handle; exact page close; incremental terminal-empty close |
| Bun A2+A3 mock | GREEN — A2 composition, raw-handle lifecycle, create/resize/frame/loss/recover/stale/drop, exact frame handback, max/+1 sessions/frames/pages, missing/bad/unsupported canvas, interrupted callback, eight-millisecond watchdog |
| Node syntax checks | GREEN — shim and mock |
| Bun standard-library schema/fixture parser | GREEN — version 1, six operations, ten trace records, eight consecutive max/+1 laws, little-endian ledger |
| Rust fixture execution | GREEN — all ten TSV records decode through the adapter and produce their declared outcome |
| `rustfmt --check` on every owned Rust file | GREEN |
| Static Rust browser-SDK deny census | GREEN — zero matches |
| Manifest retained/retired row census | GREEN — only `wgpu` and `bytemuck` remain; exactly two rows removed |
| Scoped tracked and untracked `git diff --check` | GREEN |

The direct compiler gives the emoji-rooted crate an explicit ASCII crate name. It tests the complete target-neutral ABI/state machine without invoking Cargo or a Wasm target.

## Deferred Integration Gates

- P9-A7 owns OS renderer/browser-worker/host integration and must wire its frame/job contracts to this A3 port without reintroducing a renderer-local browser protocol.
- P9-C owns replacement of `wgpu` and `bytemuck`; neither is claimed removed here. The current `wgpu` transitive browser stack therefore remains expected.
- Root Cargo/lock reconciliation and dependency-verifier integration remain with the serialized integration owner.
- Cargo package/workspace compilation, Nx, a real Wasm target build, generated-import link validation, and real browser WebGPU/canvas/device-loss execution were prohibited or intentionally deferred. They are runtime integration gates, not evidence claimed by this packet.

There is no blocker within the P9-A3 owner scope.
