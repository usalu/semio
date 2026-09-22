/** ✍️ M10 — what a hub-bound agent's WRITE path reports about itself, measured over a live MCP
 * session rather than argued from source.
 *
 * M9 proved the agent can read a hub document and stopped at `wasmtime` instantiate. This probe
 * measures the three legs M10 added, each of which is observable from `artifact_open`'s own answer
 * now that it reports the binding it mints (`sessionDocument`):
 *   1  the document is bound to the plugin/app the HUB's execution-target lease names
 *   2  on the lease's own SURFACE — never the pinned `os.mcp.probe.editor`
 *   3  with a `writePath` that is either `open` (document actor + socket up) or the exact reason
 *      it is not
 * and then drives `action_prepare` so the row after those three is measured too.
 *
 * Usage: bun 🐍️m10-hub-write-path-probe.ts <origin> <spaceId> <credentialPath> [documentId]
 * Every step prints one PASS/FAIL/INFO row; the exit code is the number of red rows.
 */
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
const repoRoot = join(here, "..", "..", "..", "..", "..", "..", "..");
const binary = process.env.M10_MCP_BINARY ?? join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌉️mcp", "📦️packages", "🦀️rust", "dist", "build", "semio-os-mcp");
const [origin, spaceId, credentialPath, documentArgument] = process.argv.slice(2);
if (!origin || !spaceId || !credentialPath) throw new Error("usage: bun 🐍️m10-hub-write-path-probe.ts <origin> <spaceId> <credentialPath> [documentId]");

