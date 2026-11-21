/** 🏃 Assembles the round 5 running child: the remodelled bodies of every scale, the thermal model and the skeleton, with the decisions, into `volumes/humans/running-child.json` (reads the catalog asset).
 * Usage: `bun r5-running-child-figure.ts`
 * @see ./r5-running-child-body.ts
 * @see ./r5-running-child-lists.ts
 * @see ./r5-running-child-texts.ts */
import { writeFileSync } from "node:fs";
import { join } from "node:path";

import { body50, bodyCoarse } from "./r5-running-child-body.ts";
import { Kit, type Json } from "./r3-running-child-kit.ts";
import { structure, thermal } from "./r5-running-child-lists.ts";
import { DECISIONS } from "./r5-running-child-texts.ts";

export const SCALES = [50, 100, 200, 500] as const;
export type Built = { volume: Json; kit: Kit };
export const ASSET_FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/running-child/🔣️.json";
export const readAsset = async (): Promise<Json> => JSON.parse(await Bun.file(ASSET_FILE).text()).asset as Json;

/** 🏗️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  if (scale === 50) body50(kit);
  else bodyCoarse(kit, scale);
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale), structure: structure(asset, scale), decisions: DECISIONS[scale]! } };
}

if (import.meta.main) {
  const asset = await readAsset();
  const volumes = Object.fromEntries(SCALES.map((s) => [String(s), build(asset, s).volume]));
  writeFileSync(join(import.meta.dir, "volumes", "humans", "running-child.json"), `${JSON.stringify({ volumes })}\n`);
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
