/** 🕳️ Drawn shapes whose ring bounds miss a box while their pen reaches into it: bun machinery-dropped.ts <asset> <scale> <view> <minX> <maxX> <minY> <maxY> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { boundsOf, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, a, b, c, d] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string];
const [x0, x1, y0, y1] = [a, b, c, d].map(Number) as [number, number, number, number];
const entry = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
for (const s of state.shapes[view] as any[]) {
  const bb = boundsOf(s.rings);
  const inside = bb[1] >= x0 && bb[0] <= x1 && bb[3] >= y0 && bb[2] <= y1;
  if (inside) continue;
  const pen = ((s.shape.penMm ?? 0.5) * Number(scaleArg)) / 1000;
  const near = bb[1] + pen >= x0 && bb[0] - pen <= x1 && bb[3] + pen >= y0 && bb[2] - pen <= y1;
  if (near) console.log(s.shape.id, s.shape.role, "filled", s.shape.filled, "bounds", bb.map((v: number) => +v.toFixed(3)).join(","));
}
console.log("done");
