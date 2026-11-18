/** 🦴 Bone frames of one asset at the default pose: origin and axes u, v, w in the world.
 *  bun machinery-bones.ts <asset> [filter] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, q] = process.argv.slice(2) as [string, string | undefined];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } }) as any;
const f = (v: any) => `(${(+v.x).toFixed(4)},${(+v.y).toFixed(4)},${(+v.z).toFixed(4)})`;
for (const [id, b] of Object.entries(state.bones as Record<string, any>)) {
  if (q !== undefined && !id.includes(q)) continue;
  console.log(id.padEnd(16), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "w", f(b.frame.w));
}
