/** 📦️ Volume extents against dimensions per scale: bun vegetation-bbox.ts <asset> ... (width x height x depth, difference in features) */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
for (const name of process.argv.slice(2)) {
  const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
  const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  if (!process.env.JSON) console.log(name, "dims", dims.join(" x "));
  for (const scale of [50, 100, 200, 500]) {
    if ((asset.volumes as any)[String(scale)]?.status !== "authored") continue;
    const f = featureOf(scale);
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
    const e = extentOf(worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true));
    const got = [e.max[0]! - e.min[0]!, e.max[1]! - e.min[1]!, e.max[2]! - e.min[2]!];
    if (process.env.JSON) console.log(JSON.stringify({ asset: name, dims, scale, f, ext: got, min: [...e.min], max: [...e.max] }));
    else console.log("  ", scale, got.map((g, k) => `${g.toFixed(4)} (${((g - dims[k]!) / f).toFixed(2)}f${Math.abs(g - dims[k]!) > f ? " FAIL" : ""})`).join("  "), "min", e.min.map((v) => v.toFixed(3)).join(","), "max", e.max.map((v) => v.toFixed(3)).join(","));
  }
}
