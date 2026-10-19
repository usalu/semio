import { readFileSync } from "node:fs";
import { lex } from "./lex.ts";
const G = `${import.meta.dir}/../🗑️generated/value-repoint`;
const ledger: Record<string, string> = JSON.parse(readFileSync(`${G}/backup/ledger.json`, "utf8"));
let bad = 0;
for (const [f, h] of Object.entries(ledger)) {
  const b = readFileSync(`${G}/backup/${h}`, "utf8");
  let c: string;
  try { c = readFileSync(f, "utf8"); } catch { console.log("MISSING", f); continue; }
  const sb = lex(b).filter((t) => t.k === "s").map((t) => t.t).sort();
  const sc = lex(c).filter((t) => t.k === "s").map((t) => t.t).sort();
  if (sb.length !== sc.length || sb.some((x, i) => x !== sc[i])) { bad++; console.log("STRINGS-DIFFER", f); }
}
console.log("files", Object.keys(ledger).length, "string-diff", bad);
