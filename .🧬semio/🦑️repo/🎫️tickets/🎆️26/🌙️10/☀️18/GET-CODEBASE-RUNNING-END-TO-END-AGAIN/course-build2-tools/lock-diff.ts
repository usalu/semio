import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
const root=process.argv[2]??".";
const strip=(t:string)=>t.replace(/,(\s*[}\]])/g,"$1");
const lock=JSON.parse(strip(readFileSync(join(root,"bun.lock"),"utf8")));
const manifest=JSON.parse(readFileSync(join(root,"package.json"),"utf8"));
const names=["dependencies","devDependencies","optionalDependencies","peerDependencies"];
const dirs=["",...(manifest.workspaces??[])];
let bad=0;
const seen=new Set<string>();
for(const dir of dirs){
  const mp=join(root,dir,"package.json");
  if(!existsSync(mp)){console.log("MISSING manifest",dir);bad++;continue;}
  seen.add(dir);
  const m=JSON.parse(readFileSync(mp,"utf8"));
  const l=lock.workspaces[dir];
  if(!l){console.log("NOT IN LOCK",dir);bad++;continue;}
  if((l.name??"")!==(m.name??"")&&dir!=="")console.log("NAME",dir,l.name,m.name);
  for(const n of names){
    const a=JSON.stringify(Object.entries(m[n]??{}).sort()),b=JSON.stringify(Object.entries(l[n]??{}).sort());
    if(a!==b){bad++;const ma=m[n]??{},la=l[n]??{};const keys=new Set([...Object.keys(ma),...Object.keys(la)]);for(const k of keys)if(ma[k]!==la[k])console.log("DIFF",dir||".",n,k,"manifest="+ma[k],"lock="+la[k]);}
  }
}
for(const k of Object.keys(lock.workspaces))if(!seen.has(k))console.log("LOCK-ONLY",k);
console.log("bad",bad);
