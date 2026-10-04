/** 🧮️ Extra checks of the round-2 hint audit over the corpora of `hint_audit2_probe.ts`: the item named first is the one the learner's keys make larger,
 * identical names in one question, answers with many wrong cells and no hint, three-hint answers on one reference, mid-sentence capitals,
 * single-quantity tasks that never name their quantity, the shown power of ten against the claim, `bun hint_audit2_extra.ts [corpus]`. */
import { readFileSync } from "node:fs";

const load = (name: string): any[] => readFileSync(new URL(`./🗑️generated/hint-audit-2/${name}/hints.jsonl`, import.meta.url), "utf8").split("\n").map((line) => JSON.parse(line));
const corpus = process.argv[2] ?? "live";
const records = [...load(corpus), ...(corpus === "live" ? load("live-101") : [])];
const compare = records.filter((r) => r.kind === "compare");
const pct = (n: number, d: number): string => `${n}/${d} (${d === 0 ? "-" : ((100 * n) / d).toFixed(1)}%)`;

const firstIsLarger = compare.filter((r) => r.verdict === "reversed").filter((r) => {
  const firstItem = r.en.startsWith(`Are you sure “${r.itemEn}”`);
  return r.claim >= 1 ? !firstItem : firstItem;
});
console.log("reversed hints whose first-named item is not the learner's larger:", firstIsLarger.length, "of", compare.filter((r) => r.verdict === "reversed").length);
const fewer = compare.filter((r) => r.verdict !== "reversed").filter((r) => {
  const first = r.en.startsWith(`Are you sure “${r.itemEn}”`) || r.en.includes(`× “${r.itemEn}”`);
  return false && first;
});
void fewer;
console.log("questions naming one label twice:", records.filter((r) => r.otherEn !== undefined && r.itemEn === r.otherEn).length);

const noQuantity = compare.filter((r) => !(r.dims > 1));
console.log("compare hints in single-quantity tasks (quantity not named):", pct(noQuantity.length, compare.length), JSON.stringify([...new Set(noQuantity.map((r) => `${r.quiz}/${r.task}`))]));
const physicalSize = noQuantity.filter((r) => /^(heating|cooling|demand)\//u.test(`${r.quiz}/${r.task}`));
console.log("... of which in tasks whose items are buildings, rooms or components where 'larger' reads as physical size:", pct(physicalSize.length, compare.length));

const capital = records.filter((r) => /(in|with) [A-Z][a-z]+ [a-z]+/u.test(r.en.replace(/“[^”]*”/gu, "X")) && !/Profile [A-Z]\b/u.test(r.en.replace(/“[^”]*”/gu, "X").replace(/Profile [A-Z]/u, "")));
console.log("EN questions with a capitalised quantity/axis in mid-sentence:", pct(records.filter((r) => /( in | with )(Heating|Cooling|Net|Ventilation) /u.test(r.en)).length, records.length), capital.length);

const shownPower = compare.filter((r) => /roughly 10[⁰¹²³⁴⁵⁶⁷⁸⁹]/u.test(r.en));
const sup = "⁰¹²³⁴⁵⁶⁷⁸⁹";
const exp = (text: string): number => Number([...(text.match(/10([⁰¹²³⁴⁵⁶⁷⁸⁹]+)/u)?.[1] ?? "")].map((c) => sup.indexOf(c)).join(""));
const offs = shownPower.map((r) => { const shown = Math.pow(10, exp(r.en)); const claim = r.additive ? (r.claim >= 1 ? r.claim : 1 / r.claim) : Math.max(r.claim, 1 / r.claim); return Math.max(shown / claim, claim / shown); });
console.log("power-of-ten hints:", shownPower.length, "| shown vs the learner's own claim, factor off: median", Number([...offs].sort((a, b) => a - b)[Math.floor(offs.length / 2)]!.toPrecision(3)), "max", Number(Math.max(...offs).toPrecision(3)), "| more than 3x off:", pct(offs.filter((o) => o > 3).length, offs.length));
const words = compare.filter((r) => / (million|billion|trillion) ×/u.test(r.en));
console.log("scale-word hints:", words.length);

const answers = new Map<string, any[]>();
for (const r of compare) (answers.get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`) ?? answers.set(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`, []).get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`)!).push(r);
const same = [...answers.values()].filter((list) => list.length === 3 && new Set(list.map((r) => `${r.dimension ?? ""}|${r.other}`)).size === 1);
console.log("answers with 3 compare hints all against one reference:", same.length, "of", [...answers.values()].filter((l) => l.length === 3).length);
for (const list of same.slice(0, 2)) console.log(list.map((r) => "   " + r.en).join("\n"), "\n");

const density = JSON.parse(readFileSync(new URL(`./🗑️generated/hint-audit-2/${corpus}/density.json`, import.meta.url), "utf8")) as any[];
const none = density.filter((x) => x.flaw !== "adjacent" && x.wrong >= 4 && x.hints === 0);
const tally = new Map<string, number>();
for (const x of none) tally.set(`${x.flaw}/${x.quiz}/${x.task}`, (tally.get(`${x.flaw}/${x.quiz}/${x.task}`) ?? 0) + 1);
console.log("answers with >=4 wrong cells and no hint (non-adjacent):", none.length, "of", density.filter((x) => x.flaw !== "adjacent" && x.wrong >= 4).length, JSON.stringify([...tally].sort((a, b) => b[1] - a[1]).slice(0, 8)));
const byFlaw = new Map<string, number[]>();
for (const x of none) (byFlaw.get(x.flaw) ?? byFlaw.set(x.flaw, []).get(x.flaw)!).push(x.wrong);
console.log("by flaw:", JSON.stringify([...byFlaw].map(([k, v]) => [k, v.length, Math.max(...v)])));

const groups = records.filter((r) => r.kind === "group");
console.log("group hints:", groups.length, "| distinct pairs:", new Set(groups.map((r) => [r.item, r.other].sort().join("|"))).size);
const prof = records.filter((r) => r.kind === "profile" && !r.other);
console.log("profile value-form hints:", prof.length, "| naming the most common value:", JSON.stringify([...(() => { const m = new Map<string, number>(); for (const r of prof) { const v = r.en.match(/about (−?[\d.,]+)/u)?.[1] ?? "?"; m.set(v, (m.get(v) ?? 0) + 1); } return m; })()].sort((a, b) => b[1] - a[1]).slice(0, 3)));
