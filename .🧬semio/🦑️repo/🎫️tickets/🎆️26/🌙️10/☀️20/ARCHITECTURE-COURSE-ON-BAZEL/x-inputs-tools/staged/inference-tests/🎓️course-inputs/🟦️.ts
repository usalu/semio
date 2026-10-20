/** 🎓️ Contract: nothing a cached course target (`test`, `typecheck`, `check`) reads lies outside its declared inputs plus the command closure the inference derives. The read sets come from third-party oracles (esbuild's metafile for module imports, the TypeScript compiler's own program for `tsc`, the repository's proctor manifest closure for Rust) and from a recorded file-system trace of the vitest run; an independent Python implementation recomputes the same law with its own glob translator and import scanner. The scenario lives in `./🔣️.json` as language-agnostic data.
 * @see ./🐍️.py */
import {expect,test} from "bun:test";
import {existsSync,readdirSync,readFileSync,writeFileSync,mkdtempSync,rmSync,realpathSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,normalize,relative,resolve,sep} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const workspace=resolve(import.meta.dirname,"../../../../../../../../../..");
const require=createRequire(join(workspace,"package.json"));
const manifest=JSON.parse(readFileSync(join(workspace,fixture.project,"📋️project.json"),"utf8"));
const skipped=new Set(["node_modules","target","dist",".git",".cache"]);
const posix=(path:string)=>path.split(sep).join("/");
const inventory=(()=>{const found=readdirSync(workspace,{withFileTypes:true}).filter(entry=>entry.isFile()).map(entry=>entry.name),walk=(directory:string)=>{for(const entry of readdirSync(join(workspace,directory),{withFileTypes:true})){const path=`${directory}/${entry.name}`;if(entry.isDirectory()){if(!skipped.has(entry.name))walk(path);}else if(entry.isFile())found.push(path);}};for(const top of fixture.inventory)walk(top);return found.sort();})();
const inInventory=new Set(inventory);
const sub=(text:string)=>text.replace("{site}",fixture.site).replace("{project}",fixture.project);
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();

