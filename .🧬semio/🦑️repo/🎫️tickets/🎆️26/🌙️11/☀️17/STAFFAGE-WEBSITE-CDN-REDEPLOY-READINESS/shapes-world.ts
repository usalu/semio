/** 📐 Evaluated drawn shapes of one asset, scale and view in world coordinates: bun shapes-world.ts <asset> <scale> <view> [filter] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, scaleArg, view, q] = process.argv.slice(2) as [string, string, string, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const rep = state.representation ?? state;
console.log(Object.keys(state));
const shapes = state.views?.[view] ?? state.geometry?.[view] ?? state.shapes?.[view];
console.log(shapes === undefined ? "no views: " + Object.keys(state).join() : shapes.length);
for (const s of shapes ?? []) {
  if (q !== undefined && !String(s.shape.id).includes(q)) continue;
  const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y);
  console.log(String(s.shape.id).padEnd(26), s.shape.role, "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
}
