import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { renderCatalogArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts";
import { renderMetabolismArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🏗️builder/📽️projection/🟦️.ts";
let written = 0;
for (const artifact of [...renderCatalogArtifacts("all"), ...renderMetabolismArtifacts()]) if (!existsSync(artifact.path)) { mkdirSync(dirname(artifact.path), { recursive: true }); writeFileSync(artifact.path, artifact.content); written++; }
console.log("Restored missing asset artifacts:", written);
