/** 📈️ Reads the records of `hint_audit_probe.ts` and prints the corpus statistics and the worst offenders per defect class. */
import { readFileSync } from "node:fs";

const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
const density = JSON.parse(readFileSync(new URL("density.json", OUT), "utf8")) as { quiz: string; task: string; flaw: string; items: number; hints: number; wrong: number }[];
const show = (title: string, list: any[], limit = 6, key: (r: any) => string = (r) => r.en): void => {
  const seen = new Set<string>();
  const rows = list.filter((r) => (seen.has(key(r)) ? false : (seen.add(key(r)), true)));
  console.log(`\n## ${title}: ${list.length} records, ${rows.length} distinct`);
  for (const r of rows.slice(0, limit)) console.log(`  [${r.quiz}/${r.task}/${r.flaw}] ${r.en}\n      ${r.de}${r.claim !== undefined ? `\n      claim=${r.claim} truth=${r.truth} factor=${r.factor} under=${r.under}` : ""}`);
};
const compare = records.filter((r) => r.kind === "compare");
const digitsRun = (text: string): number => Math.max(0, ...(text.match(/[\d.,]+/gu) ?? []).map((m) => m.replace(/[.,\s]/gu, "").length));

show("one-times (count 1 / 1 ×)", compare.filter((r) => /(^|[^\d.,])1 (×|times)/u.test(r.en) && !/add up to 1 ×/u.test(r.en.replace(/^.*?(together|takes)/u, "")) || /\b(1|1,0) × “/u.test(r.en.split("add up")[0]!) || /ganze 1-mal|nur 1-mal/u.test(r.de)));
show("long digit strings (>=7 digits)", compare.filter((r) => digitsRun(r.en) >= 7), 8);
show("huge digit strings (>=12 digits)", compare.filter((r) => digitsRun(r.en) >= 12), 8);
show("count below 2 (e.g. 1.2 times)", compare.filter((r) => { const m = r.en.match(/(\d+(?:\.\d+)?)(?: times| ×)/u) ?? r.en.match(/ ([\d.,]+) × “/u); const n = m ? Number(m[1]!.replace(/,/gu, "")) : NaN; return n < 2; }), 8);
show("label with parentheses", records.filter((r) => /[()]/u.test(r.itemEn ?? "") || /[()]/u.test(r.otherEn ?? "")), 6);
show("label length >= 38", records.filter((r) => (r.itemEn ?? "").length >= 38 || (r.otherEn ?? "").length >= 38 || (r.itemDe ?? "").length >= 38 || (r.otherDe ?? "").length >= 38), 6);

let wrongSide = 0;
let tipped = 0;
let reversed = 0;
let samePairLabels = 0;
let truthNearClaim = 0;
for (const r of compare) {
  const rho = r.claim;
  const tau = r.truth;
  const under = rho >= 1 ? tau > rho : tau < rho;
  if (under !== r.under) wrongSide++;
  const e = Math.max(rho, tau) / Math.min(rho, tau);
  if ((rho >= 1) !== (tau >= 1)) reversed++;
  if (e < 1.5) truthNearClaim++;
  if (r.itemEn === r.otherEn) samePairLabels++;
}
console.log(`\n## sanity: under flag disagrees with claim/truth: ${wrongSide}; direction reversed (truth on the other side of 1): ${reversed}; error<1.5: ${truthNearClaim}; same label: ${samePairLabels}`);

const claimOrder = (r: any): number => Math.max(r.claim, 1 / r.claim);
const absurdClaim = compare.filter((r) => claimOrder(r) < 3);
show("claimed relation < 3x (claim itself looks near-equal, doubt unclear)", absurdClaim, 8);
const big = compare.filter((r) => claimOrder(r) > 1e9);
show("claimed relation > 1e9", big, 6);

const byKind = new Map<string, number>();
for (const r of records) byKind.set(r.kind, (byKind.get(r.kind) ?? 0) + 1);
console.log("\n## kinds", JSON.stringify([...byKind]));

const stat = new Map<string, number[]>();
for (const d of density) {
  const key = `${d.quiz}/${d.task}/${d.flaw}`;
  (stat.get(key) ?? stat.set(key, []).get(key)!).push(d.hints);
}
console.log("\n## density (hints per answer: mean / max / share zero) by task and flaw");
for (const [key, values] of [...stat].sort()) {
  const mean = values.reduce((a, b) => a + b, 0) / values.length;
  console.log(`  ${key.padEnd(54)} mean ${mean.toFixed(1).padStart(5)} max ${String(Math.max(...values)).padStart(3)} zero ${((values.filter((v) => v === 0).length / values.length) * 100).toFixed(0).padStart(3)}%`);
}

const refs = new Map<string, number>();
for (const r of compare) refs.set(`${r.quiz}/${r.task}: ${r.otherEn}`, (refs.get(`${r.quiz}/${r.task}: ${r.otherEn}`) ?? 0) + 1);
console.log("\n## most used references");
for (const [k, v] of [...refs].sort((a, b) => b[1] - a[1]).slice(0, 14)) console.log(`  ${v}\t${k}`);
const itemsOf = new Map<string, number>();
for (const r of compare) itemsOf.set(`${r.quiz}/${r.task}: ${r.itemEn} | ${r.otherEn}`, (itemsOf.get(`${r.quiz}/${r.task}: ${r.itemEn} | ${r.otherEn}`) ?? 0) + 1);
