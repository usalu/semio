import {readFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {dirname,resolve,join,relative} from "node:path";
import {execFileSync} from "node:child_process";
const REPO="/Users/ueli/Documents/semio";
const SRC=REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const {readRustSourceClosureV1}=await import(SRC);
const TARGET=process.env.GC_TARGET??"wasm32-wasip2";
const tomlOf=(p:string)=>Bun.TOML.parse(readFileSync(p,"utf8")) as any;
const manifests=execFileSync("rg",["--files","-g","Cargo.toml","-g","!node_modules","-g","!target","-g","!.cache",REPO],{maxBuffer:1<<28,encoding:"utf8"}).split("\n").filter(Boolean);
const byName=new Map<string,string[]>(),info=new Map<string,any>();
for(const m of manifests){try{const t=tomlOf(m);if(!t.package?.name){if(t.workspace)info.set(dirname(m),{dir:dirname(m),toml:t,name:""});continue;}const dir=dirname(m);info.set(dir,{dir,toml:t,name:t.package.name});(byName.get(t.package.name)??byName.set(t.package.name,[]).get(t.package.name)!).push(dir);}catch{}}
const wsRoots=[...info.values()].filter(i=>i.toml.workspace).map(i=>i.dir).sort((a,b)=>b.length-a.length);
const wsOf=(dir:string)=>{const inl=info.get(dir)?.toml;for(const r of wsRoots){if(dir===r||dir.startsWith(r+"/"))return r;}return null};
const wsDeps=(root:string)=>info.get(root)?.toml.workspace?.dependencies??{};
const tgtOk=(key:string)=>{if(!/^cfg\(/.test(key))return key===TARGET;const k=key.replace(/\s+/g,"");if(/not\(target_arch="wasm32"\)|not\(target_family="wasm"\)|not\(target_os="wasi"\)|unix|windows|target_os="(macos|linux|ios|android)"|target_arch="(x86|x86_64|aarch64)"|not\(any\([^)]*wasm/.test(k)&&!/target_arch="wasm32"[^n]*$/.test(k)&&!/^cfg\(target_arch="wasm32"\)$/.test(k))return false;return /wasm|wasi/.test(k)||!/target_/.test(k);};
function depsOf(dir:string){const t=info.get(dir).toml,ws=wsOf(dir),out:{name:string;dir:string;optional:boolean;features:string[];noDefault:boolean}[]=[];const tables:any[]=[];for(const k of ["dependencies","build-dependencies"])if(t[k])tables.push(t[k]);for(const [key,v] of Object.entries<any>(t.target??{}))if(tgtOk(key))for(const k of ["dependencies","build-dependencies"])if(v[k])tables.push(v[k]);
 const inherit=ws?wsDeps(ws):{};
 for(const tab of tables)for(const [key,spec0] of Object.entries<any>(tab)){let spec=spec0,base=dir;if(typeof spec==="object"&&spec.workspace){const w=inherit[key];if(!w)continue;spec={...(typeof w==="string"?{}:w),...spec};base=ws!;}
  if(typeof spec!=="object"||!spec.path)continue;const d=resolve(base,spec.path);if(!info.has(d))continue;out.push({name:info.get(d).name,dir:d,optional:!!spec.optional,features:spec.features??[],noDefault:spec["default-features"]===false});}
 return out;}
const requested=new Map<string,Set<string>>(),wantsDefault=new Set<string>();
const expand=(dir:string,seed:string[])=>{const f=info.get(dir).toml.features??{},out=new Set<string>(),q=[...seed];while(q.length){const x=q.pop()!;if(out.has(x)||x.startsWith("dep:")||x.includes("/"))continue;out.add(x);for(const y of f[x]??[])q.push(y);}return [...out].filter(x=>x in f)};
const featuresFor=(dir:string)=>expand(dir,[...(wantsDefault.has(dir)||!requested.has(dir)?["default"]:[]),...(requested.get(dir)??[])]);
const closure=(roots:string[])=>{for(const r of roots)wantsDefault.add(r);const seen=new Map<string,{viaOptionalOnly:boolean;parent?:string}>(),q:[string,boolean,string?][]=roots.map(r=>[r,false]);while(q.length){const [d,opt,par]=q.shift()!;const prev=seen.get(d);if(prev&&(!prev.viaOptionalOnly||opt))continue;seen.set(d,{viaOptionalOnly:opt,parent:par});for(const e of depsOf(d)){if(!e.noDefault)wantsDefault.add(e.dir);(requested.get(e.dir)??requested.set(e.dir,new Set()).get(e.dir)!);for(const f of e.features)requested.get(e.dir)!.add(f);q.push([e.dir,opt||e.optional,d]);}}return seen};
const cfgCache=new Map<string,string[]>();
const rustcCfg=(features:string[])=>{const k=features.join(",");if(!cfgCache.has(k)){const a=["--print","cfg","--target",TARGET,...features.flatMap(f=>["--cfg",`feature="${f}"`])];cfgCache.set(k,execFileSync("rustc",a,{encoding:"utf8"}).trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q))));}return cfgCache.get(k)!};
const POL={bytes:262144,files:32,lines:4000};
export async function measure(dir:string,features:string[]=[]){const i=info.get(dir),t=i.toml;const lib=t.lib?.path?resolve(dir,t.lib.path):join(dir,"src/lib.rs");const entries:string[]=[];if(existsSync(lib))entries.push(lib);else return null;
 const b=t.package?.metadata?.semio?.["rust-source-boundary"];const lim=b?{bytes:Math.min(POL.bytes,b.maxSourceBytes),files:Math.min(POL.files,b.maxSourceFiles),lines:Math.min(POL.lines,b.maxActiveLines)}:POL;
 const root=REPO;
 let inputs:any[],err:string|undefined;
 try{inputs=await readRustSourceClosureV1([lib],root,rustcCfg(features),async(p:string)=>{try{return new Uint8Array(readFileSync(p));}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});}catch(e:any){err=String(e.message??e).slice(0,200);inputs=[];}
 const bytes=inputs.reduce((s,x)=>s+x.bytes,0),files=inputs.length,lines=inputs.reduce((s,x)=>s+x.activeLines,0);
 const top=[...inputs].sort((a,b)=>b.bytes-a.bytes).slice(0,5).map(x=>`${relative(dir,resolve(root,x.path)).slice(-70)}:${x.bytes}`);
 return {name:i.name,dir:relative(REPO,dir),bytes,files,lines,lim,over:{bytes:bytes-lim.bytes,files:files-lim.files,lines:lines-lim.lines},top,err};}
