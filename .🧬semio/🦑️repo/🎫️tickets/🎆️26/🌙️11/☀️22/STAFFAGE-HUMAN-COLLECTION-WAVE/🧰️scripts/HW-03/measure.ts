/** 📐️ Geometric checks of human assets: the catalog-silhouette gate (bbox, IoU, Hausdorff per view and scale) and `checkVolumes` (every volume rule).
 *  bun measure.ts <slug|file.json> [--scales 50,100] [--no-volumes] [--no-silhouette] */
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
const S = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { measureAsset, VIEWPOINTS } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { checkVolumes, featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, "no-volumes": { type: "boolean" }, "no-silhouette": { type: "boolean" } } });
const catalog = resolve(import.meta.dir, `${S}/🗂️catalog/👤️humans`);
for (const arg of positionals) {
  const path = arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const scales = (values.scales ?? "500,200,100,50").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale);
    if (!values["no-silhouette"] && asset.representations[String(scale)].status === "authored") {
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ");
      const vs = VIEWPOINTS.map((v: string) => { const x = m.views[v]; const fail = x.iou < 0.92 || x.hausdorffM > 2 * feature; if (fail) bad++; return `${v} ${x.iou.toFixed(3)}/${(x.hausdorffM * 1000).toFixed(0)}mm${fail ? "!" : ""}`; }).join("  ");
      if (bb.includes("!")) bad++;
      console.log(`${asset.id} 1:${scale} bbox ${bb}  ${vs}`);
    }
    if (!values["no-volumes"]) {
      const findings = checkVolumes(asset, { scale });
      bad += findings.length;
      for (const f of findings) console.log(`   ${f.rule} ${f.path}: ${f.message.en}`.slice(0, 600));
    }
  }
  console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} problems`}`);
}
