/** 🧵️ A bounded pool of worker threads that runs pure, module-addressed tasks with progress and cancellation; it needs no library outside the runtime and speaks only the plain-data protocol of `./🧬️schema/🔣️.json`. */
import { Worker } from "node:worker_threads";
import { availableParallelism } from "node:os";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = Object.freeze({ ...schema.definitions.Policy.const });
const WORKER = new URL("./👷️worker/🟨️.mjs", import.meta.url);
const NAME = new RegExp(schema.definitions.Task.properties.export.pattern, "u");

/** 🛑️ The reason a cancelled pool or task rejects with, preserving the caller's own reason. */
function cancellation(signal) {
  return signal?.reason ?? Object.assign(new Error("Worker pool cancelled"), { name: "AbortError" });
}

/** 🧾️ Admits only a file-addressed exported function name, so a task is data a worker can resolve and nothing else. */
function requireTask(task) {
  if (!task || typeof task !== "object" || typeof task.module !== "string" || !task.module.startsWith("file:///") || typeof task.export !== "string" || task.export.length > policy.maximumNameCharacters || !NAME.test(task.export)) throw new Error("Closed worker task required");
  return task;
}

/**
 * 🏊️ Creates a pool. `run` executes one task, `map` many in order; both reject with the cancellation reason once `signal` aborts, and a worker that crashes
 * rejects only its own task and is replaced. `onProgress({ completed, total })` reports finished tasks of the pool and, per task, the task's own
 * `progress(completed, total)` calls through `onTaskProgress`.
 * @param {{ workers?: number, signal?: AbortSignal, onProgress?: (row: { completed: number, total: number }) => void, onTaskProgress?: (row: { task: number, completed: number, total: number }) => void }} [options]
 */
export function createWorkerPoolV1({ workers = Math.min(policy.defaultWorkers, Math.max(1, availableParallelism())), signal, onProgress, onTaskProgress } = {}) {
  if (!Number.isSafeInteger(workers) || workers < 1 || workers > policy.maximumWorkers) throw new Error("Worker pool size must be an integer from 1 to " + policy.maximumWorkers);
  const slots = Array.from({ length: workers }, () => ({ worker: undefined, job: undefined })), queue = [];
  let closed = false, nextId = 0, accepted = 0, finished = 0, reason;

  const settle = (job, method, value) => {
    if (job.settled) return;
    job.settled = true;
    job.detach?.();
    method(value);
    if (method === job.resolve || method === job.reject) { finished++; onProgress?.({ completed: finished, total: accepted }); }
  };
  const retire = (slot) => {
    const worker = slot.worker;
    slot.worker = undefined;
    slot.job = undefined;
    if (worker) { worker.removeAllListeners(); worker.terminate().catch(() => {}); }
  };
  const spawn = (slot) => {
    const worker = new Worker(WORKER);
    slot.worker = worker;
    worker.on("message", (message) => {
      const job = slot.job;
      if (!job || message?.id !== job.id) return;
      if (message.type === "progress") { onTaskProgress?.({ task: job.id, completed: message.completed, total: message.total }); return; }
      slot.job = undefined;
      if (message.type === "result") settle(job, job.resolve, message.value);
      else settle(job, job.reject, Object.assign(new Error(message.message), { name: message.name }));
      pump();
    });
    const lost = (error) => {
      const job = slot.job;
      retire(slot);
      if (job) settle(job, job.reject, error instanceof Error ? error : new Error("Worker exited before its task finished"));
      pump();
    };
    worker.once("error", lost);
    worker.once("exit", () => lost(undefined));
  };
  const pump = () => {
    if (closed) return;
    for (const slot of slots) {
      if (slot.job || !queue.length) continue;
      const job = queue.shift();
      if (job.settled) continue;
      if (!slot.worker) spawn(slot);
      slot.job = job;
      slot.worker.postMessage({ type: "run", id: job.id, module: job.task.module, export: job.task.export, input: job.input });
    }
  };
  const cancelAll = (error) => {
    closed = true;
    reason = error;
    for (const job of queue.splice(0)) settle(job, job.reject, error);
    for (const slot of slots) { const job = slot.job; retire(slot); if (job) settle(job, job.reject, error); }
  };
  if (signal) {
    if (signal.aborted) cancelAll(cancellation(signal));
    else signal.addEventListener("abort", () => cancelAll(cancellation(signal)), { once: true });
  }

  /** ▶️ Runs one task; `options.signal` cancels only this task (its worker is terminated and replaced). */
  const run = (task, input, options = {}) => {
    try { requireTask(task); } catch (error) { return Promise.reject(error); }
    if (closed) return Promise.reject(reason ?? new Error("Worker pool closed"));
    if (queue.length >= policy.maximumQueuedTasks) return Promise.reject(new Error("Worker pool queue capacity exceeded"));
    return new Promise((resolve, reject) => {
      const job = { id: nextId++, task, input, resolve, reject, settled: false, detach: undefined };
      accepted++;
      const own = options.signal;
      if (own) {
        const cancel = () => {
          const slot = slots.find((candidate) => candidate.job === job);
          const at = queue.indexOf(job);
          if (at >= 0) queue.splice(at, 1);
          if (slot) retire(slot);
          settle(job, reject, cancellation(own));
          pump();
        };
        if (own.aborted) { settle(job, reject, cancellation(own)); return; }
        own.addEventListener("abort", cancel, { once: true });
        job.detach = () => own.removeEventListener("abort", cancel);
      }
      queue.push(job);
      pump();
    });
  };
  /** 🗺️ Runs one task per input and keeps the input order; the first failure rejects and cancels the rest of this call. */
  const map = async (task, inputs, options = {}) => {
    const own = new AbortController(), forward = () => own.abort(cancellation(options.signal));
    options.signal?.addEventListener("abort", forward, { once: true });
    try { return await Promise.all(inputs.map((input) => run(task, input, { signal: own.signal }))); }
    catch (error) { own.abort(error); throw error; }
    finally { options.signal?.removeEventListener("abort", forward); }
  };
  /** 🔚️ Terminates every worker; pending tasks reject. */
  const close = async () => { cancelAll(new Error("Worker pool closed")); };
  return Object.freeze({ run, map, close, size: workers });
}
