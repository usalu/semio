import { readFileSync } from "node:fs";
import { convertMutationLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const root = process.argv[2]!;
const read = (f: string) => { try { return readFileSync(f, "utf8"); } catch { return null; } };
const r = convertMutationLeaves(root, read(root)!, read);
for (const k of [...r.edits.keys(), ...r.creates.keys()]) console.log(k.split("/").slice(-4).join("/"));
