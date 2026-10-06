import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo",out=join(ticket,"🗑️generated/cargo-nested-owners"),[command,epoch="1"]=process.argv.slice(2),require=createRequire(join(root,"package.json")),Ajv=require("ajv"),TOML=require("@iarna/toml"),sha=(s:string)=>createHash("sha256").update(s).digest("hex");
mkdirSync(out,{recursive:true});
const fixture={schemaVersion:1,cases:[
 {id:"admitted-nested-owner",admitted:true,removed:"none",expectedOwnerDirectories:[".","owners/application","owners/application/extension/artifact/oracle/native"],applicationMetadataValid:true},
 {id:"unadmitted-native-owner",admitted:false,removed:"none",expectedOwnerDirectories:[".","owners/application"],applicationMetadataValid:true},
 {id:"removed-artifact-retains-consumer-error",admitted:true,removed:"artifact",expectedOwnerDirectories:[".","owners/application"],applicationMetadataValid:false},
 {id:"removed-application-retains-general",admitted:true,removed:"application",expectedOwnerDirectories:["."],applicationMetadataValid:true}
]};
const schema={$schema:"http://json-schema.org/draft-07/schema#",type:"object",additionalProperties:false,required:["schemaVersion","cases"],properties:{schemaVersion:{const:1},cases:{type:"array",minItems:4,items:{type:"object",additionalProperties:false,required:["id","admitted","removed","expectedOwnerDirectories","applicationMetadataValid"],properties:{id:{type:"string",minLength:1},admitted:{type:"boolean"},removed:{enum:["none","artifact","application"]},expectedOwnerDirectories:{type:"array",minItems:1,uniqueItems:true,items:{type:"string",minLength:1}},applicationMetadataValid:{type:"boolean"}}}}}};
const test=`import { test, expect } from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { discoverCargoWorkspaces, cargoRepositoryPackages, cargoWorkspaceForManifest } from "../../🟦️.ts";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🪆️nested-owners/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🪆️nested-owners/🔣️.json"),"utf8")),artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifactRoot)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
const put=(root:string,path:string,text:string):void=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const native=(root:string,manifest:string)=>Bun.spawnSync(["cargo","metadata","--offline","--no-deps","--format-version","1","--manifest-path",join(root,manifest)],{cwd:root,stdout:"pipe",stderr:"pipe"});
const pkg=(name:string,dependencies="")=>'[package]\\nname="'+name+'"\\nversion="0.1.0"\\nedition="2021"\\n[lib]\\npath="🦀️.rs"\\n'+dependencies;
test("nested owner corpus is closed and admits self-package manifest authority",()=>{
 const validate=new Ajv({strict:true}).compile(schema),admission=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/📇️discovery/🔣️.json"),"utf8")));
 expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:true})).toBe(false);
 expect(admission({"schema-version":1,"member-manifests":["Cargo.toml"]})).toBe(true);
 expect(admission({"schema-version":1,"member-manifests":["../Cargo.toml"]})).toBe(false);
});
test("authored nested native owners survive deletion without concealing retained broken consumers",()=>{
 for(const row of fixture.cases){
  mkdirSync(artifactRoot!,{recursive:true});const root=mkdtempSync(join(artifactRoot!,"nested-owner-"));
  put(root,"Cargo.toml",'[workspace]\\nresolver="2"\\nmembers=["framework/kernel"]\\nexclude=["owners/**"]\\n[workspace.metadata.semio.repository]\\nschema-version=1\\nmember-manifests=["framework/*/Cargo.toml"]\\nowner-manifests=["owners/*/Cargo.toml","owners/**/native/Cargo.toml"]\\n');
  put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\\n");
  if(row.removed!=="application"){
   put(root,"owners/application/Cargo.toml",'[workspace]\\nresolver="2"\\nmembers=["app"]\\nexclude=["extension/**"]\\n[workspace.metadata.semio.repository]\\nschema-version=1\\nmember-manifests=["app/Cargo.toml"]\\n');
   put(root,"owners/application/app/Cargo.toml",pkg("application",'[dev-dependencies]\\nindependent-oracle={path="../extension/artifact/oracle/native"}\\n'));put(root,"owners/application/app/🦀️.rs","pub fn app() {}\\n");
   if(row.removed!=="artifact"){
    const admission=row.admitted?'[workspace.metadata.semio.repository]\\nschema-version=1\\nmember-manifests=["Cargo.toml"]\\n':'';
    put(root,"owners/application/extension/artifact/oracle/native/Cargo.toml",'[workspace]\\nresolver="2"\\nmembers=["."]\\n'+admission+pkg("independent-oracle"));put(root,"owners/application/extension/artifact/oracle/native/🦀️.rs","pub fn oracle() {}\\n");
   }
  }
  const scopes=discoverCargoWorkspaces(root),packages=cargoRepositoryPackages(root);expect(scopes.map(s=>s.directory).sort()).toEqual([...row.expectedOwnerDirectories].sort());expect(packages.length).toBe(scopes.length);
  for(const scope of scopes){const source=readFileSync(join(root,scope.manifest),"utf8");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));const actual=native(root,scope.manifest);expect(actual.exitCode===0,actual.stderr.toString()).toBe(scope.directory==="owners/application"?row.applicationMetadataValid:true);if(actual.exitCode===0){const metadata=JSON.parse(actual.stdout.toString());expect(metadata.workspace_members.length).toBe(packages.filter(p=>p.workspace===scope.directory).length);}}
  if(row.removed==="none")expect(cargoWorkspaceForManifest(root,"owners/application/extension/artifact/oracle/native/Cargo.toml").directory).toBe("owners/application/extension/artifact/oracle/native");
  console.log("[DEBUG] Nested Cargo owner "+JSON.stringify({id:row.id,owners:scopes.map(s=>s.directory),packages:packages.map(p=>p.name)}));
 }
});
`;
const pairs:any[]=[];
function proposedApi(source:string):string{
 const keys='["schema-version", "owner-manifests", "member-manifests"]';assert.equal(source.split(keys).length-1,1);source=source.replace(keys,'["schema-version", "owner-manifests", "member-manifests", "exclude-patterns"]');
 const authority='memberManifests, contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: workspace.exclude ?? [] }) };';assert.equal(source.split(authority).length-1,1);source=source.replace(authority,'memberManifests, contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: admission["exclude-patterns"] }) };');
 const glob='const files = [...new Set(["Cargo.toml", ...patterns.flatMap(pattern => [...new Bun.Glob(pattern).scanSync({ cwd: root, dot: true, onlyFiles: true, followSymlinks: false })])])];';assert.equal(source.split(glob).length-1,1);
 source=source.replace(glob,`const files = ["Cargo.toml"], matchers=patterns.map(pattern=>new Bun.Glob(pattern)), opaque=new Set(["node_modules","target","dist","build","🤖️generated","🗑️generated","coverage","temp","compose"]);
  const prefixes=patterns.map(pattern=>pattern.split(/[*?\\[{]/u)[0]!.replace(/\\/$/u,""));
  const walk=(directory:string):void=>{
    if(directory && !prefixes.some(prefix=>!prefix || directory===prefix || directory.startsWith(prefix+"/") || prefix.startsWith(directory+"/")))return;
    const depth=directory?directory.split("/").length:0;
    if(!patterns.some(pattern=>pattern.split("/").includes("**") || depth<pattern.split("/").length))return;
    for(const entry of readdirSync(join(root,directory),{withFileTypes:true})){
      if(entry.name.startsWith(".") || opaque.has(entry.name) || entry.isSymbolicLink())continue;
      const path=directory?directory+"/"+entry.name:entry.name;
      if(entry.isDirectory())walk(path);
      else if(entry.isFile() && entry.name==="Cargo.toml" && matchers.some(pattern=>pattern.match(path)))files.push(path);
    }
  };
  walk("");`);
 const member='if (directory && leaves.some(pattern => pattern.match(directory))) {';assert.equal(source.split(member).length-1,1);source=source.replace(member,`if(directory){const manifest=join(cwd,directory,"Cargo.toml");if(existsSync(manifest) && read(root,slash(relative(root,manifest))).workspace!==undefined)return;}
    `+member);
 const start=source.indexOf('export function publishCargoWorkspaceMembership('),end=source.indexOf('\nexport type CargoPreparationV1',start);assert.ok(start>=0 && end>start);
 source=source.slice(0,start)+`export function publishCargoWorkspaceMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write"): boolean {
  const path=physical(root,owner.manifest),source=readFileSync(path,"utf8"),document=object(Bun.TOML.parse(source));
  const members=cargoWorkspaceMembers(root,owner).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))) || ".");
  const children=discoverCargoWorkspaces(root).filter(row=>row.directory!==owner.directory && (owner.directory==="." || row.directory.startsWith(owner.directory+"/"))).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))));
  const exclude=[...new Set([...owner.contribution.exclude,...children])].sort();
  if(JSON.stringify(document.workspace.members)===JSON.stringify(members) && JSON.stringify(document.workspace.exclude??[])===JSON.stringify(exclude))return false;
  if(mode==="check")throw Error(\`Cargo source membership is stale: \${owner.manifest}\`);
  let after=source;
  for(const [name,values] of [["members",members],["exclude",exclude]] as const){
    const heading=/^\\[workspace\\]\\r?$/m.exec(after);if(!heading)throw Error(\`Cargo workspace table is absent: \${owner.manifest}\`);
    const bodyStart=heading.index+heading[0].length,next=/^\\[/m.exec(after.slice(bodyStart)),section=after.slice(bodyStart,next?bodyStart+next.index:after.length),field=new RegExp("^"+name+"\\\\s*=\\\\s*\\\\[","m").exec(section);
    const replacement=name+" = [\\n"+values.map(value=>"    "+JSON.stringify(value)+",").join("\\n")+"\\n]";
    if(!field){after=after.slice(0,bodyStart)+"\\n"+replacement+after.slice(bodyStart);continue;}
    const start=bodyStart+field.index;let end=start+field[0].length,quote="",escaped=false;
    for(;end<after.length;end++){const character=after[end]!;if(quote){if(escaped)escaped=false;else if(quote==='"' && character==="\\\\")escaped=true;else if(character===quote)quote="";}else if(character==='"' || character==="'")quote=character;else if(character==="]"){end++;break;}}
    if(end>=after.length)throw Error(\`Unterminated Cargo workspace array: \${owner.manifest}\`);
    after=after.slice(0,start)+replacement+after.slice(end);
  }
  if(readFileSync(physical(root,owner.manifest),"utf8")!==source)throw Error(\`Cargo workspace changed during member discovery: \${owner.manifest}\`);
  writeFileSync(path,after);return true;
}
`+source.slice(end);
 return source;
}
function pair(path:string,after:string|null){const full=join(root,path),before=existsSync(full)?readFileSync(full,"utf8"):null;if(before===after)return; pairs.push({path,before,after,beforeHash:before===null?null:sha(before),afterHash:after===null?null:sha(after)});}
if(command==="stage"){
 const destination=join(out,"source-"+epoch+".json");assert.equal(existsSync(destination),false);
 const currentRoot=readFileSync(join(root,"Cargo.toml"),"utf8"),rootDocument=Bun.TOML.parse(currentRoot) as any;assert.deepEqual(rootDocument,TOML.parse(currentRoot));const old=rootDocument.workspace.metadata.semio.repository["owner-manifests"],pattern="[!.]*/**/📦️packages/🦀️rust/Cargo.toml";assert.equal(old.includes(pattern),false);const marker="owner-manifests = "+JSON.stringify(old);assert.equal(currentRoot.split(marker).length-1,1);pair("Cargo.toml",currentRoot.replace(marker,"owner-manifests = "+JSON.stringify([...old,pattern])));
 const isolation:any[]=[];
 const files=[...new Bun.Glob(pattern).scanSync({cwd:root,dot:false,onlyFiles:true,followSymlinks:false})].sort();
 for(const path of files){if(path.split("/").some(p=>["node_modules","target","dist","build","🤖️generated","🗑️generated"].includes(p)))continue;const before=readFileSync(join(root,path),"utf8"),document=Bun.TOML.parse(before) as any;assert.deepEqual(document,TOML.parse(before));if(!document.workspace || document.workspace.metadata?.semio?.repository)continue;const guest=path.includes("/👽️guest/");isolation.push({path,guest,members:document.workspace.members??null,package:document.package?.name??null,before,beforeHash:sha(before)});if(guest)continue;assert.ok(document.package?.name,"isolated source owner needs a self package: "+path);assert.ok(!document.workspace.members || document.workspace.members.length===0,"non-single owner needs authored membership: "+path);const heading=/^\[workspace\]\r?$/m.exec(before);assert.ok(heading);const replacement='[workspace]\nmembers = ["."]\n\n[workspace.metadata.semio.repository]\nschema-version = 1\nmember-manifests = ["Cargo.toml"]';const after=before.slice(0,heading.index)+replacement+before.slice(heading.index+heading[0].length);const proposed=Bun.TOML.parse(after) as any;assert.deepEqual(proposed,TOML.parse(after));assert.deepEqual(proposed.package,document.package);assert.deepEqual(proposed.dependencies,document.dependencies);pair(path,after);}
 const schemaPath=owner+"/🧬️schema/📇️discovery/🔣️.json",beforeSchema=readFileSync(join(root,schemaPath),"utf8"),admission=JSON.parse(beforeSchema);for(const key of ["member-manifests","owner-manifests"])admission.properties[key].items.pattern="^(?!/)(?![A-Za-z]:/)(?!.*(?:^|/)\\.\\.(?:/|$))(?!.*\\\\)(?:.*/)?Cargo\\.toml$";pair(schemaPath,JSON.stringify(admission,null,2)+"\n");
 pair(owner+"/🧫️fixtures/🪆️nested-owners/🔣️.json",JSON.stringify(fixture,null,2)+"\n");pair(owner+"/🧬️schema/🪆️nested-owners/🔣️.json",JSON.stringify(schema,null,2)+"\n");pair(owner+"/🧪️tests/🪆️nested-owners/🟦️.ts",test);
 const scriptPath=owner+"/📜️script.ts",script=readFileSync(join(root,scriptPath),"utf8"),testMarker='["test", join(import.meta.dir, "🧪️tests/🟦️.ts")]';assert.equal(script.split(testMarker).length-1,1);pair(scriptPath,script.replace(testMarker,'["test", join(import.meta.dir, "🧪️tests/🟦️.ts"), join(import.meta.dir,"🧪️tests/🪆️nested-owners/🟦️.ts")]'));
 if(Number(epoch)>=3){
  admission.required.push("exclude-patterns");admission.properties["exclude-patterns"]={type:"array",uniqueItems:true,items:{type:"string",minLength:1,pattern:"^(?!/)(?![A-Za-z]:/)(?!.*(?:^|/)\\.\\.(?:/|$))(?!.*\\\\).+$"}};
  const schemaRow=pairs.find(row=>row.path===schemaPath);schemaRow.after=JSON.stringify(admission,null,2)+"\n";schemaRow.afterHash=sha(schemaRow.after);
  const docs=["Cargo.toml",...new Bun.Glob(rootDocument.workspace.metadata.semio.repository["owner-manifests"][0]).scanSync({cwd:root,dot:false,onlyFiles:true,followSymlinks:false})];
  const scopePaths=[...new Set([...docs,...isolation.filter(row=>!row.guest).map(row=>row.path)])].filter(path=>{const row=pairs.find(x=>x.path===path),document=Bun.TOML.parse(row?.after??readFileSync(join(root,path),"utf8")) as any;return document.workspace?.metadata?.semio?.repository!==undefined;});
  for(const path of scopePaths){const existing=pairs.find(row=>row.path===path),text=existing?.after??readFileSync(join(root,path),"utf8"),document=Bun.TOML.parse(text) as any,cwd=dirname(path),children=scopePaths.filter(child=>child!==path && (path==="Cargo.toml" || child.startsWith(cwd+"/"))).map(child=>relative(cwd,dirname(child)).replaceAll("\\","/")),excluded=(document.workspace.exclude??[]).filter((pattern:string)=>!children.includes(pattern));assert.equal(document.workspace.metadata.semio.repository["exclude-patterns"],undefined);const heading="[workspace.metadata.semio.repository]";assert.equal(text.split(heading).length-1,1);const after=text.replace(heading,heading+"\nexclude-patterns = "+JSON.stringify(excluded));assert.deepEqual((Bun.TOML.parse(after) as any).package,document.package);if(existing){existing.after=after;existing.afterHash=sha(after);}else pair(path,after);}
  pair(owner+"/🟦️.ts",proposedApi(readFileSync(join(root,owner,"🟦️.ts"),"utf8")));
  const library=owner.slice(0,-"/🗂️workspaces/🦀️cargo".length);
  for(const path of [owner+"/🧪️tests/🟦️.ts",owner+"/🧪️tests/🕰️queued-preparation/🟦️.ts",library+"/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts",library+"/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts",library+"/⚡️caching/🧪️tests/📦️dependencies/🟦️.ts"]){const before=readFileSync(join(root,path),"utf8");let after=before.replaceAll('schema-version=1\\n','schema-version=1\\nexclude-patterns=[]\\n').replace('"schema-version": 1, "member-manifests"','"schema-version": 1, "exclude-patterns": [], "member-manifests"');if(path.includes("cargo-discovery-exclusions"))after=after.replace('exclude-patterns=[]\\n','exclude-patterns=\'+JSON.stringify(vector.opaquePaths)+\'\\n');assert.notEqual(after,before,path);pair(path,after);}
  for(const path of [library+"/⚡️caching/🧫️fixtures/📦️native-dependencies/🔣️.json",library+"/⚡️caching/🧫️fixtures/📦️native-dependencies/🛂️schema/🔣️.json"]){const before=readFileSync(join(root,path),"utf8"),document=JSON.parse(before),update=(value:any):any=>typeof value==="string"?value.replaceAll('schema-version = 1\n','schema-version = 1\nexclude-patterns = []\n'):Array.isArray(value)?value.map(update):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,row])=>[key,update(row)])):value;pair(path,JSON.stringify(update(document),null,2)+"\n");}
  const testRow=pairs.find(row=>row.path.endsWith("/🪆️nested-owners/🟦️.ts"));testRow.after=testRow.after.replace('cargoWorkspaceForManifest }','cargoWorkspaceForManifest, publishCargoWorkspaceMembership }').replaceAll('"member-manifests":["Cargo.toml"]','"member-manifests":["Cargo.toml"],"exclude-patterns":[]').replaceAll('schema-version=1\\n','schema-version=1\\nexclude-patterns=[]\\n').replace('const actual=native(root,scope.manifest);','publishCargoWorkspaceMembership(root,scope,"write");const actual=native(root,scope.manifest);');
  const appMarker='exclude=["extension/**"]\\n[workspace.metadata.semio.repository]\\nschema-version=1\\nexclude-patterns=[]';assert.equal(testRow.after.split(appMarker).length-1,1);testRow.after=testRow.after.replace(appMarker,'exclude=["extension/**"]\\n[workspace.metadata.semio.repository]\\nschema-version=1\\nexclude-patterns=\'+JSON.stringify(row.admitted?[]:["extension/artifact/oracle/native"])+\'');testRow.afterHash=sha(testRow.after);
 }
 const validate=new Ajv({strict:true}).compile(schema);assert.equal(validate(fixture),true);assert.equal(validate({...fixture,extra:true}),false);const admissionValidator=new Ajv({strict:true}).compile(admission);assert.equal(admissionValidator({"schema-version":1,"member-manifests":["Cargo.toml"],...(Number(epoch)>=3?{"exclude-patterns":[]}:{} )}),true);
 const contexts=[owner+"/🟦️.ts",owner+"/🧪️tests/🟦️.ts",owner+"/📋️project.json"].map(path=>{const source=readFileSync(join(root,path),"utf8");return{path,source,sha256:sha(source)};});
 for(const row of pairs)assert.equal(existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null,row.before);for(const row of contexts)assert.equal(readFileSync(join(root,row.path),"utf8"),row.source);const producer={path:import.meta.path,source:readFileSync(import.meta.path,"utf8")};
 writeFileSync(destination,JSON.stringify({producer,pairs,contexts,isolation,fixture,schema,sourceReady:true,productionPublicationReady:false,nativeDeletionReady:false}));console.log(JSON.stringify({destination,rows:pairs.length,isolated:isolation.length,retainedGuests:isolation.filter(x=>x.guest).length,admitted:isolation.filter(x=>!x.guest).length}));
}else if(command==="model" || command==="red"){
 const sourcePath=join(out,"source-"+epoch+".json"),raw=readFileSync(sourcePath,"utf8"),source=JSON.parse(raw),model=join(out,command+"-"+epoch);assert.equal(existsSync(model),false);mkdirSync(model,{recursive:true});
 for(const row of source.pairs){if(row.path.startsWith(owner+"/") && row.after!==null){const path=join(model,row.path.slice(owner.length+1));mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.after);}}
 if(command==="red"){const prior=source.pairs.find((x:any)=>x.path===owner+"/🧬️schema/📇️discovery/🔣️.json");writeFileSync(join(model,"🧬️schema/📇️discovery/🔣️.json"),prior.before);}const context=source.contexts.find((x:any)=>x.path===owner+"/🟦️.ts"),api=join(model,"🟦️.ts");writeFileSync(api,source.pairs.find((row:any)=>row.path===owner+"/🟦️.ts")?.after??context.source);const invocation=readFileSync(join(root,owner,"🧬️schema/🏃️invocation/🔣️.json"),"utf8");mkdirSync(join(model,"🧬️schema/🏃️invocation"),{recursive:true});writeFileSync(join(model,"🧬️schema/🏃️invocation/🔣️.json"),invocation);
 const env={...process.env,SEMIO_TEST_ARTIFACT_DIR:join(model,"native")};const result=Bun.spawnSync([process.execPath,"test",join(model,"🧪️tests/🪆️nested-owners/🟦️.ts")],{cwd:root,env,stdout:"pipe",stderr:"pipe"});writeFileSync(join(model,"actual.log"),Buffer.concat([result.stdout,result.stderr]));assert.equal(readFileSync(sourcePath,"utf8"),raw);const terminal={sourcePath,sourceHash:sha(raw),exitCode:result.exitCode,log:join(model,"actual.log"),nativeAndIndependentOraclesExecuted:true,productionPublicationReady:false};writeFileSync(join(model,"terminal.json"),JSON.stringify(terminal));console.log(JSON.stringify(terminal));process.exit(result.exitCode);
}else throw Error("stage|model <epoch>");
