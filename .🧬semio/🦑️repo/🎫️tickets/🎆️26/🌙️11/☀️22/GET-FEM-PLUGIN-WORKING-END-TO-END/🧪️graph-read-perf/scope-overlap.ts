import {readFileSync,readdirSync,statSync} from "node:fs";
import {createHash} from "node:crypto";
const dir="C:\\semio\\.cache\\b\\g\\m\\selected\\";
const files=readdirSync(dir).filter(n=>n.endsWith(".admission.json")).map(n=>({n,m:statSync(dir+n).mtimeMs})).sort((a,b)=>b.m-a.m);
const sets=new Map<string,Map<string,string>>();
for(const f of files){
 const memo=JSON.parse(readFileSync(dir+f.n,"utf8"));
 const m=new Map<string,string>();
 for(const o of memo.owners)m.set(o.suffix,createHash("sha1").update(JSON.stringify(o.facts)).digest("hex"));
 sets.set(f.n.slice(0,8),m);
 console.log(f.n.slice(0,8),new Date(f.m).toISOString(),"owners",m.size,"format",String(memo.format).slice(0,12));
}
const names=[...sets.keys()];
const base=names[0];
console.log("base (newest)",base);
for(const other of names.slice(1)){
 const a=sets.get(base)!,b=sets.get(other)!;let shared=0,same=0;
 for(const [k,v] of a){if(b.has(k)){shared++;if(b.get(k)===v)same++;}}
 console.log(other,"suffix shared",shared,"/",a.size,"facts identical",same);
}
