#!/usr/bin/env bun
/** 🔗️ G12 diagnostic: why a delegated gateway's hub document link sits in backoff. Human = env credentials (never argv/logged);
 * fresh space + note, one `edit` delegation, the gateway with the participant gate's argv (scopes of `.mcp.json`, `--no-bridge`,
 * optional `--auto-approve all` via G12_AUTO_APPROVE=1), then `artifact_open` in full, the link report polled for 20 s, and one
 * `addBlock` prepare + invoke; G12_SPACE + G12_DOCUMENT bind an existing space and document instead (G12_SNAPSHOT=1 / G12_CONTEXT=1 / G12_READ_LIST=1 add the participant gate's reads). Gateway stderr is inherited. usage: SEMIO_OS_MCP_BIN=<gateway> bun g12-link-probe.ts <hubOrigin> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const HUB = process.argv[2]!;
const t0 = Date.now();
const at = () => `+${((Date.now() - t0) / 1000).toFixed(1)}s`;
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, json };
};
const human = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL!, password: process.env.OS_MCP_HUB_PASSWORD!, deviceInstanceId: `g12link${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const fresh = async (): Promise<{ spaceId: string; noteId: string }> => {
  const name = `G12 link ${randomBytes(3).toString("hex")}`;
  await hub("POST", "/directory/commands", human, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))));
  const spaceId = String((await hub("GET", "/directory/spaces", human)).json?.find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = await hub("GET", creations, human);
  const kind = (catalog.json?.kinds ?? []).find((entry: any) => entry?.dialect?.artifactKind === "s.note.note");
  const requestId = randomBytes(16).toString("hex");
  let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Link" })))).json;
  while (["accepted", "preparing"].includes(creation?.phase)) { await new Promise((r) => setTimeout(r, 1000)); creation = (await hub("GET", `${creations}/${requestId}`, human)).json; }
  return { spaceId, noteId: String(creation?.ready?.artifactId ?? "") };
};
const { spaceId, noteId } = process.env.G12_SPACE ? { spaceId: process.env.G12_SPACE, noteId: process.env.G12_DOCUMENT ?? "" } : await fresh();
const head = async () => (await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(noteId)}`, human)).json?.head_seq;
console.log(`${at()} space=${spaceId} note=${noteId} head=${await head()}`);
const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "G12 link", audience: "edit", ttlSecs: 900 }));
const path = join(mkdtempSync(join(tmpdir(), "g12-link-")), "credential.json");
writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: created.json?.token }), { mode: 0o600 });
chmodSync(path, 0o600);
const args = ["stdio", "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write", "--hub", HUB, "--space", spaceId, "--credential-file", path, "--no-bridge", ...(process.env.G12_AUTO_APPROVE === "1" ? ["--auto-approve", "all"] : [])];
const client = new Client({ name: "g12-link", version: "1" }, { capabilities: {} });
await client.connect(new StdioClientTransport({ command: process.env.SEMIO_OS_MCP_BIN!, args, env: { ...process.env } as Record<string, string>, stderr: "inherit" }), { timeout: 600_000 });
console.log(`${at()} connected`);
if (process.env.G12_READ_LIST === "1") {
  const listed: any = await client.readResource({ uri: "semio://workspace/artifacts" });
  console.log(`${at()} workspace/artifacts ${String(listed.contents?.[0]?.text ?? "").slice(0, 200)}`);
}
const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
console.log(`${at()} artifact_open isError=${opened.isError === true} ${JSON.stringify(opened.structuredContent).slice(0, 1500)}`);
if (process.env.G12_SNAPSHOT === "1") {
  const snapshot: any = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  console.log(`${at()} artifact_snapshot isError=${snapshot.isError === true} ${JSON.stringify(snapshot.structuredContent).slice(0, 300)}`);
}
if (process.env.G12_CONTEXT === "1") {
  const context: any = await client.callTool({ name: "context_resolve", arguments: {} }, undefined, { timeout: 600_000 });
  console.log(`${at()} context_resolve ${JSON.stringify(context.structuredContent).slice(0, 300)}`);
}
for (let i = 0; i < 10; i += 1) {
  await new Promise((r) => setTimeout(r, 2000));
  const again: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  console.log(`${at()} reopen sync=${JSON.stringify(again.structuredContent?.sync ?? again.structuredContent?.hub ?? null)}`);
}
const search: any = await client.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } });
const id = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((cap) => cap.endsWith(".addBlock")) ?? "");
const prepared: any = await client.callTool({ name: "action_prepare", arguments: { capabilityId: id, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
console.log(`${at()} prepare ${JSON.stringify(prepared.structuredContent).slice(0, 400)}`);
const answer: any = await client.callTool({ name: "action_invoke", arguments: { preparedActionHandle: prepared.structuredContent?.preparedHandle } }, undefined, { timeout: 600_000 });
console.log(`${at()} invoke ${JSON.stringify(answer.structuredContent).slice(0, 800)}\n${at()} hub head ${await head()}`);
await client.close();
await hub("POST", `/auth/agent-delegations/${encodeURIComponent(String(created.json?.delegationId ?? ""))}/revoke`, human);
process.exit(0);
