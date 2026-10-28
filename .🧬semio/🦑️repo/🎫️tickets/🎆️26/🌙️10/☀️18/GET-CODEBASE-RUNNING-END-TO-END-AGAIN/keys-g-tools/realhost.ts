// 🧰 A real-file-system host for the input precision engine: workspace files, taxonomy file kinds, the production import-closure and Cargo analyses.
import {existsSync as fsExists,readFileSync,readdirSync} from "node:fs";
import {join,relative,dirname,resolve} from "node:path";
import type {InputPrecisionHostV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts";
export const W="/Users/ueli/Documents/semio",LIB=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const WS="{workspaceRoot}/";
export async function realHost(precision:string):Promise<{host:InputPrecisionHostV1;scripts:any;internals:any}>{
 const inference:any=await import(join(LIB,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
 const {cacheInternals,createScriptInputCache}=inference,scripts=createScriptInputCache({workspaceRoot:W}),caches={cache:new Map(),closures:new Map(),facts:new Map(),rust:cacheInternals.createRustSourceCache?.()};
 const taxonomy=JSON.parse(readFileSync(join(LIB,"🔣️taxonomy.json"),"utf8")),policy=JSON.parse(readFileSync(join(LIB,"⚡️caching/🔣️policy.json"),"utf8"));
 const rel=(path:string)=>relative(W,path).split("\\").join("/");
 const toml=(path:string)=>Bun.TOML.parse(readFileSync(path,"utf8")) as any;
 const cargoSources=(root:string):readonly string[]|undefined=>{
  const directory=join(W,root),manifest=join(directory,"Cargo.toml");if(!fsExists(manifest))return undefined;
  const cargo=toml(manifest),build=cargo.package?.build,entries=[cargo.lib?.path??"src/lib.rs","src/main.rs",...(cargo.bin??[]).map((t:any)=>t.path),...[...(cargo.test??[]),...(cargo.bench??[]),...(cargo.example??[])].map((t:any)=>t.path),...(build===false?[]:[typeof build==="string"?build:"build.rs"])].filter(Boolean).map((p:string)=>resolve(directory,p)).filter(fsExists);
  if(build!==false&&fsExists(resolve(directory,typeof build==="string"?build:"build.rs")))return undefined;
  for(const source of ["src/bin","tests","examples","benches"]){if(!fsExists(join(directory,source)))continue;for(const entry of readdirSync(join(directory,source),{withFileTypes:true})){const path=join(directory,source,entry.name,...(entry.isDirectory()?["main.rs"]:[]));if(path.endsWith(".rs")&&fsExists(path))entries.push(path);}}
  if(!entries.length)return undefined;
  try{return cacheInternals.rustSourceFiles(entries,directory,caches.rust,new Map()).map((f:string)=>WS+rel(f));}catch{return undefined;}
 };
 const observation:any=await import(join(LIB,"🏗️build/👁️observation/🟨️.mjs")),compiler=(await import("node:module")).createRequire(import.meta.url)("typescript");
 const tolerantClosure=async(entries:readonly string[],program?:{tsconfig:string}):Promise<readonly string[]>=>{
  const seen=new Set<string>(),pending=entries.map(e=>join(W,e)),memo=new Map<string,string[]>(),sys=observation.observedTypeScriptHost(compiler.sys);
  let options:any={moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true};
  if(program){const path=join(W,program.tsconfig),read=compiler.readConfigFile(path,compiler.sys.readFile);options=compiler.parseJsonConfigFileContent(read.config,compiler.sys,dirname(path),undefined,path).options;}
  const resolutionCache=compiler.createModuleResolutionCache(W,(x:string)=>x,options);
  while(pending.length){const file=pending.pop()!;if(seen.has(file)||!fsExists(file))continue;seen.add(file);if(!/\.[cm]?[jt]sx?$/u.test(file))continue;
   let imports=memo.get(file);if(!imports){imports=[];const analysis=program?inference.prepareSourceAnalysesV1().module:inference.prepareSourceAnalysesV1().command,found=observation.observedSourceAnalysisV1(analysis,file),specifiers:string[]=program?found.imports:found;for(const entry of specifiers){let resolved:string|undefined=compiler.resolveModuleName(entry,file,options,sys,resolutionCache).resolvedModule?.resolvedFileName;
    if(!resolved&&!program){try{resolved=observation.observedRequireResolve(file,entry);}catch{}}
    if(resolved&&resolved.startsWith(W+"/")&&!resolved.includes("/node_modules/"))imports.push(resolved);}
    memo.set(file,imports);}
   pending.push(...imports);}
  return [...seen].map(rel).sort();
 };
 const host:InputPrecisionHostV1={
  exists:p=>fsExists(join(W,p)),
  readText:p=>{try{return readFileSync(join(W,p),"utf8");}catch{return undefined;}},
  listDirectory:p=>{try{return readdirSync(join(W,p),{withFileTypes:true}).map(e=>({name:e.name,directory:e.isDirectory()}));}catch{return [];}},
  kinds:taxonomy.fileKinds,
  importClosure:tolerantClosure,
  cargoManifests:()=>Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z","--",":(glob)**/Cargo.toml"],{cwd:W}).stdout.toString().split("\0").filter(Boolean).map(f=>f==="Cargo.toml"?"":f.slice(0,-"/Cargo.toml".length)),
  cargoRoots:crate=>cacheInternals.nativeDependencyRoots(crate,W,true,caches.cache,caches.closures),
  cargoSources,
  cargoToolchain:policy.toolchains.cargo.files,
 };
 return {host,scripts,internals:cacheInternals};
}
