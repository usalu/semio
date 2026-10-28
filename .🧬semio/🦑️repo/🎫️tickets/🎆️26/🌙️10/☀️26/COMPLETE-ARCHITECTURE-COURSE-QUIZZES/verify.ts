import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { courseIssues, quizIssues, sheetOf, scoreRun, MOTIONS } from "../../../../../../../🧰️framework/🛍️products/📚️course/🟦️.ts";
const root = process.cwd();
const site = resolve(root, "🎓️teaching/🏛️architecture/📚️course");
const read = (file: string) => JSON.parse(readFileSync(file, "utf8"));
const course = read(resolve(site, "🔣️.json"));
const schema = read(resolve(root, "🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validQuiz = ajv.compile({ $ref: `${schema.$id}#/$defs/Quiz` });
const validSheet = ajv.compile({ $ref: `${schema.$id}#/$defs/Sheet` });
const names = process.argv.slice(2);
const quizzes = course.quizzes.map((file: string) => resolve(site, file)).filter((file: string) => existsSync(file) && (!names.length || names.some(name => file.includes(name.split("-")[0])))).map(read).filter((quiz: any) => !names.length || names.includes(quiz.id));
let checks = 0;
for (const quiz of quizzes) {
 assert.deepEqual(quizIssues(quiz), [], quiz.id + " domain");
 assert.equal(validQuiz(quiz), true, quiz.id + " AJV " + JSON.stringify(validQuiz.errors));
 for (const task of quiz.tasks) {
  assert.ok(task.pool && task.pool.seconds === 300, task.id);
  assert.equal(task.pool.slots.length, task.draw);
  const ids = task.items.map((item: any) => item.id);
  assert.deepEqual([...task.pool.slots.flat()].sort(), [...ids].sort());
  assert.ok(task.pool.slots.every((slot: any[]) => slot.length >= 5));
  assert.equal(new Set(task.items.map((item: any) => item.icon.emoji + item.icon.motion)).size, ids.length, task.id + " icon identity");
  for (const item of task.items) {
   for (const lang of ["en", "de"]) {
    assert.ok(item.label[lang] && item.explanation[lang], item.id + " " + lang);
    const short = (item.short ?? item.label)[lang];
    assert.ok([...short].length <= 40, task.id + "/" + item.id + " short " + short);
    assert.match(short, lang === "en" ? /^(?!(?:The|A|An|One) )(?!\p{L}+ing (?:(?:an?|the)\b|\d))[^,']*$/u : /^(?!(?:Der|Die|Das|Ein|Eine) )(?!\p{Lu}\p{Ll}+en (?:eines|einer|des|der) )[^,']*$/u);
   }
  }
 }
 for (const challenge of ["easy", "medium", "hard", "expert"] as const) {
  const seen = new Map();
  for (let attempt = 0; attempt < 5; attempt++) {
   const sheet = sheetOf(quiz, (0x73510000 | attempt) >>> 0, challenge);
   assert.equal(validSheet(sheet), true, quiz.id + " sheet AJV " + JSON.stringify(validSheet.errors));
   const answers: any = {};
   for (const shown of sheet.tasks) {
    const task = quiz.tasks.find((task: any) => task.id === shown.id);
    const previous = seen.get(task.id) ?? new Set();
    assert.equal(shown.items.length, task.draw);
    for (const item of shown.items) {
     assert.ok(!previous.has(item.id), task.id + " repeated " + item.id);
     previous.add(item.id);
     assert.ok(!("category" in item) && !("explanation" in item));
     assert.deepEqual(item.mockup, task.items.find((source: any) => source.id === item.id).mockup);
    }
    seen.set(task.id, previous);
    if (task.kind === "classification") answers[task.id] = { kind: "classification", assignments: Object.fromEntries(shown.items.map((item: any) => [item.id, task.items.find((source: any) => source.id === item.id).category])) };
   }
   if (quiz.tasks.every((task: any) => task.kind === "classification")) {
    assert.equal(scoreRun(quiz, sheet, answers)?.score, 1, quiz.id + " score");
    assert.equal(scoreRun(quiz, sheet, {})?.score, 0);
   }
   checks++;
  }
 }
 console.log("[DEBUG] Validated " + quiz.id + ": " + quiz.tasks.length + " tasks; " + quiz.tasks.reduce((n: number, task: any) => n + task.items.length, 0) + " items");
}
if (!names.length) assert.deepEqual(courseIssues(course, quizzes), []);
console.log("[DEBUG] Passed " + checks + " challenge/rotation runs with domain and AJV agreement.");
