// 📊 Census of the live generation: fragment sizes, token occurrences and the fully expanded size (read-only).
import { readFileSync, readdirSync } from "node:fs";
import { brotliDecompressSync } from "node:zlib";
import { join } from "node:path";
const G = process.env.GEN ?? "/Users/ueli/Documents/semio/.cache/b/g/m";
const cur = JSON.parse(readFileSync(join(G, "generations/current.json"), "utf8"));
const man = JSON.parse(readFileSync(join(G, "generations/m", cur.manifest + ".json"), "utf8"));
const owners = Object.entries(man.owners);
let jsonBytes = 0, compBytes = 0, occ = 0, slots = 0, projects = 0, maxFrag = 0;
const sizes = new Map();
const closureSize = key => { let s = sizes.get(key); if (s === undefined) { try { const t = readFileSync(join(G, "build-input-closures", key.slice(21) + ".json"), "utf8"); s = JSON.parse(t).files.length; } catch { try { JSON.parse(readFileSync(join(G, "build-lock-closures", key.slice(18) + ".json"), "utf8")); s = 2; } catch { s = 0; } } sizes.set(key, s); } return s; };
const re = /SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}/g;
const t0 = performance.now(), m0 = process.memoryUsage();
const perOwner = [];
for (const [name, row] of owners) {
  const comp = readFileSync(join(G, "generations/o", row.fragment + ".json.br")); compBytes += comp.length;
  const text = brotliDecompressSync(comp).toString("utf8"); jsonBytes += text.length; maxFrag = Math.max(maxFrag, text.length);
  let o = 0, sl = 0; for (const m of text.matchAll(re)) { o++; sl += closureSize(m[0]); }
  occ += o; slots += sl; projects++; perOwner.push([name, text.length, o, sl]);
}
perOwner.sort((a, b) => b[3] - a[3]);
console.log(JSON.stringify({ projects, compMB: +(compBytes / 1e6).toFixed(1), jsonMB: +(jsonBytes / 1e6).toFixed(1), maxFragKB: Math.round(maxFrag / 1e3), tokenOccurrences: occ, distinctClosures: sizes.size, expandedSlots: slots, expandedSlotMB: Math.round(slots * 8 / 1e6), ms: Math.round(performance.now() - t0), top: perOwner.slice(0, 5) }, null, 1));
