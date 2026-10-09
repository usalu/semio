import { readFileSync, writeFileSync, mkdirSync, existsSync, renameSync, readdirSync, rmdirSync } from "node:fs";
import { resolve, dirname, join, relative } from "node:path";
import { execFileSync } from "node:child_process";
const repo=process.cwd(), ticket=resolve(import.meta.dir,"..");
const parser=readFileSync(join(ticket,"runtime/📜️script.ts"),"utf8");
const helper=parser.slice(parser.indexOf("function mask("),parser.indexOf("const sourceModules"));
const helpers=new Function(new Bun.Transpiler({loader:"ts"}).transformSync(helper)+";return {mask,items};")();
const files=execFileSync("rg",["--files","🧰️framework"],{encoding:"utf8",maxBuffer:50*1024*1024}).trim().split("\n");
const roots=[...new Set(files.flatMap(file=>file.match(/^(.+\/🗿️artifacts\/[^/]+)\//)?.[1]??[]))];
const changed=new Set<string>();
function write(file:string,source:string){if(!existsSync(file)||readFileSync(file,"utf8")!==source){mkdirSync(dirname(file),{recursive:true});writeFileSync(file,source);changed.add(relative(repo,file));}}
const maps=roots.flatMap(root=>[
["🧬️schema/📸️snapshot/🪶️sqlite","🚪️io/🪶️sqlite/📸️snapshot"],
["🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite","🚪️io/🪶️sqlite/📸️snapshot/🧪️tests"],
["🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite","🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures"],
["🧪️tests/🪶️sqlite","🚪️io/🪶️sqlite/📸️snapshot/🧪️tests"],
["🌿️vcs/📸️snapshot/🪶️native","🚪️io/🪶️sqlite/📸️snapshot/🚦️native"]
].map(([from,to])=>({from:resolve(root,from),to:resolve(root,to)})).filter(map=>existsSync(map.from)));
function mapped(file:string){const map=maps.find(map=>file===map.from||file.startsWith(map.from+"/"));return map?map.to+file.slice(map.from.length):file;}
if(process.argv[2]==="relocate"){
 const sources=files.filter(f=>/\.(?:rs|ts|tsx|json|jsonc|md|feature|sql)$/.test(f));
 const originals=sources.map(f=>({file:resolve(f),source:readFileSync(f,"utf8")}));
 const moved:string[]=[];
 function move(from:string,to:string){mkdirSync(to,{recursive:true});for(const entry of readdirSync(from,{withFileTypes:true})){const old=join(from,entry.name),next=join(to,entry.name);if(entry.isDirectory())move(old,next);else{if(existsSync(next))throw Error("collision "+next);renameSync(old,next);moved.push(relative(repo,old),relative(repo,next));}}rmdirSync(from);}
 for(const map of maps)move(map.from,map.to);
 for(const row of originals){const next=mapped(row.file);let source=row.source.replace(/(["'])([^"'\n]+)\1/g,(full,q,value)=>{
  if(!value.includes("/"))return full;
  const old=resolve(dirname(row.file),value),target=mapped(old);
  if(target!==old||next!==row.file&&value.startsWith(".")){let valueNext=relative(dirname(next),target);if(!valueNext.startsWith("."))valueNext="./"+valueNext;return q+valueNext+q;}
  let newValue=value;for(const map of maps){const from=relative(repo,map.from),to=relative(repo,map.to);newValue=newValue.replaceAll(from,to);}
  return q+newValue+q;
 });write(next,source);}
 write(join(ticket,"framework-relocation-files.md"),"# Framework I/O Relocation Files\n\n"+[...new Set(moved)].map(file=>"- "+file).join("\n")+"\n");
 console.log(JSON.stringify({representations:maps.length,moved:moved.length/2,rewritten:changed.size}));
}
if(process.argv[2]==="extract"){
 for(const root of roots){
  const owner=existsSync(join(root,"🌿️vcs/🦀️.rs"))?join(root,"🌿️vcs/🦀️.rs"):join(root,"🦀️.rs");
  if(!existsSync(owner))continue;
  const semanticFiles=files.filter(f=>f.startsWith(root+"/")&&f.endsWith(".rs")&&!/\/(?:🚪️io|🧪️tests|🧫️fixtures)\//.test(f));
  let count=0;
  for(const file of semanticFiles){if(!existsSync(file))continue;const source=readFileSync(file,"utf8"),all=helpers.items(source);
   const codecs=all.filter((item:any)=>/\bimpl\s+(?:[\w:]+::)?(?:OpText|OpBinary|ArtifactDsl|ArtifactPack)\b/.test(item.head));
   if(!codecs.length)continue;
   const groups=new Map<string,any[]>();
   for(const item of codecs){const representation=/OpText|ArtifactDsl/.test(item.head)?"📝️text":"💾️binary",facet=/OpText|OpBinary/.test(item.head)?"🧬️mutations":"📸️snapshot",key=representation+"/"+facet;groups.set(key,[...(groups.get(key)??[]),item]);}
   const originalModule=file===owner?"super::super::super":file.includes("/🧬️schema/📸️snapshot/")?root.includes("🏃️run")||root.includes("snapshot-refusal")?"crate::snapshot":"crate::artifact":file.includes("/🧬️schema/🧬️mutations/")?root.includes("🌊️flow")?"crate::vcs::mutations":"crate::mutations":"crate";
   for(const [key,blocks]of groups){const target=join(root,"🚪️io",key,"🦀️.rs");const imports=all.filter((item:any)=>item.kind==="use").map((item:any)=>item.code.replace(/use super::/g,"use "+originalModule+"::")).join("\n");
    const prefix="//! 🚪️ Native artifact "+key.split("/").join(" ")+" codecs.\nuse "+(owner.includes("🌿️vcs")?"crate::vcs":"crate")+"::*;\n"+imports+"\n";
    write(target,(existsSync(target)?readFileSync(target,"utf8"):prefix)+"\n"+blocks.map((b:any)=>b.code).join("\n\n")+"\n");
   }
   let semantic=source;for(const item of codecs.sort((a:any,b:any)=>b.start-a.start))semantic=semantic.slice(0,item.start)+semantic.slice(item.end);
   write(file,semantic);count+=codecs.length;
  }
  if(count){
   for(const rep of["📝️text","💾️binary"]){const branches=["📸️snapshot","🧬️mutations"].filter(facet=>existsSync(join(root,"🚪️io",rep,facet,"🦀️.rs")));if(branches.length)write(join(root,"🚪️io",rep,"🦀️.rs"),"//! 🚪️ Native representation assembly.\n"+branches.map(facet=>'#[path = "'+facet+'/🦀️.rs"]\npub mod '+(facet==="📸️snapshot"?"snapshot":"mutations")+";").join("\n")+"\n");}
   const reps=[["📝️text","text"],["💾️binary","binary"],["🪶️sqlite","sqlite"]].filter(([rep])=>existsSync(join(root,"🚪️io",rep,"🦀️.rs")));
   write(join(root,"🚪️io/🦀️.rs"),"//! 🚪️ Artifact representation assembly.\n"+reps.map(([rep,name])=>'#[path = "'+rep+'/🦀️.rs"]\npub mod '+name+";").join("\n")+"\n");
   const src=readFileSync(owner,"utf8"),mount='#[path = "'+(owner.includes("🌿️vcs")?"../":"")+'🚪️io/🦀️.rs"]\npub mod io;\n';
   if(!/pub mod io;/.test(src))write(owner,src+"\n"+mount);
  }
 }
 console.log(JSON.stringify({edited:changed.size}));
}
write(join(ticket,"framework-codec-files.md"),"# Framework Codec Files\n\n"+[...changed].filter(f=>!f.includes("🎫️tickets")).map(f=>"- "+f).join("\n")+"\n");

if(process.argv[2]==="mounts"){
 for(const root of roots){
  const snapshot=join(root,"🧬️schema/📸️snapshot/🦀️.rs"),artifact=join(root,"🦀️.rs");
  if(existsSync(snapshot)){const src=readFileSync(snapshot,"utf8");write(snapshot,src.replace(/#\[path\s*=\s*"[^"]*sqlite\/🦀️\.rs"\]\s*mod sqlite;\s*/g,""));}
  if(!existsSync(join(root,"🚪️io/🪶️sqlite/📸️snapshot")))continue;
  write(join(root,"🚪️io/🪶️sqlite/🦀️.rs"),'//! 🪶️ SQLite representation assembly.\n#[path = "📸️snapshot/🦀️.rs"]\npub mod snapshot;\n');
  const assembly=join(root,"🚪️io/🦀️.rs");
  const src=existsSync(assembly)?readFileSync(assembly,"utf8"):"//! 🚪️ Artifact representation assembly.\n";
  if(!/mod sqlite;/.test(src))write(assembly,src+'#[path = "🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n');
  const sqlite=join(root,"🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs");
  if(existsSync(sqlite)){
   let src=readFileSync(sqlite,"utf8").replace(/use super::/g,"use crate::").replace(/super::super::KIND/g,"crate::KIND");
   if(root.includes("🏃️run"))src=src.replace(/super::run_(status|node_status)_ordinal/g,"crate::snapshot::run_$1_ordinal");
   write(sqlite,src);
  }
  if(existsSync(artifact)){
   let src=readFileSync(artifact,"utf8").replace(/#\[path\s*=\s*"🚪️io\/🪶️sqlite\/📸️snapshot\/🦀️\.rs"\]\s*mod (?:snapshot_sqlite|sqlite_snapshot);\s*/g,"");
   src=src.replace(/pub use snapshot_sqlite::\{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT\};/g,"pub use io::sqlite::snapshot::{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT};");
   if(!root.includes("🌊️flow")&&!root.includes("🕸️dag")&&!/pub mod io;/.test(src))src+='\n#[path = "🚪️io/🦀️.rs"]\npub mod io;\n';
   write(artifact,src);
  }
 }
 const run=roots.find(r=>r.includes("🏃️run"));
 if(run){const snapshot=join(run,"🧬️schema/📸️snapshot/🦀️.rs");write(snapshot,readFileSync(snapshot,"utf8").replace(/^fn (run_(?:status|node_status)_ordinal)/gm,"pub(crate) fn $1"));}
 const flow=roots.find(r=>r.includes("🌊️flow/🗿️artifacts/🌊️flow"));
 if(flow){
  const owner=join(flow,"🌿️vcs/🦀️.rs");let src=readFileSync(owner,"utf8");
  const all=helpers.items(src),first=all.find((i:any)=>i.name==="ValueDsl"),last=all.find((i:any)=>/impl semio_framework_dsl_record::DslField for FlowHostSnapshot\b/.test(i.head));
  if(first&&last){
   let lowered=src.slice(first.start,last.end).replace(/^(fn|struct|enum)\b/gm,"pub(crate) $1");
   lowered=lowered.replace(/flow_native_carrier::/g,"crate::io::sqlite::snapshot::flow_native_carrier::");
   const text=join(flow,"🚪️io/📝️text/📸️snapshot/🦀️.rs");write(text,readFileSync(text,"utf8")+"\n"+lowered+"\n");
   src=src.slice(0,first.start)+src.slice(last.end);
   const binary=join(flow,"🚪️io/💾️binary/📸️snapshot/🦀️.rs");write(binary,"use crate::io::text::snapshot::*;\n"+readFileSync(binary,"utf8"));
  }
  const mounts=[...src.matchAll(/#\[path = "\.\.\/🚪️io\/🪶️sqlite\/📸️snapshot\/([^"]+)"\]\s*mod (\w+);/g)];
  write(join(flow,"🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"),"//! 🌊️ Flow controlled native and relational I/O.\nuse crate::vcs::*;\nuse crate::io::text::snapshot::*;\n"+mounts.map(m=>'#[path = "'+m[1]+'"]\npub(crate) mod '+m[2]+";").join("\n")+"\n");
  for(const m of mounts)src=src.replace(m[0],"");
  write(owner,src);
  const carrier=join(flow,"🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛂️carrier/🦀️.rs");
  write(carrier,readFileSync(carrier,"utf8").replace(/pub\(super\) fn (encode_field|decode_field)/g,"pub(crate) fn $1"));
 }
 for(const file of files.filter(f=>f.includes("/🚪️io/")&&f.endsWith(".rs")&&existsSync(f)))write(file,readFileSync(file,"utf8").replace(/^pub use snapshot_sqlite::.*\n/gm,""));
 console.log(JSON.stringify({edited:changed.size}));
}

if(process.argv[2]==="clean-imports"){
 for(const root of roots)for(const rep of["📝️text","💾️binary"])for(const facet of["📸️snapshot","🧬️mutations"]){
  const file=join(root,"🚪️io",rep,facet,"🦀️.rs");if(!existsSync(file))continue;
  const source=readFileSync(file,"utf8"),first=helpers.items(source).find((item:any)=>item.kind==="impl");if(!first)continue;
  let prefix="//! 🚪️ Native artifact representation codecs.\nuse super::super::super::*;\n";
  if(root.includes("snapshot-refusal"))prefix+="use semio_framework_os_kernel::{os_spr as protocol,os_store as store};\n";
  if(root.includes("🌊️flow")&&rep==="💾️binary"&&facet==="📸️snapshot")prefix+="use crate::io::text::snapshot::*;\n";
  write(file,prefix+"\n"+source.slice(first.start));
 }
 const refusal=roots.find(root=>root.includes("snapshot-refusal"));
 if(refusal){const file=join(refusal,"🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs");write(file,readFileSync(file,"utf8").replace(/use crate::Snapshot;/g,"use super::super::super::Snapshot;").replace(/crate::KIND/g,"super::super::super::KIND"));}
 console.log(JSON.stringify({edited:changed.size}));
}
const prior=existsSync(join(ticket,"framework-codec-files.md"))?readFileSync(join(ticket,"framework-codec-files.md"),"utf8").split("\n").filter(row=>row.startsWith("- ")).map(row=>row.slice(2)):[];
write(join(ticket,"framework-codec-files.md"),"# Framework Codec Files\n\n"+[...new Set([...prior,...changed])].filter(f=>!f.includes("🎫️tickets")).map(f=>"- "+f).join("\n")+"\n");
