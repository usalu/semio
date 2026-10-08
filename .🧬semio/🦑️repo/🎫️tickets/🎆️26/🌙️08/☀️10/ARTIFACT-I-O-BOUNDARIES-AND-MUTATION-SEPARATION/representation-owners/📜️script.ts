import Parser from "web-tree-sitter";
import {execFileSync} from "node:child_process";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
await Parser.init();
const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
const ticket=resolve(import.meta.dir,"..");
const files=execFileSync("rg",["--files","✏️s","-g","*.rs"],{encoding:"utf8",maxBuffer:32*1024*1024}).trim().split("\n");
if(process.argv[2]==="root-syntax"){
 const roots=["✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/","✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/","✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/"];
 const selected=files.filter(file=>roots.some(root=>file.startsWith(root)));const errors:{file:string;details:string[]}[]=[];
 for(const file of selected){const tree=parser.parse(readFileSync(file,"utf8"))!;if(tree.rootNode.hasError()){const details:string[]=[];const visit=(node:any)=>{if(node.type==="ERROR"||node.isMissing())details.push(`${node.type}:${node.startPosition.row+1}:${node.startPosition.column+1}: ${node.text.slice(0,180)}`);for(const child of node.namedChildren)visit(child);};visit(tree.rootNode);errors.push({file,details});}tree.delete();}
 writeFileSync(join(ticket,"root-native-source-syntax.md"),"# Current Root Native Source Syntax\n\nIndependent tree-sitter parsed "+selected.length+" Rust sources under Procedural3d, SourcingCuration and ArchitectProgram. This is syntax evidence only; native compilation and execution remain required.\n\n"+(errors.length?errors.map(row=>"## "+row.file+"\n\n```text\n"+row.details.join("\n")+"\n```\n").join("\n"):"No syntax errors were reported.\n"));
 console.log(`[DEBUG] Independent root native source syntax: checked=${selected.length}; errors=${errors.length}`);if(errors.length)console.log(JSON.stringify(errors));process.exit(errors.length?1:0);
}
const representation=new Map([["ArtifactPack","💾️binary"],["OpBinary","💾️binary"],["ArtifactDsl","📝️text"],["OpText","📝️text"],["ArtifactSqliteSnapshot","🪶️sqlite"]]);
const rows:any[]=[];
for(const file of files){
 if(!file.includes("/🚪️io/")||file.includes("/🧪️tests/"))continue;
 const source=readFileSync(file,"utf8"),tree=parser.parse(source)!;
 const walk=(node:any)=>{
  if(node.type==="impl_item"){
   const trait=node.childForFieldName("trait")?.text?.split("::").at(-1),owner=representation.get(trait);
   if(owner&&!file.includes("/🚪️io/"+owner+"/")){
    const destination=file.replace(/\/🚪️io\/[^/]+\//,"/🚪️io/"+owner+"/"),type=node.childForFieldName("type")?.text;
    const scopes:any[]=[];let current=node.parent;while(current){if(current.type==="source_file"||current.type==="declaration_list")scopes.unshift(current);current=current.parent;}
    const uses=scopes.flatMap(s=>s.namedChildren.filter((v:any)=>v.type==="use_declaration").map((v:any)=>v.text));
    const defs=scopes.flatMap(s=>s.namedChildren.filter((v:any)=>["function_item","struct_item","enum_item","const_item","type_item","static_item"].includes(v.type)).map((v:any)=>({name:v.childForFieldName("name")?.text,kind:v.type,start:v.startIndex,text:v.text}))).filter(v=>v.name&&new RegExp("\\b"+v.name+"\\b").test(node.text));
    const target=existsSync(destination)?readFileSync(destination,"utf8"):"";
    rows.push({file,destination,trait,type,start:node.startIndex,end:node.endIndex,text:node.text,uses,defs:defs.map(v=>({name:v.name,kind:v.kind,visible:/^pub(?:\([^)]*\))? /.test(v.text)})),duplicate:target.includes("impl "+trait+" for "+type)||target.includes("impl protocol::"+trait+" for "+type)||target.includes("impl store::"+trait+" for "+type)});
   }
  }
  for(const child of node.namedChildren)walk(child);
 };walk(tree.rootNode);
}
writeFileSync(join(ticket,"representation-owner-inventory.md"),"# Representation Owner Inventory\n\n"+rows.map(r=>"## "+r.trait+" for "+r.type+"\n\n- Source: "+r.file+"\n- Destination: "+r.destination+"\n- Existing implementation: "+r.duplicate+"\n- Local dependencies: "+r.defs.map((d:any)=>d.name+" ("+d.kind+", "+(d.visible?"visible":"private")+")").join(", ")+"\n\n```rust\n"+r.uses.join("\n")+"\n"+r.text+"\n```").join("\n\n")+"\n");
writeFileSync(join(ticket,"🗑️generated/representation-owner-inventory.json"),JSON.stringify(rows,null,2));
console.log("[DEBUG] Misplaced physical implementations inventoried: "+rows.length+", duplicate targets: "+rows.filter(r=>r.duplicate).length);
if(process.argv[2]==="syntax"){
 const syntax:any[]=[];for(const file of files.filter(file=>file.includes("/🧩️puzzle/")&&file.includes("/🧬️schema/🔺️diff/"))){const typescript=file.replace(/🦀️\.rs$/,"🟦️.ts");if(!existsSync(typescript))continue;try{new Bun.Transpiler({loader:"ts"}).scan(readFileSync(typescript,"utf8"));}catch(error){syntax.push({file:typescript,error:String(error),details:error instanceof AggregateError?[...error.errors].map(error=>({message:error.message,position:error.position})):[]});}}
 writeFileSync(join(ticket,"puzzle-typescript-syntax-diagnostics.md"),"# Puzzle TypeScript Syntax Diagnostics\n\n"+syntax.map(row=>"## "+row.file+"\n\n```text\n"+row.error+"\n```").join("\n\n")+"\n");console.log(JSON.stringify(syntax));
}
if(process.argv[2]==="step"){
 const {renameSync,readdirSync,rmdirSync,unlinkSync}=await import("node:fs"),changed:string[]=[];
 const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets";
 for(let number=1;number<=6;number++){
  const directory=join(root,["","1️⃣cc1","2️⃣cc2","3️⃣cc3","4️⃣cc4","5️⃣cc5","6️⃣cc6"][number]!,"🚪️io"),old=join(directory,"🧬️mutations");
  const transfer=(source:string,target:string)=>{if(existsSync(target))throw Error("STEP canonical owner already exists: "+target);mkdirSync(dirname(target),{recursive:true});renameSync(source,target);changed.push(source,target);};
  for(const rep of["📝️text","💾️binary"]){transfer(join(old,rep,"🦀️.rs"),join(directory,rep,"🧬️mutations/🦀️.rs"));if(rep==="💾️binary")transfer(join(old,rep,"📡️.protocol.semio"),join(directory,rep,"🧬️mutations/📡️.protocol.semio"));const mount=join(directory,rep,"🦀️.rs");if(existsSync(mount))throw Error("STEP representation mount requires integration: "+mount);writeFileSync(mount,'//! 🚪️ Native '+rep+' artifact IO.\n#[path = "🧬️mutations/🦀️.rs"]\npub mod mutations;\n');changed.push(mount);}
  const tests=join(directory,"💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs");transfer(join(old,"🧪️tests/🔬️unit/🦀️.rs"),tests);let body=readFileSync(tests,"utf8").replace("use super::StepCc"+number+"Mutation;","use crate::standards::v_ap214::subsets::cc"+number+"::schema::mutations::StepCc"+number+"Mutation;");writeFileSync(tests,body);
  const binary=join(directory,"💾️binary/🧬️mutations/🦀️.rs");writeFileSync(binary,readFileSync(binary,"utf8")+'\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n');
  const component=join(directory,"🦀️.rs");body=readFileSync(component,"utf8").replace('#[path = "🧬️mutations/🦀️.rs"]\npub mod mutations;','#[path = "📝️text/🦀️.rs"]\npub mod text;\n#[path = "💾️binary/🦀️.rs"]\npub mod binary;');writeFileSync(component,body);changed.push(component);unlinkSync(join(old,"🦀️.rs"));changed.push(join(old,"🦀️.rs"));
  const empty=(path:string)=>{for(const child of readdirSync(path,{withFileTypes:true})){if(!child.isDirectory())throw Error("Unmoved STEP input: "+join(path,child.name));empty(join(path,child.name));}rmdirSync(path);};empty(old);
 }
 writeFileSync(join(ticket,"step-native-mutation-owner-closure.md"),"# STEP Native Mutation Ownership\n\nThe observed current-repository guard RED found all six AP214 class mutation IO facets ordered semantic-first. Their text and binary implementations and protocol/test assets now use canonical representation-first owners. Actual declared domain intents and independent mutation/diff/inverse witness bodies are preserved, with no old module aliases. Native verification remains required.\n\n"+changed.map(file=>"- "+file).join("\n")+"\n");console.log("[DEBUG] STEP class codec owners relocated: six subclasses");
}
if(process.argv[2]==="architect"){
 const artifact="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program",base=artifact+"/🏅️standards/🔖️1/🪆️subsets/✳️any",schema=base+"/🧬️schema/💡️inferences/🦀️.rs",io=base+"/🚪️io/🦀️.rs",table=base+"/🚪️io/📊️tables/🦀️.rs",csv=base+"/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs",tsv=base+"/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📑️tsv/🔖️iana/✳️any/🦀️.rs";
 const changed=new Set<string>(),prefix="crate::standards::v1::subsets::any::io",csvOwner=prefix+"::export::serializers::artifacts::csv::v_rfc4180::any",tsvOwner=prefix+"::export::serializers::artifacts::tsv::v_iana::any";
 let source=readFileSync(schema,"utf8");const tree=parser.parse(source)!;
 const groups=new Map([["REGISTER_ROW_COLUMNS",table],["RegisterCsvRow",table],["collect_rows",table],["header_row",table],["csv_record",csv],["export_registers_csv",csv],["export_relationships_csv",csv],["rows_to_csv_snapshot",csv],["export_registers_tsv",tsv],["rows_to_tsv_snapshot",tsv]]);
 const nodes=tree.rootNode.namedChildren.filter(node=>groups.has(node.childForFieldName("name")?.text??"")||node.type==="impl_item"&&node.childForFieldName("type")?.text==="RegisterCsvRow");
 if(nodes.length!==11)throw Error("Architect exchange owner inventory differs: "+nodes.length);
 const range=(node:any)=>{let start=node.startIndex,prior=node.previousNamedSibling;while(prior&&(prior.type==="attribute_item"||prior.type==="line_comment"&&prior.text.startsWith("///"))){start=prior.startIndex;prior=prior.previousNamedSibling;}return{start,end:node.endIndex};};
 const outputs=new Map<string,string>([[table,'//! 📊️ Register table projection shared by native exchange codecs.\nuse crate::{ProgramSnapshot,kernel::{EntityHeader,EntityId,PluginError}};\n'],[csv,readFileSync(csv,"utf8").replace("use crate::standards::v1::subsets::any::schema::inferences::export_registers_csv;","use crate::kernel::PluginError;\nuse crate::standards::v1::subsets::any::io::tables::{collect_rows,RegisterCsvRow,REGISTER_ROW_COLUMNS};\nuse semio_s_artifact_stdio_csv as stdio_csv;")],[tsv,'//! 📑️ Native IANA TSV register exchange.\nuse crate::{ProgramSnapshot,kernel::PluginError};\nuse crate::standards::v1::subsets::any::io::tables::{collect_rows,RegisterCsvRow,REGISTER_ROW_COLUMNS};\nuse semio_s_artifact_stdio_tsv as stdio_tsv;\nuse semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_line_ending;\n']]);
 for(const node of nodes){const at=range(node),target=node.type==="impl_item"?table:groups.get(node.childForFieldName("name")!.text)!;let body=source.slice(at.start,at.end);if(target===table)body=body.replace(/^const REGISTER_ROW_COLUMNS/m,"pub(crate) const REGISTER_ROW_COLUMNS").replace(/\n    fn columns\(/,"\n    pub(crate) fn columns(").replace(/^fn collect_rows/m,"pub(crate) fn collect_rows");outputs.set(target,outputs.get(target)!+"\n"+body+"\n");}
 for(const at of nodes.map(range).sort((a,b)=>b.start-a.start))source=source.slice(0,at.start)+source.slice(at.end);
 source=source.replace(/^use semio_s_artifact_stdio_(?:csv|tsv)[^\n]*\n/gm,"");
 outputs.set(schema,source);outputs.set(io,readFileSync(io,"utf8")+'\n#[path = "📊️tables/🦀️.rs"]\npub(crate) mod tables;\n');
 let mount=readFileSync(artifact+"/🦀️.rs","utf8");const anchor='                                pub mod xlsx {';if(!mount.includes(anchor))throw Error("Architect serializer mount missing");mount=mount.replace(anchor,'                                pub mod tsv {\n                                    #[path = "."]\n                                    pub mod v_iana {\n                                        #[path = "."]\n                                        pub mod any {\n                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📑️tsv/🔖️iana/✳️any/🦀️.rs"]\n                                            mod component;\n                                            pub use component::*;\n                                        }\n                                    }\n                                }\n                                #[path = "."]\n'+anchor);outputs.set(artifact+"/🦀️.rs",mount);
 for(const file of files.filter(file=>file.startsWith(artifact+"/")&&file!==schema&&file!==csv)){let body=readFileSync(file,"utf8"),before=body;
  body=body.replace(/use crate::standards::v1::subsets::any::schema::inferences::export_registers_csv;/g,"use "+csvOwner+"::export_registers_csv;").replace(/use crate::standards::v1::subsets::any::schema::inferences::\{export_registers_csv, export_registers_tsv\};/g,"use "+csvOwner+"::export_registers_csv;\n    use "+tsvOwner+"::export_registers_tsv;").replace(/use crate::standards::v1::subsets::any::schema::inferences::\{build_report, run_analysis, RegisterCsvRow\};/g,"use crate::standards::v1::subsets::any::schema::inferences::{build_report, run_analysis};\n    use "+prefix+"::tables::RegisterCsvRow;");
  if(file.endsWith("/tests-exchange/🦀️.rs"))body=body.replace("use super::*;","use "+csvOwner+"::export_relationships_csv;\nuse semio_s_artifact_stdio_csv as stdio_csv;");
  if(body!==before)outputs.set(file,body);
 }
 for(const[file,body]of outputs){if(parser.parse(body)!.rootNode.hasError())throw Error("Architect extraction invalid syntax: "+file);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,body);changed.add(file);}
 writeFileSync(join(ticket,"architect-native-exchange-owner-closure.md"),"# Architect Native Exchange Ownership\n\nCSV and TSV native serialization move from pure inference schema to their own canonical artifact export serializers. Register table projection is shared under IO tables, and existing editor/independent CSV witness consumers import physical owners explicitly. The source extraction preserves every existing projection and codec behavior without pure schema aliases. Fresh native compilation and runtime remain required.\n\n"+[...changed].map(file=>"- "+file).join("\n")+"\n");console.log("[DEBUG] Architect exchange extracted: 11 declarations, "+changed.size+" files");
}
if(process.argv[2]==="catalogue"){
 const artifact="✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757",base=artifact+"/🏅️standards/🔖️1/🪆️subsets/✳️any",root=artifact+"/🦀️.rs",text=base+"/🚪️io/📝️text/📸️snapshot/🦀️.rs",sqlite=base+"/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs",native=dirname(sqlite)+"/🛬️native/🦀️.rs",target=dirname(text)+"/🛬️native/🦀️.rs",schema=base+"/🧬️schema/📸️snapshot/🦀️.rs";
 let source=readFileSync(root,"utf8"),wire=readFileSync(text,"utf8"),sql=readFileSync(sqlite,"utf8"),decoded=readFileSync(native,"utf8"),semantic=readFileSync(schema,"utf8");
 const impls:any[]=[];const walk=(node:any)=>{if(node.type==="impl_item"&&node.childForFieldName("trait")?.text==="semio_framework_dsl_record::DslField")impls.push(node);for(const child of node.namedChildren)walk(child);};walk(parser.parse(source)!.rootNode);
 if(impls.length!==3)throw Error("Catalogue native field owner inventory differs: "+impls.length);
 const bodies=impls.map(node=>node.text.replace(/crate::snapshot::native_decoding::catalogue_value/g,"native_decoding::catalogue_value").replace(/crate::snapshot::native_decoding::part_number/g,"native_decoding::part_number").replace(/crate::snapshot::native_decoding::retire_value/g,"crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_value"));
 for(const node of impls.sort((a,b)=>b.startIndex-a.startIndex)){let start=node.startIndex,prior=node.previousNamedSibling;while(prior?.type==="line_comment"&&prior.text.startsWith("///")){start=prior.startIndex;prior=prior.previousNamedSibling;}source=source.slice(0,start)+source.slice(node.endIndex);}
 source=source.replace("crate::snapshot::native_decoding::retire_geometry","crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_geometry");
 const retire=parser.parse(decoded)!.rootNode.namedChildren.filter(node=>node.type==="function_item"&&["retire_value","retire_geometry"].includes(node.childForFieldName("name")?.text??""));if(retire.length!==2)throw Error("Catalogue semantic retirement inventory differs");
 const retirement=retire.map(node=>"/// ♻️ Releases decoded catalogue owners without recursive value destruction.\n"+node.text.replace(/^pub fn retire_value\(value:V\)/,"pub(crate) fn retire_decoded_catalogue_value(value:crate::CatalogueValue)").replace(/\bV::List\b/g,"crate::CatalogueValue::List").replace(/^pub fn retire_geometry/,"pub(crate) fn retire_decoded_catalogue_geometry"));
 for(const node of retire.sort((a,b)=>b.startIndex-a.startIndex))decoded=decoded.slice(0,node.startIndex)+decoded.slice(node.endIndex);
 decoded=decoded.replace(/\bretire_value\(/g,"crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_value(");
 wire+="\nuse crate::{CatalogueId,CatalogueValue,part_5::PartNumberRule};\n\n"+bodies.join("\n\n")+"\n\n"+["CatalogueId","CatalogueValue","PartNumberRule"].map(name=>"impl semio_framework_dsl_record::BorrowedDslField for "+name+" { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::"+(name==="CatalogueId"?"Text":"Value")+"; }").join("\n")+'\n\n#[path = "🛬️native/🦀️.rs"]\npub(crate) mod native_decoding;\n';
 sql=sql.replace(/\n#\[path = "🛬️native\/🦀️\.rs"\]\npub\(crate\) mod native_decoding;\n?/g,"\n");semantic+="\n"+retirement.join("\n\n")+"\n";
 const outputs=[[root,source],[text,wire],[sqlite,sql],[target,decoded],[schema,semantic]];for(const[file,body]of outputs){if(parser.parse(body)!.rootNode.hasError())throw Error("Catalogue owner extraction invalid syntax: "+file);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,body);}
 const {unlinkSync}=await import("node:fs");unlinkSync(native);
 writeFileSync(join(ticket,"catalogue-native-field-owner-closure.md"),"# Catalogue Native Field Ownership\n\nThe observed ISO16757 compiler RED included missing borrowed native field metadata. All three hand-authored native DSL field implementations now belong to text snapshot IO, alongside exact borrowed shape descriptors. Native catalogue construction moves from SQLite to text IO. Semantic recursive-owner retirement remains in snapshot schema, including the geometry derive hook. These changes preserve the existing exact integer, geometry, progress and cancellation laws; fresh native compilation and runtime verification remain required.\n\n## Updated Files\n\n"+outputs.map(([file])=>"- "+file).join("\n")+"\n- Removed: "+native+"\n");console.log("[DEBUG] Catalogue field ownership extracted: 3 native fields, 2 semantic retirement functions");
}
if(process.argv[2]==="repair-host"){
 const catalog=JSON.parse(readFileSync(join(ticket,"🗑️generated/native-host-owner-extraction.json"),"utf8")),changed:string[]=[];
 for(const row of catalog){let source=readFileSync(row.host,"utf8"),native=readFileSync(row.file,"utf8");const before=source;
  source=source.replace(/use (crate::standards::v1::subsets::any::io::binary::mutations)::\{([^}]+)\};/g,(match,prefix,members)=>{const keep=members.split(",").map((v:string)=>v.trim()).filter((v:string)=>!/Mutation$/.test(v));return keep.length?"use "+prefix+"::{"+keep.join(",")+"};":"";});
  source=source.replace(/pub use (?:mutations_codec|snapshot_codec|diff_codec)::\*;\n/g,"");
  const tree=parser.parse(source)!;for(const node of [...tree.rootNode.namedChildren].reverse()){if(node.type!=="use_declaration")continue;if(native.includes("#[cfg(test)]\n"+node.text)&&!source.slice(Math.max(0,node.startIndex-14),node.startIndex).includes("#[cfg(test)]"))source=source.slice(0,node.startIndex)+"#[cfg(test)]\n"+source.slice(node.startIndex);}
  if(source!==before){writeFileSync(row.host,source);changed.push(row.host);}
 }
 writeFileSync(join(ticket,"native-host-owner-import-repairs.md"),"# Host Owner Import Repairs\n\nOriginal cfg(test) admission is preserved for test-only semantic helpers. Host source imports semantic types from schema directly, and imported physical codec functions remain explicit at IO. No private IO semantic type aliases are added. Current compilation remains pending.\n\n"+changed.map(file=>"- "+file).join("\n")+"\n");console.log("[DEBUG] Host source import closure repaired: "+changed.length+" files");
}
if(process.argv[2]==="repair"){
 const changed=new Set<string>();
 for(const file of files){
  if(!file.includes("/🚪️io/💾️binary/")||file.includes("/🧪️tests/"))continue;
  let source=readFileSync(file,"utf8");if(!source.includes("mod native_codec"))continue;
  const clean=source.replace(/\bpub use (?:mutations_codec|snapshot_codec|diff_codec)::\*;\n/g,match=>{const start=source.indexOf("mod native_codec");return source.indexOf(match)>start?"":match;});
  if(clean!==source){writeFileSync(file,clean);changed.add(file);}
 }
 for(const file of files){
  if(!file.includes("/🚪️io/📝️text/🧬️mutations/")||file.includes("/🧪️tests/"))continue;
  let source=readFileSync(file,"utf8");if(!/\bfn \w+_bin\b/.test(source))continue;
  const tree=parser.parse(source)!,nodes=tree.rootNode.namedChildren.filter(node=>node.type==="function_item"&&node.childForFieldName("name")?.text.endsWith("_bin"));if(!nodes.length)continue;
  const names=new Set(nodes.map(node=>node.childForFieldName("name")!.text));let residue=source;for(const node of nodes)residue=residue.replace(node.text,"");const remainder=parser.parse(residue)!;
  const referenced=new Set<string>();const visit=(node:any)=>{if(["identifier","type_identifier"].includes(node.type)&&names.has(node.text))referenced.add(node.text);if(!["string_literal","raw_string_literal","line_comment","block_comment"].includes(node.type))for(const child of node.namedChildren)visit(child);};visit(remainder.rootNode);if(referenced.size)throw Error("Text consumer of physical binary helper requires explicit review: "+file+" "+[...referenced]);
  const destination=file.replace("/📝️text/","/💾️binary/"),target=readFileSync(destination,"utf8"),binaryTree=parser.parse(target)!,native=binaryTree.rootNode.namedChildren.find(node=>node.type==="mod_item"&&node.childForFieldName("name")?.text==="native_codec");if(!native)throw Error("Binary helper lacks canonical implementation owner: "+destination);
  let output=target,at=native.childForFieldName("body")!.endIndex-1;const declarations=nodes.map(node=>node.text.replace(/^(?:pub(?:\([^)]*\))? )?fn /,"pub(crate) fn ")).join("\n\n");output=output.slice(0,at)+"\n"+declarations+"\n"+output.slice(at);
  output=output.replace(/use crate::standards::v1::subsets::any::io::text::mutations::\{([^}]+)\};/g,(match,members)=>{const keep=members.split(",").map((v:string)=>v.trim()).filter((v:string)=>!names.has(v));return keep.length?"use crate::standards::v1::subsets::any::io::text::mutations::{"+keep.join(",")+"};":"";});
  for(const node of nodes.sort((a,b)=>b.startIndex-a.startIndex))source=source.slice(0,node.startIndex)+source.slice(node.endIndex);writeFileSync(file,source);writeFileSync(destination,output);changed.add(file);changed.add(destination);
 }
 writeFileSync(join(ticket,"native-codec-extraction-repairs.md"),"# Native Codec Extraction Repairs\n\nThe fresh ISO16757 compiler RED exposed a copied local module re-export. Local source module exports are excluded from relocated implementation scopes. Binary helper declarations retained only by documentation references now move to the binary implementation owner; no actual text consumer is permitted to depend on them. Fresh compiler and ownership checks remain required.\n\n## Updated Files\n\n"+[...changed].sort().map(file=>"- "+file).join("\n")+"\n");console.log("[DEBUG] Canonical codec relocation repairs: "+changed.size+" files");
}
if(process.argv[2]==="host"){
 const changed=new Set<string>(),catalog:any[]=[];
 const gen="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/",binary=gen+"💾️binary/🧬️mutations/🦀️.rs",text=gen+"📝️text/🧬️mutations/🦀️.rs";
 let source=readFileSync(binary,"utf8"),target=readFileSync(text,"utf8");const genTree=parser.parse(source)!;
 const genNames=new Set(["Generation2dOperationDsl","generation2d_operation_to_dsl","generation2d_operation_from_dsl"]);
 const genNodes=genTree.rootNode.namedChildren.filter(node=>genNames.has(node.childForFieldName("name")?.text??"")||node.type==="impl_item"&&node.childForFieldName("trait")?.text.endsWith("OpText"));
 const range=(node:any)=>{let start=node.startIndex,prior=node.previousNamedSibling;while(prior&&(prior.type==="attribute_item"||prior.type==="line_comment"&&prior.text.startsWith("///"))){start=prior.startIndex;prior=prior.previousNamedSibling;}return{start,end:node.endIndex};};
 const genUses=genTree.rootNode.namedChildren.filter(node=>node.type==="use_declaration"&&node.text.includes("io::text::snapshot")).map(node=>node.text).join("\n");
 const genMoved=genNodes.map(node=>node.text.replace(/^(enum|fn) /,"pub(crate) $1 ")).join("\n\n");
 for(const item of genNodes.map(range).sort((a,b)=>b.start-a.start))source=source.slice(0,item.start)+source.slice(item.end);
 source+="\nuse crate::standards::v1::subsets::any::io::text::mutations::{Generation2dOperationDsl,generation2d_operation_to_dsl,generation2d_operation_from_dsl};\n";
 target+="\n"+genUses+"\n"+genMoved+"\n";writeFileSync(binary,source);writeFileSync(text,target);changed.add(binary);changed.add(text);
 for(const file of files){
  if(!file.includes("/🚪️io/")||file.includes("/🧪️tests/"))continue;
  let before=readFileSync(file,"utf8");if(!/ArtifactStore|DocumentStoreOwners|PublicationLease/.test(before))continue;
  const tree=parser.parse(before)!,nodes=tree.rootNode.namedChildren.filter(node=>["function_item","struct_item","enum_item","const_item","static_item","type_item","impl_item"].includes(node.type));
  const tokens=(node:any):Set<string>=>{const names=new Set<string>();const visit=(node:any)=>{if(["identifier","type_identifier"].includes(node.type))names.add(node.text);if(!["string_literal","raw_string_literal","line_comment","block_comment"].includes(node.type))for(const child of node.namedChildren)visit(child);};visit(node);return names;};
  const names=new Map(nodes.map(node=>[node,node.childForFieldName("name")?.text??node.childForFieldName("type")?.text?.replace(/<.*$/s,"").split("::").at(-1)])),words=new Map(nodes.map(node=>[node,tokens(node)]));
  const wire=(node:any)=>node.type==="impl_item"&&/ArtifactPack|ArtifactDsl|OpBinary|OpText|ArtifactSqliteSnapshot/.test(node.childForFieldName("trait")?.text??"")||/^(?:encode|decode|print|parse)(?:_|[A-Z])/u.test(names.get(node)??"")||/^(?:COMPONENT_|MUTATION_GRAMMAR|Generation2dOperationDsl|generation2d_operation_)/u.test(names.get(node)??"");
  const selected=new Set(nodes.filter(node=>!wire(node)&&[...words.get(node)!].some(name=>/^(?:ArtifactStore|DocumentStoreOwners|\w*PublicationLease)$/u.test(name))));
  let again=true;while(again){again=false;const ownedNames=new Set([...selected].map(node=>names.get(node)).filter(Boolean));for(const node of nodes){if(wire(node)||selected.has(node))continue;const dependent=[...words.get(node)!].some(name=>ownedNames.has(name));const dependency=[...selected].some(owner=>names.get(node)&&words.get(owner)!.has(names.get(node)!));if(dependent||dependency){selected.add(node);again=true;}}}
  if(!selected.size){const clean=before.replace(/\buse store::\{ArtifactEnvelope, ArtifactStore\};/g,"use store::ArtifactEnvelope;");if(clean!==before){writeFileSync(file,clean);changed.add(file);}continue;}
  const artifact=file.slice(0,file.indexOf("/🏅️standards/")),host=artifact+"/🔨️modules/🏠️host/🧰️owned/🦀️.rs",mount=artifact+"/🦀️.rs",facet=file.includes("/📸️snapshot/")?"snapshot":"mutations",representation=file.includes("/📝️text/")?"text":"binary",owner="crate::standards::v1::subsets::any::io::"+representation+"::"+facet;
  const imported=nodes.filter(node=>node.type!=="impl_item"&&wire(node)&&[...selected].some(host=>names.get(node)&&words.get(host)!.has(names.get(node)!))).map(node=>names.get(node)).filter(Boolean);
  const imports=tree.rootNode.namedChildren.filter(node=>node.type==="use_declaration"&&!/\b(?:snapshot_codec|mutations_codec|diff_codec)::\*/.test(node.text)).map(node=>{const r=range(node);return before.slice(r.start,r.end);}).join("\n");
  const moved=[...selected].map(node=>{const r=range(node);return before.slice(r.start,r.end);}).join("\n\n");
  const publicNames=[...selected].filter(node=>node.type!=="impl_item").map(node=>names.get(node)).filter(Boolean) as string[];
  for(const r of [...selected].map(range).sort((a,b)=>b.start-a.start))before=before.slice(0,r.start)+before.slice(r.end);
  before=before.replace(/\buse store::\{ArtifactEnvelope, ArtifactStore\};/g,"");
  const native=nodes.filter(node=>!selected.has(node)).flatMap(node=>[...words.get(node)!]);
  if(!native.includes("ArtifactStore"))before=before.replace(/\bArtifactStore\s*,\s*|,\s*ArtifactStore\b/g,"");
  const output=(existsSync(host)?readFileSync(host,"utf8"):"//! 🏠️ Artifact document-store and publication authorities.\n")+"\n"+imports+(imported.length?"\nuse "+owner+"::{"+imported.join(",")+"};":"")+"\n"+moved+"\n";
  if(parser.parse(before)!.rootNode.hasError()&&!tree.rootNode.hasError())throw Error("Host extraction damaged native syntax: "+file);
  if(parser.parse(output)!.rootNode.hasError())throw Error("Host extraction damaged owner syntax: "+host);
  mkdirSync(dirname(host),{recursive:true});writeFileSync(file,before);writeFileSync(host,output);changed.add(file);changed.add(host);
  let mounted=readFileSync(mount,"utf8");if(!mounted.includes('pub mod host {')){mounted+='\n#[path = "."]\npub mod host {\n#[path = "🔨️modules/🏠️host/🧰️owned/🦀️.rs"]\npub mod owned;\n}\n';writeFileSync(mount,mounted);changed.add(mount);}else if(!mounted.includes('🔨️modules/🏠️host/🧰️owned/🦀️.rs'))throw Error("Existing host requires explicit mount integration: "+mount);
  catalog.push({artifact,file,host,owner,names:publicNames,imported,declarations:selected.size});
 }
 const consumers=execFileSync("rg",["--files","✏️s","🧰️framework","-g","*.rs"],{encoding:"utf8",maxBuffer:32*1024*1024}).trim().split("\n");
 for(const file of consumers){const before=readFileSync(file,"utf8");let source=before;for(const row of catalog){const native=row.owner.replace("crate::","");const names=new Set(row.names);source=source.replace(new RegExp("(\\b(?:\\w+::)+)"+native+"::(\\{[^}]*\\}|\\w+)","gu"),(match,prefix,members)=>{const list=members.startsWith("{")?members.slice(1,-1).split(",").map((v:string)=>v.trim()).filter(Boolean):[members],owned=list.filter((v:string)=>names.has(v.split(/\\s+as\\s+/)[0])),other=list.filter((v:string)=>!owned.includes(v));if(!owned.length)return match;return prefix+"host::owned::"+(members.startsWith("{")?"{"+owned.join(",")+"}":owned[0])+(other.length?";\nuse "+prefix+native+"::{"+other.join(",")+"}":"");});
   if(file.startsWith(dirname(row.file)+"/🧪️tests/"))source=source.replace(/\buse super::\*;/g,"use super::*;\nuse crate::host::owned::*;");
  }if(source!==before){writeFileSync(file,source);changed.add(file);}}
 writeFileSync(join(ticket,"native-host-owner-extraction.md"),"# Native Host Owner Extraction\n\nThe observed host-owner RED and independent tree-sitter GREEN now cover native document-store and publication authority ownership. Source extraction moves actual authorities and their local dependency closure into host::owned; native codec functions remain under their declared representation. Generation2d OpText and its local grammar twin/converters now belong to text mutation IO. No old IO re-export forwards to the host implementation. Fresh native verification and repository scan remain required.\n\n## Authorities\n\n"+catalog.map(row=>"- "+row.artifact+": "+row.declarations+" declarations").join("\n")+"\n\n## Updated Files\n\n"+[...changed].sort().map(file=>"- "+file).join("\n")+"\n");
 writeFileSync(join(ticket,"🗑️generated/native-host-owner-extraction.json"),JSON.stringify(catalog,null,2));console.log("[DEBUG] Host authority extraction completed: "+catalog.length+" owners, "+changed.size+" files");
}
if(process.argv[2]==="extract"){
 const grouped=new Map<string,any[]>();for(const row of rows){if(row.file.includes("/🌀️generation2d/")&&row.trait==="OpText")continue;const group=grouped.get(row.file)??[];group.push(row);grouped.set(row.file,group);}
 const changed:string[]=[];
 for(const[file,items]of grouped){
  if(items.some(row=>row.duplicate))throw Error("Duplicate implementation needs explicit review: "+file);
  let source=readFileSync(file,"utf8");const tree=parser.parse(source)!,selected=new Set(items.map(row=>row.start));
  const nodes:any[]=[];const walk=(node:any)=>{nodes.push(node);for(const child of node.namedChildren)walk(child);};walk(tree.rootNode);
  const scopes=new Set<any>();for(const node of nodes.filter(node=>selected.has(node.startIndex))){let current=node.parent;while(current){if(current.type==="source_file"||current.type==="declaration_list")scopes.add(current);current=current.parent;}}
  const local=new Map<string,any>();for(const scope of scopes)for(const node of scope.namedChildren){if(["function_item","struct_item","enum_item","const_item","type_item","static_item"].includes(node.type)){const name=node.childForFieldName("name")?.text;if(name)local.set(name,node);}}
  const needed=new Set<string>();for(const row of items)for(const[name]of local)if(new RegExp("\\b"+name+"\\b").test(row.text))needed.add(name);
  const move=new Set<string>();for(const name of needed)if(name.endsWith("_bin")||name.startsWith("TAG_")||name==="WIRE_PROTOCOL")move.add(name);
  let again=true;while(again){again=false;for(const name of [...move])for(const[dependency,node]of local)if(new RegExp("\\b"+dependency+"\\b").test(local.get(name).text)&&!needed.has(dependency)){needed.add(dependency);if(dependency.endsWith("_bin")||dependency.startsWith("TAG_")||dependency==="WIRE_PROTOCOL")move.add(dependency);again=true;}}
  for(const name of [...move]){const node=local.get(name);const other=source.slice(0,node.startIndex)+source.slice(node.endIndex);let residue=other;for(const row of items)residue=residue.replace(row.text,"");for(const dependency of move)if(dependency!==name)residue=residue.replace(local.get(dependency).text,"");if(new RegExp("\\b"+name+"\\b").test(residue))move.delete(name);}
  const owner=file.includes("/📝️text/")?"text":"binary",facet=file.includes("/📸️snapshot/")?"snapshot":"mutations",module="crate::standards::v1::subsets::any::io::"+owner+"::"+facet;
  const uses=[...scopes].flatMap(scope=>scope.namedChildren.filter((node:any)=>node.type==="use_declaration").map((node:any)=>node.text)).map(use=>use.replace(/\buse super::/g,"use "+module+"::"));
  const imports=[...needed].filter(name=>!move.has(name));
  const removals:{start:number,end:number}[]=items.map(row=>({start:row.start,end:row.end}));
  const declarations=[...move].map(name=>{const node=local.get(name);removals.push({start:node.startIndex,end:node.endIndex});return node.text;});
  const expose=imports.filter(name=>!/^pub(?:\([^)]*\))? /.test(local.get(name).text)).map(name=>({start:local.get(name).startIndex,text:"pub(crate) "}));
  const edits:any[]=[...removals.map(range=>({...range,text:""})),...expose.map(edit=>({...edit,end:edit.start}))];
  for(const edit of edits.sort((a,b)=>b.start-a.start))source=source.slice(0,edit.start)+edit.text+source.slice(edit.end);
  if(parser.parse(source)!.rootNode.hasError()&&!tree.rootNode.hasError())throw Error("Extraction caused source parse error: "+file);
  const destination=items[0].destination,target=readFileSync(destination,"utf8");
  const relocated=(text:string)=>text.replace(/include_str!\("(?:\.\.\/)+💾️binary\/🧬️mutations\/📡️\.protocol\.semio"\)/g,'include_str!("📡️.protocol.semio")');
  const body="\nmod native_codec {\nuse super::*;\n"+[...new Set(uses)].join("\n")+(imports.length?"\nuse "+module+"::{"+imports.join(",")+"};":"")+"\n"+declarations.map(relocated).join("\n\n")+"\n"+items.map(row=>relocated(row.text)).join("\n\n")+"\n}\n";
  if(parser.parse(target+body)!.rootNode.hasError()&&!parser.parse(target)!.rootNode.hasError())throw Error("Extraction caused destination parse error: "+destination);
  writeFileSync(file,source);writeFileSync(destination,target+body);changed.push(file,destination);
 }
 writeFileSync(join(ticket,"representation-owner-extraction.md"),"# Representation Owner Extraction\n\nThe observed repository-wide guard RED found fifty misplaced representation files. This extraction moved "+[...grouped.values()].flat().length+" concrete implementations into their canonical binary facets. Shared native record twins and conversion functions remain in the text grammar owner with crate-limited visibility; exclusive binary readers, writers and tags moved with their implementations. Existing wire bodies and authored protocol inputs are preserved. Generation2d text/runtime authority extraction is separate and remains pending. Native compilation and the fresh ownership scan remain required.\n\n## Updated Files\n\n"+changed.map(file=>"- "+file).join("\n")+"\n");
 console.log("[DEBUG] Canonical physical codec extraction completed: "+[...grouped.values()].flat().length+" implementations, "+changed.length+" files");
}
