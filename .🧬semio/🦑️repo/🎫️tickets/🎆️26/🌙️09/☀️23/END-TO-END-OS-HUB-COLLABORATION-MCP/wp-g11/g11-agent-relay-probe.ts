#!/usr/bin/env bun
/** 🔬️ G11: does a hub-bound agent's committed edit reach the hub? API-only (no browser): a fresh space + note by the
 * hub's authorities, a delegated `semio-os-mcp --hub` agent opens it and commits `addBlock`, then the hub's own document
 * head is polled. Prints the gateway's stderr tail on a miss. usage: bun g11-agent-relay-probe.ts <hub> <rounds> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { requireMcpBinary, spawnRawMcp } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const [HUB, ROUNDS = "3"] = process.argv.slice(2);
if (!HUB) throw new Error("usage: bun g11-agent-relay-probe.ts <hubOrigin> [rounds] — no default hub (7800 belongs to W2)");
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};
const token = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user2@semio.dev", password: "gm1-local-dev-pass-2", deviceInstanceId: `g11relay${randomBytes(12).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
for (let round = 0; round < Number(ROUNDS); round += 1) {
  const spaceName = `G11 relay ${randomBytes(3).toString("hex")}`;
  await hub("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
  const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = await hub("GET", creations, token);
  const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
  const requestId = randomBytes(16).toString("hex");
  let creation = (await hub("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "relay" })))).json;
  while (["accepted", "preparing"].includes(creation?.phase)) {
    await pause(1_000);
    creation = (await hub("GET", `${creations}/${requestId}`, token)).json;
  }
  const documentId = String(creation?.ready?.artifactId ?? "");
  const delegation = await hub("POST", "/auth/agent-delegations", token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "relay probe", audience: "edit", ttlSecs: 900 }));
  const credentialPath = join(mkdtempSync(join(tmpdir(), "g11-relay-")), "credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  chmodSync(credentialPath, 0o600);
  const agent = spawnRawMcp(requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"]);
  const started = Date.now();
  await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g11-relay", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const call = async (name: string, args: Record<string, unknown>) => ((await agent.request("tools/call", { name, arguments: args }, 600_000)).result ?? {}) as any;
  const opened = await call("artifact_open", { artifactId: documentId });
  const invoked = await call("action_invoke", { capabilityId: "note.s.note.note@1/*#editor.addBlock", input: { kind: "text" } });
  const committedAt = Date.now();
  let head = -1;
  for (let tick = 0; tick < 120 && head < 1; tick += 1) {
    head = Number((await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token)).json?.head_seq ?? -1);
    if (head < 1) await pause(1_000);
  }
  const reopened = await call("artifact_open", { artifactId: documentId });
  console.log(`round ${round}: open=${opened.isError ? "ERR" : opened.structuredContent?.sessionDocument?.writePath} invoke=${invoked.structuredContent?.status} postconditions=${JSON.stringify(invoked.structuredContent?.postconditions)} warnings=${JSON.stringify(invoked.structuredContent?.warnings)} head_seq=${head} after ${((Date.now() - committedAt) / 1000).toFixed(1)} s (session ${((committedAt - started) / 1000).toFixed(1)} s) sync=${JSON.stringify(reopened.structuredContent?.sessionDocument?.sync)}`);
  if (head < 1) console.log(`  stderr tail: ${agent.stderrText().slice(-3000).replace(/\n/g, " | ")}`);
  await agent.close();
  await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(String(delegation.json?.delegationId ?? ""))}`, token);
}
