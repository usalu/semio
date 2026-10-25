import { build } from "esbuild";
import { readFileSync, writeFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio", owner = "♻️mit-bestand/📋️bericht/", path = `${root}/${owner}🧪️tests/🧫️fixtures/🧭️commands.json`;
const contract = JSON.parse(readFileSync(path, "utf8"));
const sourceInputs: Record<string, unknown> = {};
for (const entry of contract.entries as string[]) {
  const b = await build({ absWorkingDir: root, entryPoints: [`${root}/${owner}${entry}`], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  const mine = Object.entries(b.metafile.inputs).filter(([p]) => p.startsWith(owner));
  sourceInputs[entry] = { sources: mine.map(([p]) => p).sort(), owners: [...new Set(mine.flatMap(([, i]) => i.imports).filter(x => !x.external).map(x => x.path).filter(p => !p.startsWith(owner)).map(p => p.split("/").slice(0, 3).join("/")))].sort() };
}
delete contract.maxSourceFiles;
const { entries, forbidden, externalDependencies, ...rest } = contract;
writeFileSync(path, JSON.stringify({ entries, sourceInputs, forbidden, externalDependencies, ...rest }, null, 2) + "\n");
console.log(JSON.stringify(sourceInputs, null, 1));
