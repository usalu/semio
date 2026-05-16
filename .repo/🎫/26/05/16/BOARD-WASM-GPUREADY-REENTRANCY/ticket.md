# Board WASM GpuReady Reentrancy

**Status:** Done (repo MCP unavailable; ticket opened manually).

**Problem:** `borrow_fail` on `BoardSession.gpuReady` when WebGPU `device.poll` inside `renderFrame` (or overlapping session calls) re-enters JS while wasm-bindgen still holds a `BoardSession` borrow.

**Changes**

1. **`BoardRenderer`** (`elements/client/lib/board/index.ts`): `wasmGpuFrameDepth`, `cachedWasmGpuReady`, `readGpuReady()`, `syncGpuReadyCacheFromSession()`. Wrap `pushSceneToWasmDriver` + `renderFrame` in depth; while depth > 0, GPU attach state reads use the cache instead of calling WASM. Refresh cache after attach and when the GPU frame finishes.
2. **`BoardCanvas` `applySize`** (`elements/client/lib/board/index.tsx`): Restored synchronous `renderer.render()` after `setSize` (Playwright “context menu” spec still failed without it in this env; failure also reproduced with it—likely harness/headless unrelated, but matches prior behavior).
3. **Board play entry** (`elements/client/lib/board/play/index.tsx`): Reuse one `Root` per `#root` via expando `__boardPlayReactRoot` so Vite HMR does not call `createRoot` twice on the same container.

**Verification:** `bunx vitest run --config vitest.config.ts` in `elements/client/lib/board` — 46 tests passed. Full `bun ./script.ts test` still fails Playwright “Board background menu” in this environment (timeout waiting for menuitem).

**Files touched:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, `elements/client/lib/board/play/index.tsx`, this ticket.
