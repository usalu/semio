/** 🧾️ W0-F probe: replays every hub presence-normalization vector through the TypeScript replication codec. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { decodePresencePeer, encodePresencePeer, type ArtifactPresencePeer } from "../../../../../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";

const repo = join(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(join(repo, "🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json"), "utf8"));
let failures = 0;
for (const vector of fixture.vectors) {
  let normalized: string | null = null;
  try {
    const raw = Buffer.from(vector.rawPeerHex, "hex");
    const input = decodePresencePeer(raw, [0]);
    if (Buffer.from(encodePresencePeer(input)).toString("hex") !== vector.rawPeerHex) throw new Error("raw not canonical");
    const a = vector.admission;
    const output: ArtifactPresencePeer = { actor: a.actor, connectedAtMs: a.connectedAtMs, label: a.label ?? undefined, userId: a.userId ?? undefined, role: a.role ?? undefined, color: a.color, surface: a.surface ?? undefined, presencePack: input.presencePack, dragGhostJson: input.dragGhostJson, interaction: input.interaction, views: input.views, ui: input.ui, toolRun: input.toolRun };
    const bytes = Uint8Array.from(encodePresencePeer(output));
    if (bytes.length > fixture.maximumEntryBytes) throw new Error("entry over limit");
    decodePresencePeer(bytes, [0]);
    normalized = Buffer.from(bytes).toString("hex");
  } catch {
    normalized = null;
  }
  const ok = (normalized !== null) === vector.expected.accepted && normalized === vector.expected.normalizedPeerHex;
  if (!ok) failures++;
  console.log(`${ok ? "ok  " : "FAIL"} ${vector.name}`);
}
console.log(`hub presence normalization vectors: ${fixture.vectors.length} total, ${failures} failed`);
process.exit(failures === 0 ? 0 : 1);
