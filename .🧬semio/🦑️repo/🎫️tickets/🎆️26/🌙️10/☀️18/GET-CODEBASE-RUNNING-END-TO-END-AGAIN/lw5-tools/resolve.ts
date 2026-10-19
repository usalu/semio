import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const R = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
let bad = 0;
for (const f of plan.fixes) {
  try {
    const got = Bun.resolveSync(f.replacement, dirname(R + f.from));
    const want = resolve(dirname(R + f.from), f.specifier);
    if (got.normalize("NFC") !== want.normalize("NFC") && !got.endsWith(f.to.normalize("NFC"))) { bad++; console.log("DIFFERENT", f.replacement, got, "!=", f.to); }
  } catch (e) { bad++; console.log("UNRESOLVED", f.replacement, "from", f.from, String(e).slice(0, 100)); }
}
console.log("checked", plan.fixes.length, "bad", bad);
