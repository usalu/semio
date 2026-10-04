/** 👯️ Counts identical questions asked twice in one answer (two dimensions) and the largest digit counts per locale. */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
const answers = new Map<string, any[]>();
for (const r of records) (answers.get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`) ?? answers.set(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`, []).get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`)!).push(r);
let dup = 0, two = 0, answersWithHints = 0;
const perTask = new Map<string, [number, number]>();
for (const [key, list] of answers) {
  answersWithHints++;
  const t = key.split("/").slice(0, 2).join("/");
  if (!list.some((r) => r.dimension)) continue;
  two++;
  const texts = list.map((r) => r.en);
  const d = texts.length - new Set(texts).size;
  const c = perTask.get(t) ?? [0, 0]; c[1]++; if (d > 0) c[0]++; perTask.set(t, c);
  if (d > 0) dup++;
}
console.log("matching answers with hints:", two, "with an identical question asked in both dimensions:", dup, JSON.stringify([...perTask]));
const nums = records.flatMap((r) => (r.de.match(/\d[\d.]*\d/gu) ?? []).map((m: string) => ({ n: m.length, r })));
nums.sort((a, b) => b.n - a.n);
console.log("max chars in a DE number:", nums[0]!.n, nums[0]!.r.de);
const flawStats = new Map<string, number>();
for (const r of records) flawStats.set(r.flaw, (flawStats.get(r.flaw) ?? 0) + 1);
console.log(JSON.stringify([...flawStats]));
const distinctPerKind = new Map<string, Set<string>>();
for (const r of records) (distinctPerKind.get(r.kind) ?? distinctPerKind.set(r.kind, new Set()).get(r.kind)!).add(r.en);
console.log(JSON.stringify([...distinctPerKind].map(([k, v]) => [k, v.size])));
