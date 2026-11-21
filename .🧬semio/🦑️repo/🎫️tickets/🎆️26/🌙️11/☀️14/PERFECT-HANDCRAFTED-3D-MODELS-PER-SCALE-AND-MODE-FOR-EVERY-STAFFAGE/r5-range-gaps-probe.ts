/** 🔭️ R01 probe: for every asset with a coverage hole (or the assets named), closes the hole by widening the border band back to its promised end and prints the print-profile violations `validateRanges` then reports at the widened end (what has to be redrawn). Reads the catalog file live.
 * Usage: `bun r5-range-gaps-probe.ts [<category>/<slug> ...] [--json]` */
import { uncoveredDenominators, validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { catalogEntries, loadAsset, type Json } from "./volumes-lib.ts";

/** 🩹️ The asset with every hole closed by widening the band that borders it. */
export function widened(asset: Json): Json {
  const out = structuredClone(asset);
  for (const gap of uncoveredDenominators(out)) {
    const range = out.representations[`${gap.border}`].validRange;
    if (gap.from >= range.maxDenominator) range.maxDenominator = gap.to;
    else range.minDenominator = Math.min(range.minDenominator, gap.from);
  }
  return out;
}

if (import.meta.main) {
  const named = process.argv.slice(2).filter((a) => !a.startsWith("--"));
  const rows: Json[] = [];
  for (const entry of catalogEntries()) {
    if (named.length > 0 && !named.includes(`${entry.category}/${entry.slug}`)) continue;
    const asset = loadAsset(entry.path);
    const gaps = uncoveredDenominators(asset);
    if (gaps.length === 0 && named.length === 0) continue;
    const fixed = widened(asset);
    const issues = (validateRanges(fixed, DEFAULT_PRINT_PROFILE) as readonly Json[]).filter((i) => i.code.startsWith("range-end") || i.code.startsWith("nominal"));
    rows.push({ id: `${entry.category}/${entry.slug}`, gaps: gaps.map((g) => [g.from, g.to, g.border]), issues: issues.map((i) => ({ code: i.code, band: i.data.denominator, at: i.data.scale, shapes: i.data.shapes, measuredMm: i.data.measuredMm, requiredMm: i.data.requiredMm })) });
    if (!process.argv.includes("--json")) {
      console.log(`## ${entry.category}/${entry.slug} holes ${gaps.map((g) => `${g.from}-${g.to}@${g.border}`).join(",")}: ${issues.length} violation(s) when widened`);
      for (const i of issues) console.log(`  ${i.code} band 1:${i.data.denominator} at 1:${i.data.scale} ${i.data.shapes.join("+")} ${i.data.measuredMm} < ${i.data.requiredMm}`);
    }
  }
  if (process.argv.includes("--json")) console.log(JSON.stringify(rows, null, 1));
}
