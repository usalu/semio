/** 🎨️ Writes only the Rust twin of the styling tokens from the shared byte plan (the hub build needs it fresh). */
import { writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio";
const { renderStylingArtifacts } = await import(`${R}/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts`);
for (const artifact of renderStylingArtifacts()) if (artifact.path.endsWith("🔤️tokens/🦀️.rs")) { writeFileSync(artifact.path, artifact.content); console.log("wrote", artifact.path); }
