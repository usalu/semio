import {readFileSync} from "node:fs";
import {join,normalize,relative} from "node:path";
const root=process.argv[2];
const ws=Bun.TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")).workspace;
const members=new Set(ws.members.map(m=>normalize(join(root,m))));
const wsdeps=ws.dependencies??{};
const secs=["dependencies","dev-dependencies","build-dependencies"];
function deps(t){const out=[];for(const s of secs)for(const [k,v] of Object.entries(t[s]??{}))out.push([s,k,v]);for(const [tg,tv] of Object.entries(t.target??{}))for(const s of secs)for(const [k,v] of Object.entries(tv[s]??{}))out.push([tg+s,k,v]);return out;}
const bad=new Map();
for(const m of [...members].sort()){let t;try{t=Bun.TOML.parse(readFileSync(join(m,"Cargo.toml"),"utf8"));}catch(e){console.log("ERR",m,String(e).slice(0,100));continue;}
 for(const [s,k,v] of deps(t)){let path;if(v&&typeof v==="object"){if(v.workspace){const w=wsdeps[k];path=w&&typeof w==="object"&&w.path?join(root,w.path):undefined;}else if(v.path)path=join(m,v.path);}
  if(path){path=normalize(path);if(!members.has(path)){const r=relative(root,m);if(!bad.has(r))bad.set(r,[]);bad.get(r).push([s,k,relative(root,path)]);}}}}
for(const [m,l] of bad){console.log(m);for(const x of l)console.log("   ",x.join(" | "));}
console.log(members.size,"members",bad.size,"violators");
