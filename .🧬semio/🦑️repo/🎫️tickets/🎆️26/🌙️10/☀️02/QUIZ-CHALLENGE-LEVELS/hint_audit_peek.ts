/** 👀️ Peeks at corpus subsets of `hint_audit_probe.ts`: profile hints, digit-leading labels, mixed-dimension matching. */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
const seen = new Set<string>();
const uniq = (list: any[]) => list.filter((r) => (seen.has(r.en) ? false : (seen.add(r.en), true)));
console.log("--- profile");
for (const r of uniq(records.filter((x) => x.kind === "profile"))) console.log(`(${r.flaw}) ${r.en}\n   ${r.de}`);
console.log("--- digit-leading DE label in sentence");
for (const r of uniq(records.filter((x) => /× „\d/u.test(x.de) || /× “\d/u.test(x.en))).slice(0, 4)) console.log(`${r.en}\n   ${r.de}`);
console.log("--- two-dimension matching same pair, different dimension (no dimension in text)");
const dim = records.filter((x) => x.dimension && (x.quiz === "heating" && x.task === "heating-load-and-demand"));
const by = new Map<string, any[]>();
for (const r of dim) (by.get(`${r.seed}/${r.flaw}`) ?? by.set(`${r.seed}/${r.flaw}`, []).get(`${r.seed}/${r.flaw}`)!).push(r);
let shown = 0;
for (const [k, list] of by) if (list.length >= 3 && shown++ < 2) { console.log(k); for (const r of list) console.log(`  [${r.dimension}] ${r.en}`); }
console.log("--- DE long-label parentheses");
for (const r of uniq(records.filter((x) => x.kind === "compare" && /\(.*\).*\(.*\)/u.test(x.de))).slice(0, 4)) console.log(`${r.de}`);
console.log("--- DE 'ganze' with fractional count");
for (const r of uniq(records.filter((x) => /ganze \d+,\d/u.test(x.de))).slice(0, 3)) console.log(`${r.de}`);
console.log("--- DE 'nur 1,x-mal' / 'nur 2-mal'");
for (const r of uniq(records.filter((x) => /nur [12](,\d)?-mal/u.test(x.de))).slice(0, 4)) console.log(`${r.de}`);
console.log("--- EN: 'a 0.0000372' etc small counts and comma");
for (const r of uniq(records.filter((x) => /[\d,]{9,} ×/u.test(x.en))).slice(0, 3)) console.log(r.en);
