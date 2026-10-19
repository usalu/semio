import { readModuleTreeSource, readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const [mode, path, ...needles] = process.argv.slice(2);
const text = mode === "family" ? readSourceFamilySource(path!) : readModuleTreeSource(path!);
console.log("bytes", text.length);
for (const needle of needles) console.log(text.includes(needle) ? "HAS " : "MISS", needle.slice(0, 120));
