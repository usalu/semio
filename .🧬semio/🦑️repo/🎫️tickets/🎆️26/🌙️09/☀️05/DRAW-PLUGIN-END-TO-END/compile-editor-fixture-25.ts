/** 🧪️ Compiles a Draw editor companion against one previously built editor dependency closure. */
import { readdirSync, readFileSync, writeFileSync, statSync, existsSync } from "node:fs";
import { join, resolve, dirname, basename } from "node:path";
const [manifestPath, fingerprint] = Bun.argv.slice(2);
const generated = join(import.meta.dir, "🗑️generated");
const build = join(generated, "e25/b/debug/build");
const manifest = Bun.TOML.parse(readFileSync(manifestPath,"utf8")) as any;
type Provider = {name:string; pkg:string; fp:string; id:string; features:string[]; deps:any[]; out:string; time:number};
const byDigest = new Map<string,Provider>(), packages = new Map<string,Provider[]>(), outs = new Set<string>();
for (const pkg of readdirSync(build)) for (const id of readdirSync(join(build,pkg))) {
  const dir=join(build,pkg,id,"fingerprint"),out=join(build,pkg,id,"out");
  if(existsSync(out))outs.add(out);
  if(!existsSync(dir))continue;
  for(const leaf of readdirSync(dir).filter(v=>v.startsWith("lib-")&&v.endsWith(".json"))) {
    const file=join(dir,leaf),key=file.slice(0,-5);
    if(!existsSync(key))continue;
    const data=JSON.parse(readFileSync(file,"utf8")),fp=readFileSync(key,"utf8").trim(),name=leaf.slice(4,-5);
    if(!/^[0-9a-f]{16}$/.test(fp))continue;
    const provider={name,pkg,fp,id,features:JSON.parse(data.features),deps:data.deps,out,time:statSync(file).mtimeMs};
    byDigest.set(fp,provider);
    packages.set(pkg,[...(packages.get(pkg)??[]),provider]);
  }
}
const selected = new Map<string,Provider>(), visited=new Set<string>();
function walk(p:Provider){
  if(visited.has(p.fp))return;
  visited.add(p.fp);
  if(!selected.has(p.pkg))selected.set(p.pkg,p);
  for(const row of p.deps){const d=byDigest.get(row[3]);if(d)walk(d);}
}
for(const list of packages.values())for(const p of list){
  const file=join(build,p.pkg,p.id,"fingerprint","lib-"+p.name+".json");
  p.deps=Array.from(readFileSync(file,"utf8").matchAll(/\[\d+,"([^"]+)",(?:true|false),(\d+)\]/g),m=>[null,m[1],false,Buffer.from(BigInt(m[2]).toString(16).padStart(16,"0"),"hex").reverse().toString("hex")]);
}
const seed=packages.get("semio-s-artifact-draw-drawing-editor")?.find(p=>p.id===fingerprint);
if(!seed)throw Error("Missing built editor "+fingerprint);
walk(seed);
const name=manifest.package.name.replaceAll("-","_"),directory=dirname(resolve(manifestPath)),dst=join(generated,"focused-"+name+(process.env.DRAW_FIXTURE_VARIANT??""));
const args=[resolve(directory,manifest.lib.path),"--test","--edition=2021","--crate-name",name,"-Zthreads=1","-Cdebuginfo=0","-o",dst,...[...outs].map(o=>"-Ldependency="+o)];
const workspace = Bun.TOML.parse(readFileSync(resolve(directory,manifest.package.workspace,"Cargo.toml"),"utf8")) as any;
for(const [alias,localSpec] of Object.entries<any>({...manifest.dependencies,...manifest["dev-dependencies"]})){
 const spec=localSpec.workspace?{...workspace.workspace.dependencies[alias],...localSpec}:localSpec;
 const pkg=spec.package??alias,need=spec.features??[];
 const allowed=(p:Provider)=>need.every((f:string)=>p.features.includes(f));
 let p=selected.get(pkg);
 if(!p||!allowed(p)){
   const candidates=(packages.get(pkg)??[]).filter(allowed);
   const score=(a:Provider)=>a.deps.reduce((n,row)=>{const d=byDigest.get(row[3]);if(!d||!d.pkg.startsWith("semio-"))return n;const known=selected.get(d.pkg);return n+(known?(known.fp===d.fp?1:-100):0);},0);
   p=candidates.sort((a,b)=>score(b)-score(a)||b.time-a.time)[0];
 }
 if(!p)throw Error("Missing built provider "+pkg+" "+need);
 const artifacts=readdirSync(p.out).filter(n=>n.startsWith("lib"+p.name+"-")&&/\.(rlib|rmeta|dylib|so|dll)$/.test(n));
 if(!artifacts.length)throw Error("Missing artifacts "+p.out);
 for(const a of artifacts)args.push("--extern="+(spec.package?alias.replaceAll("-","_"):p.name)+"="+join(p.out,a));
}
writeFileSync(dst+".args",args.join("\n")+"\n");
const child=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+dst+".args"],{env:{...process.env,CARGO_MANIFEST_DIR:directory,CARGO_PKG_NAME:manifest.package.name,CARGO_PKG_VERSION:"0.1.0",RUST_MIN_STACK:"134217728",SEMIO_BUILD_MODE:"prototype"},stdout:"inherit",stderr:"inherit"});
process.exit(await child.exited);
