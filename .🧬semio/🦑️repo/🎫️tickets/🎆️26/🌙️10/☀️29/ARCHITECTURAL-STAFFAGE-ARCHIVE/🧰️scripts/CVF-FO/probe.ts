import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const ids = process.argv[2]!.split(",");
const scales = (process.argv[3] ?? "500,200,100,50").split(",").map(Number);
const limit = Number(process.argv[4] ?? 12);
const dirs: Record<string, string> = {};
for (const [cat, list] of [["🛋️furniture", "park-bench cafe-chair office-swivel-chair height-adjustable-desk"], ["🪑️objects", "street-lamp market-parasol waste-bin playground-swing"]] as const) for (const i of list.split(" ")) dirs[i] = `${S}/🗂️catalog/${cat}/${i}/🔣️.json`;
for (const id of ids) {
  const asset = JSON.parse(readFileSync(dirs[id]!, "utf8")).asset;
  for (const d of scales) {
    const rep = asset.representations[String(d)];
    if (rep.status !== "authored") { console.log(`${id} 1:${d} unavailable`); continue; }
    const t0 = Date.now();
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`${id} 1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length} ${Date.now() - t0}ms`);
    for (const v of s.violations.slice(0, limit)) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
  }
}
