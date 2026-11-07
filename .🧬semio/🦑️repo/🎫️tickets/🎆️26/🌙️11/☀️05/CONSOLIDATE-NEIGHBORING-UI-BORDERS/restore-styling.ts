import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { renderStylingArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
for (const artifact of renderStylingArtifacts()) if (!existsSync(artifact.path)) { mkdirSync(dirname(artifact.path), { recursive: true }); writeFileSync(artifact.path, artifact.content); console.log(artifact.path); }
