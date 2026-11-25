/** 🖨️ Whether every authored band of an asset prints under the default print profile, at its key and at both ends of its valid range: `bun prints.ts <id|file.json>…` prints one line per asset (`every authored scale prints` or the count of refusals) and one line per refused denominator with its violations. Run it from `T`.
 * @see ../MF/print.ts — the same check of batch MF */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset;
  const lines: string[] = [];
  for (const [key, rep] of Object.entries(asset.representations) as [string, any][]) {
    if (rep.geometry === undefined) continue;
    const ends = new Set<number>([Number(key), rep.validRange?.minDenominator ?? Number(key), rep.validRange?.maxDenominator ?? Number(key)]);
    for (const denominator of [...ends].sort((a, b) => a - b)) {
      let line: string;
      try {
        const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
        const violations = resolution.suitability?.violations ?? [];
        if (resolution.kind !== "unsuitable" && resolution.kind !== "unavailable" && violations.length === 0) continue;
        line = `   1:${denominator} (band ${key}) ${resolution.kind} ${JSON.stringify(violations).slice(0, 600)}`;
      } catch (error) {
        line = `   1:${denominator} (band ${key}) throws ${(error as Error).message.slice(0, 300)}`;
      }
      lines.push(line);
    }
  }
  console.log(`${asset.id}\t${lines.length === 0 ? "every authored scale prints" : `${lines.length} refusals`}`);
  for (const line of lines) console.log(line);
}
