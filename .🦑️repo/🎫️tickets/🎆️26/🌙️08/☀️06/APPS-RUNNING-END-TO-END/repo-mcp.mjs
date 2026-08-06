import { spawn } from "child_process";
import { writeFileSync, readFileSync, readdirSync, existsSync, mkdirSync, copyFileSync } from "fs";
import { join } from "path";

const ROOT = process.cwd();

function ticketBase() {
  const ticketsRoot = join(ROOT, ".🦑️repo", "🎫️tickets");
  const year = readdirSync(ticketsRoot).find((x) => /26/.test(x) && x.length > 2);
  const month = readdirSync(join(ticketsRoot, year)).find((x) => /08/.test(x) && x.length > 2);
  const day = readdirSync(join(ticketsRoot, year, month)).find((x) => /06/.test(x) && x.length > 2);
  return join(ticketsRoot, year, month, day);
}

export function ticketDir(slug) {
  return join(ticketBase(), slug);
}

export async function withRepoMcp(fn) {
  const child = spawn("bun", ["./📜️script.ts", "dev", "mcp", "stdio", "cursor"], {
    cwd: ROOT,
    stdio: ["pipe", "pipe", "pipe"],
  });
  let buf = "";
  const pending = new Map();
  child.stdout.on("data", (d) => {
    buf += d.toString();
    let idx;
    while ((idx = buf.indexOf("\n")) >= 0) {
      const line = buf.slice(0, idx);
      buf = buf.slice(idx + 1);
      if (!line.trim()) continue;
      try {
        const msg = JSON.parse(line);
        if (msg.id != null && pending.has(msg.id)) pending.get(msg.id)(msg);
      } catch {}
    }
  });
  let nextId = 1;
  const call = (method, params) => {
    const id = nextId++;
    return new Promise((resolve, reject) => {
      const t = setTimeout(() => reject(new Error(`timeout ${method}`)), 180000);
      pending.set(id, (msg) => {
        clearTimeout(t);
        pending.delete(id);
        if (msg.error) reject(Object.assign(new Error(JSON.stringify(msg.error)), msg.error));
        else resolve(msg.result);
      });
      child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
    });
  };
  try {
    await call("initialize", {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: { name: "agent", version: "0.0.1" },
    });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
    const tool = (name, args) => call("tools/call", { name, arguments: args });
    const resource = (uri) => call("resources/read", { uri });
    return await fn({ call, tool, resource });
  } finally {
    child.kill();
  }
}

if (import.meta.main) {
  const [cmd, ...rest] = process.argv.slice(2);
  if (cmd === "close-duplicate") {
    const result = await withRepoMcp(({ tool }) =>
      tool("ticket_close", {
        path: "26/08/06/APPS-RUNNING-END-TO-END-AFTER-RESTRUCTURE",
        summary:
          "Duplicate of open ticket 26/08/06/APPS-RUNNING-END-TO-END covering the same apps E2E work. Continuing on the existing ticket.",
        files: [],
        no_management: false,
      }),
    );
    console.log(JSON.stringify(result, null, 2));
  } else if (cmd === "base") {
    console.log(ticketBase());
  } else {
    console.error("usage: close-duplicate | base");
    process.exit(1);
  }
}
