import { spawn } from "node:child_process";
import { openSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const output = join(ticket, "🗑️generated", "g");
mkdirSync(output, { recursive: true });
const log = openSync(join(output, "controller.log"), "a");
const python = process.argv[2];
if (!python) throw new Error("Expected the active Python executable");
const child = spawn(python, [resolve("🎓️teaching/render_videos.py"), "--output-root", output,
  "--jobs", "14", "--log-output", "--force", "--no-play"], {
  stdio: ["ignore", log, log], windowsHide: true,
  env: { ...process.env, SEMIO_TEACHING_OUTPUT_ROOT: output, PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1" },
});
console.log(`[DEBUG] Corrected stable controller PID ${child.pid}; log ${join(output, "controller.log")}`);
child.on("error", error => { console.error(error); process.exitCode = 1; });
child.on("exit", code => { process.exitCode = code ?? 1; });
