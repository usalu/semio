const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { loadCatalogFromFiles } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { validateManifest } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
const c = await loadCatalogFromFiles();
const issues = validateManifest(c.manifest, c.assets);
console.log(issues.length, issues.slice(0, 5).map((i: any) => i.path + " " + i.message.en));
