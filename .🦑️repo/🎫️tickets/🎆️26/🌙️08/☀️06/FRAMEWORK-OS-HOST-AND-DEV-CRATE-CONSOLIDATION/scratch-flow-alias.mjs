import { existsSync, readFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function findFile(root, pred, depth=0, out=[]) {
  if (depth>12 || !existsSync(root)) return out;
  let ents; try { ents=readdirSync(root);} catch { return out; }
  for (const name of ents) {
    if (["node_modules","target",".git","dist"].includes(name)) continue;
    const p=join(root,name);
    let st; try { st=statSync(p);} catch { continue; }
    if (st.isDirectory()) findFile(p, pred, depth+1, out);
    else if (pred(name,p)) out.push(p);
  }
  return out;
}

const realJs = findFile("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow", (n)=>n==="flow_core.js");
console.log("realJs", realJs);
for (const p of realJs) {
  const t = readFileSync(p,"utf8");
  console.log(p, "bytes", t.length);
  console.log("default", t.match(/export default [^\n;]+/)?.[0]);
  console.log("init", t.match(/export (async )?function (__wbg_init|init)/)?.[0]);
}

console.log("STUB", readFileSync("/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/flow_core.js","utf8"));
console.log("NM_PKG", readFileSync("/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/package.json","utf8"));

const demoVites = findFile("/Users/ueli/Documents/semio/♻️mit-bestand", (n,p)=>n==="⚙️vite.config.ts" && p.includes("demonstrator"));
console.log("demoVites", demoVites);
for (const p of demoVites) console.log(readFileSync(p,"utf8"));

const osVites = findFile("/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os", (n)=>n==="⚙️vite.config.ts");
// emoji fix
const osVites2 = findFile("/Users/ueli/Documents/semio/𝒯framework", (n)=>n==="⚙️vite.config.ts");
const osVites3 = findFile("/Users/ueli/Documents/semio/𝒯framework/🛍️products", (n)=>n==="⚙️vite.config.ts");
const allVites = findFile("/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules", (n)=>n==="⚙️vite.config.ts");
console.log("vite configs under os modules", allVites);
for (const p of allVites) {
  const lines = readFileSync(p,"utf8").split("\n").filter(l=>/flow-core|framework-surface|framework-editor|alias:|find:/.test(l));
  if (lines.length) {
    console.log("ALIASES", p);
    console.log(lines.join("\n"));
  }
}
