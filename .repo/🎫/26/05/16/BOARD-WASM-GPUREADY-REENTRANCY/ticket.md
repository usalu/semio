# Board WASM GpuReady Reentrancy

**Status:** Done (follow-up: commit `379` regression fixed; repo MCP unavailable).

**Problem:** `borrow_fail` on `BoardSession.gpuReady` when WebGPU `device.poll` inside `renderFrame` (or overlapping session calls) re-enters JS while wasm-bindgen still holds a `BoardSession` borrow.

**Changes**

1. **`BoardRenderer`** (`elements/client/lib/board/index.ts`): `wasmGpuFrameDepth`, `cachedWasmGpuReady`, `readGpuReady()`, `syncGpuReadyCacheFromSession()`. Wrap `pushSceneToWasmDriver` + `renderFrame` in depth; while depth > 0, GPU attach state reads use the cache instead of calling WASM. Refresh cache after attach and when the GPU frame finishes.
2. **`BoardCanvas` `applySize`** (`elements/client/lib/board/index.tsx`): ~~Restored synchronous `renderer.render()` after `setSize`~~ **Reverted (2026-05-16):** rely on `setSize` → `markDirty` → `invalidate` only; avoids re-entrant `render` during async `attach_canvas`.
3. **Board play entry** (`elements/client/lib/board/play/index.tsx`): Reuse one `Root` per `#root` via expando `__boardPlayReactRoot` so Vite HMR does not call `createRoot` twice on the same container.

**Follow-up (2026-05-16, `borrow_fail` at `BoardSession.setSize`):** Commit `379` called `pushSceneToWasmDriver()` from every `render()` while `initGpuSurfaceOnce` still held `wasmSessionBorrowDepth` across `await attach_canvas`, so overlapping `render`/`applySize` hit `setSize` → `borrow_fail`, frozen canvas (“cut board”), DnD still worked.

4. **`pushSceneToWasmDriver`**: early-return when `wasmSessionBorrowDepth > 0 || wasmGpuFrameDepth > 0`, set `invalidated = true` for retry.
5. **`initGpuSurfaceOnce` `finally`**: after decrement, if depth is zero and not disposed, `invalidate()` so the deferred push runs after attach completes.

**Verification:** `bunx vitest run --config vitest.config.ts` in `elements/client/lib/board` — 46 tests passed. `bunx playwright test --config play/playwright.config.ts` — 1 passed, 1 skipped (no WebGPU adapter).

**Files touched:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, `elements/client/lib/board/play/index.tsx`, this ticket.
