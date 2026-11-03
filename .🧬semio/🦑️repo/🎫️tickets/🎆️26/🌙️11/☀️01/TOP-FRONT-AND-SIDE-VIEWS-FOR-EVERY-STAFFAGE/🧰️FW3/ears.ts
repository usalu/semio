import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = catalog.assets.find((a) => a.id === "elderly-with-cane")!;
const rep: any = asset.representations["100"];
for (const vp of ["front", "plan"]) for (const s of rep.geometry[vp]) if (/^ear/.test(s.id)) console.log(vp, JSON.stringify(s));
for (const grow of [1, 1.5, 2, 3]) {
  const a: any = structuredClone(asset);
  const r = a.representations["100"];
  for (const vp of ["front", "plan"]) for (const s of r.geometry[vp]) if (/^ear/.test(s.id)) for (const k of ["r", "r0", "r1", "w", "h", "rx", "ry"]) if (typeof s[k] === "number") s[k] *= grow;
  const res = measureSuitability(a, r, 100, DEFAULT_PRINT_PROFILE);
  console.log("grow", grow, res.suitable, res.violations.filter((v) => v.check === "feature").map((v) => `${v.shapes}:${v.measuredMm.toFixed(3)}`).join(" "));
}
