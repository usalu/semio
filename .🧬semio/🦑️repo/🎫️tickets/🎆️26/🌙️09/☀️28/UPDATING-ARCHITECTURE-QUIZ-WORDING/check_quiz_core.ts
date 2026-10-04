/** 🧪️ Cross-checks the architecture quiz content against the owned TS quiz core: no validation issue, sheets for many seeds
 * honour the draws, perfect answers score exactly 1, reversed sortings score 0, and seeded random answers stay in [0, 1].
 * Usage (repo root): bun "<ticket>/check_quiz_core.ts"
 * @see ../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/✅️validation/🟦️.ts */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { catalogIssues, quizIssues } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/✅️validation/🟦️.ts";
import { sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts";
import { scoreRun } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/📏️scoring/🟦️.ts";
import { Mt19937 } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🎲️randomness/🟦️.ts";
import type { Answer, Catalog, Quiz, Sheet } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const catalogPath = resolve(root, "🎓️teaching/🏛️architecture/❓️quiz/🔣️.json");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const catalog = read(catalogPath) as Catalog;
const quizzes = catalog.quizzes.map((path) => read(resolve(dirname(catalogPath), path)) as Quiz);
const failures: string[] = [];
const fail = (text: string) => failures.push(text);

const catalogProblems = catalogIssues(catalog, quizzes);
console.log(`catalogIssues: ${catalogProblems.length === 0 ? "none" : JSON.stringify(catalogProblems)}`);
if (catalogProblems.length) fail("catalog issues");

function answers(quiz: Quiz, sheet: Sheet, mode: "perfect" | "reversed" | "random", random: Mt19937): Record<string, Answer> {
  const result: Record<string, Answer> = {};
  for (const sheetTask of sheet.tasks) {
    const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id)!;
    if (sheetTask.kind === "classification" && task.kind === "classification") {
      const assignments: Record<string, string> = {};
      for (const item of sheetTask.items) {
        const truth = task.items.find((candidate) => candidate.id === item.id)!.category;
        assignments[item.id] = mode === "perfect" ? truth : sheetTask.categories[random.next() % sheetTask.categories.length]!.id;
      }
      result[sheetTask.id] = { kind: "classification", assignments };
    } else if (sheetTask.kind === "sorting" && task.kind === "sorting") {
      const value = (id: string) => task.items.find((candidate) => candidate.id === id)!.value;
      const ids = sheetTask.items.map((item) => item.id);
      const order = mode === "random" ? ids : [...ids].sort((a, b) => value(a) - value(b));
      result[sheetTask.id] = { kind: "sorting", order: mode === "reversed" ? order.reverse() : order };
    } else if (sheetTask.kind === "matching" && task.kind === "matching") {
      const assignments: Record<string, Record<string, number>> = {};
      for (const dimension of sheetTask.dimensions) {
        const used = new Set<number>();
        const byItem: Record<string, number> = {};
        const items = mode === "random" ? sheetTask.items : sheetTask.items;
        const free = dimension.cards.map((_, index) => index);
        for (const item of items) {
          const truth = task.items.find((candidate) => candidate.id === item.id)!.values[dimension.id]!;
          const index = mode === "perfect" ? dimension.cards.findIndex((card, at) => card === truth && !used.has(at)) : free.splice(random.next() % free.length, 1)[0]!;
          used.add(index);
          byItem[item.id] = index;
        }
        assignments[dimension.id] = byItem;
      }
      result[sheetTask.id] = { kind: "matching", assignments };
    }
  }
  return result;
}

for (const quiz of quizzes) {
  const issues = quizIssues(quiz);
  console.log(`\n${quiz.id}: quizIssues ${issues.length === 0 ? "none" : JSON.stringify(issues)}`);
  if (issues.length) fail(`${quiz.id} issues`);
  const random = new Mt19937(20260928);
  const randomScores: number[] = [];
  for (let run = 0; run < 200; run++) {
    const seed = random.next();
    const sheet = sheetOf(quiz, seed);
    for (const sheetTask of sheet.tasks) {
      const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id)!;
      const expected = task.draw !== undefined && task.draw < task.items.length ? task.draw : task.items.length;
      if (sheetTask.items.length !== expected) fail(`${quiz.id}/${task.id} seed ${seed}: ${sheetTask.items.length} items, expected ${expected}`);
    }
    const perfect = scoreRun(quiz, sheet, answers(quiz, sheet, "perfect", random)).score;
    if (perfect !== 1) fail(`${quiz.id} seed ${seed}: perfect answers scored ${perfect}`);
    const reversed = scoreRun(quiz, sheet, answers(quiz, sheet, "reversed", random));
    for (const task of reversed.tasks) if (task.kind === "sorting" && task.score !== 0) fail(`${quiz.id}/${task.task} seed ${seed}: reversed sorting scored ${task.score}`);
    const scored = scoreRun(quiz, sheet, answers(quiz, sheet, "random", random)).score;
    if (!(scored >= 0 && scored <= 1)) fail(`${quiz.id} seed ${seed}: random answers scored ${scored}`);
    randomScores.push(scored);
  }
  const mean = randomScores.reduce((sum, score) => sum + score, 0) / randomScores.length;
  const first = sheetOf(quiz, 1);
  console.log(`  200 seeded runs: perfect = 1, reversed sortings = 0, random answers mean score ${mean.toFixed(3)} (min ${Math.min(...randomScores).toFixed(3)}, max ${Math.max(...randomScores).toFixed(3)})`);
  console.log(`  sheet(seed 1) task order: ${first.tasks.map((task) => `${task.id}[${task.items.length}]`).join(", ")}`);
}

console.log(`\nfailures: ${failures.length}`);
for (const failure of failures.slice(0, 20)) console.log(`  ✗ ${failure}`);
process.exit(failures.length ? 1 : 0);
