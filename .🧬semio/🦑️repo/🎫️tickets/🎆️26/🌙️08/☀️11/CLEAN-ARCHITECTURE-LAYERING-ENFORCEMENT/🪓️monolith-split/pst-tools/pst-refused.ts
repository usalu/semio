import { projectParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const under = process.argv[2] ?? "✏️s";
const r = projectParsers({ root: "/Users/ueli/Documents/semio", mode: "report", under, formats: new Set(["🟦️typescript"]), closing: true, replace: new Set(), restate: new Set() });
for (const row of r.refused) console.log(row);
