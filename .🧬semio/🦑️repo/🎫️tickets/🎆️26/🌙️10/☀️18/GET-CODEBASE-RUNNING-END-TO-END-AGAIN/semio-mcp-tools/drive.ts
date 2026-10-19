import { spawn } from "node:child_process";
import { createWriteStream, readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const server = process.argv[2] ?? "semio";
const logPath = process.argv[3] ?? `${import.meta.dir}/../🗑️generated/semio-mcp-${server}.log`;
const callName = process.argv[4];
const callArgs = process.argv[5] ? JSON.parse(process.argv[5]) : {};
const spec = JSON.parse(readFileSync(`${root}/.mcp.json`, "utf8")).mcpServers[server];
const log = createWriteStream(logPath);
const t0 = Date.now();
const child = spawn(spec.command, spec.args, { cwd: root, env: { ...process.env, ...(spec.env ?? {}) }, stdio: ["pipe", "pipe", "pipe"] });
child.stderr.on("data", (d) => log.write(d));
let buf = "";
const waiters = new Map<number, (m: any) => void>();
const stdoutNoise: string[] = [];
child.stdout.on("data", (d) => {
  buf += d.toString();
  let i;
  while ((i = buf.indexOf("\n")) >= 0) {
    const line = buf.slice(0, i).trim();
    buf = buf.slice(i + 1);
    if (!line) continue;
    try {
      const m = JSON.parse(line);
      if (m.id !== undefined && waiters.has(m.id)) waiters.get(m.id)!(m);
      else console.log(`[${Date.now() - t0}ms] notif`, line.slice(0, 200));
    } catch {
      stdoutNoise.push(line);
      console.log("STDOUT-POLLUTION:", line.slice(0, 200));
    }
  }
});
child.on("exit", (c, s) => console.log(`[${Date.now() - t0}ms] EXIT code=${c} signal=${s}`));
let nextId = 1;
const send = (method: string, params?: unknown, timeout = 120000) =>
  new Promise<any>((res, rej) => {
    const id = nextId++;
    const to = setTimeout(() => rej(new Error(`timeout ${method}`)), timeout);
    waiters.set(id, (m) => { clearTimeout(to); res(m); });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
  });
try {
  const init = await send("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "drive", version: "0" } });
  console.log(`[${Date.now() - t0}ms] initialize`, JSON.stringify(init).slice(0, 400));
  child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
  const list = await send("tools/list", {});
  const tools = list.result?.tools ?? [];
  console.log(`[${Date.now() - t0}ms] tools/list`, list.error ? JSON.stringify(list.error) : `${tools.length} tools: ${tools.slice(0, 8).map((t: any) => t.name).join(",")}`);
  if (callName) {
    const r = await send("tools/call", { name: callName, arguments: callArgs });
    console.log(`[${Date.now() - t0}ms] tools/call ${callName}`, JSON.stringify(r).slice(0, 1500));
    const sc = r.result?.structuredContent;
    console.log("structuredContent:", sc === undefined ? "omitted" : sc === null ? "NULL (invalid)" : typeof sc);
  }
} catch (e) {
  console.log("FAIL", String(e));
}
console.log("stdout pollution lines:", stdoutNoise.length);
child.kill();
setTimeout(() => process.exit(0), 200);
