/** 📐️ Every check of the catalog-silhouette guard of furniture at every authored scale (bbox per axis, IoU and largest boundary distance per view, `!` marks a failing one), the seconds each scale took, and optionally `checkVolumes`.
 *  bun measure.ts <slug|file.json>… [--scales 50,100] [--volumes] */
import { parseArgs } from "node:util";
import { measureAsset, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { checkVolumes, featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { fileOf } from "./lib.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, volumes: { type: "boolean" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    if (asset.representations[String(scale)].status === "authored") {
      const t = performance.now();
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ");
      const vs = VIEWPOINTS.map((v) => { const x = m.views[v]; const fail = x.iou < 0.92 || x.hausdorffM > 2 * feature; if (fail) bad++; return `${v} ${x.iou.toFixed(3)}/${(x.hausdorffM * 1000).toFixed(1)}mm${fail ? "!" : ""}${m.sections[v] ? "§" : ""}`; }).join("  ");
      bad += bb.split("!").length - 1;
      console.log(`${asset.id} 1:${scale} bbox ${bb}  ${vs}  (${((performance.now() - t) / 1000).toFixed(1)} s)`);
    }
    if (values.volumes) {
      const findings = checkVolumes(asset, { scale });
      for (const f of findings) console.log(`   1:${scale} ${f.rule} ${f.path}: ${f.message.en}`.slice(0, 500));
    }
  }
  console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
}
