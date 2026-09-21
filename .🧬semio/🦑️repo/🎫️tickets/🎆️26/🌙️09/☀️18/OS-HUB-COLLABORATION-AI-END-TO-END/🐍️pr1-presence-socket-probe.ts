/** 👥️ PR1 — the hub's presence roster measured on TWO real document sockets held by TWO humans.
 *
 * The browser route to a document socket is closed on this catalog (the browser actor dies on its
 * first `reactor.poll`, JC1/C5's lane) and C3's browser reading was a single sample at attach time,
 * which cannot separate a join-replay gap from a lease decay. This probe reaches the very same hub
 * handler the browser would (`POST /auth/sessions` → `…/open-plan` → `…/socket-grants` →
 * `GET …/socket/v1` upgraded with `Sec-WebSocket-Protocol: semio.socket.v1, <grant>` → the
 * `SocketHelloV1` frame `handle_ws` waits 2 s for) with no plugin, no component and no browser, and
 * then reads what the HUB publishes to each socket over time. The per-socket roster is the last
 * `ServerFrame::Presence` that socket received — exactly what the shell projects into its chrome.
 *
 * `user1` attaches first and is left to settle so `user2` is a genuine LATE joiner.
 * `PR1_BEAT_MS=0` stops the client beats after the first one, which is the exact shape a wedged
 * `ephemeralSnapshot` produces in the real shell (`🏛️ShellHost/🟦️.tsx` `beatOneDocument`).
 *
 * Usage: bun 🐍️pr1-presence-socket-probe.ts <origin> [spaceId] [documentId] [surfaceId]
 * Env:   PR1_TAG, PR1_WINDOW_MS (120000), PR1_SAMPLE_MS (5000), PR1_BEAT_MS (5000),
 *        PR1_JOIN_GAP_MS (20000), PR1_CLOSE_WINDOW_MS (40000) */
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
const { encodeClientFrame, decodeServerFrame, encodePresencePeer, decodePresencePeer } = await import(join(repoRoot, "🧰️framework", "🔨️modules", "📡️replication", "🟦️.ts"));

const origin = process.argv[2] ?? "http://127.0.0.1:7611";
const spaceId = process.argv[3] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const documentId = process.argv[4] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const surfaceId = process.argv[5] ?? "s.gis.gismap@1/*#editor";
const TAG = process.env.PR1_TAG ?? "pr1";
const WINDOW_MS = Number(process.env.PR1_WINDOW_MS ?? 120_000);
const SAMPLE_MS = Number(process.env.PR1_SAMPLE_MS ?? 5_000);
const BEAT_MS = Number(process.env.PR1_BEAT_MS ?? 5_000);
const JOIN_GAP_MS = Number(process.env.PR1_JOIN_GAP_MS ?? 20_000);
const CLOSE_WINDOW_MS = Number(process.env.PR1_CLOSE_WINDOW_MS ?? 40_000);
const WITH_AGENT = process.env.PR1_WITH_AGENT === "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

const t0 = Date.now();
const ms = (): number => Date.now() - t0;
const report: Record<string, any> = { origin, spaceId, documentId, surfaceId, windowMs: WINDOW_MS, sampleMs: SAMPLE_MS, beatMs: BEAT_MS, joinGapMs: JOIN_GAP_MS, startedAt: new Date().toISOString(), samples: [], verdicts: [] };
const save = (): void => writeFileSync(join(OUT, `${TAG}-presence-socket-probe.json`), JSON.stringify(report, null, 2));
const say = (line: string): void => console.log(`${String(ms()).padStart(7)} ${line}`);
const verdict = (name: string, pass: boolean, detail: string): void => {
  report.verdicts.push({ name, pass, at: ms(), detail });
  console.log(`VERDICT ${name}: ${pass ? "PASS" : "FAIL"} — ${detail}`);
  save();
};

type Peer = { actor: string; label?: string; color?: number; userId?: string; principalKind?: string; surface?: string };
type Holder = {
  label: string;
  actor: string;
  socket: WebSocket;
  roster: Peer[];
  presenceFrames: number;
  closed: boolean;
  closeCode: number | null;
  beatSeq: number;
  beat: () => void;
};

/** 🪪️ One human's hub session, minted the way the browser sign-in form mints it. */
async function humanToken(user: (typeof USERS)[number]): Promise<string> {
  const minted = await fetch(`${origin}/auth/sessions`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: user.email, password: user.password, deviceInstanceId: `pr1-${user.label}`, clientClass: "browser" }),
  });
  const body = (await minted.json()) as { token?: string };
  if (minted.status !== 200) throw new Error(`${user.label} sign-in ${minted.status}`);
  return body.token ?? "";
}

/** 🤖️ One DELEGATED agent session, exactly as `semio-os-mcp --hub` mints it: the human creates a
 * delegation in its own space and the agent exchanges that capability for a session whose record
 * carries `session_kind = agent`. The hub stamps `principal_kind` onto every roster row from the
 * session it AUTHENTICATED, so this is the only way an `agent` row can ever appear — M6 §M6b.4
 * step 6, which no roster had ever shown. */
