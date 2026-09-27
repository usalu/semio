# WGPU Interaction Checkout Stall Evidence

## Observed runtime evidence

The 2026-09-26 browser capture reached boot ready, then quarantined after 71,741 ms with:

`presentation stalled: phase=Render engine=0 upload=1 gpu-cursor=None upload-progress=(0, 0, 0) input-wait=InteractionCheckout`

This proves that presentation was waiting for the checked-out `AppInteractionState`. The old diagnostic named every ordinary deferred checkout only `frame-deferred`, so the capture does not prove which `FrameDeferredWork` was active and does not prove a network request was outstanding.

## Verified source chain

- `RuntimeApply::start_frame_deferred` removes `AppInteractionState` before starting one deferred work item.
- The browser `PumpSync` arm awaits `ShellState::pump_sync_events` while holding that state.
- The browser directory pump can await `GET /_semio/hub/auth/sessions/me` through `BrowserDoorDirectoryTransport`.
- `directory_ctx()` supplies no deadline for that identity request.
- The browser transport previously called `host_io_call` without a timeout, and page `directoryHttp` used `fetch` without an abort signal.

This is a real unbounded live-owner defect and is consistent with the capture. The available console capture does not identify the endpoint, so it remains a candidate cause until a fresh runtime reproduces with the new retained trace.

## Bounded directory hop

The directory door now carries a required `timeoutMs` of 5,000 ms. The page validates the bound and owns an `AbortController` for the complete fetch/body-read lifetime. An unresolved directory hop returns a transport error and releases the ordinary deferred future; it does not change abandoned-owner detection.

Neutral contract:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/⏳️wgpu-directory-http-deadline/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/⏳️wgpu-directory-http-deadline/🔣️.json`

Laws:

- Rust `every_directory_hop_carries_the_neutral_bounded_deadline`
- TypeScript `aborts an unresolved page fetch before it can retain the renderer interaction owner`

The full WGPU TypeScript receipt is 419 passing tests across 42 files, including the new page-host oracle.

## Retained diagnostics

`FrameDeferredCursor::checkout_site` now names the exact owned category without allocation:

- `frame-deferred-shell-maintenance`
- `frame-deferred-pump-sync`
- `frame-deferred-action`
- `frame-deferred-tutorial-flush`
- `frame-deferred-settle`

The browser host-I/O await holds one fixed `AtomicU8` trace category and resets it through a drop guard. The presentation watchdog retains both static labels in `InteractionCheckout { site, request }`. Request labels expose only safe operation classes such as `directory-get-session` and `directory-post-command`; they retain no URL query, body, bearer, or user data.

A fresh reproduction can now distinguish a live pump/network wait from a dropped ordinary future. The previous capture cannot be retrospectively classified.
