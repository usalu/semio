import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const directory = path.dirname(fileURLToPath(import.meta.url));
let workspace = directory;
while (!fs.existsSync(path.join(workspace, ".git"))) {
  const parent = path.dirname(workspace);
  if (parent === workspace) throw Error("Repository root required");
  workspace = parent;
}
const ticket = path.relative(workspace, directory).replaceAll("\\", "/");
const files = new Set([".gitignore"]);
const add = candidate => {
  if (!candidate || path.isAbsolute(candidate) || candidate.includes("\\") || candidate.split("/").includes("..")) return;
  if (path.resolve(workspace, candidate).length > 256) throw Error("Finite inventory path required: " + candidate);
  files.add(candidate);
};
const walk = relative => {
  const absolute = path.join(workspace, relative);
  if (!fs.existsSync(absolute)) return;
  for (const entry of fs.readdirSync(absolute, { withFileTypes: true })) {
    if (["🗑️generated", "__pycache__", "node_modules"].includes(entry.name)) continue;
    const candidate = relative + "/" + entry.name;
    if (entry.isDirectory()) walk(candidate);
    else if (entry.isFile()) add(candidate);
  }
};
for (const name of ["assignment-engine.md", "course-assignment.md", "latex-product.md", "assignment-content.md", "integration-build.md", "integration-final-audit.md", "integration-course.md"]) {
  const report = path.join(directory, name);
  if (!fs.existsSync(report)) continue;
  for (const line of fs.readFileSync(report, "utf8").split("\n")) {
    const candidate = line.match(/^- \x60?([^\x60]+)\x60?$/u)?.[1]?.trim();
    if (candidate && candidate.includes("/") && !candidate.includes(" ")) add(candidate);
  }
}
const print = "🧰️framework/🛍️products/📓️print";
const framework = "🧰️framework/🛍️products/📚️course";
const assignment = "🎓️teaching/🏛️architecture/📐️parametric/📝️assignment";
for (const relative of [ticket, assignment, print + "/🧾️template/📝️assignment", print + "/🧪️tests/📝️assignment", print + "/🧪️tests/🏷️page-chrome"]) walk(relative);
for (const suffix of ["🧬️schema/🔣️.json", "🧬️schema/🟦️.ts", "🧬️schema/📚️quiz-documents/🦀️.rs", "🔨️modules/✅️validation/🟦️.ts", "🔨️modules/✅️validation/🦀️.rs", "🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs", "🧫️fixtures/📝️assignments/🔣️.json", "🧪️tests/📝️assignments/🟦️.ts", "🧪️tests/📝️assignments/🥒️.feature"]) add(framework + "/" + suffix);
for (const name of ["assignment.cls", "semio-assignment.sty", "semio-core.sty", "semio-window.sty"]) add(print + "/🖋️latex/" + name);
add("🎓️teaching/🏛️architecture/📚️course/🔣️.json");
for (const name of ["parametrization-en.tex", "parametrization-de.tex", "parametrization-en.content.tex", "parametrization-de.content.tex", "parametrization-geometry.tex", "parametrization-icons.json"]) add(print + "/🧾️template/📝️assignment/" + name);
for (const name of ["🧫️cases.json", "🧬️cases.schema.json", "🥒️.feature", "🟦️.ts", "🔮️svg-provenance.py"]) add(print + "/🧪️tests/📝️parametrization-assignment/" + name);
const plan = JSON.parse(fs.readFileSync(path.join(workspace, assignment, "icons.json"), "utf8"));
for (const resource of plan.resources) add(print + "/🧾️template/📝️assignment/" + resource.output.replace(/\.png$/u, ".pdf"));
for (const name of ["Assignment-Table-Silhouette.pdf", "Fingeruebung-Parametrisierung-Ohne-Fliesstext.pdf", "Fingeruebung-Parametrisierung.pdf"]) add("output/pdf/" + name);
console.log(JSON.stringify([...files].sort()));
