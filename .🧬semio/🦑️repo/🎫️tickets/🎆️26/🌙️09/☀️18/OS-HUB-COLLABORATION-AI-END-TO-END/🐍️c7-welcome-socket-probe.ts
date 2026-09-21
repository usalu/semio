/** 📬️ C7 — read the `Welcome` frame's `bootstrap` variant off a real hub document socket.
 *
 * C6 §3.4's last hop is an inference from a live region that never rendered. This reads the byte:
 * the same hub handler the browser reaches (`POST /auth/sessions` → `…/open-plan` →
 * `…/socket-grants` → `GET …/socket/v1` upgraded with `Sec-WebSocket-Protocol: semio.socket.v1,
 * <grant>` → `SocketHelloV1`), decoded with the product's own `decodeServerFrame`, with no browser,
 * no plugin and no component in the way. Every server frame the hub sends in the window is printed
 * in order, so a `Bootstrap::None` is distinguished from an `ArtifactBootstrap` that simply arrives
 * late, and the follow-up `ArtifactBootstrapChunk`/`Done` frames are counted.
 *
 * Usage: bun 🐍️c7-welcome-socket-probe.ts <origin> [spaceId] [documentId] [surfaceId]
 * Env:   C7_TAG, C7_WINDOW_MS (20000), C7_USER (user1) */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

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

const repoRoot = findRepoRoot(import.meta.dir);
const { encodeClientFrame, decodeServerFrame } = await import(join(repoRoot, "🧰️framework", "🔨️modules", "📡️replication", "🟦️.ts"));

const origin = process.argv[2] ?? "http://127.0.0.1:7621";
const spaceId = process.argv[3] ?? "01a0c314-e41f-780d-a980-3adda40ca9f7";
const documentId = process.argv[4] ?? "artifact-0954e2d10d8fff9605f101b0dba34f3b";
const surfaceId = process.argv[5] ?? "s.gis.gismap@1/*#editor";
const TAG = process.env.C7_TAG ?? "c7";
const WINDOW_MS = Number(process.env.C7_WINDOW_MS ?? 20_000);
const USERS: Record<string, { email: string; password: string }> = {
  user1: { email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  user2: { email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
};
const who = process.env.C7_USER ?? "user1";
const user = USERS[who]!;
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const t0 = Date.now();
const ms = (): number => Date.now() - t0;
const report: Record<string, any> = { origin, spaceId, documentId, surfaceId, who, startedAt: new Date().toISOString(), frames: [] };
const save = (): void => writeFileSync(join(OUT, `${TAG}-welcome-socket.json`), JSON.stringify(report, null, 2));
const say = (line: string): void => console.log(`${String(ms()).padStart(7)} ${line}`);

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: user.email, password: user.password, deviceInstanceId: `c7-${who}`, clientClass: "browser" }),
});
if (minted.status !== 200) throw new Error(`${who} sign-in ${minted.status}`);
const token = ((await minted.json()) as { token?: string }).token ?? "";
const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };

const planResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
  method: "POST",
  headers,
  body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `c7-${who}` }),
});
const planText = await planResponse.text();
if (planResponse.status !== 200) throw new Error(`open-plan ${planResponse.status} ${planText.slice(0, 300)}`);
const plan = JSON.parse(planText) as { receipt: string; artifact: { schema: string; packSchemaHash: string }; checkpoint?: Record<string, any> };
report.checkpoint = plan.checkpoint ?? null;
say(`OPEN-PLAN schema=${plan.artifact.schema} checkpoint=${JSON.stringify(plan.checkpoint ?? null).slice(0, 320)}`);

const packSchemaHash = [...(plan.artifact.packSchemaHash.match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
const grantResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
  method: "POST",
  headers,
  body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
});
const grantText = await grantResponse.text();
if (grantResponse.status !== 200) throw new Error(`socket-grant ${grantResponse.status} ${grantText.slice(0, 300)}`);
const receipt = JSON.parse(grantText) as { grant: string; protocol: string; actorId: string };

const wsUrl = `${origin.replace(/^http/u, "ws")}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket/v1?surface=${encodeURIComponent(surfaceId)}`;
const socket = new WebSocket(wsUrl, [receipt.protocol, receipt.grant]);
socket.binaryType = "arraybuffer";
socket.addEventListener("message", (event) => {
  if (!(event.data instanceof ArrayBuffer)) return;
  const bytes = new Uint8Array(event.data);
  let frame: any;
  try {
    frame = decodeServerFrame(bytes).frame;
  } catch (error) {
    report.frames.push({ at: ms(), bytes: bytes.length, decodeError: String(error).slice(0, 200) });
    save();
    return;
  }
  const kind = typeof frame === "string" ? frame : Object.keys(frame)[0];
  const row: Record<string, any> = { at: ms(), bytes: bytes.length, kind };
  if (kind === "Welcome") {
    const bootstrap = frame.Welcome.bootstrap;
    row.bootstrap = typeof bootstrap === "string" ? bootstrap : Object.keys(bootstrap)[0];
    row.serverFrontier = { document_id: frame.Welcome.server_frontier.document_id, head_edit_ordinal: frame.Welcome.server_frontier.head_edit_ordinal, head_edit_id: frame.Welcome.server_frontier.head_edit_id, last_commit_seq: frame.Welcome.server_frontier.last_commit_seq };
    if (row.bootstrap === "ArtifactBootstrap") {
      const artifact = bootstrap.ArtifactBootstrap;
      row.artifactBootstrap = { format_version: artifact.format_version, artifact_schema: artifact.artifact_schema, artifact_kind: artifact.artifact_kind, pack_length: artifact.pack_length, spr_length: artifact.spr_length, chunk_count: artifact.chunk_count, inline: artifact.inline !== null };
    }
    report.welcome = row;
  }
  if (kind === "Error") row.error = frame.Error;
  report.frames.push(row);
  say(`FRAME ${kind}${row.bootstrap ? ` bootstrap=${row.bootstrap}` : ""} ${bytes.length}B`);
  save();
});
socket.addEventListener("close", (event) => {
  report.close = { at: ms(), code: event.code, reason: event.reason };
  say(`WS-CLOSE code=${event.code} reason=${event.reason}`);
  save();
});
for (let wait = 0; wait < 200 && socket.readyState === WebSocket.CONNECTING; wait += 1) await Bun.sleep(50);
if (socket.readyState !== WebSocket.OPEN) throw new Error(`socket did not open (state ${socket.readyState})`);
socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
say(`HELLO actor=${receipt.actorId}`);

await Bun.sleep(WINDOW_MS);
socket.close();
await Bun.sleep(500);
save();
console.log("");
console.log(`WELCOME.bootstrap ${report.welcome?.bootstrap ?? "<no Welcome frame>"}`);
console.log(`SERVER-FRONTIER ${JSON.stringify(report.welcome?.serverFrontier ?? null)}`);
console.log(`ARTIFACT-BOOTSTRAP ${JSON.stringify(report.welcome?.artifactBootstrap ?? null)}`);
console.log(`FRAME-KINDS ${JSON.stringify(report.frames.map((frame: any) => frame.kind ?? "decode-error"))}`);
console.log(`CHECKPOINT ${JSON.stringify(report.checkpoint)}`);
