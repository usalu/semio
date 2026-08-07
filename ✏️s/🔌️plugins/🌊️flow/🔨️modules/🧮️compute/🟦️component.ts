/** @emoji 🧵️ Flow compute thread-pool helpers (wasm-bindgen-rayon). */

// 🐛️ `export { … } from "…"` is a pure re-export — it does NOT bind the names locally (confirmed via a
// real `tsc`/`bun run` repro: `initFlowThreadPool` below throws `ReferenceError` at runtime otherwise).
// This file pre-existed with the broken bare re-export form (dead code path, never exercised by a test);
// fixed here as a forward-fix while relocating it, per this initiative's established "port the bug fix
// along with the move" convention (see master ticket §12.3's `Ok(`-corruption precedent).
import { defaultComputeWorkerCount, effectiveComputeWorkerCount, isCrossOriginIsolatedRuntime, readStoredComputeWorkerCount, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, writeStoredComputeWorkerCount } from "@semio-tech/ui-react";
import { type StoragePort, createBrowserStoragePort } from "@semio-tech/framework";

export { defaultComputeWorkerCount, effectiveComputeWorkerCount, isCrossOriginIsolatedRuntime, readStoredComputeWorkerCount, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, writeStoredComputeWorkerCount };

export type FlowThreadPoolInit = (numThreads: number) => Promise<unknown>;

/** Initialize the flow rayon thread pool when cross-origin isolated. */
export async function initFlowThreadPool(init: FlowThreadPoolInit, storage: StoragePort = createBrowserStoragePort(), requested?: number): Promise<number> {
  const workers = effectiveComputeWorkerCount(storage, requested ?? readStoredComputeWorkerCount(storage));
  try {
    await init(workers);
    return workers;
  } catch (err) {
    console.warn("[flow] thread pool init failed; falling back to single-thread", err);
    try {
      await init(1);
    } catch {
      /* vitest / non-threaded wasm */
    }
    return 1;
  }
}

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("initFlowThreadPool", () => {
    it("returns the successful init's worker count", async () => {
      const workers = await initFlowThreadPool(async () => undefined, 4);
      expect(workers).toBeGreaterThanOrEqual(1);
    });

    it("falls back to a single thread when init throws, retrying with 1", async () => {
      let calls = 0;
      const workers = await initFlowThreadPool(async (n: number) => {
        calls++;
        if (calls === 1) throw new Error("simulated non-threaded env");
        expect(n).toBe(1);
      }, 4);
      expect(workers).toBe(1);
      expect(calls).toBe(2);
    });

    it("swallows a second failure (non-threaded wasm / vitest) and still returns 1", async () => {
      const workers = await initFlowThreadPool(async () => {
        throw new Error("always fails");
      }, 4);
      expect(workers).toBe(1);
    });
  });
}
// #endregion 🧪️Tests
