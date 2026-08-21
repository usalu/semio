# Phase 5 Renderer Checkpoint Audit

Read-only checkpoint at 2026-08-21. Scope: Phase 2/3/5 records and the current indexed diffs in `📦️glue.rs` and `🦀️winit_app.rs`. No product, source, or configuration file was changed by this audit.

## Attribution and intent

Phase 2 supplies the real `InteractiveJob` protocol: synchronous bounded steps, cancellation, `StepOutcome`, worker submission, and the generation/revision vocabulary. Phase 3a supplies enqueue-only input and a mutex-backed latest `RenderSnapshot`; Phase 3b supplies `FrameBuildJob`/`FrameBuildHandle`, which submits a `Send` deadline scan to the pre-existing renderer worker pool and uses `try_recv` plus UI-side candidate revalidation.

The two currently indexed renderer diffs are a Phase-3c-shaped follow-up, although the referenced `p3c-send-and-thread-locals.md` record is not present in the ticket folder at this checkpoint:

- `📦️glue.rs` removes `AppRuntime::self_weak`, introduces the explicit `AppHandle = Weak<RefCell<AppRuntime>>`, threads it through every deferred continuation, removes dead `PointerCallbacks` construction, and adds a native `assert_send::<AppRuntime>()` compile-time assertion.
- `🦀️winit_app.rs` creates the weak handle at its actual `Rc<RefCell<AppRuntime>>` owner, passes it through deferred input dispatch, and passes it to `AppRuntime::frame`.

Intent is sound: eliminate the self-reference that made the runtime definitionally `!Send` without changing the current UI-thread ownership model. It does **not** move `AppRuntime::frame`, chrome construction, or GPU submission to a worker. The staged diff itself explicitly records one unrelated pre-existing gap: the removed, unused callbacks had been the only caller of `handle_context_menu`; right-click dispatch remains unwired after the P3a enqueue cutover.

## Gate status

| Gate | Status | Evidence |
| --- | --- | --- |
| No new third-party dependency | Met | `bun ./📜️script.ts verify dependencies`: baseline/current `238`, exit 0. |
| Interactivity audit runs | Met, WARN only | `bun ./📜️script.ts verify interactivity`: exit 0, 180 findings; 124 blocking bridges, 36 sync-fs, 8 thread-pool. One stale allowlist entry remains. This is not a zero-finding gate. |
| Target static color lint | Met | target `📜️script.ts lint`: exit 0. |
| Phase-2 protocol conformance | Previously reported, not freshly re-run | P2 record reports checks/clippy and 16 tests in debug/release plus wasm builds. No claim is carried forward as a fresh result. |
| Phase-3 standalone snapshot/frame-job tests | Previously reported, not freshly re-run | P3 records describe standalone verifier crates and their prior results; they validate reproduced logic, not the mounted renderer crate. |
| Mounted renderer Rust test/build | Unverified | target `bun ./📜️script.ts test` waited on the shared Cargo build-directory lock and produced no completed test result. Prior P3 records also say transitive `os-infinite`/`s-plugin-stdio` errors prevent rustc reaching this crate. |
| `AppRuntime: Send` after `self_weak` removal | Unverified | The assertion is correctly compiler-backed, but the renderer crate has not reached rustc. P3b already flags `FontAtlas`/related UI-WGPU internals as unaudited; thread-local state remains an independent execution-safety blocker even if the assertion passes. |
| 8 ms interactive / 2 ms present ceilings under a real renderer event storm | Unverified | Watchdogs and narrow non-blocking job verification exist in Phase 3 records, but no mounted `OsHost` stress test has run. |
| Phase-5 transaction/render split | Not met | `AppRuntime::frame` still executes chrome, immediate-mode hit testing, layout/draw construction, GPU rendering, and presentation on the UI thread. P3b moved only the deadline-scan mechanism. |

## Remaining file-disjoint packets, in dependency order

1. **P3c completion and compile gate — renderer target only.** Own `📦️glue.rs`, `🦀️winit_app.rs`, and the missing Phase-3 record. Finish the explicit-handle audit, restore or deliberately rewire context-menu dispatch, then run the mounted native and wasm checks once the shared build lock and transitive compiler blockers clear. Do not begin worker migration from this packet until `assert_send::<AppRuntime>()` has an actual compiler result.
2. **Thread-local extraction — Shell only.** Own `🧱️elements/Shell/🧊️component.rs` and any new Shell-local state module; do not edit the two renderer files. Classify every `thread_local!` value as UI-present state or explicit build input/output. Re-home the latter into owned data; preserve UI-only behavior on the present side. This is required before worker-side chrome building, regardless of `Send`.
3. **Prepared-render seam — UI-WGPU only.** Own `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/**`; do not edit Shell or renderer host glue. Split encode/preparation from submit/present, with an owned bounded packet whose application does not touch worker-invalid window/surface state.
4. **Hit-test contract — UI-render/InputState boundary only.** Own `ui_render`/`ui_wgpu::InputState` integration files. Either migrate to an immutable `DispatchTree` queried on the UI thread, or define and test an immediate-mode staleness contract. Do not pretend `RenderSnapshot::dispatch_tree` is populated while it is `None`.
5. **Phase-5 transaction job — new renderer job module plus narrow host wiring.** After 1–4, own a new `frame_transaction_job`-style module and its test fixture, then make the minimum `🦀️winit_app.rs`/`📦️glue.rs` wiring change. Use Phase-2 steps/checkpoints/cancellation, bounded input/output, candidate validation, previous-snapshot re-presentation, and absolute timing for caret blinking. Keep submit/present UI-bound.
6. **End-to-end budgets and cancellation — ticket-local verifier plus target tests.** Own only the new verification fixture and tests. Stall layout/tessellation deliberately; prove UI event callbacks and presentation remain within their budgets, cancellation is observed, and stale generation/revision results cannot apply. Then run the real target suite in debug/release/native/wasm.

## Collision risks

- `📦️glue.rs` and `🦀️winit_app.rs` are actively shared and already carry the staged explicit-handle change. They are collision hotspots; packet 5 must wait for packet 1 or coordinate line ownership.
- `🧱️elements/Shell/🧊️component.rs` is both the thread-local blocker and a broad, high-churn UI file. Give the extraction packet exclusive ownership.
- UI-WGPU has to coordinate with the renderer because its packet type becomes the only supported build/present seam; avoid parallel ad-hoc packet definitions.
- Cargo diagnostics are currently unreliable for ownership attribution while another process holds the shared target lock and while transitive crates fail first. Treat any result that does not name the renderer crate as a dependency gate result, not validation of these diffs.

## Runnable now

- Passed now: root dependency ratchet, root WARN-mode interactivity audit, target color-literal lint.
- Worth running once Cargo is free: target `bun ./📜️script.ts test`; then the mounted renderer `cargo check`/tests (native and wasm) and the existing standalone Phase-3 verifier crates.
- Do not claim mounted renderer validation from `rustfmt`, static grep, or copied verifier crates alone.
