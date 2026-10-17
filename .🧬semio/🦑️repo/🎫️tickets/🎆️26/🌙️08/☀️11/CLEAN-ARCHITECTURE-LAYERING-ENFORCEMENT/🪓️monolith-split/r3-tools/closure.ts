// 🔗️ Item closure inside one Rust file: which top-level items does a set of entry items need (read-only).
import {readFileSync} from "node:fs";
import {parseRustSourceTree, referencedItemNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [file, moduleName, ...entries] = process.argv.slice(2);
const text = readFileSync(file!, "utf8");
const tree = parseRustSourceTree(text);
const scope = moduleName === "-" ? tree.items : tree.modules.find(m => m.name === moduleName)!.items;
const byName = new Map(scope.filter(i => i.name && i.kind !== "use").map(i => [i.name!, i]));
const seen = new Set<string>(); const queue = [...entries];
while (queue.length) { const n = queue.pop()!; if (seen.has(n)) continue; const item = byName.get(n); if (!item) { console.log("MISSING", n); continue; } seen.add(n); for (const id of referencedItemNames(text.slice(item.declStart, item.end))) if (byName.has(id) && id !== n) queue.push(id); }
const all = [...byName.keys()];
console.log("total items", all.length, "closure", seen.size);
console.log([...seen].sort().join(" "));
console.log("NOT in closure:", all.filter(n => !seen.has(n)).join(" "));
const usedByRest = all.filter(n => !seen.has(n)).filter(n => { const item = byName.get(n)!; return [...referencedItemNames(text.slice(item.declStart, item.end))].some(id => seen.has(id)); });
console.log("rest items using closure:", usedByRest.join(" "));
