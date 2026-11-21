import { spawn } from "node:child_process";
import { mkdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createInterface } from "node:readline";

const root = process.cwd();
const name = process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp";
const binary = resolve(import.meta.dir, "🗑️generated/native-mcp/debug", name);
const folder = resolve(import.meta.dir, "🗑️generated/mcp-smoke");
mkdirSync(folder, { recursive: true });
const config = JSON.parse(readFileSync(resolve(root, ".mcp.json"), "utf8")).mcpServers.semio;
const supplied: string[] = config.args.slice(4);
const take = (flag: string) => {
  const at = supplied.indexOf(flag);
  if (at < 0) throw new Error(`Configured launch lacks ${flag}`);
  return supplied.splice(at, 2)[1]!;
};
const catalog = resolve(root, take("--catalog"));
take("--catalog-target");
take("--folder");
const hostPolicy = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json");
const child = spawn(binary, ["stdio", ...supplied, "--folder", folder, "--host-driver-policy", hostPolicy], {
  cwd: root,
  env: { ...process.env, ...config.env, SEMIO_PLUGIN_CATALOG: catalog },
  stdio: ["pipe", "pipe", "inherit"],
});
const lines = createInterface({ input: child.stdout });
const exited = new Promise<number | null>((done, error) => {
  child.once("close", done);
  child.once("error", error);
});
void exited.catch(() => {});
const received = lines[Symbol.asyncIterator]();
const send = (method: string, params: object, id?: number) => child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method, params, id }) + "\n");
const wait = setTimeout(() => child.kill("SIGKILL"), 120_000);
try {
  send("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "semio-native-smoke", version: "1" } }, 1);
  const initialize = await received.next();
  const hello = initialize.done ? null : JSON.parse(initialize.value);
  if (hello?.id !== 1 || !hello.result?.serverInfo?.name) throw new Error(`Native initialize failed: ${JSON.stringify(hello)}`);
  send("notifications/initialized", {});
  const checks = new Map([[2, ["tools/list", "tools"]], [3, ["resources/list", "resources"]], [4, ["prompts/list", "prompts"]]]);
  for (const [id, [method]] of checks) send(method, {}, id);
  while (checks.size) {
    const next = await received.next();
    if (next.done) throw new Error("Native server closed during discovery");
    const message = JSON.parse(next.value);
    const check = checks.get(message.id);
    if (!check) continue;
    const rows = message.result?.[check[1]];
    if (!Array.isArray(rows) || !rows.length) throw new Error(`Native ${check[0]} failed: ${JSON.stringify(message)}`);
    console.error(`[DEBUG] Native ${check[0]} returned ${rows.length} entries`);
    checks.delete(message.id);
  }
} finally {
  lines.close();
  child.stdin.end();
  const status = await exited.finally(() => clearTimeout(wait));
  if (status !== 0) throw new Error(`Native stdio exited with status ${status}`);
}
