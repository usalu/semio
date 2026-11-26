import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { artifactCreationSucceededV1, mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { readNativeCatalogV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";

const [executable, products, operationPolicy, driverPolicy] = process.argv.slice(2);
assert(executable && products, "Native executable and products partition required");
const cwd = process.cwd();
const { args, env } = takeMcpPluginCatalog(mcpServerEntries(cwd).semio.args.slice(4), cwd);
assert(env.SEMIO_PLUGIN_CATALOG);
const folderAt = args.indexOf("--folder");
const operationPolicyAt = args.indexOf("--operation-policy");
assert(folderAt >= 0 && operationPolicyAt >= 0, "Configured workspace folder and operation policy required");
const { plugins } = readNativeCatalogV1(JSON.parse(await readFile(join(env.SEMIO_PLUGIN_CATALOG, "🦀️catalog.json"), "utf8")));
const note = plugins.find(row => row.pluginId === pins.pluginId);
assert(note, "Note catalog entry required");
const mode = "prototype";
const root = resolve(products, "browser", mode, "components", componentModelDirectoryKeyV1(note.cratePath));
const preparedActor = await actorPreparationImageIdentityV1(join(root, actorPreparationPolicyV1.image), AbortSignal.timeout(30_000));
const models = encodeMcpComponentModelManifestV1({ version: 1, mode, models: [{ pluginId: note.pluginId, cratePath: note.cratePath, root, preparedActor }] });
const generated = join(import.meta.dirname, "🗑️generated");
await mkdir(generated, { recursive: true });
const folder = await mkdtemp(join(generated, "sdk-note-"));
args[folderAt + 1] = folder;
if (operationPolicy) args[operationPolicyAt + 1] = resolve(operationPolicy);
if (driverPolicy) args.push("--host-driver-policy", resolve(driverPolicy));
const client = new Client({ name: "semio-sdk-note-create", version: "1" });
const trace = process.env.SEMIO_MCP_POLL_TRACE === "1";
const startTransport = () => new StdioClientTransport({ command: resolve(executable), args: withMcpHostDriverPolicy(["stdio", ...args, "--no-bridge", "--auto-approve", "all"]), cwd, env: { ...env, SEMIO_MCP_COMPONENT_MODELS: models, ...(trace ? { SEMIO_MCP_POLL_TRACE: "1" } : {}) }, stderr: trace ? "inherit" : "ignore" });
const transport=startTransport();
try {
  const readOnly = process.env.SEMIO_NOTE_SEEDED_READBACK === "1";
  const seeded = readOnly || process.env.SEMIO_NOTE_SEEDED_HISTORY === "1";
  if (seeded) await (await import("./note-seeded-history.ts")).seedNote(generated,folder);
  await client.connect(transport);
  console.log(JSON.stringify({ initialized: client.getServerVersion(), tools: (await client.listTools()).tools.length, model: note.pluginId }));
  if (seeded) {
    const {verifySeededNote}=await import("./note-seeded-history.ts");
    await verifySeededNote(client,root,generated,readOnly,false);
    if (readOnly) {
      const first=transport.pid;
      await client.close();
      const restarted=new Client({name:"semio-note-restart",version:"1"});
      const second=startTransport();
      try {
        await restarted.connect(second);
        assert(first && second.pid && second.pid!==first);
        await verifySeededNote(restarted,root,generated,true,false);
      } finally { await restarted.close(); }
    }
  } else {
  const started = performance.now();
  const artifactId = "sdk-note-create";
  const timeout = Number(process.env.SEMIO_NOTE_SDK_TIMEOUT_MS ?? 240_000);
  assert(Number.isSafeInteger(timeout) && timeout > 0);
  const created = await client.callTool({ name: "artifact_create", arguments: { artifactId, kind: pins.artifactKind } }, undefined, { timeout });
  console.log(JSON.stringify({ elapsedSeconds: (performance.now() - started) / 1000, isError: created.isError, result: created.structuredContent }));
  const snapshot = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId } });
  console.log(JSON.stringify({ snapshot: snapshot.isError ? snapshot.structuredContent : "captured" }));
  await client.ping();
  console.log(JSON.stringify({ ping: "ok" }));
  assert(artifactCreationSucceededV1(created, artifactId, pins.artifactKind, pins.pluginId), JSON.stringify(created.structuredContent));
  const body = snapshot.structuredContent;
  assert(!snapshot.isError && body?.artifactId === artifactId, JSON.stringify(body));
  const envelope = body.untrusted as { schema: string; content: { packBase64: string; sprBase64: string } };
  assert.equal(envelope.schema, "semio.mcp.untrusted-content/v1");
  for (const [encoded, length] of [[envelope.content.packBase64, body.packBytes], [envelope.content.sprBase64, body.sprBytes]] as const) {
    const bytes = Buffer.from(encoded, "base64");
    assert(bytes.length > 0 && bytes.length === length && bytes.toString("base64") === encoded);
  }
  }
} finally {
  await client.close();
  await rm(folder, { recursive: true, force: true });
}
