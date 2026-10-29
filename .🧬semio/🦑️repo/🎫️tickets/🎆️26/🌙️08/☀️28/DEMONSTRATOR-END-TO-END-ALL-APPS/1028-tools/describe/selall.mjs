import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const { selectRustSourceV1 } = await import(process.argv[2]);
const walk = d => readdirSync(d).flatMap(n => { const p = join(d, n); if (n === "target" || n === "node_modules") return []; return statSync(p).isDirectory() ? walk(p) : p.endsWith(".rs") ? [p] : []; });
let n=0; for (const f of walk(process.argv[3])) { n++; try { selectRustSourceV1(readFileSync(f,"utf8"),["feature=\"x\""].slice(1)); } catch(e){ console.log("FAIL",f,e.message); } }
console.log("scanned",n);
