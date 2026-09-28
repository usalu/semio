#!/usr/bin/env bun
/** 🔬️ G12 copy of G11/H13's reproducer (credentials from OS_MCP_HUB_EMAIL/OS_MCP_HUB_PASSWORD, never argv/logged; note kind by dialect): what the semio MCP answers when the hub refuses an agent's edit — (a) a `read`-audience delegation whose
 * gateway claims artifact.write, (b) an `edit` delegation revoked between two edits. Prints each answer in full and the hub's
 * ledger head. usage: SEMIO_OS_MCP_BIN=<gateway> OS_MCP_HUB_EMAIL=… OS_MCP_HUB_PASSWORD=… bun g12-refused-relay-probe.ts <hubOrigin> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { agentDelegationRevokePathV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🤖️delegations/🟦️.ts";
const HUB = process.argv[2]!;
const BIN = process.env.SEMIO_OS_MCP_BIN!;
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, json };
};
const human = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL!, password: process.env.OS_MCP_HUB_PASSWORD!, deviceInstanceId: `g11refused${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const name = `G11 refused ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", human, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", human)).json?.find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, human);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => entry?.dialect?.artifactKind === "s.note.note");
console.log(`catalog generation=${catalog.json?.catalogGenerationId} kinds=${(catalog.json?.kinds ?? []).length} note kind=${kind?.kindId}`);
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Refused" })))).json;
while (["accepted", "preparing"].includes(creation?.phase)) { await new Promise((r) => setTimeout(r, 1000)); creation = (await hub("GET", `${creations}/${requestId}`, human)).json; }
const noteId = String(creation?.ready?.artifactId ?? "");
const head = async () => (await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(noteId)}`, human)).json?.head_seq;
const delegate = async (audience: "read" | "edit") => {
  const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: `G11 ${audience}`, audience, ttlSecs: 900 }));
  const path = join(mkdtempSync(join(tmpdir(), "g11-refused-")), "credential.json");
  writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience, token: created.json?.token }), { mode: 0o600 });
  chmodSync(path, 0o600);
  return { id: String(created.json?.delegationId ?? ""), path };
};
const connect = async (path: string) => {
  const client = new Client({ name: "g11-refused", version: "1" }, { capabilities: {} });
  await client.connect(new StdioClientTransport({ command: BIN, args: ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", path, "--scopes", "workspace.read,artifact.write", "--no-bridge", "--auto-approve", "all"], env: { ...process.env } as Record<string, string>, stderr: "inherit" }), { timeout: 600_000 });
  return client;
};
const addBlock = async (client: Client) => {
  const search: any = await client.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } });
  const id = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((cap) => cap.endsWith(".addBlock")) ?? "");
  const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  if (opened.isError) return { ms: 0, isError: true, structured: { open: opened.structuredContent } };
  const started = Date.now();
  const answer: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: id, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  return { ms: Date.now() - started, isError: answer.isError === true, structured: answer.structuredContent };
};
console.log(`space=${spaceId} note=${noteId} head=${await head()}`);
const reader = await delegate("read");
const readClient = await connect(reader.path);
const readEdit = await addBlock(readClient);
console.log(`(a) read delegation edit: ${JSON.stringify(readEdit)}\n    hub head after: ${await head()}`);
await readClient.close();
const editor = await delegate("edit");
const editClient = await connect(editor.path);
const first = await addBlock(editClient);
console.log(`(b0) edit before revoke: ${JSON.stringify(first)}\n    hub head: ${await head()}`);
const revoke = await hub("POST", agentDelegationRevokePathV1(editor.id), human);
const after = await addBlock(editClient);
console.log(`(b1) edit after revoke (${revoke.status}): ${JSON.stringify(after)}\n    hub head: ${await head()}`);
await editClient.close();
process.exit(0);
