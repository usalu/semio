#!/usr/bin/env bun
/** 🎯️ G12 diagnostic: what an agent reads back from ONE verb on ONE existing hub document. Human = env credentials (never argv/logged);
 * an `edit` delegation on G12_SPACE, the gateway with the coverage gate's argv, `semio://workspace/artifacts` → the first document of
 * dialect G12_KIND freshly created in G12_SPACE, `artifact_open`, then `action_prepare` G12_CAPABILITY with G12_INPUT (JSON, default {}) + `action_invoke`, printing both
 * answers in full and the hub head before/after. usage: SEMIO_OS_MCP_BIN=<gateway> bun g12-invoke-probe.ts <hubOrigin> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const HUB = process.argv[2]!;
const spaceId = process.env.G12_SPACE!;
const t0 = Date.now();
const at = () => `+${((Date.now() - t0) / 1000).toFixed(1)}s`;
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, json };
};
const human = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL!, password: process.env.OS_MCP_HUB_PASSWORD!, deviceInstanceId: `g12inv${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "G12 invoke", audience: "edit", ttlSecs: 900 }));
const path = join(mkdtempSync(join(tmpdir(), "g12-invoke-")), "credential.json");
writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: created.json?.token }), { mode: 0o600 });
chmodSync(path, 0o600);
const args = ["stdio", "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write", "--hub", HUB, "--space", spaceId, "--credential-file", path, "--no-bridge", "--auto-approve", "all"];
const client = new Client({ name: "g12-invoke", version: "1" }, { capabilities: {} });
await client.connect(new StdioClientTransport({ command: process.env.SEMIO_OS_MCP_BIN!, args, env: { ...process.env } as Record<string, string>, stderr: "inherit" }), { timeout: 600_000 });
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, human);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => entry?.dialect?.artifactKind === process.env.G12_KIND);
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Invoke probe" })))).json;
while (["accepted", "preparing"].includes(creation?.phase)) { await new Promise((r) => setTimeout(r, 1000)); creation = (await hub("GET", `${creations}/${requestId}`, human)).json; }
const documentId = String(creation?.ready?.artifactId ?? "");
const head = async () => (await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, human)).json?.head_seq;
console.log(`${at()} created ${process.env.G12_KIND} (${kind?.kindId}) → ${documentId} phase=${creation?.phase} head=${await head()}`);
const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: documentId } }, undefined, { timeout: 600_000 });
console.log(`${at()} open isError=${opened.isError === true} ${JSON.stringify(opened.structuredContent?.sessionDocument?.sync ?? opened.structuredContent).slice(0, 400)}`);
const prepared: any = await client.callTool({ name: "action_prepare", arguments: { capabilityId: process.env.G12_CAPABILITY!, input: JSON.parse(process.env.G12_INPUT ?? "{}") } }, undefined, { timeout: 600_000 });
console.log(`${at()} prepare ${JSON.stringify(prepared.structuredContent).slice(0, 2500)}`);
const answer: any = await client.callTool({ name: "action_invoke", arguments: { preparedActionHandle: prepared.structuredContent?.preparedHandle } }, undefined, { timeout: 600_000 });
console.log(`${at()} invoke ${JSON.stringify(answer.structuredContent).slice(0, 2500)}`);
await new Promise((r) => setTimeout(r, 3000));
console.log(`${at()} hub head ${await head()}`);
await client.close();
await hub("POST", `/auth/agent-delegations/${encodeURIComponent(String(created.json?.delegationId ?? ""))}/revoke`, human);
process.exit(0);
