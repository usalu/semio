/** 🧪️ R01: the print-profile violations `validateRanges` reports for an asset with the given valid ranges (catalog file read live, optionally a candidate document), without writing anything; only the named scales are measured.
 * Usage: `bun r5-range-gaps-try.ts <category>/<slug> <scale>=<min>-<max> [...] [--file <asset document json>]` */
import { validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset, resolveAsset, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const file = args.includes("--file") ? args[args.indexOf("--file") + 1] : undefined;
const asset: Json = structuredClone(loadAsset(file ?? resolveAsset(args[0]!).path));
const wanted = new Set<string>();
for (const arg of args.filter((a) => /^\d+=/.test(a))) {
  const [scale, range] = arg.split("=");
  const [low, high] = range!.split("-").map(Number);
  asset.representations[scale!].validRange = { minDenominator: low, maxDenominator: high };
  wanted.add(scale!);
}
for (const key of Object.keys(asset.representations)) if (!wanted.has(key)) asset.representations[key] = { ...asset.representations[key], status: "skip" };
const issues = validateRanges(asset, DEFAULT_PRINT_PROFILE) as readonly Json[];
for (const i of issues) console.log(`${i.code} band 1:${i.data.denominator} at 1:${i.data.scale} ${i.data.shapes.join("+")} ${i.data.measuredMm} < ${i.data.requiredMm}`);
console.log(`${issues.length} violation(s)`);
