const { spawnSync } = require("node:child_process");
const { join } = require("node:path");
const bundled = "C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe";
const ticket = __dirname;
const script = process.argv[2];
if (!["convert-assignment-icons.py", "verify-assignment-icons.py"].includes(script)) throw Error("Authored icon operation required");
const result = spawnSync("repo", ["tools", "exec", "--light", "python", "--", join(ticket, script), "--library", join(ticket, "🗑️generated/python")], { cwd: process.cwd(), env: { ...process.env, SEMIO_TOOL_PYTHON: bundled }, stdio: "inherit", shell: process.platform === "win32" });
process.exit(result.status ?? 1);
