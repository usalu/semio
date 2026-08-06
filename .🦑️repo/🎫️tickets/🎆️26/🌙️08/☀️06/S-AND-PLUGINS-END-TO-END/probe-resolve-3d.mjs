import { readdirSync, statSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = "/Users/ueli/Documents/semio";
function findFiles(dir, pred, depth = 0, acc = []) {
  if (depth > 16) return acc;
  let ents; try { ents = readdirSync(dir); } catch { return acc; }
  for (const name of ents) {
    if (["node_modules","target","dist",".git",".nx","storybook-static"].includes(name)) continue;
    const p = join(dir, name);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) findFiles(p, pred, depth+1, acc);
    else if (pred(name,p)) acc.push(p);
  }
  return acc;
}
const fwName = readdirSync(root).find((n) => n.includes("framework") && readdirSync(join(root,n)).includes("🔨️modules"));
const idx = findFiles(join(root, fwName), (n,p) => n.endsWith("index.ts") && p.includes("repo") && p.includes("lib") && p.includes("packages")).find(p => readFileSync(p,"utf8").includes("loadFrameworkOsPlaygroundCatalog"));
console.log("importing", idx);
const mod = await import(pathToFileURL(idx).href);
const catalog = mod.loadFrameworkOsPlaygroundCatalog();
console.log("catalog rows", catalog.length);
console.log("resolve 3d", mod.resolveFrameworkOsPlaygroundPlugin(catalog, ["3d"]));
console.log("resolve puzzle3d", mod.resolveFrameworkOsPlaygroundPlugin(catalog, ["puzzle3d"]));
console.log("env", mod.frameworkOsPlaygroundDevEnv(catalog, "puzzle3d", {}, {}));
