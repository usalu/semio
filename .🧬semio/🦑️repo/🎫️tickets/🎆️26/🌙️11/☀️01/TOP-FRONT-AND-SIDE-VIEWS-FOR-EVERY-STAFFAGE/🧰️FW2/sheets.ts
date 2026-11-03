const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = process.argv[2]!;
const { runPreview } = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { resolveRepresentation, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const jobs: [string, number][] = [["retractable-bollard", 500], ["street-lamp", 200], ["gas-boiler", 200], ["bookshelf", 50], ["curtain-wall-bay", 50], ["masonry-wall-with-lintel", 50]];
for (const [asset, scale] of jobs) {
  try {
    const files = await runPreview(["--asset", asset, "--scale", String(scale), "--views", "--out", `${OUT}/${asset}`]);
    console.log(asset, scale, files.length);
  } catch (error) {
    console.log(asset, scale, "ERROR", String(error).slice(0, 300));
  }
}
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const hob = catalog.assets.find((a: { id: string }) => a.id === "gas-hob");
const r = resolveRepresentation(hob, { denominator: 200 }, DEFAULT_PRINT_PROFILE);
console.log("gas-hob 1:200", JSON.stringify(r, (k, v) => (k === "representation" || k === "asset" ? undefined : v), 1).slice(0, 4000));
