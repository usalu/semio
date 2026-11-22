/** 🌓️ The catalog-silhouette gate for the given humans (as 🧪️tests/🧊️catalog-silhouette measures it): per authored scale the volume bbox against `dimensions`
 *  (one smallest feature), and per view IoU (≥ 0.92) and Hausdorff (≤ 2 features) between drawing and volume in the default pose.
 *  bun silhouette.ts slug [slug ...] */
import { join, resolve } from "node:path";
import { SCALE_DENOMINATORS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const asset = (await Bun.file(join(catalog, slug, "🔣️.json")).json()).asset;
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of SCALE_DENOMINATORS) {
    if (asset.volumes[`${scale}`]?.status !== "authored" || asset.representations[`${scale}`].status !== "authored") continue;
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`);
    const views = VIEWPOINTS.map((v) => { const x = m.views[v]; const ok = x.iou >= 0.92 && x.hausdorffM <= 2 * feature; if (!ok) bad++; return `${v} iou ${x.iou.toFixed(3)} h ${(x.hausdorffM * 1000).toFixed(0)}mm${ok ? "" : " FAIL"}`; });
    if (m.bbox.some((b: number, k: number) => Math.abs(b - dims[k]!) > feature)) bad++;
    console.log(`${slug} 1:${scale} feature ${feature * 1000} mm bbox ${bb.join(" ")} | ${views.join(" | ")}`);
  }
}
console.log(`failing checks ${bad}`);
