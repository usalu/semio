# Board WASM GpuReady Reentrancy

**Status:** Done (follow-up: commit `379` regression fixed; repo MCP unavailable).

**Problem:** `borrow_fail` on `BoardSession.gpuReady` when WebGPU `device.poll` inside `renderFrame` (or overlapping session calls) re-enters JS while wasm-bindgen still holds a `BoardSession` borrow.

**Changes**

1. **`BoardRenderer`** (`elements/client/lib/board/index.ts`): `wasmGpuFrameDepth`, `cachedWasmGpuReady`, `readGpuReady()`, `syncGpuReadyCacheFromSession()`. Wrap `pushSceneToWasmDriver` + `renderFrame` in depth; while depth > 0, GPU attach state reads use the cache instead of calling WASM. Refresh cache after attach and when the GPU frame finishes.
2. **`BoardCanvas` `applySize`** (`elements/client/lib/board/index.tsx`): ~~Restored synchronous `renderer.render()` after `setSize`~~ **Reverted (2026-05-16):** rely on `setSize` → `markDirty` → `invalidate` only; avoids re-entrant `render` during async `attach_canvas`.
3. **Board play entry** (`elements/client/lib/board/play/index.tsx`): Reuse one `Root` per `#root` via expando `__boardPlayReactRoot` so Vite HMR does not call `createRoot` twice on the same container.

**Follow-up 2 (2026-05-16):** Guarding only `pushSceneToWasmDriver` was insufficient: **`setCamera`**, **`setSelectionIds`** (still called `setSelectionIdsJson` after an early-returning push), **`setSelectionOptions`**, **`setWorldRasterTilingOption`**, pointer/wheel/context handlers, and **`deleteSelection`** all called `this.session.*` while `attach_canvas` still held `&mut BoardSession` → same `borrow_fail`. Added **`wasmSessionCallBlockedForReentry()`** and fenced those paths; **`setCamera`** updates JS camera + store + emit then defers wasm; **`setSelectionIds`** defers to **`updateSelection`**. **`dispose`** only clears **`__boardRenderer`** when it still points at this renderer. Playwright: **`no wasm borrow_fail during load and viewport resize stress`** (skips without WebGPU adapter; use **`BOARD_PLAYWRIGHT_CHANNEL=chrome`** on Windows for real GPU).

**Follow-up 3 (2026-05-16):** **`handlePointerDown`** called **`setPointerCapture`** *before* the WASM re-entry guard. If the user clicked while **`attach_canvas`** held the borrow, the handler returned early **without** releasing capture — the canvas kept capture forever while WASM never received **`pointerDown`** → **no mouse interaction** until full reload. **Fix:** **`releasePointerCapture`** on the blocked early-return path. **Layout:** BoardCanvas root/inner/canvas stack now uses **`flex flex-1 min-h-0 flex-col`** so nested flex under Golden Layout gets a correct height (mitigates “canvas cut at bottom” from **`h-full`** in a flex child).

**Verification:** Vitest 46 passed.

**Files touched:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, `elements/client/lib/board/play/index.tsx`, this ticket.
