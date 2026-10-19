import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fam = (...d: string[]) => d.map((x) => readSourceFamilySource(`${P}/${x}`)).join("");
const instance = fam("🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job");
const re = new RegExp(process.argv[2]!, "g");
const ctx = Number(process.argv[3] ?? 120);
let n = 0;
for (const m of instance.matchAll(re)) { console.log("@" + m.index, JSON.stringify(instance.slice(Math.max(0, m.index! - ctx / 3), m.index! + ctx))); if (++n > 12) break; }
console.log("matches", n);
