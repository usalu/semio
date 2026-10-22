import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
console.log(JSON.stringify(["typescript-source", "graphql", "json", "protobuf"].map((k) => canonicalPrimaryFilenameForKind(k, loadCatalogTaxonomy()))));
