#!/usr/bin/env bun
/** 🔎️ H13: one fresh semio MCP gateway run in an existing hub space opens one document of every plugin once — read-only (a
 * `read` delegation) — and prints per open: plugin, ms, outcome, how many `notifications/progress` it heard and how many of
 * them were component `receiving:` bytes. Run it twice over one `SEMIO_EXECUTION_TARGET_STORE_DIR`: the first run streams
 * each component with progress, the second must hear no `receiving:` at all. Human = env credentials (never argv, never
 * printed); the delegation is revoked at the end. usage: SEMIO_OS_MCP_BIN=<bin> bun h13-mcp-asset-probe.ts <hub> <spaceId> <run> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";

const HUB = process.argv[2]!;
const SPACE = process.argv[3]!;
const RUN = process.argv[4] ?? "run";
const t0 = Date.now();
const at = () => `+${((Date.now() - t0) / 1000).toFixed(1)}s`;
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, json };
};
const signIn = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL!, password: process.env.OS_MCP_HUB_PASSWORD!, deviceInstanceId: `h13asset${Date.now().toString(16)}`, clientClass: "browser" }));
const human = String(signIn.json?.token ?? "");
const page = await hub("GET", `/directory/spaces/${encodeURIComponent(SPACE)}`, human);
const documents: any[] = page.json?.documents?.rows ?? [];
const firstPerPlugin = new Map<string, string>();
for (const row of documents) {
  const plugin = String(row?.descriptor?.owner?.pluginId ?? row?.owner?.pluginId ?? "");
  const id = String(row?.descriptor?.documentId ?? row?.documentId ?? "");
  if (plugin && id && !firstPerPlugin.has(plugin)) firstPerPlugin.set(plugin, id);
}
console.log(`${at()} ${RUN} space=${SPACE} documents=${documents.length} plugins=${firstPerPlugin.size} signIn=${signIn.status} store=${process.env.SEMIO_EXECUTION_TARGET_STORE_DIR ?? "(per-user)"}`);
const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: SPACE, agentLabel: "H13 asset probe", audience: "read", ttlSecs: 3600 }));
const path = join(mkdtempSync(join(tmpdir(), "h13-asset-")), "credential.json");
writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId: SPACE, audience: "read", token: created.json?.token }), { mode: 0o600 });
chmodSync(path, 0o600);
const client = new Client({ name: "h13-asset-probe", version: "1" }, { capabilities: {} });
await client.connect(new StdioClientTransport({ command: process.env.SEMIO_OS_MCP_BIN!, args: ["stdio", "--scopes", "workspace.read", "--hub", HUB, "--space", SPACE, "--credential-file", path, "--no-bridge"], env: { ...process.env } as Record<string, string>, stderr: "inherit" }), { timeout: 600_000 });
console.log(`${at()} ${RUN} connected (delegation ${created.status})`);
let passed = 0;
let receivingOpens = 0;
for (const [plugin, id] of firstPerPlugin) {
  const heard: string[] = [];
  const started = Date.now();
  const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: id } }, undefined, { timeout: 900_000, onprogress: (progress: any) => heard.push(String(progress?.message ?? "")), resetTimeoutOnProgress: true });
  const receiving = heard.filter((message) => message.startsWith("receiving:"));
  const awaiting = heard.filter((message) => message.startsWith("awaiting-stream:") || message.startsWith("retrying:"));
  if (receiving.length > 0) receivingOpens += 1;
  const outcome = opened.isError === true ? `ERROR ${JSON.stringify(opened.structuredContent ?? opened.content).slice(0, 220)}` : "ok";
  if (opened.isError !== true) passed += 1;
  console.log(`${at()} ${RUN} ${plugin} ${Date.now() - started}ms progress=${heard.length} receiving=${receiving.length} waits=${awaiting.length} last=${JSON.stringify((receiving.at(-1) ?? "").split(" — ")[0])} ${outcome}`);
}
console.log(`${at()} ${RUN} opens ok ${passed}/${firstPerPlugin.size} opensThatReceivedComponentBytes=${receivingOpens}`);
await client.close();
const revoked = await hub("POST", `/auth/agent-delegations/${encodeURIComponent(String(created.json?.delegationId ?? ""))}/revoke`, human);
console.log(`${at()} ${RUN} delegation revoked ${revoked.status}`);
process.exit(passed === firstPerPlugin.size ? 0 : 1);
