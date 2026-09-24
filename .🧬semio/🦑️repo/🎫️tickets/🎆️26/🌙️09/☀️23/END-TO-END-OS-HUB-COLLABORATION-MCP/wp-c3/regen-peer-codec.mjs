
import { encodePresencePeer, decodePresencePeer } from "/Users/ueli/Documents/semio/\ud83e\uddf0\ufe0fframework/\ud83d\udd28\ufe0fmodules/\ud83d\udce1\ufe0freplication/\ud83d\udfe6\ufe0f.ts";
import { readFileSync, writeFileSync } from "fs";
const fixturePath = "/Users/ueli/Documents/semio/\ud83e\uddf0\ufe0fframework/\ud83d\udd28\ufe0fmodules/\ud83d\udce1\ufe0freplication/\ud83e\uddeb\ufe0ffixtures/\ud83d\udc65\ufe0fpresence-peer-codec-v1/\ud83d\udd23\ufe0f.json";
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));

function toPeer(expected) {
  const peer = {
    actor: expected.actor,
    connectedAtMs: expected.connectedAtMs,
    views: expected.views ?? [],
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

for (const row of fixture.cases) {
  if (!row.accepted || !row.expected) continue;
  if (row.id !== "every-field-present" && row.id !== "active-tool-only") continue;
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
  if (pos[0] !== bytes.length) throw new Error(row.id + " trailing");
  console.log(row.id, hex.slice(0, 40) + "...", "len", bytes.length, "activeTool", round.activeTool);
}
writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
console.log("fixture written");
