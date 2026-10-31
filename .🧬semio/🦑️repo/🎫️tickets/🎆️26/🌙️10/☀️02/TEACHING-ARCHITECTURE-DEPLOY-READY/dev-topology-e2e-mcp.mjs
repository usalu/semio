import { spawn } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";

const outDir = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated";
mkdirSync(outDir, { recursive: true });

const child = spawn("repo", ["mcp"], {
  cwd: "/home/deck/semio",
  env: { ...process.env, SEMIO_REPO_MCP_CLIENT: "cursor", SEMIO_LANGUAGE: "en" },
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
    let message;
    try { message = JSON.parse(line); } catch { process.stderr.write(`NONJSON ${line}\n`); continue; }
    if (message.id !== undefined && pending.has(message.id)) pending.get(message.id)(message);
  }
});
child.stderr.on("data", (chunk) => process.stderr.write(chunk));

let nextId = 1;
function request(method, params) {
  const id = nextId++;
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`timeout ${method}`)), 90000);
    pending.set(id, (message) => { clearTimeout(timer); pending.delete(id); resolve(message); });
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
notify("notifications/initialized", {});
let ping = null;
for (let attempt = 0; attempt < 20; attempt++) {
  ping = await request("ping", {});
  if (!ping.error) break;
  await new Promise((resolve) => setTimeout(resolve, 50));
}
const goals = await request("resources/read", { uri: "repo://goals" });
const reopened = await request("tools/call", {
  name: "ticket_reopen",
  arguments: {
    path: "26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY",
    prompt: "Run the complete dev topology end-to-end suite, both shared and exclusive waves, all 85 tests across 9 projects and 16 specs.",
    client: "cursor-chat",
    llm: "grok-4.7",
    goal: "🎯runningframework🎯runningproducts",
    no_management: true,
  },
});
const goalBody = goals.result?.contents?.[0]?.text ?? JSON.stringify(goals);
const summary = {
  initOk: !init.error,
  pingOk: !ping?.error,
  goalBytes: String(goalBody).length,
  goalHead: String(goalBody).slice(0, 2500),
  reopen: reopened.result ?? reopened.error,
};
writeFileSync(`${outDir}/dev-topology-e2e-mcp.json`, JSON.stringify(summary, null, 2));
process.stdout.write(`${JSON.stringify({ initOk: summary.initOk, pingOk: summary.pingOk, goalBytes: summary.goalBytes, reopen: summary.reopen })}\n`);
child.stdin.end();
setTimeout(() => { child.kill(); process.exit(0); }, 200);
