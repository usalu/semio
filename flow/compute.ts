/** @emoji 🧵 Flow compute thread-pool helpers (wasm-bindgen-rayon). */

export {
  defaultComputeWorkerCount,
  effectiveComputeWorkerCount,
  isCrossOriginIsolatedRuntime,
  readStoredComputeWorkerCount,
  UI_COMPUTE_WORKER_COUNT_STORAGE_KEY,
  writeStoredComputeWorkerCount,
} from "@semio-tech/ui-react";

export type FlowThreadPoolInit = (numThreads: number) => Promise<unknown>;

/** Initialize the flow_core rayon thread pool when cross-origin isolated. */
export async function initFlowThreadPool(init: FlowThreadPoolInit, requested = readStoredComputeWorkerCount()): Promise<number> {
  const workers = effectiveComputeWorkerCount(requested);
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
