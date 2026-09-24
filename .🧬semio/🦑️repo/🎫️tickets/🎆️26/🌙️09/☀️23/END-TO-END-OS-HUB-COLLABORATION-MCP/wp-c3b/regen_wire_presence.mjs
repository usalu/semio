/** Regenerate client/server presence wire bins + presence-peer-codec hex after rayOrigin. */
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const replicationEntry = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication.ts";
const {
  encodePresencePeer,
  decodePresencePeer,
  encodeClientFrame,
  encodeServerFrame,
  decodeClientFrame,
  decodeServerFrame,
} = await import(pathToFileURL(replicationEntry).href);

function toPeer(expected) {
  const peer = {
    actor: expected.actor,
    connectedAtMs: expected.connectedAtMs,
    views: (expected.views ?? []).map((view) => {
      const next = {
        windowId: view.windowId,
        space: view.space,
        kind: view.kind,
        size: view.size,
      };
      if (view.pointer !== undefined) next.pointer = view.pointer;
      if (view.rayOrigin !== undefined) next.rayOrigin = view.rayOrigin;
      return next;
    }),
  };
  if (expected.label !== undefined) peer.label = expected.label;
  if (expected.presencePack !== undefined) peer.presencePack = Array.from(Buffer.from(expected.presencePack, "base64"));
  if (expected.userId !== undefined) peer.userId = expected.userId;
  if (expected.role !== undefined) peer.role = expected.role;
  if (expected.dragGhostJson !== undefined) peer.dragGhostJson = expected.dragGhostJson;
  if (expected.interaction !== undefined) {
    peer.interaction = {
      app_id: expected.interaction.appId,
      domains: expected.interaction.domains,
    };
  }
  if (expected.color !== undefined) peer.color = expected.color;
  if (expected.surface !== undefined) peer.surface = expected.surface;
  if (expected.ui !== undefined) peer.ui = expected.ui;
  if (expected.toolRun !== undefined) peer.toolRun = expected.toolRun;
  if (expected.principalKind !== undefined) peer.principalKind = expected.principalKind;
  if (expected.activeTool !== undefined) peer.activeTool = expected.activeTool;
  return peer;
}

const codecPath = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/presence-peer-codec.json";
const fixture = JSON.parse(readFileSync(codecPath, "utf8"));
for (const row of fixture.cases) {
  if (!row.accepted || !row.expected) continue;
  const peer = toPeer(row.expected);
  const bytes = encodePresencePeer(peer);
  const hex = Buffer.from(bytes).toString("hex");
  row.prefixHex = hex;
  row.canonicalHex = hex;
  row.repeatHex = "00";
  row.repeatCount = 0;
  row.suffixHex = "";
  const pos = [0];
  const round = decodePresencePeer(Uint8Array.from(bytes), pos);
  if (pos[0] !== bytes.length) throw new Error(`${row.id} trailing`);
  console.log("codec", row.id, "len", bytes.length, "activeTool", round.activeTool);
}
writeFileSync(codecPath, JSON.stringify(fixture, null, 2) + "\n");

const samplePeer = {
  actor: "actor-1",
  connectedAtMs: 1_700_000_000_000,
  label: "Ada",
  userId: "user-9",
  role: "owner",
  interaction: {
    app_id: "space",
    domains: [
      { domain: "outline", granularity: "task", selected: ["t1", "t2"], hovered: [] },
      { domain: "board", granularity: "card", selected: [], hovered: ["c1"] },
      { domain: "canvas", granularity: "node", selected: ["n9"], hovered: ["n9", "n10"] },
    ],
  },
  color: 5,
  surface: "s.space.home@1/*#editor",
  views: [
    {
      windowId: "w1",
      space: "world",
      kind: { kind: "orbit", position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 },
      size: [1024.0, 768.0],
      pointer: [0.5, 0.5, 0.5],
      rayOrigin: [1.0, 2.0, 3.0],
    },
    {
      windowId: "w2",
      space: "canvas",
      kind: { kind: "canvas", x: 12.5, y: -4.0, zoom: 1.0 },
      size: [800.0, 600.0],
    },
  ],
  ui: { hoveredPath: "row[2]#t1" },
};

const peerBytes = encodePresencePeer(samplePeer);
{
  const pos = [0];
  decodePresencePeer(Uint8Array.from(peerBytes), pos);
  if (pos[0] !== peerBytes.length) throw new Error("sample peer trailing");
  console.log("sample peer ok", peerBytes.length);
}

const fixturesRoot = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication-fixtures";
const wireName = readdirSync(fixturesRoot).find((n) => n.includes("wire"));
const wireRoot = join(fixturesRoot, wireName);
const clientName = readdirSync(wireRoot).find((n) => n.includes("client-presence"));
const serverName = readdirSync(wireRoot).find((n) => n.includes("server-presence"));
function binPath(dir) {
  const name = readdirSync(dir).find((n) => n.includes("bin") || n.endsWith(".bin")) ?? readdirSync(dir)[0];
  return join(dir, name);
}
const clientPath = binPath(join(wireRoot, clientName));
const serverPath = binPath(join(wireRoot, serverName));

const clientBytes = encodeClientFrame({ Presence: { peer: peerBytes } }, "preview");
writeFileSync(clientPath, clientBytes);
console.log("wrote", clientPath, clientBytes.length, decodeClientFrame(clientBytes).lane);

const serverBytes = encodeServerFrame(
  { Presence: { peers: [new TextEncoder().encode('{"id":"a"}'), peerBytes] } },
  "preview",
);
writeFileSync(serverPath, serverBytes);
console.log("wrote", serverPath, serverBytes.length, decodeServerFrame(serverBytes).lane);
console.log("done");
