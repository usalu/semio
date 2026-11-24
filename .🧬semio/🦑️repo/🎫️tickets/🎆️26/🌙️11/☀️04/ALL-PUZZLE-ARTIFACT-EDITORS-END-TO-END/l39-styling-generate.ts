/** 🎨️ Regenerates framework/ui/styling artifacts directly when the repo script runner is unavailable. Usage: bun l39-styling-generate.ts */
import { generateStylingArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
generateStylingArtifacts(writeGeneratedFileIfChanged);
