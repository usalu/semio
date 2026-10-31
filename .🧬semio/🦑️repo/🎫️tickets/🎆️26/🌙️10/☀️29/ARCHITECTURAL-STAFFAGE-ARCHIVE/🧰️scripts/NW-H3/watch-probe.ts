import { readFileSync, statSync, writeFileSync, existsSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE";
const OUT = `${T}/🗑️generated/NW-H3`;
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const ids = process.argv.slice(2);
const file = (id: string) => `${S}/🗂️catalog/👤️humans/${id}/🔣️.json`;
const seen = new Map<string, number>();
const deadline = Date.now() + 110 * 60 * 1000;
while (Date.now() < deadline && !existsSync(`${OUT}/STOP`)) {
  for (const id of ids) {
    let m = 0;
    try { m = statSync(file(id)).mtimeMs; } catch { continue; }
    if (seen.get(id) === m) continue;
    seen.set(id, m);
    try {
      const asset = JSON.parse(readFileSync(file(id), "utf8")).asset;
      const lines: string[] = [`# ${id} ${new Date().toISOString()}`];
      for (const d of [500, 200, 100, 50]) {
        const rep = asset.representations[String(d)];
        if (rep.status !== "authored") { lines.push(`1:${d} unavailable`); continue; }
        const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
        lines.push(`1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
        for (const v of s.violations) lines.push(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
      }
      writeFileSync(`${OUT}/watch-${id}.txt`, lines.join("\n") + "\n");
    } catch (e) { writeFileSync(`${OUT}/watch-${id}.txt`, `ERROR ${(e as Error).message}\n`); }
  }
  await new Promise((r) => setTimeout(r, 1500));
}
