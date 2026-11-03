// 🔢 Read-only tallies of manifest rows: unavailable scale bands, analytics sets and export sets.
import { readFileSync } from "node:fs";

const [manifestPath] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const tally = (items, key) => {
  const out = {};
  for (const i of items) {
    const k = key(i);
    out[k] = (out[k] ?? 0) + 1;
  }
  return out;
};
const rows = manifest.entries;
console.log(`[unavailableByScale] ${JSON.stringify(tally(["50", "100", "200", "500"].flatMap((s) => rows.filter((r) => r.representations[s] === "unavailable").map(() => s)), (s) => s))}`);
console.log(`[rowsWithAnyUnavailable] ${rows.filter((r) => Object.values(r.representations).includes("unavailable")).length}`);
console.log(`[analyticsSets] ${JSON.stringify(tally(rows, (r) => r.analytics.join("+")))}`);
console.log(`[exportSets] ${JSON.stringify(tally(rows, (r) => r.exports.join("+")))}`);
console.log(`[symbolCountDist] ${JSON.stringify(tally(rows, (r) => r.symbols.length))}`);
console.log(`[posesCountDist] ${JSON.stringify(tally(rows, (r) => r.poses.length))}`);
console.log(`[clipsCountDist] ${JSON.stringify(tally(rows, (r) => r.clips.length))}`);
console.log(`[rowsWithTransport] ${rows.filter((r) => r.analytics.includes("transport")).length}`);
console.log(`[rowsWithNoStructural] ${rows.filter((r) => !r.analytics.includes("structural")).length}`);
console.log(`[rowsWithNoThermal] ${rows.filter((r) => !r.analytics.includes("thermal")).length}`);
console.log(`[unavailableRows] ${rows.filter((r) => Object.values(r.representations).includes("unavailable")).map((r) => r.id + "(" + Object.entries(r.representations).filter(([, v]) => v === "unavailable").map(([k]) => k).join("/") + ")").join(", ")}`);
