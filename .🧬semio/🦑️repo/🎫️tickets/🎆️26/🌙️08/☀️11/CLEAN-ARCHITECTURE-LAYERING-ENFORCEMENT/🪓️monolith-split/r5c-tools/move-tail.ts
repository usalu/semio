// ✂️ Moves the tail of a module file (from a line containing <marker>) into another file, carrying the imports of the source file it uses. Usage: <from.rs> <marker-line-substring> <to.rs> <glob-import-lines...>
import {readFileSync, writeFileSync} from "node:fs";
import {identifierSet, parseRustSourceTree, prunedUse, useLeafNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [from, marker, to, ...extra] = process.argv.slice(2);
const text = readFileSync(from!, "utf8");
const lines = text.split("\n");
const at = lines.findIndex(l => l.includes(marker!));
if (at < 0) throw new Error("marker not found");
let start = at; while (start > 0 && /^\s*(\/\/|#\[)/.test(lines[start - 1]!)) start--;
const tail = lines.slice(start).join("\n");
const head = lines.slice(0, start).join("\n").replace(/\n+$/, "\n");
const used = identifierSet(tail);
const carried: string[] = [];
const tree = parseRustSourceTree(text);
for (const item of tree.items) if (item.kind === "use") {
  const st = text.slice(item.declStart, item.end);
  const leaves = useLeafNames(st).filter(l => used.has(l));
  if (leaves.length) { const p = prunedUse(st, leaves); if (p) carried.push(p); }
}
const dest = readFileSync(to!, "utf8");
writeFileSync(from!, head);
writeFileSync(to!, dest.replace(/\n*$/, "\n") + "\n" + [...carried.map(c => c.replace(/^(\s*)/, "")), ...extra].join("\n") + "\n\n" + tail.replace(/\n*$/, "\n"));
console.log("moved", tail.length, "bytes; carried", carried.length, "imports");
