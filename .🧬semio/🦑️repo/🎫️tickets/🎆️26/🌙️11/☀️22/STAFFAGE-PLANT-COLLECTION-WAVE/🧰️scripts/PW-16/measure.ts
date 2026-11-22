/** 📏️ PW-16 author probe: validation issues, the catalog-silhouette gate (bbox within one feature, IoU ≥ 0.92, Hausdorff ≤ 2 features per view and scale) and optionally the geometric volume rules of one asset file.
 *
 * `repo tools exec bun -- measure.ts <asset.json> [--scales 50,100] [--volumes rule,rule|all] [--silhouette 0]`
 *
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
import type { Asset, ScaleDenominator } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { VOLUME_RULES, checkVolumes, featureOf, measureVolumes } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string", default: "500,200,100,50" }, volumes: { type: "string", default: "" }, silhouette: { type: "string", default: "1" }, info: { type: "string", default: "" } } });
const asset = (JSON.parse(readFileSync(positionals[0]!, "utf8")) as { asset: Asset }).asset;
const scales = values.scales.split(",").map(Number) as ScaleDenominator[];
const issues = validateAsset(asset);
console.log(`[DEBUG] ${asset.id}: ${issues.length} validation issues`);
for (const issue of issues.slice(0, 60)) console.log(`[DEBUG]   ${issue.code} ${issue.path}: ${issue.message.en}`);
if (values.silhouette === "1")
  for (const scale of scales) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
    const bbox = m.bbox.map((b, k) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "✗" : ""}`).join(" ");
    const views = VIEWPOINTS.map((v) => `${v} iou ${m.views[v].iou.toFixed(3)}${m.views[v].iou < 0.92 ? "✗" : ""} h ${(m.views[v].hausdorffM * 1000).toFixed(0)}mm${m.views[v].hausdorffM > 2 * feature ? "✗" : ""} rec ${m.views[v].recall.toFixed(2)} pre ${m.views[v].precision.toFixed(2)}`).join(" | ");
    console.log(`[DEBUG] 1:${scale} bbox ${bbox} | ${views}`);
  }
if (values.volumes !== "") {
  const rules = (values.volumes === "all" ? VOLUME_RULES : values.volumes.split(",")) as (typeof VOLUME_RULES)[number][];
  for (const scale of scales) {
    const findings = checkVolumes(asset, { scale, rules });
    console.log(`[DEBUG] volumes 1:${scale}: ${findings.length} findings`);
    for (const f of findings.slice(0, 40)) console.log(`[DEBUG]   ${f.rule} ${f.path}: ${f.message.en}`);
    if (values.info !== "") for (const m of measureVolumes(asset, { scale, rules }).filter((x) => x.name.includes(values.info))) console.log(`[DEBUG]   ${m.name} ${m.value.toFixed(4)} (${m.bound} ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)}) ${m.ok ? "" : "✗"}`);
  }
}
