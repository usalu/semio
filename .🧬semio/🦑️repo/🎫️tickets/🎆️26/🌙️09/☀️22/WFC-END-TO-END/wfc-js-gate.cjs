/** 🧪️ Streams the independent registered WFC JavaScript gate into ticket-owned evidence. */
const fs = require("node:fs");
const path = require("node:path");
const { spawn } = require("node:child_process");
const ticket = __dirname;
const output = path.join(ticket, "🗑️generated", "registered-js-r3.log");
fs.mkdirSync(path.dirname(output), { recursive: true });
const log = fs.openSync(output, "wx");
const command = ["run", "@semio-tech/wfc-js:test", "--lang=en"];
const write = bytes => { fs.writeSync(log, bytes); process.stdout.write(bytes); };
write("[DEBUG] registered WFC JS gate begin " + new Date().toISOString() + "\n");
const child = spawn(process.platform === "win32" ? "repo.exe" : "repo", command, { stdio: ["inherit", "pipe", "pipe"], windowsHide: true });
process.on("SIGINT", () => write("[DEBUG] original console cancellation requested; awaiting gate settlement\n"));
child.stdout.on("data", write);
child.stderr.on("data", write);
child.on("error", error => write("[DEBUG] registered gate spawn failure: " + error.message + "\n"));
child.on("close", (code, signal) => {
    write("[DEBUG] registered WFC JS gate terminal code=" + code + " signal=" + signal + " at=" + new Date().toISOString() + "\n");
    fs.closeSync(log);
    process.exitCode = code ?? 1;
});
