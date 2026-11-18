/** 📏 Extent of every solid of a scale including textures, sorted by the largest value of one axis. bun humansb-A-ext.ts <asset> <scale> <axis 0|1|2> [max|min] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, axisArg, mode] = process.argv.slice(2) as [string, string, string, string | undefined];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (process.env.PATCH) {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } }) as any;
const rows = worldSolids(asset, state.bones as never, Number(sc)).map((w) => { const e = extentOf([w]); return { id: w.id, tex: w.texture === true, slender: w.slender === true, min: e.min, max: e.max }; });
const a = Number(axisArg);
rows.sort((x, y) => (mode === "min" ? x.min[a]! - y.min[a]! : y.max[a]! - x.max[a]!));
for (const r of rows.slice(0, 8)) console.log(r.id.padEnd(26), r.tex ? "tex" : "   ", r.slender ? "slender" : "       ", r.min.map((v) => v.toFixed(4)).join(","), "|", r.max.map((v) => v.toFixed(4)).join(","));
