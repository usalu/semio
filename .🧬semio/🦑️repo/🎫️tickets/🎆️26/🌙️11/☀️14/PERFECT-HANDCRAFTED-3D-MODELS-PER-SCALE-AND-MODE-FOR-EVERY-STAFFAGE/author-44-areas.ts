/** 📐️ Per-solid exposed thermal area of a region at one scale, with the product measures. Usage: bun author-44-areas.ts <category>/<slug> <scale> [region ...] */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { contactPlanes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { exposedArea } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { solidsOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️lists/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";

const [spec, scaleArg, ...only] = process.argv.slice(2);
const { entry, asset: base } = entryOf(spec!);
const asset: any = merged(base, loadSource(entry)!);
const d = Number(scaleArg);
const pose = asset.presentation.defaultPose;
const state: any = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: d } });
const feature = 0.00015 * d;
const W = (list: string): any[] => worldSolids(asset, state.bones, d, list as never) as any[];
const body = W("body"), thermal = W("thermal"), structure = W("structure");
const against = [...body, ...thermal, ...structure];
const planes = contactPlanes(state, feature);
const volume = asset.volumes[String(d)];
let total = 0;
for (const region of asset.thermal.regions) {
  if (only.length && !only.includes(region.id)) continue;
  const own = volume.thermal.filter((s: any) => s.region === region.id);
  const outerLayer = region.layers.length > 1 && d === 50 ? region.layers[region.layers.length - 1].id : undefined;
  const shell = outerLayer === undefined ? own : own.filter((s: any) => s.layer === outerLayer);
  const hidden = region.hidden !== undefined;
  const ownWorlds = new Set(own.map((s: any) => thermal.find((w) => w.id === s.id)));
  const rows = (shell.length ? shell : own).map((s: any) => {
    const w = thermal.find((x) => x.id === s.id);
    if (!w) return [s.id, NaN];
    const a = exposedArea([w], hidden ? [] : against, planes, (o: any) => ownWorlds.has(o), Math.max(1e-4, 0.02 * feature));
    return [s.id, a];
  });
  const sum = rows.reduce((m: number, r: any) => m + r[1], 0);
  total += sum;
  process.stdout.write(`${region.id}: exposed ${sum.toFixed(3)} declared ${region.areaM2} ratio ${(sum / region.areaM2).toFixed(2)}\n`);
  for (const [id, a] of rows.sort((p: any, q: any) => q[1] - p[1])) process.stdout.write(`   ${id} ${Number(a).toFixed(3)}\n`);
}
