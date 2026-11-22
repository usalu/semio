/** 🌓️ The catalog-silhouette gate (bbox within one feature, IoU and Hausdorff per view) for given human assets: bun silhouette.ts <slug|file> ... */
import { join, resolve } from "node:path";
import { evaluate, SCALE_DENOMINATORS } from "./lib.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { GATE, VIEWPOINTS, bodyDimensions, worldSolids, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json")).json()).asset;
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of SCALE_DENOMINATORS) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones, scale));
    const rows = [`1:${scale} bbox ${bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ")}`];
    for (const view of VIEWPOINTS) {
      const v = m.views[view];
      const fail = v.iou < GATE.iou || v.hausdorffM > 2 * feature;
      if (fail) bad++;
      rows.push(`${view} iou ${v.iou.toFixed(3)} haus ${v.hausdorffM.toFixed(3)}${fail ? " FAIL" : ""}`);
    }
    bad += bbox.filter((b: number, k: number) => Math.abs(b - dims[k]!) > feature).length;
    console.log(asset.id, rows.join(" | "));
  }
  console.log(asset.id, bad ? `FAILING ${bad}` : "ok", "gate iou", GATE.iou);
}
