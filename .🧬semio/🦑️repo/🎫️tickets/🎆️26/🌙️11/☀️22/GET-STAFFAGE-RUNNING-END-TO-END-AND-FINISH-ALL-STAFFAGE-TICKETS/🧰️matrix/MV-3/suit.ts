/** 🖨️ Whether every authored scale of an asset prints: `bun suit.ts <id|file.json>…` resolves each authored scale under the default print profile as the product does before it draws and prints the kind of the resolution and the first violations (`N=<count>` shows more); the last line per asset is `prints` or the number of unsuitable scales. The exit code is 1 when a scale does not print.
 * @see ../../🧰️silhouette/R4-P/suit.ts — the probe this one follows, for trial files as well */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
let unsuitable = 0;
for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset;
  let bad = 0;
  for (const key of Object.keys(asset.representations)) {
    if (asset.representations[key]?.status !== "authored") continue;
    const resolved: any = resolveRepresentation(asset, { denominator: Number(key) } as never, DEFAULT_PRINT_PROFILE);
    const violations = resolved.suitability?.violations ?? resolved.violations ?? [];
    if (resolved.kind !== "authored") bad++;
    console.log(`${asset.id} 1:${key}: ${resolved.kind}${violations.length > 0 ? `, ${violations.length} violations` : ""}`);
    for (const violation of violations.slice(0, Number(process.env.N ?? 6))) console.log(`   ${JSON.stringify(violation).slice(0, 360)}`);
  }
  unsuitable += bad;
  console.log(`${asset.id}: ${bad === 0 ? "prints" : `${bad} scales unsuitable`}`);
}
process.exit(unsuitable === 0 ? 0 : 1);
