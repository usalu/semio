import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import fixture from "./stdio-discovery-fixture.json" with { type: "json" };

const workspace = process.cwd();
const [native, selected] = process.argv.slice(2);
const generated = join(import.meta.dirname, "🗑️generated");
await mkdir(generated, { recursive: true });
const folder = await mkdtemp(join(generated, "sdk-"));
const config = JSON.parse(await readFile(join(workspace, ".mcp.json"), "utf8")).mcpServers;
const nativeOptions = [
  "stdio",
  "--host-driver-policy", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json",
  "--operation-policy", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/💡️host-operation-policy.json",
];
const methods = { "tools/list": "listTools", "resources/list": "listResources", "prompts/list": "listPrompts" } as const;
const environment = Object.fromEntries(Object.entries(process.env).flatMap(([key, value]) => value === undefined ? [] : [[key, value]]));
try {
  const results = await Promise.allSettled(Object.entries(fixture).filter(([server]) => !selected || selected === server).map(async ([server, checks]) => {
    const direct = server === "semio" && native && native !== "-";
    const entry = config[server];
    const args = [...(direct ? nativeOptions : entry.args), ...(server === "semio" ? ["--folder", folder, "--no-bridge"] : [])];
    const transport = new StdioClientTransport({
      command: direct ? resolve(native) : entry.command,
      args,
      cwd: workspace,
      env: {
        ...environment,
        ...entry.env,
        ...(direct ? { SEMIO_PLUGIN_CATALOG: resolve(entry.args[entry.args.indexOf("--catalog") + 1]) } : {}),
      },
      stderr: "pipe",
    });
    transport.stderr?.on("data", chunk => process.stderr.write(chunk));
    const client = new Client({ name: "semio-stdio-sdk-verification", version: "1" });
    const timeout = { timeout: 60_000 };
    const started = performance.now();
    try {
      await client.connect(transport, timeout);
      console.log(`[DEBUG] ${server} connected in ${Math.round(performance.now() - started)} ms`);
      await Promise.all(checks.map(async check => {
        const method = methods[check.method as keyof typeof methods];
        assert(method, check.method);
        const result = await client[method]({}, timeout);
        const collection = result[check.collection] as Record<string, unknown>[];
        const identities = collection.map(row => row[check.identity]);
        assert(check.expected.every(identity => identities.includes(identity)), `${server} ${check.method} missing ${check.expected.filter(identity => !identities.includes(identity)).join(", ")}`);
        console.log(`[DEBUG] ${server} ${check.method} discovered ${identities.length} items in ${Math.round(performance.now() - started)} ms`);
      }));
      if (server === "repo") {
        const result = await client.readResource({ uri: "repo://goals" }, timeout);
        assert(result.contents.length > 0);
        console.log(`[DEBUG] repo read repo://goals (${result.contents.length} content item)`);
      }
    } finally {
      await client.close();
    }
  }));
  const failures = results.filter(result => result.status === "rejected");
  if (failures.length) throw new AggregateError(failures.map(failure => failure.reason), "MCP discovery verification failed");
} finally {
  await rm(folder, { recursive: true, force: true });
}
