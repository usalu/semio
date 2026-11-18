/** 📦 World extent of every volume solid of one asset scale as JSON: bun vehicles-ext.ts <asset> <scale> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg] = process.argv.slice(2) as [string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } });
const out: Record<string, [number[], number[]]> = {};
for (const s of worldSolids(asset, state.bones as never, Number(scaleArg)).filter((w) => w.texture !== true)) {
  const e = extentOf([s]);
  out[s.id] = [e.min.map((x) => +x.toFixed(4)), e.max.map((x) => +x.toFixed(4))];
}
console.log(JSON.stringify(out));
