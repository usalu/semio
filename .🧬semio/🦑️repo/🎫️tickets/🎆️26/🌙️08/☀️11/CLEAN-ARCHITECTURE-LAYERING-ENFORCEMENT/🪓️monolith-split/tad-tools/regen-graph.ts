/** 🏭️ Runs the real graph-manifest generator (renderGraphArtifacts + writeGraphArtifacts, exactly like GenerateScript.run) without the repo launcher. usage: bun regen-graph.ts <check|write> <artifact-dir>... */
import { join, relative } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import { readGraphOutputCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts";
import { graphOutputInventory, graphOutputNodes, writeGraphArtifacts } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const [mode, ...dirs] = process.argv.slice(2);
if (mode !== "check" && mode !== "write") throw new Error("mode check|write");
let failed = false;
for (const dir of dirs) {
  const outDir = join(dir, "🤖️generated");
  const rendered = renderGraphArtifacts(root, outDir, readGraphOutputCatalog(join(dir, "🛂️manifest/📇️outputs.json")), false);
  const expected = graphOutputNodes(outDir, rendered.artifacts);
  const actual = existsSync(outDir) ? graphOutputInventory(outDir) : [];
  const stale = rendered.artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => relative(outDir, artifact.path));
  const membership = JSON.stringify(actual) === JSON.stringify(expected);
  console.log(`[tad-regen] ${mode} ${relative(root, dir)}: ${rendered.manifestCount} manifests, membership ${membership ? "equal" : "DIFFERS"}, ${stale.length} stale files ${JSON.stringify(stale)}`);
  if (mode === "write") {
    writeGraphArtifacts(outDir, rendered.artifacts);
    console.log(`[tad-regen] wrote ${rendered.artifacts.length} artifacts to ${relative(root, outDir)}`);
  } else if (!membership || stale.length) failed = true;
}
process.exit(failed ? 1 : 0);
