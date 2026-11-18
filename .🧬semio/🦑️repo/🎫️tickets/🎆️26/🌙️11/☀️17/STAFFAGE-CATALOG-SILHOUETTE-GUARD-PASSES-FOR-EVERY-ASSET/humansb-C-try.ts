/** 🧪 Guard line of an asset with in-memory patches (comma-separated). bun humansb-C-try.ts <asset> <patches|-> [scale ...] ; prints also the 3 worst clusters per failing view. */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { GATE, VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, patch, ...sc] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
await applyPatch(asset, patch);
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
for (const scale of sc.length ? sc.map(Number) : [500, 200, 100, 50]) {
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale as never, undefined, undefined, feature);
  const out: string[] = [];
  m.bbox.forEach((b, k) => { if (Math.abs(b - dims[k]!) > feature) out.push(`bbox${k} ${b.toFixed(4)} vs ${dims[k]}`); });
  for (const v of VIEWPOINTS) { const x = m.views[v]; out.push(`${v[0]}:${x.iou.toFixed(3)}/${(x.hausdorffM * 1000).toFixed(1)}mm${x.iou < GATE.iou || x.hausdorffM > 2 * feature ? "!" : ""}`); }
  console.log(scale, `limit ${(2 * feature * 1000).toFixed(0)}mm`, out.join("  "));
}