let red = 0;
const row = (ok: boolean | null, step: string, detail: string) => {
  if (ok === false) red += 1;
  console.log(`${ok === null ? "INFO" : ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
};

const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
const stderr: string[] = [];
child.stderr.on("data", (chunk) => stderr.push(chunk.toString()));
let buffer = "";
const pending = new Map<number, (message: any) => void>();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let index: number;
  while ((index = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, index).trim();
    buffer = buffer.slice(index + 1);
    if (!line) continue;
    let message: any;
    try { message = JSON.parse(line); } catch { continue; }
    const resolve = pending.get(message.id);
    if (resolve) { pending.delete(message.id); resolve(message); }
  }
});
let nextId = 1;
const call = (method: string, params: unknown): Promise<any> => new Promise((resolve) => {
  const id = nextId++;
  pending.set(id, resolve);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
const timer = setTimeout(() => { row(false, "probe", "timed out after 180 s"); child.kill(); process.exit(red || 1); }, 180_000);

const initialized = await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10-write-path-probe", version: "1" } });
row(Boolean(initialized?.result), "1 the gateway serves over the delegated hub session", `server=${initialized?.result?.serverInfo?.name}@${initialized?.result?.serverInfo?.version}`);
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);

const context = await call("tools/call", { name: "context_resolve", arguments: {} });
row(String(context?.result?.structuredContent?.principal ?? "").startsWith("agent:"), "2 context_resolve names the agent's own principal", `principal=${context?.result?.structuredContent?.principal}`);

const artifacts = await call("resources/read", { uri: "semio://workspace/artifacts" });
const listed = JSON.parse(artifacts?.result?.contents?.[0]?.text ?? "{}");
const documentId = documentArgument ?? (listed.artifacts ?? listed.ids ?? [])[0]?.artifactId ?? (listed.artifacts ?? [])[0] ?? (listed.ids ?? [])[0];
row(Boolean(documentId), "3 the agent sees the space's own documents", `documentId=${documentId}`);

const opened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
const open = opened?.result?.structuredContent;
row(!opened?.result?.isError, "4 artifact_open of a HUB document answers", `kind=${open?.kind} artifactKind=${open?.artifactKind} sizeBytes=${open?.sizeBytes}`);

const session = open?.sessionDocument;
row(Boolean(session), "5 artifact_open BINDS the document to a plugin session", session ? `pluginId=${session.pluginId} appId=${session.appId}` : "no sessionDocument in the answer");
row(Boolean(session?.surfaceId) && session?.surfaceId !== "os.mcp.probe.editor", "5b the binding carries the HUB lease's own surface, not the probe surface", `surfaceId=${session?.surfaceId}`);
row(Boolean(session) && session.packBytes > 0 && session.sprBytes > 0, "5c the binding holds the document's canonical pair for LoadDocument", `packBytes=${session?.packBytes} sprBytes=${session?.sprBytes}`);
row(session?.writePath === "open", "5d the document's write path is open (document actor + socket)", `writePath=${session?.writePath}`);

// 🔎️ `results`, the key `capabilities_search_output_shape` declares — the same read the
// `hub-agent-participant` gate makes, so the two lanes select the same verb.
const searched = await call("tools/call", { name: "capabilities_search", arguments: { query: "add", kind: "mutation" } });
const hits = (searched?.result?.structuredContent?.results ?? []) as Array<Record<string, any>>;
const verb = hits.find((capability) => capability.artifactKind === open?.artifactKind) ?? hits[0];
const capabilityId = String(verb?.id ?? verb?.capabilityId ?? "");
row(capabilityId !== "", "6 a mutation verb typed against the opened document's kind", `capability=${capabilityId} artifactKind=${verb?.artifactKind} hits=${hits.length}`);

// 🧾️ The verb's own required arguments, read off its catalog row rather than guessed: an empty
// `input` is refused by schema validation before the guest is ever reached, which is a probe
// defect wearing a gateway error (measured 2026-09-22: `missing required property \`kind\``).
const verbInput = Object.fromEntries(
  (verb?.arguments ?? verb?.args ?? []).filter((arg: any) => arg?.required).map((arg: any) => [arg.id, arg.default ?? (arg?.schema?.kind === "number" ? 0 : arg?.schema?.options?.[0]?.value ?? "")]),
);
const inputOverride = process.env.M10_ACTION_INPUT ? JSON.parse(process.env.M10_ACTION_INPUT) : undefined;
const actionInput = inputOverride ?? verbInput;
console.log(`INFO  6b action input — ${JSON.stringify(actionInput)}`);
const prepared = await call("tools/call", { name: "action_prepare", arguments: { capabilityId, input: actionInput } });
const preparedBody = prepared?.result?.structuredContent;
row(!prepared?.result?.isError, "7 action_prepare reaches a guest the HUB authorized", prepared?.result?.isError ? JSON.stringify(preparedBody) : `preparedHandle=${preparedBody?.preparedHandle}`);

const handle = preparedBody?.preparedHandle ?? preparedBody?.handle ?? preparedBody?.actionHandle;
if (handle) {
  const invoked = await call("tools/call", { name: "action_invoke", arguments: { preparedActionHandle: handle } });
  const outcome = invoked?.result?.structuredContent;
  row(!invoked?.result?.isError && outcome?.status === "SUCCEEDED", "8 action_invoke commits the agent's edit", `status=${outcome?.status} ${JSON.stringify(outcome?.revisionAfter ?? outcome).slice(0, 220)}`);
} else {
  row(false, "8 action_invoke commits the agent's edit", "action_prepare minted no handle");
}

// 🧾️ 9 — the HUB's own ledger, read as the delegating human. This is the row the bar is about:
// rows 7 and 8 are the guest's answer, and a guest can succeed while its envelopes go nowhere
// (measured 2026-09-22 on this very hub: `SUCCEEDED` with `head_seq 0`). The document actor
// connects its socket, takes the hub's artifact bootstrap and only then flushes its outbox, so
// this POLLS rather than reads once — and the gateway is deliberately kept alive while it does.
// 🧾️ …and re-open the artifact first, so `sessionDocument.relayedBatches` says whether the guest's
// envelopes even reached the document actor. "The guest published nothing" and "the actor has them
// and the socket has not flushed" are different defects and a probe that cannot tell them apart
// costs the next reader the measurement again.
const reopened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
const relayed = reopened?.result?.structuredContent?.sessionDocument;
row(Number(relayed?.relayedBatches ?? 0) > 0, "8b the committed envelopes reached the document actor", `relayedBatches=${relayed?.relayedBatches} writePath=${relayed?.writePath}`);

const humanToken = process.env.M10_HUMAN_TOKEN ?? "";
if (humanToken && documentId) {
  const deadline = Date.now() + Number(process.env.M10_LEDGER_WAIT_MS ?? 120_000);
  let ledger: any = {};
  let moved = false;
  while (Date.now() < deadline) {
    const response = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { headers: { authorization: `Bearer ${humanToken}` } });
    ledger = await response.json().catch(() => ({}));
    if (Number(ledger?.head_seq ?? 0) > 0 || Number(ledger?.commit_seq ?? 0) > 0) { moved = true; break; }
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }
  row(moved, "9 the HUB's own ledger carries the agent's Commands frame", `head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq} epoch=${ledger?.epoch}`);
} else {
  row(null, "9 the HUB's own ledger carries the agent's Commands frame", "no M10_HUMAN_TOKEN given — not attempted");
}

clearTimeout(timer);
console.log(`m10-write-path: ${red} red row(s)`);
if (stderr.length > 0) console.log(`--- gateway stderr (tail) ---\n${stderr.join("").split("\n").slice(-12).join("\n")}`);
child.kill();
process.exit(red);
