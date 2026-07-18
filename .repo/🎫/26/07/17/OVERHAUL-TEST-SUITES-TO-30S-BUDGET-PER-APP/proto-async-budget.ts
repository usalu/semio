import { spawn } from "node:child_process";

function killTree(pid: number): void {
  if (process.platform === "win32") return;
  try {
    process.kill(-pid, "SIGKILL");
  } catch {
    /* already gone */
  }
}

async function runTestBudgeted(cmd: string, args: string[], budgetMs: number): Promise<void> {
  const child = spawn(cmd, args, { stdio: "inherit", detached: process.platform !== "win32" });
  let timedOut = false;
  const timer = setTimeout(() => {
    timedOut = true;
    if (child.pid) killTree(child.pid);
  }, budgetMs);
  const { code, signal } = await new Promise<{ code: number | null; signal: NodeJS.Signals | null }>((res, rej) => {
    child.on("error", rej);
    child.on("exit", (code, signal) => res({ code, signal }));
  }).finally(() => clearTimeout(timer));
  if (timedOut) {
    console.log("[proto] TIMED OUT, exiting 1");
    process.exit(1);
  }
  if (signal || code !== 0) {
    console.log("[proto] nonzero exit", code, signal);
    process.exit(code ?? 1);
  }
  console.log("[proto] success");
}

// fire-and-forget, exactly like a bare `run(): void { runVitest(...); }` call site
function run(): void {
  const script = `const cp=require("node:child_process"); const w=cp.spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"}); require("node:fs").writeFileSync("/tmp/dbg-worker-pid5.txt", String(w.pid)); setInterval(()=>{},1000);`;
  runTestBudgeted(process.execPath, ["-e", script], 1200);
}

console.log("[proto] module top: calling run() without await");
run();
console.log("[proto] module top: run() returned synchronously, module finishing");
