/** 🧾️ Validates the staffage manifest against every asset file and prints the issues: `bun manifest-check.ts`. */
import { loadCatalogFromFiles } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateManifest } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const catalog = await loadCatalogFromFiles();
const assets = [...catalog.assets.values()];
const issues = validateManifest(catalog.manifest, assets);
for (const issue of issues) console.log(issue.code, issue.path, issue.message);
console.log(`assets ${assets.length}, manifest issues ${issues.length}`);
