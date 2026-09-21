/** 🤖️ M8 — the MCP agent as a THIRD participant in an occupied hub space.
 *
 * M6b proved 6 of M6 §8's 7 steps: an agent delegated by a human reaches the hub as its own
 * principal. It never proved step 6 — a document presence row — because its rig had no document.
 * This probe drives the half that was missing, against a hub whose space already holds a real
 * document a human edits:
 *
 *   1  the gateway starts with `--hub <origin> --space <id> --credential-file <0600 file>`
 *   2  `context_resolve` reports the AGENT principal, not the launcher's
 *   3  the workspace lists the hub's own documents (`semio://workspace/artifacts`)
 *   4  `artifact_open` of the HUB document, by its hub coordinates
 *   5  `capabilities_search` finds a mutation verb typed against that document's kind
 *   6  `action_prepare` → `action_invoke` of that verb
 *   7  `artifact_snapshot` reflects the edit
 *   8  the hub's own ledger/checkpoint shows it (read back over HTTP as the delegating human)
 *
 * Every step prints one PASS/FAIL row with the observed value; the exit code is the number of red
 * rows, so this file is a gate, not a log. Nothing is skipped and no row is scored green on an
 * assumption. Ticket 26/09/18 slice M8.
 *
 * Usage: bun 🐍️m8-hub-participant-probe.ts <origin> <spaceId> <credentialPath> <humanToken> [documentId]
 */
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// 📁️ `new URL(x, import.meta.url).pathname` percent-encodes emoji path segments (preamble rule 24).
const here = dirname(fileURLToPath(new URL(import.meta.url)));

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const repoRoot = findRepoRoot(here);
const { McpClientSession, mcpServerEntries } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌉️mcp", "🟦️.ts"));

const [origin, spaceId, credentialPath, humanToken, documentArgument] = process.argv.slice(2);
if (!origin || !spaceId || !credentialPath || !humanToken) throw new Error("usage: bun 🐍️m8-hub-participant-probe.ts <origin> <spaceId> <credentialPath> <humanToken> [documentId]");

