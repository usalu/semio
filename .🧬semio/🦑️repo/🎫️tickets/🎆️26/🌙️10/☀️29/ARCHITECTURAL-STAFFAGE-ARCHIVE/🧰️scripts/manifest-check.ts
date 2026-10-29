import { loadCatalogFromFiles } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateManifest } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const catalog = await loadCatalogFromFiles();
const issues = validateManifest(catalog.manifest, catalog.assets);
console.log(`[DEBUG] assets ${catalog.assets.length}, manifest issues ${issues.length}`);
for (const issue of issues.slice(0, 20)) console.log(`[DEBUG] ${issue.code} ${issue.path}: ${issue.message.en}`);
const counts: Record<string, number> = {};
for (const asset of catalog.assets) counts[asset.category] = (counts[asset.category] ?? 0) + 1;
console.log("[DEBUG] per category", JSON.stringify(counts), "poses(named)", catalog.assets.reduce((n, a) => n + a.poses.filter((p) => !p.key).length, 0), "clips", catalog.assets.reduce((n, a) => n + a.clips.length, 0));
