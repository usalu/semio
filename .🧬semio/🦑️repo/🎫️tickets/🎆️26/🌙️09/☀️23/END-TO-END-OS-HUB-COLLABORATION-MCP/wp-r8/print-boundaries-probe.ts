/** 🔬️ R8 probe: measures the external import set of each print command-boundary entry exactly as the law computes it. */
import { build } from "esbuild";
import { readFileSync } from "node:fs";
import { join } from "node:path";
const workspaceRoot = "/Users/ueli/Documents/semio";
const productRoot = join(workspaceRoot, "🧰️framework/🛍️products/📓️print");
const contract = JSON.parse(readFileSync(join(productRoot, "🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
for (const entry of contract.entries) {
  const result = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const external = [...new Set(Object.values(result.metafile!.outputs).flatMap(output => output.imports).filter(item => item.external && !item.path.startsWith("node:") && item.path !== "bun").map(item => item.path.startsWith("@") ? item.path.split("/").slice(0, 2).join("/") : item.path.split("/")[0]))].sort();
  console.log(entry, Object.keys(result.metafile!.inputs).length, JSON.stringify(external));
}
