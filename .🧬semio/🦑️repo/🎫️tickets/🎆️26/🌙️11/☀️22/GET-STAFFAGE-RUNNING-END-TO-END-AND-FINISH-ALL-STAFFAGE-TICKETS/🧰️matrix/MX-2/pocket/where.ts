/** 🧭️ `where.ts` of batch MU on a chosen copy of the silhouette module: `MODULE=<path> bun where.ts <scale> <view> <id|file.json>…` prints drawn, volume, both and the IoU of `measureAsset` of that module (default the trial copy of the pocket rule), so a candidate rule is measured on the same document as the live one.
 * @see ../../MU/where.ts — the original */
import { join } from "node:path";
import { S, readDoc } from "../../MR/lib.ts";
const { measureAsset } = await import(process.env.MODULE ?? join(import.meta.dir, "silhouette.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const [scale, view, ...files] = process.argv.slice(2) as [string, string, ...string[]];
for (const file of files) {
  const m = measureAsset(readDoc(file).doc.asset, Number(scale), undefined, undefined, featureOf(Number(scale))).views[view];
  console.log(`${file.split("/").pop()} ${process.env.TRUTH ?? "q90"} drawn ${m.counts.drawn} volume ${m.counts.volume} both ${m.counts.both} either ${m.counts.either} recall ${m.recall.toFixed(4)} precision ${m.precision.toFixed(4)} iou ${m.iou.toFixed(4)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm`);
}
