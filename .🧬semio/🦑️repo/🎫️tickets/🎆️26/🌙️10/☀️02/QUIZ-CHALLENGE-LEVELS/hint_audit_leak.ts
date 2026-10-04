/** 🚰️ Checks whether a shown count comes within 10 % of the truth (either orientation), and what share of hints the claim is a near miss (error below 3). */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line)).filter((r) => r.kind === "compare");
const shown = (r: any): number => { const m = r.en.match(/Are you sure (?:it takes )?([\d.,]+) ×/u) ?? r.en.match(/(?:only|as much as) ([\d.,]+) times/u); return Number(m[1].replace(/,/gu, "")); };
let leak = 0, near = 0;
const out: string[] = [];
for (const r of records) {
  const s = shown(r);
  const t = Math.max(r.truth, 1 / r.truth);
  if (Math.abs(s / t - 1) < 0.1) { leak++; out.push(`${r.en} truth=${r.truth}`); }
  const e = Math.max(r.claim / r.truth, r.truth / r.claim);
  if (e < 3) near++;
}
console.log("shown within 10% of truth:", leak, out.slice(0, 3));
console.log("error (claim vs truth) < 3:", near, "of", records.length);
const errs = records.map((r) => Math.max(r.claim / r.truth, r.truth / r.claim)).sort((a, b) => a - b);
console.log("error quantiles min/10%/median/90%:", errs[0], errs[Math.floor(errs.length * 0.1)], errs[Math.floor(errs.length / 2)], errs[Math.floor(errs.length * 0.9)]);
