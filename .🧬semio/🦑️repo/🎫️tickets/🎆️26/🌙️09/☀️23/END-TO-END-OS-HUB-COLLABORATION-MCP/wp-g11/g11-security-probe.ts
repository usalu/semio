#!/usr/bin/env bun
/** 🛡️ G11 item 4: the semio MCP's security boundary against a live hub, measured with the official
 * `@modelcontextprotocol/sdk` client as the third-party oracle. A human (user2) signs in, owns a fresh space with one note
 * and delegates agents; each agent is a real `semio-os-mcp stdio --hub` process.
 *   S1 scopes: the same delegation under a read-only grant and under the full grant — every tool the read-only grant does
 *      not cover is refused `PERMISSION_DENIED` (and changes nothing), and the full grant is never refused for a scope.
 *   S2 audience: a `read` delegation whose gateway claims `artifact.write` still cannot edit (the hub caps it).
 *   S3 rate limit: a burst of agent-session exchanges for one delegation is refused `429 rate-limited` with `Retry-After`.
 *   S4 revocation: a connected agent edits, the human revokes, the agent's next request is refused and the hub closes the
 *      agent's sockets (hub trace), timed from the revoke answer.
 * usage: bun g11-security-probe.ts <hubOrigin> <captureDir> <hubStateDir>   (SEMIO_OS_MCP_BIN = the gateway binary) */
import { chmodSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { agentDelegationRevokePathV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🤖️delegations/🟦️.ts";

const [HUB, OUT, HUB_STATE] = process.argv.slice(2);
const BINARY = process.env.SEMIO_OS_MCP_BIN ?? "";
if (!HUB || !OUT || !HUB_STATE || !existsSync(BINARY)) throw new Error("usage: SEMIO_OS_MCP_BIN=<gateway> bun g11-security-probe.ts <hubOrigin> <captureDir> <hubStateDir>");
mkdirSync(OUT, { recursive: true });
const CREDENTIALS = join(OUT, "credentials");
mkdirSync(CREDENTIALS, { recursive: true, mode: 0o700 });
const started = Date.now();
const seconds = () => ((Date.now() - started) / 1000).toFixed(1);
const rows: { step: string; ok: boolean; detail: string }[] = [];
const row = (step: string, ok: boolean, detail: string): void => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}`);
};
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json, retryAfter: response.headers.get("retry-after") };
};

const human = String((await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user2@semio.dev", password: "gm1-local-dev-pass-2", deviceInstanceId: `g11security${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const spaceName = `G11 security ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", human, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", human)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, human);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Security probe" })))).json;
for (const deadline = Date.now() + 1_800_000; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
  await pause(1_000);
  creation = (await hub("GET", `${creations}/${requestId}`, human)).json;
}
const noteId = String(creation?.ready?.artifactId ?? "");
row("0 setup: user2 owns a fresh space with one note", spaceId.length > 0 && noteId.length > 0, `space=${spaceId} note=${noteId}`);

/** 🎫️ One delegation turned into a 0600 credential file, exactly the `semio.hub.agent-credential/v1` the shell writes. */
const delegate = async (label: string, audience: "read" | "edit") => {
  const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience, ttlSecs: 900 }));
  const path = join(CREDENTIALS, `${label.replace(/[^a-z0-9]+/giu, "-")}.json`);
  writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience, token: created.json?.token }), { mode: 0o600 });
  chmodSync(path, 0o600);
  return { id: String(created.json?.delegationId ?? ""), token: String(created.json?.token ?? ""), path, status: created.status };
};
/** 🔌️ The official SDK client on a real gateway process bound to the hub space under `scopes`. */
const connect = async (name: string, credential: string, scopes: string) => {
  const client = new Client({ name, version: "1" }, { capabilities: {} });
  const transport = new StdioClientTransport({ command: BINARY, args: ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", credential, "--scopes", scopes, "--no-bridge", "--auto-approve", "all"], env: { ...process.env } as Record<string, string>, stderr: "pipe" });
  await client.connect(transport, { timeout: 600_000 });
  return client;
};
const call = async (client: Client, name: string, args: Record<string, unknown>) => {
  const answer: any = await client.callTool({ name, arguments: args }, undefined, { timeout: 600_000 }).catch((error: unknown) => ({ isError: true, structuredContent: { code: "CLIENT", message: String(error) } }));
  return { isError: answer.isError === true, code: String(answer.structuredContent?.code ?? ""), content: answer.structuredContent ?? {}, message: String(answer.structuredContent?.message ?? "").slice(0, 200) };
};

const editor = await delegate("G11 security editor", "edit");
const reader = await connect("g11-scope-read", editor.path, "workspace.read");
const full = await connect("g11-scope-full", editor.path, "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write");
const search: any = await full.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } }, undefined, { timeout: 120_000 });
const addBlock = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((id) => id.endsWith(".addBlock")) ?? "");
const probes: readonly { tool: string; args: Record<string, unknown>; grantedBy: string }[] = [
  { tool: "context_resolve", args: {}, grantedBy: "workspace.read" },
  { tool: "capabilities_search", args: { query: "add a block" }, grantedBy: "workspace.read" },
  { tool: "artifact_open", args: { artifactId: noteId }, grantedBy: "workspace.read" },
  { tool: "artifact_snapshot", args: { artifactId: noteId }, grantedBy: "workspace.read" },
  { tool: "action_prepare", args: { capabilityId: addBlock, input: { kind: "text" } }, grantedBy: "artifact.write" },
  { tool: "action_invoke", args: { capabilityId: addBlock, input: { kind: "text" } }, grantedBy: "artifact.write" },
  { tool: "inference_list", args: {}, grantedBy: "workspace.read" },
  { tool: "ui_reveal", args: { artifactId: noteId }, grantedBy: "ui.control" },
  { tool: "ui_focus", args: { artifactId: noteId }, grantedBy: "ui.control" },
  { tool: "conversation_reply", args: { text: "G11 scope probe" }, grantedBy: "conversation.write" },
];
const scopeRows: string[] = ["| tool | granted by | read-only grant | full grant |", "|---|---|---|---|"];
let scopeOk = true;
for (const probe of probes) {
  const narrow = await call(reader, probe.tool, probe.args);
  const wide = await call(full, probe.tool, probe.args);
  const narrowCovered = probe.grantedBy === "workspace.read";
  const ok = (narrowCovered ? narrow.code !== "PERMISSION_DENIED" : narrow.isError && narrow.code === "PERMISSION_DENIED") && wide.code !== "PERMISSION_DENIED";
  scopeOk &&= ok;
  scopeRows.push(`| \`${probe.tool}\` | \`${probe.grantedBy}\` | ${narrow.isError ? `refused ${narrow.code}` : "ok"} | ${wide.isError ? `refused ${wide.code} (${wide.message.slice(0, 60)})` : "ok"} |`);
}
row("S1 every tool outside a read-only grant is refused PERMISSION_DENIED; the full grant is never refused for a scope", scopeOk, `${probes.length} tools × 2 grants (table in scopes.md)`);
writeFileSync(join(OUT, "scopes.md"), `${scopeRows.join("\n")}\n`);
const headAfterNarrow = await call(full, "artifact_snapshot", { artifactId: noteId });
await reader.close().catch(() => undefined);

