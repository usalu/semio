import { spawn } from "node:child_process";
const [command, ...args] = process.argv.slice(2);
if (!command) throw new Error("Expected an executable and its arguments");
const child = spawn(command, args, { cwd: process.env.TASK_CWD || process.cwd(), env: process.env, stdio: "inherit", windowsHide: true });
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => child.kill(signal));
child.on("error", error => { console.error("[DEBUG]", error.message); process.exitCode = 1; });
child.on("exit", code => { process.exitCode = code ?? 1; });
