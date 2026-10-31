import { dirname, join } from "node:path";
import { statSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";

let root = dirname(fileURLToPath(import.meta.url));
while (!statSync(join(root, "🧰️framework"), { throwIfNoEntry: false })) root = dirname(root);
const typology = await import(pathToFileURL(join(root, "🎓️teaching/🏛️architecture/🏙️typology/📝️assignment/🧪️tests/🟦️.ts")).href);
typology.verifyTypologyAssignmentContentV1();
for (const [name, scenario] of Object.entries(typology.default.scenarios as Record<string, { oracle: () => unknown; subject: () => unknown }>)) {
  const oracle = JSON.stringify(scenario.oracle()), subject = JSON.stringify(scenario.subject());
  if (oracle !== subject) throw Error(`[DEBUG] ${name} differs\n${oracle}\n${subject}`);
  console.log("[DEBUG] scenario", name, "agrees");
}
const parametric = await import(pathToFileURL(join(root, "🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🟦️.ts")).href);
parametric.verifyParametrizationAssignmentContentV1();
const { courseAssignmentPdfDocumentsV1 } = await import(pathToFileURL(join(root, "🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📃️documents/🟦️.ts")).href);
const course = (await import(pathToFileURL(join(root, "🎓️teaching/🏛️architecture/📚️course/🔣️.json")).href, { with: { type: "json" } })).default;
console.log("[DEBUG] publications", courseAssignmentPdfDocumentsV1(course.assignments.flatMap((assignment: { documents: object }) => Object.values(assignment.documents))).map((row: { file: string }) => row.file));
