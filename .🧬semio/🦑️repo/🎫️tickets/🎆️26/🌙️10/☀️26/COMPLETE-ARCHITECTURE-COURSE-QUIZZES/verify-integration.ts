import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { loadArchitectureCourseMaterial } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts";
import { assembleMenagerie, ensembleIssues, menagerieIssues, speciesIssues } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🟦️.ts";
const root = process.cwd(), site = resolve(root, "🎓️teaching/🏛️architecture/📚️course");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const course = read(resolve(site, "🔣️.json"));
const quizzes = course.quizzes.map((file: string) => read(resolve(site, file)));
assert.deepEqual(await loadArchitectureCourseMaterial(), { course, quizzes });
assert.deepEqual(course.topics, read(resolve(site, "🧫️fixtures/🗂️topics/🔣️.json")).topics);
assert.deepEqual(quizzes.map((quiz: any) => quiz.id), read(resolve(site, "🧫️fixtures/📚️course/🔣️.json")).quizzes);
assert.deepEqual(course.unlocks, { energy: 0, parametric: null, grasshopper: null, excel: null, bim: null, optimization: null });
const curriculum = read(resolve(site, "🧫️fixtures/🧩️curriculum/🔣️.json"));
for (const [topic, quiz] of Object.entries(curriculum.topics)) assert.deepEqual(course.topics.find((entry: any) => entry.id === topic).quizzes, [quiz]);
let expectations = 0;
for (const area of ["🏗️bim", "📐️parametric", "📊️excel", "🦗️grasshopper", "🎯️optimization"]) {
 const fixture = read(resolve(site, `🧫️fixtures/${area}/🔣️.json`));
 const quiz = quizzes.find((quiz: any) => quiz.id === fixture.quiz);
 assert.ok(quiz, area);
 assert.deepEqual(quiz.tasks.map((task: any) => task.id), curriculum.tasks[quiz.id]);
 for (const [taskId, answers] of Object.entries(fixture.classifications)) for (const [itemId, category] of Object.entries(answers as object)) {
  const item = quiz.tasks.find((task: any) => task.id === taskId).items.find((item: any) => item.id === itemId);
  assert.equal(item?.category, category, `${taskId}/${itemId}`);
  expectations++;
 }
 if (fixture.pareto) for (const vector of fixture.pareto.cases) {
  const baseline = fixture.pareto.baseline;
  assert.equal(baseline.every((value: number, index: number) => value <= vector.values[index]) && baseline.some((value: number, index: number) => value < vector.values[index]), vector.dominated, vector.id);
 }
}
const pets = resolve(root, "🎓️teaching/🏛️architecture/🐾️pets"), ensemble = read(resolve(pets, "🔣️.json"));
const species = ensemble.species.map((file: string) => read(resolve(pets, file)));
assert.deepEqual(ensembleIssues(ensemble), []);
for (const entry of species) assert.deepEqual(speciesIssues(entry), [], entry.id);
const menagerie = assembleMenagerie(ensemble, species);
assert.deepEqual(menagerieIssues(menagerie), []);
const grounds = new Set(quizzes.flatMap((quiz: any) => [quiz.id, ...quiz.tasks.flatMap((task: any) => [quiz.id + "/" + task.id, ...task.items.map((item: any) => quiz.id + "/" + task.id + "/" + item.id)])]));
for (const topic of course.topics) grounds.add("topic/" + topic.id);
for (const entry of species) for (const ground of entry.grounds) assert.ok(grounds.has(ground), entry.id + "/" + ground);
const expectedScenes = ["home", ...course.topics.map((topic: any) => "topic-" + topic.id), ...quizzes.flatMap((quiz: any) => [quiz.id, ...quiz.tasks.map((task: any) => "question-" + quiz.id + "-" + task.id)])];
assert.deepEqual(ensemble.casts.map((cast: any) => cast.scene).sort(), expectedScenes.sort());
const castFixture = read(resolve(site, "🧫️fixtures/🐾️context-casts/🔣️.json"));
for (const expected of castFixture.questions) {
 const cast = ensemble.casts.find((cast: any) => cast.scene === "question-" + expected.quiz + "-" + expected.task);
 assert.deepEqual(cast, { scene: "question-" + expected.quiz + "-" + expected.task, core: expected.core, rotation: expected.rotation });
 for (const id of [...cast.core, ...cast.rotation]) assert.ok(species.find((species: any) => species.id === id).grounds.some((ground: string) => ground === expected.quiz + "/" + expected.task || ground.startsWith(expected.quiz + "/" + expected.task + "/")));
}
const schema = read(resolve(root, "🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false, allErrors: true }); ajv.addSchema(schema);
const valid = ajv.compile({ $ref: `${schema.$id}#/$defs/Menagerie` });
assert.equal(valid(menagerie), true, JSON.stringify(valid.errors));
for (const file of ["🎭️e2e/🚶️learner/🟦️.ts", "🧪️tests/🎯️quiz-runs/🟦️.ts", "🧪️tests/🗣️both-languages/🟦️.ts", "🧪️tests/🥞️layered-home/🟦️.ts", "🧪️tests/🧩️curriculum/🟦️.ts"]) new Bun.Transpiler({ loader: "tsx" }).transformSync(readFileSync(resolve(site, file), "utf8"));
console.log(`[DEBUG] Integration passed: ${quizzes.length} quizzes, ${expectations} authored classification expectations, ${species.length} species and ${expectedScenes.length} scenes; source syntax parsed.`);
