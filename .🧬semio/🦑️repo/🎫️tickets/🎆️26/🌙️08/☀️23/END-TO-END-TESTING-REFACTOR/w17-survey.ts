import { surveyUnmanagedTests } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const rows = surveyUnmanagedTests("/Users/ueli/Documents/semio");
for (const r of rows) console.log(r.area, "|", r.path, "|", r.framework);
