/** 🧮️ The polygon-clipping oracle of the catalog silhouette test (`exactIou` of its `🔍️oracles`) per authored scale and view of an asset file: the exact IoU beside the owned one, or the error the library throws.
 *  bun exact.ts <id|file.json>… */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { exactIou } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";
import { fileOf } from "./files.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  for (const scale of [50, 100, 200, 500]) {
    if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") continue;
    const m = measureAsset(asset, scale, undefined, undefined, featureOf(scale));
    for (const view of VIEWPOINTS) {
      try { const e = exactIou(m.views[view]); console.log(`${asset.id} 1:${scale} ${view} exact ${e.toFixed(4)} owned ${m.views[view].iou.toFixed(4)}${Math.abs(e - m.views[view].iou) > 0.06 ? " DIFFER" : ""}`); }
      catch (error) { console.log(`${asset.id} 1:${scale} ${view} THROWS ${String(error).slice(0, 160)}`); }
    }
  }
}
