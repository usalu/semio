/** Usage: bun merge.ts <out.json> <shard.json>... : merges failures.ts shard outputs (assets, crashed) and recomputes the header from the merged map. */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

type Entry = { failures: number; codes: Record<string, number>; issues: unknown[] };
type Report = { assetsChecked: number; crashed: Record<string, string>; assets: Record<string, Entry> };

const [out, ...inputs] = process.argv.slice(2);
const assets: Record<string, Entry> = {};
const crashed: Record<string, string> = {};
let checked = 0;
for (const input of inputs) {
  const report = JSON.parse(readFileSync(resolve(input), "utf8")) as Report;
  checked += report.assetsChecked;
  Object.assign(crashed, report.crashed);
  for (const [key, entry] of Object.entries(report.assets)) {
    const slot = (assets[key] ??= { failures: 0, codes: {}, issues: [] });
    slot.failures += entry.failures;
    for (const [code, n] of Object.entries(entry.codes)) slot.codes[code] = (slot.codes[code] ?? 0) + n;
    slot.issues.push(...entry.issues);
  }
}
const codeTotals: Record<string, number> = {};
const assetsPerCode: Record<string, number> = {};
for (const entry of Object.values(assets)) for (const [code, n] of Object.entries(entry.codes)) {
  codeTotals[code] = (codeTotals[code] ?? 0) + n;
  assetsPerCode[code] = (assetsPerCode[code] ?? 0) + 1;
}
writeFileSync(resolve(out!), JSON.stringify({ generatedBy: "sonnet-contracts", shard: "merged", assetsChecked: checked, assetsFailing: Object.keys(assets).length, assetsCrashed: Object.keys(crashed).length, codeTotals, assetsPerCode, crashed, assets }, null, 1));
console.log(JSON.stringify({ checked, failing: Object.keys(assets).length, codeTotals }, null, 1));
