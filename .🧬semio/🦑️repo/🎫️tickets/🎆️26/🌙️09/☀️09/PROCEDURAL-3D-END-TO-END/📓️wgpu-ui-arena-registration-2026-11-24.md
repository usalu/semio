# WGPU frame Worker: UI arena registration (2026-11-24)

## Root cause
`UI arena has no accepting lifetime registration` (`collection-arena/🦀️.rs:138`, `with_ui_value_arena`) fires in `bootstrap.bootShell()` (stack: `ActionsCtx::boot` -> `settle_boot` -> `dispatch_action(setActiveExample)` -> `publish_shell_panel_document` -> `UiMap::deserialize` -> `UiMapBuilder::try_new`).
Two independent gaps in the Worker boot path:
1. `BrowserRendererWorker::admit_original_frame` called `OriginalActorGuestEntry::admit_step`, which only admits the actor context. Only `admit_plugin_step` also advances `OriginalGuestUiArena` (`UiArenaOpening` -> `UiArenaRegistration::admit`). So no arena was ever registered in the Worker.
2. The entry (and its admission) was created in `BrowserRendererBootstrap::finish`, AFTER `bootShell`, i.e. after the shell had already built UI values.

## Fix
- `reactor/🎟️context/🌉️call/🦀️.rs`: `OriginalActorGuestEntry::admit_frame_step` (funded `admit_plugin_step` with an empty `ensure`).
- `🌐️browser-worker/🦀️.rs`: the boxed entry + receipt now live in `BrowserRendererBootstrap` from `semioWgpuWorkerBootstrap`; the bootstrap exposes `admitOriginalFrame` / `originalFrameReceiptScalar` / `originalFrameReceiptRefusal`; `finish` hands the same entry (stable `Box` address, it is used as source identity) to the Worker. Shared helpers `admit_original_frame_turn`, `original_receipt_scalar`. Retirement is unchanged: `close_original_frame_context` -> `close_step` -> `close_ui_step` retires the registration and backing.
- `🎞️frame-worker/🎟️original/🟦️.ts`: `OriginalBrowserFrameAdmissionNative` (admission-only) split from `OriginalBrowserFrameNative`; `OriginalBrowserFrameSource.attach(native)` rebinds to the finished Worker.
- `🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts`: admit the Source against the bootstrap BEFORE the boot-step loop, `attach(runtime)` after `finish()`.
- `🎞️frame-worker/🧬️bindings-types/🟦️.ts`: bootstrap handle type includes the admission surface.

## Tests
- Rust (reactor): `original_frame_admission_registers_the_ui_arena_before_any_ui_list_is_built` in `reactor/🎟️context/🌉️call/🧪️tests/🤝️admission/🦀️.rs` (borrow is an error before admission, admit to ready, build a `UiList`, drop, drain, close, registry closed again). Both arena tests there now hold `ui_arena_serial()`.
- TS: `🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts` (3 pass) guards that the bootstrap admits before `boot_shell` and that the entry handover is the only `take()`.

## Rebuild and smoke (2026-11-24 14:20)
- Wasm rebuilt (`wgpu-native-wasm-build.sh`), closure + boot/renderer-boot/frame-worker bundles regenerated (the kernel index now imports `node-kind-trigger`, so `wgpu-native-closure.ts` had to run first), activation unchanged, `g3dwgpu` restarted.
- Shell boot now completes (~13 s, panels render, no arena panic).
- Second root cause found at page close: `ActiveFrameBuild` (`🧵️frame-job/🦀️.rs`) implements `InteractiveJob` without close-demand declarations, so the default `close_demand_without_owner` refused ("interactive job retained close demand is undeclared") and `closeOriginalFrame` threw `original-frame.native-authority-refused`. Added the four `next_close_*_demand` overrides.
- Final smoke (`🗑️generated/wgpu-native-smoke/{console.txt,shot.png}`): no panic, no pageerror; only the known 404 on `/extension-modules/install` and the aborted double fetch of `_bg.wasm`. Screenshot is a blank dark canvas (headless paint, see `📓️wgpu-blank-paint-2026-09-12.md`).
- Not run: the native reactor Rust test (disk gate <25 GiB: shared disk was 1-15 GiB free).
