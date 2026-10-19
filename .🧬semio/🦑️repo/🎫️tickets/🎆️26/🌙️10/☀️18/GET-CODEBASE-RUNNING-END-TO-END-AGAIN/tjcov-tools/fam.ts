import { listSourceFamily, readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const d = process.argv[2]!;
const t = Date.now();
const files = listSourceFamily(d, { tests: process.argv[3] === "tests" });
const s = readSourceFamilySource(d, { tests: process.argv[3] === "tests" });
console.log("files", files.length, "bytes", s.length, "ms", Date.now() - t);
