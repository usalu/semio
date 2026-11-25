import { readdirSync, readFileSync } from "node:fs"; import { brotliDecompressSync } from "node:zlib";
const dir = process.env.STORE, sample = Number(process.env.SAMPLE ?? 40), byOwner = new Map(), shared = [], fns = {};
for (const name of readdirSync(dir + "/e")) {
  let rec; try { rec = JSON.parse(brotliDecompressSync(readFileSync(dir + "/e/" + name)).toString()); } catch { continue; }
  const layers = rec.layers ?? [], fn = rec.recipe?.fn ?? "?", owner = rec.recipe?.projectRoot;
  fns[fn] = (fns[fn] ?? 0) + 1;
  const row = { fn, layers, rows: (rec.rows ?? []).length }; (globalThis.byFn ??= {})[fn] ??= { entries: 0, refs: 0, max: 0, rows: 0 }; const f = globalThis.byFn[fn]; f.entries++; f.refs += layers.length; f.rows += row.rows; if (layers.length > f.max) f.max = layers.length;
  if (owner === undefined) shared.push(row); else { if (!byOwner.has(owner)) byOwner.set(owner, []); byOwner.get(owner).push(row); }
}
const owners = [...byOwner.keys()].sort(), step = Math.max(1, Math.floor(owners.length / sample)), picked = owners.filter((_, at) => at % step === 0).slice(0, sample);
let sumOwn = 0, rowsOwn = 0; const unionOwn = new Set(), per = [];
for (const owner of picked) { let n = 0; for (const e of byOwner.get(owner)) { n += e.layers.length; rowsOwn += e.rows; for (const d of e.layers) unionOwn.add(d); } sumOwn += n; per.push(n); }
let sumShared = 0; const unionShared = new Set(); for (const e of shared) { sumShared += e.layers.length; for (const d of e.layers) unionShared.add(d); }
const all = new Set([...unionOwn, ...unionShared]), inShared = [...unionOwn].filter(d => unionShared.has(d)).length;
per.sort((a, b) => a - b);
console.log(JSON.stringify({ byFn: globalThis.byFn, store: dir, entries: Object.values(fns).reduce((a, b) => a + b, 0), recipes: fns, ownersWithEntries: owners.length, sampled: picked.length, ownerEntries: picked.reduce((s, o) => s + byOwner.get(o).length, 0), ownerLayerRefsSum: sumOwn, ownerLayerUnion: unionOwn.size, ownerOwnRows: rowsOwn, ownerLayersAlsoInSharedEntries: inShared, perOwnerLayerRefs: { min: per[0], median: per[per.length >> 1], max: per.at(-1) }, sharedEntries: shared.length, sharedLayerRefsSum: sumShared, sharedLayerUnion: unionShared.size, unionAll: all.size }, null, 1));
