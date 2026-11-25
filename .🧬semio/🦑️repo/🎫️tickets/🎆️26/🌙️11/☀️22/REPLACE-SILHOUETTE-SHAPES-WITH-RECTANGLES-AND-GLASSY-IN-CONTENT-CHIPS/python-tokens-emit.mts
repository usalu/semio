import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const projection = await import(pathToFileURL(join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts")).href);
const py = projection.renderStylingArtifacts().find((a: { path: string }) => a.path.endsWith("🐍️.py"))!;
writeFileSync(py.path, py.content);
console.log("wrote", py.path);
