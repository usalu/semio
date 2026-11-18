/** 🔬 The strokes of slender volume solids of tower-crane as the guard draws them: ring bounds per solid id regex.
 *  bun crane-strokes.ts <scale> <view> <idRegex> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, strokesOf, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [scaleArg, view, regex] = process.argv.slice(2) as [string, Viewpoint, string];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === "tower-crane")!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const drawn = drawnFor(asset, state, view, scale);
const re = new RegExp(regex);
for (const s of solids) {
  if (!re.test(s.id) || s.slender !== true) continue;
  const rings = strokesOf(view, s, drawn);
  const out = rings.map((r) => { const xs = r.map((p) => p.x), ys = r.map((p) => p.y); return `[${Math.min(...xs).toFixed(3)},${Math.max(...xs).toFixed(3)}|${Math.min(...ys).toFixed(3)},${Math.max(...ys).toFixed(3)}]`; });
  console.log(s.id, out.slice(0, 6).join(" "));
  if (process.env.POINTS !== undefined) for (const r of rings.slice(0, Number(process.env.POINTS))) console.log("  ", r.map((p) => `(${p.x.toFixed(4)},${p.y.toFixed(4)})`).join(" "));
}
