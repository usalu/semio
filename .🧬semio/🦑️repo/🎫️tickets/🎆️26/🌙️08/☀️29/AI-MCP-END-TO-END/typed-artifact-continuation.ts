import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { getDefaultEnvironment, StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import fixture from "./typed-artifact-continuation.json";

const { command, args: configured, env } = JSON.parse(await readFile(".mcp.json", "utf8")).mcpServers.semio;
const args: string[] = [...configured];
const binary = process.env.SEMIO_OS_MCP_BIN;
if (binary) {
  const target = args.indexOf("--catalog-target");
  if (target >= 0) args.splice(target, 2);
}
const folder = await mkdtemp(join(import.meta.dir, "🗑️generated", "typed-"));
args[args.indexOf("--folder") + 1] = folder;
args.push("--no-bridge");
for (const [envKey, flag, source, section] of [
  ["SEMIO_DIAGNOSTIC_OPERATION_ITEMS", "--host-driver-policy", "🏛️host-driver-policy.json", "operation"],
  ["SEMIO_DIAGNOSTIC_TRANSIT_ITEMS", "--operation-policy", "💡️host-operation-policy.json", "transit"],
] as const) {
  const raw = process.env[envKey];
  if (!raw) continue;
  const maximumItems = Number(raw);
  assert.ok(Number.isSafeInteger(maximumItems) && maximumItems > 0, envKey);
  const configRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config";
  const i = args.indexOf(flag);
  const policy = JSON.parse(await readFile(i < 0 ? join(configRoot, source) : args[i + 1], "utf8"));
  const grant = section === "operation" ? policy.operation : policy.transit.retained;
  grant.maximumItems = maximumItems;
  const path = join(folder, source);
  await writeFile(path, JSON.stringify(policy));
  if (i < 0) args.push(flag, path);
  else args[i + 1] = path;
}
const client = new Client({ name: "semio-typed-artifact-continuation", version: "1" });
const transport = new StdioClientTransport({
  command,
  args,
  cwd: process.cwd(),
  stderr: "pipe",
  env: { ...getDefaultEnvironment(), ...env, ...(binary ? { SEMIO_OS_MCP_BIN: binary } : {}), ...(process.env.RUST_BACKTRACE ? { RUST_BACKTRACE: process.env.RUST_BACKTRACE } : {}) },
});
const diagnostics: string[] = [];
const sourceDiagnostics: string[] = [];
const firstPanic: string[] = [];
let stderrReportsPanic = false;
const stderr = createInterface({ input: transport.stderr! });
stderr.on("line", (line) => {
  if (line.includes('"activity"')) return;
  if (line.includes("[DEBUG] poll ")) {
    sourceDiagnostics.push(line);
    console.error(line);
  }
  stderrReportsPanic ||= line.includes("panicked at");
  if (stderrReportsPanic && firstPanic.length < 35) firstPanic.push(line);
  diagnostics.push(line);
  if (diagnostics.length > 40) diagnostics.shift();
});
const timeout = { timeout: 180_000 };

try {
  await client.connect(transport, timeout);
  const created = await client.callTool(fixture.request, undefined, timeout);
  assert.equal(fixture.followUp, "tools/list");
  const followUp = await client.listTools({}, timeout);
  const uri = `semio://artifact/${fixture.request.arguments.artifactId}`;
  const durable = await client.readResource({ uri }, timeout).then(result => result.contents.some(item => item.uri === uri), () => false);
  if (created.isError) {
    console.error(`[DEBUG] typed artifact readable after refusal: ${durable}`);
  }
  const toolText = created.content.flatMap(item => item.type === "text" ? [item.text] : []);
  assert.deepEqual({
    artifactCreateReturnsToolResult: Array.isArray(created.content),
    artifactCreateSucceeds: created.isError !== true,
    persistedArtifactIsReadable: durable,
    sameSessionListsArtifactCreate: followUp.tools.some(({ name }) => name === fixture.request.name),
    retirementSlotRefused: toolText.some(message => message.includes("no available explicit retirement slot")),
    obsoletePollAbiRefused: toolText.some(message => message.includes("expected 19 values, received 2")),
    startupGrantRefused: [...sourceDiagnostics, ...toolText].some(message => message.includes("Core execution requires the actual incoming input and unchanged physical ceiling")),
    stderrReportsPanic,
  }, fixture.expected, JSON.stringify(created));
  console.log(JSON.stringify({ ...fixture.expected, created }));
} catch (error) {
  for (const line of firstPanic.length ? firstPanic : diagnostics) {
    if (!line.includes("[DEBUG] poll ")) console.error(`[DEBUG] ${line}`);
  }
  throw error;
} finally {
  try {
    await client.close();
  } finally {
    stderr.close();
    await rm(folder, { recursive: true, force: true });
  }
}
