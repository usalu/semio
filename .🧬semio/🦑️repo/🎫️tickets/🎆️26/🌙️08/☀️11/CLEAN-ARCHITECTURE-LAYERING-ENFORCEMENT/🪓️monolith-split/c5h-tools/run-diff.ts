import { SchemaCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { auditLeaves, projectParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const mode = process.argv[2] as "report" | "apply";
const module = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const catalog = new SchemaCatalog(root, ["✏️s/🔌️plugins/🏛️architect", "🧰️framework"]);
const report = projectParsers({ root, mode, under: module, catalog, formats: new Set(["🟦️typescript"]), closing: true, replace: new Set(), restate: new Set(), distribute: [`${module}/🔺️diff`] });
console.log(JSON.stringify({ ...report, touched: report.touched.map((p) => p.slice(module.length)) }, null, 1));
if (mode === "apply") {
  const audit = auditLeaves({ root, under: module, catalog });
  console.log("audit", audit.drift.length, audit.drift.filter((d) => !d.path.includes("🧬️schema/🟦️.ts")).map((d) => `${d.reason} ${d.name} ${d.path.slice(module.length)}`).slice(0, 20), audit.helperless);
}
