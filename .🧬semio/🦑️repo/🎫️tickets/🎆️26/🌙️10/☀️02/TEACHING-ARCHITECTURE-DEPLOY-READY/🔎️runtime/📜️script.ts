import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";

const root = process.cwd();
const product = resolve(root, "🧰️framework/🛍️products/📚️course");
const site = resolve(root, "🎓️teaching/🏛️architecture/📚️course");
const load = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const { courseIssues, quizIssues } = await import(pathToFileURL(resolve(product, "🔨️modules/✅️validation/🟦️.ts")).href);
const { sheetOf } = await import(pathToFileURL(resolve(product, "🔨️modules/🃏️sheet/🟦️.ts")).href);
const { scoreRun } = await import(pathToFileURL(resolve(product, "🔨️modules/📏️scoring/🟦️.ts")).href);
const schema = load(resolve(product, "🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validators = Object.fromEntries(["Course", "Quiz", "Sheet", "RunResult"].map(name => [name, ajv.compile({ $ref: `${schema.$id}#/$defs/${name}` })]));
const valid = (name: string, value: unknown) => assert.equal(validators[name]!(value), true, JSON.stringify(validators[name]!.errors));
const course = load(resolve(site, "🔣️.json"));
const quizzes = course.quizzes.map((file: string) => load(resolve(site, file)));
const { ARCHITECTURE_COURSE_MATERIAL } = await import(pathToFileURL(resolve(site, "📚️course/🟦️.ts")).href);
assert.deepEqual(ARCHITECTURE_COURSE_MATERIAL, { course, quizzes });
assert.deepEqual(courseIssues(course, quizzes), []);
valid("Course", course);
console.log(`[DEBUG] architecture course owned validation + AJV PASS (${quizzes.length} quizzes)`);
let attempts = 0;
let tasks = 0;
for (const quiz of quizzes) {
  assert.deepEqual(quizIssues(quiz), []);
  valid("Quiz", quiz);
  for (const challenge of ["easy", "medium", "hard", "expert"]) {
    const seen = new Map<string, Set<string>>();
    for (let attempt = 0; attempt < 5; attempt++) {
      const sheet = sheetOf(quiz, (0x73510000 | attempt) >>> 0, challenge);
      valid("Sheet", sheet);
      const answers = Object.fromEntries(sheet.tasks.map((task: any) => {
        const source = quiz.tasks.find((candidate: any) => candidate.id === task.id);
        const items = task.items.map((item: any) => source.items.find((known: any) => known.id === item.id));
        const previous = seen.get(task.id) ?? new Set<string>();
        assert.equal(items.some((item: any) => previous.has(item.id)), false);
        assert.equal(items.length, source.draw);
        for (const slot of source.pool.slots) assert.equal(items.filter((item: any) => slot.includes(item.id)).length, 1);
        for (const item of items) previous.add(item.id);
        seen.set(task.id, previous);
        assert.equal(task.seconds, challenge === "expert" ? 300 : undefined);
        tasks++;
        if (task.kind === "classification") return [task.id, { kind: task.kind, assignments: Object.fromEntries(items.map((item: any) => [item.id, item.category])) }];
        if (task.kind === "sorting") return [task.id, { kind: task.kind, order: [...items].sort((a: any, b: any) => a.value - b.value).map((item: any) => item.id), ...(task.keys ? {} : { guesses: Object.fromEntries(items.map((item: any) => [item.id, item.value])) }) }];
        const hidden = task.dimensions.some((dimension: any) => dimension.cards === undefined);
        return [task.id, { kind: task.kind, [hidden ? "guesses" : "assignments"]: Object.fromEntries(task.dimensions.map((dimension: any) => {
          const used = new Set<number>();
          return [dimension.id, Object.fromEntries(items.map((item: any) => {
            if (hidden) return [item.id, item.values[dimension.id]];
            const index = dimension.cards.findIndex((value: number, index: number) => value === item.values[dimension.id] && !used.has(index));
            assert.notEqual(index, -1);
            used.add(index);
            return [item.id, index];
          }))];
        })) }];
      }));
      const result = scoreRun(quiz, sheet, answers);
      assert.ok(result);
      valid("RunResult", result);
      assert.equal(result.score, 1);
      assert.equal(result.points, { easy: 100, medium: 200, hard: 300, expert: 400 }[challenge]);
      attempts++;
    }
  }
  console.log(`[DEBUG] ${quiz.id} owned validation + AJV + all four challenges with five balanced perfect runs PASS`);
}
console.log(`[DEBUG] narrow course data check PASS: ${attempts} scored runs, ${tasks} task presentations; full managed tests and browser behavior remain unverified`);
