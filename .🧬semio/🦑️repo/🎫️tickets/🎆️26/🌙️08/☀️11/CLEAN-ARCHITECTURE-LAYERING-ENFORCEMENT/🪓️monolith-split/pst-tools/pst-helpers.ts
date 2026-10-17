import { existsSync, readFileSync } from "node:fs";
import { SchemaCatalog, facetOf, leafPath, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";
import { existingHelper } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📖️declared-types/🟦️.ts";
import { PARSER_REGION, helpersBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🚪️guard-helpers/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const catalog = new SchemaCatalog(root);
const seen = new Set<string>();
const c: Record<string, number> = {};
const bump = (k: string) => (c[k] = (c[k] ?? 0) + 1);
const odd: string[] = [];
for (const scope of ["✏️s", "🌎️hub"]) for (const module of catalog.modules(scope)) for (const path of catalog.documents(module)) {
  const leaf = leafPath(root, module, facetOf(module, path), "🟦️typescript");
  if (seen.has(leaf) || !existsSync(leaf)) continue; seen.add(leaf);
  const src = readFileSync(leaf, "utf8").replace(/\r\n?/g, "\n");
  const helper = existingHelper(src);
  const hasRegion = src.includes(PARSER_REGION);
  bump(`helper=${!!helper} region=${hasRegion}`);
  if (helper && hasRegion) {
    const start = src.indexOf(PARSER_REGION);
    const end = src.indexOf("//#endregion 🚪️Parsers", start);
    const block = src.slice(start, end + "//#endregion 🚪️Parsers".length + 1);
    const want = helpersBlock(helper).trimEnd() + "\n";
    if (block === want) bump("block-identical"); else { bump("block-differs"); if (odd.length < 400) odd.push(leaf); }
  }
}
console.log(JSON.stringify(c, null, 1));
await Bun.write("/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/odd-blocks.txt", odd.join("\n") + "\n");
