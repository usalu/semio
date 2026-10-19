import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fam = (...d: string[]) => d.map((x) => readSourceFamilySource(`${P}/${x}`)).join("");
const instance = fam("🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job");
console.log(instance.slice(Number(process.argv[2]), Number(process.argv[3])));
