/** 🛡️ The silhouette guard of assets, nothing else: `bun guard.ts <id|file.json>… [--scales 50,100]` prints per authored scale the boxes against the dimensions and per view the IoU and the largest boundary distance of `measureAsset` with the gate of `🧊️catalog-silhouette` (IoU 0.92, twice the smallest feature), one line per scale and one verdict per asset. Run it from `S/📦️packages/🟦️typescript`.
 * @see ../MR/measure.ts — the same guard with validation and the three-views sweep */
import { parseArgs } from "node:util";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, tag: { type: "string" } } });
for (const arg of positionals) {
  const asset = readDoc(arg).doc.asset;
  const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? (bad++, "!") : ""}`).join(" ");
    const vs = VIEWPOINTS.map((v: string) => `${v} ${m.views[v].iou.toFixed(4)}${m.views[v].iou < 0.92 ? (bad++, "!") : ""}/${(m.views[v].hausdorffM * 1000).toFixed(1)}mm${m.views[v].hausdorffM > 2 * feature ? (bad++, "!") : ""}`).join("  ");
    console.log(`${values.tag ?? ""}${asset.id} 1:${scale} bbox ${bb}  ${vs}`);
  }
  console.log(`${values.tag ?? ""}${asset.id} silhouette: ${bad === 0 ? "OK" : `${bad} failing checks`}`);
}
