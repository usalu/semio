import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { SchemaCatalog, exportsOf, facetOf, leafPath, portable, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { parserRange } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📖️declared-types/🟦️.ts";
import { loadUnowned, unownedKey } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧱️unowned/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root);
const listed = new Set(loadUnowned().parsers.map((r) => unownedKey(r.path, r.name)));
const rows: { name: string; path: string }[] = [];
const seen = new Set<string>();
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const doc = readDocument(root, path); if (!doc) continue;
  const leaf = leafPath(root, module, facetOf(module, path), "🟦️typescript");
  if (!existsSync(leaf)) continue;
  const src = readFileSync(leaf, "utf8").replace(/\r\n?/g, "\n");
  const region = src.indexOf("//#region 🚪️Parsers"); if (region < 0) continue;
  const rel = portable(root, leaf);
  for (const name of exportsOf(doc).keys()) {
    const r = parserRange(src, name);
    if (!r || r[0] < region) continue;
    const key = unownedKey(rel, name); if (listed.has(key) || seen.has(key)) continue; seen.add(key);
    rows.push({ name, path: rel });
  }
}
writeFileSync(process.argv[2]!, JSON.stringify(rows));
console.log(rows.length, "generated parsers");
