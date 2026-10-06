import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const repo="/Users/ueli/Documents/semio";
const root=join(repo,"✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation");
const pairs:{path:string;before:string;after:string}[]=[];
function explicitMembers(text:string){
 const name="semio_framework_pack_json::from_json_str";
 let cursor=0;
 while((cursor=text.indexOf(name,cursor))>=0){
  const open=text.indexOf("(",cursor+name.length);assert(open>=0);
  let depth=1,quoted=false,escaped=false,commas=0,close=open+1;
  for(;close<text.length;close++){const char=text[close];if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"'){quoted=true;continue;}if(char==="(")depth++;else if(char===")"){depth--;if(depth===0)break;}else if(char===","&&depth===1)commas++;}
  assert.equal(depth,0);
  if(commas===0){const policy=", semio_framework_pack_json::JsonMemberPolicy::Reject";text=text.slice(0,close)+policy+text.slice(close);close+=policy.length;}
  cursor=close+1;
 }
 return text;
}
for(const relative of new Bun.Glob("**/*.rs").scanSync({cwd:root,onlyFiles:true})){const path=join(root,relative),before=readFileSync(path,"utf8");let after=before.replaceAll("semio_framework_os_kernel::json::","semio_framework_pack_json::").replaceAll("dsl::json::","semio_framework_pack_json::").replaceAll("store::json::","semio_framework_pack_json::").replaceAll("dsl::DslValue","semio_framework_value::DslValue").replaceAll("store::DslValue","semio_framework_value::DslValue");after=explicitMembers(after);if(after!==before)pairs.push({path,before,after});}
function edit(relative:string,old:string,next:string){const path=join(root,relative);let pair=pairs.find(pair=>pair.path===path);if(!pair){const before=readFileSync(path,"utf8");pair={path,before,after:before};pairs.push(pair);}assert.equal(pair.after.split(old).length,2,"Missing unique authored join "+relative+" "+old);pair.after=pair.after.replace(old,next);}
const base="🏅️standards/🔖️1/🪆️subsets/✳️any/";
edit(base+"✏️editor/🦀️.rs","let bytes = sourcing_curation_config_mutation_retained_bytes(mutation)?;","let bytes = sourcing_curation_config_mutation_retained_bytes(mutation).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, message))?;");
edit(base+"✏️editor/🦀️.rs","let bytes = sourcing_curation_mutation_retained_bytes(mutation)?;","let bytes = sourcing_curation_mutation_retained_bytes(mutation).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, message))?;");
edit(base+"✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs","sourcing_curation_config_mutation_footprint(&mutation)","sourcing_curation_config_mutation_retained_bytes(&mutation)");
edit(base+"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs",".map_err(|e|e.to_string()).and_then(|d|CurationSnapshot::from_sqlite_database", ".and_then(|d|CurationSnapshot::from_sqlite_database");
edit("🧪️tests/🔬️unit/🦀️.rs","CurationIntoTxt::serialize(&snapshot)","CurationIntoTxt::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty())");
edit(base+"✏️editor/🧪️tests/🔬️unit/🦀️.rs",'MediaPayload::Binary { .. } => panic!("expected a Structured payload"),','MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => panic!("expected a Structured payload"),');
const output=join(import.meta.dir,"guarded-pairs.json");
writeFileSync(output,JSON.stringify(pairs,null,2)+"\n");
if(process.argv[2]==="mount"){for(const pair of pairs)assert.equal(readFileSync(pair.path,"utf8"),pair.before,"Concurrent Curation change: regenerate "+pair.path);for(const pair of pairs)writeFileSync(pair.path,pair.after);console.log("[DEBUG] Curation actual canonical JSON/Value and typed caller joins mounted paths="+pairs.length+" original_assertions_preserved=true");}else console.log("[DEBUG] Curation local authority joins staged paths="+pairs.length+" production_mutations=0");
