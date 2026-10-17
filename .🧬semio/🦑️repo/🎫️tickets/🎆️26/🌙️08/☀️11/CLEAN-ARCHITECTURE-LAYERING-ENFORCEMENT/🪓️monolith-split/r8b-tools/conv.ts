import { readFileSync } from "node:fs";
import { convertMutationLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const root = process.argv[2]!;
const read = (f: string) => { try { return readFileSync(f, "utf8"); } catch { return null; } };
const r = convertMutationLeaves(root, read(root)!, read);
console.log(r.notes.join("\n"));
const pat = process.argv[3] ?? "replace-block";
for (const [k, v] of [...r.edits, ...r.creates]) if (k.includes(pat)) { console.log("=====", k); console.log(v); }
