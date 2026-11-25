/** 🖨️ Whether the authored drawings of an asset print, with the parts that do not: `bun gaps.ts <id|file.json>… [--scales 50,100]` measures every authored scale under the default print profile (the measurement of the check CLI and of the matrix, over all named poses and the three views) and prints per scale `prints` or every violation with its check, the shapes concerned and the measured and required millimetres on paper. Run it from `S/📦️packages/🟦️typescript`. */
import { join } from "node:path";
import { parseArgs } from "node:util";
import { S, readDoc } from "../MR/lib.ts";

const { measureSuitability, DEFAULT_PRINT_PROFILE, printProfile } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" } } });
const profile = typeof DEFAULT_PRINT_PROFILE === "string" ? printProfile(DEFAULT_PRINT_PROFILE) : DEFAULT_PRINT_PROFILE;
for (const arg of positionals) {
  const asset = readDoc(arg).doc.asset;
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.status !== "authored" || (values.scales !== undefined && !values.scales.split(",").includes(key))) continue;
    const suitability = measureSuitability(asset, representation, Number(key), profile);
    console.log(`${asset.id} 1:${key} ${suitability.suitable ? "prints" : "DOES NOT PRINT"} (${profile.id})`);
    for (const violation of suitability.violations ?? []) console.log(`   ${violation.check} ${violation.shapes.join(" | ")}: ${violation.measuredMm.toFixed(3)} mm < ${violation.requiredMm} mm`);
  }
}
