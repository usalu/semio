import { existsSync, readFileSync } from "node:fs";
import { SchemaCatalog, exportsOf, facetOf, leafPath, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root);
const stats: Record<string, number> = {};
const bump = (k: string, n = 1) => (stats[k] = (stats[k] ?? 0) + n);
const walk = (node: any, where: string) => {
  if (!node || typeof node !== "object") return;
  if (Array.isArray(node)) { node.forEach((n) => walk(n, where)); return; }
  if (node.type === "object" || node.properties) {
    const ap = node.additionalProperties;
    const hasProps = node.properties && Object.keys(node.properties).length > 0;
    const kind = ap === false ? "false" : ap === true ? "true" : ap && typeof ap === "object" ? "schema" : "absent";
    bump(`obj:${hasProps ? "props" : "noprops"}:ap=${kind}`);
    if (node.allOf && ap === false) bump("closed+allOf");
    if (node.patternProperties) bump("patternProperties");
  }
  for (const [k, v] of Object.entries(node)) if (k !== "enum" && k !== "const" && k !== "default" && k !== "examples") walk(v, where);
};
let docs = 0, withLeaf = 0, closedExports = 0, exportCount = 0;
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const doc = readDocument(root, path); if (!doc) continue; docs++;
  const leaf = leafPath(root, module, facetOf(module, path), "🟦️typescript");
  const text = existsSync(leaf) ? readFileSync(leaf, "utf8") : "";
  if (text) withLeaf++;
  walk(doc, path);
  for (const [name, def] of exportsOf(doc)) { exportCount++; if (text.includes(`function parse${name}(`)) bump("exports-with-parser"); }
}
console.log(JSON.stringify({ docs, withLeaf, exportCount, ...stats }, null, 1));
