import { existsSync, readdirSync, statSync } from "fs";
import { join, resolve } from "path";
function walk(dir, pred, acc=[], depth=0) {
  if (depth > 8) return acc;
  let entries; try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (["node_modules","target","dist",".git"].includes(name)) continue;
    const p = join(dir, name);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, pred, acc, depth+1);
    else if (pred(p)) acc.push(p);
  }
  return acc;
}
const root = process.cwd();
const hits = walk(join(root, "タバframework"), p => p.endsWith("index.ts") && p.includes("repo") && p.includes("lib") && p.includes("typescript"));
const hits2 = walk(join(root, "タバframework"), p => p.includes("repo") && p.endsWith("📦️index.ts"));
console.log("hits", hits.slice(0,10));
console.log("hits2", hits2.slice(0,10));
const fromStory = resolve(root, ".storybook", "../タバframework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts");
console.log("fromStory", fromStory, existsSync(fromStory));
const direct = resolve(root, "タバframework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts");
console.log("direct", direct, existsSync(direct));
try {
  const pkgs = readdirSync(join(root, "node_modules/@semio-tech")).filter(x => x.includes("repo"));
  console.log("repo pkgs", pkgs);
} catch(e) { console.log(e.message); }
