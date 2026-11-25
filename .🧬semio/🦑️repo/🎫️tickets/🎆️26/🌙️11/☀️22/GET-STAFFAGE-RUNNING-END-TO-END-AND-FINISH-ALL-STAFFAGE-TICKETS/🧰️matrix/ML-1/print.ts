/** 🖨️ Whether every authored scale of asset documents and both ends of its range print under the default print profile, with the violations where one does not: `bun print.ts <id|file.json>…` (exit 1 when a scale does not print). */
import { join } from "node:path";
import { S, catalogFile } from "./files.ts";

const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
let bad = 0;
for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(arg.endsWith(".json") ? arg : catalogFile(arg)).json()).asset;
  const lines: string[] = [];
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.geometry === undefined) continue;
    const range = representation.validRange;
    for (const denominator of [...new Set([Number(key), range.minDenominator, range.maxDenominator])]) {
      const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
      if (resolution.kind === "authored") continue;
      bad++;
      lines.push(`  1:${denominator} (band ${key}) ${resolution.kind}: ${JSON.stringify(resolution.suitability?.violations ?? resolution).slice(0, 500)}`);
    }
  }
  console.log(`${asset.id} print: ${lines.length === 0 ? "every authored scale and range end prints" : `${lines.length} do not print`}`);
  for (const line of lines) console.log(line);
}
process.exit(bad === 0 ? 0 : 1);
