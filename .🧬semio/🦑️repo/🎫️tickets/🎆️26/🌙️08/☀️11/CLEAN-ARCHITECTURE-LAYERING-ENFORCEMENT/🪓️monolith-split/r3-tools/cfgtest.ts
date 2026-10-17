// 🧪️ Finds #[cfg(test)] items defined in one crate of a carved subset and named by another crate of the same subset (read-only).
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {artifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const [dir, prefix] = process.argv.slice(2) as [string, string];
const crates = artifactCrates(root, dir).filter(c => c.name.startsWith(prefix) && !/-(export|import)$/u.test(c.name));
const files = new Map<string, {lib: Set<string>; test: Set<string>}>();
for (const c of crates) {
  const lib = await measureRustCrate(root, c.entry, false, new Map(), join(root, c.dir));
  const test = await measureRustCrate(root, c.entry, true, new Map(), join(root, c.dir));
  files.set(c.name, {lib: new Set(lib.sourcePaths), test: new Set(test.sourcePaths)});
}
const defs: {crate: string; name: string; file: string}[] = [];
for (const [name, {lib}] of files) for (const f of lib) {
  const text = readFileSync(join(root, f), "utf8");
  for (const m of text.matchAll(/#\[cfg\(test\)\]\s*(?:(?:#\[[^\]]*\]|\/\/[^\n]*)\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|const|static|struct|enum|type|trait)\s+([A-Za-z_]\w*)/gu)) defs.push({crate: name, name: m[1]!, file: f});
}
console.log("cfg(test) item definitions in lib closures:", defs.length);
let bad = 0;
for (const d of defs) for (const [name, {test}] of files) {
  if (name === d.crate) continue;
  for (const f of test) { if (files.get(d.crate)!.test.has(f) && files.get(d.crate)!.lib.has(f)) continue; const text = readFileSync(join(root, f), "utf8"); if (new RegExp(`\\b${d.name}\\b`, "u").test(text)) { bad++; console.log(`${d.name} (${d.crate}, ${d.file.split("/").slice(-4).join("/")}) named by ${name} in ${f.split("/").slice(-4).join("/")}`); } }
}
console.log("cross-crate references:", bad);
