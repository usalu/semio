import { readFileSync, readdirSync, statSync } from "fs";
import { join } from "path";
function findDir(root, pred, depth=6) {
  if (depth<0) return null;
  for (const n of readdirSync(root)) {
    const p = join(root, n);
    try { if (!statSync(p).isDirectory()) continue; } catch { continue; }
    if (pred(n,p)) return p;
    const hit = findDir(p, pred, depth-1); if (hit) return hit;
  }
  return null;
}
const dslImpl = findDir(".", (n,p) => n.includes("rust") && p.includes("dsl") && p.includes("implementations") && !p.includes("grammar") && !p.includes("family") && !p.includes("plugins") && p.endsWith(n));
console.log("dslImpl", dslImpl);
const lib = readFileSync(join(dslImpl, readdirSync(dslImpl).find(n=>n.includes("lib")||n.endsWith(".rs")&&n.includes("lib"))), "utf8");
const idx = lib.indexOf("struct IdiomHooks");
console.log(lib.slice(Math.max(0,idx-120), idx+900));
console.log("DEFAULT", /impl Default for IdiomHooks/.test(lib));
