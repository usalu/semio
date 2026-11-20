/** 🏗️ Writes the authored volumes of `humans/person-on-crutches` into the source `volumes/humans/person-on-crutches.json` and the candidate asset document `🗑️generated/r3-person-on-crutches/🔣️.json` (catalog asset with the root fixes of `r3-person-on-crutches-roots.ts`, the volumes, and the redrawn 2D geometry when `geometry.json` exists there).
 *
 * Usage: `bun r3-person-on-crutches-build.ts`
 * @see ./r3-person-on-crutches-s50.ts
 * @see ./r3-person-on-crutches-coarse.ts */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { sourcePath, type Json } from "./volumes-lib.ts";
import { ASSET, ENTRY } from "./r3-person-on-crutches-kit.ts";
import { DECISIONS_50, body50, structure50, thermal50 } from "./r3-person-on-crutches-s50.ts";
import { DECISIONS, body100, body200, body500, structure100, structure200, structure500, thermal100, thermal200, thermal500 } from "./r3-person-on-crutches-coarse.ts";

export const OUT = join(import.meta.dir, "🗑️generated", "r3-person-on-crutches");
export const CANDIDATE = join(OUT, "🔣️.json");
/** 📦 The three lists and the decisions of every scale. */
export function volumes(): Json {
  const b50 = body50();
  const b100 = body100();
  const b200 = body200();
  const b500 = body500();
  return {
    "50": { status: "authored", denominator: 50, body: b50, thermal: thermal50(b50), structure: structure50(b50), decisions: DECISIONS_50 },
    "100": { status: "authored", denominator: 100, body: b100, thermal: thermal100(b100), structure: structure100(), decisions: DECISIONS[100] },
    "200": { status: "authored", denominator: 200, body: b200, thermal: thermal200(b200), structure: structure200(), decisions: DECISIONS[200] },
    "500": { status: "authored", denominator: 500, body: b500, thermal: thermal500(b500), structure: structure500(), decisions: DECISIONS[500] },
  };
}
if (import.meta.main) {
  const v = volumes();
  writeFileSync(sourcePath(ENTRY), `${JSON.stringify({ volumes: v })}\n`);
  const asset = JSON.parse(JSON.stringify(ASSET)) as Json;
  asset.volumes = v;
  const drawn = join(OUT, "geometry.json");
  if (existsSync(drawn)) {
    const g = JSON.parse(readFileSync(drawn, "utf8")) as Json;
    for (const scale of Object.keys(g.geometry)) asset.representations[scale].geometry = g.geometry[scale];
    Object.assign(asset.dimensions, g.dimensions);
  }
  mkdirSync(OUT, { recursive: true });
  writeFileSync(CANDIDATE, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }));
  for (const scale of Object.keys(v)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${v[scale][l]?.length}`).join(", "));
}
