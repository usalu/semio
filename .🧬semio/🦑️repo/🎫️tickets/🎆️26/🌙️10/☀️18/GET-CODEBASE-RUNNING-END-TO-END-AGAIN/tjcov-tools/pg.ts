import { readSourceFamilySource, listSourceFamily } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync } from "node:fs";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const dir = process.argv[2] ? P + "/" + process.argv[2] : P;
const re = new RegExp(process.argv[3]!, "g");
const ctx = Number(process.argv[4] ?? 160);
const tests = process.argv[5] === "tests";
let n = 0;
for (const f of listSourceFamily(dir, { tests })) {
  const t = readFileSync(f, "utf8");
  for (const m of t.matchAll(re)) { console.log(f.slice(P.length + 1) + ":" + t.slice(0, m.index).split("\n").length, JSON.stringify(t.slice(Math.max(0, m.index! - ctx / 4), m.index! + ctx))); if (++n > 25) process.exit(0); }
}
console.log("matches", n);
