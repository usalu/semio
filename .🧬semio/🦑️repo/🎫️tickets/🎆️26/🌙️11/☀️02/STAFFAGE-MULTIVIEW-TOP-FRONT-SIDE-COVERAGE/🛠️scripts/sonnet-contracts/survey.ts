import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const out: Record<string, unknown> = {};
let unavailable = 0;
let symbols = 0;
let symbolsNoFront = 0;
let symbolsNoPlan = 0;
const byUnits: Record<string, number> = {};
const unavailableByAsset: Record<string, number[]> = {};
for (const asset of catalog.assets) {
  for (const [key, rep] of Object.entries(asset.representations)) {
    if (rep.status === "unavailable") {
      unavailable++;
      (unavailableByAsset[asset.id] ??= []).push(Number(key));
    }
  }
  for (const symbol of asset.symbols) {
    symbols++;
    byUnits[`${symbol.semantics}/${symbol.units}`] = (byUnits[`${symbol.semantics}/${symbol.units}`] ?? 0) + 1;
    if (!symbol.geometry.front || symbol.geometry.front.length === 0) symbolsNoFront++;
    if (!symbol.geometry.plan || symbol.geometry.plan.length === 0) symbolsNoPlan++;
  }
}
out.assets = catalog.assets.length;
out.unavailable = unavailable;
out.symbols = symbols;
out.symbolsNoFront = symbolsNoFront;
out.symbolsNoPlan = symbolsNoPlan;
out.byUnits = byUnits;
out.unavailableByAsset = unavailableByAsset;
console.log(JSON.stringify(out, null, 1));
