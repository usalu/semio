/** 🖨️ Prints the violations of the print profile of the check CLI per authored scale of one asset document: `bun prints.ts <file.json>` (run it from `S/📦️packages/🟦️typescript`).
 * @see ./solve.ts — the same measurement inside the solver */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
for (const scale of Object.keys(asset.representations)) {
  const result = measureSuitability(asset, asset.representations[scale], Number(scale), DEFAULT_PRINT_PROFILE);
  console.log(`1:${scale} ${result.violations.length} violations`, JSON.stringify(result).slice(0, 1500));
}
