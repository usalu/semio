import {readdirSync,readFileSync,writeFileSync,existsSync} from "node:fs";
import {join,dirname,relative,normalize} from "node:path";
const R="/Users/Shared/semio/",SKIP=new Set([".cache","node_modules",".git","target",".🧬semio"]);
const mans:string[]=[];
(function walk(d:string){let es;try{es=readdirSync(d,{withFileTypes:true});}catch{return;}
 if(es.some(e=>e.isFile()&&e.name==="Cargo.toml"))mans.push(d);
 for(const e of es)if(e.isDirectory()&&!SKIP.has(e.name))walk(join(d,e.name));})(R.slice(0,-1));
const raw=new Map<string,any>();
for(const d of mans){try{raw.set(d,Bun.TOML.parse(readFileSync(join(d,"Cargo.toml"),"utf8")));}catch{}}
const wsdeps=new Map<string,Record<string,any>>();
for(const [d,t] of raw)if(t.workspace)wsdeps.set(d,t.workspace.dependencies??{});
const wsroot=(d:string)=>{let x=d;while(x.length>=R.length-1){if(wsdeps.has(x))return x;x=dirname(x);}return undefined;};
const pk=new Map<string,string>();for(const [d,t] of raw)if(t.package)pk.set(d,t.package.name??d);
function deps(d:string,t:any,kinds:string[]):string[]{
 const out:string[]=[];let ws:string|undefined;
 for(const k of kinds){const tables=[t[k]??{},...Object.values<any>(t.target??{}).map(x=>x[k]??{})];
  for(const tb of tables)for(const [name,v] of Object.entries<any>(tb)){
   if(typeof v!=="object")continue;let p=v.path;
   if(p===undefined&&v.workspace){ws??=wsroot(d);const w=ws?wsdeps.get(ws)?.[name]:undefined;if(w?.path)p=relative(d,join(ws!,w.path));}
   if(p)out.push(normalize(join(d,p)));}}
 return out;}
const g=new Map<string,string[]>(),gt=new Map<string,string[]>();
for(const d of pk.keys()){g.set(d,deps(d,raw.get(d),["dependencies","build-dependencies"]).filter(x=>pk.has(x)));gt.set(d,deps(d,raw.get(d),["dependencies","build-dependencies","dev-dependencies"]).filter(x=>pk.has(x)));}
const rev=(m:Map<string,string[]>)=>{const r=new Map<string,Set<string>>();for(const [a,bs] of m)for(const b of bs){if(!r.has(b))r.set(b,new Set());r.get(b)!.add(a);}return r;};
const closure=(r:Map<string,Set<string>>,s:string)=>{const seen=new Set<string>(),st=[s];while(st.length){const x=st.pop()!;for(const y of r.get(x)??[])if(!seen.has(y)){seen.add(y);st.push(y);}}return seen;};
const r=rev(g),rt=rev(gt);
const rows=[...pk.keys()].map(d=>({n:closure(r,d).size,nt:closure(rt,d).size,name:pk.get(d)!,dir:relative(R,d)})).sort((a,b)=>b.n-a.n);
const rel=(d:string)=>relative(R,d);
writeFileSync(process.argv[2],JSON.stringify({crates:pk.size,graph:Object.fromEntries([...g].map(([d,v])=>[rel(d),v.map(rel)])),graphTests:Object.fromEntries([...gt].map(([d,v])=>[rel(d),v.map(rel)])),names:Object.fromEntries([...pk].map(([d,n])=>[rel(d),n])),top:rows.slice(0,40)}));
console.log(pk.size,"crates",[...g.values()].reduce((a,b)=>a+b.length,0),"edges");
for(const x of rows.slice(0,14))console.log(x.n,x.nt,x.name,x.dir);
