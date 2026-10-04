/** 🎲️ Prints a stratified sample of distinct questions of a fresh seed range (default `live-101`) of `hint_audit2_probe.ts`:
 * per quiz task and hint form (reversed, under, over, scale word, power of ten, profile relative or value, group) a fixed quota;
 * `bun hint_audit2_sample.ts [corpus] [perStratum]`. */
import { readFileSync } from "node:fs";

const corpus = process.argv[2] ?? "live-101";
const quota = Number(process.argv[3] ?? 4);
const records = readFileSync(new URL(`./🗑️generated/hint-audit-2/${corpus}/hints.jsonl`, import.meta.url), "utf8").split("\n").map((line) => JSON.parse(line));
const form = (r: any): string => {
  if (r.kind === "compare") {
    if (/roughly 10[⁰¹²³⁴⁵⁶⁷⁸⁹]/u.test(r.en)) return `${r.verdict}/power`;
    if (/ (million|billion|trillion)/u.test(r.en)) return `${r.verdict}/word`;
    return `${r.verdict}${r.additive ? "/sum" : r.difference !== undefined ? "/diff" : "/ratio"}${r.dims > 1 ? "/dim" : ""}`;
  }
  if (r.kind === "profile") return r.other ? "profile/relative" : "profile/value";
  return r.kind;
};
const strata = new Map<string, any[]>();
for (const r of records) {
  const key = `${r.quiz}/${r.task}/${form(r)}`;
  const list = strata.get(key) ?? strata.set(key, []).get(key)!;
  if (!list.some((x) => x.en === r.en)) list.push(r);
}
let total = 0;
for (const [key, list] of [...strata].sort()) {
  const step = Math.max(1, Math.floor(list.length / quota));
  const take = list.filter((_, i) => i % step === 0).slice(0, quota);
  console.log(`\n=== ${key} (${list.length} distinct)`);
  for (const r of take) {
    total++;
    console.log(`#${total} EN ${r.en}\n   DE ${r.de}${r.claim === undefined ? "" : `\n   claim ${Number(r.claim.toPrecision(3))} truth ${Number(r.truth.toPrecision(3))}`}`);
  }
}
console.log("\n[DEBUG] total", total);
