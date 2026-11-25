/** ⚖️ What the repaired pocket rule changes on catalog assets: `bun measure.ts <id>…` measures every authored scale and view of each asset in its default pose with `measureAsset` of the catalog module and of the trial copy `./silhouette.ts` and prints, per asset, scale and view, the IoU and the largest boundary distance of both and whether the view changes sides at the gate of `🧊️catalog-silhouette` (IoU 0.92, twice the smallest feature). Run it from `S/📦️packages/🟦️typescript`.
 * @see ./patch.py — the patch */
import { join } from "node:path";
import { S, readDoc } from "../../MR/lib.ts";

const now = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const trial = await import("./silhouette.ts");
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));

let [views, moved, toPass, toFail] = [0, 0, 0, 0];
for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset;
  const started = Date.now();
  for (const scale of [50, 100, 200, 500].filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored")) {
    const feature = featureOf(scale);
    const [a, b] = [now.measureAsset(asset, scale, undefined, undefined, feature), trial.measureAsset(asset, scale, undefined, undefined, feature)];
    for (const view of now.VIEWPOINTS) {
      const [x, y] = [a.views[view], b.views[view]];
      const pass = (m: { iou: number; hausdorffM: number }): boolean => m.iou >= 0.92 && m.hausdorffM <= 2 * feature;
      views++;
      const same = Math.abs(x.hausdorffM - y.hausdorffM) < 1e-9 && Math.abs(x.iou - y.iou) < 1e-9;
      if (!same) moved++;
      if (pass(x) !== pass(y)) pass(y) ? toPass++ : toFail++;
      if (!same || !pass(x)) console.log(`${asset.id} 1:${scale} ${view}: now ${x.iou.toFixed(4)}/${(x.hausdorffM * 1000).toFixed(1)} mm ${pass(x) ? "pass" : "FAIL"} | repaired ${y.iou.toFixed(4)}/${(y.hausdorffM * 1000).toFixed(1)} mm ${pass(y) ? "pass" : "FAIL"} (limit ${(2000 * feature).toFixed(0)} mm)${pass(x) !== pass(y) ? "  CHANGES SIDES" : ""}`);
    }
  }
  console.log(`[DEBUG] ${asset.id} measured in ${((Date.now() - started) / 1000).toFixed(0)} s`);
}
console.log(`views ${views}, measure moved ${moved}, fail to pass ${toPass}, pass to fail ${toFail}`);
