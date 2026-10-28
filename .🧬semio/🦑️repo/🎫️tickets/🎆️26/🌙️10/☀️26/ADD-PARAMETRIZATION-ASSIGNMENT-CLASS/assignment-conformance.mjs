import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const base = resolve("🧰️framework/🛍️products/📚️course/");
const require = createRequire(resolve(base, "📦️packages/🟦️typescript/package.json"));
const Ajv = require("ajv");
const contract = JSON.parse(readFileSync(resolve(base, "🧬️schema/🔣️.json"), "utf8"));
const cases = JSON.parse(readFileSync(resolve(base, "🧫️fixtures/📝️assignments/🔣️.json"), "utf8"));
const { courseIssues, courseViewIssues } = await import(pathToFileURL(resolve(base,"🔨️modules/✅️validation/🟦️.ts")));
const { courseView } = await import(pathToFileURL(resolve(base,"🔨️modules/👁️views/🟦️.ts")));
const validate = new Ajv({strict:false}).compile({$ref:"#/$defs/Assignment",$defs:contract.$defs});
const quiz = {schema:"semio.quiz/v1",id:"sample",emoji:"📐",title:{en:"Sample",de:"Beispiel"},description:{en:"Sample",de:"Beispiel"},tasks:[]};
const course = assignment => ({schema:"semio.course/v1",id:"sample-course",title:quiz.title,introduction:{title:quiz.title,paragraphs:[quiz.title]},quizzes:["sample.json"],topics:[{id:"parametric",emoji:"📐",title:quiz.title,quizzes:["sample"]}],badges:[],assignments:[assignment]});
for (const assignment of cases.valid) {
 assert.equal(validate(assignment),true);assert.deepEqual(courseIssues(course(assignment),[quiz]),[]);
 const view=courseView(course(assignment),[quiz]);assert.deepEqual(view.assignments,[assignment]);assert.deepEqual(courseViewIssues(view),[]);
}
for (const sample of cases.invalid) {
 const assignment=structuredClone(cases.valid[0]);const parts=(sample.remove??sample.replace).split("/").slice(1),key=parts.pop();let parent=assignment;for(const part of parts)parent=parent[part];if(sample.remove)delete parent[key];else parent[key]=sample.value;
 assert.equal(validate(assignment),false,sample.case);assert.ok(courseIssues(course(assignment),[quiz]).length>0,sample.case);
}
assert.ok(courseIssues(course({...cases.valid[0],topic:"unknown"}),[quiz]).length>0);
assert.ok(courseIssues({...course(cases.valid[0]),assignments:[cases.valid[0],cases.valid[0]]},[quiz]).length>0);
console.log("PASS: 7 language-neutral assignment cases, independent AJV agreement and CourseView projection");
