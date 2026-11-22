/** 🖨️ Print suitability of every authored scale of one human asset with the violating shapes, viewpoint and paper position.
 *  bun suit.ts <slug> [limit] */
import { join, resolve } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [slug, limit] = process.argv.slice(2);
const asset = (await Bun.file(slug!.endsWith(".json") ? slug! : join(catalog, slug!, "🔣️.json")).json()).asset;
for (const d of [500, 200, 100, 50]) {
  const rep = asset.representations[String(d)];
  const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
  console.log(`1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
  for (const v of s.violations.slice(0, Number(limit ?? 12))) console.log(`   ${JSON.stringify(v).slice(0, 300)}`);
}
