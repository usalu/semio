/** 📐️ Every check of the silhouette guard of people, not only the first: bounding box per axis, IoU and largest boundary distance per scale and view, with `!` on each failing one; optional ops applied first.
 *  bun measure.ts <slug|file.json>… [--scales 50,100] [--ops <file>] [--volumes] */
import { parseArgs } from "node:util";
import { measureAsset, VIEWPOINTS, bodyDimensions, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { checkVolumes, featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, ops: { type: "string" }, volumes: { type: "boolean" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  if (values.ops) applyOps(asset, JSON.parse(await Bun.file(values.ops).text()));
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
    } else {
      const bbox = bodyDimensions(worldSolids(asset, (evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }) as any).bones, scale));
      console.log(`${asset.id} 1:${scale} bbox ${bbox.map((b: number, k: number) => { const f = Math.abs(b - dims[k]!) > feature; if (f) bad++; return `${b.toFixed(4)}/${dims[k]}${f ? "!" : ""}`; }).join(" ")} (no drawing)`);
    }
    if (values.volumes) for (const f of checkVolumes(asset, { scale: scale as never })) if (f.rule !== "volumes-silhouette") console.log(`   ${f.rule} ${f.path}: ${f.message.en}`.slice(0, 400));
  }
  console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
}
