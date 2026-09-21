# Browser Component Asset Cancellation Repair

## Contract

The browser Worker admits one exact Rust asset response owner at a time. A retired component makes `assetResponseCurrent()` false. That witness must abort the same Fetch controller, cancel only that owner's page-image decode request, and return the Rust owner before a later asset is polled. A temporarily busy handback remains in the live `asset_fetch` slot and is retried; it is not moved to the whole-worker `asset_blocked` close path.

The neutral fixture and strict schema are:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🛑️browser-component-asset-cancellation/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🛑️browser-component-asset-cancellation/📐️schema.json`

The first-party bounded cursor is `🎯️targets/🧊️wgpu/🎞️frame-worker/🧩️asset-cancellation/🟦️.ts`. It has three states: idle, retrying the exact handback, and returned but awaiting release of the browser pump owner. It has no queue and does not alter asset byte/page capacities.

## Fail-first receipt

Command:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker
```

Receipt: `🗑️generated/astra-runtime/browser-asset-cancel-red/run.log`.

The valid second run executed 11 files and 151 tests: 150 passed and the new law failed. Vitest took 5.27 seconds; Nx took 11.1 seconds. The strict schema passed, the cancellation cursor returned `waiting`, and the assertion `controller.signal.aborted === true` failed because the retained non-reference Fetch was not aborted. This isolates the production omission rather than a fixture decode failure. The preceding run used the wrong fixture taxonomy emoji and is not behavioral evidence.

## Production repair status

The TypeScript cursor now aborts the exact controller, cancels the exact page-image decode, retries `returnResponseOwner`, and holds its returned state until the browser pump's `finally` releases the JS owner. The generated Worker ABI now expects `abortAssetResponse(): boolean`. The Rust browser worker preserves a busy owner in `asset_fetch` and reports whether the exact handback completed; ordinary component cancellation no longer moves the owner into the global-close `asset_blocked` slot.

Review found a second race after the first green: a retired response could be returned while its Fetch continuation was suspended in reserve, read, decode, or seal. The pump now retains a local controller through `finally` and checks both that signal and `assetResponseCurrent()` before every response-owner mutation and after every suspension. An abort is terminal for that continuation, so its catch path does not call the already returned Rust owner again or publish an `asset-stream-fault`.

## Focused green receipt

The same scoped Bun Nx command is green after the bounded cursor repair: 11/11 files and 152/152 tests passed, Vitest 10.30 seconds and Nx 18.2 seconds. This census includes the concurrently added native-page Chromium oracle. Receipt: `🗑️generated/astra-runtime/browser-asset-cancel-green/run.log`.

After the continuation fence was added, the suite passed 11/11 files and 153/153 tests, with 3.87 seconds of test time and 8.3 seconds for Nx. The added law starts with a busy reserve, retires and returns that exact response across the suspension, then proves no push or other response-owner call occurs and no non-abort fault is emitted. Receipt: `🗑️generated/astra-runtime/browser-asset-cancel-continuation/test.log`.

The helper module is registered in the authored taxonomy browser profile, the generator input patterns, and the WGPU TypeScript Nx inputs. The scoped generator produced a fresh `🎞️frame-worker.js`; both `check-browser-worker` and `check-frame-worker` passed, with the latter explicitly reporting the generated worker fresh. Receipts are under `🗑️generated/astra-runtime/browser-asset-cancel-generated/`.

Review then exposed a separate polling admission race. After the aborted pump released its JS owner, a subsequent frame could retry the Rust handback, receive `Busy`, and still unconditionally schedule `pumpAsset`. That poll could refetch closing A before the exact response authority returned. The new fail-first run executed 154 tests: the polling-gate law and one unrelated concurrent worker-source policy law failed, while 152 passed. Receipt: `🗑️generated/astra-runtime/browser-asset-handback-gate-red/run.log`.

`runFrameTurn` now schedules polling only while the cancellation cursor is `idle`, or after a `returned` cursor is released while no async pump still owns JS continuation state. `waiting` never schedules a poll. The post-repair run has the new browser transport law green: 153 passed, with only the same unrelated `next_deadline` worker-source policy law failing. Receipt: `🗑️generated/astra-runtime/browser-asset-handback-gate-green/run.log`. The worker was regenerated again and `check-frame-worker` plus `check-browser-worker` passed; their receipts are in the same directory.

A final admission audit found three callers of `scheduleAssetPump`: boot, the frame turn, and the async pump's `finally`. The scheduler itself now checks `assetCancellation.pollAdmitted()`, so every caller refuses both `waiting` and `returned` handback states. The `finally` path can requeue only after `releaseReturned()` changes the cursor to `idle`; a Busy handback leaves it `waiting` and cannot self-poll the closing response. The root-owned final browser census is green at 11 files and 154/154 tests (`🗑️generated/astra-runtime/browser-worker-handback-final/run.log`). The authored worker was regenerated after this centralized guard, `check-frame-worker` reported the generated worker fresh, and `check-browser-worker` passed. The receipts are `🗑️generated/astra-runtime/browser-asset-handback-final/generate-central-admission.log`, `check-frame-central-admission.log`, and `check-browser-central-admission.log`.

The final focused law ran after the centralized guard and generated-worker refresh: 1 passed, 153 outside the name filter, across the 11-file browser profile; tests took 7 ms and Nx 12.8 seconds. Receipt: `🗑️generated/astra-runtime/browser-asset-handback-final/focused-central-admission.log`.

These are TypeScript/DOM and generated-worker receipts. The root-owned native/wasm build and browser activation remain the runtime acceptance authority.

After the final Ink presentation fixture/schema and presenter progress integration settled, the complete browser-worker target was rerun with cache disabled. All 11 files and 154/154 tests passed; Vitest took 10.06 seconds and Nx took 19.0 seconds. Receipt: `🗑️generated/astra-runtime/browser-worker-final-ink-presenter/run.log`. The suite exposed no generated-worker freshness error, so the conditional generation checks were not repeated.
