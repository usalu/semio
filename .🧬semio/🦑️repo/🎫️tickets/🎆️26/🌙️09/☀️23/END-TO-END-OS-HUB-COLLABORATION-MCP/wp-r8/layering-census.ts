/** 🏛️ R8 probe: layering ratchet census on the live tree — every file above its baseline, totals by area. Usage: bun layering-census.ts */
import { getWorkspaceRoot, layeringCounts, layeringReferences, loadLayeringBaseline } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const root = getWorkspaceRoot();
const started = Date.now();
const counts = layeringCounts(layeringReferences(root));
const allowed = loadLayeringBaseline(root).allowed;
const over = Object.entries(counts).map(([file, count]) => ({ file, count, allowed: allowed[file] ?? 0 })).filter((row) => row.count > row.allowed).sort((a, b) => b.count - b.allowed - (a.count - a.allowed));
const excess = over.reduce((sum, row) => sum + row.count - row.allowed, 0);
const stale = Object.entries(allowed).filter(([file, value]) => value > 0 && (counts[file] ?? 0) === 0).map(([file]) => file);
console.log(`secs=${Math.round((Date.now() - started) / 1000)} files-with-refs=${Object.keys(counts).length} over-baseline=${over.length} excess-refs=${excess} baseline-entries=${Object.keys(allowed).length} stale-baseline=${stale.length}`);
for (const row of over.slice(0, 40)) console.log(`${row.count - row.allowed}\t${row.count}/${row.allowed}\t${row.file}`);
