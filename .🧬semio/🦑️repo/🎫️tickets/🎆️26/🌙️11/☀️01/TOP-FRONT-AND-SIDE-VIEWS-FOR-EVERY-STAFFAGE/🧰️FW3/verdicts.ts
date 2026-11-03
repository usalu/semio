import { writeFileSync } from "node:fs";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { SCALE_DENOMINATORS } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const out: Record<string, unknown> = {};
for (const asset of catalog.assets)
  for (const d of SCALE_DENOMINATORS) {
    const rep = asset.representations[`${d}`];
    if (rep === undefined || rep.status !== "authored") continue;
    try {
      const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
      out[`${asset.id}/${d}`] = { suitable: s.suitable, minFeature: Math.round(s.minFeatureMm * 1e4) / 1e4, feature: s.violations.filter((v) => v.check === "feature").map((v) => `${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`) };
    } catch (e) { out[`${asset.id}/${d}`] = { error: String(e) }; }
  }
writeFileSync(process.env.FW_LOG!, JSON.stringify(out));
