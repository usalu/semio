#!/usr/bin/env bun
/** 🔬️ G12 reproducer (from G11's) (WG9's finding): two sequential agent sessions on one hub note each `addBlock`; prints what each session's
 * `artifact_open` saw (revision, sync) and the block ids each commit minted (from the hub ledger's envelopes via the gateway's own
 * snapshot) plus the hub head. usage: SEMIO_OS_MCP_BIN=<gateway> bun g11-two-session-ids-probe.ts <hubOrigin> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { inflateSync, inflateRawSync } from "node:zlib";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
const HUB = process.argv[2]!;
const BIN = process.env.SEMIO_OS_MCP_BIN!;
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, json };
};
const human = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user2@semio.dev", password: "gm1-local-dev-pass-2", deviceInstanceId: `g11ids${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const name = `G11 ids ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", human, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", human)).json?.find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, human);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Ids" })))).json;
while (["accepted", "preparing"].includes(creation?.phase)) { await new Promise((r) => setTimeout(r, 1000)); creation = (await hub("GET", `${creations}/${requestId}`, human)).json; }
const noteId = String(creation?.ready?.artifactId ?? "");
const head = async () => (await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(noteId)}`, human)).json?.head_seq;
const idsIn = (value: unknown): string[] => {
  const found = new Set<string>();
  const scan = (text: string) => { for (const match of text.matchAll(/(?:text|note-text)-[0-9a-f]{16}(?:-\d+)?/gu)) found.add(match[0]); };
  const walk = (node: unknown) => {
    if (typeof node === "string") {
      scan(node);
      if (/^[A-Za-z0-9+/=]{24,}$/u.test(node)) {
        const bytes = Buffer.from(node, "base64");
        scan(bytes.toString("latin1"));
        for (let offset = 0; offset < bytes.length - 2; offset += 1) {
          for (const inflate of [inflateSync, inflateRawSync]) { try { scan(inflate(bytes.subarray(offset)).toString("latin1")); } catch {} }
        }
      }
    } else if (Array.isArray(node)) node.forEach(walk);
    else if (node && typeof node === "object") Object.values(node).forEach(walk);
  };
  walk(value);
  return [...found].sort();
};
const session = async (label: string) => {
  const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: `G11 ${label}`, audience: "edit", ttlSecs: 900 }));
  const path = join(mkdtempSync(join(tmpdir(), "g11-ids-")), "credential.json");
  writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: created.json?.token }), { mode: 0o600 });
  chmodSync(path, 0o600);
  const client = new Client({ name: `g11-ids-${label}`, version: "1" }, { capabilities: {} });
  await client.connect(new StdioClientTransport({ command: BIN, args: ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", path, "--scopes", "workspace.read,artifact.write", "--no-bridge"], env: { ...process.env } as Record<string, string>, stderr: "ignore" }), { timeout: 600_000 });
  const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  const before: any = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  const search: any = await client.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } });
  const addBlock = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((cap) => cap.endsWith(".addBlock")) ?? "");
  const prepared: any = await client.callTool({ name: "action_prepare", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  const edit: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  const prepared2: any = await client.callTool({ name: "action_prepare", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  const history: any = await client.readResource({ uri: `semio://artifact/${noteId}/history` }).catch((error: unknown) => ({ error: String(error) }));
  const after: any = await client.callTool({ name: "artifact_snapshot", arguments: { artifactId: noteId } }, undefined, { timeout: 600_000 });
  console.log(`${label}: open revision=${JSON.stringify(opened.structuredContent?.revision ?? opened.structuredContent?.sessionDocument ?? {}).slice(0, 300)}`);
  const { writeFileSync: dump } = await import("node:fs");
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/snapshot-${label}-before.json`, JSON.stringify(before, null, 1));
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/snapshot-${label}-after.json`, JSON.stringify(after, null, 1));
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/invoke-${label}.json`, JSON.stringify(edit, null, 1));
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/prepare-${label}-1.json`, JSON.stringify(prepared, null, 1));
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/prepare-${label}-2.json`, JSON.stringify(prepared2, null, 1));
  dump(`${process.env.G12_DUMP_DIR ?? "/tmp"}/history-${label}.json`, JSON.stringify(history, null, 1));
  console.log(`${label}: prepare ids first=${JSON.stringify(idsIn(prepared.structuredContent))} after-own-commit=${JSON.stringify(idsIn(prepared2.structuredContent))}`);
  console.log(`${label}: ids before=${JSON.stringify(idsIn(before.structuredContent))} after=${JSON.stringify(idsIn(after.structuredContent))} status=${edit.structuredContent?.status} post=${JSON.stringify(edit.structuredContent?.postconditions)} hub head=${await head()}`);
  await client.close();
};
console.log(`space=${spaceId} note=${noteId} head=${await head()}`);
await session("A");
await session("B");
process.exit(0);
