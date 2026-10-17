import { projectParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const [under, reasons] = process.argv.slice(2);
const r = projectParsers({ root: "/Users/ueli/Documents/semio", mode: "report", under: under ?? "✏️s", formats: new Set(["🟦️typescript"]), closing: true, replace: new Set(), restate: new Set((reasons ?? "stale").split(",") as never[]) });
console.log(JSON.stringify(r.added));
for (const row of r.refused) console.log(row);
