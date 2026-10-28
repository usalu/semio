// refusal-census: bun refusal-census.ts <absolute library root> <out.json> : for every tracked Cargo package (hidden trees excluded) the verdict of the STAGED `cargoSourceSetV1` for both flavors: resolved / emitters / unresolved with reason. Crates are read from the live checkout, library code from the given root (a mirror with the batch applied). Streams, RSS guard 900 MB.
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",lib=process.argv[2]!;
const inference:any=await import(join(lib,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {cacheInternals}=inference;
const manifests=Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z","--",":(glob)**/Cargo.toml"],{cwd:W,maxBuffer:64<<20}).stdout.toString().split("\0").filter(Boolean).map(f=>f==="Cargo.toml"?"":f.slice(0,-"/Cargo.toml".length)).filter(d=>!d.split("/").some(p=>p.startsWith(".")));
const rows:any[]=[];
for(const root of manifests){
 let name:string|undefined;try{name=(Bun.TOML.parse(readFileSync(join(W,root,"Cargo.toml"),"utf8")) as any).package?.name;}catch{continue;}
 if(!name)continue;
 const row:any={root,name};
 for(const [flavor,named] of [["named",true],["dep",false]] as const){
  const set=cacheInternals.cargoSourceSetV1(root,W,new Map(),named,!named);
  row[flavor]=set.resolved?{status:"ok",files:set.files.length,missing:set.files.filter((f:string)=>!existsSync(join(W,f.slice("{workspaceRoot}/".length)))).length,...(set.emitters?{emitters:set.emitters.length}:{})}:{status:"fallback",reason:set.reason,message:(set.message??"").slice(0,200)};
 }
 rows.push(row);
}
writeFileSync(process.argv[3]!,JSON.stringify(rows));
console.log(JSON.stringify({crates:rows.length,rssMB:Math.round(process.memoryUsage().rss/1048576)}));
if(process.memoryUsage().rss>900<<20)throw new Error("RSS above 900 MB");
