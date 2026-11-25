import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const projection = await import(pathToFileURL(join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts")).href);
const only = process.argv.slice(2);
for (const artifact of projection.renderStylingArtifacts() as { path: string; content: string }[]) {
  if (only.some((suffix) => artifact.path.endsWith(suffix))) { writeFileSync(artifact.path, artifact.content); console.log("wrote", artifact.path); }
}
