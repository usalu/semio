/** 🔬️ Finisher probe of one asset file: JSON schema (ajv), `validateAsset` with the environment presets, the measured volume rules (`checkVolumes`) and the catalog-silhouette gate (bounding box within one smallest feature of `dimensions`, IoU at least the gate of the fixture, Hausdorff at most two features per view and authored scale).
 *
 * `repo tools exec bun -- probe.ts <asset document json> [--no-volumes] [--no-gate]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadEnvironments } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { checkVolumes, featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const args = process.argv.slice(2);
const doc = JSON.parse(readFileSync(args[0]!, "utf8"));
const asset = doc.asset;
const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ strict: false, allErrors: true }).addSchema(schema).compile({ $ref: `${schema.$id}#/$defs/AssetDocument` });
const schemaErrors = validate(doc) ? [] : (validate.errors ?? []).filter((e) => e.keyword !== "oneOf" && e.keyword !== "anyOf");
console.log(`SCHEMA ${asset.id}: ${schemaErrors.length} error(s)`);
for (const e of schemaErrors.slice(0, 30)) console.log(`  ${e.instancePath} ${e.message} ${JSON.stringify(e.params)}`);
const environments = new Set(((await loadEnvironments(DEFAULT_CATALOG_ROOT))?.presets ?? []).map((preset: { id: string }) => preset.id));
const issues = validateAsset(asset, { assets: new Map([[asset.id, asset]]), environments } as never);
console.log(`VALIDATE ${asset.id}: ${issues.length} issue(s)`);
for (const i of issues.slice(0, 60)) console.log(`  ${i.code} ${i.path}: ${i.message.en.slice(0, 500)}`);
let findings = 0;
if (!args.includes("--no-volumes")) {
  const t0 = Date.now();
  const found = checkVolumes(asset);
  findings = found.length;
  console.log(`VOLUMES ${asset.id}: ${found.length} finding(s) (${((Date.now() - t0) / 1000).toFixed(0)} s)`);
  for (const f of found.slice(0, 80)) console.log(`  ${f.rule} ${f.path}: ${f.message.en.slice(0, 500)}`);
}
let bad = 0;
if (!args.includes("--no-gate")) {
  const gate = JSON.parse(readFileSync(join(root, "🧫️fixtures/🧊️catalog-silhouette/🔣️.json"), "utf8")).gate as { iou: number };
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of [50, 100, 200, 500] as const) {
    if (asset.volumes?.[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") { console.log(`GATE 1:${scale} skipped (${asset.representations[String(scale)].status})`); continue; }
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const box = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "✗" : ""}`).join(" ");
    const views = VIEWPOINTS.map((v) => `${v} iou ${m.views[v].iou.toFixed(3)}${m.views[v].iou < gate.iou ? "✗" : ""} h ${(m.views[v].hausdorffM * 1000).toFixed(0)}mm${m.views[v].hausdorffM > 2 * feature ? "✗" : ""}`).join(" | ");
    const line = `GATE 1:${scale} bbox ${box} | ${views}`;
    if (line.includes("✗")) bad++;
    console.log(line);
  }
  console.log(`GATE ${asset.id}: ${bad} failing scale(s) at IoU ${gate.iou}`);
}
console.log(`PROBE ${asset.id}: ${schemaErrors.length + issues.length + findings + bad === 0 ? "✓" : "✗"}`);
process.exitCode = schemaErrors.length + issues.length + findings + bad === 0 ? 0 : 1;
