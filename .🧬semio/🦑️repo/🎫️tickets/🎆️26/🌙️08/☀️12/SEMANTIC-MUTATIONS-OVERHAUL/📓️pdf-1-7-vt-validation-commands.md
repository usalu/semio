# PDF 1.7/VT Validation Commands

Exact non-Cargo validation commands executed for the VT cutover. All filesystem roots are explicit and exclude `compose/**`.

## ajv

```sh
bun -e 'const fs=require("fs"),path=require("path"),Ajv=require("ajv"); const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations"; const meta=JSON.parse(fs.readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json","utf8")); const ajv=new Ajv({strict:false,allErrors:true}); const check=ajv.compile(meta); const dirs=fs.readdirSync(root,{withFileTypes:true}).filter(e=>e.isDirectory()&&fs.existsSync(path.join(root,e.name,"🔣️component.json"))); const files=["🦀️component.rs","🟦️component.ts","🔗️component.graphql","🛰️component.proto","🔣️payload.schema.json","📝️text/🦀️component.rs","💾️binary/🦀️component.rs"]; let errors=[],surfaces=0,payloadCases=0,validSamples=[]; const aggregate=JSON.parse(fs.readFileSync(root+"/🔣️component.json","utf8")); for(const d of dirs){  const owner=root+"/"+d.name;  const desc=JSON.parse(fs.readFileSync(owner+"/🔣️component.json","utf8"));  if(!check(desc))errors.push([d.name,check.errors]);  if(desc.owner!==owner)errors.push([d.name,"owner mismatch"]);  for(const file of files){if(fs.existsSync(owner+"/"+file))surfaces++;else errors.push([d.name,"missing "+file]);}  const payload=JSON.parse(fs.readFileSync(owner+"/"+desc.payloadSchema,"utf8"));  const validatePayload=ajv.compile(payload);
 const sample={mutation:payload.properties.mutation.const};
 for(const [key,property] of Object.entries(payload.properties)){
  if(key==="mutation")continue;
  sample[key]=property.type==="boolean"?true:property.type==="string"?"sample":property.type==="integer"?0:property.type==="array"?[0,0,100,100]:{num:1,gen:0};
 }
 const invalidIdentity={...sample,mutation:"unknownMutation"},extra={...sample,extra:true};
 for(const [candidate,expected] of [[sample,true],[invalidIdentity,false],[extra,false]]){
  payloadCases++;if(validatePayload(candidate)!==expected)errors.push([desc.semanticKind,"payload instance",candidate]);
 }
 validSamples.push(sample);  const ref=aggregate.oneOf.find(r=>r.$ref.includes(d.name)).$ref;  ajv.addSchema(payload,ajv.opts.uriResolver.resolve(aggregate.$id,ref)); } try { const validateRoot=ajv.compile(aggregate); for(const sample of validSamples){payloadCases++;if(!validateRoot(sample))errors.push(["root payload",sample,validateRoot.errors]);} } catch(error) { errors.push(String(error)); } console.log(JSON.stringify({descriptors:dirs.length,payloads:dirs.length,surfaces,payloadCases,rootSchemaCompiled:errors.length===0,errors},null,2)); process.exit(errors.length?1:0);'
```

## internal

```sh
bun -e 'import { validateJsonSchemaSubset } from "./📜️script.ts";
const fs=require("fs"),Ajv=require("ajv");
const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations";
const schema=JSON.parse(fs.readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json","utf8"));
const oracle=new Ajv({strict:false}).compile(schema);
let cases=0,errors=[];
for(const entry of fs.readdirSync(root,{withFileTypes:true})){
 const p=root+"/"+entry.name+"/🔣️component.json";
 if(!entry.isDirectory()||!fs.existsSync(p))continue;
 const descriptor=JSON.parse(fs.readFileSync(p,"utf8"));
 const candidates=[descriptor,{...descriptor,binaryTag:-1},{...descriptor,outcomeClasses:[]},{...descriptor,invertibility:"unclassified"},{...descriptor,diffParticipation:"unclassified"},{...descriptor,extra:true}];
 candidates.forEach((value,index)=>{cases++;const own=validateJsonSchemaSubset(schema,value).length===0,third=oracle(value);if(own!==third||own!==(index===0))errors.push([entry.name,index,own,third]);});
}
console.log(JSON.stringify({internalAjvAgreementCases:cases,errors},null,2));
process.exit(errors.length?1:0);'
```

## parity

