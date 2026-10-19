const root = "/Users/ueli/Documents/semio";
const L = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const {loadCatalogTaxonomy, semanticDirectoryKindId} = await import(`${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const t = loadCatalogTaxonomy();
for (const rel of process.argv.slice(2)) {
  let parent: string | undefined, anc: string[] = [], out: string[] = [];
  for (const seg of rel.split("/")) {
    const kind = semanticDirectoryKindId(seg, t, { parentKindId: parent, ancestorKindIds: [...anc] });
    out.push(`${seg}=${kind}`);
    if (kind) { if (parent) anc = [parent, ...anc]; parent = kind; }
  }
  console.log(out.join(" / "));
}