const declaredInputs=(inputs:readonly unknown[]):Set<string>=>{
 const nx=JSON.parse(readFileSync(join(workspace,"nx.json"),"utf8")).namedInputs??{},named:Record<string,unknown[]>={default:["{projectRoot}/**/*"],...nx,...(manifest.namedInputs??{})},patterns:string[]=[];
 const walk=(items:readonly unknown[],depth=0)=>{if(depth>20)throw Error("named input cycle");for(const item of items){if(typeof item!=="string"||item.startsWith("^"))continue;if(Object.hasOwn(named,item)){walk(named[item]!,depth+1);continue;}patterns.push(item.replaceAll("{projectRoot}",fixture.project).replaceAll("{workspaceRoot}/","").replace(/^\/+/u,""));}};
 walk(inputs);
 const selected=new Set<string>();
 for(const pattern of patterns.filter(value=>!value.startsWith("!"))){if(/[*?{[]/u.test(pattern)){const glob=new Bun.Glob(pattern);for(const file of inventory)if(glob.match(file))selected.add(file);}else if(inInventory.has(pattern))selected.add(pattern);}
 for(const pattern of patterns.filter(value=>value.startsWith("!")).map(value=>value.slice(1))){const glob=new Bun.Glob(pattern);for(const file of [...selected])if(glob.match(file))selected.delete(file);}
 return selected;
};

const esbuildReads=async(entries:readonly string[]):Promise<Set<string>>=>{
 const present=entries.filter(entry=>inInventory.has(entry));
 const result=await require("esbuild").build({entryPoints:present.map(entry=>join(workspace,entry)),absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outdir:join(tmpdir(),"course-inputs-law"),loader:{".json":"json"},define:{"import.meta.vitest":"undefined"},plugins:[{name:"workspace-only",setup(build:any){build.onResolve({filter:/^[^./]/},(args:any)=>/^(?:@semio-tech|@teaching)\//u.test(args.path)?undefined:{path:args.path,external:true});}}]});
 return new Set(Object.keys(result.metafile.inputs).map(path=>posix(relative(workspace,resolve(workspace,path)))).filter(path=>!path.startsWith("..")&&!path.split("/").includes("node_modules")));
};
const vitestEntries=(config:string)=>{const text=readFileSync(join(workspace,config),"utf8"),block=/include:\s*\[(.*?)\]/su.exec(text)?.[1]??"";return[config,...[...block.matchAll(/"([^"]+)"/gu)].map(match=>posix(normalize(join(fixture.site,match[1]!))))];};
const nearestTsconfig=(path:string)=>{let directory=dirname(path);while(directory&&directory!=="."){const candidate=`${directory}/tsconfig.json`;if(inInventory.has(candidate))return candidate;directory=dirname(directory);}return inInventory.has("tsconfig.json")?"tsconfig.json":undefined;};
const workspaceManifests=()=>{const root=join(workspace,fixture.workspaces),base=dirname(root),members=JSON.parse(readFileSync(root,"utf8")).workspaces as string[];return[fixture.workspaces,posix(join(dirname(fixture.workspaces),"bun.lock")),...members.map(member=>posix(relative(workspace,join(base,member,"package.json")))).filter(path=>inInventory.has(path))];};
const tscReads=(config:string):Set<string>=>{
 const ts=require("typescript") as typeof import("typescript"),path=join(workspace,config),read=ts.readConfigFile(path,ts.sys.readFile),parsed=ts.parseJsonConfigFileContent(read.config,ts.sys,dirname(path),undefined,path);
 const program=ts.createProgram({rootNames:parsed.fileNames,options:{...parsed.options,noEmit:true,incremental:false,tsBuildInfoFile:undefined}});
 return new Set([config,...program.getSourceFiles().map(file=>posix(relative(workspace,realpathSync(file.fileName)))).filter(file=>!file.startsWith("..")&&!file.split("/").includes("node_modules")&&!file.startsWith(".cache/"))]);
};
const rustReads=async(spec:{package:string;pattern:string})=>{
 const {proctorSourceDirectories}=await import(join(workspace,"🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts")) as any,pattern=new RegExp(spec.pattern,"u"),found=new Set<string>();
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(entry.isDirectory()){if(!["target","node_modules","dist",".git"].includes(entry.name))walk(join(directory,entry.name));}else if(pattern.test(entry.name))found.add(posix(relative(workspace,join(directory,entry.name))));}};
 for(const directory of await proctorSourceDirectories(workspace) as string[])walk(directory);
 return found;
};

const readsOf=async(spec:{reads:readonly any[]}):Promise<Set<string>>=>{
 const reads=new Set<string>();
 for(const read of spec.reads){
  if(read.kind==="vitest"){for(const path of await esbuildReads(vitestEntries(sub(read.config))))reads.add(path);}
  else if(read.kind==="imports"){for(const path of await esbuildReads(read.entries.map(sub)))reads.add(path);}
  else if(read.kind==="workspace-manifests"){for(const path of workspaceManifests())reads.add(path);}
  else if(read.kind==="nearest-tsconfig"){for(const file of [...reads]){const path=/\.[cm]?[jt]sx?$/u.test(file)?nearestTsconfig(file):undefined;if(path)reads.add(path);}}
  else if(read.kind==="files"){for(const path of read.paths.map(sub)){if(inInventory.has(path))reads.add(path);}}
  else if(read.kind==="tsconfig"){for(const path of tscReads(sub(read.path)))reads.add(path);}
  else if(read.kind==="rust"){for(const path of await rustReads(read))reads.add(path);}
 }
 return new Set([...reads].filter(path=>inInventory.has(path)||existsSync(join(workspace,path))));
};

test("course input law fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

const python=Bun.which("python3");
test("python3 is available for the independent implementation",()=>{expect(python).toBeString();});

const results:Record<string,{closure:string[];declared:Set<string>;reads:Set<string>}>={};
for(const [name,spec] of Object.entries(fixture.targets)){
 test(`${name}: every file the target reads is declared or inferred`,async()=>{
  const target=manifest.targets[name],sources=await cacheInternals.targetScriptSources(target,workspace,createScriptInputCache(),fixture.project);
  const declared=declaredInputs(target.inputs??["default"]),inferred=new Set(strip(sources.closure??[])),reads=await readsOf(spec as any);
  results[name]={closure:sources.closure??[],declared,reads};
  const gap=[...reads].filter(path=>!path.includes("/🤖️generated/")&&!declared.has(path)&&!inferred.has(path)).sort();
  expect(gap).toEqual([]);
 },600_000);
}

test("the law is sensitive: removing a runtime-read input or the declared closure exposes the stale-green gap",async()=>{
 const target=manifest.targets.test,row=results.test!;
 const without=declaredInputs((target.inputs as unknown[]).filter(input=>input!=="proctorTestReads"&&input!=="workspaceManifests"));
 const sources=await cacheInternals.targetScriptSources({...target,metadata:undefined},workspace,createScriptInputCache(),fixture.project);
 const inferred=new Set(strip(sources.closure??[])),gap=[...row.reads].filter(path=>!without.has(path)&&!inferred.has(path));
 expect(gap).toContain("🎓️teaching/🛂️proctor/🧪️tests/🧫️fixtures/📚️catalog/🔣️.json");
 expect(gap).toContain("🎓️teaching/package.json");
 expect(gap.some(path=>path.includes("⚡️energy/"))).toBe(true);
},600_000);

test("an independent Python implementation agrees on the glob selection and finds the same empty gap",()=>{
 const temporary=mkdtempSync(join(realpathSync(tmpdir()),"course-inputs-"));
 try{
  const args=Object.entries(results).map(([name,row])=>{const file=join(temporary,`${name}.json`);writeFileSync(file,JSON.stringify({closure:row.closure}));return `${name}=${file}`;});
  const run=Bun.spawnSync([python!,"-I",resolve(import.meta.dirname,"🐍️.py"),resolve(import.meta.dirname,"🔣️.json"),workspace,join(workspace,fixture.project,"📋️project.json"),...args],{stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  const out=JSON.parse(run.stdout.toString()) as Record<string,{gap:string[];declared:string[];reads:string[]}>;
  for(const [name,row] of Object.entries(results)){
   const python=new Set(out[name]!.reads),shared=out[name]!.gap.filter(path=>row.reads.has(path)),onlyPython=[...python].filter(path=>!row.reads.has(path)),onlyOracle=[...row.reads].filter(path=>!python.has(path));
   console.log(JSON.stringify({name,oracle:row.reads.size,python:python.size,onlyPython:onlyPython.length,onlyOracle:onlyOracle.length}));
   expect(out[name]!.declared,`${name} declared selection`).toEqual([...row.declared].sort());
   expect(shared,`${name} files both implementations read but no input covers`).toEqual([]);
  }
 }finally{rmSync(temporary,{recursive:true,force:true});}
},600_000);
