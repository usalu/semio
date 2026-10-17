import { writeFileSync } from "node:fs";
import { auditParsers } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const [root, under, out] = process.argv.slice(2);
const drift = auditParsers({ root: root!, under: under || "✏️s" });
writeFileSync(out!, JSON.stringify(drift, null, 1));
console.log(drift.length);
