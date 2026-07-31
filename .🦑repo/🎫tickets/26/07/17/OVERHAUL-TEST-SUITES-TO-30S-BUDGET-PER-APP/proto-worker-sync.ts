import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";
import { spawn } from "node:child_process";

function killTree(pid: number): void {
  if (process.platform === "win32") return;
  try {
    process.kill(-pid, "SIGKILL");
  } catch {
    /* already gone */
  }
}

if (isMainThread) {
  const sab = new SharedArrayBuffer(8);
  const state = new Int32Array(sab); // [0] = done flag, [1] = exit status (or -1 for timeout)
  const worker = new Worker(new URL(import.meta.url), { workerData: { sab } });
  worker.on("error", (e) => console.error("worker error", e));

  console.log("main: blocking via Atomics.wait...");
  const res = Atomics.wait(state, 0, 0, 5000);
  console.log("main: Atomics.wait returned:", res, "state:", state[0], state[1]);
  worker.terminate();

  setTimeout(() => {
    const wpid = Number(require("node:fs").readFileSync("/tmp/dbg-worker-pid4.txt", "utf8"));
    console.log("checking worker pid", wpid, "alive:", (() => {
      try { process.kill(wpid, 0); return true; } catch { return false; }
    })());
    process.exit(0);
  }, 500);
} else {
  const { sab } = workerData as { sab: SharedArrayBuffer };
  const state = new Int32Array(sab);
  const budgetMs = 1200;
  const script = `const cp=require("node:child_process"); const w=cp.spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"}); require("node:fs").writeFileSync("/tmp/dbg-worker-pid4.txt", String(w.pid)); setInterval(()=>{},1000);`;
  const child = spawn(process.execPath, ["-e", script], { stdio: "inherit", detached: true });
  const timer = setTimeout(() => {
    console.log("worker-thread: timeout, killing tree", child.pid);
    if (child.pid) killTree(child.pid);
  }, budgetMs);
  child.on("exit", (code, signal) => {
    clearTimeout(timer);
    Atomics.store(state, 0, 1);
    Atomics.store(state, 1, code ?? -1);
    Atomics.notify(state, 0);
    parentPort?.postMessage({ code, signal });
  });
}
