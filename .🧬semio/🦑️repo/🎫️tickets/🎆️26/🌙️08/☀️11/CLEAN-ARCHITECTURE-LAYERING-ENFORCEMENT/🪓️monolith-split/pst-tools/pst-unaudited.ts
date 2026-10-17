import { existsSync, readFileSync } from "node:fs";
import { SchemaCatalog, exportsOf, facetOf, leafPath, portable, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { declaresParser, parserRange } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📖️declared-types/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root);
let exportsTotal = 0, withParser = 0, audited = 0, unaudited = 0, noLeaf = 0, leavesUnaudited = new Set<string>();
const seen = new Set<string>();
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const doc = readDocument(root, path); if (!doc) continue;
  const leaf = leafPath(root, module, facetOf(module, path), "🟦️typescript");
  const rel = portable(root, leaf);
  const src = existsSync(leaf) ? readFileSync(leaf, "utf8").replace(/\r\n?/g, "\n") : undefined;
  for (const name of exportsOf(doc).keys()) {
    const key = rel + "\0" + name; if (seen.has(key)) continue; seen.add(key);
    exportsTotal++;
    if (src === undefined) { noLeaf++; continue; }
    if (!declaresParser(src, name)) continue;
    withParser++;
    const region = src.indexOf("//#region 🚪️Parsers");
    const r = parserRange(src, name);
    if (region >= 0 && r && r[0] > region) audited++; else { unaudited++; leavesUnaudited.add(rel); }
  }
}
console.log(JSON.stringify({ exportsTotal, noLeaf, withParser, audited, unaudited, leavesWithUnaudited: leavesUnaudited.size }));
