import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { CallToolResultSchema } from "@modelcontextprotocol/sdk/types.js";
import { exportedDocumentMatchesSnapshotV1 } from "@semio-tech/framework-os-mcp";
import { mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { readNativeCatalogV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";

const [executable, products, operationPolicy, driverPolicy, ...requestedCuts] = process.argv.slice(2);
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
const transport = startTransport();
const artifactId = "note-seeded-sdk";

try {
  await (await import("./note-seeded-history.ts")).seedNote(generated, folder);
  await client.connect(transport);
  if (process.env.SEMIO_EXPORT_LAW_OPEN_ARTIFACT === "1") {
    const opened = await client.callTool({ name: "artifact_open", arguments: { artifactId } }, CallToolResultSchema);
    assert(!opened.isError && opened.structuredContent, JSON.stringify(opened.structuredContent));
    assert.equal(opened.structuredContent.artifactKind, pins.artifactKind);
  }
  const stablePair = async () => {
    const reply = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId } }, CallToolResultSchema);
    assert(!reply.isError && reply.structuredContent, JSON.stringify(reply.structuredContent));
    const envelope = reply.structuredContent.untrusted as { schema: string; content: { packBase64: string; sprBase64: string } };
    assert.equal(envelope.schema, "semio.mcp.untrusted-content/v1");
    return envelope.content;
  };
  const expected = await stablePair();
  const seed = (await readFile(join(generated, "note-wasi-seeded-pair.tsv"), "utf8")).trim().split("\t");
  assert.deepEqual([expected.packBase64, expected.sprBase64], seed);
  const request = { name: "artifact_export", arguments: { artifactId, format: "artifact:out" } };
  const cuts = requestedCuts.length ? requestedCuts : ["loading-document", "reading-document"];
  const onlyComplete = process.env.SEMIO_EXPORT_LAW_ONLY_COMPLETE === "1";
  const admittedAfter = new Map([["opening-guest", 0], ["loading-document", 0.75], ["reading-document", 0.875]]);
  for (const cut of onlyComplete ? [] : cuts) {
    const after = admittedAfter.get(cut);
    assert(after !== undefined, `Unknown export cut ${cut}`);
    let requestId;
    let stop;
    const send = transport.send.bind(transport);
    transport.send = (message, options) => {
      if ("id" in message && "method" in message && message.method === "tools/call" && message.params?.name === "artifact_export") requestId = message.id;
      return send(message, options);
    };
    const progress = [];
    const reply = await client.callTool(request, CallToolResultSchema, {
      timeout:240000, onprogress:note => {
        if (progress.at(-1) !== note.message) { progress.push(note.message); console.log("[DEBUG] SDK export phase", note.message); }
        if (!stop && note.progress > after && note.message === `exporting: ${cut}`) {
          assert(requestId !== undefined);
          console.log("[DEBUG] SDK cancelling export", JSON.stringify({ cut, progress: note.progress, requestId }));
          stop = send({jsonrpc:"2.0",method:"notifications/cancelled",params:{requestId,reason:"stop at "+cut}});
        }
      },
    }).finally(() => { transport.send = send; });
    await stop;
    console.log("[DEBUG] SDK export answered",JSON.stringify(reply.structuredContent));
    assert(stop && reply.isError && reply.structuredContent?.code === "CANCELLED", JSON.stringify({cut,progress,result:reply.structuredContent}));
    assert.equal(reply.structuredContent.untrusted, undefined);
    assert.deepEqual(await stablePair(), expected);
    await client.ping();
    console.log("[DEBUG] SDK export cancellation", JSON.stringify({cut,progress,persistedPairUnchanged:true}));
  }
  if (onlyComplete || process.env.SEMIO_EXPORT_LAW_NO_RETRY !== "1") {
    let phase: string | undefined;
    const completed = await client.callTool(request, CallToolResultSchema, {
      timeout: 240000,
      onprogress: note => {
        if (phase === note.message) return;
        phase = note.message;
        console.log("[DEBUG] SDK completion phase", phase);
      },
    });
    const result = completed.structuredContent;
    assert(!completed.isError && result, JSON.stringify(result));
    assert.equal(result.artifactId, request.arguments.artifactId);
    assert.equal(result.format, request.arguments.format);
    assert.equal(result.pluginId, note.pluginId);
    const exported = result.untrusted as { schema: string; content: { contentBase64: string } };
    assert.equal(exported.schema, "semio.mcp.untrusted-content/v1");
    assert.equal(result.contentBytes, Buffer.from(exported.content.contentBase64, "base64").length);
    assert(exportedDocumentMatchesSnapshotV1(exported.content.contentBase64, seed.join(".")));
    assert.deepEqual(await stablePair(), expected);
    await client.ping();
    console.log("[DEBUG] Actual Note archive exactly matches original Pack and SPR", result.contentBytes);
  }
} finally {
  await client.close();
  await rm(folder, {recursive:true,force:true});
}
