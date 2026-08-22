# Phase 3h Browser Worker Architecture

Date: 2026-08-22

## Validated source boundary

- The generic browser `CanvasHost` still invokes `WindowDelegate::redraw` from its
  `requestAnimationFrame` closure.
- The mounted WGPU browser renderer does not use `CanvasHost`, but its winit-web redraw callback
  reaches the same defect through the wasm branch of `FrameBuildHandle::poll_runtime_and_resubmit`.
  That branch runs `FrameBuildJob`, `RuntimeMailbox::apply_pending`, `AppRuntime::frame`, shell
  traversal, layout/tessellation, and `PreparedRenderJob` synchronously in the browser UI isolate.
- `AppRuntime` is already separated from `Window` and `GpuContext`. The remaining browser-only
  platform coupling is `AppPresenter.window` and `GpuContext::from_window`.
- WGPU 27 already exposes `SurfaceTarget::OffscreenCanvas`; no new runtime dependency is required.

## Selected architecture

The browser renderer transfers its canvas to one dedicated module Worker. A second renderer Wasm
instance is initialized in that Worker. The Worker owns `RuntimeMailbox`, `AppRuntime`, `OsHost`,
`FrameBuildHandle`, `PreparedRenderJob`, `AppPresenter`, and the WebGPU surface created from the
transferred `OffscreenCanvas`. The UI isolate owns only DOM input normalization, bounded transport
admission/coalescing, one requestAnimationFrame presentation/directive turn, cursor/fullscreen
directives, and lifecycle/fault display.

The existing wasm `FrameBuildHandle` synchronous drive is retained because its executing isolate is
now the dedicated Worker, not the UI isolate. Native continues to use the process `WorkerPool`.

Transport invariants:

- replaceable pointer-move, wheel, and resize messages are latest-wins;
- lossless pointer/key/text/IME messages use bounded item and byte credits and fail closed on
  exhaustion;
- one frame request may be in flight; newer generation requests supersede queued replaceable work;
- every response carries lifecycle, generation, and sequence authority and stale responses are
  rejected;
- close cancels queued work and terminates the Worker;
- Worker construction, OffscreenCanvas transfer, boot, message, or runtime faults enter a stable
  fail-closed state; the main-thread renderer is never invoked as a fallback.

## Verification boundary

Cargo gates remain deliberately unrun while repository disk pressure is active. Focused Bun/Nx
transport tests and a browser harness are required after implementation. Runtime proof must include
the Worker global scope identity and UI callback timing; static source shape alone is insufficient.
