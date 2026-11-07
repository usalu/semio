const { spawn } = require("node:child_process");
const { createInterface } = require("node:readline");
const { readFileSync } = require("node:fs");

async function main() {
  const child = spawn(process.env.SEMIO_NATIVE_REPO ?? (process.platform === "win32" ? JSON.parse(readFileSync(require("node:path").join(process.env.LOCALAPPDATA, "semio/repo/installed.json"), "utf8")).executable : "repo"), ["mcp"], { stdio: ["pipe", "pipe", "inherit"], windowsHide: true });
  const pending = new Map();
  const lines = createInterface({ input: child.stdout });
  let id = 0;
  lines.on("line", line => {
    const message = JSON.parse(line);
    if (message.id !== undefined) pending.get(message.id)?.(message);
  });
  const call = (method, params) => new Promise((resolve, reject) => {
    const request = ++id;
    const timer = setTimeout(() => reject(new Error(`Timeout: ${method}`)), 30000);
    pending.set(request, message => { clearTimeout(timer); pending.delete(request); resolve(message); });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id: request, method, params }) + "\n");
  });
  try {
    await call("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex", version: "1" } });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
    const operation = process.argv[2] ?? "resources/read";
    const argument = process.argv[3];
    const params = argument ? JSON.parse(argument.startsWith("@") ? readFileSync(argument.slice(1), "utf8") : argument) : { uri: "repo://goals" };
    const response = await call(operation, params);
    if (operation === "tools/list") response.result.tools = response.result.tools.filter(tool => new RegExp(process.argv[4] ?? "run|test|tools_exec").test(tool.name));
    console.log(JSON.stringify(response));
  } finally {
    child.stdin.end();
    lines.close();
  }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
