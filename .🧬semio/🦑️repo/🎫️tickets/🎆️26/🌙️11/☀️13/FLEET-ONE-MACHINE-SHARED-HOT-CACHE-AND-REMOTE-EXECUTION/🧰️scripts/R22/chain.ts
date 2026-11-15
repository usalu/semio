import {diskZeroTouchTreeV1,scanModuleSpecifiersV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
import {posix} from "node:path";import {readFileSync} from "node:fs";
const R="/Users/Shared/semio";const tree=diskZeroTouchTreeV1(R);
const reg=JSON.parse(readFileSync(R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json","utf8")).commands;
const EXT=[".ts",".tsx",".mts",".js",".mjs"],CODE=/\.(?:[cm]?[jt]s|tsx)$/;
const rt=(from:string,s:string)=>{const b=posix.normalize(posix.join(posix.dirname(from),s.split("?")[0]));for(const c of [b,...EXT.map(e=>b+e)])if(CODE.test(c)&&tree.read(c)!==undefined)return c};
const bare=(s:string)=>Object.keys(tree.packages).some(n=>s===n||s.startsWith(n+"/"));
const want=process.argv.slice(2);
for(const c of reg.filter((c:any)=>want.includes(c.id))){
 const par=new Map<string,[string,string]|null>([[c.module,null]]),q=[c.module];const hits:string[]=[];
 while(q.length){const f=q.shift()!;for(const {specifier,kind} of scanModuleSpecifiersV1(f,tree.read(f)!)){
  if(specifier.startsWith(".")){const t=rt(f,specifier);if(t&&!par.has(t)){par.set(t,[f,kind]);q.push(t)}}
  else if(bare(specifier)&&!specifier.startsWith("node:")){hits.push(f)}}}
 const firsts=new Set<string>();
 console.log("== "+c.id);
 const seenTop=new Set<string>();
 for(const h of new Set(hits)){const ch=[h];let p=par.get(h);while(p){ch.unshift(p[0]+"["+p[1]+"]");p=par.get(p[0])}
  const key=ch.slice(0,Math.min(ch.length,8)).join(" > ");
  const short=(x:string)=>x.replace(/🧰️framework\//,"").replace(/🛍️products\/🦑️repo\/🔨️modules\/📚️library\//,"L/");
  console.log(short(ch.slice(0,-1).slice(-4).join(" > "))+" => "+short(h)+"  (depth "+ch.length+")");}
}
