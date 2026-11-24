/** 📤️ Dumps the measured views (the polygons `exactIou` receives and the owned IoU) of the five `exact` assets, one JSON file per asset, so two versions of the oracle can be compared on identical input.
 *  bun dump.ts <out-directory> [id…] */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const C = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const FILES: Record<string, string> = { "cooking-pot": "🪑️objects", hedgehog: "🐕️animals", "retractable-bollard": "🪑️objects", "panel-radiator": "🧱️components", "sliding-door": "🧱️components" };
const [out, ...ids] = process.argv.slice(2);
for (const id of ids.length > 0 ? ids : Object.keys(FILES)) {
  const asset = (await Bun.file(`${C}/${FILES[id]}/${id}/🔣️.json`).json()).asset;
  const views: unknown[] = [];
  for (const scale of [50, 100, 200, 500]) {
    if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") continue;
    const m = measureAsset(asset, scale, undefined, undefined, featureOf(scale));
    for (const view of VIEWPOINTS) {
      const v = m.views[view];
      views.push({ id, scale, view, iou: v.iou, section: v.section, filled: v.filled, outlines: v.outlines, reference: v.reference, visible: v.visible, full: v.full });
    }
  }
  await Bun.write(`${out}/${id}.json`, JSON.stringify(views));
  console.log(`${id}: ${views.length} views`);
}
