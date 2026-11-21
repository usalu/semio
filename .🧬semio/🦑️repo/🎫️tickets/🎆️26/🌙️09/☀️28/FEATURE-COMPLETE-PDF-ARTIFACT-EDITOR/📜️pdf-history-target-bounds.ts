/** 📐 Check every actual production and test owner of the staged PDF history routing. */
import fs from "node:fs";
import assert from "node:assert/strict";
import {join,dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
const d=join(import.meta.dir,"🗑️generated");
const overlaySource=fs.readFileSync(join(d,"pdf-history-target-overlay.json"));
const staged=JSON.parse(overlaySource.toString());
const overlay=new Map(staged.map(([p,,s]:[string,string|null,string])=>[resolve(p),s]));
const graph=JSON.parse(fs.readFileSync(join(d,"wD","unit-graph-prototype-00-retry.json"),"utf8"));
const units=new Map();
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
for(const u of graph.units){
 if(!u.pkg_id.startsWith("path+file:")||!u.target.kind.includes("lib")||!u.target.name.startsWith("semio_s_artifact_stdio_pdf"))continue;
 const dir=fileURLToPath(u.pkg_id.slice(5).split("#")[0]);const p=join(dir,"Cargo.toml");
 if(fs.existsSync(p))units.set(u.target.name,[u.target.src_path,dir,u.features]);
}
const [[vtManifest,,vtContents]]=staged.filter(([p]:[string])=>p.endsWith("/🧾️vt/🧬️schema/📦️packages/🦀️rust/Cargo.toml"));
const conf=Bun.TOML.parse(vtContents);
const vtDir=dirname(resolve(vtManifest));
const vtTest=conf.test.find((row:{name:string})=>row.name==="vt_conformance");
units.set("vt_conformance",[resolve(vtDir,vtTest.path),vtDir,[]]);
const found=new Set();
const out=[];
for(const [name,[entry,manifest,features]]of units){
 for(const os of ["linux","macos","windows","wasi"]){
  const cfg=[`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?"wasm":os==="windows"?"windows":"unix",...features.map((value:string)=>`feature="${value}"`)];
  for(const test of name==="vt_conformance"?[true]:[false,true]){
   const {files,lines,bytes,sourcePaths}=await measureRustCrate(process.cwd(),entry,test,overlay,manifest,cfg);
   const affected=sourcePaths.filter(f=>overlay.has(resolve(f)));if(!affected.length)continue;
   for(const f of affected)found.add(resolve(f));
   const admitted=files<=32&&lines<=4000&&bytes<=262144;
   out.push({name,os,test,files,lines,bytes,admitted});
  }
 }
}
for(const p of overlay.keys())if(p.endsWith(".rs"))assert.ok(found.has(p),p);
fs.writeFileSync(join(d,"pdf-history-target-bounds.json"),JSON.stringify({stage:new Bun.CryptoHasher("sha256").update(overlaySource).digest("hex"),checks:out},null,2)+"\n");
const uncovered=out.filter(x=>!x.admitted);
console.log("[DEBUG]",out.length,"checks",uncovered);
if(uncovered.length)process.exitCode=1;
