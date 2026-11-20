/** 📏️ Outer area per thermal region of `machinery/tower-crane` at one scale, exactly as `volumes-thermal-coverage` takes it (C29: the region's own outermost-layer solids against each other, envelopes and emitters left out), without the slow exposure part; prints region, measured, declared and the ratio.
 *
 * Usage: `bun r3-tower-crane-areas.ts <asset document json> <scale>`
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { SEAM_FEATURES, exposedArea } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const d = Number(process.argv[3] ?? 50);
const feature = featureOf(d);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } } as never);
const worlds = worldSolids(asset, state.bones as never, d, "thermal");
const byId = new Map(worlds.map((w) => [w.id, w] as const));
const volume = asset.volumes[String(d)];
let [sum, dec] = [0, 0];
for (const region of asset.thermal.regions) {
  const own = volume.thermal.filter((s: { region: string }) => s.region === region.id);
  const outer = region.layers.length > 1 && d === 50 ? region.layers[region.layers.length - 1].id : undefined;
  const shell = outer === undefined ? own : own.filter((s: { layer?: string }) => s.layer === outer);
  const picked = (shell.length > 0 ? shell : own).map((s: { id: string }) => byId.get(s.id)).filter((w: any) => w !== undefined && w.envelope !== true && w.emitter !== true);
  if (picked.length === 0) {
    process.stdout.write(`${region.id.padEnd(14)} (envelope or none)\n`);
    continue;
  }
  const area = exposedArea(picked, region.hidden !== undefined ? [] : picked, [], () => false, Math.max(1e-4, 0.02 * feature), SEAM_FEATURES * feature);
  sum += area;
  dec += region.areaM2;
  process.stdout.write(`${region.id.padEnd(14)} ${area.toFixed(2).padStart(8)} ${String(region.areaM2).padStart(8)} ${(area / region.areaM2).toFixed(3)}\n`);
}
process.stdout.write(`total ${sum.toFixed(1)} / ${dec.toFixed(1)} = ${(sum / dec).toFixed(3)}\n`);
