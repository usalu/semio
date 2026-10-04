/** 📈️ Reads the records of `hint_audit2_probe.ts` (modes `live`, `nofamiliar`, `fulllabels`) and prints the before/after figures of the
 * nine issues of the first hint audit: reversal wording, question and label length, digits and number words, familiar references,
 * profile hints, duplicates across matching columns, hints per task, leaks of the true ratio and grammar checks. */
import { readFileSync } from "node:fs";

const load = (mode: string): any[] => readFileSync(new URL(`./🗑️generated/hint-audit-2/${mode}/hints.jsonl`, import.meta.url), "utf8").split("\n").map((line) => JSON.parse(line));
const density = (mode: string): any[] => JSON.parse(readFileSync(new URL(`./🗑️generated/hint-audit-2/${mode}/density.json`, import.meta.url), "utf8"));
const pct = (n: number, d: number): string => `${n}/${d} (${d === 0 ? "-" : ((100 * n) / d).toFixed(1)}%)`;
const quant = (list: number[], q: number): number => [...list].sort((a, b) => a - b)[Math.min(list.length - 1, Math.floor(list.length * q))]!;
const by = <T,>(list: T[], key: (x: T) => string): Map<string, T[]> => {
  const map = new Map<string, T[]>();
  for (const x of list) (map.get(key(x)) ?? map.set(key(x), []).get(key(x))!).push(x);
  return map;
};

const live = load("live");
const nofam = load("nofamiliar");
const full = load("fulllabels");
const compare = live.filter((r) => r.kind === "compare");
const digitRun = (text: string): number => Math.max(0, ...(text.match(/\d[\d.,]*/gu) ?? []).map((m) => m.replace(/[.,]/gu, "").length));

console.log("## corpus", live.length, "hints,", new Set(live.map((r) => r.en)).size, "distinct EN,", new Set(live.map((r) => r.de)).size, "distinct DE; compare", compare.length, "group", live.filter((r) => r.kind === "group").length, "profile", live.filter((r) => r.kind === "profile").length);
console.log("verdicts", JSON.stringify([...by(compare, (r) => r.verdict)].map(([k, v]) => [k, v.length])));

console.log("\n## 1 reversal");
const indep = (r: any): string => {
  const pivot = r.difference !== undefined ? 0 : 1;
  const claim = r.difference !== undefined ? r.claim : r.claim;
  void pivot;
  void claim;
  return (r.claim >= 1) !== (r.truth >= 1) && r.claim !== 1 ? "reversed" : "";
};
const revIndep = compare.filter((r) => indep(r) === "reversed");
const revCore = compare.filter((r) => r.verdict === "reversed");
console.log("reversed by claim/truth side:", revIndep.length, "core verdict reversed:", revCore.length, "disagree:", compare.filter((r) => (indep(r) === "reversed") !== (r.verdict === "reversed")).length);
const bare = (text: string): string => text.replace(/“[^”]*”|„[^“]*“/gu, "X"); const orderWording = (r: any): boolean => /(is larger than|lies above) X/u.test(bare(r.en)) && /(größer ist als|über X liegt)/u.test(bare(r.de)) && !/[0-9]/u.test(bare(r.en)) && !/[0-9]/u.test(bare(r.de));
console.log("reversed hints worded as an order question without a digit:", pct(revCore.filter(orderWording).length, revCore.length));
console.log("reversed hints that mention a number outside the names:", revCore.filter((r) => /[0-9]|times as|so groß/u.test(bare(r.en + r.de))).length);
console.log("under/over hints worded 'only' / 'really':", compare.filter((r) => r.verdict !== "reversed").length, "; truth side wrong (under means truth beyond claim):", compare.filter((r) => r.verdict !== "reversed").filter((r) => { const c = r.claim >= 1 ? r.claim : r.claim; const under = c >= 1 ? r.truth > c : r.truth < c; return (r.verdict === "under") !== under; }).length);
console.log("share of old 'over'-worded hints that are now reversed: before 81% of all compare hints were over+reversed; now reversed", pct(revCore.length, compare.length), "| over", pct(compare.filter((r) => r.verdict === "over").length, compare.length), "| under", pct(compare.filter((r) => r.verdict === "under").length, compare.length));
console.log("reversed share per task:", [...by(compare, (r) => `${r.quiz}/${r.task}`)].map(([k, v]) => `${k} ${pct(v.filter((r) => r.verdict === "reversed").length, v.length)}`).join("; "));
console.log("old wording leftovers (as much as / ganze):", live.filter((r) => /as much as|ganze/u.test(r.en + r.de)).length);

