import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
const R="/Users/ueli/Documents/semio";
const j = JSON.parse(readFileSync(import.meta.dir + "/../🗑️generated/aud-cachecov.json","utf8"));
const files = [...new Set(j.targets.filter((t:any)=>t.owner==="✏️s" && t.file.includes("🦀️rust")).map((t:any)=>t.file))] as string[];
let outside=0, inside=0; const kinds:any={};
for (const f of files){ const dir=dirname(f); const cargo=join(R,dir,"Cargo.toml"); if(!existsSync(cargo)){kinds.nocargo=(kinds.nocargo??0)+1;continue;}
 const txt=readFileSync(cargo,"utf8"); const paths=[...txt.matchAll(/^path\s*=\s*"([^"]+)"/gm)].map(m=>m[1]).filter(p=>!/framework|\.\.\/\.\.\/\.\.\/\.\./.test(p)||true);
 const hasLib=/^\[lib\]/m.test(txt); const libp=txt.match(/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/)?.[1];
 const k=(libp?(libp.startsWith("../")?"lib-outside":"lib-inside"):hasLib?"lib-default":"nolib"); kinds[k]=(kinds[k]??0)+1;
}
console.log(files.length,kinds);
