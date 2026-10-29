import {readFileSync,existsSync,readdirSync} from "node:fs";
import {dirname,resolve,join,relative} from "node:path";
import {execFileSync} from "node:child_process";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const POL={bytes:262144,files:32,lines:4000};
const cfg=[...execFileSync("rustc",["--print","cfg"],{encoding:"utf8"}).trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q))),"test","debug_assertions"];
const ws=process.argv[2];
const wt=Bun.TOML.parse(readFileSync(join(ws,"Cargo.toml"),"utf8")) as any;
const rows:any[]=[];
for(const m of wt.workspace?.members??[]){
 const dir=resolve(ws,m),mp=join(dir,"Cargo.toml");if(!existsSync(mp))continue;
 const t=Bun.TOML.parse(readFileSync(mp,"utf8")) as any;const name=t.package?.name;
 const b=t.package?.metadata?.semio?.["rust-source-boundary"];const lim=b?{bytes:Math.min(POL.bytes,b.maxSourceBytes),files:Math.min(POL.files,b.maxSourceFiles),lines:Math.min(POL.lines,b.maxActiveLines)}:POL;
 const entries:{kind:string;tname:string;path:string}[]=[];
 const lib=t.lib?.path?resolve(dir,t.lib.path):join(dir,"src/lib.rs");if(existsSync(lib))entries.push({kind:"lib-test",tname:"lib",path:lib});
 for(const x of t.test??[])entries.push({kind:"test",tname:x.name,path:resolve(dir,x.path)});
 const td=join(dir,"tests");if(existsSync(td)&&t.package?.autotests!==false)for(const f of readdirSync(td))if(f.endsWith(".rs"))entries.push({kind:"test",tname:f,path:join(td,f)});
 for(const e of entries){let inputs:any[]=[],err="";try{inputs=await readRustSourceClosureV1([e.path],REPO,cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});}catch(x:any){err=String(x.message??x).slice(0,160)}
  const bytes=inputs.reduce((s:number,x:any)=>s+x.bytes,0),files=inputs.length,lines=inputs.reduce((s:number,x:any)=>s+x.activeLines,0);
  const over=bytes>lim.bytes||files>lim.files||lines>lim.lines;
  rows.push({name,target:e.tname,kind:e.kind,bytes,files,lines,over,err});}
}
for(const r of rows)if(r.over||r.err)console.log(JSON.stringify(r));
console.log("TOTAL targets",rows.length,"over",rows.filter(r=>r.over).length,"err",rows.filter(r=>r.err).length);
