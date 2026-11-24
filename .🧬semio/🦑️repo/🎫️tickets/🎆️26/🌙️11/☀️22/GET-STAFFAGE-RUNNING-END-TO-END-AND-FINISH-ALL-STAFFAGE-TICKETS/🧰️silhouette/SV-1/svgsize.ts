/** 🧮️ The size of the SVG text the sharp oracle of `🧊️catalog-silhouette` builds per scale and view (the path data of the filled shapes, the outline strokes and the reference triangles, formatted as the oracle formats them), against the 10 000 000 characters librsvg accepts in one text node.
 *  bun svgsize.ts <id|file.json>… */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./plants.ts";

const size = (groups: any[], box: any, cell: number): number => {
  let n = 0;
  for (const g of groups) for (const ring of g) { n += 2; for (const p of ring) n += ((p.x - box.minX) / cell).toFixed(3).length + ((p.y - box.minY) / cell).toFixed(3).length + 2; }
  return n;
};
for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  for (const scale of [500, 200, 100, 50]) {
    if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)]?.status !== "authored") continue;
    const m: any = measureAsset(asset, scale, undefined, undefined, featureOf(scale));
    console.log(`${asset.id} 1:${scale}: ${VIEWPOINTS.map((v: string) => { const x = m.views[v]; const [f, o, r] = [size(x.filled, x.box, x.cell), size(x.outlines, x.box, x.cell), size(x.reference, x.box, x.cell)]; return `${v} filled ${(f / 1e6).toFixed(2)} M, outlines ${(o / 1e6).toFixed(2)} M, reference ${(r / 1e6).toFixed(2)} M (${x.reference.length} triangles)${Math.max(f, o, r) > 1e7 ? " OVER" : ""}`; }).join("; ")}`);
  }
}
