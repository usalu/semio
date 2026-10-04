/** 🧮️ Extra corpus statistics of `hint_audit_probe.ts`: reversal share, mirrored pairs, tiny counts, ties, label and question lengths, quotes in labels, repeated references per answer. */
import { readFileSync } from "node:fs";

const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
const compare = records.filter((r) => r.kind === "compare");
const pct = (n: number, d: number): string => `${n}/${d} (${((100 * n) / d).toFixed(1)}%)`;

const reversed = (r: any): boolean => (r.claim >= 1) !== (r.truth >= 1);
const under = compare.filter((r) => r.under);
const over = compare.filter((r) => !r.under);
console.log("under", under.length, "over", over.length, "over+reversed", pct(over.filter(reversed).length, over.length), "over same-direction", over.filter((r) => !reversed(r)).length);
console.log("distinct compare EN", new Set(compare.map((r) => r.en)).size, "of", compare.length);
const perTaskReversed = new Map<string, [number, number]>();
for (const r of compare) {
  const key = `${r.quiz}/${r.task}`;
  const cur = perTaskReversed.get(key) ?? [0, 0];
  cur[1]++;
  if (!r.under && reversed(r)) cur[0]++;
  perTaskReversed.set(key, cur);
}
console.log("share of hints that are 'over + direction reversed' per task:", [...perTaskReversed].map(([k, [a, b]]) => `${k}: ${pct(a, b)}`).join("; "));

const shown = (r: any): number => { const m = r.en.match(/(?:only add up|takes) /u) ? r.en.match(/(?:Are you sure (?:it takes )?)([\d.,]+) ×/u) : r.en.match(/(?:only|as much as) ([\d.,]+) times/u); return m ? Number(m[1].replace(/,/gu, "")) : NaN; };
const tiny = compare.filter((r) => shown(r) < 1.5);
console.log("shown count < 1.5:", tiny.length, tiny.slice(0, 5).map((r) => `${r.en} | claim ${r.claim} truth ${r.truth}`));
console.log("shown count NaN:", compare.filter((r) => Number.isNaN(shown(r))).length);
console.log("claim == 1:", compare.filter((r) => r.claim === 1).length, "truth == 1:", compare.filter((r) => r.truth === 1).length);
const oneX = compare.filter((r) => /\b1(\.0)? ×/u.test(r.en.split("add up to")[0]!) || shown(r) === 1);
console.log("count shown as 1:", oneX.length, oneX.slice(0, 3).map((r) => r.en));
console.log("fractional additive counts (e.g. 2.8 x item):", pct(compare.filter((r) => /(?:Are you sure|takes) [\d,]*\.\d+ ×/u.test(r.en)).length, compare.filter((r) => / ×/u.test(r.en)).length));

const mirrored = new Map<string, number>();
const answers = new Map<string, any[]>();
for (const r of compare) (answers.get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`) ?? answers.set(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`, []).get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`)!).push(r);
let pairs = 0;
let mirrors = 0;
let sameRef = 0;
let multi = 0;
for (const [key, list] of answers) {
  if (list.length < 2) continue;
  multi++;
  const set = new Set(list.map((r) => `${r.dimension ?? ""}|${r.item}|${r.other}`));
  for (const r of list) {
    pairs++;
    if (set.has(`${r.dimension ?? ""}|${r.other}|${r.item}`)) mirrors++;
  }
  const refs = list.map((r) => r.other);
  if (new Set(refs).size < refs.length) sameRef++;
  void mirrored;
}
console.log("answers with >=2 compare hints:", multi, "| hints whose mirror (swapped roles) is also asked:", pct(mirrors, pairs), "| answers re-using one reference:", pct(sameRef, multi));

const labels = new Set<string>();
for (const r of records) for (const l of [r.itemEn, r.otherEn, r.itemDe, r.otherDe]) if (l) labels.add(l);
console.log("labels with quote chars:", [...labels].filter((l) => /[“”„"]/u.test(l)));
console.log("labels starting with digit:", [...labels].filter((l) => /^\d/u.test(l)));
console.log("labels with parentheses:", [...labels].filter((l) => /[()]/u.test(l)).length, "of", labels.size);
console.log("label length max", Math.max(...[...labels].map((l) => l.length)), "labels >60:", [...labels].filter((l) => l.length > 60).length, [...labels].filter((l) => l.length > 70).slice(0, 6));
const lens = records.map((r) => r.en.length).sort((a, b) => a - b);
console.log("EN question length median", lens[Math.floor(lens.length / 2)], "p90", lens[Math.floor(lens.length * 0.9)], "max", lens[lens.length - 1], ">160:", pct(lens.filter((l) => l > 160).length, lens.length));
const dlens = records.map((r) => r.de.length).sort((a, b) => a - b);
console.log("DE question length median", dlens[Math.floor(dlens.length / 2)], "p90", dlens[Math.floor(dlens.length * 0.9)], "max", dlens[dlens.length - 1]);
console.log("longest EN:", records.sort((a, b) => b.en.length - a.en.length).slice(0, 3).map((r) => `${r.en.length}: ${r.en}`));
const groups = records.filter((r) => r.kind === "group");
console.log("group together", groups.filter((r) => r.together).length, "apart", groups.filter((r) => !r.together).length);
console.log("apart sample:", groups.filter((r) => !r.together).slice(0, 3).map((r) => r.en));
console.log("category kind:", records.filter((r) => r.kind === "category").length);

const itemSet = new Map<string, number>();
for (const r of compare) itemSet.set(r.otherEn, (itemSet.get(r.otherEn) ?? 0) + 1);
console.log("top references:", [...itemSet].sort((a, b) => b[1] - a[1]).slice(0, 10).map(([k, v]) => `${v}x ${k}`));
const refShare = new Map<string, Map<string, number>>();
for (const r of compare) {
  const m = refShare.get(r.task) ?? refShare.set(r.task, new Map()).get(r.task)!;
  m.set(r.otherEn, (m.get(r.otherEn) ?? 0) + 1);
}
for (const [task, m] of refShare) {
  const total = [...m.values()].reduce((a, b) => a + b, 0);
  const top = [...m].sort((a, b) => b[1] - a[1]).slice(0, 2);
  console.log(`  ${task}: ${m.size} distinct references; top: ${top.map(([k, v]) => `${((100 * v) / total).toFixed(0)}% ${k.slice(0, 40)}`).join(" | ")}`);
}

const ordersOfMagnitude = (r: any): number => Math.abs(Math.log10(r.truth));
const huge = compare.filter((r) => /\d[\d.,]{9,}/u.test(r.en));
console.log("hints with a count of >= 10 digits:", pct(huge.length, compare.length));
const sci = compare.filter((r) => /e\+|E\d/u.test(r.en));
console.log("hints containing exponent notation:", sci.length);
console.log("claimed relation < 3 (absurd-looking claim):", pct(compare.filter((r) => Math.max(r.claim, 1 / r.claim) < 3).length, compare.length));
void ordersOfMagnitude;
