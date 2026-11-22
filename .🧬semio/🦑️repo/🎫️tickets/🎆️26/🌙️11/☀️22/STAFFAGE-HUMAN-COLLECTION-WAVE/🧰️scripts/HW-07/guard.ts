/** 🧊️ The catalog silhouette guard of `🧪️tests/🧊️catalog-silhouette` for one asset file or slug: bounding box within one feature of
 * `dimensions`, IoU ≥ 0.92 and Hausdorff ≤ 2 features in every view of every authored scale (default pose). bun guard.ts <slug|file> */
import { join } from "node:path";
import { SCALE_DENOMINATORS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { HUMANS } from "./lib.ts";

for (const t of process.argv.slice(2)) {
  const path = t.endsWith(".json") ? t : join(HUMANS, t, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of SCALE_DENOMINATORS) {
    if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") continue;
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const line: string[] = [];
    m.bbox.forEach((b: number, k: number) => { const ok = Math.abs(b - dims[k]!) <= feature; if (!ok) bad++; line.push(`${"xyz"[k]} ${b.toFixed(3)}/${dims[k]}${ok ? "" : " FAIL"}`); });
    for (const v of VIEWPOINTS) { const mv = m.views[v]; const ok = mv.iou >= 0.92 && mv.hausdorffM <= 2 * feature; if (!ok) bad++; line.push(`${v} iou ${mv.iou.toFixed(3)} h ${(mv.hausdorffM * 1000).toFixed(0)}mm${ok ? "" : " FAIL"}`); }
    console.log(`${asset.id} 1:${scale}  ${line.join(" | ")}`);
  }
  console.log(`${asset.id} guard ${bad === 0 ? "PASS" : `FAIL ${bad}`}`);
}
