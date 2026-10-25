// usage: bun derive-standalone.mjs <project> <target> <rootDir> <out.json>
// Runs the precision engine of the CURRENT rules with a read-only host (no custody, no lock marker): derived patterns, files, groups.
import {readFileSync,writeFileSync,existsSync,readdirSync} from "node:fs";
import {join,resolve,relative} from "node:path";
const ws="/Users/ueli/Documents/semio";
const L=ws+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const engine=await import(L+"/🏗️build/📥️inputs/🎯️precision/🟦️.ts");
const inference=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs");
await inference.libraryBootstrap;
const ci=inference.cacheInternals;
const [project,target,out]=process.argv.slice(2);
const document=engine.readInputPrecisionV1(JSON.parse(readFileSync(L+"/🏗️build/📥️inputs/🎯️precision/🔣️.json","utf8")));
const policy=JSON.parse(readFileSync(L+"/⚡️caching/🔣️policy.json","utf8"));
const rule=engine.selectRuleV1(document,{project,target});
if(!rule){console.log("no rule for",project,target);process.exit(1)}
const toml=(path)=>Bun.TOML.parse(readFileSync(path,"utf8"));
const buildPath=(p)=>p.split("\\").join("/");
const cache={cache:new Map(),closures:new Map()};
const facts=new Map();
const cargoSources=(root,tests)=>{
  const directory=join(ws,root),manifest=join(directory,"Cargo.toml");
  if(!existsSync(manifest))return undefined;
  const cargo=toml(manifest);
  const build=cargo.package?.build;
  const entries=[cargo.lib?.path??"src/lib.rs","src/main.rs",...(cargo.bin??[]).map(t=>t.path),...(tests?[...(cargo.test??[]),...(cargo.bench??[]),...(cargo.example??[])]:[]).map(t=>t.path),...(build===false?[]:[typeof build==="string"?build:"build.rs"])].filter(Boolean).map(p=>resolve(directory,p)).filter(existsSync);
  if(build!==false&&existsSync(resolve(directory,typeof build==="string"?build:"build.rs")))return undefined;
  for(const source of ["src/bin",...(tests?["tests","examples","benches"]:[])]){
    if(!existsSync(join(directory,source)))continue;
    for(const entry of readdirSync(join(directory,source),{withFileTypes:true})){
      const path=join(directory,source,entry.name,...(entry.isDirectory()?["main.rs"]:[]));
      if(path.endsWith(".rs")&&existsSync(path))entries.push(path);
    }
  }
  if(!entries.length)return undefined;
  try{return ci.rustSourceFiles(entries,directory,undefined,facts,!tests).map(f=>`{workspaceRoot}/${buildPath(relative(ws,f))}`);}catch(e){console.error("rustSourceFiles failed",root,e.message);return undefined;}
};
const host={
  exists:(p)=>existsSync(join(ws,p)),
  readText:(p)=>{try{return readFileSync(join(ws,p),"utf8")}catch{return undefined}},
  listDirectory:(p)=>{try{return readdirSync(join(ws,p),{withFileTypes:true}).map(e=>({name:e.name,directory:e.isDirectory()}))}catch{return[]}},
  kinds:JSON.parse(readFileSync(L+"/🔣️taxonomy.json","utf8")).fileKinds,
  importClosure:async()=>[],
  cargoManifests:()=>[],
  cargoRoots:(crate)=>ci.nativeDependencyRoots(crate,ws,true,cache.cache,cache.closures),
  cargoSources,
  cargoToolchain:policy.toolchains.cargo.files,
};
const outputs=JSON.parse(process.env.OUTPUTS??"[]");
const t0=performance.now();
const derived=await engine.deriveInputsV1(document,rule,{project,target,outputs},host);
console.log(rule.id,"ms",Math.round(performance.now()-t0),"patterns",derived.patterns.length,"files",derived.files.length,"groups",derived.groups,"unresolved",derived.unresolved.length,"reads",JSON.stringify(derived.reads));
writeFileSync(out,JSON.stringify(derived));