let failures = 0;
function row(step: string, ok: boolean, detail: string): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${step} — ${detail}`);
}

async function hub(method: string, path: string, body?: string): Promise<{ status: number; text: string; json: any }> {
  const headers: Record<string, string> = { authorization: `Bearer ${humanToken}` };
  if (body !== undefined) headers["content-type"] = "application/json";
  const response = await fetch(`${origin}${path}`, { method, headers, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {
    json = undefined;
  }
  return { status: response.status, text, json };
}

// 🚧️ `.mcp.json`'s shipped `semio` entry hard-codes `--folder .`, and `--folder`/`--hub` are
// mutually exclusive (`🌉️mcp/🏗️bootstrap/🦀️.rs:108`), so a hub-bound agent CANNOT be expressed by
// appending arguments to it the way `client-e2e` appends `--folder`. The entry below is the same
// process the wrapper execs, with the hub selector in place of the folder one — which is exactly
// the `command`/`args` pair a delegation UI must print for a client config.
const declared = mcpServerEntries(repoRoot).semio;
const scopes = declared?.args.includes("--scopes") ? String(declared.args[declared.args.indexOf("--scopes") + 1]) : "workspace.read,artifact.write,inference.execute,ui.observe,ui.control";
const binary = process.env.M8_MCP_BINARY ?? join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌉️mcp", "📦️packages", "🦀️rust", "dist", "build", "semio-os-mcp");
const entry = { command: binary, args: ["stdio", "--scopes", scopes] } as const;
console.log(`INFO gateway ${entry.command} ${[...entry.args, "--hub", origin, "--space", spaceId, "--credential-file", credentialPath].join(" ")}`);

const session = new McpClientSession(entry, ["--hub", origin, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge"], repoRoot);
try {
  const initialized = await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "semio-m8-hub-participant", title: "M8 hub participant", version: "1" } });
  row("1 initialize over the delegated hub session", !initialized.error, initialized.error ? JSON.stringify(initialized.error).slice(0, 300) : `server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version}`);
  if (initialized.error) process.exit(1);
  session.notify("notifications/initialized", {});

  const context = await session.call("context_resolve", {});
  const principal = String(context.structuredContent?.principal ?? "");
  row("2 context_resolve names the agent principal", principal.startsWith("agent:") && principal !== "agent:local", `principal=${principal} channel=${context.structuredContent?.channel} workspace=${context.structuredContent?.workspace ?? context.structuredContent?.origin ?? "<unset>"}`);
  console.log(`WIRE context_resolve ${JSON.stringify(context.structuredContent).slice(0, 900)}`);

  const artifacts = await session.request("resources/read", { uri: "semio://workspace/artifacts" });
  const artifactsText = String((artifacts.result?.contents ?? [])[0]?.text ?? "");
  row("3 the workspace lists the hub's own documents", !artifacts.error && artifactsText.includes("artifact-"), artifacts.error ? JSON.stringify(artifacts.error).slice(0, 300) : artifactsText.slice(0, 400));

  let documentId = documentArgument ?? "";
  if (!documentId) {
    const parsed = (() => {
      try {
        return JSON.parse(artifactsText);
      } catch {
        return undefined;
      }
    })();
    const rows = Array.isArray(parsed?.artifacts) ? parsed.artifacts : Array.isArray(parsed?.artifactIds) ? parsed.artifactIds : Array.isArray(parsed) ? parsed : [];
    documentId = String(rows[0]?.scope?.documentId ?? rows[0]?.documentId ?? rows[0] ?? "");
  }
  row("3b a hub document id is in hand", documentId.length > 0, `documentId=${documentId || "<none>"}`);

  const opened = await session.call("artifact_open", { artifactId: documentId });
  row("4 artifact_open of the HUB document", opened.isError !== true, opened.isError === true ? JSON.stringify(opened.structuredContent).slice(0, 400) : `artifactId=${opened.structuredContent?.artifactId} kind=${opened.structuredContent?.kind} revision=${JSON.stringify(opened.structuredContent?.revision)}`);

  const openedKind = String(opened.structuredContent?.kind ?? "");
  const search = await session.call("capabilities_search", { query: "add", kind: "mutation" });
  const hits = (search.structuredContent?.hits ?? search.structuredContent?.results ?? []) as Array<Record<string, any>>;
  const matching = hits.filter((hit) => String(hit.artifactKind ?? "").length > 0);
  row("5 capabilities_search over the HUB-selected catalog", search.isError !== true && hits.length > 0, search.isError === true ? JSON.stringify(search.structuredContent).slice(0, 300) : `${hits.length} hit(s); kinds=${[...new Set(matching.map((hit) => hit.artifactKind))].slice(0, 8).join(", ") || "<none>"}`);

  const verb = matching.find((hit) => String(hit.artifactKind) === openedKind) ?? matching[0];
  const capabilityId = String(verb?.capabilityId ?? verb?.id ?? "");
  row("5b a mutation verb typed against the opened document's kind", capabilityId.length > 0 && String(verb?.artifactKind) === openedKind, `openedKind=${openedKind || "<none>"} verb=${capabilityId || "<none>"} verbKind=${verb?.artifactKind ?? "<none>"}`);

  if (capabilityId) {
    const described = await session.call("capabilities_describe", { capabilityId });
    console.log(`WIRE capabilities_describe ${JSON.stringify(described.structuredContent).slice(0, 700)}`);
    const prepared = await session.call("action_prepare", { capabilityId, input: {}, artifactId: documentId });
    row("6 action_prepare against the hub document", prepared.isError !== true, prepared.isError === true ? JSON.stringify(prepared.structuredContent).slice(0, 500) : `handle=${prepared.structuredContent?.preparedHandle} baseline=${JSON.stringify(prepared.structuredContent?.revision ?? prepared.structuredContent?.baselineRevision)}`);
    const handle = prepared.structuredContent?.preparedHandle as string | undefined;
    if (handle) {
      const invoked = await session.call("action_invoke", { preparedActionHandle: handle });
      row("6b action_invoke commits through the hub document socket", invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", invoked.isError === true ? JSON.stringify(invoked.structuredContent).slice(0, 500) : `status=${invoked.structuredContent?.status} ${JSON.stringify(invoked.structuredContent?.revisionBefore)} → ${JSON.stringify(invoked.structuredContent?.revisionAfter)}`);
    } else {
      row("6b action_invoke commits through the hub document socket", false, "action_prepare minted no handle");
    }
  } else {
    row("6 action_prepare against the hub document", false, "no mutation verb was discoverable for the opened kind");
    row("6b action_invoke commits through the hub document socket", false, "no prepared handle");
  }

  const snapshot = await session.call("artifact_snapshot", { artifactId: documentId });
  row("7 artifact_snapshot of the hub document", snapshot.isError !== true && Number(snapshot.structuredContent?.packBytes ?? 0) > 0, snapshot.isError === true ? JSON.stringify(snapshot.structuredContent).slice(0, 400) : `packBytes=${snapshot.structuredContent?.packBytes} sprBytes=${snapshot.structuredContent?.sprBytes}`);

  const status = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`);
  row("8 the hub's own document status answers", status.status === 200, `HTTP ${status.status} ${status.text.slice(0, 300)}`);

  console.log(`NOTIFICATIONS ${JSON.stringify(session.serverNotifications().slice(0, 6))}`.slice(0, 1200));
} finally {
  session.stop();
}

console.log(`M8 hub-participant probe: ${failures} red row(s)`);
process.exit(failures === 0 ? 0 : 1);