console.log("\n## 2 giant numbers / digits");
const digitsEn = live.map((r) => digitRun(r.en));
const digitsDe = live.map((r) => digitRun(r.de));
console.log("max digit run EN", Math.max(...digitsEn), "DE", Math.max(...digitsDe), "| hints with a digit run >=7:", live.filter((r) => digitRun(r.en) >= 7 || digitRun(r.de) >= 7).length, "| >=5:", live.filter((r) => digitRun(r.en) >= 5).length);
const wordHints = live.filter((r) => / (million|billion|trillion)/u.test(r.en));
const powerHints = live.filter((r) => /roughly 10[⁰¹²³⁴⁵⁶⁷⁸⁹]/u.test(r.en));
console.log("hints with a scale word:", wordHints.length, "| with 'roughly 10^n':", powerHints.length, "| of all non-reversed compare", compare.filter((r) => r.verdict !== "reversed").length);
const numberPhrases = new Map<string, number>();
for (const r of live) {
  for (const m of r.en.matchAll(/([\d.,]+) (million|billion|trillion)/gu)) numberPhrases.set(`EN ${m[1]} ${m[2]}`, (numberPhrases.get(`EN ${m[1]} ${m[2]}`) ?? 0) + 1);
  for (const m of r.de.matchAll(/([\d.,]+) (Millionen|Million|Milliarden|Milliarde|Billionen|Billion)/gu)) numberPhrases.set(`DE ${m[1]} ${m[2]}`, (numberPhrases.get(`DE ${m[1]} ${m[2]}`) ?? 0) + 1);
}
console.log("number phrases:", JSON.stringify([...numberPhrases].sort()));
let agreementBad = 0;
const agreementBadList: string[] = [];
for (const r of live) for (const m of r.de.matchAll(/([\d.,]+) (Millionen|Million|Milliarden|Milliarde|Billionen|Billion)/gu)) {
  const one = m[1] === "1";
  const singular = /^(Million|Milliarde|Billion)$/u.test(m[2]!);
  if (one !== singular) { agreementBad++; agreementBadList.push(r.de); }
}
console.log("DE number-word agreement violations:", agreementBad, agreementBadList.slice(0, 3));
const pair = new Map<string, string>([["million", "Million"], ["billion", "Milliarde"], ["trillion", "Billion"]]);
let scaleMismatch = 0;
for (const r of wordHints) {
  const en = / (million|billion|trillion)/u.exec(r.en)![1]!;
  const de = / (Millionen|Million|Milliarde|Billion)/u.exec(r.de)?.[1];
  if (pair.get(en) !== de) scaleMismatch++;
}
console.log("EN/DE scale word mismatches:", scaleMismatch);
console.log("multiplier forms DE:", JSON.stringify([...by(compare.filter((r) => /-mal|Mal/u.test(r.de)), (r) => (/ Mal /u.test(r.de) ? "N Mal" : /-mal/u.test(r.de) ? "N-mal" : "other")).entries()].map(([k, v]) => [k, v.length])));
const sampleWords = (re: RegExp, n: number): string[] => [...new Set(live.filter((r) => re.test(r.de)).map((r) => r.de))].slice(0, n);
console.log("DE samples million:", sampleWords(/Million/u, 4));
console.log("DE samples Milliarde:", sampleWords(/Milliarde/u, 3));
console.log("DE samples Billion:", sampleWords(/Billion/u, 3));
console.log("DE samples rund 10:", sampleWords(/rund 10/u, 3));

