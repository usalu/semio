#!/usr/bin/env bun
/** 🔎️ A1 probe: which plugin capability actually survives `action_prepare` over the live os MCP
 * gateway? Spawns the real `semio-os-mcp` stdio binary bound to the repo root, reads the compiled
 * catalog, then tries `action_prepare` per candidate capability and prints the verdict per plugin.
 *
 * `bun .🧬semio/…/🐍️a1-capability-probe.ts [pluginId…]`
 */
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";

const REPO_ROOT = "/Users/ueli/Documents/semio";
const BIN = `${REPO_ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`;

const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control";
const child = spawn(BIN, ["stdio", "--folder", REPO_ROOT, "--scopes", SCOPES], { cwd: REPO_ROOT, stdio: ["pipe", "pipe", "pipe"] });
const pending = new Map<number, (value: any) => void>();
let nextId = 1;
createInterface({ input: child.stdout }).on("line", (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;
  const envelope = JSON.parse(trimmed);
  const resolve = pending.get(envelope.id);
  if (resolve) {
    pending.delete(envelope.id);
    resolve(envelope);
  }
});
child.stderr.on("data", () => {});

function request(method: string, params: unknown, budgetMs = 120_000): Promise<any> {
  const id = nextId++;
  const answered = new Promise<any>((resolve, reject) => {
    pending.set(id, resolve);
    setTimeout(() => {
      if (pending.delete(id)) reject(new Error(`${method} timed out`));
    }, budgetMs).unref?.();
  });
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  return answered;
}

const call = async (name: string, args: Record<string, unknown>): Promise<any> => (await request("tools/call", { name, arguments: args })).result ?? {};

await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "a1-capability-probe", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} })}\n`);

const wanted = process.argv.slice(2);
const byPlugin = new Map<string, Array<{ id: string; kind: string }>>();
for (const plugin of wanted) {
  const found = await call("capabilities_search", { query: "set add create edit", owner: plugin });
  const rows = ((found.structuredContent?.results ?? []) as Array<Record<string, any>>).filter((row) => String(row.capabilityId).includes("#editor.")).map((row) => ({ id: String(row.capabilityId), kind: String(row.kind) }));
  byPlugin.set(plugin, rows);
}
console.log(`${byPlugin.size} plugin(s), ${[...byPlugin.values()].reduce((sum, rows) => sum + rows.length, 0)} editor action(s)`);

/** 🧪️ Smallest input a capability's own JSON Schema admits — every required property, typed. */
function minimalInput(schema: any): Record<string, unknown> {
  const properties = (schema?.properties ?? {}) as Record<string, any>;
  const input: Record<string, unknown> = {};
  for (const name of (schema?.required ?? []) as string[]) {
    const property = properties[name] ?? {};
    const type = Array.isArray(property.type) ? property.type[0] : property.type;
    if (Array.isArray(property.enum) && property.enum.length > 0) input[name] = property.enum[0];
    else if (type === "number" || type === "integer") input[name] = 1;
    else if (type === "boolean") input[name] = false;
    else if (type === "array") input[name] = [];
    else if (type === "object") input[name] = {};
    else input[name] = `a1-${name}`;
  }
  return input;
}

for (const [plugin, rows] of byPlugin) {
  let verdict = "no candidate";
  for (const row of rows.slice(0, 8)) {
    const described = await call("capabilities_describe", { capabilityId: row.id });
    const schema = described.structuredContent?.inputSchema ?? described.structuredContent?.capability?.inputSchema ?? {};
    const input = minimalInput(schema);
    const prepared = await call("action_prepare", { capabilityId: row.id, input });
    const preparedDetail = prepared.isError === true ? `${prepared.structuredContent?.code}: ${String(prepared.structuredContent?.message).slice(0, 140)}` : `prepared`;
    let invokeDetail = "";
    if (prepared.isError !== true) {
      const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
      invokeDetail = invoked.isError === true ? ` | invoke ${invoked.structuredContent?.code}: ${String(invoked.structuredContent?.message).slice(0, 140)}` : ` | INVOKED status=${invoked.structuredContent?.status} undoToken=${invoked.structuredContent?.undoToken}`;
      if (invoked.isError !== true) {
        console.log(`  ${plugin} ${row.id} input=${JSON.stringify(input).slice(0, 80)} → ${preparedDetail}${invokeDetail}`);
        verdict = `GREEN via ${row.id}`;
        break;
      }
    }
    console.log(`  ${plugin} ${row.id} input=${JSON.stringify(input).slice(0, 80)} → ${preparedDetail}${invokeDetail}`);
    verdict = `${preparedDetail}${invokeDetail}`;
  }
  console.log(`${plugin}: ${verdict}`);
}

child.kill();
