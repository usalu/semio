# Original Host Lifecycle Ingress

## Current Compiler Integration

The interpreter owner now requires an explicit lifetime clock at `OwnedRuntime::new`. The parent forwards one function unchanged through `run_kernel_pool` and `KernelPoolState`, selecting the existing native process monotonic source at the singleton root. The scale entrypoint supplies its native root clock to `scale_bench::run`, which passes it unchanged to the runtime. `WasmtimeNodeHost::new` now requires a clock from its run-bootstrap caller. These constructor edits introduce no per-instance clock or deadline rebasing. Native product compilation has not reached these downstream callsites yet; its latest verified production refusal was earlier in inference ownership fields. The exact interpreter clock law belongs to the native executor and is separately recorded.

The host retirement Result conversion had three remaining bare bool branches. Those pending branches now return `Ok(false)` and scene-camera failure returns the actual original typed failure through the incoming context. This is a compiler-shape repair, not qualification of the remaining primitive teardown children.

The web executor owns `RuntimeDispatchCursor`, its dispatch/apply/mailbox regions, `pump_frame_inputs` and original snapshot build methods, plus the frame-job mailbox pump. The original source must be admitted there before asynchronous input dispatch and carried with the cursor/intents, then validated by later genuine frame/mailbox contexts. A borrowed StepContext must never be retained across awaits. Parent retains responsibility for the native Winit root supplier and event/lifecycle callbacks.

Additional parent paths: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs`.

The browser worker already owns an authentic `original_frame` entry with explicit business identity, epoch, deadline, fuel and retained input. Its old shutdown order closed only the frame and then retired that context before general host/world shutdown. The web executor now drives the full owned host teardown inside that original cleanup dispatch and leaves `closeStep` as a terminal witness. Browser tick no longer attempts to reap globally abandoned foreign hosts.

The native winit event loop currently owns no corresponding supplied context. `about_to_wait` calls the abandoned-host reaper and its own retirement handle with no caller argument. Its initial host uses `FRAME_BOOTSTRAP_ROOT_GRANT`; neither that copied primitive grant nor a new `StepContext` inside world shutdown establishes original supplier custody. Native lifecycle admission and its root policy/provider remain unfinished. They must preserve actual source, clock, cancellation, finite budget, receipt recipient and child return through shutdown.

Parent edits propagate `Result<bool, OriginalTurnFailure>` plus the actual incoming `StepContext` through `OsHostRetirement::close_step`, its retained state, the frame close call, and `RuntimeMailbox::close_world3d_dynamic_step`. The own-host close path no longer invokes a global reaper. World closure removes its hardcoded operation/generation, root cancel token, fresh maintenance deadline/context, synthetic sequence and 16 MiB primitive release grant. Terrain receives the actual remaining incoming grant and posts its real returned tuple to that original recipient.

An ordered mutable `AdmittedSurfaceMap::get_mut_at` borrows the already retained surface owner, avoiding the previous temporary copied String. It uses the same original `order` entry as `id_at`, rather than indexing the unrelated sparse slot iterator. The abandoned-host reaper now requires caller context and explicitly refuses an occupied table whose original supplier was lost; it mutates no foreign owner. This is an unfinished boundary requiring a proper retained supplier or replacement of that global abandonment design, not a successful native shutdown claim.

The existing browser schema/corpus gate was authored and run before the lower implementation and now checks six ordered close stages, exact original recipient/business/deadline and terminal-only witness behavior. Native runtime and full browser/WASM qualification remain pending. The renderer's current compilation has so far stopped earlier at actual neural dependency and UI NativeClipboardJob API mismatches. The web executor owns clipboard migration and its existing physical fixture; the native executor owns rerunning the real producer after those fixes. No new context factory or unbounded grant was introduced to satisfy a compiler signature.

Exact parent-owned paths in this propagation:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`

Browser worker and fixture/schema/runner paths belong to the web executor and are recorded in its audit. Complete prototype/release graphs and accepted warm launcher latency remain unverified.
