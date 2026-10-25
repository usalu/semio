import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const bin = process.env.SEMIO_OS_MCP_BIN ?? `${root}/${C}/🗑️generated/mcp-b/semio-os-mcp`;
const folder = process.env.MCP_FOLDER ?? ".🧬semio/🔗space/os-mcp";
const pol = `${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json`;
const calls: { name: string; args: unknown }[] = JSON.parse(readFileSync(process.argv[2], "utf8"));
const child = spawn(bin, ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write", "--host-driver-policy", pol, ...(process.env.MCP_EXTRA ? JSON.parse(process.env.MCP_EXTRA) : [])], { cwd: root, stdio: ["pipe", "pipe", "pipe"] });
let buf = ""; const waiters = new Map<number, (m: any) => void>(); let err = "";
child.stderr.on("data", (d) => (err += d));
child.stdout.on("data", (d) => { buf += d; let i; while ((i = buf.indexOf("\n")) >= 0) { const l = buf.slice(0, i).trim(); buf = buf.slice(i + 1); if (!l) continue; try { const m = JSON.parse(l); waiters.get(m.id)?.(m); } catch { console.log("POLLUTION", l.slice(0, 200)); } } });
let id = 1;
const send = (method: string, params?: unknown) => new Promise<any>((res, rej) => { const n = id++; const t = setTimeout(() => rej(new Error("timeout " + method)), 120000); waiters.set(n, (m) => { clearTimeout(t); res(m); }); child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id: n, method, params }) + "\n"); });
const t0 = Date.now();
const init = await send("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "mcp-c", version: "0" } });
console.log("initialize", init.result?.serverInfo, Date.now() - t0, "ms");
child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
for (const c of calls) {
  const t = Date.now();
  const r = await send(c.name === "tools/list" ? "tools/list" : "tools/call", c.name === "tools/list" ? {} : { name: c.name, arguments: c.args });
  console.log("---", c.name, JSON.stringify(c.args), Date.now() - t, "ms");
  console.log(JSON.stringify(r.error ?? r.result, null, 1).slice(0, Number(process.env.MCP_MAX ?? 2500)));
}
console.log("--- stderr\n" + err.slice(0, 3000));
child.kill("SIGTERM"); setTimeout(() => process.exit(0), 1000);
