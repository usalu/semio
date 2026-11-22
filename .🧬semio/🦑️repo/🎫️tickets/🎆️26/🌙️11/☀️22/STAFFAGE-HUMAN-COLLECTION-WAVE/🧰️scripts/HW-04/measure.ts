/** 📏️ The catalog-silhouette measure of one asset (slug or JSON path) at every authored scale: bbox against dimensions, IoU and Hausdorff per view, the worst drawn shapes.
 *  bun measure.ts <slug|path.json> [pose] [scales=50,100,200,500] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const [arg, poseArg, scalesArg] = process.argv.slice(2);
const path = arg!.endsWith(".json") ? arg! : join(S, "🗂️catalog/👤️humans", arg!, "🔣️.json");
const asset = (await Bun.file(path).json()).asset;
const pose = poseArg && poseArg !== "-" ? poseArg : asset.presentation.defaultPose;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of (scalesArg ?? "50,100,200,500").split(",").map(Number)) {
  const f = featureOf(scale);
  const m = measureAsset(asset, scale, pose, undefined, f);
  const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}(${(b - dims[k]!) >= 0 ? "+" : ""}${((b - dims[k]!) * 1000).toFixed(0)}mm${Math.abs(b - dims[k]!) > f ? "!" : ""})`).join(" ");
  const vs = VIEWPOINTS.map((v: string) => { const x = m.views[v]; const ok = x.iou >= 0.92 && x.hausdorffM <= 2 * f; if (!ok) bad++; return `${v} iou ${x.iou.toFixed(3)} r ${x.recall.toFixed(2)} p ${x.precision.toFixed(2)} h ${(x.hausdorffM * 1000).toFixed(0)}mm${ok ? "" : " FAIL"}`; }).join(" | ");
  console.log(`1:${scale} ${pose} bbox ${bb}\n   ${vs}`);
  if (m.bbox.some((b: number, k: number) => Math.abs(b - dims[k]!) > f)) bad++;
}
console.log(bad === 0 ? "PASS" : `FAILURES ${bad}`);
