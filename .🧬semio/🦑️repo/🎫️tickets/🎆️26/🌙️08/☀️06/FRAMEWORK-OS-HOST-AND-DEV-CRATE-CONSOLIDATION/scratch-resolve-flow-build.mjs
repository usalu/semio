import { existsSync, readFileSync, readdirSync, statSync, realpathSync } from "fs";
import { join } from "path";

function walk(root, pred, depth=0, out=[]) {
  if (depth>12 || !existsSync(root)) return out;
  let ents; try { ents=readdirSync(root);} catch { return out; }
  for (const name of ents) {
    if (["node_modules","target",".git","dist"].includes(name)) continue;
    const p=join(root,name);
    let st; try { st=statSync(p);} catch { continue; }
    if (st.isDirectory()) walk(p,pred,depth+1,out);
    else if (pred(name,p)) out.push(p);
  }
  return out;
}

const nm = "/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core";
console.log("nm listing", readdirSync(nm));
try { console.log("realpath", realpathSync(nm)); } catch(e) { console.log("realpath err", e.message); }
console.log("pkg", readFileSync(join(nm,"package.json"),"utf8"));

// workspace package.json packages field / overrides
const rootPkg = JSON.parse(readFileSync("/Users/ueli/Documents/semio/package.json","utf8"));
console.log("workspaces", rootPkg.workspaces);
const overrides = rootPkg.overrides || rootPkg.resolutions;
console.log("overrides keys with flow", Object.keys(overrides||{}).filter(k=>/flow/i.test(k)));

// find all package.json with name flow-core
const pkgs = walk("/Users/ueli/Documents/semio/𝒯framework", (n)=>n==="package.json")
  .concat(walk("/Users/ueli/Documents/semio/✏️s", (n)=>n==="package.json"))
  .filter(p => {
    try { return JSON.parse(readFileSync(p,"utf8")).name === "@semio-tech/flow-core"; } catch { return false; }
  });
console.log("flow-core package.json locations", pkgs);

// list flow module tree shallow
const flow = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow";
function tree(dir, depth=0, max=3) {
  if (depth>max || !existsSync(dir)) return;
  let ents; try { ents=readdirSync(dir);} catch { return; }
  for (const name of ents) {
    const p=join(dir,name);
    let st; try { st=statSync(p);} catch { continue; }
    console.log("  ".repeat(depth)+ (st.isDirectory()?"[D] ":"[F] ")+name+(st.isFile()?` (${st.size})`:""));
    if (st.isDirectory() && !["target","node_modules","pkg"].includes(name)) tree(p, depth+1, max);
    if (st.isDirectory() && name==="pkg") {
      for (const n of readdirSync(p)) console.log("  ".repeat(depth+1)+"[F] "+n+` (${statSync(join(p,n)).size})`);
    }
  }
}
console.log("FLOW TREE");
tree(flow);

// find wasm scripts mentioning flow_core
const scripts = walk("/Users/ueli/Documents/semio/𝒯framework", (n)=>n==="📜️script.ts").filter(p=>{
  try { return /flow_core|flow-core|wasm-pack/.test(readFileSync(p,"utf8")); } catch { return false; }
});
console.log("scripts mentioning flow/wasm-pack count", scripts.length);
for (const s of scripts.slice(0,30)) {
  const t=readFileSync(s,"utf8");
  if (/flow_core|@semio-tech\/flow-core/.test(t)) console.log("HIT", s);
}
