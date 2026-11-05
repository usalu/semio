import fs from "node:fs/promises";
import path from "node:path";
const {readRustSourceClosureV1}=await import(path.join(process.cwd(),"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS";
const owners=["🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema","🧰️framework/🔨️modules/🎠️kernel","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/staged-form","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/🎟️source","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/📝️forms","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔗️wit-bindings","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧠️kernel-runtime","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🫒️tests-05-3"];
const cfg=["debug_assertions","target_arch=x86_64","target_os=linux","target_family=unix","unix","target_env=gnu","target_pointer_width=64","target_endian=little","target_has_atomic=8","target_has_atomic=16","target_has_atomic=32","target_has_atomic=64","target_has_atomic=ptr","feature=artifact-app-testing"];
const rows=[];
for(const dir of owners){
 const manifest=path.join(dir,"📦️packages/🦀️rust/Cargo.toml");
 try{
  const doc=Bun.TOML.parse(await fs.readFile(manifest,"utf8")) as any;
  const entry=path.resolve(path.dirname(manifest),doc.lib?.path??"src/lib.rs");
  for(const kind of ["lib","test"]){
   try{
    const inputs=await readRustSourceClosureV1([entry],process.cwd(),kind==="test"?[...cfg,"test"]:cfg,async file=>{try{return new Uint8Array(await fs.readFile(file));}catch(error){if((error as any).code==="ENOENT")return null;throw error;}},{manifestDirectory:path.resolve(path.dirname(manifest)),generated:()=>{}});
    const bytes=inputs.reduce((sum,input)=>sum+input.bytes,0),lines=inputs.reduce((sum,input)=>sum+input.activeLines,0),limits=doc.package?.metadata?.semio?.["rust-source-boundary"];
    rows.push({owner:doc.package.name,kind,files:inputs.length,bytes,activeLines:lines,limits,within:!!limits&&inputs.length<=limits.maxSourceFiles&&bytes<=limits.maxSourceBytes&&lines<=limits.maxActiveLines});
   }catch(error){rows.push({owner:doc.package.name,kind,error:String(error)})}
  }
 }catch(error){rows.push({owner:dir,error:String(error)})}
}
await fs.writeFile(path.join(ticket,"🗑️generated/native-current/source-census.json"),JSON.stringify({cfg,note:"Conservative Linux cfg census using the existing source collector; actual registered Cargo admission remains required.",rows},null,2)+"\n");
const lines=["# Changed Native Source Owner Census","","This read-only census uses the existing readRustSourceClosureV1 collector with an explicit conservative Linux test/library cfg snapshot, including artifact-app-testing. It does not replace actual Cargo compiler cfg admission or compilation. No limits were changed.","","| Owner | Mode | Files | Bytes | Active Lines | Within Declared Bounds |","|---|---|---:|---:|---:|---|",...rows.map(row=>row.error?"| "+row.owner+" | "+(row.kind??"")+" | — | — | — | "+row.error.replaceAll("|","/")+" |":"| "+row.owner+" | "+row.kind+" | "+row.files+" | "+row.bytes+" | "+row.activeLines+" | "+row.within+" |")];
await fs.mkdir(path.join(ticket,"🔍️audits"),{recursive:true});await fs.writeFile(path.join(ticket,"🔍️audits/native-source-census.md"),lines.join("\n")+"\n");
process.stdout.write(JSON.stringify(rows.filter(row=>row.error||!row.within))+"\n");
