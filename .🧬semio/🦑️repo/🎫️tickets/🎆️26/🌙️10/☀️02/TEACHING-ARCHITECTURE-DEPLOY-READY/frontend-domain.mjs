/** 🧪️ Compares authored course material with owned domain validation and AJV. */
import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
const workspace = process.cwd(), site = path.join(workspace, "🎓️teaching/🏛️architecture/📚️course");
const read = file => JSON.parse(fs.readFileSync(file, "utf8"));
const core = await import(pathToFileURL(path.join(workspace, "🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts")));
const course = read(path.join(site, "🔣️.json")), quizzes = course.quizzes.map(file => read(path.resolve(site, file)));
const schema = read(path.join(workspace, "🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json")), ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const results = [];
for (const [document, definition, issues] of [[course, "Course", core.courseIssues(course, quizzes)], ...quizzes.map(quiz => [quiz, "Quiz", core.quizIssues(quiz)])]) {
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/" + definition });
  assert.deepEqual(issues, [], document.id);
  assert.equal(validate(document), true, JSON.stringify(validate.errors));
  results.push({ id: document.id, ownedIssues: 0, ajv: true });
}
const view = core.courseView(course, quizzes), validateView = ajv.compile({ $ref: schema.$id + "#/$defs/CourseView" });
assert.deepEqual(core.courseViewIssues(view), []);
assert.equal(validateView(view), true, JSON.stringify(validateView.errors));
console.log(JSON.stringify({ results, courseView: { ownedIssues: 0, ajv: true } }));
const material = await import(pathToFileURL(path.join(site, "📚️course/🟦️.ts"))), loaded = await material.loadArchitectureCourseMaterial();
assert.deepEqual(loaded, { course, quizzes });
assert.deepEqual(Object.keys(material), ["loadArchitectureCourseMaterial"]);
console.log(JSON.stringify({ materialMatchesDisk: true, materialIds: loaded.quizzes.map(quiz => quiz.id) }));
