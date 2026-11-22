/** 🧊️ HW-09 scoped run of the catalog-silhouette gate (🧪️tests/🧊️catalog-silhouette) for the given human slugs: bbox against dimensions
 * within one smallest feature, IoU and Hausdorff of drawing and volume per authored scale and view. bun silhouette.ts <slug>... */
import { SCALE_DENOMINATORS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { GATE, VIEWPOINTS, bodyDimensions, worldSolids, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { loadHuman } from "./kin.ts";
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const { asset } = await loadHuman(slug);
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of SCALE_DENOMINATORS) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones, scale));
    const line = [`${slug} 1:${scale} bbox ${bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}`).join(" ")}`];
    bbox.forEach((b: number, k: number) => { if (Math.abs(b - dims[k]!) > feature) { bad++; line.push(`BBOX${k}`); } });
    for (const v of VIEWPOINTS) {
      const x = m.views[v];
      const ok = x.iou >= GATE.iou && x.hausdorffM <= 2 * feature;
      if (!ok) bad++;
      line.push(`${v} iou ${x.iou.toFixed(3)} H ${(x.hausdorffM * 1000).toFixed(0)}mm${ok ? "" : " FAIL"}`);
    }
    console.log(line.join(" | "));
  }
}
console.log(`failures ${bad} (gate iou ${GATE.iou})`);
