import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { renderStylingArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";

/** 🎨️ Rewrites only the byte-stale styling artifacts from `🔣️.json`; unlike the full generator it removes nothing, so concurrent peers' files stay untouched.
 * Usage: `bun regenerate-styling-stale.ts` from the workspace root. */

for (const artifact of renderStylingArtifacts()) {
  if (existsSync(artifact.path) && readFileSync(artifact.path, "utf8") === artifact.content) continue;
  writeFileSync(artifact.path, artifact.content);
  console.log(`regenerated ${artifact.path}`);
}
