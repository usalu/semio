import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
// usage: peek.ts <project-name> <target>
const [pn, tn] = process.argv.slice(2);
for (const f of projectFiles) { const j=readJson(f); if (j.name!==pn) continue; const t=j.targets?.[tn]; if(!t){console.log("no target");continue;} 
 console.log(JSON.stringify(t).slice(0,600));
 const dir=dirname(f); const cmd:string=t.options?.command??""; const m=cmd.match(/(\S+\.ts)/); const sp = m? join(ROOT, t.options?.cwd?.startsWith("🧰️") ? t.options.cwd : dir, m[1].replace(/^["']|["']$/g,"")) : join(ROOT,dir,"📜️script.ts"); 
 const own = join(ROOT,dir,"📜️script.ts"); const file = existsSync(own)? own : sp;
 const src=readFileSync(file,"utf8"); const words = cmd.split(/\s+/).filter(w=>!w.startsWith("-")&&!w.endsWith(".ts")&&w!=="bun"); const key = tn.replace(/-source$|-native$/,"");
 const re = new RegExp(`register\\(\\s*"(${[...words,tn,key].filter(Boolean).map(s=>s.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|")})"\\s*,\\s*(\\w+)`,"g");
 for (const mm of src.matchAll(re)) { const cls=mm[2]; const i=src.indexOf("class "+cls); console.log("---",mm[1],"->",cls); if(i>=0) console.log(src.slice(i, i+900)); }
}
