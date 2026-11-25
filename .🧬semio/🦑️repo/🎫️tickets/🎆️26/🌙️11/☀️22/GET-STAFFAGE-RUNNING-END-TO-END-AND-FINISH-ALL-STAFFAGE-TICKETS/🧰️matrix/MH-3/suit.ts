/** 🖨️ The print suitability of asset documents outside the check CLI: `bun suit.ts <id|file.json>…` prints per authored scale what `measureSuitability` of the product says under the default print profile (suitable or not, smallest stroke, gap and feature in paper millimetres, the first violations). The check CLI fails an asset whose authored scale is unsuitable; a refit that changes how much of a small part shows (an ear beside a head) can do that, so this runs after every dry run. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { DEFAULT_PRINT_PROFILE, measureSuitability } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
let bad = 0;
for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset;
  for (const [key, rep] of Object.entries(asset.representations) as [string, any][]) {
    if (rep.status !== "authored") continue;
    const s = measureSuitability(asset, rep, Number(key), DEFAULT_PRINT_PROFILE);
    if (!s.suitable) bad++;
    const mm = (value: number | null | undefined): string => (typeof value === "number" ? value.toFixed(3) : "–");
    console.log(`${asset.id} 1:${key} ${s.suitable ? "suitable" : "UNSUITABLE"} stroke ${mm(s.minStrokeMm)} gap ${mm(s.minGapMm)} feature ${mm(s.minFeatureMm)}${s.suitable ? "" : ` ${JSON.stringify(s.violations ?? s.issues ?? s.reasons ?? "").slice(0, 300)}`}`);
  }
}
console.log(`suitability: ${bad === 0 ? "OK" : `${bad} unsuitable scales`}`);
