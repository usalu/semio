/** 🎲 Random search over small translations and inflations of poly shapes of one view that minimise the Hausdorff distance of that view (IoU kept above the gate). Writes ops (shape replacements) appended to the start ops.
 *  bun humansb-D-jitter.ts <asset> <scale> <view> <start-ops.json> <out.json> <shapeId,...> [iterations] [maxShiftFeatures] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, startFile, outFile, idsArg, itArg, shiftArg] = process.argv.slice(2) as string[];
const scale = Number(sc), feature = featureOf(scale);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const geo = (): any[] => (asset as any).representations[sc].geometry[view];
const ops: any[] = [];
if (startFile !== "-") for (const o of JSON.parse(await Bun.file(startFile).text())) if (o.scale === sc) {
  const l = (asset as any).representations[o.scale].geometry[o.view];
  if (o.op === "shape-add") l.splice(l.findIndex((x: any) => x.id === o.after) + 1, 0, o.shape); else if (o.op === "shape") l[l.findIndex((x: any) => x.id === o.id)] = o.shape; else if (o.op === "shape-set") Object.assign(l.find((x: any) => x.id === o.id), o.set);
  ops.push(o);
}
const ids = idsArg!.split(",");
const shapes = ids.map((id) => geo().find((s: any) => s.id === id)!);
const orig = shapes.map((s) => (s.points as number[][]).map((p) => [...p]));
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = asset.presentation.defaultPose;
const score = (): number => {
  const state: any = evaluate(asset, { kind: "pose", pose }, sel);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view as any, scale, undefined, feature, Infinity, false);
  return m.hausdorffM / (2 * feature) + (m.iou < 0.93 ? (0.93 - m.iou) * 10 : 0);
};
let best = score();
let cfg = ids.map(() => [0, 0]);
console.error(`[DEBUG] start score ${best.toFixed(3)}`);
const maxShift = Number(shiftArg ?? 0.5) * feature;
for (let it = 0; it < Number(itArg ?? 200) && best > 0.9; it++) {
  const trial = cfg.map(([dx, dy], k) => (Math.random() < 0.5 ? [dx + (Math.random() - 0.5) * maxShift * 0.5, dy + (Math.random() - 0.5) * maxShift * 0.5] : [dx, dy]));
  shapes.forEach((s, k) => { s.points = orig[k]!.map(([x, y]) => [+(x! + trial[k]![0]!).toFixed(4), +(y! + trial[k]![1]!).toFixed(4)]); });
  const sc2 = score();
  if (sc2 < best) { best = sc2; cfg = trial; console.error(`[DEBUG] it ${it} score ${best.toFixed(3)} cfg ${JSON.stringify(cfg.map((c) => c.map((v) => +(v * 1000).toFixed(2))))}`); }
}
shapes.forEach((s, k) => { s.points = orig[k]!.map(([x, y]) => [+(x! + cfg[k]![0]!).toFixed(4), +(y! + cfg[k]![1]!).toFixed(4)]); ops.push({ op: "shape", scale: sc, view, id: ids[k], shape: structuredClone(s) }); });
await Bun.write(outFile!, JSON.stringify(ops));
console.error(`[DEBUG] final score ${best.toFixed(3)}`);
