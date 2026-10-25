import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,relative,resolve,sep,join,isAbsolute} from "node:path";
import {isBuiltin} from "node:module";
const root=process.cwd();
const parser=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs"));
const scripts=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"));
const policy=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","utf8"));
const boundaries=new Set<string>(((process.env.NO_BOUNDARY==="1"?[]:policy.spawnBoundaries)??[]).map((x:string)=>resolve(root,x)));
const presentation=new Set<string>((policy.presentationResources??[]).map((x:string)=>resolve(root,x)));
const designationPath=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🔣️.json");
const designation=JSON.parse(readFileSync(designationPath,"utf8")).providers as Record<string,string>;
const ALL_PROVIDERS=process.env.ALL_PROVIDERS==="1";
const loaderPath=resolve(root,"🧰️framework/🔨️modules/🏃️process/🔌️providers/🟦️.ts");
const entries=process.argv.slice(2).map(x=>resolve(root,x));
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile();
const skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
const isModule=(p:string)=>/\.[cm]?[jt]sx?$/u.test(p);
const edgesCache=new Map<string,{to:string;kind:string}[]>();
const edges=(path:string)=>{
  if(edgesCache.has(path))return edgesCache.get(path)!;
  const out:{to:string;kind:string}[]=[];
  if(isModule(path)){
    const text=readFileSync(path,"utf8");
    for(const spec of parser.commandSourceImports(path,text)){const p=resolve(dirname(path),spec);const r=regular(p)?p:(()=>{try{return Bun.resolveSync(spec,dirname(path));}catch{return undefined}})();if(r&&!skip(r)&&r.startsWith(root))out.push({to:r,kind:"static"});}
    for(const spec of parser.runtimeModuleSourceImports(path,text)){if(spec.startsWith(".")||isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))continue;try{const r=Bun.resolveSync(spec,dirname(path));if(!skip(r)&&r.startsWith(root))out.push({to:r,kind:"package"});}catch{}}
    for(const lit of parser.commandUrlReferences(path,text)){const t=resolve(dirname(path),lit);if(!regular(t)||skip(t))continue;if(boundaries.has(t))continue;out.push({to:t,kind:"url"});}
    if(!ALL_PROVIDERS){const ports=scripts.scriptProviderPortsV1(text);if(ports.length){out.push({to:designationPath,kind:"resource"});const names=ports.includes("*")?Object.keys(designation):ports.filter((n:string)=>Object.hasOwn(designation,n));for(const n of names)out.push({to:resolve(dirname(designationPath),designation[n]!),kind:"provider"});}}
    else if(path===loaderPath){out.push({to:designationPath,kind:"resource"});for(const m of Object.values(designation))out.push({to:resolve(dirname(designationPath),m),kind:"provider"});}
  }
  edgesCache.set(path,out);return out;
};
const union=new Set<string>();const result:Record<string,string[]>={};
for(const e of entries){
  const seen=new Set([e]),q=[e];
  while(q.length){const p=q.shift()!;if(presentation.has(p))continue;for(const {to} of edges(p)){if(seen.has(to)||!regular(to))continue;seen.add(to);q.push(to);}}
  result[relative(root,e)]=[...seen].map(x=>relative(root,x));for(const x of seen)union.add(x);
  console.log(String(seen.size).padStart(5),"tests",String([...seen].filter(x=>x.includes("🧪️tests")).length).padStart(3),relative(root,e).replace("🧰️framework/🛍️products/","").slice(-80));
}
console.log("union",union.size);
if(process.env.OUT)await Bun.write(process.env.OUT,JSON.stringify(result));
if(process.env.DOM){
  const start=entries[0]!;
  const reach=(skipNode?:string)=>{const s=new Set([start]),st=[start];while(st.length){const u=st.pop()!;if(presentation.has(u))continue;for(const {to} of edges(u)){if(s.has(to)||to===skipNode||!regular(to))continue;s.add(to);st.push(to);}}return s;};
  const total=reach().size;const rows:{n:string;d:number}[]=[];
  for(const n of reach()){if(n===start)continue;rows.push({n,d:total-reach(n).size});}
  rows.sort((a,b)=>b.d-a.d);
  const incoming=new Map<string,{from:string;kind:string}[]>();
  for(const u of reach())for(const e of edges(u)){const l=incoming.get(e.to)??[];l.push({from:u,kind:e.kind});incoming.set(e.to,l);}
  const short=(p:string)=>relative(root,p).replace("🧰️framework/","").split("/").slice(-4).join("/");
  for(const r of rows.slice(0,Number(process.env.DOM))){console.log(String(r.d).padStart(5),short(r.n),"<-",(incoming.get(r.n)??[]).slice(0,5).map(x=>short(x.from).split("/").slice(-2).join("/")+"["+x.kind+"]").join(", ").slice(0,220));}
}
