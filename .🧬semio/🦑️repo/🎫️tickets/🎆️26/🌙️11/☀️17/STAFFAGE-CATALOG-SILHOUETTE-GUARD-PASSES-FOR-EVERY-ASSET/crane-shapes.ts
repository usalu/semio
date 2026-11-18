/** 🔬 Evaluated drawn shapes of tower-crane (rings, closed, pen) per id regex.
 *  bun crane-shapes.ts <scale> <view> <idRegex> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [scaleArg, view, regex] = process.argv.slice(2) as [string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === "tower-crane")!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const re = new RegExp(regex);
for (const s of state.shapes[view] as any[]) {
  if (!re.test(s.shape.id)) continue;
  if (process.env.BOX !== undefined) {
    const [x0, x1, y0, y1] = process.env.BOX.split(",").map(Number) as [number, number, number, number];
    const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y);
    const [a, b, c, e] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
    if (b < x0 || a > x1 || e < y0 || c > y1) continue;
    console.log(s.shape.id, s.shape.kind, s.shape.bone, `pen ${s.shape.penMm} filled ${s.shape.filled} closed ${s.closed} [${a.toFixed(3)},${b.toFixed(3)}|${c.toFixed(3)},${e.toFixed(3)}]`);
    continue;
  }
  console.log(s.shape.id, s.closed, s.shape.penMm, s.shape.filled, s.rings.map((r: any[]) => r.map((p) => `(${p.x.toFixed(4)},${p.y.toFixed(4)})`).join(" ")).join(" | "));
}
