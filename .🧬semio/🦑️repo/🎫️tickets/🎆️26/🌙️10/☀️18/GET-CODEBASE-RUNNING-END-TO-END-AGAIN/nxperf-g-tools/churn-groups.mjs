import {pathToFileURL} from "node:url";
import {statSync} from "node:fs";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const m=await import(pathToFileURL(lib+"🟨️.mjs").href),e=await import(pathToFileURL(lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts").href);
const hours=Number(process.argv[2]??6),now=Date.now(),depth=Number(process.argv[3]??2);
const {keyed}=e.splitNxEngineFilesV1(m.nxBuildEngineClosureV1().files);
const short=p=>p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","LIB/").replace("🧰️framework/🔨️modules/","FM/");
const groups=new Map();let n=0;
for(const f of keyed){if(now-statSync(f).mtimeMs>hours*3600000)continue;n++;const k=short(f).split("/").slice(0,depth+(short(f).startsWith("LIB/")||short(f).startsWith("FM/")?1:0)).join("/");groups.set(k,(groups.get(k)??0)+1);}
console.log(`keyed files changed in ${hours} h: ${n}/${keyed.length}`);
for(const [k,c] of [...groups].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(String(c).padStart(4),k);