async function agentToken(delegatingToken: string, label: string): Promise<string> {
  const headers = { authorization: `Bearer ${delegatingToken}`, "content-type": "application/json" };
  const created = await fetch(`${origin}/auth/agent-delegations`, {
    method: "POST",
    headers,
    body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience: "edit", ttlSecs: 3600 }),
  });
  const delegation = (await created.json()) as { delegationId?: string; token?: string; agentPrincipalId?: string };
  if (created.status !== 201) throw new Error(`agent delegation ${created.status}`);
  say(`DELEGATION id=${delegation.delegationId} principal=${delegation.agentPrincipalId}`);
  const exchanged = await fetch(`${origin}/auth/agent-sessions`, {
    method: "POST",
    headers: { authorization: `Bearer ${delegation.token ?? ""}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: "edit", agentInstanceId: "pr1.presence.1" }),
  });
  const session = (await exchanged.json()) as { token?: string; agentPrincipalId?: string };
  if (exchanged.status !== 200) throw new Error(`agent session ${exchanged.status}`);
  report.agentPrincipalId = session.agentPrincipalId;
  return session.token ?? "";
}

async function hold(user: (typeof USERS)[number], suppliedToken?: string): Promise<Holder> {
  const token: string = suppliedToken ?? (await humanToken(user));
  const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
  const planResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
    method: "POST",
    headers,
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `pr1-${user.label}` }),
  });
  const planText = await planResponse.text();
  if (planResponse.status !== 200) throw new Error(`${user.label} open-plan ${planResponse.status} ${planText.slice(0, 200)}`);
  const plan = JSON.parse(planText) as { receipt: string; surface?: { surfaceId?: string }; artifact: { schema: string; packSchemaHash: string } };
  const packSchemaHash = [...(plan.artifact.packSchemaHash.match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  const grantResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
    method: "POST",
    headers,
    body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
  });
  const grantText = await grantResponse.text();
  if (grantResponse.status !== 200) throw new Error(`${user.label} socket-grant ${grantResponse.status} ${grantText.slice(0, 200)}`);
  const receipt = JSON.parse(grantText) as { grant: string; protocol: string; actorId: string };
  const wsUrl = `${origin.replace(/^http/u, "ws")}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket/v1?surface=${encodeURIComponent(surfaceId)}`;
  const socket = new WebSocket(wsUrl, [receipt.protocol, receipt.grant]);
  socket.binaryType = "arraybuffer";
  const holder: Holder = { label: user.label, actor: receipt.actorId, socket, roster: [], presenceFrames: 0, closed: false, closeCode: null, beatSeq: 0, beat: () => undefined };
  holder.beat = (): void => {
    if (socket.readyState !== WebSocket.OPEN) return;
    holder.beatSeq += 1;
    // 💓️ The shell's own heartbeat shape: the app-owned `presencePack` is the only field that
    // varies, everything identity-shaped is overwritten by the hub at ingress anyway.
    const peer = { actor: receipt.actorId, connectedAtMs: t0, label: user.label, presencePack: [holder.beatSeq & 0xff], views: [] };
    socket.send(encodeClientFrame({ Presence: { peer: encodePresencePeer(peer as any) } }, "preview"));
  };
  socket.addEventListener("message", (event) => {
    if (!(event.data instanceof ArrayBuffer)) return;
    let frame: any;
    try {
      frame = decodeServerFrame(new Uint8Array(event.data)).frame;
    } catch {
      return;
    }
    if ("Presence" in frame) {
      holder.presenceFrames += 1;
      holder.roster = frame.Presence.peers.flatMap((bytes: number[]) => {
        try {
          const peer = decodePresencePeer(new Uint8Array(bytes), [0]);
          return [{ actor: peer.actor, label: peer.label, color: peer.color, userId: peer.userId, principalKind: peer.principalKind, surface: peer.surface }];
        } catch {
          return [];
        }
      });
    }
  });
  socket.addEventListener("close", (event) => {
    holder.closed = true;
    holder.closeCode = event.code;
    say(`WS-CLOSE ${user.label} code=${event.code} reason=${event.reason}`);
  });
  for (let wait = 0; wait < 200 && socket.readyState === WebSocket.CONNECTING; wait += 1) await Bun.sleep(50);
  if (socket.readyState !== WebSocket.OPEN) throw new Error(`${user.label} socket did not open (state ${socket.readyState})`);
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
  say(`ATTACH ${user.label} actor=${receipt.actorId} surface=${plan.surface?.surfaceId ?? "<none>"}`);
  return holder;
}

const holders: Holder[] = [];
holders.push(await hold(USERS[0]!));
await Bun.sleep(2_000);
holders[0]!.beat();
say(`SETTLE user1 alone for ${JOIN_GAP_MS} ms before user2 joins`);
const settleUntil = Date.now() + JOIN_GAP_MS;
while (Date.now() < settleUntil) {
  await Bun.sleep(Math.max(BEAT_MS, 1_000));
  if (BEAT_MS > 0) holders[0]!.beat();
}
holders.push(await hold(USERS[1]!));
await Bun.sleep(2_000);
holders[1]!.beat();
// 🤖️ A THIRD participant that is not a person: a delegated agent session on the same document.
if (WITH_AGENT) {
  const delegating = await humanToken(USERS[0]!);
  holders.push(await hold({ label: "agent", email: "", password: "" }, await agentToken(delegating, "Drafting agent")));
  await Bun.sleep(2_000);
  holders[2]!.beat();
}

const sample = (phase: string): Record<string, any> => {
  const rows = holders.map((holder) => ({
    user: holder.label,
    closed: holder.closed,
    presenceFrames: holder.presenceFrames,
    peers: holder.roster.map((peer) => peer.actor),
    labels: holder.roster.map((peer) => peer.label ?? null),
    colors: holder.roster.map((peer) => peer.color ?? null),
    kinds: holder.roster.map((peer) => peer.principalKind ?? "human"),
  }));
  const row = { at: ms(), phase, rows };
  report.samples.push(row);
  say(`SAMPLE ${phase} ${rows.map((r) => `${r.user}:${r.closed ? "CLOSED" : `n=${r.peers.length} colors=[${r.colors.join(",")}] kinds=[${r.kinds.join(",")}] frames=${r.presenceFrames}`}`).join(" | ")}`);
  save();
  return row;
};

const windowStart = Date.now();
let nextBeat = Date.now();
while (Date.now() - windowStart < WINDOW_MS) {
  sample("both-attached");
  const until = Date.now() + SAMPLE_MS;
  while (Date.now() < until) {
    await Bun.sleep(250);
    if (BEAT_MS > 0 && Date.now() >= nextBeat) {
      nextBeat = Date.now() + BEAT_MS;
      for (const holder of holders) holder.beat();
    }
  }
}

holders[1]!.socket.close();
const closedAt = ms();
let droppedAt: number | null = null;
const closeUntil = Date.now() + CLOSE_WINDOW_MS;
while (Date.now() < closeUntil) {
  const row = sample("after-close-user2");
  const first = row.rows[0]!;
  if (droppedAt === null && !first.closed && first.peers.length < holders.length) droppedAt = row.at;
  const until = Date.now() + SAMPLE_MS;
  while (Date.now() < until) {
    await Bun.sleep(250);
    if (BEAT_MS > 0 && Date.now() >= nextBeat) {
      nextBeat = Date.now() + BEAT_MS;
      holders[0]!.beat();
    }
  }
}
report.removalLatencyMs = droppedAt === null ? null : droppedAt - closedAt;

const attached = report.samples.filter((row: any) => row.phase === "both-attached");
const expectedPeers = holders.length;
const bothEverywhere = attached.filter((row: any) => row.rows.every((r: any) => r.peers.length === expectedPeers));
verdict("symmetric-roster-for-the-whole-window", attached.length > 0 && bothEverywhere.length === attached.length, `${bothEverywhere.length}/${attached.length} samples list ALL ${expectedPeers} peers in EVERY socket; sizes ${JSON.stringify(attached.map((row: any) => row.rows.map((r: any) => r.peers.length)))}`);
const last = attached[attached.length - 1];
verdict("no-decay-while-the-socket-is-open", last !== undefined && last.rows.every((r: any) => r.peers.length === expectedPeers), `last sample sizes ${JSON.stringify(last?.rows.map((r: any) => r.peers.length) ?? [])}`);
const twoPeer = attached.filter((row: any) => row.rows.some((r: any) => r.peers.length === expectedPeers));
const colourDistinct = twoPeer.every((row: any) => row.rows.every((r: any) => r.peers.length < expectedPeers || new Set(r.colors).size === r.colors.length));
verdict("distinct-colour-per-session", twoPeer.length > 0 && colourDistinct, `${twoPeer.length} full-roster samples; distinct=${colourDistinct}; colours ${JSON.stringify(twoPeer.slice(-1).map((row: any) => row.rows.map((r: any) => r.colors)))}`);
verdict("removal-after-close", report.removalLatencyMs !== null, `user1 roster dropped user2 after ${report.removalLatencyMs ?? `never (within ${CLOSE_WINDOW_MS} ms)`} ms`);
verdict("sockets-never-closed-themselves", holders[0]!.closeCode === null, `user1 closeCode=${holders[0]!.closeCode}`);
if (WITH_AGENT) {
  const withAgent = attached.filter((row: any) => row.rows[0]!.kinds.includes("agent"));
  const humanRows = attached.flatMap((row: any) => row.rows[0]!.kinds.filter((kind: string) => kind === "human"));
  verdict(
    "a-delegated-agent-is-its-own-roster-row",
    withAgent.length > 0 && humanRows.length > 0,
    `${withAgent.length}/${attached.length} samples carry a peer whose admitted principalKind is "agent" (${report.agentPrincipalId}); last roster kinds ${JSON.stringify(attached.slice(-1).map((row: any) => row.rows.map((r: any) => r.kinds)))}`,
  );
}

report.finishedAt = new Date().toISOString();
save();
holders[0]!.socket.close();
say(`DONE samples=${report.samples.length} report=${TAG}-presence-socket-probe.json`);
process.exit(0);
