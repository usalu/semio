import { writeFileSync } from "node:fs";
import { renderStylingArtifacts } from "C:/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
for (const artifact of renderStylingArtifacts()) if (artifact.path.endsWith("🦀️.rs") || artifact.path.endsWith("🐍️.py")) { writeFileSync(artifact.path, artifact.content); console.log("wrote", artifact.path); }
