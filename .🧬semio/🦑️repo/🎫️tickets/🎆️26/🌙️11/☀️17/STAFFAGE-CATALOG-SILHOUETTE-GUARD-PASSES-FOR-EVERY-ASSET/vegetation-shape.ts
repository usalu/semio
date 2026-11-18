/** 🔎 The evaluated bounding box of drawn shapes in world metres: bun vegetation-shape.ts <asset> <scale> <view> <id>... */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, scale, view, ...ids] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
for (const s of state.shapes[view] as any[]) {
  if (!ids.includes(s.shape.id)) continue;
  const p = s.rings.flat();
  console.log(s.shape.id, s.shape.bone, s.shape.kind, "closed", s.closed, "x", Math.min(...p.map((q: any) => q.x)).toFixed(3), Math.max(...p.map((q: any) => q.x)).toFixed(3), "y", Math.min(...p.map((q: any) => q.y)).toFixed(3), Math.max(...p.map((q: any) => q.y)).toFixed(3), "pts", p.length);
}
