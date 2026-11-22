/** 🚧️ PW-12 catalog-silhouette gate probe: the per-asset assertions of `🧪️tests/🧊️catalog-silhouette` (bounding box within one smallest feature of `dimensions`, IoU at least the gate and Hausdorff at most two features per view, every authored scale) for the given assets only.
 *
 * `repo tools exec bun -- <this file> <asset id>...`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { SCALE_DENOMINATORS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { loadCatalogFromFiles } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const gate = JSON.parse(readFileSync(join(root, "🧫️fixtures/🧊️catalog-silhouette/🔣️.json"), "utf8")).gate as { iou: number };
const ids = process.argv.slice(2);
const catalog = await loadCatalogFromFiles(undefined, undefined, new Set(ids));
let bad = 0;
for (const asset of catalog.assets.filter((a) => ids.includes(a.id))) {
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of SCALE_DENOMINATORS) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const box = m.bbox.map((b, k) => `${(b - dims[k]!).toFixed(4)}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`);
    const views = VIEWPOINTS.map((v) => `${v} iou ${m.views[v].iou.toFixed(3)}${m.views[v].iou < gate.iou ? "!" : ""} h ${m.views[v].hausdorffM.toFixed(3)}${m.views[v].hausdorffM > 2 * feature ? "!" : ""}`);
    const line = `[DEBUG] ${asset.id} 1:${scale} feature ${feature} bbox ${box.join(" ")} | ${views.join(" | ")}`;
    if (line.includes("!")) bad++;
    console.log(line);
  }
}
console.log(`[DEBUG] gate ${gate.iou}: ${bad} failing scales`);
