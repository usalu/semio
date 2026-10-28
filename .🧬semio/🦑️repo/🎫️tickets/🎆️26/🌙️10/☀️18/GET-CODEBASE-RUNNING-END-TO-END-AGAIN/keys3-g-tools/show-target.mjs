// keys3-g: prints the sealed-generation definition (inputs, namedInputs subset) of project:target. usage: bun show-target.mjs <project> [target] ; RSS guard 500 MB.
import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const [project, target] = process.argv.slice(2);
const generations = "/Users/ueli/Documents/semio/.cache/b/g/m/generations/";
const cur = JSON.parse(readFileSync(generations + "current.json", "utf8"));
const manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
const row = manifest.owners[project];
if (!row) { console.error("unknown project", project); process.exit(2); }
const fragment = JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + row.fragment + ".json.br")).toString("utf8"));
console.log("generation", cur.generation, "root", fragment.root);
const names = fragment.node?.namedInputs ?? {};
for (const [name, list] of Object.entries(names)) if (/toolchain|cargo/i.test(name)) console.log("namedInput", name, JSON.stringify(list));
console.log("namedInputs keys", Object.keys(names).join(","));
for (const [key, def] of Object.entries(fragment.node?.targets ?? {})) {
  if (target && key !== target) continue;
  console.log(key, JSON.stringify({inputs: def.inputs, command: def.options?.command, cache: def.cache, metadata: def.metadata}).slice(0, 1500));
}