console.log("\n## 3 length and labels");
const lens = (list: any[], k: "en" | "de"): number[] => list.map((r) => r[k].length);
for (const [name, data] of [["after", live], ["labels-only (full labels, same hints)", full]] as const) {
  console.log(name, "EN median", quant(lens(data, "en"), 0.5), "p90", quant(lens(data, "en"), 0.9), "max", Math.max(...lens(data, "en")), ">160", pct(lens(data, "en").filter((l) => l > 160).length, data.length), "| DE median", quant(lens(data, "de"), 0.5), "p90", quant(lens(data, "de"), 0.9), "max", Math.max(...lens(data, "de")), ">160", pct(lens(data, "de").filter((l) => l > 160).length, data.length));
}
const labelSet = (data: any[], locale: "En" | "De"): Set<string> => new Set(data.flatMap((r) => [r["item" + locale], r["other" + locale]]).filter(Boolean));
for (const [name, data] of [["after (short ?? label)", live], ["before-ish (full labels)", full]] as const) {
  const en = labelSet(data, "En");
  const de = labelSet(data, "De");
  const all = [...en, ...de];
  console.log(name, "distinct EN labels", en.size, "max len", Math.max(...[...en].map((l) => l.length)), "DE max", Math.max(...[...de].map((l) => l.length)), "| >40:", all.filter((l) => l.length > 40).length, "| >60:", all.filter((l) => l.length > 60).length, "| parentheses:", all.filter((l) => /[()]/u.test(l)).length, "| digit start:", all.filter((l) => /^\d/u.test(l)).length);
}
console.log("compare hints with an item named by a label longer than 40:", live.filter((r) => [r.itemEn, r.otherEn, r.itemDe, r.otherDe].some((l) => l && l.length > 40)).length);
console.log("labels with 'near-twin' parentheses styles: n/a (shorts have no parentheses)");

console.log("\n## 4 familiar references");
const share = (data: any[]): string => {
  const c = data.filter((r) => r.kind === "compare");
  return pct(c.filter((r) => r.otherFamiliar).length, c.length);
};
console.log("compare hints whose reference is familiar: live", share(live), "| counterfactual without flags", share(nofam), "(familiar flags stripped from the same quizzes; reference rule otherwise as now)");
console.log("compare hints whose hinted item is familiar:", pct(compare.filter((r) => r.itemFamiliar).length, compare.length));
const refStats = (data: any[]): string[] => {
  const out: string[] = [];
  for (const [task, list] of by(data.filter((r) => r.kind === "compare"), (r) => `${r.quiz}/${r.task}`)) {
    const counts = new Map<string, number>();
    for (const r of list) counts.set(r.otherEn, (counts.get(r.otherEn) ?? 0) + 1);
    const top = [...counts].sort((a, b) => b[1] - a[1])[0]!;
    out.push(`${task}: ${counts.size} refs, top ${((100 * top[1]) / list.length).toFixed(0)}% ${top[0]}`);
  }
  return out;
};
console.log("references per task (live):\n  " + refStats(live).join("\n  "));
console.log("references per task (no flags):\n  " + refStats(nofam).join("\n  "));
const answerKey = (r: any): string => `${r.quiz}/${r.task}/${r.seed}/${r.flaw}`;
const reuse = (data: any[]): string => {
  const answers = [...by(data.filter((r) => r.kind === "compare"), answerKey)].filter(([, v]) => v.length >= 2);
  return pct(answers.filter(([, v]) => new Set(v.map((r) => `${r.dimension ?? ""}|${r.other}`)).size < v.length).length, answers.length);
};
console.log("multi-hint answers re-using a reference (same dimension): live", reuse(live), "| no flags", reuse(nofam));
const mirror = (data: any[]): string => {
  const answers = by(data.filter((r) => r.kind === "compare"), answerKey);
  let pairs = 0, mirrors = 0;
  for (const list of answers.values()) {
    const set = new Set(list.map((r) => `${r.dimension ?? ""}|${r.item}|${r.other}`));
    for (const r of list) { pairs++; if (set.has(`${r.dimension ?? ""}|${r.other}|${r.item}`)) mirrors++; }
  }
  return pct(mirrors, pairs);
};
console.log("mirrored pairs:", mirror(live));
console.log("group hints: other familiar", pct(live.filter((r) => r.kind === "group" && r.otherFamiliar).length, live.filter((r) => r.kind === "group").length), "| item familiar", pct(live.filter((r) => r.kind === "group" && r.itemFamiliar).length, live.filter((r) => r.kind === "group").length));
console.log("claim < 3x (near-equal claims):", pct(compare.filter((r) => Math.max(r.claim, 1 / r.claim) < 3).length, compare.length), "| error claim/truth quantiles min/10%/median:", (() => { const e = compare.map((r) => Math.max(r.claim / r.truth, r.truth / r.claim)); return [Math.min(...e), quant(e, 0.1), quant(e, 0.5)].map((x) => Number(x.toPrecision(3))); })());

