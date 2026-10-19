import {readFileSync,existsSync,statSync} from "node:fs";
import {join} from "node:path";
const R="/Users/ueli/Documents/semio/";
const f=process.argv[2];
const j=JSON.parse(readFileSync(R+f,"utf8"));
const found=new Set<string>();
const walk=(v:any)=>{ if(typeof v==="string"){ const p=v.replace(/^\.\//,""); if(p.includes("/")&&!p.includes(" ")&&existsSync(R+p)) found.add(p+(statSync(R+p).isDirectory()?"/":"")); } else if(Array.isArray(v)) v.forEach(walk); else if(v&&typeof v==="object") Object.values(v).forEach(walk); };
walk(j);
console.log([...found].sort().join("\n"), "\n#", found.size);
