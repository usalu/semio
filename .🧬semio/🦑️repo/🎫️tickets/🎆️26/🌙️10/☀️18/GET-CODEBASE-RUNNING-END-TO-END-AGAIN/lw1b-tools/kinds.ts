import { loadCatalogTaxonomy, semanticDirectoryKindId } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadCatalogTaxonomy();
for (const n of ["🧱️norm-definition-ownership", "🔬️cad-presence-retirement", "🔮️oracle-source-ownership", "🚀️launch-catalog", "🔬️no-such-case-x"]) {
  console.log(n, "tests:", semanticDirectoryKindId(n, t, { parentKindId: "tests" }), "fixtures:", semanticDirectoryKindId(n, t, { parentKindId: "fixtures" }));
}
