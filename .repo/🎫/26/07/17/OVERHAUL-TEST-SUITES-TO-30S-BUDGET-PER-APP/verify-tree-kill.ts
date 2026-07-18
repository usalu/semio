import { runTestBudgeted } from "/Users/ueli/Documents/semio/repo/lib/js/index.ts";

// Parent forks a grandchild worker that ignores SIGTERM-style politeness and just burns CPU.
// If only the direct child is killed, the grandchild's pid (printed to a file) stays alive.
const script = `
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const worker = spawn(process.execPath, ["-e", "setInterval(() => {}, 1000);"], { detached: false, stdio: "ignore" });
fs.writeFileSync("${"/tmp/semio-tree-kill-worker-pid.txt"}", String(worker.pid));
setInterval(() => {}, 1000);
`;

try {
  runTestBudgeted(process.execPath, ["-e", script], { budgetMs: 1500 });
} catch {
  // runTestBudgeted calls process.exit(1) on timeout, so this catch is unreachable in the timeout path.
}
