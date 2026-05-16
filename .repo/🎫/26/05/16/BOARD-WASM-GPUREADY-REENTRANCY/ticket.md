# Board WASM GpuReady Reentrancy

**Status:** In progress (repo MCP unavailable this session).

**Problem:** `borrow_fail` on `BoardSession.gpuReady` when WebGPU `device.poll` inside `renderFrame` re-enters JS (ResizeObserver / layout) while WASM still holds `&mut BoardSession`.

**Fix:**

- `wasmGpuFrameDepth` + `cachedWasmGpuReady` + `readGpuReady()` / `syncGpuReadyCacheFromSession()` on `BoardRenderer`.
- `BoardCanvas` `applySize`: drop synchronous `renderer.render()`; rely on `setSize` → `markDirty` → `invalidate`.

**Files:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`.
