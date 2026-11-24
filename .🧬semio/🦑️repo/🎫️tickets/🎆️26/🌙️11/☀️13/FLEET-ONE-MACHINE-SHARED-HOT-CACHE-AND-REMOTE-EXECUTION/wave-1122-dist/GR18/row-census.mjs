import { readdirSync, readFileSync } from "node:fs"; import { brotliDecompressSync } from "node:zlib"; import { dirname } from "node:path";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/";
const { libraryBootstrap } = await import(INF + "🟨️.mjs"); await libraryBootstrap;
const { createClosureStoreV1 } = await import(INF + "🧭️script-inputs/💾️entries/🟨️.mjs");
const dir = process.env.STORE, store = createClosureStoreV1({ directory: dir, readOnly: true });
for (const name of readdirSync(dir + "/e")) {
  const rec = JSON.parse(brotliDecompressSync(readFileSync(dir + "/e/" + name)).toString()); const stored = await store.load(rec.id); if (!stored) continue;
  const kinds = {}, selfs = new Set(), parents = new Set(); let rows = 0; const seenDict = new Map();
  for (const digest of stored.digests) { const l = store.layerListingSync(digest); if (!l) continue; let ids = seenDict.get(l.dictionary); if (!ids) seenDict.set(l.dictionary, ids = new Set()); for (const id of l.ids) { if (ids.has(id)) continue; ids.add(id); const row = l.rowSync(id); rows++; kinds[row.kind] = (kinds[row.kind] ?? 0) + 1; if (["lstat","stat","exists","fileExists","directoryExists"].includes(row.kind)) { selfs.add(row.path); parents.add(dirname(row.path)); } } }
  console.log(stored.digest?.slice(0, 8) ?? name.slice(0, 8), rec.recipe?.name ?? rec.recipe?.fn ?? rec.recipe?.port, "layers", stored.digests.length, "rows", rows, "selfPaths", selfs.size, "parentPaths", parents.size, JSON.stringify(kinds));
}
process.exit(0);
