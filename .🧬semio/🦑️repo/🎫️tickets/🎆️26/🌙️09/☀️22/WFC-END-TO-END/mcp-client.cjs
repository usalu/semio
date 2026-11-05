const { spawn } = require("node:child_process");
const path = require("node:path"), fs = require("node:fs");
let root = __dirname;
while (!fs.existsSync(path.join(root, ".git"))) {
  const parent = path.dirname(root);
  if (parent === root) throw Error("Repository root required");
  root = parent;
}
const windows = process.platform === "win32";
const child = spawn(windows ? process.env.ComSpec || "cmd.exe" : "repo", windows ? ["/d", "/s", "/c", "repo mcp"] : ["mcp"], { cwd: root, stdio: ["pipe", "pipe", "inherit"] });
let buffer = "", id = 0;
const pending = new Map();
child.stdout.on("data", chunk => {
  buffer += chunk;
  let end;
  while ((end = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, end);
    buffer = buffer.slice(end + 1);
    let value;
    try { value = JSON.parse(line); } catch { continue; }
    const wait = pending.get(value.id);
    if (wait) {
      pending.delete(value.id);
      value.error ? wait.reject(Error(JSON.stringify(value.error))) : wait.resolve(value.result);
    }
  }
});
function request(method, params) {
  return new Promise((resolve, reject) => {
    const key = ++id;
    pending.set(key, { resolve, reject });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id: key, method, params }) + "\n");
  });
}
const deadline = setTimeout(() => { child.kill(); process.exitCode = 1; }, 45000);
(async () => {
  try {
    await request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex-wfc", version: "1" } });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
    let result;
    if (process.argv[2] === "tool") result = await request("tools/call", { name: process.argv[3], arguments: JSON.parse(process.argv[4]) });
    else if (process.argv[2] === "list") {
      const listed = await request("tools/list", {});
      result = listed.tools.filter(tool => !process.argv[3] || tool.name.includes(process.argv[3]));
    } else result = await request("resources/read", { uri: process.argv[2] || "repo://goals" });
    process.stdout.write(JSON.stringify(result) + "\n");
  } catch (error) {
    process.stderr.write(error.stack + "\n");
    process.exitCode = 1;
  } finally {
    clearTimeout(deadline);
    child.kill();
  }
})();
