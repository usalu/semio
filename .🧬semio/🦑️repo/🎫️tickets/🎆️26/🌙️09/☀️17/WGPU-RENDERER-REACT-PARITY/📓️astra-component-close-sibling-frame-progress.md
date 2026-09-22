# Component Close and Sibling Frame Progress

## Finding

Native `OsHost::build_and_publish_snapshot` pumps one component-close unit and returns whenever that exact close remains nonterminal. The return occurs before EventQueue drain, worker frame admission, presenter progress, and immutable snapshot publication. `redraw_core` then presents the last snapshot again. A transport- or retirement-blocked component A can therefore freeze new interaction and pixels for unrelated window B even though B's callback already admitted its input.

Source authority:

- `🪟️winit-app/🦀️.rs:239-251` performs the close step and returns.
- the first B input drain is `:252-258`.
- frame admission is `:261-276`.
- snapshot publication is `:288-307`.
- `redraw_core` still calls `present_snapshot` after the build half returns (`:217-220`), so the failure re-presents stale pixels rather than making the window disappear.
- `OsHost::advance_component_surface_close` owns one exact close request and returns `true` while its bounded close step, terminal drain, or terminal publication is incomplete (`🏠️os-host/🦀️.rs:694-714`).

## Schema-first law

The new neutral contract is:

- `🧫️fixtures/🧵️component-close-frame-turn/📐️schema.json`
- `🧫️fixtures/🧵️component-close-frame-turn/🔣️.json`

It fixes one close unit per turn, distinct A and B host identities, two externally waiting A close turns, B input sequences 41/42, B snapshot revisions 8/9, a nonterminal A close, and zero A publication.

The third-party oracle in `🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts` validates the fixture with Ajv 2020 and uses Chromium `MessageChannel` turns to prove B's two event/frame publications progress while A's independent external promise remains unresolved.

Receipt:

```text
SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true \
bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- \
'🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts' \
-t 'keeps an unrelated window event and frame turn live while one component close waits externally'
```

Result: PASS, 1 selected / 154 skipped, 11 files discovered, Vitest 7.16 s, Nx 14.4 s. Receipt: `🗑️generated/astra-component-close-frame-turn/neutral-chromium.log`.

## Native fail-first laws

`component_close_external_wait_keeps_unrelated_window_ingress_routable` in `🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs` drives the real `enqueue_host_event`, `EventQueue`, generation, and bounded page drain twice. It establishes that B's exact discrete events are valid and routable independently of A.

`component_close_external_wait_does_not_stop_unrelated_frame_publication` in `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` applies the same neutral contract to the real native build function. It requires:

1. one A close step before B work;
2. no `return` between that close step and B's EventQueue drain;
3. B input drain before frame admission before immutable snapshot publication;
4. the nonterminal A close retaining its `RESOURCE_READY` wake;
5. zero A publications in the neutral receipt.

The second law is intentionally RED on current source because the exact close branch contains the early `return`. No Cargo command was run locally. Root's exact native filter is:

```text
test(component_close_external_wait_)
```

Both edited Rust files parse under `rustfmt --edition 2021 --emit stdout`; this is a syntax receipt, not a compile or behavior result.

Native130 executed the full renderer census and recorded the intended source-control RED in `🗑️generated/astra-runtime/renderer-native130-full/failures.json`: the close branch still returned before unrelated ingress. The overall census was 1,359 run / 1,306 pass / 53 fail in 29.006 s; the other failures are not attributed to this packet.

## Repair boundary after RED

Advance at most one close unit and retain `RESOURCE_READY`, then continue the ordinary event/frame/presenter path for live siblings. A's host remains excluded by its existing exact close/retirement fences. The repair must not publish or re-enter A, skip A's close owner, clear the atomic input/presentation fence, increase any queue/capacity, or turn an external wait into an uncharged loop.

## Production repair

`build_and_publish_snapshot` now advances one close unit, retains `RESOURCE_READY` while that owner remains nonterminal, and continues through the ordinary event drain, frame admission, presenter progress and snapshot publication path. This keeps the unit bound while allowing live sibling B to advance.

Close admission now waits for both pre-close owner classes: `AppPresenter::has_pending_presentation` and `FrameBuildHandle::has_live_session`. Once both are empty, the exact A close owner begins; later A-local external waits no longer become a global frame barrier. The source law also asserts this two-owner admission fence before the exact close owner is taken.

The affected Winit, OsHost and renderer-law Rust sources parse under `rustfmt --edition 2021 --emit stdout`. Native post-repair execution remains owned by root.

## Superseding close-created-frame finding and repair

The earlier sibling-progress model was incomplete once a Chrome frame itself created the component-close request. That frame remained live while `OsHost::advance_component_surface_close` refused to take the bridge behind the blanket live-frame fence. The frame waited for bridge terminal and the bridge waited for that same frame to disappear.

The neutral contract is now `semio.component-close-frame-turn.v2`: input for an unrelated window remains queued through one transient frame-retirement turn and one external-owner turn, no frame publishes before terminal, and the same input generation may publish after terminal. Its Ajv/Chromium oracle passed in the canonical browser-worker target: 11 files / 156 tests, Vitest 6.91 s, Nx 13.5 s.

Production now has a reusable `FrameBuildHandle::retire_for_component_surface_close_step`. It cancels and retires one exact worker-session owner unit without setting permanent `closing` or removing the installed completion waker. When that owner reaches terminal it clears the native `last_submitted_generation` fence, permitting a same-generation successor. `OsHost` preserves the presenter packet fence, transiently retires a live creating frame before taking the bridge, and the Winit/browser shared build path suppresses fresh frame admission while `component_surface_close_pending()` remains true. Terminal changes that predicate to false, allowing the normal successor frame to consume the bridge acknowledgement.

The native behavioral law is `component_close_transiently_retires_the_creating_frame_and_readmits_the_same_generation`; the source/admission and ingress laws are `component_close_retires_its_creating_frame_before_external_progress_and_readmits_after_terminal` and `component_close_handoff_keeps_unrelated_window_ingress_routable`. Their Rust sources parse under rustfmt; root owns native execution.
