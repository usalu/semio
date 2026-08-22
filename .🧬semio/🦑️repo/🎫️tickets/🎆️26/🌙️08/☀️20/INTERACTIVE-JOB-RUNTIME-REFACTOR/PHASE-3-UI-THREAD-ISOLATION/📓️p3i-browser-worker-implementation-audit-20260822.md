# Phase 3i Browser Worker Implementation Audit

Date: 2026-08-22  
Verdict: **Source and focused TypeScript gates advanced; Phase 3 remains open.**

## Implemented browser boundary

- The browser renderer now transfers an `OffscreenCanvas` to one dedicated module Worker. The UI
  isolate owns only bounded admission/coalescing, message transfer, RAF scheduling, callback timing,
  and directives. There is no inline redraw fallback.
- Boot receives deterministic renderer JavaScript/Wasm URLs. Product catalog/plugin discovery and
  module loading execute in the Worker. Async operations are passed as thunks so their synchronous
  prefix is timed, and a post-resolution macrotask lets the heartbeat observe a long final promise
  continuation.
- Lossless text/paste ingress uses conservative UTF-8 reservation, surrogate-safe chunks, and a
  16-KiB wire batch ceiling. Pointer movement uses sixteen fixed numeric identity slots. Future
  generations and protocol corruption fault closed.
- Worker frame realization and offscreen presentation use the Worker-owned present capability; the
  browser Worker does not mint the native UI presentation token.
- Fault/quarantine now shuts the interactive scheduler and enters the same bounded close loop as a
  requested close. Runtime and interactive scheduler close steps alternate across macrotasks. The UI
  transport no longer force-terminates after a fixed timeout; it terminates only after Worker
  `closed` acknowledgement.

## Shared interactive-job substrate

- A domain-neutral `InteractiveJobPort` is exported through the React host port seam. It has fixed
  slots, revisioned stable readiness snapshots, bounded observers, per-kind page limits, aggregate
  process item/byte credits, exact generation/operation authority, and closeable consumers.
- Consumers remain owned through terminal, fault, replacement, and port close until both
  `closeStep()` and `terminalIsEmpty()` witness completion. Normal completion drains only its slot and
  leaves other jobs and the shared port live.
- The Worker registry is a fixed static descriptor table. There is no dynamic map or Diagram logic
  in the transport. Each scheduled turn performs one governed job action/phase transition. Factory,
  ingress, cancellation, step, output, close, and post exceptions are converted to protocol
  quarantine and retained bounded close.
- The first registered job kind is `diagram-directed-layout-v1`, implemented in the P10-owned pure
  module and executed on the existing frame Worker rather than a second Worker.

## Paged text authority progress

- Browser text streams use slot epochs, aggregate byte credits, segmented pages, Unicode-safe
  boundaries, balanced cached-byte roots, atomic root publication, and bounded projection.
- Focus now accepts already-owned identifier/value strings. Full editor-document focus no longer
  clones the document into the generic input buffer; editor focus publishes only its owned identity.
- Older undo roots are retained in fixed root-retirement slots before disposal. Publication yields
  before mutation if retirement credit is unavailable; undo leaves ownership in place until credit
  exists.
- `InputState` exposes close-step and terminal-empty witnesses and the browser runtime uses them.
- `InputState` now preallocates and caps hit targets, pending lossless events, pending keys, and drag
  points. Overflow records a deterministic authority fault instead of allowing unbounded growth.
- `TextEditAuthority` fail-safe destruction now also detaches owned ingress, cancelled ingress,
  retired storage, and every page Arc, so an abandoned authority has a shallow destructor. Normal
  cancellation retains owned ingress until its operation retirement is admitted and releases pages
  cursor-by-cursor.
- `OsHostRetirement` now owns the realm-close cursor. Frame handles, host input events, deadline maps,
  snapshots, capabilities, scheduler/kernel/runtime, and presenter are released as separate governed
  turns, followed by an explicit terminal-empty witness. Its fail-safe destructor detaches remaining
  owners rather than recursively destroying them.

## Executed gates

- `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker` — **passed**, 2 files / 32
  tests.
- `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker` — **passed**; UI boot bundle
  39.60 KB, Worker bundle 0.63 MB.
- `rustfmt --edition 2021` was run on the modified Rust source files and completed successfully.
- `bun nx show project @semio-tech/framework-renderer-wgpu` confirmed the permanent `wasm` Nx target.

## Unrun and blocking gates

- Cargo, Trunk, wasm-bindgen compilation, native renderer tests, and the real browser
  Worker/OffscreenCanvas harness were not run. Current filesystem free space is 3.2 GiB; the shared
  `target` directory is 107 GiB and the renderer artifact cache is 419 MiB. The exact controlled build
  command is `bun nx run @semio-tech/framework-renderer-wgpu:wasm`; its output directory is
  `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`. A cold or invalidated incremental wgpu Wasm
  build can plausibly add approximately 0.5–3 GiB, so it is unsafe in the current disk state.
- `BrowserRendererWorker::close_step` now transfers the host into persistent `OsHostRetirement` and
  requires its terminal-empty witness. The outer host fields are cursorized, but `RuntimeMailbox` and
  `AppPresenter` remain single-owner retirement steps; their internal AppRuntime/Shell/GPU graphs are
  not yet cursorized. Therefore the hard sub-8-ms realm-wide close proof remains red.
- Generic `ui_render::DispatchState` still owns its legacy `String` edit state and ignores segmented
  edit events. The paged authority is integrated into the actual wgpu product path, but it is not yet
  the sole authority for every generic/native dispatch consumer.
- Worker frame admission now caps the JSON wire message at 4 KiB (down from 16 KiB), including
  worst-case escape-aware UI chunking. JSON stringify, Rust decode, atomic preflight, event apply, and
  tick still share one Worker callback and have no resumable parse/apply cursor, so only runtime timing
  can establish whether this cap is sufficient; a hard source proof remains red.
- The owned convenience path now rejects payloads above one 16-KiB page. Larger replacement/paste
  must use segmented begin/push/commit, whose pages own independent bounded allocations. Ordinary
  generic/native dispatch is not wired to that segmented requirement yet and therefore remains part
  of the `DispatchState` residual above.
- Rust compiler evidence and actual runtime timing are mandatory before Phase 3 acceptance. No claim
  is made that the Rust/Wasm browser path compiles or runs.
- Rust bootstrap phase labels are persistent, but atlas construction/upload, plugin parse/filter,
  `ShellState` construction, `shell.boot().await`, and final runtime/host construction remain coarse
  owned phases rather than internally fuel/deadline-cursorized jobs. The thunk/heartbeat detects an
  overrun; it does not prevent the owned CPU phase from overrunning before quarantine.
