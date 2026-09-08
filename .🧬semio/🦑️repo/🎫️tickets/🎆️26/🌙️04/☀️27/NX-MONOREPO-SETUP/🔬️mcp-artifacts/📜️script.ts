import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", `mcp-restore-${Date.now()}`);
mkdirSync(evidence, { recursive: true });
const output = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build");
const snapshot = (): unknown => Object.fromEntries(readdirSync(output).sort().map((path) => [path, { hash: createHash("sha256").update(readFileSync(join(output, path))).digest("hex"), mode: lstatSync(join(output, path)).mode & 0o777 }]));
const run = async (name: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/framework-os-mcp-rs:build", "--output-style=stream"], { cwd: root, env: { ...process.env, npm_lifecycle_event: "", npm_lifecycle_script: "" }, stdout: "pipe", stderr: "pipe" });
  const [out, err, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, name + ".log"), out + err);
  assert.equal(code, 0, out + err); return out;
};
await run("warm");
const before = snapshot(), backup = join(evidence, "backup");
renameSync(output, backup);
try {
  const restored = await run("restore");
  assert.match(restored, /local cache|existing outputs match the cache/);
  assert.deepEqual(snapshot(), before);
} finally {
  if (!existsSync(output)) renameSync(backup, output);
}
const executable = join(output, "semio-os-mcp" + (process.platform === "win32" ? ".exe" : ""));
const transport = new StdioClientTransport({ command: executable, args: ["stdio"], stderr: "pipe" });
const client = new Client({ name: "nx-artifact-consumer", version: "1.0.0" }, { capabilities: {} });
try {
  await client.connect(transport);
  assert.equal(client.getServerVersion()?.name, "semio-os-mcp");
  const first = await client.listTools(), second = await client.listTools();
  assert.deepEqual(second, first);
  assert.ok(Array.isArray(first.tools));
  const report = { server: client.getServerVersion(), toolCount: first.tools.length, artifacts: before };
  writeFileSync(join(evidence, "consumer.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(ticket, "📓️mcp-artifacts.md"), "# OS MCP Artifact Restoration\n\nThe fixed debug executable restored from Nx cache with identical bytes and modes after its entire staged output directory was removed. The independent installed MCP SDK connected to the restored executable over stdio, verified server identity, and observed identical tools/list results twice. Compiler target state was not consulted by the consumer.\n\n" + JSON.stringify(report, null, 2) + "\n");
  console.log("[DEBUG] Nx MCP restore, executable modes and independent SDK consumer: PASS");
} finally { await client.close(); }
