/** 📏 Extents of the volume solids of the date-palm fronds in the bone frame: bun datepalm-ext.ts <scale> [bonePrefix] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [scaleArg, prefix = "fr-"] = process.argv.slice(2);
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === "date-palm")!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
for (const s of worldSolids(asset, state.bones as never, scale)) {
  if (!(s.bone ?? "").startsWith(prefix)) continue;
  const f = state.bones[s.bone!].frame;
  const o = f.o;
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let i = 0; i < s.positions.length; i += 3) {
    const d = [s.positions[i]! - o.x, s.positions[i + 1]! - o.y, s.positions[i + 2]! - o.z];
    const c = [f.u, f.v, f.w].map((a: any) => d[0]! * a.x + d[1]! * a.y + d[2]! * a.z);
    for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, c[k]!); hi[k] = Math.max(hi[k]!, c[k]!); }
  }
  console.log(s.id, "u", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "v", lo[1]!.toFixed(3), hi[1]!.toFixed(3), "w", lo[2]!.toFixed(3), hi[2]!.toFixed(3), "frame u", JSON.stringify(f.u, (k, x) => typeof x === "number" ? +x.toFixed(2) : x));
}
