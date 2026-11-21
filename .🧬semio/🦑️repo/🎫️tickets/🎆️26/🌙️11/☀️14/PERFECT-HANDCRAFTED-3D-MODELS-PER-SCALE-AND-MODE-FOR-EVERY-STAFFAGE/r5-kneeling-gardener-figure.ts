/** 🌻️ Assembles the round 5 kneeling gardener: body, thermal model and skeleton of 1:50 and 1:100 from the hand-stated lists and the round 5 figure masses of 1:200 and 1:500 (`r5-kneeling-gardener-coarse.ts`), written to `volumes/humans/kneeling-gardener.json` and into the candidate asset document `🗑️generated/r3-kneeling-gardener/🔣️.json` (catalog asset with the root fixes of `r3-kneeling-gardener-roots.py --candidate`).
 * Usage: `bun r5-kneeling-gardener-figure.ts`
 * @see ./r5-kneeling-gardener-body.ts
 * @see ./r5-kneeling-gardener-lists.ts
 * @see ./r5-kneeling-gardener-coarse.ts
 * @see ./r3-humans-kneeling-gardener.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, type Json } from "./r3-kneeling-gardener-kit.ts";
import { body50 } from "./r5-kneeling-gardener-body.ts";
import { DECISIONS, body100, structure, thermal } from "./r5-kneeling-gardener-lists.ts";
import { coarseBody } from "./r5-kneeling-gardener-coarse.ts";

export const SCALES = [50, 100] as const;
export const CANDIDATE = join(import.meta.dir, "🗑️generated", "r3-kneeling-gardener", "🔣️.json");
export type Built = { volume: Json; kit: Kit };

/** 🏗️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  ({ 50: body50, 100: body100 })[scale](kit);
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale), structure: structure(asset, scale), decisions: DECISIONS[scale]! } };
}

if (import.meta.main) {
  const document = JSON.parse(readFileSync(CANDIDATE, "utf8")) as Json;
  const asset = document.asset as Json;
  const volumes: Json = { ...asset.volumes };
  for (const s of SCALES) volumes[String(s)] = build(asset, s).volume;
  for (const s of [200, 500] as const) volumes[String(s)] = { ...volumes[String(s)], body: coarseBody(volumes[String(s)], s) };
  writeFileSync(join(import.meta.dir, "volumes", "humans", "kneeling-gardener.json"), `${JSON.stringify({ volumes })}\n`);
  asset.volumes = volumes;
  writeFileSync(CANDIDATE, JSON.stringify(document));
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
