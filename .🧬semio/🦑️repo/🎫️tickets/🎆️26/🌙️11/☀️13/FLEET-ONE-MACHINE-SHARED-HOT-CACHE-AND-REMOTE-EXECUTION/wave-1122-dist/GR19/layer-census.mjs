import { readdirSync, readFileSync } from "node:fs"; import { brotliDecompressSync } from "node:zlib";
const dir = process.env.STORE, names = readdirSync(dir + "/e"), refs = new Map(), sizes = []; let total = 0;
for (const name of names) { try { const rec = JSON.parse(brotliDecompressSync(readFileSync(dir + "/e/" + name)).toString()); const n = rec.layers.length; total += n; sizes.push([n, rec.recipe?.fn ?? rec.recipe?.name ?? "?"]); for (const d of rec.layers) refs.set(d, (refs.get(d) ?? 0) + 1); } catch {} }
sizes.sort((a, b) => b[0] - a[0]);
const hist = {}; for (const [n] of sizes) { const k = n === 0 ? "0" : n < 10 ? "1-9" : n < 100 ? "10-99" : n < 1000 ? "100-999" : n < 10000 ? "1k-9k" : "10k+"; hist[k] = (hist[k] ?? 0) + 1; }
const shared = {}; for (const c of refs.values()) { const k = c === 1 ? "1" : c < 10 ? "2-9" : c < 100 ? "10-99" : "100+"; shared[k] = (shared[k] ?? 0) + 1; }
console.log(JSON.stringify({ entries: names.length, refsTotal: total, distinct: refs.size, hist, sharedByRefCount: shared, top: sizes.slice(0, 12) }));
