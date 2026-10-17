import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { convertMutationLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const root = process.argv[2]!; const out = process.argv[3]!;
const read = (f: string) => { try { return readFileSync(f, "utf8"); } catch { return null; } };
const r = convertMutationLeaves(root, read(root)!, read);
for (const [k, v] of [...r.edits, ...r.creates]) { const dest = out + "/" + k.replace(/^\//, ""); mkdirSync(dirname(dest), { recursive: true }); writeFileSync(dest, v); }
console.log(r.notes.join("\n"));
