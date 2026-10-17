// 🧰 bd10: prints the pruned `use` lines a set of files needs from a use pool (the former parent), crate paths rewritten to a crate identifier.
// usage: bun pool-uses.ts <pool.rs> <crateIdent> <file.rs>...
import {readFileSync} from "node:fs";
import {parseRustSourceTree, useLeafNames, prunedUse, referencedItemNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {declaredNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const [poolPath, ident, ...files] = process.argv.slice(2);
const pool = readFileSync(poolPath!, "utf8");
const text = files.map((f) => readFileSync(f, "utf8")).join("\n");
const referenced = referencedItemNames(text);
const declared = declaredNames(text, false);
const out: string[] = [];
const covered = new Set<string>();
for (const item of parseRustSourceTree(pool).items) {
  if (item.kind !== "use") continue;
  const statement = pool.slice(item.declStart, item.end);
  const leaves = useLeafNames(statement).filter((leaf) => referenced.has(leaf) && !declared.has(leaf) && !covered.has(leaf));
  if (!leaves.length) continue;
  let pruned = prunedUse(statement, leaves).replace(/^\s*pub(\([^)]*\))?\s+/u, "").trim();
  if (!pruned) continue;
  pruned = pruned.replace(/(?<![\w:])crate::/gu, `${ident}::`);
  out.push(pruned.endsWith(";") ? pruned : pruned + ";");
  for (const leaf of leaves) covered.add(leaf);
}
console.log(out.join("\n"));
