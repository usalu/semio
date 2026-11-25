/** 🧭️ Where a silhouette comparison loses its overlap: `bun where.ts <scale> <view> <id|file.json>…` prints per document the cells drawn, of the volume, in both and in either, with recall, precision, IoU and the largest boundary distance of `measureAsset` (run it from `S/📦️packages/🟦️typescript`). */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const [scale, view, ...files] = process.argv.slice(2) as [string, string, ...string[]];
for (const file of files) {
  const m = measureAsset(readDoc(file).doc.asset, Number(scale), undefined, undefined, featureOf(Number(scale))).views[view];
  console.log(`${file.split("/").pop()} drawn ${m.counts.drawn} volume ${m.counts.volume} both ${m.counts.both} either ${m.counts.either} recall ${m.recall.toFixed(4)} precision ${m.precision.toFixed(4)} iou ${m.iou.toFixed(4)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm open strokes ${m.openStrokes}`);
}
