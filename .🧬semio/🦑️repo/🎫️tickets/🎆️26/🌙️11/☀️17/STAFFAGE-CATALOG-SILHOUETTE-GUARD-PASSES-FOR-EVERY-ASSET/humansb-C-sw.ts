/** 📐 Evaluated drawn shapes (world extents) and solid extents after patches. bun humansb-C-sw.ts <asset> <scale> <view> <patches|-> <filter> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, sc, view, patch, q] = process.argv.slice(2);
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
await applyPatch(asset, patch);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
for (const s of state.shapes[view]) {
  if (q && !String(s.shape.id).includes(q)) continue;
  const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y);
  console.log("D", String(s.shape.id).padEnd(24), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
}
for (const w of worldSolids(asset, state.bones as never, scale)) {
  if (q && !w.id.includes(q)) continue;
  const xs: number[] = [], ys: number[] = [];
  for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as Viewpoint, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); xs.push(p.x); ys.push(p.y); }
  console.log("V", w.id.padEnd(24), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
}
