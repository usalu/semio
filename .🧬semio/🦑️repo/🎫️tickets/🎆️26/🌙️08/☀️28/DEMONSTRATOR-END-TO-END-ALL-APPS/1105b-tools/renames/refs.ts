import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
const R="/Users/ueli/Documents/semio",SKIP=new Set(["node_modules","target",".git","🗑️generated","🎫️tickets","dist",".🧬semio"]);
const names=new Set(readFileSync("names.tsv","utf8").split("\n").map(l=>l.split("\t")[0]));
const c=new Map<string,number>();
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(SKIP.has(e.name)||e.name.startsWith(".cache"))continue;walk(join(d,e.name));}else if(e.name==="Cargo.toml"){const t=readFileSync(join(d,e.name),"utf8");
 for(const m of t.matchAll(/^\s*(?:\[(?:[\w.-]*\.)?dependencies\.)?(semio[\w-]*)\]?\s*(?:=|\.workspace)/gm)){const n=m[1];if(!names.has(n))c.set(n,(c.get(n)??0)+1);}
 for(const m of t.matchAll(/package\s*=\s*"([^"]+)"/g)){if(!names.has(m[1]))c.set("pkg:"+m[1],(c.get("pkg:"+m[1])??0)+1);}}}};
walk(R);
for(const[k,v]of [...c].sort((a,b)=>b[1]-a[1]).slice(0,80))console.log(v,k,names.has(k.replace(/^pkg:/,"").replace("semio-framework-","semio-"))?"=>stripped exists":"");
console.log(c.size);
