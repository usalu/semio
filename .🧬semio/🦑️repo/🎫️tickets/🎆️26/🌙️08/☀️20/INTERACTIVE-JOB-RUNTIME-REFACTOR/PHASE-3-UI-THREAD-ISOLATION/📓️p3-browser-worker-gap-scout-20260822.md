# Phase 3 Browser Worker Gap Scout

Date: 2026-08-22  
Verdict: **Phase 3 remains open.**

## Current source boundary

- `🧰️framework/🛠️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs` owns the browser `CanvasHost`. Its `requestAnimationFrame` callback calls `delegate.redraw(reason)` directly on the browser UI thread.
- `🧰️framework/🛒️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊f️wgpu/🦀️frame_job.rs` explicitly records that the crate has no Web Worker bridge.
- The native `winit_app.rs` path schedules the frame transaction on the process worker pool and keeps presentation on the UI thread. That evidence does not prove the browser target, because wasm cooperative pool pumping can still execute the frame job inside the RAF callback's browser isolate.
- Existing TypeScript workers (`effect-backbone.ts`, actor shard clients, plugin module workers) are separate product/plugin transports. None is wired as the browser owner of `FrameTransaction` and `RenderSnapshot` construction.

## Required repair packet

Implement a browser frame-worker transport behind the existing host/renderer interfaces. The UI isolate may only enqueue timestamped/coalesced events, atomically accept the newest prepared snapshot or transferable packet, submit/present it, and apply cursor/IME/accessibility directives. `FrameTransaction`, intent routing, reconciliation, layout, tessellation, and render-packet construction must execute in the worker isolate with operation/revision/generation authority.

The transport must provide bounded item and byte credits, latest-wins event coalescing, lossless bounded commit/checkpoint lanes, stale-generation rejection, worker termination/cancellation on close, device-loss recovery, and an explicit no-Worker fail-closed state. It must not fall back to inline RAF computation.

## Acceptance evidence still required

- Real browser Worker runtime proving product/plugin/domain code does not run on the UI isolate.
- RAF callback and presentation p99 at or below 2 ms, with no callback at or above 8 ms.
- Stress coverage for large transactions, resize/pointer storms, worker saturation, stale snapshots, cancel/close races, worker fault, and device loss.
- Both browser Wasm targets and the native renderer target after the cross-target API change.

No source was modified and no build/runtime gate was run by this scout.
