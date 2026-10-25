import {resolve,relative,dirname} from "node:path";
import {closure} from "./closure-of.ts";
const bz=resolve(process.argv[2]!);
const specs=process.argv.slice(3);
const roots=specs.map(s=>resolve(bz,s));
const all=closure(roots);
console.log("union",all.size);
const by=new Map<string,number>();
for(const f of all){const r=relative(resolve(bz,"../../.."),f).split("/").slice(0,4).join("/");by.set(r,(by.get(r)??0)+1);}
for(const [k,v] of [...by].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(String(v).padStart(5),k);
if(process.env.LIST)for(const f of [...all].sort())console.log(relative(bz,f));
