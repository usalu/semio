import { readFileSync, existsSync } from "node:fs";
import { SchemaCatalog, SourceStore, exportsOf, facetOf, leafPath, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { declares, declaresParser } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📖️declared-types/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root, ["✏️s", "🌎️hub"]);
const sources = new SourceStore();
let leaves = 0, split = 0, mixed = 0;
const rows: string[] = [];
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const document = readDocument(root, path);
  if (!document) continue;
  const target = leafPath(root, module, facetOf(module, path), "🟦️typescript");
  if (!existsSync(target)) continue;
  leaves++;
  const own = sources.read(target);
  const tree = sources.declarations(target);
  if (tree.length === own.length) continue;
  split++;
  let inChild = 0, parsersInLeaf = 0, parsersAny = 0;
  for (const [name] of exportsOf(document)) {
    const typeInLeaf = declares("🟦️typescript", own, name);
    if (declaresParser(own, name)) parsersInLeaf++;
    if (!typeInLeaf && declares("🟦️typescript", tree, name)) { inChild++; if (declaresParser(own, name)) mixed++; }
  }
  rows.push(`${target.slice(root.length + 1)}\tleafLines=${own.split("\n").length}\ttypesInChild=${inChild}\tparsersInLeaf=${parsersInLeaf}`);
}
console.log(`leaves=${leaves} split=${split} typedChildParsersInLeaf=${mixed}`);
console.log(rows.join("\n"));
