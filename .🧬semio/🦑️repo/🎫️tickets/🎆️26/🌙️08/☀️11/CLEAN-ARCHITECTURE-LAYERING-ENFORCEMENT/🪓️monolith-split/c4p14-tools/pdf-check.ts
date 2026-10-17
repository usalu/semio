// 🔎️ Loads the split pdf diff facet and checks its schema against the pre-edit literal and a few parser probes.
import { readFileSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";
const F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/";
const live = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts";
const old = readFileSync(F + "🗑️generated/c4p14-snap/pdf_diff.ts", "utf8");
const a = "export const schema = ", b = "\n} as const;\n";
const literal = JSON.parse(old.slice(old.indexOf(a) + a.length, old.indexOf(b) + 2));
const mod = await import(live);
console.log("schema deep-equal old literal:", isDeepStrictEqual(mod.schema, literal));
console.log("exports:", Object.keys(mod).length, "parsers:", Object.keys(mod).filter((k) => k.startsWith("parse")).length);
const probes: [string, unknown][] = [["parsePdfDiff", {}], ["parsePdfDiff", { declaredVersion: "1.7" }], ["parsePdfDiff", { declaredVersion: 7 }], ["parsePdfPageBox", "crop"], ["parsePdfPageBox", "nope"], ["parsePdfPathSegment", { kind: "arrayIndex", index: 3 }], ["parsePdfPathSegment", { kind: "dictKey" }], ["parsePdfDictDiff", { removed: ["a"] }], ["parsePdfDictDiff", { removed: [1] }]];
for (const [name, value] of probes) {
  let outcome: string;
  try { mod[name](value); outcome = "accepts"; } catch (error) { outcome = "refuses: " + (error as Error).message; }
  console.log(name, JSON.stringify(value), "=>", outcome);
}
