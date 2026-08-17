import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { join, relative, dirname } from "node:path";
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
const fwPath = join(root, fwName);
const hits = findFiles(fwPath, (n,p) => n.endsWith("index.ts") && p.includes("repo") && p.includes("lib") && p.includes("packages") && p.includes("typescript"));
console.log("index candidates:");
for (const p of hits) {
  const src = readFileSync(p, "utf8");
  const has = src.includes("loadFrameworkOsPlaygroundCatalog");
  console.log(has ? "HIT" : "miss", p);
}
const idx = hits.find(p => readFileSync(p,"utf8").includes("loadFrameworkOsPlaygroundCatalog"));
if (!idx) { console.error("no idx"); process.exit(1); }
const actualJson = findFiles(fwPath, (n,p) => n.endsWith("playgrounds.json") && p.includes("generated"))[0];
const actualTs = findFiles(fwPath, (n,p) => n.endsWith("playgrounds.ts") && p.includes("generated"))[0];
let src = readFileSync(idx, "utf8");
const beforeJson = src.match(/join\(getWorkspaceRoot\(\),\s*"([^"]+)"\)/)?.[1];
const beforeImport = [...src.matchAll(/from\s+"([^"]*playgrounds\.ts)"/g)].map(m => m[1]);
console.log({ beforeJson, beforeImport, actualJson, actualTs });
console.log("before exists?", existsSync(join(root, beforeJson.replace(/^\.\//,""))));

const relJsonFromRoot = "./" + relative(root, actualJson).split("\\").join("/");
const relTsFromIdx = relative(dirname(idx), actualTs).split("\\").join("/");
console.log({ relJsonFromRoot, relTsFromIdx });

let src2 = src.replace(
  /join\(getWorkspaceRoot\(\),\s*"[^"]+playgrounds\.json"\)/,
  `join(getWorkspaceRoot(), ${JSON.stringify(relJsonFromRoot)})`,
);
for (const imp of beforeImport) {
  const abs = join(dirname(idx), imp);
  if (!existsSync(abs) || imp.includes("implementations")) {
    const target = relTsFromIdx.startsWith(".") ? relTsFromIdx : "./" + relTsFromIdx;
    src2 = src2.replaceAll(`from "${imp}"`, `from "${target}"`);
  }
}

if (src2 === src) {
  console.log("NO TEXT CHANGE");
} else {
  writeFileSync(idx, src2);
  console.log("UPDATED", idx);
}
const after = readFileSync(idx, "utf8");
const afterJson = after.match(/join\(getWorkspaceRoot\(\),\s*"([^"]+)"\)/)?.[1];
console.log("afterJson", afterJson, "exists", existsSync(join(root, afterJson.replace(/^\.\//,""))));
console.log("after imports", [...after.matchAll(/from\s+"([^"]*playgrounds\.ts)"/g)].map(m => m[1]));