if(false){
 const rootNames=readFileSync(process.argv[2],"utf8").split("\n").map(s=>s.trim()).filter(Boolean);
 const roots:string[]=[];for(const n of rootNames){const ds=byName.get(n);if(!ds){console.error("NOROOT",n);continue}roots.push(...ds);}
 const cl=closure(roots);const rows:any[]=[];
 for(const [d,m] of cl){const r=await measure(d,featuresFor(d));if(!r)continue;{const chain:string[]=[];let c:string|undefined=d;while(c){chain.push(info.get(c).name);c=cl.get(c)?.parent;}rows.push({...r,viaOptionalOnly:m.viaOptionalOnly,chain:chain.slice(0,8)});}}
 console.log(JSON.stringify({closure:cl.size,rows}));}
{
 const rootNames=readFileSync(process.argv[2],"utf8").split("\n").map(s=>s.trim()).filter(Boolean);
 const roots:string[]=[];for(const n of rootNames){const ds=byName.get(n);if(ds)roots.push(...ds);}
 const cl=closure(roots);
 const isIfc=(n:string)=>/^semio-s-artifact-stdio-ifc/.test(n);
 const out:string[]=[];
 for(const [d] of cl){const me=info.get(d).name;if(isIfc(me))continue;const hit=depsOf(d).filter(e=>isIfc(e.name)).map(e=>e.name+(e.optional?"?":""));if(hit.length){const ch:string[]=[];let c:string|undefined=d;while(c){ch.push(info.get(c).name);c=cl.get(c)?.parent;}out.push(`${me}\t${hit.join(",")}\t${ch.join(" < ")}`);}}
 console.log(out.join("\n"));
 console.log("closure",cl.size);
}
