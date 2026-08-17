import { pathToFileURL } from "url";
import { readdirSync, readFileSync, writeFileSync } from "fs";
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
console.log("getWorkspaceRoot", mod.getWorkspaceRoot());
const catalog = mod.loadFrameworkOsPlaygroundCatalog();
console.log("catalog", catalog.length);
console.log("resolve", mod.resolveFrameworkOsPlaygroundPlugin(catalog, ["fem","3d"]));
writeFileSync(join(process.argv[2], "probe-root-result.json"), JSON.stringify({
  root: mod.getWorkspaceRoot(),
  catalog: catalog.length,
  resolve: mod.resolveFrameworkOsPlaygroundPlugin(catalog, ["fem","3d"]),
}, null, 2));
