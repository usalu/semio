# Renderer Library-Test Diagnostics

Command: `cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format=short`

Result at the renderer bridge checkpoint: production code compiled; Rust test compilation stopped on exactly 34 fixture/seam diagnostics. The one renderer-owned seam drift (`UiIntent::seq`) has since been repaired. The remaining Dock, Interpreter, and Shell fixture migrations are intentionally assigned to the integration owner.

## Exact diagnostics

```text
Dock/🧊️component.rs:1562:20: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:1566:94: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:1573:32: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:1590:97: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2129:54: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2129:193: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2131:58: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2134:48: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2135:164: error[E0433]: cannot find type `LocalizedLabel` in this scope
Dock/🧊️component.rs:2138:65: error[E0433]: cannot find type `LocalizedLabel` in this scope
Interpreter/🧊️component.rs:944:23: error[E0433]: cannot find type `UiPresence` in this scope
Interpreter/🧊️component.rs:1418:111: error[E0433]: cannot find type `UiPresence` in this scope
🦀️kernel_seam.rs:184:9: error[E0063]: missing field `seq` in initializer of `semio_framework_ui_contract::UiIntent`
Shell/🧊️component.rs:5864:69: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5864:81: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5864:93: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5870:72: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5871:76: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5872:82: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5880:66: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5881:66: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5974:109: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5974:121: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:5974:133: error[E0277]: the trait bound `dock::DockStackTab: From<&str>` is not satisfied
Shell/🧊️component.rs:12412:28: error[E0061]: this method takes 1 argument but 0 arguments were supplied
Shell/🧊️component.rs:611:175: error[E0560]: struct `dsl::PresencePeer` has no field named `cursor`
Shell/🧊️component.rs:611:189: error[E0560]: struct `dsl::PresencePeer` has no field named `viewport`
Shell/🧊️component.rs:7420:21: error[E0308]: mismatched types: expected `ConfigSpec`, found future
Shell/🧊️component.rs:12650:36: error[E0308]: mismatched types: expected `IntroductionInteraction`, found future
Shell/🧊️component.rs:9016:59: error[E0308]: mismatched types: expected `&IntroductionPoint`, found `&impl Future<Output = IntroductionPoint>`
Dock/🧊️component.rs:1606:21: error[E0308]: mismatched types: expected `ConfigSpec`, found future
Dock/🧊️component.rs:1607:30: error[E0308]: mismatched types: expected `CommandGrammar`, found future
Shell/🧊️component.rs:7421:30: error[E0308]: mismatched types: expected `CommandGrammar`, found future
Dock/🧊️component.rs:2059:13: error[E0277]: can't compare `dock::DockStackTab` with `std::string::String`
```

## Ownership grouping

- Dock: 13 diagnostics.
- Interpreter: 2 diagnostics.
- Shell: 18 diagnostics.
- Renderer kernel seam: 1 diagnostic, repaired by supplying the typed intent sequence.

## 2026-08-22 final status

The 34-diagnostic historical checkpoint above is resolved. The current renderer library-test build
reaches and runs the crate:

```text
cargo test -p semio-framework-os-renderer-wgpu --lib async_boundary_tests -- --nocapture
test result: ok. 4 passed; 0 failed; 0 ignored; 361 filtered out
```

Additional focused filters from that freshly-built test binary:

```text
kernel_seam::tests
test result: ok. 3 passed; 0 failed

frame_job::tests
test result: ok. 6 passed; 0 failed

winit_app::callback_latency_tests::mounted_pointer_storm_callback_p99_stays_below_two_milliseconds
test result: ok. 1 passed; 0 failed

shell::media_frames_tests::stalled_shell_io_keeps_mailbox_poll_p99_below_two_ms
test result: ok. 1 passed; 0 failed
```

Production gates also pass in dev and release profiles. `bun ./📜️script.ts verify
interactivity` reports `DENY mode — clean`.

The initial wasm attempt stopped in dependencies before reaching the renderer:

```text
cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --message-format=short
editor/component.rs:1381: E0599, E0277
surface/paint/component.rs:1051: E0599, E0277
surface/node-graph/component.rs:590: E0599, E0277
surface/tiled-map/component.rs:3316: E0599, E0277
```

Each E0599 was a `.map_err` call on the future returned by the now-async `RenderContext::new`; E0277
was derivative error recovery. This checkpoint is retained as diagnostic history; the repairs and
green final compiler evidence are immediately below.

## 2026-08-22 cross-target closure

The four upstream `RenderContext::new` call sites now await correctly, and the browser build reaches
the renderer.

```text
cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --message-format=short
Finished dev profile in 21.91s; warnings only

cargo check -p semio-framework-os-renderer-wgpu --target wasm32-wasip2 --message-format=short
Finished dev profile in 0.46s; three mailbox-core dead-code warnings only

cargo check -p semio-framework-os-renderer-wgpu --release --message-format=short
Finished release profile in 1m 01s; warnings only

cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format=short
Finished test profile in 1m 25s
```

Final focused results from that native test build:

- `async_boundary_tests`: 4 passed.
- `runtime_mailbox_core::tests`: 1 passed.
- `kernel_seam::tests`: 3 passed.
- `frame_job::tests`: 6 passed.
- `winit_app::callback_latency_tests`: 2 passed; mounted pointer and resize storms each assert p99
  below 2 ms over 20,000 samples.
- `shell::media_frames_tests::stalled_shell_io_keeps_mailbox_poll_p99_below_two_ms`: 1 passed.
- `bun ./📜️script.ts verify interactivity`: `DENY mode — clean`.

The Wasip2 target compiles the renderer's shared bounded mailbox core without winit or native window
code. Browser Wasm compiles the presentation path with cooperative `spawn_local`, bounded
continuations, target-local non-`Send` host wakers, and inline capacity-one frame preparation.
