import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";
const root = process.cwd();
function walk(d, depth=0, out=[]) {
  if (depth>8) return out;
  let ents; try { ents = readdirSync(d,{withFileTypes:true}); } catch { return out; }
  for (const e of ents) {
    if (["node_modules","target",".git"].includes(e.name)) continue;
    const p = join(d,e.name);
    if (e.isFile() && e.name.includes("script.ts") && p.includes("dev") && p.includes("typescript") && p.includes("os")) out.push(p);
    if (e.isDirectory()) walk(p, depth+1, out);
  }
  return out;
}
// narrower: from framework products os modules
const fw = readdirSync(root).find(n=>n.includes("framework"));
const products = readdirSync(join(root,fw)).find(n=>n.includes("products"));
const osn = readdirSync(join(root,fw,products)).find(n=>n.endsWith("os"));
const modules = readdirSync(join(root,fw,products,osn)).find(n=>n.includes("modules"));
const dev = readdirSync(join(root,fw,products,osn,modules)).find(n=>n.includes("dev"));
const packages = readdirSync(join(root,fw,products,osn,modules,dev)).find(n=>n.includes("packages"));
const ts = readdirSync(join(root,fw,products,osn,modules,dev,packages)).find(n=>n.includes("typescript"));
const dir = join(root,fw,products,osn,modules,dev,packages,ts);
const script = readdirSync(dir).find(n=>n.includes("script.ts"));
const src = readFileSync(join(dir,script),"utf8");
const m = [...src.matchAll(/PLUGIN_WASM_TARGET\s*=\s*["']([^"']+)["']/g)];
const uses = [...src.matchAll(/PLUGIN_WASM_TARGET/g)].length;
console.log("script", join(dir,script));
console.log("defs", m.map(x=>x[1]));
console.log("uses", uses);
writeFileSync(join(process.argv[2],"wasm-target.json"), JSON.stringify({script:join(dir,script), defs:m.map(x=>x[1]), uses},null,2));
