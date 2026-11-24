/** 🧱️ Which solids make the sharp reference of one view slow: per body solid of the scale its triangles, the projected polygons with their summed bounding-box area in cells (what a scanline rasteriser pays) and the seconds librsvg takes for that solid alone on the grid of the guard; printed as each ends. SKIP names solids to leave out (comma list of id prefixes), MAXCELLS skips the raster of a solid above that summed area, TOGETHER=1 rasterises the solids that are left in one path, as the guard does.
 *  bun oracle-solids.ts <slug|file.json> <scale> <view> */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { boundsOf, measureView, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { raster } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, view] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature);
const [width, height] = [Math.ceil((m.box.maxX - m.box.minX) / m.cell), Math.ceil((m.box.maxY - m.box.minY) / m.cell)];
console.log(`${asset.id} 1:${scale} ${view}: grid ${width} x ${height}, reference ${m.reference.length} polygons`);
const prefixes = (process.env.SKIP ?? "").split(",").filter((p) => p !== "");
const skip = { has: (id: string): boolean => prefixes.some((p) => id.startsWith(p)) };
if (process.env.TOGETHER) {
  const kept = solids.filter((w) => !skip.has(w.id));
  const groups = referenceOf(view as never, kept, undefined).cut;
  const t = performance.now();
  await raster(groups, m.box, m.cell, width, height);
  console.log(`  together ${kept.length} solids, ${groups.length} polygons: sharp ${((performance.now() - t) / 1000).toFixed(1)} s`);
  process.exit(0);
}
const rows: { id: string; triangles: number; polygons: number; cells: number; seconds: number }[] = [];
for (const solid of solids) {
  if (skip.has(solid.id)) continue;
  const groups = referenceOf(view as never, [solid], undefined).cut;
  let cells = 0;
  for (const g of groups) { const b = boundsOf(g); cells += ((b[1] - b[0]) / m.cell + 1) * ((b[3] - b[2]) / m.cell + 1); }
  let seconds = NaN;
  if (cells <= Number(process.env.MAXCELLS ?? Infinity)) { const t = performance.now(); await raster(groups, m.box, m.cell, width, height); seconds = (performance.now() - t) / 1000; }
  rows.push({ id: solid.id, triangles: solid.indices.length / 3, polygons: groups.length, cells, seconds });
  console.log(`  ${solid.id}: ${solid.indices.length / 3} triangles, ${groups.length} polygons, ${(cells / 1e6).toFixed(1)} M box cells, sharp ${seconds.toFixed(1)} s`);
}
console.log(`  total ${rows.reduce((n, r) => n + r.cells, 0) / 1e6} M box cells, sharp ${rows.reduce((n, r) => n + (r.seconds || 0), 0).toFixed(1)} s`);
