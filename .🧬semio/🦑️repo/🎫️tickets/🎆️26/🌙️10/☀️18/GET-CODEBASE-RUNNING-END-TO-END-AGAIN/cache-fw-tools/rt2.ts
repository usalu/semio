import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
const out:string[]=[]; let n=0, d=0;
for (const f of projectFiles) {
  const t = readFileSync(join(ROOT,f),"utf8"); const j=JSON.parse(t);
  if(!Object.keys(j.targets??{}).length) continue; n++;
  const o=JSON.stringify(j,null,2)+"\n";
  if (t!==o && t!==o.trimEnd()) { d++; out.push(f); }
}
console.log(n,d);
const f=out[0]; const t=readFileSync(join(ROOT,f),"utf8"); const o=JSON.stringify(JSON.parse(t),null,2)+"\n";
writeFileSync(join(import.meta.dir,"../🗑️generated/rt-a.json"),t); writeFileSync(join(import.meta.dir,"../🗑️generated/rt-b.json"),o);
console.log(out.map(x=>x.slice(-80)).join("\n"));