const readOnly = await delegate("G11 security reader", "read");
const readClient = await connect("g11-audience-read", readOnly.path, "workspace.read,artifact.write").catch((error: unknown) => error);
if (readClient instanceof Client) {
  const edit = await call(readClient, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
  row("S2 a read delegation cannot edit even when its gateway claims artifact.write (hub audience cap)", edit.isError, edit.isError ? `refused ${edit.code}: ${edit.message}` : `EDITED: ${JSON.stringify(edit.content).slice(0, 160)}`);
  await readClient.close().catch(() => undefined);
} else row("S2 a read delegation cannot edit even when its gateway claims artifact.write (hub audience cap)", true, `the read gateway refused to start with a write grant: ${String(readClient).slice(0, 200)}`);

const burst = await delegate("G11 security burst", "edit");
const answers: { status: number; retryAfter: string | null; error: string }[] = [];
for (let attempt = 0; attempt < 14; attempt += 1) {
  const answer = await hub("POST", "/auth/agent-sessions", burst.token, JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: "edit", agentInstanceId: `g11burst${attempt}` }));
  answers.push({ status: answer.status, retryAfter: answer.retryAfter, error: String(answer.json?.error ?? "") });
}
const refused = answers.filter((answer) => answer.status === 429);
row("S3 a burst of agent-session exchanges for one delegation is rate-limited (429 + Retry-After)", refused.length > 0 && refused.every((answer) => Number(answer.retryAfter ?? 0) > 0), `statuses ${answers.map((answer) => answer.status).join(",")}; first refusal ${JSON.stringify(refused[0] ?? null)}`);

const beforeRevoke = await call(full, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
const revokeStarted = Date.now();
const revoke = await hub("POST", agentDelegationRevokePathV1(editor.id), human);
const revokeMs = Date.now() - revokeStarted;
const afterRevoke = await call(full, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
const refusedMs = Date.now() - revokeStarted;
row("S4a a connected agent edits, the human revokes, the agent's very next request is refused", beforeRevoke.content?.status === "SUCCEEDED" && revoke.status < 300 && afterRevoke.isError, `before=${beforeRevoke.content?.status ?? beforeRevoke.code} revoke=${revoke.status} in ${revokeMs} ms; next=${afterRevoke.isError ? `refused ${afterRevoke.code} ${afterRevoke.message}` : `EDITED ${afterRevoke.content?.status}`} at +${refusedMs} ms`);
await pause(3_000);
const capture = existsSync(join(HUB_STATE, "capture.txt")) ? readFileSync(join(HUB_STATE, "capture.txt"), "utf8") : "";
const agentPrincipal = `agent:${editor.id}`;
const closes = capture.split("\n").filter((line) => line.includes(agentPrincipal) && /socket/u.test(line) && /closed|revoked|4401/u.test(line));
row("S4b the hub closes the revoked agent's sockets (hub trace)", closes.length > 0, closes.length > 0 ? closes.slice(-3).map((line) => line.slice(0, 220)).join(" | ") : `no socket close for ${agentPrincipal} in the hub capture tail (${capture.split("\n").length} lines)`);
await full.close().catch(() => undefined);
writeFileSync(join(OUT, "security-rows.json"), JSON.stringify({ rows, headAfterNarrow: headAfterNarrow.content?.revision ?? null }, null, 2));
const red = rows.filter((entry) => !entry.ok);
console.log(`g11-security-probe: ${rows.length - red.length}/${rows.length} rows green in ${seconds()} s`);
process.exit(red.length === 0 ? 0 : 1);
