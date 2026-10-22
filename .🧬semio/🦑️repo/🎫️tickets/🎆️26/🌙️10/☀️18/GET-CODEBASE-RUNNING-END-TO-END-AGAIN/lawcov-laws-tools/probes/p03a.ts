import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy, loadTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadCatalogTaxonomy();
console.log(JSON.stringify({ g: canonicalPrimaryFilenameForKind("grammar-semio", t), p: canonicalPrimaryFilenameForKind("protocol-semio", t), ts: canonicalPrimaryFilenameForKind(t.componentFileKinds["🟦️typescript"]!, t), ex: loadTaxonomy().exampleAssetsDirName, areas: loadTaxonomy().areas }));
