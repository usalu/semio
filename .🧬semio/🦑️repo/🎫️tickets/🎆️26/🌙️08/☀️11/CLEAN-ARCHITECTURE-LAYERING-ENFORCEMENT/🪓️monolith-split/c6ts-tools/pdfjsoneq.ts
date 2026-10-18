import json from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json" with { type: "json" };
import { readFileSync } from "node:fs";
const canon = (o: unknown): string => JSON.stringify(o, (_k, v) => v && typeof v === "object" && !Array.isArray(v) ? Object.fromEntries(Object.entries(v).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) : v);
const text = readFileSync(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/c6ts-orig/pdf-snapshot.ts", "utf8");
const i = text.indexOf("export const schema = ") + "export const schema = ".length, j = text.indexOf("} as const;", i) + 1;
const orig = new Function("return (" + text.slice(i, j) + ")")();
const a = canon(json), b = canon(orig);
console.log(JSON.stringify({ equalToOriginalTsCopy: a === b, length: a.length, defs: Object.keys((json as any).$defs).length }));
