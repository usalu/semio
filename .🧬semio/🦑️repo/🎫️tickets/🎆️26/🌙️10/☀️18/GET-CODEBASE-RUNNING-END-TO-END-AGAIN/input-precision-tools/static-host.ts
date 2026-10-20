// 🧰 A dependency-free host for the input precision engine: plain file reads, a regex relative-import closure and a Cargo path-dependency walk; used while the production analyses cannot boot.
import {existsSync as fsExists,readFileSync,readdirSync,statSync} from "node:fs";
import {join,dirname,resolve,relative} from "node:path";
import type {InputPrecisionHostV1} from "/private/tmp/ip/run/p/🟦️.ts";
export const W="/Users/ueli/Documents/semio",LIB=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const WS="{workspaceRoot}/",SPEC=/(?:from\s*|import\s*\(\s*|import\s*|require\s*\(\s*)["'](\.{1,2}\/[^"']+)["']/gu,EXT=["",".ts",".tsx",".mts",".js",".mjs",".json","/🟦️.ts","/🟦️.tsx"];
const rel=(path:string)=>relative(W,path).split("\\").join("/");
const tomlMemo=new Map<string,any>();
const toml=(path:string):any=>{let v=tomlMemo.get(path);if(v===undefined){try{v=Bun.TOML.parse(readFileSync(path,"utf8"));}catch{v={};}tomlMemo.set(path,v);}return v;};
export function staticHost():InputPrecisionHostV1{
 const policy=JSON.parse(readFileSync(join(LIB,"⚡️caching/🔣️policy.json"),"utf8")),taxonomy=JSON.parse(readFileSync(join(LIB,"🔣️taxonomy.json"),"utf8"));
 const text=(path:string)=>{try{return readFileSync(path,"utf8");}catch{return undefined;}};
 const fileMemo=new Map<string,readonly string[]>(),closure=(entries:readonly string[])=>{const key=entries.join("\0"),known=fileMemo.get(key);if(known)return known;const result=closureUncached(entries);fileMemo.set(key,result);return result;};
 const closureUncached=(entries:readonly string[])=>{
  const seen=new Set<string>(),pending=entries.map(entry=>resolve(W,entry));
  while(pending.length){const file=pending.pop()!;if(seen.has(file)||!fsExists(file))continue;seen.add(file);if(!/\.[cm]?[jt]sx?$/u.test(file))continue;const source=text(file)??"";
   for(const match of source.matchAll(SPEC)){const base=resolve(dirname(file),match[1]!);const hit=EXT.map(e=>base+e).find(c=>{try{return statSync(c).isFile();}catch{return false;}});if(hit)pending.push(hit);}}
  return [...seen].map(rel).sort();
 };
 const rootMemo=new Map<string,readonly string[]>();
 const workspaceOf=(directory:string)=>{let current=directory;for(;;){const manifest=join(current,"Cargo.toml");if(fsExists(manifest)&&toml(manifest).workspace)return current;if(current===W)return W;current=dirname(current);}};
 const roots=(crate:string):readonly string[]=>{
  const seen=new Set<string>(),pending=[resolve(W,crate)];let first=true;
  while(pending.length){const directory=pending.pop()!;if(seen.has(directory))continue;seen.add(directory);const manifest=toml(join(directory,"Cargo.toml")),workspace=toml(join(workspaceOf(directory),"Cargo.toml")).workspace??{};
   for(const table of [manifest,...Object.values<any>(manifest.target??{})])for(const kind of ["dependencies","build-dependencies",...(first?["dev-dependencies"]:[])])for(const [alias,declaration] of Object.entries<any>(table[kind]??{})){if(typeof declaration!=="object")continue;const inherited=declaration.workspace===true,dependency=inherited?workspace.dependencies?.[alias]:declaration;if(dependency?.path)pending.push(resolve(inherited?workspaceOf(directory):directory,dependency.path));}
   first=false;}
  return [...seen].map(rel).sort();
 };
 return {exists:p=>fsExists(join(W,p)),readText:p=>text(join(W,p)),listDirectory:p=>{try{return readdirSync(join(W,p),{withFileTypes:true}).map(e=>({name:e.name,directory:e.isDirectory()}));}catch{return [];}},kinds:taxonomy.fileKinds,importClosure:async entries=>closure(entries),cargoManifests:()=>{const out=Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z","--",":(glob)**/Cargo.toml"],{cwd:W}).stdout.toString().split("\0").filter(Boolean);return out.map(f=>f==="Cargo.toml"?"":f.slice(0,-"/Cargo.toml".length));},cargoRoots:crate=>{let r=rootMemo.get(crate);if(!r){r=roots(crate);rootMemo.set(crate,r);}return r;},cargoSources:()=>undefined,cargoToolchain:policy.toolchains.cargo.files};
}
