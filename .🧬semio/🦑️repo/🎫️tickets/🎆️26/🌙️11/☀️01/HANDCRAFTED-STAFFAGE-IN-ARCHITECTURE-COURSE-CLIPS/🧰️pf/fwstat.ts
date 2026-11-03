import { readFile } from "node:fs/promises";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const [manifest, out, catalog] = process.argv.slice(2);
const m = JSON.parse(await readFile(manifest!, "utf8"));
await exportCast(m, Object.keys(m.casts), out!, { language: "en", catalog: catalog! });
const fw = (globalThis as any).__fw as [number, string, number][];
const seen = new Map<string, number>();
let total = 0, dup = 0, dupTime = 0;
for (const [, k, t] of fw) { total += t; if (seen.has(k)) { dup++; dupTime += t; } else seen.set(k, t); }
console.log("calls", fw.length, "unique", seen.size, "total s", (total / 1000).toFixed(1), "dup s", (dupTime / 1000).toFixed(1));
const sorted = [...fw].sort((a, b) => b[2] - a[2]).slice(0, 8).map((x) => [x[0], Math.round(x[2])]);
console.log(JSON.stringify(sorted));
const buckets = [0, 10, 100, 1000, 10000, 100000].map((lo, i, a) => { const hi = a[i + 1] ?? 1e9; const xs = fw.filter((x) => x[2] >= lo && x[2] < hi); return `${lo}-${hi}ms n=${xs.length} s=${(xs.reduce((s, x) => s + x[2], 0) / 1000).toFixed(1)}`; });
console.log(buckets.join(" | "));
