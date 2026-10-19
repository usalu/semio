import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fam = (...d: string[]) => d.map((x) => readSourceFamilySource(`${P}/${x}`)).join("");
const instance = fam("🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job");
const [needle, after = "200", before = "50"] = process.argv.slice(2);
let from = 0, n = 0;
for (;;) { const i = instance.indexOf(needle!, from); if (i < 0 || n > 6) break; console.log("@" + i, JSON.stringify(instance.slice(Math.max(0, i - Number(before)), i + Number(after)))); from = i + 1; n++; }
console.log("count", n);
