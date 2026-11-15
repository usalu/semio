import { readFileSync } from "node:fs";
const { measureAsset } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next/🌓️silhouette/🟦️.ts`);
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
for (const cells of [300, 450, 600, 900, 1200]) {
  const m = measureAsset(a, 200, undefined, cells, 0.03);
  console.log(cells, Object.entries(m.views).map(([v, x]) => `${v} p95 ${(x.p95M * 1000).toFixed(1)} hd ${(x.hausdorffM * 1000).toFixed(0)} iou ${x.iou.toFixed(3)}`).join(" | "));
}
