import {loadCatalogTaxonomy} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t=loadCatalogTaxonomy();
console.log("taxonomy ok", Object.keys(t).length);
