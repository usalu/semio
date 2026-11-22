/** 🔎️ PW-07 probe: validation issues, every print-suitability violation with its paper position, and the catalog-silhouette measures (bbox, IoU, Hausdorff per view and scale) of one asset file.
 *
 * `repo tools exec bun -- probe.ts --file <asset json> [--scales 500,200,100,50] [--no-suit] [--no-sil] [--limit 40]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { fileURLToPath } from "node:url";

const S = fileURLToPath(new URL("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/", import.meta.url));
const { validateAsset } = await import(`${S}🔨️modules/🗂️catalog/🟦️.ts`);
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}🔨️modules/🖨️representation/🟦️.ts`);
const { measureAsset } = await import(`${S}🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);

const { values } = parseArgs({ options: { file: { type: "string" }, scales: { type: "string", default: "500,200,100,50" }, "no-suit": { type: "boolean", default: false }, "no-sil": { type: "boolean", default: false }, "no-val": { type: "boolean", default: false }, limit: { type: "string", default: "40" } } });
const asset = JSON.parse(readFileSync(values.file!, "utf8")).asset;
const scales = values.scales!.split(",").map(Number);
const limit = Number(values.limit);
if (!values["no-val"]) {
  const t0 = Date.now();
  const issues = validateAsset(asset);
  console.log(`[DEBUG] validate ${issues.length} issues ${Date.now() - t0} ms`);
  for (const issue of issues.slice(0, limit)) console.log(`  ${issue.code} ${issue.path}: ${issue.message.en}`);
}
for (const d of scales) {
  const rep = asset.representations[String(d)];
  if (!values["no-suit"]) {
    const t0 = Date.now();
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`[DEBUG] 1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length} ${Date.now() - t0} ms`);
    for (const v of s.violations.slice(0, limit)) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm?.toFixed?.(2)},${v.at.yMm?.toFixed?.(2)} ${JSON.stringify(v.at).slice(0, 80)}` : ""}${v.viewpoint ? ` ${v.viewpoint}` : ""}`);
  }
  if (!values["no-sil"]) {
    const t0 = Date.now();
    const m = measureAsset(asset, d, undefined, undefined, featureOf(d));
    const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
    console.log(`[DEBUG] 1:${d} bbox ${m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]} (${((b - dims[k]!) * 1000).toFixed(0)} mm)`).join(" ")} feature ${(featureOf(d) * 1000).toFixed(1)} mm ${Date.now() - t0} ms`);
    for (const view of ["elevation", "front", "plan"]) {
      const v = m.views[view];
      console.log(`   ${view}: iou ${v.iou.toFixed(4)} hausdorff ${(v.hausdorffM * 1000).toFixed(1)} mm (limit ${(2 * featureOf(d) * 1000).toFixed(1)}) recall ${v.recall.toFixed(3)} precision ${v.precision.toFixed(3)} shapes ${v.shapes}`);
    }
  }
}
