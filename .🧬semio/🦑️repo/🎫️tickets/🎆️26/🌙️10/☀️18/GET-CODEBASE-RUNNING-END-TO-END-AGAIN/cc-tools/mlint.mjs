import {readFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {join,resolve,dirname} from "node:path";
const roots=process.argv.slice(2);
const files=[];
const walk=d=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.name==="node_modules"||e.name===".git"||e.name==="target"||e.name===".cache"||e.name==="🗑️generated"||e.name.startsWith(".🧬"))continue;const p=join(d,e.name);if(e.isDirectory())walk(p);else if(e.name==="Cargo.toml")files.push(p);}};
for(const r of roots)walk(r);
const parsed=new Map();
const load=f=>{if(parsed.has(f))return parsed.get(f);let v=null;try{v=Bun.TOML.parse(readFileSync(f,"utf8"));}catch(e){v={__err:String(e).slice(0,200)};}parsed.set(f,v);return v;};
const out=[];
const depTables=m=>{const r=[];for(const k of ["dependencies","dev-dependencies","build-dependencies"])if(m[k])r.push([k,m[k]]);for(const [t,v] of Object.entries(m.target??{}))for(const k of ["dependencies","dev-dependencies","build-dependencies"])if(v[k])r.push([`target.${t}.${k}`,v[k]]);return r;};
for(const f of files){
 const m=load(f);const dir=dirname(f);
 if(m.__err){out.push(`${f}: PARSE ${m.__err}`);continue;}
 if(!m.package)continue;
 const tables=depTables(m);
 const names=new Map();
 for(const [t,deps] of tables)for(const [k,v] of Object.entries(deps))names.set(k,(names.get(k)??[]).concat([[t,v]]));
 const wsDir=m.package.workspace?resolve(dir,m.package.workspace):null;
 let ws=null;if(wsDir&&existsSync(join(wsDir,"Cargo.toml")))ws=load(join(wsDir,"Cargo.toml"));
 for(const [t,deps] of tables)for(const [k,v] of Object.entries(deps)){
  const o=typeof v==="string"?{version:v}:v;
  if(t.includes("dev-dependencies")&&o.optional)out.push(`${f}: optional dev-dependency ${k}`);
  if(o.workspace){if(ws&&!(ws.workspace?.dependencies??{})[k])out.push(`${f}: workspace dep ${k} missing in ${wsDir}`);}
  else if(!o.path&&!o.version&&!o.git)out.push(`${f}: dep ${k} without source`);
  if(o.path){const p=resolve(dir,o.path);const tf=join(p,"Cargo.toml");if(!existsSync(tf)){out.push(`${f}: path dep ${k} -> missing ${p}`);continue;}
   const tm=load(tf);if(tm.package){const pn=o.package??k;if(tm.package.name!==pn)out.push(`${f}: dep ${k} path name ${tm.package.name} != ${pn}`);
    for(const ft of o.features??[]){if(!(tm.features??{})[ft]&&!(tm.dependencies??{})[ft.replace(/^dep:/,"")])out.push(`${f}: dep ${k} feature ${ft} absent in ${tm.package.name}`);}}}
 }
 for(const [fn,list] of Object.entries(m.features??{}))for(const it of list){
  if(it.startsWith("dep:")){const n=it.slice(4);if(!names.has(n))out.push(`${f}: feature ${fn} dep:${n} not a dependency`);}
  else if(it.includes("/")){const [n]=it.split("/");const nn=n.replace(/\?$/,"");if(!names.has(nn))out.push(`${f}: feature ${fn} ${it}: ${nn} not a dependency`);}
  else if(!(m.features[it])&&!names.has(it))out.push(`${f}: feature ${fn} refers unknown ${it}`);
 }
}
console.log(out.join("\n"));console.error(`checked ${files.length} manifests, ${out.length} findings`);
