import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles } from "./lib.ts";
let same=0, diff=0; const bad:string[]=[];
for (const f of projectFiles) {
  const t = readFileSync(join(ROOT,f),"utf8");
  let j; try { j = JSON.parse(t);} catch { bad.push(f); continue; }
  const out2 = JSON.stringify(j,null,2)+"\n", out3=JSON.stringify(j,null,2);
  if (t===out2||t===out3) same++; else { diff++; if(diff<6) bad.push("DIFF "+f.slice(-60)); }
}
console.log({same,diff,total:projectFiles.length}); console.log(bad.join("\n"));
