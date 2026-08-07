import { readdirSync, readFileSync, statSync, writeFileSync } from "fs";
import { join } from "path";
const root = "/Users/ueli/Documents/semio";
function walk(dir, pred, acc=[]) {
  let entries; try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (["node_modules","target","dist",".git"].includes(name)) continue;
    const p = join(dir, name);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, pred, acc);
    else if (pred(p)) acc.push(p);
  }
  return acc;
}
const uidriver = walk(join(root, "🧰️framework"), p => p.includes("UiDriver") && p.endsWith("component.tsx"));
const out = { uidriver, imports: [] };
for (const p of uidriver) {
  const lines = readFileSync(p,"utf8").split("\n").slice(0,50);
  lines.forEach((l,i)=>{ if (l.includes("import") || l.toLowerCase().includes("ephemeral")) out.imports.push(`${i+1}:${l}`); });
}
const glue = walk(join(root, "🧰️framework/📦️packages"), p => /glue\.ts$|index\.ts$/.test(p));
out.pkgFiles = glue.slice(0,20);
out.glueSnippets = {};
for (const g of glue.filter(p=>p.includes("typescript")).slice(0,8)) {
  out.glueSnippets[g] = readFileSync(g,"utf8").slice(0,1500);
}
writeFileSync(join(import.meta.dirname, "probe-uidriver-result.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify({ uidriver: out.uidriver, imports: out.imports, pkgFiles: out.pkgFiles }, null, 2));
