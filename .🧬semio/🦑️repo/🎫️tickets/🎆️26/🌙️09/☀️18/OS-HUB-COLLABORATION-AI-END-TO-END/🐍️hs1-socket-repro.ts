/** 🧵️ HS1 — the cheapest deterministic reproduction of the hub's pool-worker stack overflow.
 *
 * C2 reached the hub document socket through two real browser contexts on a vite serve. That is
 * expensive and racy. The hub only needs four calls to reach the very same handler:
 * `POST /auth/sessions` → `POST …/open-plan` → `POST …/socket-grants` → `GET …/socket/v1` upgraded
 * with `Sec-WebSocket-Protocol: semio.socket.v1, <grant>`. The execution-target manifest/component/
 * descriptor reads are a CLIENT-side lease (`👷️worker/🟦️.ts:2720`) and the hub never requires them,
 * so this probe skips them and still lands on `document_ws_v1` → `handle_ws`.
 *
 * Prints one line per stage and, once attached, holds the socket while sending `n` edits, so the
 * "survives 5 minutes of idle + 20 edits" bar can be measured from the same process.
 *
 * The `SocketHelloV1` frame is NOT optional: `handle_ws` waits 2 s for it and answers
 * `error_frame("protocol", "expected socket hello")` otherwise, so a probe that only upgrades never
 * reaches a single line of the document session. It is built with the product's OWN
 * `encodeClientFrame` (`📡️replication/🟦️.ts`), and its `schema`/`pack_schema_hash` are taken from the
 * open plan's `artifact` block, which is exactly what the hub compares against the durable
 * descriptor.
 *
 * Usage: `bun 🐍️hs1-socket-repro.ts <origin> <spaceId> <documentId> [surfaceId] [holdMs] [edits]` */
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";

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

const origin = process.argv[2] ?? "http://127.0.0.1:7631";
const spaceId = process.argv[3]!;
const documentId = process.argv[4]!;
const surfaceId = process.argv[5] ?? "s.gis.gismap@1/*#editor";
const holdMs = Number(process.argv[6] ?? 15_000);
const edits = Number(process.argv[7] ?? 0);
const email = process.env.HS1_EMAIL ?? "user1@semio.dev";
const password = process.env.HS1_PASSWORD ?? "gm1-local-dev-pass-1";

const stamp = () => new Date().toISOString().slice(11, 23);
const say = (line: string) => console.log(`${stamp()} ${line}`);

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "hs1-socket-repro", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as { token?: string })?.token ?? "";
say(`SIGN-IN status=${minted.status} tokenShape=${/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(token)}`);
if (minted.status !== 200) process.exit(2);

const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
const planResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
  method: "POST",
  headers,
  body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: "hs1-socket-repro" }),
});
const planText = await planResponse.text();
say(`OPEN-PLAN status=${planResponse.status}`);
if (planResponse.status !== 200) {
  say(`OPEN-PLAN body=${planText.slice(0, 400)}`);
  process.exit(3);
}
const plan = JSON.parse(planText) as { receipt: string; surface?: { surfaceId?: string }; artifact: { schema: string; packSchemaHash: string } };
say(`OPEN-PLAN surface=${plan.surface?.surfaceId ?? "<none>"} schema=${plan.artifact.schema} packSchemaHash=${plan.artifact.packSchemaHash.slice(0, 16)}…`);
const packSchemaHash = [...(plan.artifact.packSchemaHash.match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));

const grantResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
  method: "POST",
  headers,
  body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
});
const grantText = await grantResponse.text();
say(`SOCKET-GRANT status=${grantResponse.status}`);
if (grantResponse.status !== 200) {
  say(`SOCKET-GRANT body=${grantText.slice(0, 400)}`);
  process.exit(4);
}
const receipt = JSON.parse(grantText) as { grant: string; protocol: string; actorId: string };

const wsUrl = `${origin.replace(/^http/u, "ws")}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket/v1?surface=${encodeURIComponent(surfaceId)}`;
say(`WS-CONNECT ${wsUrl}`);
const socket = new WebSocket(wsUrl, [receipt.protocol, receipt.grant]);
socket.binaryType = "arraybuffer";
let frames = 0;
let opened = false;
socket.addEventListener("open", () => {
  opened = true;
  say(`WS-OPEN protocol=${socket.protocol}`);
  const hello = encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command");
  socket.send(hello);
  say(`WS-HELLO sent bytes=${hello.byteLength}`);
});
socket.addEventListener("message", (event) => {
  frames += 1;
  if (!(event.data instanceof ArrayBuffer)) {
    if (frames <= 12) say(`WS-FRAME #${frames} text ${String(event.data).slice(0, 200)}`);
    return;
  }
  let named = `binary bytes=${event.data.byteLength}`;
  try {
    named = `${Object.keys(decodeServerFrame(new Uint8Array(event.data)).frame)[0]} bytes=${event.data.byteLength}`;
  } catch (error) {
    named = `undecodable bytes=${event.data.byteLength} (${(error as Error).message.slice(0, 80)})`;
  }
  if (frames <= 12) say(`WS-FRAME #${frames} ${named}`);
});
socket.addEventListener("error", () => say("WS-ERROR"));
socket.addEventListener("close", (event) => say(`WS-CLOSE code=${event.code} reason=${event.reason}`));

let alive = true;
// ⏳️ The upgrade is still in flight here: without this the frame loop below sees `CONNECTING` and
// sends nothing while still reporting success.
for (let wait = 0; wait < 100 && socket.readyState === WebSocket.CONNECTING; wait += 1) await Bun.sleep(50);
// 🖊️ Well-formed client frames on the live socket, sent BEFORE the hold: the hub's session token
// expires this connection at ~30 s with `4401 unauthorized`, so anything that must run on an open
// socket runs first. `Presence` is a real `ClientFrame` the hub decodes and broadcasts — this is not
// a semantic document edit (that needs a sealed `Commands` batch), and the report says so.
for (let index = 0; index < edits && socket.readyState === WebSocket.OPEN; index += 1) {
  socket.send(encodeClientFrame({ Presence: { peer: [...new Uint8Array(8).fill(index + 1)] } }, "command"));
  await Bun.sleep(100);
  const beat = await fetch(`${origin}/readyz`, { signal: AbortSignal.timeout(3_000) }).catch(() => undefined);
  if (beat === undefined) {
    say(`HUB-GONE after client frame ${index + 1}`);
    alive = false;
    break;
  }
}
if (edits > 0 && alive) say(`WS-FRAMES-SENT ${edits} presence frames, hub answered /readyz after each`);
const deadline = Date.now() + holdMs;
while (alive && Date.now() < deadline) {
  await Bun.sleep(5_000);
  const probe = await fetch(`${origin}/readyz`, { signal: AbortSignal.timeout(5_000) }).catch(() => undefined);
  if (probe === undefined) {
    say("HUB-GONE /readyz did not answer — the process is down");
    alive = false;
    break;
  }
}
say(`RESULT opened=${opened} frames=${frames} readyState=${socket.readyState} hubAlive=${alive}`);
socket.close();
process.exit(alive && opened ? 0 : 1);
