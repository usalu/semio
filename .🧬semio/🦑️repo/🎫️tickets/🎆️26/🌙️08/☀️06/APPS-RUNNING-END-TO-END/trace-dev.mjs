import { pathToFileURL } from "url";
import { readdirSync, writeFileSync } from "fs";
import { join } from "path";
const root = process.cwd();
const fw = readdirSync(root).find((n) => n.includes("framework"));
const products = readdirSync(join(root, fw)).find((n) => n.includes("products"));
const repo = readdirSync(join(root, fw, products)).find((n) => n.includes("repo"));
const rmod = readdirSync(join(root, fw, products, repo)).find((n) => n.includes("modules"));
const lib = readdirSync(join(root, fw, products, repo, rmod)).find((n) => n.includes("lib"));
const lpkg = readdirSync(join(root, fw, products, repo, rmod, lib)).find((n) => n.includes("packages"));
const lts = readdirSync(join(root, fw, products, repo, rmod, lib, lpkg)).find((n) => n.includes("typescript"));
const idxdir = join(root, fw, products, repo, rmod, lib, lpkg, lts);
const idx = readdirSync(idxdir).find((n) => n.endsWith("index.ts") && !n.includes("test"));
const mod = await import(pathToFileURL(join(idxdir, idx)).href);
const catalog = mod.loadFrameworkOsPlaygroundCatalog();
const apps = [
  ["fem","3d"],["fem","2d"],["procedural","3d"],["puzzle","3d"],["cad"],["s"],["block","3d"],["gis","3d"],["os","multi"],
];
// os multi is special-cased
const results = {};
for (const segs of apps) {
  results[segs.join(" ")] = mod.resolveFrameworkOsPlaygroundPlugin(catalog, segs);
}
writeFileSync(join(process.argv[2], "trace-dev-result.json"), JSON.stringify({ catalog: catalog.length, results }, null, 2));
console.log(JSON.stringify({ catalog: catalog.length, results }, null, 2));
