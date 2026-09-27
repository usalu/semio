/** 🔬️ G12: one hub-created document of `<kindId>`, opened by a delegated agent over the semio MCP — prints the open's
 * `sessionDocument` (write path, sync) and one export attempt. Credentials from env. usage: bun g12-open-report-probe.ts <hub> <kindId> */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { McpClientSession, requireMcpBinary } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
const [HUB, KIND] = [process.argv[2]!, process.argv[3]!];
const repoRoot = "/Users/ueli/Documents/semio";
const call = async (method: string, path: string, token?: string, body?: string) => (await fetch(`${HUB}${path}`, { method, headers: { ...(body ? { "content-type": "application/json" } : {}), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body ? { body } : {}) })).json().catch(() => null);
const token = String((await call("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL, password: process.env.OS_MCP_HUB_PASSWORD, deviceInstanceId: `g12open${randomBytes(10).toString("hex")}`, clientClass: "browser" })))?.token ?? "");
const name = `G12 open ${randomBytes(3).toString("hex")}`;
await call("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))));
const spaceId = String(((await call("GET", "/directory/spaces", token)) ?? []).find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await call("GET", creations, token);
const requestId = randomBytes(16).toString("hex");
let creation = await call("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog?.catalogGenerationId ?? ""), kindId: KIND, name: "Open probe" })));
while (["accepted", "preparing"].includes(creation?.phase)) { await new Promise((r) => setTimeout(r, 1000)); creation = await call("GET", `${creations}/${requestId}`, token); }
const documentId = String(creation?.ready?.artifactId ?? "");
const delegation = await call("POST", "/auth/agent-delegations", token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "G12 open probe", audience: "edit", ttlSecs: 900 }));
const path = join(mkdtempSync(join(tmpdir(), "g12-open-")), "credential.json");
writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: delegation?.token }), { mode: 0o600 });
chmodSync(path, 0o600);
const session = new McpClientSession({ command: process.env.SEMIO_OS_MCP_BIN ?? requireMcpBinary(repoRoot), args: ["stdio", "--scopes", "workspace.read,artifact.write"] }, ["--hub", HUB, "--space", spaceId, "--credential-file", path, "--no-bridge"], repoRoot);
await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g12-open", version: "1" } });
session.notify("notifications/initialized", {});
const opened = await session.call("artifact_open", { artifactId: documentId }, 600_000);
console.log(`document=${documentId} open=${JSON.stringify(opened.structuredContent).slice(0, 1500)}`);
const exported = await session.call("artifact_export", { artifactId: documentId }, 600_000);
console.log(`export=${JSON.stringify(exported.structuredContent).slice(0, 600)}`);
session.stop();
process.exit(0);
