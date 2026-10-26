import { spawn } from "node:child_process";

const [command, ...args] = process.argv.slice(2);
if (!command) throw new Error("Expected a command");
const child = spawn(command, args, { stdio: "inherit", env: process.env });
process.on("SIGINT", () => child.kill("SIGINT"));
child.on("error", error => { console.error(error); process.exitCode = 1; });
child.on("exit", code => { process.exitCode = code ?? 1; });
