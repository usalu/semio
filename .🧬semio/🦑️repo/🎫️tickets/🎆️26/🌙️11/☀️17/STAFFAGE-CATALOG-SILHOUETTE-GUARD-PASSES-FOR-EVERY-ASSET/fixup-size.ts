/** 🧪 Groups, rings and points of every raster layer of one view, with the characters of the sharp svg path of each.
 *  bun fixup-size.ts <asset folder> <scale> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [folder, scale] = process.argv.slice(2) as [string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const m = measureAsset(asset, Number(scale), undefined, undefined, featureOf(Number(scale)));
for (const view of VIEWPOINTS) {
  const v = m.views[view];
  for (const layer of ["filled", "outlines", "reference", "visible", "full"] as const) {
    const g = v[layer] as readonly (readonly (readonly { x: number; y: number }[])[])[];
    let rings = 0, points = 0, chars = 0;
    for (const grp of g) for (const r of grp) { rings++; points += r.length; for (const p of r) chars += ((p.x - v.box.minX) / v.cell).toFixed(3).length + ((p.y - v.box.minY) / v.cell).toFixed(3).length + 2; }
    console.log(view, layer, "groups", g.length, "rings", rings, "points", points, "chars", chars);
  }
}
