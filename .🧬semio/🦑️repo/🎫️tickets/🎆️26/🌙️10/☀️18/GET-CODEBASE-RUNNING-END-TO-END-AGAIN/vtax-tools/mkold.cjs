// makes orig/taxonomy-authority.abs.ts: the pre-split authority module with every relative import specifier made absolute, so the old and the new validateTaxonomy can run side by side
const fs = require("fs"), path = require("path");
const DIR = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority";
const text = fs.readFileSync(path.join(__dirname, "orig/taxonomy-authority.ts"), "utf8");
const out = text.replace(/^(import \{[^}]*\} from )"(\.[^"]+)";/gmu, (m, head, spec) => `${head}"${path.resolve(DIR, spec)}";`);
fs.writeFileSync(path.join(__dirname, "orig/taxonomy-authority.abs.ts"), out);
console.log("relative imports left:", (out.match(/from "\.\.?\//g) || []).length);
