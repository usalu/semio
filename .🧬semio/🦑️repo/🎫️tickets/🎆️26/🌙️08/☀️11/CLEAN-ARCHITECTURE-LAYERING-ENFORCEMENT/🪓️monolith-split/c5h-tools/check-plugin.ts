import { SchemaCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { auditLeaves, projectParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const plugin = process.argv[2]!;
const catalog = new SchemaCatalog(root, [plugin, "🧰️framework"]);
const closing = projectParsers({ root, mode: "report", under: plugin, catalog, formats: new Set(["🟦️typescript"]), closing: true, replace: new Set(), restate: new Set() });
const audit = auditLeaves({ root, under: plugin, catalog });
console.log(JSON.stringify({ added: closing.added, pending: closing.touched, refused: closing.refused.slice(0, 5), helperless: audit.helperless, drift: audit.drift.map((d) => `${d.reason} ${d.name} ${d.path}`) }, null, 1));
