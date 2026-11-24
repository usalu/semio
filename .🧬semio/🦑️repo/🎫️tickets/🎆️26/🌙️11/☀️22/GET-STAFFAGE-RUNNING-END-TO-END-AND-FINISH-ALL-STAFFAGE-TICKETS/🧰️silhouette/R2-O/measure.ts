/** 📐️ The checks of the `🧊️catalog-silhouette` test for objects of batch R2-O: bounding box per axis, IoU and largest boundary distance per authored scale and view (`!` marks a failing one), optionally `checkVolumes`.
 *  bun measure.ts <id|file.json>… [--volumes] */
import { parseArgs } from "node:util";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureAsset, VIEWPOINTS } = await import(`${S}/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts`);
const { checkVolumes, featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);
const { values, positionals } = parseArgs({ allowPositionals: true, options: { volumes: { type: "boolean" }, scales: { type: "string" } } });
for (const arg of positionals) {
  const path = arg.endsWith(".json") ? arg : `${S}/🗂️catalog/🪑️objects/${arg}/🔣️.json`;
  const asset = (await Bun.file(path).json()).asset;
  const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    if (asset.representations[String(scale)].status === "authored") {
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      const bb = m.bbox.map((b: number, k: number) => { const f = Math.abs(b - dims[k]!) > feature; if (f) bad++; return `${b.toFixed(4)}/${dims[k]}${f ? "!" : ""}`; }).join(" ");
      const vs = VIEWPOINTS.map((v: string) => { const x = m.views[v]; const f1 = x.iou < 0.92, f2 = x.hausdorffM > 2 * feature; if (f1) bad++; if (f2) bad++; return `${v}${m.sections[v] === undefined ? "" : "(s)"} ${x.iou.toFixed(3)}${f1 ? "!" : ""}/${(x.hausdorffM * 1000).toFixed(1)}mm${f2 ? "!" : ""}`; }).join("  ");
      console.log(`${asset.id} 1:${scale} f=${feature * 1000}mm bbox ${bb}  ${vs}`);
    } else console.log(`${asset.id} 1:${scale} representation ${asset.representations[String(scale)].status}`);
    if (values.volumes) for (const f of checkVolumes(asset, { scale })) { bad++; console.log(`   ${f.rule} ${f.path}: ${f.message.en}`.slice(0, 400)); }
  }
  console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
}
