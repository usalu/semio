// [scratch] NW-A1 one-process probe of the four assets: validation issues, planted travel, suitability violations with positions
import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { validateAsset } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
const { sampleClip } = await import(`${S}/🔨️modules/🎞️animation/🟦️.ts`);
const ids: string[] = process.argv.slice(2);
for (const id of ids) {
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/🐕️animals/${id}/🔣️.json`, "utf8")).asset;
  console.log(`== ${id}`);
  for (const i of validateAsset(asset, {})) console.log(`  issue ${i.code} ${i.path}: ${i.message.en}`);
  for (const c of asset.clips) if (c.rootMotion === "plant" && c.loop) console.log(`  travel ${c.id} ${(sampleClip(asset, c, c.durationS - 1e-9).root.x - sampleClip(asset, c, 0).root.x).toFixed(4)} advance ${c.advanceM}`);
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (rep.status !== "authored") { console.log(`  1:${d} unavailable`); continue; }
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`  1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
    for (const v of s.violations.slice(0, 14)) console.log(`     ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
  }
}
