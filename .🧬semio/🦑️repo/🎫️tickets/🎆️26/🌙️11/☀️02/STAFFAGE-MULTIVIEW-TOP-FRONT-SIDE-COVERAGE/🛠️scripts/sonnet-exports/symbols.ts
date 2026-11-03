import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const c = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
for (const a of c.assets) for (const s of a.symbols) { const g = s.geometry as any; if (s.semantics !== "true-extent") console.log(a.id, s.id, s.semantics, s.denominators.join(","), g.elevation?.length, g.front?.length, g.plan?.length); }
