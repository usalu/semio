import { readFileSync } from "node:fs";
import { parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/34f3999f-e145-4d4e-ab13-c3c2aef22ddf/scratchpad";
const root = "/Users/ueli/Documents/semio/";
let dn = 0, up = 0;
for (const line of readFileSync(`${S}/map.txt`, "utf8").trim().split("\n")) {
  const [i, f] = line.split("|");
  const before = parseFeature(readFileSync(`${S}/hf/${i}.feature`, "utf8"));
  const after = parseFeature(readFileSync(root + f, "utf8"));
  const b = new Set(before.scenarios.map((s) => s.id)), a = new Set(after.scenarios.map((s) => s.id));
  const lost = [...b].filter((x) => !a.has(x)), gained = [...a].filter((x) => !b.has(x));
  const name = f.split("/").slice(-2)[0];
  if (lost.length || gained.length || before.oracle !== after.oracle || before.noOracleDecision !== after.noOracleDecision || before.comparison !== after.comparison) {
    console.log(`${name}: ${before.scenarios.length} -> ${after.scenarios.length}  oracle ${before.oracle} -> ${after.oracle}  noOracle ${before.noOracleDecision} -> ${after.noOracleDecision}  cmp ${before.comparison} -> ${after.comparison}`);
    if (lost.length) console.log(`   LOST(${lost.length}): ${lost.slice(0, 12).join(", ")}`);
    if (gained.length) console.log(`   GAINED(${gained.length}): ${gained.slice(0, 6).join(", ")}${gained.length > 6 ? " …" : ""}`);
  }
  dn += before.scenarios.length; up += after.scenarios.length;
}
console.log(`TOTAL over changed features: ${dn} -> ${up}`);
