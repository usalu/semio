import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
const R="/Users/ueli/Documents/semio",SKIP=new Set(["node_modules","target",".git","🗑️generated","dist",".🧬semio"]);
const bad:string[]=[];let n=0;
const kinds=["dependencies","dev-dependencies","build-dependencies"];
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(SKIP.has(e.name)||e.name.startsWith(".cache"))continue;walk(join(d,e.name));}else if(e.name==="Cargo.toml"){const p=join(d,e.name);let t:any;try{t=Bun.TOML.parse(readFileSync(p,"utf8"));}catch{continue;}
 if(!t.features)continue;n++;
 const deps=new Map<string,any>();const add=(tab:any)=>{for(const[k,v]of Object.entries<any>(tab??{}))deps.set(k,v);};
 for(const k of kinds)add(t[k]);for(const tv of Object.values<any>(t.target??{}))for(const k of kinds)add(tv[k]);
 const feats=new Set(Object.keys(t.features));
 for(const[f,list]of Object.entries<string[]>(t.features))for(const s of list){
  let m=s.match(/^dep:(.+)$/);if(m){if(!deps.has(m[1]))bad.push(`${p.slice(R.length+1)} ${f}: ${s}`);continue;}
  m=s.match(/^([^/?]+)\??\/(.+)$/);if(m){if(!deps.has(m[1]))bad.push(`${p.slice(R.length+1)} ${f}: ${s}`);continue;}
  if(!feats.has(s)&&!deps.has(s))bad.push(`${p.slice(R.length+1)} ${f}: ${s}`);}
}}};
walk(R);console.log(n,bad.length);for(const b of bad.slice(0,60))console.log(b.slice(-260));
