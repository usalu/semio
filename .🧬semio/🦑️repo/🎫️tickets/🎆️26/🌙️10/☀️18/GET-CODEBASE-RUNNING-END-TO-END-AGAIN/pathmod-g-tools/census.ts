// census: bun census.ts <out.json> : for EVERY tracked Cargo manifest (hidden trees excluded like crateIndexV1) the LIVE cargoSourceInputs verdict for both router flavors (named = tests included / dependency = compiled without cfg(test)) with the reason when it returns undefined, plus the package name. Streams one row per crate; RSS guard 900 MB.
import {existsSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
const W="/Users/ueli/Documents/semio",LIB=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(LIB,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {cacheInternals}=inference,rustCache=cacheInternals.createRustSourceCache();
const manifests=Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z","--",":(glob)**/Cargo.toml"],{cwd:W,maxBuffer:64<<20}).stdout.toString().split("\0").filter(Boolean).map(f=>f==="Cargo.toml"?"":f.slice(0,-"/Cargo.toml".length)).filter(d=>!d.split("/").some(p=>p.startsWith(".")));
const toml=(p:string)=>Bun.TOML.parse(readFileSync(p,"utf8")) as any;
const diagnose=(root:string,named:boolean)=>{
 const directory=join(W,root),manifest=join(directory,"Cargo.toml"),cargo=toml(manifest),build=cargo.package?.build;
 const entries=[cargo.lib?.path??"src/lib.rs","src/main.rs",...(cargo.bin??[]).map((t:any)=>t.path),...(named?[...(cargo.test??[]),...(cargo.bench??[]),...(cargo.example??[])].map((t:any)=>t.path):[]),...(build===false?[]:[typeof build==="string"?build:"build.rs"])].filter(Boolean).map((p:string)=>resolve(directory,p)).filter(existsSync);
 if(build!==false&&existsSync(resolve(directory,typeof build==="string"?build:"build.rs")))return {status:"fallback",reason:"build-script"};
 for(const source of ["src/bin",...(named?["tests","examples","benches"]:[])]){if(!existsSync(join(directory,source)))continue;for(const e of readdirSync(join(directory,source),{withFileTypes:true})){const p=join(directory,source,e.name,...(e.isDirectory()?["main.rs"]:[]));if(p.endsWith(".rs")&&existsSync(p))entries.push(p);}}
 if(!entries.length)return {status:"fallback",reason:"no-entries"};
 try{cacheInternals.rustSourceFiles(entries,directory,rustCache,new Map(),!named);}catch(e){return {status:"fallback",reason:"error:"+String((e as Error).message).replace(W+"/","").slice(0,200)};}
 return {status:"ok"};
};
const rows:any[]=[];
for(const root of manifests){
 let name:string|undefined;try{name=toml(join(W,root,"Cargo.toml")).package?.name;}catch{rows.push({root,error:"toml"});continue;}
 if(!name)continue;
 const row:any={root,name};
 for(const [flavor,named] of [["named",true],["dep",false]] as const){
  const d=diagnose(root,named),real=cacheInternals.cargoSourceInputs(root,W,new Map(),named,!named);
  if((d.status==="ok")!==(real!==undefined))row.mismatch=flavor;
  row[flavor]=d.status==="ok"?{status:"ok",files:real.map((f:string)=>f.replace("{workspaceRoot}/",""))}:d;
 }
 rows.push(row);
}
writeFileSync(process.argv[2]!,JSON.stringify(rows));
console.log(JSON.stringify({crates:rows.length,rssMB:Math.round(process.memoryUsage().rss/1048576)}));
if(process.memoryUsage().rss>900<<20)throw new Error("RSS above 900 MB");
