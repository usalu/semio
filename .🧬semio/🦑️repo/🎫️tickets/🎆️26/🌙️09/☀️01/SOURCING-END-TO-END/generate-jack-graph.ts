import { join, relative } from "node:path";
import { readGraphOutputCatalog } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts";
import { writeGraphArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts";

/** 🕸️ Writes the jack graph catalog outputs (`🤖️generated`) natively, the way `GenerateScript` does without the repo process observation.
 * Usage: `bun generate-jack-graph.ts [artifact-dir]` from the repo root; defaults to jack. */
const root = process.cwd();
const jack = join(root, process.argv[2] ?? "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack");
const outDir = join(jack, "🤖️generated");
const rendered = renderGraphArtifacts(root, outDir, readGraphOutputCatalog(join(jack, "🛂️manifest/📇️outputs.json")));
writeGraphArtifacts(outDir, rendered.artifacts);
console.log(`wrote ${rendered.manifestCount} manifests to ${relative(root, outDir)}`);
