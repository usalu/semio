/** 🏺️ Replays the real asset owners while dependency-store task dispatch is unavailable. */
import { generateStylingArtifacts, checkStylingArtifacts, renderStylingArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { publishAssetArtifacts, checkAssetArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts";

generateStylingArtifacts(writeGeneratedFileIfChanged);
const count = publishAssetArtifacts();
checkStylingArtifacts();
checkAssetArtifacts();
console.log(`Prepared and verified ${renderStylingArtifacts().length} styling and ${count} asset artifacts from authored sources.`);
