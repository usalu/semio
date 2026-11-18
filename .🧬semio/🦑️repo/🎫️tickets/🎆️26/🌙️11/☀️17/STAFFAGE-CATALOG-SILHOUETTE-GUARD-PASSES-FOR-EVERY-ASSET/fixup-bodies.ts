/** 🧪 Triangles and connected pieces of every solid of an asset at a scale.
 *  bun fixup-bodies.ts <asset folder> <scale> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { pieces, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [folder, scale] = process.argv.slice(2) as [string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
for (const w of worldSolids(asset, state.bones as never, Number(scale))) {
  const p = pieces(w).length;
  const t = w.indices.length / 3;
  console.log(w.id.padEnd(22), "triangles", String(t).padStart(8), "pieces", String(p).padStart(6), "product", t * p);
}
