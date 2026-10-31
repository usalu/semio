import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { loadCatalogFromFiles } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { values } = parseArgs({ options: { assets: { type: "string" }, limit: { type: "string", default: "40" } } });
const cat = await loadCatalogFromFiles();
for (const id of (values.assets as string).split(",")) {
  const asset = cat.assets.find((a: any) => a.id === id);
  console.log(`== ${id}`);
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (rep.status !== "authored") { console.log(`1:${d} unavailable`); continue; }
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`1:${d} suitable=${s.suitable} violations=${s.violations.length}`);
    for (const v of s.violations.slice(0, Number(values.limit))) {
      const at = v.at ? ` at model x=${((v.at.xMm * d) / 1000).toFixed(3)} y=${((-v.at.yMm * d) / 1000).toFixed(3)}` : "";
      console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${at}`);
    }
  }
}
