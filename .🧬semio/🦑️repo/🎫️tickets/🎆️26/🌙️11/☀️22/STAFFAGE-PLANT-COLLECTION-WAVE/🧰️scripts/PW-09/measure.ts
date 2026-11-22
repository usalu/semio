/** 📏️ Measures a morphed asset for its generator (PW-09): default-pose extents per authored scale and view, the summed bone mass and the validation issues with their data.
 *
 * `repo tools exec bun -- measure.ts --file <asset json> --out <json> [--no-validate]`
 *
 * @see ./morph.py */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";

const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { validateAsset } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
const { loadEnvironments, DEFAULT_CATALOG_ROOT } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);

const { values } = parseArgs({ options: { file: { type: "string" }, out: { type: "string" }, "no-validate": { type: "boolean", default: false } } });
const asset = JSON.parse(readFileSync(resolve(values.file!), "utf8")).asset;
const extent = (pts: { x: number; y: number }[]) => (pts.length === 0 ? null : { minX: Math.min(...pts.map((p) => p.x)), maxX: Math.max(...pts.map((p) => p.x)), minY: Math.min(...pts.map((p) => p.y)), maxY: Math.max(...pts.map((p) => p.y)) });
const scales: Record<string, unknown> = {};
for (const d of [50, 100, 200, 500]) {
  if (asset.representations[`${d}`]?.status !== "authored") continue;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const views: Record<string, unknown> = {};
  for (const view of ["elevation", "front", "plan"]) views[view] = extent(state.shapes[view].filter((s: { shape: { role: string } }) => s.shape.role !== "ground").flatMap((s: { rings: { x: number; y: number }[][] }) => s.rings.flat()));
  scales[`${d}`] = { bounds: state.bounds, views };
}
const massKg = asset.rig.bones.reduce((sum: number, b: { mass?: { kg: number } }) => sum + (b.mass?.kg ?? 0), 0);
const environments = new Set(((await loadEnvironments(DEFAULT_CATALOG_ROOT))?.presets ?? []).map((p: { id: string }) => p.id));
const issues = values["no-validate"] ? [] : validateAsset(asset, { environments });
writeFileSync(resolve(values.out!), JSON.stringify({ scales, massKg, issues }, null, 1));
console.log(`[DEBUG] measured ${asset.id}: ${issues.length} issues`);
