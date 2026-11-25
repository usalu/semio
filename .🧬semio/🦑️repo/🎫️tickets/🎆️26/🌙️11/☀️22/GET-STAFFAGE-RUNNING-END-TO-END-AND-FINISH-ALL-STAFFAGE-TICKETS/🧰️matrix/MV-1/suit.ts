/** 🖨️ The print suitability of every authored scale of an asset or of a trial copy, as the check CLI and the scan measure it (`measureSuitability` under the default print profile), outside the catalog: `bun suit.ts <id|file.json>… [--scales 200]` prints one line per scale and every violation. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
const args = process.argv.slice(2);
const only = args.includes("--scales") ? args[args.indexOf("--scales") + 1]!.split(",") : undefined;
let bad = 0;
for (const arg of args.filter((value, i) => !value.startsWith("--") && args[i - 1] !== "--scales")) {
  const asset = readDoc(arg).doc.asset;
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.status !== "authored" || (only !== undefined && !only.includes(key))) continue;
    const s = measureSuitability(asset, representation, Number(key), DEFAULT_PRINT_PROFILE);
    if (!s.suitable) bad++;
    console.log(`${asset.id} 1:${key} ${s.suitable ? "suitable" : "UNSUITABLE"}: min stroke ${s.minStrokeMm} mm, min gap ${s.minGapMm} mm, min feature ${s.minFeatureMm} mm`);
    for (const v of s.violations ?? []) console.log(`   ${v.check} ${v.shapes.join("+")}: ${v.measuredMm} mm < ${v.requiredMm} mm`);
  }
}
console.log(`unsuitable scales: ${bad}`);
