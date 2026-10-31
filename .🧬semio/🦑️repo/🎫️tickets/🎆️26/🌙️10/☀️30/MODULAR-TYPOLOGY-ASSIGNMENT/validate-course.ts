import { dirname, join, resolve } from "node:path";
import { readFileSync, statSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";
import Ajv from "ajv";

let root = dirname(fileURLToPath(import.meta.url));
while (!statSync(join(root, "🧰️framework"), { throwIfNoEntry: false })) root = dirname(root);
const product = join(root, "🧰️framework/🛍️products/📚️course");
const contract = JSON.parse(readFileSync(join(product, "🧬️schema/🔣️.json"), "utf8"));
const coursePath = join(root, "🎓️teaching/🏛️architecture/📚️course/🔣️.json");
const course = JSON.parse(readFileSync(coursePath, "utf8"));
const validate = new Ajv({ strict: false }).compile({ $ref: "#/$defs/Assignment", $defs: contract.$defs });
for (const assignment of course.assignments) console.log("[DEBUG] schema", assignment.id, validate(assignment), JSON.stringify(validate.errors ?? []));
const { courseIssues } = await import(pathToFileURL(join(product, "🔨️modules/✅️validation/🟦️.ts")).href);
const quizzes = course.quizzes.map((path: string) => JSON.parse(readFileSync(resolve(dirname(coursePath), path), "utf8")));
console.log("[DEBUG] courseIssues", JSON.stringify(courseIssues(course, quizzes)));
