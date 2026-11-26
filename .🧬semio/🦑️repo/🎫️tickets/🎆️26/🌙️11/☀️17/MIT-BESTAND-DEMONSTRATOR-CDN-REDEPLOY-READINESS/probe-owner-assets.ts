import { readFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { demonstratorRuntimeAssetSources } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import { copyBrowserArtifacts, type BrowserArtifactSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";

import { browserCoreName, gzipCoreSuffix } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🗜️cores/🟦️.ts";

const [directory, ...modules] = process.argv.slice(2);
if (!directory || !modules.length || new Set(modules).size !== modules.length) throw new Error("Select distinct staged module directories");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const output = resolve(directory);
const all = demonstratorRuntimeAssetSources(workspace, "release");
const support = new Set(["browser-support:preview2", "browser-support:shard"]);
const selected = modules.map(directory => {
  const matches = all.filter(source => !support.has(source.owner) && basename(source.root) === directory);
  if (matches.length !== 1) throw new Error(`Expected one runtime source for ${directory}`);
  return matches[0]!;
});
const sources: BrowserArtifactSource[] = [...selected, ...all.filter(source => support.has(source.owner))];
if (sources.length !== selected.length + support.size) throw new Error("Incomplete browser support inventory");

for (const source of sources) {
  const marker = JSON.parse(readFileSync(join(source.root, ".semio-artifact.json"), "utf8"));
  for (const file of marker.files) {
    const published = browserCoreName.test(file) ? file + gzipCoreSuffix : file;
    if (join(output, source.destination, published).length > 256) throw new Error(`Diagnostic asset path exceeds 256 characters: ${file}`);
  }
}

await copyBrowserArtifacts(output, sources, { gzipComponentCores: true });
console.log(JSON.stringify({ modules: selected.map(source => source.destination), mounts: sources.map(source => source.destination) }));
