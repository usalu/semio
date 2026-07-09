# Board WASM GpuReady Reentrancy

**Status:** Done (follow-up: commit `379` regression fixed; repo MCP unavailable).

**Problem:** `borrow_fail` on `BoardSession.gpuReady` when WebGPU `device.poll` inside `renderFrame` (or overlapping session calls) re-enters JS while wasm-bindgen still holds a `BoardSession` borrow.

**Changes**

1. **`BoardRenderer`** (`elements/client/lib/board/index.ts`): `wasmGpuFrameDepth`, `cachedWasmGpuReady`, `readGpuReady()`, `syncGpuReadyCacheFromSession()`. Wrap `pushSceneToWasmDriver` + `renderFrame` in depth; while depth > 0, GPU attach state reads use the cache instead of calling WASM. Refresh cache after attach and when the GPU frame finishes.
2. **`BoardCanvas` `applySize`** (`elements/client/lib/board/index.tsx`): **368** called **`renderer.render()`** after **`setSize`**; that was restored (2026-05-16) for 368 parity after an intermediate experiment dropped it.
3. **Board play entry** (`elements/client/lib/board/play/index.tsx`): Reuse one `Root` per `#root` via expando `__boardPlayReactRoot` so Vite HMR does not call `createRoot` twice on the same container.

**Follow-up 2 (2026-05-16):** Guarding only `pushSceneToWasmDriver` was insufficient: **`setCamera`**, **`setSelectionIds`** (still called `setSelectionIdsJson` after an early-returning push), **`setSelectionOptions`**, **`setWorldRasterTilingOption`**, pointer/wheel/context handlers, and **`deleteSelection`** all called `this.session.*` while `attach_canvas` still held `&mut BoardSession` → same `borrow_fail`. Added **`wasmSessionCallBlockedForReentry()`** and fenced those paths; **`setCamera`** updates JS camera + store + emit then defers wasm; **`setSelectionIds`** defers to **`updateSelection`**. **`dispose`** only clears **`__boardRenderer`** when it still points at this renderer. Playwright: **`no wasm borrow_fail during load and viewport resize stress`** (skips without WebGPU adapter; use **`BOARD_PLAYWRIGHT_CHANNEL=chrome`** on Windows for real GPU).

**Follow-up 3 (2026-05-16):** **`handlePointerDown`** called **`setPointerCapture`** _before_ the WASM re-entry guard. If the user clicked while **`attach_canvas`** held the borrow, the handler returned early **without** releasing capture — the canvas kept capture forever while WASM never received **`pointerDown`** → **no mouse interaction** until full reload. **Fix:** **`releasePointerCapture`** on the blocked early-return path. **Layout:** BoardCanvas root/inner/canvas stack now uses **`flex flex-1 min-h-0 flex-col`** so nested flex under Golden Layout gets a correct height (mitigates “canvas cut at bottom” from **`h-full`** in a flex child).

**Verification:** Vitest 46 passed.

**Follow-up 4 (2026-05-16):** Root cause for regressions after WebGPU attach is wasm-bindgen’s **`async fn attach_canvas(&mut self)`** holding the session **`RefCell` borrow across `await`**. Any overlapping **`setSize`** (from **`pushSceneToWasmDriver`**) still produced **`borrow_fail`** when fences missed timing. **Rust fix:** `BoardSession` now wraps state in **`Rc<RefCell<BoardSessionInner>>`**; **`attach_canvas`** is a sync export returning **`future_to_promise`** so the outer wasm borrow ends before GPU **`await`**, while **`setSize` / `renderFrame`** take short **`borrow_mut`** scopes. **`BoardSessionInner`** helpers satisfy the borrow checker for resize + GPU frame.

**Verification (follow-up 4):** `bun ./rs/scripts/build-wasm.script.ts`; Vitest 46 passed.

**Follow-up 5 (2026-05-16):** Compared **`a01093653` (368)** vs HEAD: **368 never called `pushSceneToWasmDriver` from `render()` for main-thread canvas**—only **`syncGpuFrame`** ran when **`gpuReady`**, and **`syncGpuFrame`** did **`pushSceneToWasmDriver` then `renderFrame`**. Later code pushed WASM every frame **before** the swapchain existed, which differed from 368 and amplified attach/resize races. **Restored 368 behavior:** canvas branch only runs GPU init + **`syncGpuFrame`** when `!gpuSurfaceUnavailable`; **`syncGpuFrame`** prepends **`pushSceneToWasmDriver`**. **`BoardCanvas` `applySize`** again calls **`renderer.render()`** after **`setSize`** (368). Vitest 46 passed.

**Follow-up 6 (2026-05-16):** **`git diff a01093653 c3889ca27 -- elements/client/lib/board/index.tsx`** shows **`369` added the stacked text-overlay canvas + inner wrapper**; **`368` used a single canvas** as the observed flex child. Vitest does not exercise WebGPU/main-thread DOM. **Reverted `BoardCanvas` to one canvas** (overlay captions disabled until a safe re-stack). Plan updated: `.cursor/plans/fix_board_wasm_reentrant_borrow_c9d29413.plan.md` §0 §7–§9.

**Files touched:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, `elements/client/lib/board/play/index.tsx`, `elements/client/lib/board/rs/lib.rs`, `elements/client/lib/board/rs/pkg/*` (generated), this ticket.
