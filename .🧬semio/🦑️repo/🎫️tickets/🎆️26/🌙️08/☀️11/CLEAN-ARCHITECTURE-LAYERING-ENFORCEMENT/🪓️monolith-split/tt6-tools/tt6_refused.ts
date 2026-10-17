/** 📋️ Prints every refused position of `schema parsers report` (the command truncates at 60). usage: bun tt6_refused.ts <under> */
import { projectParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const under = process.argv[2] ?? "✏️s";
const report = projectParsers({ root: "/Users/ueli/Documents/semio", mode: "report", under, formats: new Set(["🔗️graphql", "🛰️protobuf", "🟦️typescript"]), closing: false, replace: new Set(), restate: new Set() });
for (const row of report.refused) console.log("REFUSED " + row);
console.error(`refused=${report.refused.length}`);
