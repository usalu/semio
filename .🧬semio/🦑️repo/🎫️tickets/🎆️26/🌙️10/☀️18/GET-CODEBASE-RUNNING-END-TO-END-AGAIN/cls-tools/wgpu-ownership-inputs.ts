// usage: bun wgpu-ownership-inputs.ts [taxonomy.json] : the final loop of testWgpuGeneratorOwnership, reported instead of asserted
import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { relativeSourceInputs, readSourceInputContract } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs";

const workspace = "/Users/ueli/Documents/semio";
const taxonomyPath = process.argv[2] ?? join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const taxonomy = JSON.parse(readFileSync(taxonomyPath, "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const project = JSON.parse(readFileSync(join(workspace, profile.ownerPath, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
const browserSources = new Set<string>(relativeSourceInputs(readSourceInputContract(join(workspace, profile.ownerPath, "⚙️browser-build/🔣️.json")), workspace).files.map((path: string) => relative(workspace, path).replaceAll("\\", "/")));
const missing: string[] = [];
for (const source of profile.sourceModulePaths as string[]) {
  if (source.includes("/🤖️generated/")) continue;
  if (!(project.namedInputs.frameWorkerSources.includes("{workspaceRoot}/" + source) || browserSources.has(source))) missing.push(source);
}
console.log(JSON.stringify({ browserSources: browserSources.size, sourceModulePaths: profile.sourceModulePaths.length, missing }, null, 1));