console.log("\n## 5 profile hints");
const profile = live.filter((r) => r.kind === "profile");
console.log("profile hints", profile.length, "distinct EN", new Set(profile.map((r) => r.en)).size, "| naming another item (relative):", pct(profile.filter((r) => r.other).length, profile.length), "| value form:", pct(profile.filter((r) => !r.other).length, profile.length));
console.log("profile above/below:", JSON.stringify([...by(profile.filter((r) => r.other), (r) => String(r.above))].map(([k, v]) => [k, v.length])), "| hyphen minus:", profile.filter((r) => /(^|\s)-\d/u.test(r.en + r.de)).length, "| U+2212:", profile.filter((r) => /−\d/u.test(r.en)).length);
console.log("profile axes:", JSON.stringify([...by(profile, (r) => r.axis)].map(([k, v]) => [k, v.length])));
console.log("profile samples (distinct):\n  " + [...new Set(profile.map((r) => r.en + "\n  " + r.de))].slice(0, 12).join("\n  "));
const profileValueDistinct = profile.filter((r) => !r.other);
console.log("value-form recurring numbers:", JSON.stringify([...by(profileValueDistinct, (r) => (r.en.match(/about (−?[\d.,]+)/u) ?? [])[1] ?? "?")].map(([k, v]) => [k, v.length])));

console.log("\n## 6 duplicates across columns");
const matching = [...by(live.filter((r) => r.dimension), answerKey)];
let dup = 0, dupAnswers = 0;
for (const [, list] of matching) { const d = list.length - new Set(list.map((r) => r.en)).size; if (d > 0) { dupAnswers++; dup += d; } }
console.log("two-dimension answers with hints:", matching.length, "| with identical questions in both columns:", pct(dupAnswers, matching.length), "| identical EN pairs total", dup);
const dupDe = matching.filter(([, list]) => list.length !== new Set(list.map((r) => r.de)).size).length;
console.log("same in DE:", dupDe);
const multiDim = live.filter((r) => r.dimension && r.dims > 1);
console.log("two-dimension hints naming the quantity ('in <q>' / 'in puncto'):", pct(multiDim.filter((r) => / in [A-Za-z]/u.test(r.en.replace(/“[^”]*”/gu, "")) && /in puncto/u.test(r.de)).length, multiDim.length));
console.log("quantity-name samples:", [...new Set(multiDim.map((r) => r.en.replace(/^.*” (?:is|lies|really)?/u, "").slice(-60)))].slice(0, 6));

