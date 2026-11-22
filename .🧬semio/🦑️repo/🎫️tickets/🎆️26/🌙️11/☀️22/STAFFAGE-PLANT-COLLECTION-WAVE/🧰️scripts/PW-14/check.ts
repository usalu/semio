/** 🩺️ PW-14 check of one asset file: schema (ajv), `validateAsset`, the catalog check, `checkVolumes` (every geometric volume rule) and the catalog-silhouette gate (bbox within one feature, IoU ≥ 0.92, Hausdorff ≤ 2 features per view and scale).
 * `repo tools exec bun -- check.ts <asset.json> [--fast]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
const S = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
const { checkAssetDetailed } = await import(`${S}/🔨️modules/🗂️catalog/🩺️check/🟦️.ts`);
const { checkVolumes, measureVolumes } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);
const { measureAsset } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);
const file = process.argv[2]!;
const fast = process.argv.includes("--fast");
const doc = JSON.parse(readFileSync(file, "utf8"));
const asset = doc.asset;
const schema = JSON.parse(readFileSync(new URL(`${S}/🧬️schema/🔣️.json`, import.meta.url), "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(schema);
const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/AssetDocument` });
const ok = validate(doc);
const schemaErrors = ok ? [] : (validate.errors ?? []).filter((e) => e.keyword !== "oneOf" && e.keyword !== "anyOf");
console.log(`[DEBUG] schema ${ok ? "ok" : `${schemaErrors.length} errors`}`);
for (const e of schemaErrors.slice(0, 20)) console.log(`[DEBUG]   ${e.instancePath} ${e.message} ${JSON.stringify(e.params)}`);
const issues = validateAsset(asset);
console.log(`[DEBUG] validateAsset ${issues.length}`);
for (const i of issues.slice(0, 40)) console.log(`[DEBUG]   ${i.code} ${i.path}: ${i.message.en}`);
const report = checkAssetDetailed(asset, { samples: 9 });
console.log(`[DEBUG] check ${report.problems.length ? "✗" : "✓"}`);
for (const l of report.lines) console.log(`[DEBUG]   ${l}`);
for (const p of report.problems.slice(0, 40)) console.log(`[DEBUG]   ${p}`);
if (process.argv.includes("--areas")) {
  for (const m of measureVolumes(asset, { rules: ["volumes-thermal-coverage", "volumes-cross-scale"] })) if (m.name.startsWith("info:outerAreaM2") || m.name.startsWith("areaDeviation") || m.name.startsWith("iou") || m.name.startsWith("unionVolume")) console.log(`[DEBUG] measure 1:${m.scale} ${m.name} ${m.value.toFixed(4)} ${m.ok ? "" : "✗"}`);
}
if (!fast) {
  const findings = checkVolumes(asset);
  console.log(`[DEBUG] checkVolumes ${findings.length}`);
  for (const f of findings.slice(0, 60)) console.log(`[DEBUG]   ${f.rule} ${f.path}: ${f.message.en}`);
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of [50, 100, 200, 500]) {
    if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") continue;
    const f = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, f);
    const bbox = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > f ? "✗" : ""}`).join(" ");
    const views = Object.entries(m.views).map(([v, x]: [string, any]) => `${v} iou ${x.iou.toFixed(3)}${x.iou < 0.92 ? "✗" : ""} h ${(x.hausdorffM * 1000).toFixed(0)}mm${x.hausdorffM > 2 * f ? "✗" : ""}`).join(", ");
    console.log(`[DEBUG] gate 1:${scale} bbox ${bbox} | ${views}`);
  }
}
