import assert from "node:assert/strict";
import { lstat, mkdir, mkdtemp, readdir, readFile, rm } from "node:fs/promises";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { getDefaultEnvironment, StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { repositoryCachePartitionDirectoryV1 } from "@semio-tech/repo-lib/storage";

const workspace = process.cwd();
const generated = join(import.meta.dir, "🗑️generated");
const requireAbsentProducts = process.env.SEMIO_MCP_TEST_ABSENT_PRODUCTS === "1";
const products = repositoryCachePartitionDirectoryV1(workspace, "products");
if (requireAbsentProducts) await assert.rejects(lstat(products), { code: "ENOENT" });
const fixture = JSON.parse(await readFile(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔄️end-to-end/🧫️fixtures/🔣️.json"), "utf8"));
const kind = process.argv[2] ?? fixture.kind;
const { command, args: configured, env } = JSON.parse(await readFile(join(workspace, ".mcp.json"), "utf8")).mcpServers.semio;
const args: string[] = [...configured];
const binary = process.env.SEMIO_OS_MCP_BIN;
if (binary) {
  const target = args.indexOf("--catalog-target");
  if (target >= 0) args.splice(target, 2);
}
await mkdir(generated, { recursive: true });
const folder = await mkdtemp(join(generated, "smoke-"));
args[args.indexOf("--folder") + 1] = folder;
args.push("--no-bridge");
const client = new Client({ name: "semio-configured-smoke", version: "1" });
const launch = {
  command,
  args,
  cwd: workspace,
  stderr: "pipe" as const,
  env: { ...getDefaultEnvironment(), ...env, ...(binary ? { SEMIO_OS_MCP_BIN: binary } : {}), ...(process.env.RUST_BACKTRACE ? { RUST_BACKTRACE: process.env.RUST_BACKTRACE } : {}), ...(requireAbsentProducts ? { SEMIO_MCP_COMPONENT_MODELS: "unissued model manifest" } : {}) },
};
const transport = new StdioClientTransport(launch);
const diagnostics: string[] = [];
const firstPanic: string[] = [];
const stderr = createInterface({ input: transport.stderr! });
stderr.on("line", line => {
  if (line.includes('"activity"')) return;
  if (firstPanic.length < 32 && (firstPanic.length || line.includes("panicked at"))) firstPanic.push(line);
  diagnostics.push(line);
  if (diagnostics.length > 12) diagnostics.shift();
});
const startup = { timeout: 180_000 };
let operation = "connect";

try {
  await client.connect(transport, startup);
  assert.equal(client.getServerVersion()?.name, "semio");
  assert.equal(client.getServerCapabilities()?.resources?.subscribe, true);
  operation = "discover";
  const [tools, resources, prompts] = await Promise.all([
    client.listTools({}, startup),
    client.listResources({}, startup),
    client.listPrompts({}, startup),
  ]);
  assert(tools.tools.some(tool => tool.name === "artifact_create"));
  assert(resources.resources.some(resource => resource.uri === "semio://workspace"));
  assert(prompts.prompts.some(prompt => prompt.name === "explore_workspace"));

  operation = "read the workspace";
  const workspaceInfo = await client.readResource({ uri: "semio://workspace" });
  assert(workspaceInfo.contents.some(content => content.uri === "semio://workspace" && "text" in content));

  const uri = `semio://artifact/${fixture.artifactId}`;
  operation = "create artifact";
  const created = await client.callTool({
    name: "artifact_create",
    arguments: { artifactId: fixture.artifactId, kind, ...(kind === fixture.kind ? { initial: fixture.initial } : {}) },
  });
  assert.equal(created.isError, false, JSON.stringify(created));
  operation = "subscribe to and read the artifact";
  await client.subscribeResource({ uri });
  const snapshot = await client.readResource({ uri });
  const body = snapshot.contents.find(content => content.uri === uri);
  assert(body && "text" in body);
  await client.unsubscribeResource({ uri });
  const duplicate = await client.callTool({ name: "artifact_create", arguments: { artifactId: fixture.artifactId, kind } });
  assert.equal(duplicate.isError, true, JSON.stringify(duplicate));
  await client.close();

  operation = "read the persisted artifact after restart";
  const reopened = new Client({ name: "semio-configured-smoke-reopen", version: "1" });
  try {
    await reopened.connect(new StdioClientTransport({ ...launch, stderr: "ignore" }), startup);
    const persisted = (await reopened.readResource({ uri }, startup)).contents.find(content => content.uri === uri);
    assert(persisted && "text" in persisted);
    assert.deepEqual(JSON.parse(persisted.text).untrusted.content, JSON.parse(body.text).untrusted.content);
  } finally {
    await reopened.close();
  }
  if (requireAbsentProducts) await assert.rejects(lstat(products), { code: "ENOENT" });
  console.log(JSON.stringify({ nativeTools: tools.tools.length, resources: resources.resources.length, prompts: prompts.prompts.length, artifact: uri, kind, persisted: true, ...(requireAbsentProducts ? { productsAbsent: true } : {}) }));
} catch (error) {
  console.error(`[DEBUG] Could not ${operation}`);
  console.error("[DEBUG] Workspace entries", await readdir(folder, { recursive: true }));
  for (const line of firstPanic.length ? firstPanic : diagnostics) console.error(`[DEBUG] ${line}`);
  throw error;
} finally {
  try {
    await client.close();
  } finally {
    stderr.close();
    await rm(folder, { recursive: true, force: true });
  }
}
