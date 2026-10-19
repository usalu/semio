import { auditLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
import { writeFileSync } from "node:fs";
const [under, out] = [process.argv[2]!, process.argv[3]!];
const a = auditLeaves({ root: "/Users/ueli/Documents/semio", under });
const keys = a.drift.map((d: any) => `${d.path.split("/").slice(-4).join("/")}|${d.name}|${d.reason}`).sort();
writeFileSync(out, JSON.stringify({ drift: keys, helperless: a.helperless }, null, 1));
console.log("drift", keys.length, "helperless", JSON.stringify(a.helperless).length);
