// 📖️ c4p35 reader probe: the policy reader text of a split root (policyReadRustPolicySource) against the pre-edit file, as stripped non-empty line multisets. usage: bun policy-read.ts <repo-relative root> <orig snapshot>
import {readFileSync} from "node:fs";
import {policyReadRustPolicySource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
const [root, orig] = [process.argv[2]!, process.argv[3]!];
const text = policyReadRustPolicySource("/Users/ueli/Documents/semio", root);
const count = (t: string) => { const m = new Map<string, number>(); for (const l of t.split("\n")) { const s = l.trim(); if (s) m.set(s, (m.get(s) ?? 0) + 1); } return m; };
const a = count(readFileSync(orig, "utf8")), b = count(text);
let missing = 0, extra = 0; const miss: string[] = [];
for (const [k, v] of a) { const d = v - (b.get(k) ?? 0); if (d > 0) { missing += d; miss.push(k.slice(0, 100)); } }
for (const [k, v] of b) { const d = v - (a.get(k) ?? 0); if (d > 0) extra += d; }
console.log(JSON.stringify({root: root.split("/").slice(-3).join("/"), policyTextBytes: text.length, origLines: a.size, missingLines: missing, extraLines: extra, missingSample: miss.slice(0, 8)}));
