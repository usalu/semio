import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { CallToolResultSchema, McpError, type JSONRPCMessage } from "@modelcontextprotocol/sdk/types.js";
import { mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { readNativeCatalogV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";

class ObservedStdio extends StdioClientTransport {
  onSend?: (message: JSONRPCMessage) => void;
  override async send(message: JSONRPCMessage): Promise<void> {
    this.onSend?.(message);
    await super.send(message);
  }
}

async function within<T>(promise: Promise<T>, milliseconds: number, label: string): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([promise, new Promise<never>((_, reject) => {
      timer = setTimeout(() => reject(Error(label)), milliseconds);
    })]);
  } finally {
    clearTimeout(timer);
  }
}

const requireCoreInput = !process.argv.includes("--opening-only");
const workspace = process.cwd();
const generated = join(import.meta.dirname, "🗑️generated");
const option = (name: string) => process.argv.slice(2).find(arg => arg.startsWith(name + "="))?.slice(name.length+1);
const executable = resolve(option("--gateway") ?? join(generated, "native-host", "debug", process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp"));
const entry = mcpServerEntries(workspace).semio;
const catalog = takeMcpPluginCatalog(entry.args.slice(4), workspace);
assert(catalog.env.SEMIO_PLUGIN_CATALOG);
const { plugins } = readNativeCatalogV1(JSON.parse(await readFile(join(catalog.env.SEMIO_PLUGIN_CATALOG, "🦀️catalog.json"), "utf8")));
const mode = "prototype";
const models = [];
for (const pluginId of [pins.pluginId, pins.inferencePluginId]) {
  const plugin = plugins.find(row => row.pluginId === pluginId);
  assert(plugin, `Missing catalog owner ${pluginId}`);
  const root = join(resolve(option("--" + pluginId) ?? join(generated, "m-canon")), "browser", mode, "components", componentModelDirectoryKeyV1(plugin.cratePath));
  const pair = verifySemanticDescriptorPairBytesV1(await readFile(join(root, "🔣️.json")), await readFile(join(root, "🛂️.descriptor.semio")));
  assert.equal(pair.pluginId, pluginId);
  const preparedActor = await actorPreparationImageIdentityV1(join(root, actorPreparationPolicyV1.image), AbortSignal.timeout(30_000));
  models.push({ pluginId, cratePath: plugin.cratePath, root, preparedActor });
  console.log("[DEBUG] model", pluginId, pair.semanticSha256);
}
await mkdir(generated, { recursive: true });
const folder = await mkdtemp(join(generated, "sdk-cancel-"));
const artifactId = `sdk-cancel-${randomUUID()}`;
const reason = "Cancel the observed typed Note create";
const requestOptions = { timeout: 30_000 };
const env = Object.fromEntries(Object.entries({
  ...process.env, ...catalog.env, ...entry.env,
  SEMIO_REPO_ROOT: workspace,
  SEMIO_BUILD_MODE: mode,
  SEMIO_SCRIPT_PROGRESS: "human",
  SEMIO_TRACE_OWNED_INPUT_WITNESS: "1",
  SEMIO_MCP_COMPONENT_MODELS: encodeMcpComponentModelManifestV1({ version: 1, mode, models }),
}).filter((pair): pair is [string, string] => typeof pair[1] === "string"));
const transport = new ObservedStdio({
  command: executable, args: withMcpHostDriverPolicy(["stdio", ...catalog.args, "--folder", folder, "--no-bridge", "--auto-approve", "all"]),
  cwd: workspace, env, stderr: "pipe",
});
const client = new Client({ name: "semio-mcp-sdk-note-cancel", version: "1" }, { capabilities: {} });
const cancellation = new AbortController();
const terminal = Promise.withResolvers<JSONRPCMessage>();
const stderrLines: string[] = [];
const progressMessages: string[] = [];
let tail = "";
let requestId: string | number | undefined;
let progressToken: unknown;
let cancelledRequestId: unknown;
let opening: { ms: number; message: string } | undefined;
let input: { ms: number; line: string } | undefined;
let cancelledAt: number | undefined;
let terminalAt: number | undefined;
let started = 0;
const elapsed = () => Math.round(performance.now() - started);
function cancelWhenObserved() {
  if (opening && (input || !requireCoreInput) && terminalAt === undefined && !cancellation.signal.aborted) {
    cancelledAt = elapsed();
    console.log("[DEBUG] aborting", JSON.stringify({ requestId, progressToken, opening, input, cancelledAt }));
    cancellation.abort(new Error(reason));
  }
}
transport.onSend = message => {
  if ("method" in message && message.method === "tools/call" && "id" in message && message.params?.name === "artifact_create") {
    requestId = message.id;
    progressToken = (message.params?._meta as { progressToken?: unknown } | undefined)?.progressToken;
    console.log("[DEBUG] create request", JSON.stringify({ requestId, progressToken, artifactId }));
  }
  if ("method" in message && message.method === "notifications/cancelled" && !("id" in message)) {
    cancelledRequestId = message.params?.requestId;
    console.log("[DEBUG] cancellation notification", JSON.stringify(message));
  }
};
transport.stderr?.setEncoding("utf8").on("data", (chunk: string) => {
  const lines = (tail + chunk).split("\n");
  tail = (lines.pop() ?? "").slice(-4096);
  for (const line of lines) {
    stderrLines.push(line.slice(0, 500));
    if (stderrLines.length > 24) stderrLines.shift();
    if (requestId !== undefined && !input && line.startsWith("[DEBUG] Core Input witness ")) {
      input = { ms: elapsed(), line: line.slice(0, 500) };
      console.log("[DEBUG] input", JSON.stringify(input));
      cancelWhenObserved();
    }
  }
});

try {
  await within(client.connect(transport), 30_000, "MCP SDK handshake timed out");
  assert.equal(client.getServerVersion()?.name, "semio");
  const downstream = transport.onmessage;
  assert(downstream);
  transport.onmessage = (message, extra) => {
    if ("id" in message && requestId !== undefined && message.id === requestId && ("result" in message || "error" in message)) {
      terminalAt = elapsed();
      terminal.resolve(message);
      if (cancellation.signal.aborted) return;
    }
    downstream(message, extra);
  };
  const context = await client.callTool({ name: "context_resolve", arguments: { locale: pins.caller.locale } }, CallToolResultSchema, requestOptions);
  assert.notEqual(context.isError, true);
  assert.equal(context.structuredContent?.channel, "headless");
  started = performance.now();
  const create = client.callTool({ name: "artifact_create", arguments: { artifactId, kind: pins.artifactKind } }, CallToolResultSchema, {
    signal: cancellation.signal,
    timeout: 150_000,
    onprogress: progress => {
      if (progress.message && progressMessages.at(-1) !== progress.message && progressMessages.length < 25) progressMessages.push(progress.message);
      if (!opening && progress.message === "creating: opening-guest") {
        opening = { ms: elapsed(), message: progress.message };
        console.log("[DEBUG] progress", JSON.stringify(opening));
        cancelWhenObserved();
      }
    },
  }).then(result => ({ result }), error => ({ error }));
  const response = await within(terminal.promise, 110_000, "Typed create sent no terminal response");
  console.log("[DEBUG] terminal", JSON.stringify(response));
  assert(opening, "No opening-guest progress was received");
  assert(!requireCoreInput || input, "No charged Core Input witness was received");
  assert(requestId !== undefined && progressToken !== undefined);
  assert.equal(cancelledRequestId, requestId);
  assert(cancelledAt !== undefined && terminalAt !== undefined && cancelledAt <= terminalAt);
  assert("result" in response && !("error" in response), "Expected a terminal structured tool result");
  const cancelled = CallToolResultSchema.parse(response.result);
  assert.notEqual(cancelled.isError, true);
  assert.equal(cancelled.structuredContent?.status, "CANCELLED");
  assert.equal(cancelled.structuredContent?.artifactId, artifactId);
  assert.equal(cancelled.structuredContent?.kind, pins.artifactKind);
  assert.equal(cancelled.structuredContent?.pluginId, pins.pluginId);
  const local = await create;
  assert("error" in local && local.error instanceof McpError && local.error.message.includes(reason));
  const snapshot = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId } }, CallToolResultSchema, requestOptions);
  assert.equal(snapshot.isError, true);
  assert.equal(snapshot.structuredContent?.code, "NOT_FOUND");
  await client.ping(requestOptions);
  console.log("[DEBUG] proof", JSON.stringify({ artifactId, requestId, progressToken, opening, input, cancelledAt, terminalAt, tool: cancelled.structuredContent, snapshot: snapshot.structuredContent, serverPing: true }));
} catch (error) {
  console.error("[DEBUG] observed progress", JSON.stringify(progressMessages));
  for (const line of stderrLines) console.error(`[DEBUG] gateway stderr: ${line}`);
  throw error;
} finally {
  try { await client.close(); } finally { await rm(folder, { recursive: true, force: true }); }
}
