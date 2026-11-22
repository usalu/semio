/** 🌓️ Silhouette guard line (bbox, IoU, Hausdorff per view and scale) of an asset document, as `catalog-silhouette` measures it.
 *  bun measure.ts <file.json|slug> [--pose id] [--dims] [scale ...] */
import { join, resolve } from "node:path";
import type { Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { GATE, VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const target = args.shift()!;
let pose: string | undefined;
const scales: number[] = [];
for (let i = 0; i < args.length; i++) args[i] === "--pose" ? (pose = args[++i]) : scales.push(Number(args[i]));
const path = target.endsWith(".json") ? target : join(catalog, target, "🔣️.json");
const asset = (await Bun.file(path).json()).asset as Asset;
if (pose) asset.presentation.defaultPose = pose;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of scales.length ? scales : [500, 200, 100, 50]) {
  const feature = featureOf(scale as never);
  const m = measureAsset(asset, scale as never, undefined, undefined, feature);
  const out: string[] = [`bbox ${m.bbox.map((b) => b.toFixed(4)).join("/")}`];
  m.bbox.forEach((b, k) => { if (Math.abs(b - dims[k]!) > feature) { out.push(`!bbox${k} ${b.toFixed(4)} vs ${dims[k]}`); bad++; } });
  for (const v of VIEWPOINTS) { const x = m.views[v]; const f = x.iou < GATE.iou || x.hausdorffM > 2 * feature; if (f) bad++; out.push(`${v}:${x.iou.toFixed(3)}/${(x.hausdorffM * 1000).toFixed(1)}mm${f ? "!" : ""}`); }
  console.log(`1:${scale} limit ${(2 * feature * 1000).toFixed(0)}mm  ${out.join("  ")}`);
}
console.log(bad === 0 ? "silhouette ok" : `silhouette failing ${bad}`);
