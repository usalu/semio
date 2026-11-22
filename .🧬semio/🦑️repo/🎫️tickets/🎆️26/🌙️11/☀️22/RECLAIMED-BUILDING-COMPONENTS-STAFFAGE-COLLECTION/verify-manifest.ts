/** 📋️ Coordinator proof: loads the whole catalog and prints every manifest inconsistency, marking those that concern a reclaimed asset.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts — `validateManifest` */
import { validateManifest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const issues = validateManifest(catalog.manifest, catalog.assets);
const reclaimed = catalog.manifest.entries.map((entry, index) => ({ entry, index })).filter(({ entry }) => entry.id.startsWith("reclaimed-"));
const ours = issues.filter((issue) => JSON.stringify(issue).includes("reclaimed-") || reclaimed.some(({ index }) => issue.path.startsWith(`entries[${index}]`)));
console.log(`assets ${catalog.assets.length}, entries ${catalog.manifest.entries.length}, reclaimed entries ${reclaimed.length}, manifest issues ${issues.length}, of them reclaimed ${ours.length}`);
for (const issue of ours) console.log(`RECLAIMED ${issue.code} ${issue.path}: ${issue.message.en}`);
for (const issue of issues.filter((issue) => !ours.includes(issue)).slice(0, 12)) console.log(`other ${issue.code} ${issue.path}: ${issue.message.en}`);
