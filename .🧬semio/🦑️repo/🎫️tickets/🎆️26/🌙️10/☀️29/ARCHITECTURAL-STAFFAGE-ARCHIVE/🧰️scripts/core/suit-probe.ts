import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { loadCatalogFromFiles } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, limit: { type: "string", default: "12" } } });
const asset = values.file ? JSON.parse(readFileSync(values.file, "utf8")).asset : (await loadCatalogFromFiles()).assets.find((a: any) => a.id === values.asset);
for (const d of [500, 200, 100, 50]) {
  const rep = asset.representations[String(d)];
  if (rep.status !== "authored") { console.log(`1:${d} unavailable`); continue; }
  const t0 = Date.now();
  const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
  console.log(`1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length} ${Date.now() - t0}ms`);
  for (const v of s.violations.slice(0, Number(values.limit))) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
}
