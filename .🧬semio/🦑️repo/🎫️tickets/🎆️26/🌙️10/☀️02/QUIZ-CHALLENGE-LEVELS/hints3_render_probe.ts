/** 🧪️ Renders the core's hints of random easy answers over the four live energy quizzes with the React target's round-3
 * `compareText` and `classificationHintText` in English and German, and prints counts, the longest questions, rule breaks
 * (a compare question without its quantity, a capitalised quantity or axis mid-sentence in English, a bare power of ten,
 * raw digits beyond six, hyphen minus, leftovers of older wordings) and one sample per form. Usage: `bun hints3_render_probe.ts`. */
import { readFileSync } from "node:fs";
import { hintsOf, sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { compareText, hintName, hintTerm } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx";
import { classificationHintText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🗂️classification/🟦️.tsx";
import { quizText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";

const ENERGY = new URL("../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/", import.meta.url);
const quizzes = ["🧲️physics", "🔥️heating", "❄️cooling", "📊️demand"].map((folder) => JSON.parse(readFileSync(new URL(`${folder}/❓️quiz/🔣️.json`, ENERGY), "utf8")));
let state = 7;
const random = (): number => ((state = (state * 1103515245 + 12345) % 2147483648) / 2147483648);
const shuffled = <T,>(list: readonly T[]): T[] => {
  const copy = [...list];
  for (let i = copy.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [copy[i], copy[j]] = [copy[j]!, copy[i]!];
  }
  return copy;
};

type Entry = { readonly form: string; readonly en: string; readonly de: string; readonly term?: { readonly en: string; readonly de: string } };
const questions = new Map<string, Entry>();
const counts = new Map<string, number>();
for (const quiz of quizzes)
  for (let seed = 1; seed <= 60; seed++) {
    const sheet = sheetOf(quiz, seed, "easy");
    for (const sheetTask of sheet.tasks) {
      const task = quiz.tasks.find((candidate: any) => candidate.id === sheetTask.id);
      const answer: any =
        sheetTask.kind === "sorting"
          ? { kind: "sorting", order: shuffled(sheetTask.items.map((item: any) => item.id)) }
          : sheetTask.kind === "matching"
            ? { kind: "matching", assignments: Object.fromEntries(sheetTask.dimensions.map((dimension: any) => [dimension.id, Object.fromEntries(shuffled(sheetTask.items.map((item: any) => item.id)).map((id, index) => [id, index]))])) }
            : { kind: "classification", assignments: Object.fromEntries(sheetTask.items.map((item: any) => [item.id, sheetTask.categories[Math.floor(random() * sheetTask.categories.length)].id])) };
      for (const hint of hintsOf(task, sheetTask as any, answer)) {
        const quantity = hint.kind !== "compare" ? undefined : sheetTask.kind === "sorting" ? (sheetTask as any).quantity : (sheetTask as any).dimensions.find((candidate: any) => candidate.id === hint.dimension).quantity;
        const axis = hint.kind === "profile" ? (sheetTask as any).axes.find((candidate: any) => candidate.id === hint.axis) : undefined;
        const say = (locale: "en" | "de"): string => {
          const text = quizText(locale);
          const named = (id: string): string => hintName((sheetTask as any).items.find((item: any) => item.id === id), locale);
          return hint.kind === "compare" ? compareText(hint, quantity, named, text, locale) : classificationHintText(sheetTask as any, hint, text, locale);
        };
        const form = `${hint.kind}${hint.kind === "compare" ? `:${hint.verdict}:${hint.factor === undefined ? "difference" : quantity.additive ? "sum" : "ratio"}` : hint.kind === "profile" ? `:${hint.other === undefined ? "value" : hint.above ? "higher" : "lower"}` : ""}`;
        counts.set(form, (counts.get(form) ?? 0) + 1);
        const named = quantity ?? axis;
        const en = say("en");
        questions.set(en, { form, en, de: say("de"), term: named === undefined ? undefined : { en: hintTerm(named, "en"), de: hintTerm(named, "de") } });
      }
    }
  }

const all = [...questions.values()];
console.log("[DEBUG] hints by form", Object.fromEntries(counts));
console.log("[DEBUG] distinct questions", all.length, "compare", all.filter((entry) => entry.form.startsWith("compare")).length);
for (const locale of ["en", "de"] as const) {
  const lengths = all.map((entry) => entry[locale].length).sort((a, b) => a - b);
  console.log(`[DEBUG] ${locale} length median ${lengths[Math.floor(lengths.length / 2)]} p90 ${lengths[Math.floor(lengths.length * 0.9)]} max ${lengths.at(-1)}`);
  const longest = all.reduce((best, entry) => (entry[locale].length > best[locale].length ? entry : best), all[0]!);
  console.log(`[DEBUG] longest ${locale}`, longest[locale]);
}
const unnamed = all.filter((entry) => entry.term !== undefined && (!entry.en.includes(entry.term.en) || !entry.de.includes(entry.term.de)));
console.log("[DEBUG] question without its quantity or axis", unnamed.length, unnamed.slice(0, 3));
const capitalised = all.filter((entry) => entry.term !== undefined && /^\p{Lu}\p{Ll}/u.test(entry.term.en));
console.log("[DEBUG] EN term capitalised mid-sentence", capitalised.length, capitalised.slice(0, 3));
const terms = new Set(all.flatMap((entry) => (entry.term === undefined ? [] : [`${entry.term.en} / ${entry.term.de}`])));
console.log("[DEBUG] terms used", [...terms]);
const breaks = all.filter((entry) => [entry.en, entry.de].some((question) => /\d{7}|\d[,.]\d{3}[,.]\d{3}|(^|\s)-\d|as much as|ganze|roughly|rund 10|(?<!×)(^|\s)10[⁰¹²³⁴⁵⁶⁷⁸⁹]|\{\{|quiz\.|is larger than|größer ist als|lies (above|below)|so groß/u.test(question)));
console.log("[DEBUG] rule breaks", breaks.length, breaks.slice(0, 5));
const powers = all.filter((entry) => /× 10[⁰¹²³⁴⁵⁶⁷⁸⁹]/u.test(entry.en));
console.log("[DEBUG] power-of-ten questions", powers.length, powers.slice(0, 3));
for (const form of [...counts.keys()].sort()) {
  const sample = all.find((entry) => entry.form === form);
  console.log(`[DEBUG] sample ${form}\n  EN ${sample?.en}\n  DE ${sample?.de}`);
}
