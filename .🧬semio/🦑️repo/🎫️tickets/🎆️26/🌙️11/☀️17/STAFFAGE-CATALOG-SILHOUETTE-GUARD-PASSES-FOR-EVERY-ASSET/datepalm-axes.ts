/** 📐 The solids that set the bounding box of the date palm per scale and axis: bun datepalm-axes.ts */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const entry = (await assetPathsFromFiles()).find((e) => e.folder === "date-palm")!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
for (const scale of [500, 200, 100, 50]) {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
  const lo: [number, string][] = [[1e9, ""], [1e9, ""], [1e9, ""]], hi: [number, string][] = [[-1e9, ""], [-1e9, ""], [-1e9, ""]];
  for (const s of worldSolids(asset, state.bones as never, scale)) {
    if (s.texture === true) continue;
    for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) {
      const v = s.positions[i + k]!;
      if (v < lo[k]![0]) lo[k] = [v, s.id];
      if (v > hi[k]![0]) hi[k] = [v, s.id];
    }
  }
  console.log(scale, [0, 1, 2].map((k) => `${"xyz"[k]} ${lo[k]![0].toFixed(4)}(${lo[k]![1]}) ${hi[k]![0].toFixed(4)}(${hi[k]![1]}) size ${(hi[k]![0] - lo[k]![0]).toFixed(4)}`).join(" | "));
}
console.log("dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
