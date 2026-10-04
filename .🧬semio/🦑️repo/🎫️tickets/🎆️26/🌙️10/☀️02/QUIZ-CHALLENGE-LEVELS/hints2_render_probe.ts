/** 🧪️ Renders the core's round-2 hints of random easy answers over the four live energy quizzes with the React target's own
 * `compareText` and `classificationHintText` in English and German, and prints counts, the longest question, rule breaks
 * (raw digits beyond six, hyphen minus, leftovers of the old wording) and samples. Usage: `bun hints2_render_probe.ts`. */
import { readFileSync } from "node:fs";
import { hintsOf, sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { compareText, hintName } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx";
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

const questions = new Map<string, { en: string; de: string }>();
const counts = new Map<string, number>();
for (const quiz of quizzes)
  for (let seed = 1; seed <= 40; seed++) {
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
        const say = (locale: "en" | "de"): string => {
          const text = quizText(locale);
          const named = (id: string): string => hintName((sheetTask as any).items.find((item: any) => item.id === id), locale);
          if (hint.kind !== "compare") return classificationHintText(sheetTask as any, hint, text, locale);
          if (sheetTask.kind === "sorting") return compareText(hint, sheetTask.quantity, named, text, locale);
          const dimension = (sheetTask as any).dimensions.find((candidate: any) => candidate.id === hint.dimension);
          return compareText(hint, dimension.quantity, named, text, locale, (sheetTask as any).dimensions.length > 1 ? hintName(dimension.quantity, locale) : undefined);
        };
        const key = `${hint.kind}${hint.kind === "compare" ? `:${hint.verdict}:${hint.factor === undefined ? "difference" : "factor"}` : hint.kind === "profile" ? `:${hint.other === undefined ? "value" : "relative"}` : ""}`;
        counts.set(key, (counts.get(key) ?? 0) + 1);
        const en = say("en");
        questions.set(en, { en, de: say("de") });
      }
    }
  }

const all = [...questions.values()];
console.log("[DEBUG] hints by kind", Object.fromEntries(counts));
console.log("[DEBUG] distinct questions", all.length);
const longest = all.reduce((best, entry) => (entry.en.length > best.en.length ? entry : best), all[0]!);
console.log("[DEBUG] longest EN", longest.en.length, longest.en);
const breaks = all.filter((entry) => [entry.en, entry.de].some((question) => /\d{7}|\d[,.]\d{3}[,.]\d{3}|(^|\s)-\d|as much as|ganze|\{\{|quiz\./u.test(question)));
console.log("[DEBUG] rule breaks", breaks.length, breaks.slice(0, 5));
for (const pattern of [/larger than|lies above “/u, /million|billion|trillion/u, /roughly 10/u, / in [a-zA-Z]/u, /lies (above|below) “.*” in /u, /fits /u, /same category|different categories/u, /belongs to/u])
  console.log("[DEBUG] sample", pattern.source, JSON.stringify(all.find((entry) => pattern.test(entry.en)) ?? null));
