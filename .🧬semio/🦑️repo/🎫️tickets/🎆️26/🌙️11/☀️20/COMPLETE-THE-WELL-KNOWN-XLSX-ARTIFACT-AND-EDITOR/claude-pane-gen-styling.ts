import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
/** 🛠️ [DEBUG] Regenerates the gitignored styling artifacts from the committed schema with the production writer plan. */
const root = "/Users/ueli/Documents/semio";
const { generateStylingArtifacts } = await import(join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts"));
let count = 0;
generateStylingArtifacts((path: string, content: string) => { const target = isAbsolute(path) ? path : join(root, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, content); count++; });
console.log(`[DEBUG] wrote ${count} styling artifacts`);