console.log("\n## 7 hints per task");
const d = density("live");
const hist = new Map<number, number>();
for (const x of d) hist.set(x.hints, (hist.get(x.hints) ?? 0) + 1);
console.log("hints per answer histogram (all answers):", JSON.stringify([...hist].sort()), "of", d.length, "| max", Math.max(...d.map((x) => x.hints)));
const byFlaw = by(d, (x) => x.flaw);
for (const [flaw, list] of byFlaw) console.log(`  ${flaw.padEnd(9)} mean ${(list.reduce((a, b) => a + b.hints, 0) / list.length).toFixed(2)} max ${Math.max(...list.map((x) => x.hints))} zero ${pct(list.filter((x) => x.hints === 0).length, list.length)} capped(=3) ${pct(list.filter((x) => x.hints === 3).length, list.length)}`);
const randomAnswers = d.filter((x) => x.flaw === "random");
console.log("random answers: mean wrong", (randomAnswers.reduce((a, b) => a + b.wrong, 0) / randomAnswers.length).toFixed(1), "mean hints", (randomAnswers.reduce((a, b) => a + b.hints, 0) / randomAnswers.length).toFixed(2));
for (const [key, list] of by(d, (x) => `${x.quiz}/${x.task}`)) console.log(`  ${key.padEnd(46)} adjacent ${(d.filter((x) => `${x.quiz}/${x.task}` === key && x.flaw === "adjacent").reduce((a, b) => a + b.hints, 0) / (list.length / 6)).toFixed(2)}  swapFar ${(d.filter((x) => `${x.quiz}/${x.task}` === key && x.flaw === "swapFar").reduce((a, b) => a + b.hints, 0) / (list.length / 6)).toFixed(2)}  random ${(d.filter((x) => `${x.quiz}/${x.task}` === key && x.flaw === "random").reduce((a, b) => a + b.hints, 0) / (list.length / 6)).toFixed(2)}`);
const wrongNoHint = d.filter((x) => x.flaw !== "adjacent" && x.wrong >= 4 && x.hints === 0);
console.log("answers with >=4 wrong cells and no hint at all (non-adjacent):", wrongNoHint.length, "of", d.filter((x) => x.wrong >= 4).length, JSON.stringify(by(wrongNoHint, (x) => `${x.quiz}/${x.task}`).size));

console.log("\n## 8 leaks of the true ratio");
const numberShown = compare.filter((r) => r.verdict !== "reversed");
const leak = numberShown.filter((r) => Math.abs(Math.log(r.claim / r.truth)) < Math.log(1.1) || Math.abs(Math.log(r.claim * r.truth)) < Math.log(1.1));
console.log("non-reversed hints:", numberShown.length, "| shown claim within 10% of truth or its inverse:", leak.length, leak.slice(0, 3).map((r) => r.en));
const minErr = Math.min(...compare.filter((r) => r.difference === undefined).map((r) => Math.max(r.claim / r.truth, r.truth / r.claim)));
console.log("min claim/truth error over all compare hints:", Number(minErr.toPrecision(3)));
console.log("reversed hints with digits in text outside the names:", revCore.filter((r) => /[0-9]/u.test(bare(r.en + r.de))).length);
console.log("any hint stating the truth: a reversed hint states only the claimed order; the true order is not named");

console.log("\n## 9 grammar and phrasing");
console.log("hint starts with the hinted item (compare, EN text begins with item label):", pct(compare.filter((r) => r.en.startsWith(`Are you sure “${r.itemEn}”`) || r.en.startsWith(`Are you sure it takes`) && false).length, compare.length), "| begins with the reference:", pct(compare.filter((r) => r.en.startsWith(`Are you sure “${r.otherEn}”`)).length, compare.length), "| additive (item inside a sum):", pct(compare.filter((r) => r.en.startsWith("Are you sure ") && !r.en.startsWith("Are you sure “")).length, compare.length));
const forms = by(live, (r) => r.en.replace(/“[^”]*”/gu, "“…”").replace(/[\d.,]+( (million|billion|trillion))?|roughly 10[⁰¹²³⁴⁵⁶⁷⁸⁹]+/gu, "N").replace(/Profile [A-Z]/u, "Profile X").replace(/ in [A-Za-z][^?]*\?$/u, " in Q?"));
console.log("distinct EN templates:", forms.size);
for (const [k, v] of [...forms].sort((a, b) => b[1].length - a[1].length)) console.log(`  ${String(v.length).padStart(4)}  ${k}`);
const formsDe = by(live, (r) => r.de.replace(/„[^“]*“/gu, "„…“").replace(/[\d.,]+( (Millionen|Million|Milliarden|Milliarde|Billionen|Billion))?|rund 10[⁰¹²³⁴⁵⁶⁷⁸⁹]+/gu, "N").replace(/Profil [A-Z]/u, "Profil X").replace(/in puncto [^?]*? (nur|tatsächlich|größer|über|unter|zusammen|braucht)/u, "in puncto Q $1"));
console.log("distinct DE templates:", formsDe.size);
for (const [k, v] of [...formsDe].sort((a, b) => b[1].length - a[1].length)) console.log(`  ${String(v.length).padStart(4)}  ${k}`);