```sh
bun -e 'const fs=require("fs"),path=require("path");
const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations", expected=[{"emoji":"🔒️","kind":"insert-encryption-dictionary","variant":"InsertEncryptionDictionary","fields":[["version","i64"],["revision","i64"]],"tag":0,"folder":"🔒️insert-encryption-dictionary","module":"insert_encryption_dictionary","json":"insertEncryptionDictionary"},{"emoji":"🔓️","kind":"remove-encryption-dictionary","variant":"RemoveEncryptionDictionary","fields":[["version","i64"],["revision","i64"]],"tag":1,"folder":"🔓️remove-encryption-dictionary","module":"remove_encryption_dictionary","json":"removeEncryptionDictionary"},{"emoji":"🏳️","kind":"set-output-intent","variant":"SetOutputIntent","fields":[["identifier","String"]],"tag":2,"folder":"🏳️set-output-intent","module":"set_output_intent","json":"setOutputIntent"},{"emoji":"🧽️","kind":"remove-output-intent","variant":"RemoveOutputIntent","fields":[],"tag":3,"folder":"🧽️remove-output-intent","module":"remove_output_intent","json":"removeOutputIntent"},{"emoji":"📐️","kind":"set-trim-box","variant":"SetTrimBox","fields":[["page_index","usize"],["trim_box","[f64; 4]"]],"tag":4,"folder":"📐️set-trim-box","module":"set_trim_box","json":"setTrimBox"},{"emoji":"🧽️","kind":"remove-trim-box","variant":"RemoveTrimBox","fields":[["page_index","usize"]],"tag":5,"folder":"🧽️remove-trim-box","module":"remove_trim_box","json":"removeTrimBox"},{"emoji":"🔤️","kind":"embed-font-file","variant":"EmbedFontFile","fields":[["descriptor_ordinal","usize"],["key","String"],["program","ObjRef"]],"tag":6,"folder":"🔤️embed-font-file","module":"embed_font_file","json":"embedFontFile"},{"emoji":"🧺️","kind":"remove-font-file","variant":"RemoveFontFile","fields":[["descriptor_ordinal","usize"]],"tag":7,"folder":"🧺️remove-font-file","module":"remove_font_file","json":"removeFontFile"},{"emoji":"📜️","kind":"insert-javascript-action","variant":"InsertJavascriptAction","fields":[["script","String"]],"tag":8,"folder":"📜️insert-javascript-action","module":"insert_javascript_action","json":"insertJavascriptAction"},{"emoji":"🚫️","kind":"remove-javascript-action","variant":"RemoveJavascriptAction","fields":[["script","String"]],"tag":9,"folder":"🚫️remove-javascript-action","module":"remove_javascript_action","json":"removeJavascriptAction"},{"emoji":"🚀️","kind":"insert-launch-action","variant":"InsertLaunchAction","fields":[["target","String"]],"tag":10,"folder":"🚀️insert-launch-action","module":"insert_launch_action","json":"insertLaunchAction"},{"emoji":"🛬️","kind":"remove-launch-action","variant":"RemoveLaunchAction","fields":[["target","String"]],"tag":11,"folder":"🛬️remove-launch-action","module":"remove_launch_action","json":"removeLaunchAction"},{"emoji":"🎬️","kind":"insert-media-annotation","variant":"InsertMediaAnnotation","fields":[["subtype","String"],["title","String"]],"tag":12,"folder":"🎬️insert-media-annotation","module":"insert_media_annotation","json":"insertMediaAnnotation"},{"emoji":"⏹️","kind":"remove-media-annotation","variant":"RemoveMediaAnnotation","fields":[["subtype","String"],["title","String"]],"tag":13,"folder":"⏹️remove-media-annotation","module":"remove_media_annotation","json":"removeMediaAnnotation"},{"emoji":"🗂️","kind":"set-dpart-root","variant":"SetDpartRoot","fields":[["job","String"]],"tag":14,"folder":"🗂️set-dpart-root","module":"set_dpart_root","json":"setDpartRoot"},{"emoji":"🧹️","kind":"remove-dpart-root","variant":"RemoveDpartRoot","fields":[],"tag":15,"folder":"🧹️remove-dpart-root","module":"remove_dpart_root","json":"removeDpartRoot"},{"emoji":"🏷️","kind":"set-dpart-metadata","variant":"SetDpartMetadata","fields":[["job","String"]],"tag":16,"folder":"🏷️set-dpart-metadata","module":"set_dpart_metadata","json":"setDpartMetadata"},{"emoji":"🗑️","kind":"remove-dpart-metadata","variant":"RemoveDpartMetadata","fields":[],"tag":17,"folder":"🗑️remove-dpart-metadata","module":"remove_dpart_metadata","json":"removeDpartMetadata"}];
let errors=[];
const read=p=>fs.readFileSync(p,"utf8");
const source=read(root+"/🦀️component.rs");
const variants=[...source.matchAll(/^    (\w+)\(\1\),$/gm)].map(m=>m[1]);
const descriptors=fs.readdirSync(root,{withFileTypes:true}).filter(e=>e.isDirectory()&&fs.existsSync(root+"/"+e.name+"/🔣️component.json")).map(e=>JSON.parse(read(root+"/"+e.name+"/🔣️component.json"))).sort((a,b)=>a.binaryTag-b.binaryTag);
const kinds=descriptors.map(d=>d.semanticKind),variantNames=descriptors.map(d=>d.aggregateVariant);
if(JSON.stringify(variants)!==JSON.stringify(variantNames))errors.push("root enum order");
if(JSON.stringify(kinds)!==JSON.stringify(expected.map(e=>e.kind)))errors.push("descriptor roster");
const files=["🦀️component.rs","🟦️component.ts","🔗️component.graphql","🛰️component.proto","🔣️payload.schema.json","📝️text/🦀️component.rs","💾️binary/🦀️component.rs"];
for(const d of descriptors){
 const dir=d.owner;
 for(const f of files)if(!fs.existsSync(dir+"/"+f))errors.push("missing "+dir+"/"+f);
 const leaf=read(dir+"/🦀️component.rs");
 if(!leaf.includes("impl MutationKind<PdfSnapshot, PdfVtMutation> for "+d.aggregateVariant))errors.push("leaf behavior "+d.semanticKind);
 if(!source.includes(dir.slice(root.length+1)+"/🦀️component.rs"))errors.push("mount "+d.semanticKind);
 if(!read(dir+"/📝️text/🦀️component.rs").includes('\''OPCODE: &str = "'\''+d.textOpcode+'\''"'\''))errors.push("text opcode "+d.semanticKind);
 if(!read(dir+"/💾️binary/🦀️component.rs").includes("TAG: u8 = "+d.binaryTag+";"))errors.push("binary tag "+d.semanticKind);
 for(const f of ["🟦️component.ts","🔗️component.graphql","🛰️component.proto","📝️text/🦀️component.rs","💾️binary/🦀️component.rs"])if(!read(root+"/"+f).includes(d.aggregateVariant))errors.push("root identity "+f+" "+d.semanticKind);
}
const rootSchema=JSON.parse(read(root+"/🔣️component.json"));
if(rootSchema.oneOf.length!==18)errors.push("root schema count");
const manifest=JSON.parse(read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧪️oracle/🔣️component.json"));
if(JSON.stringify(manifest.mutationCatalogs[0].kinds)!==JSON.stringify(kinds))errors.push("oracle manifest kinds");
for(const file of ["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-vt/🦀️component.rs"]){
 const local=read(file).match(/(?:pub )?const KINDS:.*?= &\[(.*?)\];/s);
 const items=[...local[1].matchAll(/"([^"]+)"/g)].map(m=>m[1]);
 if(JSON.stringify(items)!==JSON.stringify(kinds))errors.push("oracle/adapter kinds "+file);
}
const rows=[...read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-vt/component.feature").matchAll(/^\s*\| ([a-z][a-z-]+)\s+\|/gm)].map(m=>m[1]).filter(k=>k!=="id");
if(rows.length!==36||new Set(rows).size!==18||rows.some(k=>!kinds.includes(k)))errors.push("feature rows");
const walk=d=>fs.readdirSync(d,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(d+"/"+e.name):[d+"/"+e.name]);
const all=walk(root);
if(all.some(p=>p.includes("/🦠️mutation/")||p.includes("/set-snapshot/")||p.includes("🧾️component.schema.json")))errors.push("forbidden owner/schema path");
console.log(JSON.stringify({leaves:descriptors.length,files:all.length,rust:18,typescript:18,graphql:18,protobuf:18,jsonSchema:18,text:18,binary:18,oracle:kinds.length,featureRows:rows.length,featureKinds:new Set(rows).size,tags:descriptors.map(d=>d.binaryTag),errors},null,2));
process.exit(errors.length?1:0);'
```

