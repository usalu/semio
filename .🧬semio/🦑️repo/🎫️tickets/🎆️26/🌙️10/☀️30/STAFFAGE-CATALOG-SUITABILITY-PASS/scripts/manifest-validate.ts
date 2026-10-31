import { loadCatalogFromFiles } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateManifest } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const catalog = await loadCatalogFromFiles();
const issues = validateManifest(catalog.manifest, catalog.assets);
for (const entry of catalog.loaded) {
  const category = catalog.manifest.categories.find((c) => c.id === entry.asset.category);
  if (entry.folder !== entry.asset.id) issues.push({ path: entry.path, code: "asset-folder", message: { en: entry.folder, de: entry.folder } });
  if (category && category.folder !== entry.categoryFolder) issues.push({ path: entry.path, code: "category-folder", message: { en: entry.categoryFolder, de: entry.categoryFolder } });
}
console.log("assets", catalog.assets.length, "issues", issues.length);
for (const issue of issues) console.log(issue.code, issue.path, issue.message.en);
