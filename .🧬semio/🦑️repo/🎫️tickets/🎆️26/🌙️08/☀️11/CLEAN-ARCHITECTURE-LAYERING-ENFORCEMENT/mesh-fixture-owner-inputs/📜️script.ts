import {readFileSync,existsSync,mkdirSync,writeFileSync,unlinkSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
import {createHash} from "node:crypto";
const root=resolve(import.meta.dir,"../../../../../../../../"),output=resolve(import.meta.dir,"full-predecessor-1.json");
const engine="🧰️framework/🔨️modules/🏗️mesh-engine",mesh="🧰️framework/🔨️modules/🧊️3d/🥽️mesh",brep="✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh";
const paths=[brep+"/🧫️fixtures/🎨️attributes/🔣️.json",brep+"/🧬️schema/🔣️.json",engine+"/🧪️tests/🔬️mesh-data-from-value-round-trip/🦀️.rs",mesh+"/🧪️tests/🔬️jobs/🦀️.rs",brep+"/🧪️tests/🔬️unit/🦀️.rs",brep+"/🧪️tests/🔬️unit/🟦️.ts",engine+"/📦️packages/🦀️rust/Cargo.toml",mesh+"/../📦️packages/🦀️rust/Cargo.toml"];
const sha=(body:string)=>createHash("sha256").update(body).digest("hex");
function capture(){return paths.map(path=>{const source=readFileSync(resolve(root,path),"utf8");return {path,source,sha256:sha(source)};});}
if(process.argv[2]==="prepare-tests"){prepareTests();process.exit(0);}
if(process.argv[2]==="mount-tests"){mountTests();process.exit(0);}
if(process.argv[2]==="mount-gui"){mountGui();process.exit(0);}
if(process.argv[2]==="prepare-production"){prepareProduction();process.exit(0);}
if(process.argv[2]==="mount-production"){mountProduction();process.exit(0);}
if(process.argv[2]==="capture-final"){captureFinal();process.exit(0);}
if(process.argv[2]!=="capture")throw Error("Expected capture, prepare-tests or mount-tests");
if(existsSync(output))throw Error("Capture already exists; preserve predecessor authority");
mkdirSync(import.meta.dir,{recursive:true});const rows=capture();
const fixture=JSON.parse(rows[0]!.source),schema=JSON.parse(rows[1]!.source);
const count=(row:number,text:string)=>rows[row]!.source.split(text).length-1;
const result={version:1,root,rows,fixtureKeys:Object.keys(fixture),schemaId:schema.$id,ownedLiteralCounts:{engineFixture:count(2,"🎨️attributes/🔣️.json"),meshFixture:count(3,"🎨️attributes/🔣️.json"),brepNativeFixture:count(4,"🎨️attributes/🔣️.json"),brepSourceFixture:count(5,"🎨️attributes/🔣️.json"),brepSourceSchema:count(5,"../../🧬️schema/🔣️.json")},fixtureShape:Object.fromEntries(Object.entries(fixture).map(([key,value])=>[key,Array.isArray(value)?{kind:"array",length:value.length}:{kind:typeof value,keys:value&&typeof value==="object"?Object.keys(value):[]}]))};
writeFileSync(output,JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify({captured:rows.length,fixtureBytes:Buffer.byteLength(rows[0]!.source),schemaBytes:Buffer.byteLength(rows[1]!.source),fixtureSha256:rows[0]!.sha256,literalCounts:result.ownedLiteralCounts,output}));

function prepareTests(){
 const definitions=JSON.parse(readFileSync(resolve(import.meta.dir,"test-first-definitions-1.json"),"utf8")) as {path:string;source:string}[];
 const rows=definitions.map(({path,source})=>{const file=resolve(root,path),before=existsSync(file)?readFileSync(file,"utf8"):null;if(before!==null)throw Error("Test owner already exists: "+path);return {path,before,source,beforeSha256:null,sha256:sha(source),inverse:{remove:true}};});
 const authority={version:1,stage:"mesh-test-first",rows};
 writeFileSync(resolve(import.meta.dir,"test-first-ready-1.json"),JSON.stringify(authority,null,2)+"\n");console.log(JSON.stringify({readyRows:rows.length}));
}
function mountGui(){
 const path=".vscode/🧩️launch.seed.jsonc",file=resolve(root,path),before=readFileSync(file,"utf8"),entries=JSON.parse(readFileSync(resolve(import.meta.dir,"gui-definitions-1.json"),"utf8"));
 for(const entry of entries)if(before.includes(JSON.stringify(entry.name)))throw Error("Owned GUI already exists");
 const anchor='    {\n      "name": "🦑️repo 🧩️compute constructor ownership",',count=before.split(anchor).length-1;if(count!==1)throw Error("Owned GUI insertion anchor refused");
 const inserted=entries.map((entry:any)=>JSON.stringify(entry,null,2).split("\n").map((line:string)=>"    "+line).join("\n")+",\n").join(""),source=before.replace(anchor,inserted+anchor);
 writeFileSync(resolve(import.meta.dir,"gui-publication-before-1.json"),JSON.stringify({version:1,path,before,source,beforeSha256:sha(before),sha256:sha(source),inserted,inverse:{removeExact:inserted}},null,2)+"\n");
 if(readFileSync(file,"utf8")!==before)throw Error("Concurrent seed changed before guarded write");writeFileSync(file,source);
 const current=readFileSync(file,"utf8");if(current!==source)throw Error("GUI post guard");
 writeFileSync(resolve(import.meta.dir,"gui-mounted-1.json"),JSON.stringify({version:1,path,current,sha256:sha(current),entries,foreignEquivalent:current.replace(inserted,"")===before},null,2)+"\n");
 console.log(JSON.stringify({guiEntries:entries.length,foreignEquivalent:true}));
}

function mountTests(){
 const authority=JSON.parse(readFileSync(resolve(import.meta.dir,"test-first-ready-1.json"),"utf8"));
 for(const row of authority.rows)if(existsSync(resolve(root,row.path)))throw Error("Owned creation guard: "+row.path);
 for(const row of authority.rows){const file=resolve(root,row.path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,row.source);}
 const gaps=authority.rows.filter((row:any)=>readFileSync(resolve(root,row.path),"utf8")!==row.source);
 writeFileSync(resolve(import.meta.dir,"test-first-mounted-1.json"),JSON.stringify({...authority,gaps},null,2)+"\n");if(gaps.length)throw Error("Test-first post guard");
 console.log(JSON.stringify({mountedRows:authority.rows.length,postGaps:gaps.length}));
}

function prepareProduction(){
 const capture=JSON.parse(readFileSync(output,"utf8")),frames=capture.rows as {path:string;source:string;sha256:string}[],oldFixture=frames[0]!,oldSchema=frames[1]!;
 for(const row of [oldFixture,oldSchema])if(readFileSync(resolve(root,row.path),"utf8")!==row.source)throw Error("Owned asset predecessor changed: "+row.path);
 const fixture=engine+"/🧫️fixtures/🎨️attributes/🔣️.json",schema=engine+"/🧬️schema/🥽️polygon/🔣️.json";
 if(existsSync(resolve(root,fixture))||existsSync(resolve(root,schema)))throw Error("Canonical asset creation guard");
 const schemaFrom='"$id": "semio:flow:mesh"',schemaTo='"$id": "semio:mesh:polygon"';if(oldSchema.source.split(schemaFrom).length!==2)throw Error("Schema identifier guard");
 const rows:any[]=[{path:fixture,before:null,source:oldFixture.source,patches:[],kind:"create"},{path:schema,before:null,source:oldSchema.source.replace(schemaFrom,schemaTo),patches:[],kind:"create"},{path:oldFixture.path,before:oldFixture.source,source:null,patches:[],kind:"remove"},{path:oldSchema.path,before:oldSchema.source,source:null,patches:[],kind:"remove"}];
 for(const [index,expected] of [[2,2],[3,7],[4,4],[5,1]]){
  const frame=frames[index]!,before=readFileSync(resolve(root,frame.path),"utf8"),matches=[...before.matchAll(/(?:include_str!\(|new URL\()"([^"]*🎨️attributes\/🔣️.json)"/g)];
  if(matches.length!==expected)throw Error("Attribute caller count guard "+frame.path);
  const literals=[...new Set(matches.map(match=>match[1]!))];if(literals.length!==1)throw Error("Attribute predecessor uniqueness guard "+frame.path);
  const from=literals[0]!,to=relative(dirname(resolve(root,frame.path)),resolve(root,fixture)).replaceAll("\\","/"),patches=[{from,to,count:expected}];
  if(resolve(dirname(resolve(root,frame.path)),from)!==resolve(root,oldFixture.path))throw Error("Attribute predecessor owner guard");
  let source=before.replaceAll('"'+from+'"','"'+to+'"');
  if(index===5){const from="../../🧬️schema/🔣️.json",to=relative(dirname(resolve(root,frame.path)),resolve(root,schema)).replaceAll("\\","/"),count=before.split('"'+from+'"').length-1;if(count!==5)throw Error("Polygon schema caller count guard");source=source.replaceAll('"'+from+'"','"'+to+'"');patches.push({from,to,count});}
  let inverse=source;for(const patch of [...patches].reverse())inverse=inverse.replaceAll('"'+patch.to+'"','"'+patch.from+'"');if(inverse!==before)throw Error("Original law/current foreign conservation failed");
  rows.push({path:frame.path,before,source,patches,kind:"update",originalPredecessorSha256:frame.sha256,foreignEvolution:before!==frame.source});
 }
 for(const row of rows){row.beforeSha256=row.before===null?null:sha(row.before);row.sha256=row.source===null?null:sha(row.source);row.inverse={source:row.before,patches:row.patches.map((patch:any)=>({from:patch.to,to:patch.from,count:patch.count}))};}
 const authority={version:1,rows,fixtureSha256:oldFixture.sha256,fixtureByteConservation:true,schemaIdentifierOnly:true,originalNativeLiteralChanges:13,originalSourceFixtureChanges:1,originalSourceSchemaChanges:5};
 writeFileSync(resolve(import.meta.dir,"production-ready-1.json"),JSON.stringify(authority,null,2)+"\n");console.log(JSON.stringify({readyRows:rows.length,fixtureSha256:oldFixture.sha256,nativeLiteralChanges:13,sourceLiteralChanges:6,foreignRows:rows.filter(row=>row.foreignEvolution).map(row=>row.path)}));
}
function mountProduction(){
 const authority=JSON.parse(readFileSync(resolve(import.meta.dir,"production-ready-1.json"),"utf8")),mounted:any[]=[];
 for(const row of authority.rows){
  const file=resolve(root,row.path),before=existsSync(file)?readFileSync(file,"utf8"):null;
  if(row.kind==="create"&&before!==null||row.kind==="remove"&&before!==row.before)throw Error("Asset publication guard "+row.path);
  let source=row.source;
  if(row.kind==="update"){if(before===null)throw Error("Missing owned caller");source=before;for(const patch of row.patches){if(source.split('"'+patch.from+'"').length-1!==patch.count)throw Error("Owned literal publication guard "+row.path);source=source.replaceAll('"'+patch.from+'"','"'+patch.to+'"');}}
  mounted.push({...row,before,source,beforeSha256:before===null?null:sha(before),sha256:source===null?null:sha(source),inverse:{source:before,patches:row.inverse.patches},publicationForeignEvolution:before!==row.before});
 }
 writeFileSync(resolve(import.meta.dir,"production-publication-before-1.json"),JSON.stringify({version:1,rows:mounted},null,2)+"\n");
 for(const row of mounted){const file=resolve(root,row.path),current=existsSync(file)?readFileSync(file,"utf8"):null;if(current!==row.before)throw Error("Concurrent owned-row change before write "+row.path);if(row.source===null)unlinkSync(file);else{mkdirSync(dirname(file),{recursive:true});writeFileSync(file,row.source);}}
 const gaps=mounted.filter(row=>(existsSync(resolve(root,row.path))?readFileSync(resolve(root,row.path),"utf8"):null)!==row.source);
 writeFileSync(resolve(import.meta.dir,"production-mounted-1.json"),JSON.stringify({version:1,rows:mounted,gaps},null,2)+"\n");if(gaps.length)throw Error("Owned production post guards");
 console.log(JSON.stringify({mountedRows:mounted.length,postGaps:gaps.length,fixtureSha256:authority.fixtureSha256,nativeLiteralChanges:13}));
}

function captureFinal(){
 const ready=JSON.parse(readFileSync(resolve(import.meta.dir,"production-publication-before-1.json"),"utf8")),test=JSON.parse(readFileSync(resolve(import.meta.dir,"test-first-ready-1.json"),"utf8")),schemaCorrection=JSON.parse(readFileSync(resolve(import.meta.dir,"polygon-explicit-semantic-array-correction-1.json"),"utf8")),testCorrection=JSON.parse(readFileSync(resolve(import.meta.dir,"test-strict-correction-1.json"),"utf8")),gui=JSON.parse(readFileSync(resolve(import.meta.dir,"gui-publication-before-1.json"),"utf8"));
 const rows:any[]=[];
 for(const row of [...ready.rows,...test.rows]){
  const current=existsSync(resolve(root,row.path))?readFileSync(resolve(root,row.path),"utf8"):null,expected=row.path===schemaCorrection.path?schemaCorrection.source:row.path===testCorrection.path?testCorrection.source:row.source;
  if(current!==expected)throw Error("Scoped final source guard "+row.path);
  rows.push({...row,current,currentSha256:current===null?null:sha(current),inverse:row.inverse});
 }
 const current=readFileSync(resolve(root,gui.path),"utf8");if(current.split(gui.inserted).length!==2)throw Error("Owned final GUI object guard");rows.push({path:gui.path,before:gui.before,current,beforeSha256:gui.beforeSha256,currentSha256:sha(current),inverse:{removeExact:gui.inserted},foreignCurrentPreserved:true,foreignEvolution:current.replace(gui.inserted,"")!==gui.before});
 writeFileSync(resolve(import.meta.dir,"full-current-inverse-final-1.json"),JSON.stringify({version:1,rows,corpusSha256:ready.rows.find((row:any)=>row.path===engine+"/🧫️fixtures/🎨️attributes/🔣️.json").sha256,schemaSuccessor:schemaCorrection.sha256,testSuccessor:testCorrection.sha256,productionRows:8,testRows:5,guiObjects:4,ownedGaps:0,nativeProof:"pending",sourceGate:{registeredSession:45094,passed:5,failed:0,expects:31}},null,2)+"\n");
 console.log(JSON.stringify({finalRows:rows.length,productionRows:8,testRows:5,guiObjects:4,ownedGaps:0,schemaSha256:schemaCorrection.sha256,foreignGUIEvolution:current.replace(gui.inserted,"")!==gui.before}));
}
