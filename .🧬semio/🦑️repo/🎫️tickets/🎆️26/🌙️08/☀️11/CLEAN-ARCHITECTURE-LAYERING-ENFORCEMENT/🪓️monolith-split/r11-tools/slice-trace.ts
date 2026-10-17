// 🔬️ Prints the buildSlices union trace (why decls are grouped) of one artifact package, without test transforms.
import {existsSync, readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const {buildSlices, collectDecls} = await import(`${L}/🧱️decl-tree/🟦️.ts`);
const {inlineMountTreeFiles} = await import(`${L}/🌲️inline-mounts/🟦️.ts`);
const {readCargoLib} = await import(`${L}/🗺️plan/🟦️.ts`);
const root = "/Users/ueli/Documents/semio";
const lib = readCargoLib(resolve(root, process.argv[2]!, "Cargo.toml"));
const readText = (p: string): string | null => existsSync(p) ? readFileSync(p, "utf8") : null;
const folded = inlineMountTreeFiles(lib.entry, readFileSync(lib.entry, "utf8"), readText);
const tree = collectDecls(lib.entry, folded.rootSource, (p: string) => p === lib.entry ? folded.rootSource : readText(p));
const filter = process.argv[3] ?? "";
const built = buildSlices(tree, (p: string) => p === lib.entry ? folded.rootSource : readText(p), (m: string) => { if (!filter || m.includes(filter)) console.log(m.replace(/[^ ]*✏️editor/g, "…✏️editor")); }, {detachAllTests: true});
console.log("slices", built.slices.length);
