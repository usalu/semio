// 🧰 bd10: replaces `use super::*;` of a moved child by the pruned imports it really uses, taken from the use items of the former parent file.
// usage: bun prune-uses.ts <child.rs> <former-parent.rs> [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {parseRustSourceTree, useLeafNames, prunedUse, referencedItemNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {declaredNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const [childPath, parentPath] = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const write = process.argv.includes("--write");
const child = readFileSync(childPath!, "utf8");
const parent = readFileSync(parentPath!, "utf8");
const referenced = referencedItemNames(child.replace(/^use super::\*;\n/mu, ""));
const declared = declaredNames(child.replace(/^use super::\*;\n/mu, ""), false);
const out: string[] = [];
const covered = new Set<string>();
for (const item of parseRustSourceTree(parent).items) {
  if (item.kind !== "use") continue;
  const statement = parent.slice(item.declStart, item.end);
  const leaves = useLeafNames(statement).filter((leaf) => referenced.has(leaf) && !declared.has(leaf) && !covered.has(leaf));
  if (!leaves.length) continue;
  const text = prunedUse(statement, leaves).replace(/^\s*pub(\([^)]*\))?\s+/u, "").trim();
  if (text) { out.push(text.endsWith(";") ? text : text + ";"); for (const leaf of leaves) covered.add(leaf); }
}
const keepGlob = process.argv.includes("--keep-glob");
const text = child.replace(/^use super::\*;\n/mu, `${keepGlob ? "use super::*;\n" : ""}${out.join("\n")}\n`);
console.log(out.join("\n"));
if (write) writeFileSync(childPath!, text);
