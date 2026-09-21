# Browser Component Asset Cancellation

## Scope

Read-only source audit of the browser fetch owner when component **A** closes while its response body is stalled and component **B** remains live. No build or runtime command was run for this report.

## Confirmed reachable block

`BrowserRendererWorker` has exactly one active browser response owner: `asset_fetch` ([browser worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:121)). `poll_asset_request` only draws another request when that slot is empty ([lines 129-145](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:129)). It follows that a live B cannot be fetched while A remains there.

The component close path explicitly marks a World asset owner closing, then waits for the decoder/probe/World borrower to return before it can finish ([renderer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11771)). The browser tick stays live while any component close is occupied ([browser worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:379)), so a post-tick cancellation opportunity exists without manufacturing an input or frame generation.

`assetResponseCurrent` is the right stale witness. It becomes false only for an explicit cancellation/retirement; a transient runtime-lock miss remains current ([renderer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11853), [lines 11864-11870](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11864)).

The frame worker, however, acts on that witness only for `pageImageDecode` ([frame worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:387)). `pumpAsset` owns `assetAbort` ([line 703](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:703)) and awaits `reader.read(...)` ([lines 715-725](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:715)). The before/after-read current checks are restricted to `request.referenceImage` ([lines 719-727](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:719)). A stalled non-reference GLB, terrain, map, or UI asset never reaches another check. It retains A indefinitely and blocks B's `pollAssetRequest`.

This is production causality, not a test-helper limitation. Full worker shutdown already does the required three actions—abort fetch, cancel page image decode, and return the retained Rust response owner ([frame worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:472-486))—but ordinary component closure does not use that path.

## Narrow repair protocol

Add one frame-worker helper adjacent to `cancelPageImageDecode`, invoked immediately after `runtime.tick(...)` returns:

1. It does nothing if no `assetAbort` and no `pageImageDecode` are live, or if `runtime.assetResponseCurrent()` is true.
2. On explicit false, it aborts the current `AbortController`, cancels the exact outstanding `pageImageDecode` request, and asks Rust to retire the exact `asset_fetch` owner. Do not clear `assetAbort` there; `pumpAsset` remains the sole owner that clears it in `finally`.
3. Make the Rust handback a typed retry, rather than moving a temporarily unreturnable owner to the global-close-only `asset_blocked` field. `abort_asset_response` currently puts a failed handback there and throws ([browser worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:250-268)); ordinary `tick` never retries `asset_blocked`, while `close_step` merely destroys it during whole-worker shutdown ([lines 409-433](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:409)).

The smallest coherent Rust shape is for `abortAssetResponse` to return `returned: boolean`: it keeps a temporarily unreturnable closing owner in `asset_fetch`, answers `false`, and retries the same exact return on the next tick. `assetResponseCurrent()` remains false for that closing owner, so the post-tick helper naturally retries. A successful return empties `asset_fetch`; only then may the existing post-frame `scheduleAssetPump()` ([frame worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:393)) admit B. This avoids a new unbounded queue and preserves the one-owner-per-turn contract.

Do not use `asset_blocked` for a component cancellation. That field's present `owner.close_step()` path is appropriate only after the full host closes; it bypasses the live component's required `return_renderer_asset_owner` handback. Do not start B while the retrying A remains in `asset_fetch`, because `poll_asset_request` would otherwise re-expose closing A.

The stream itself need not poll an arbitrary state between body chunks: `AbortController.abort()` is the cancellation source for Fetch's reader. The post-tick helper is what reaches that controller while `reader.read()` is pending. The catch already treats `AbortError` as cancellation; its `finally` releases JS ownership ([frame worker](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:801-815)).

## Required fail-first browser law

Extend the real browser frame-worker asset-pump seam, rather than a source-string assertion. If the module's private pump prevents direct invocation, extract only that cursor into a first-party testable unit with the existing `BrowserRendererWorkerHandle` methods; retain the worker as its production owner.

Use two distinct exact response descriptors and a mocked `fetch`:

1. A resolves headers and exposes a `ReadableStream` whose first `read()` remains pending until its supplied `AbortSignal` fires. Record A's signal and exact response token.
2. Begin A, wait until its reader is pending, then make A's fake `assetResponseCurrent()` false to model `close_component_asset_step`.
3. Drive the same post-tick cancellation helper. Assert A's signal is aborted, its optional image decode receives only A's `image-decode-cancel` request id, and `abortAssetResponse(A)` is retried until its ownership handback is terminal. No `asset-stream-fault`, byte page, seal, or asset publish may be produced for A.
4. Resolve the aborted reader as `AbortError`. Once A's handback is terminal, drive the existing scheduled pump. Assert B is polled, fetched, reserved, paged, sealed, and sent to exactly one decode opportunity; B's abort signal remains false and it receives no A cancellation.
5. Add a busy handback branch: first A `abortAssetResponse` answers `false`, then `true`. It proves the live retry keeps A exclusive and does not turn a temporary runtime checkout into a global `asset_blocked` leak.

The closest existing oracle is the real transport's exact image-decode cancellation law ([browser frame transport test](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts:453-485)); it already verifies request-id-specific `AbortError`. It does not exercise the worker's fetch/body reader, so it is insufficient alone.

## Decoder close control

The portable decoder's `RendererAssetDecodeJob::begin_close` now only marks `closing` and clears its stored terminal result ([renderer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2937-2940)). It no longer sets an explicit cancellation flag. This is correct for a normal Ready terminal whose probe must still hand its result through the exact boundary; the browser body-close repair must keep that distinction and request cancellation only for the retired A owner.

## Confidence

High for the A-stalls-B causal chain and for the global-only `asset_blocked` retry gap, both directly established by source. Medium for the exact `boolean` ABI naming; the required invariant is a retryable exact handback retained in the browser's live owner slot until completion.
