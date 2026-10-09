import { spawn } from "node:child_process";

const child = spawn(".🧬semio/🦑️repo/⚡️cache/🗃️bin/repo.exe", [], {
  cwd: "C:/git/semio",
  env: { ...process.env, SEMIO_REPO_MCP_CLIENT: "cursor" },
  stdio: ["pipe", "pipe", "pipe"],
});

let buffer = "";
const pending = new Map();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let newline = buffer.indexOf("\n");
  while (newline >= 0) {
    const line = buffer.slice(0, newline);
    buffer = buffer.slice(newline + 1);
    newline = buffer.indexOf("\n");
    if (!line.trim()) continue;
    const message = JSON.parse(line);
    if (message.id !== undefined && pending.has(message.id)) pending.get(message.id)(message);
  }
});
child.stderr.on("data", (chunk) => process.stderr.write(chunk));

let nextId = 1;
function request(method, params) {
  const id = nextId++;
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`timeout ${method}`)), 60000);
    pending.set(id, (message) => {
      clearTimeout(timer);
      pending.delete(id);
      resolve(message);
    });
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  });
}
function notify(method, params) {
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method, params })}\n`);
}

const init = await request("initialize", {
  protocolVersion: "2024-11-05",
  capabilities: {},
  clientInfo: { name: "cursor-chat", version: "1" },
});
process.stdout.write(`INIT ${JSON.stringify(init)}\n`);
notify("notifications/initialized");
const goals = await request("resources/read", { uri: "repo://goals" });
process.stdout.write(`GOALS ${JSON.stringify(goals)}\n`);
const reopened = await request("tools/call", {
  name: "ticket_reopen",
  arguments: {
    path: "26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY",
    prompt: "Get teaching architecture frontend and backend ready to be redeployed. Everything end to end. Tested, completely bug free, fault tolerant.",
    client: "cursor-chat",
    llm: "grok-4.7",
    goal: "🎯runningframework🎯runningproducts",
    no_management: true,
  },
});
process.stdout.write(`REOPEN ${JSON.stringify(reopened)}\n`);
child.stdin.end();
child.kill();
process.exit(0);
