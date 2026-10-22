import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadTaxonomy();
console.log(JSON.stringify({ marker: canonicalFilenameForKind(t.windowEmptyFacetFileKindId, t), comps: Object.fromEntries(Object.entries(t.componentFileKinds).map(([k, v]) => [k, canonicalPrimaryFilenameForKind(v as string, t)])) }));
