import {readdirSync,readFileSync,statSync} from "node:fs";
import {join} from "node:path";
import {stampPathV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";
const root="C:/semio/.cache/b/r/m/u/a",rows:any[]=[];
for(const d of readdirSync(root))for(const f of readdirSync(join(root,d)))if(f.endsWith(".json")){try{rows.push({f,age:(Date.now()-statSync(join(root,d,f)).mtimeMs)/3600000,memo:JSON.parse(readFileSync(join(root,d,f),"utf8"))});}catch{}}
let hit=0;const why:Record<string,number>={};const paths=new Set<string>();let stamps=0;
for(const r of rows){let ok=true;for(const s of r.memo.stamps){stamps++;paths.add(s.path);const n=stampPathV1(s.path);for(const k of ["present","dev","ino","size","mtimeNs","ctimeNs"] as const)if(s[k]!==n[k]){ok=false;why[k]=(why[k]??0)+1;}}if(ok)hit++;}
console.log({memos:rows.length,hit,stamps,distinctPaths:paths.size,why,ages:rows.map(r=>r.age).sort((a,b)=>a-b).filter((_,i,a)=>i%Math.ceil(a.length/6)==0).map(x=>x.toFixed(1))});
