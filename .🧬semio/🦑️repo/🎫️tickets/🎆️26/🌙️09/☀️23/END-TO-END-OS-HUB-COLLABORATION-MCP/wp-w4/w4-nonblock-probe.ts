/** 🔬️ W4 (14c): does a Bun-spawned child inherit a NON-BLOCKING stderr? `inherit` vs `pipe` + forward, byte-exact under a slow reader. */
import { spawn } from "node:child_process";
const [mode, touch] = [process.argv[2] as "inherit" | "pipe", process.argv[3] === "touch"];
const burst = "import sys\ntry:\n  sys.stderr.write('x'*4000000); sys.stderr.flush(); sys.stdout.write('burst ok\\n')\nexcept Exception as e:\n  sys.stdout.write('burst FAILED %r\\n' % e)";
if (touch) process.stderr.write("");
const child = spawn("python3", ["-c", burst], { stdio: ["inherit", "pipe", mode] });
child.stdout!.on("data", (d) => process.stdout.write(`${mode}: ${d}`));
if (mode === "pipe") child.stderr!.pipe(process.stderr, { end: false });
child.on("close", () => process.stdout.write(`${mode}: child closed\n`));