## nightly

```sh
bun -e 'const fs=require("fs"),cp=require("child_process");
const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt",adapter="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7-vt/🦀️component.rs";
const walk=d=>fs.readdirSync(d,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(d+"/"+e.name):[d+"/"+e.name]);
const local=walk(base).filter(p=>p.endsWith(".rs")), files=[...local,adapter],errors=[];
for(const file of files){const result=cp.spawnSync("rustc",["+nightly","-Z","parse-crate-root-only","--edition","2021","--crate-type","lib",file],{encoding:"utf8"});if(result.status!==0)errors.push({file,status:result.status,stderr:result.stderr});}
console.log(JSON.stringify({vtRustFiles:local.length,adapterFiles:1,errors},null,2));
process.exit(errors.length?1:0);'
```

## ts

```sh
bun -e 'const fs=require("fs");
const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations";
await import("./"+root+"/🟦️component.ts");
let imports=1;
for(const dir of fs.readdirSync(root,{withFileTypes:true})){
 if(!dir.isDirectory()||!fs.existsSync(root+"/"+dir.name+"/🔣️component.json"))continue;
 await import("./"+root+"/"+dir.name+"/🟦️component.ts");imports++;
}
console.log(JSON.stringify({typescriptImports:imports,errors:0}));'
```
