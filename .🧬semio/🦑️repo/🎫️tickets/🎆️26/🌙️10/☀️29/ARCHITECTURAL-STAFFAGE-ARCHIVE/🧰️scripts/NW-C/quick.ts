import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { validateAsset } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
for (const id of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/🧱️components/${id}/🔣️.json`, "utf8")).asset;
  console.log(`=== ${id}`);
  for (const i of validateAsset(asset, { assets: new Set(["wheelchair-user", id]) } as any)) console.log(`  ISSUE ${i.code} ${i.path}: ${i.message.en.slice(0, 160)}`);
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (rep.status !== "authored") { console.log(`1:${d} unavailable`); continue; }
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
    for (const v of s.violations.slice(0, 14)) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
  }
}
