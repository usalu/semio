// 🔬️ Dumps the mutation-family conversion of the pdf artifact for one aggregate so the leaf output can be read.
import {readFileSync, existsSync, writeFileSync, mkdirSync} from "node:fs";
import {convertMutationLeaves} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs";
const readText = (p: string) => existsSync(p) ? readFileSync(p, "utf8") : null;
const result = convertMutationLeaves(root, readFileSync(root, "utf8"), readText);
const out = process.argv[2]!;
mkdirSync(out, {recursive: true});
let n = 0;
for (const [file, text] of [...result.edits, ...result.creates]) { if (!file.includes(process.argv[3] ?? "")) continue; writeFileSync(`${out}/${n++}.rs`, `// ${file}\n${text}`); }
console.log(result.notes.join("\n"));
console.log(n, "files written");
