/** 🌓️ The catalog-silhouette guard of `🧪️tests/🧊️catalog-silhouette` for the given assets: bbox within one feature, IoU ≥ 0.92 and Hausdorff ≤ 2 features per view and scale in the default pose.
 *  bun guard.ts <slug> [scales,comma] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const slug = process.argv[2]!;
const category = slug === "labrador-dog" ? "🐕️animals" : "👤️humans";
const file = process.env.FILE ?? join(S, "🗂️catalog", category, slug, "🔣️.json");
const asset = (await Bun.file(file).json()).asset;
const scales = (process.argv[3] ?? "50,100,200,500").split(",").map(Number);
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of scales) {
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale, process.env.POSE ?? undefined, undefined, feature);
  const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]) > feature ? "!" : ""}`);
  if (m.bbox.some((b: number, k: number) => Math.abs(b - dims[k]) > feature)) bad++;
  const vs = VIEWPOINTS.map((v: string) => {
    const x = m.views[v];
    const fail = x.iou < 0.92 || x.hausdorffM > 2 * feature;
    if (fail) bad++;
    return `${v} iou ${x.iou.toFixed(3)} h ${(x.hausdorffM * 1000).toFixed(0)}mm${fail ? " FAIL" : ""} (p ${x.precision.toFixed(3)} r ${x.recall.toFixed(3)})`;
  });
  console.log(`1:${scale} bbox ${bb.join(" ")} | ${vs.join(" | ")}`);
}
console.log(bad === 0 ? "guard ok" : `guard ${bad} failing`);
