/** 📐️ Every check of the silhouette guard for people or asset files (bounding box per axis, IoU and largest boundary distance per scale and view) and, with `--volumes`, every rule of `checkVolumes`.
 *  bun measure.ts <slug|file.json>… [--scales 50,100] [--volumes] */
import { parseArgs } from "node:util";
import { measureAsset, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { checkVolumes, featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { fileOf } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, volumes: { type: "boolean" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    if (asset.representations[String(scale)].status === "authored") {
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      const bb = m.bbox.map((b: number, k: number) => { const f = Math.abs(b - dims[k]!) > feature; if (f) bad++; return `${b.toFixed(4)}/${dims[k]}${f ? "!" : ""}`; }).join(" ");
      const vs = VIEWPOINTS.map((v) => { const x = m.views[v]; const fi = x.iou < 0.92, fh = x.hausdorffM > 2 * feature; if (fi) bad++; if (fh) bad++; return `${v} ${x.iou.toFixed(4)}${fi ? "!" : ""}/${(x.hausdorffM * 1000).toFixed(1)}mm${fh ? "!" : ""}`; }).join("  ");
      console.log(`${asset.id} 1:${scale} bbox ${bb}  ${vs}`);
    }
    if (values.volumes) {
      const findings = checkVolumes(asset, { scale }).filter((f: any) => f.rule !== "volumes-silhouette");
      for (const f of findings) console.log(`   1:${scale} ${f.rule} ${f.path}: ${f.message.en}`.slice(0, 400));
      console.log(`${asset.id} 1:${scale} volume findings ${findings.length}`);
    }
  }
  console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
}
