/** ⏱️ Times the flood from outside (C31) on the thermal list of one snapshot asset and scale: triangles, build of the flood, and the outer area of every region with and without it.
 *
 * Usage: `bun r4-area-profile.ts <category>.<slug> <scale>` (reads `🗑️generated/r4-area/snapshot/`, written by `r4-area-census.ts --snapshot save`)
 * @see ./r4-area-flood.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED } from "./volumes-lib.ts";

const S = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { exposedArea, outsideOf } = await import(`${S}/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts`);
const { worldSolids } = await import(`${S}/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const [name, scaleText] = process.argv.slice(2);
const scale = Number(scaleText);
const asset = JSON.parse(readFileSync(join(GENERATED, "r4-area", "snapshot", `${name}.json`), "utf8"));
const feature = (0.15 * scale) / 1000;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const worlds = worldSolids(asset, state.bones, scale, "thermal").filter((w: any) => w.envelope !== true && w.emitter !== true);
const thermal = asset.volumes[String(scale)].thermal as { id: string; region: string }[];
const triangles = worlds.reduce((n: number, w: any) => n + w.indices.length / 3, 0);
const probe = Math.max(1e-4, 0.02 * feature);
let t = performance.now();
const outside = outsideOf(worlds, feature, probe);
process.stdout.write(`${name} 1:${scale}: ${worlds.length} solids, ${triangles} triangles, flood ${(performance.now() - t).toFixed(0)} ms, cell ${outside.cellM}\n`);
for (const region of asset.thermal.regions) {
  const ids = new Set(thermal.filter((s) => s.region === region.id).map((s) => s.id));
  const own = worlds.filter((w: any) => ids.has(w.id));
  if (own.length === 0) continue;
  t = performance.now();
  const plain = exposedArea(own, own, [], () => false, probe, 0.2 * feature);
  const t1 = performance.now();
  const flooded = exposedArea(own, own, [], () => false, probe, 0.2 * feature, (b: never, n: never) => outside.counts(b, n, (w: unknown) => own.includes(w)));
  process.stdout.write(`  ${region.id}: unflooded ${plain.toFixed(4)} (${(t1 - t).toFixed(0)} ms) flooded ${flooded.toFixed(4)} (${(performance.now() - t1).toFixed(0)} ms) declared ${region.areaM2}\n`);
}
