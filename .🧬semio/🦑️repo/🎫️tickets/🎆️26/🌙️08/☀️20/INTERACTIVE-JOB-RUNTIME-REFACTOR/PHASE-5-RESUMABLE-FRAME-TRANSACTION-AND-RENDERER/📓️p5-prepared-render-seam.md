# Phase 5 Prepared Render Seam

## Outcome

The UI-WGPU boundary now exposes an owned, `Send` prepared-render packet and a resumable worker job. GPU submission remains UI-authorized through a deliberately `!Send` token. Preparation rejects cancellation, stale preview generations, and item/byte credit overruns before publishing a packet. Presentation validates revision, generation, and credits before changing GPU state, and commits the last-valid packet only after successful submission.

The worker handoff is capacity one: `PreparedRenderReceiver` retains the sealed `Arc<PreparedRenderPacket>` even when a pool closure consumes the job. Both `take_packet()` and `take_latest()` consume the single published packet.

## Stable Renderer API

- `PreparedRenderInput::new(scene_revision, preview_generation, draw, overlay, time_seconds)`
- `PreparedRenderJob::new(input, items_per_step)`
- `PreparedRenderJob::take_packet(&self) -> Option<Arc<PreparedRenderPacket>>`
- `PreparedRenderJob::receiver(&self) -> PreparedRenderReceiver`
- `PreparedRenderGate`
- `PreparedRenderPacket` with read-only revision, generation, damage, clips, directives, uploads, usage, and limit accessors
- `UiPresentToken`, constructible only inside the WGPU crate and intentionally `!Send`
- `GpuContext::submit_prepared(&mut self, window, packet, live_revision, live_generation, token)`

`PreparedRenderJob`, `PreparedRenderReceiver`, `PreparedRenderPacket`, and `PreparedRenderGate` are compile-time asserted `Send`. `DrawList` remains movable and `Default`.

## Bounded Work

The job cursor is hierarchical and persistent. Layer headers, raster entries, pass headers, mesh draw headers, individual 3D instances, line buffers, textured draw headers, individual textured instances, glass regions, uploads, and metadata advance independently. A large single layer or scene pass therefore cannot hide an unbounded inner traversal inside one scheduled work item. Every processed cursor item consumes one fuel unit; `items_per_step`, scheduler deadline, cancellation, and generation freshness are checked at resumable boundaries.

Packet fields that determine credit validity are crate-private. External code cannot forge usage or limits, and only the UI submit path can commit the gate's last-valid packet.

## Verification Evidence

All commands ran from the repository root on 2026-08-21.

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-rs:test-wgpu-engine -- wgpu::prepared::tests` | PASS, 11/11 after the final bounded-cursor change |
| `SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/ui-rs:test-wgpu-engine -- --release wgpu::prepared::tests` | PASS, 11/11 |
| `bun nx run @semio-tech/ui-rs:check-wgpu-engine-wasm` | PASS for `wasm32-unknown-unknown` |
| `bun ./📜️script.ts verify dependencies` | PASS, baseline 238/current 238, no new dependency |
| Full debug `--lib --no-fail-fast` | PASS after fixture correction, 260/260 |
| Full release `--release --lib --no-fail-fast` | Initially exposed the same incorrect pre-existing fixture; focused final release is green |
| `cargo clippy ... --all-targets --no-deps -- -D warnings` | Existing crate debt: 27 non-seam diagnostics; no `prepared.rs` diagnostic |
| `cargo clippy ... --lib --no-deps -- -D warnings` | Existing crate debt: 19 non-seam diagnostics; no `prepared.rs` diagnostic |
| `git diff --check` on UI-WGPU and this ticket | PASS |

The full-suite runs exposed an incorrect pre-existing fixture in `wgpu::component::layout::layout_wire_format_tests::action_descriptor_and_style_spec_serialize_to_golden_json`. `DslValue::Number(f64)` uses `Serialize::serialize_f64`, so the canonical output for the test value is `42.0`; the fixture incorrectly expected `42`. The coordinator authorized the one-line fixture correction to `42.0`. This is not prepared-render behavior.

After that correction the full debug suite passed all 260 tests. The final focused release suite passed all 11 prepared-render tests after recompiling the bounded cursor.

The focused release run initially exceeded the repository wrapper's default 15-second command budget during first optimized compilation. Re-running with the documented 120-second test budget completed and passed.

## Files

- `🎯️targets/🧊️wgpu/🦀️prepared.rs`
- `🎯️targets/🧊️wgpu/🦀️gpu.rs`
- `🎯️targets/🧊️wgpu/🦀️draw.rs`
- `🎯️targets/🧊️wgpu/📦️glue.rs`
- `📝️p5-prepared-render-diagnostics.txt`

## Renderer mailbox follow-through

Phase 3's runtime ownership transfer now feeds this prepared-render seam without retaining
`AppRuntime` or a mutex guard across suspension. Native continuations are retained-waker jobs on the
single process `WorkerPool`; their capacity-128 serial completion mailbox reserves one slot for the
owned interaction-state return, rejects lossless keyless overflow, and only coalesces matching
replaceable keys. Frame preparation remains capacity one and rejects stale generations before UI
presentation.

The same generic mailbox core is compiled for all targets. Browser Wasm keeps bounded cooperative
`spawn_local` driving and inline frame preparation. The new crate-root boundary excludes winit and
native window dependencies for Wasip2 while compiling that core instead of patching external winit.

Final follow-through gates on 2026-08-22:

| Gate | Result |
| --- | --- |
| Renderer native dev check | PASS, 21.80 s, warnings only |
| Renderer native release check | PASS, 1 min 01 s, warnings only |
| Renderer lib no-run | PASS, 1 min 25 s |
| Browser `wasm32-unknown-unknown` check | PASS, 21.91 s, warnings only |
| Framework API `wasm32-wasip2` check | PASS, 0.46 s, three mailbox-core dead-code warnings |
| Async boundary / mailbox core | PASS, 4/4 + 1/1 |
| Kernel seam / frame generation | PASS, 3/3 + 6/6 |
| Mounted pointer + resize p99 | PASS, 2/2, each below 2 ms over 20,000 samples |
| Stalled mailbox p99 | PASS, 1/1, below 2 ms |
| Interactivity deny gate | PASS, clean |
