import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,mkdirSync,existsSync,lstatSync,realpathSync,openSync,writeSync,fsyncSync,closeSync} from "node:fs";
import {createRequire} from "node:module";
import {resolve,join,relative,isAbsolute,dirname} from "node:path";
const base=import.meta.dir,ticket=resolve(base,"../.."),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),output=join(ticket,"🗑️generated/current-native-world/source-model-5"),proposalPath=join(base,"🧩️world-full-pairs-5.json"),guiPath=join(base,"gui-source-model-5.json"),receiptPath=join(output,"admission.json"),sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
let cancelled=false,child:ReturnType<typeof Bun.spawn>|undefined;
for(const signal of ["SIGINT","SIGTERM"] as const)process.on(signal,()=>{cancelled=true;child?.kill();});
const check=()=>assert.equal(cancelled,false,"Source model was cancelled");
const within=(path:string)=>{const rel=relative(ticket,path);assert.ok(rel&&!rel.startsWith("..")&&!isAbsolute(rel),"Write must remain inside the ticket");return path;};
const write=(path:string,source:string)=>{check();within(path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,source);};
const raw=(path:string)=>{check();const stat=lstatSync(path);assert.ok(stat.isFile()&&!stat.isSymbolicLink(),"Authority must be a regular file: "+path);return readFileSync(path,"utf8");};
const current=(path:string)=>existsSync(path)?raw(path):null;
const bindings=new Map<string,string>();
const bind=(path:string,source?:string)=>{const body=source??raw(path),hash=sha(body);assert.equal(raw(path),body,"Binding advanced: "+path);bindings.set(path,hash);return {path,sha256:hash};};
const progress=(phase:string,details:object={})=>console.log("[DEBUG] "+JSON.stringify({phase,...details}));
type Schema={type?:string;const?:unknown;enum?:unknown[];required?:string[];additionalProperties?:boolean;properties?:Record<string,Schema>;items?:Schema;minItems?:number;maxItems?:number;minLength?:number;maxLength?:number};
/** 🧬️ Closed review-schema interpretation, independently compared with the installed AJV oracle. */
function ownSchema(schema:Schema,value:unknown):boolean{
 if("const" in schema&&!Bun.deepEquals(schema.const,value))return false;
 if(schema.enum&&!schema.enum.some(item=>Bun.deepEquals(item,value)))return false;
 if(schema.type==="object"){
  if(typeof value!=="object"||value===null||Array.isArray(value))return false;
  const record=value as Record<string,unknown>;
  if(schema.required?.some(key=>!Object.hasOwn(record,key)))return false;
  for(const [key,item]of Object.entries(record)){const child=schema.properties?.[key];if(!child){if(schema.additionalProperties===false)return false;}else if(!ownSchema(child,item))return false;}
 }
 if(schema.type==="array"){if(!Array.isArray(value))return false;if(schema.minItems!==undefined&&value.length<schema.minItems||schema.maxItems!==undefined&&value.length>schema.maxItems)return false;if(schema.items&&value.some(item=>!ownSchema(schema.items!,item)))return false;}
 if(schema.type==="string"){if(typeof value!=="string")return false;const length=[...value].length;if(schema.minLength!==undefined&&length<schema.minLength||schema.maxLength!==undefined&&length>schema.maxLength)return false;}
 if(schema.type==="boolean"&&typeof value!=="boolean")return false;
 return true;
}
/** 🧪️ One isolated test-oracle boundary, with no exported third-party runtime API. */
function oracle(schema:Schema){const Ajv=require("ajv/dist/2020.js").default;return new Ajv({strict:true,allErrors:true}).compile(schema);}
function schemaCases(schema:Schema,cases:{id:string;value:unknown;expected:boolean}[]){
 const reference=oracle(schema),outcomes=[];
 for(const entry of cases){check();const own=ownSchema(schema,entry.value),external=Boolean(reference(entry.value));assert.equal(own,external,entry.id+" independent schema output");assert.equal(own,entry.expected,entry.id+" normative output");outcomes.push({id:entry.id,own,oracle:external,expected:entry.expected});}
 return outcomes;
}
/** 🦀️ Parses full current and staged Rust bodies through rustfmt stdin without a compiler or source write. */
async function rustSyntax(id:string,source:string,executable:string){
 check();const stdout=join(output,id+".stdout.rs"),stderr=join(output,id+".stderr.log");
 child=Bun.spawn([executable,"--edition","2024","--emit","stdout","--config","skip_children=true"],{cwd:ticket,stdin:Buffer.from(source),stdout:Bun.file(within(stdout)),stderr:Bun.file(within(stderr)),env:{...process.env}});
 const active=child,timer=setTimeout(()=>active.kill(),60000);const code=await active.exited;clearTimeout(timer);child=undefined;check();
 const observation={id,argv:["rustfmt","--edition","2024","--emit","stdout","--config","skip_children=true"],code,stdout,stderr,sourceHash:sha(source),diagnostics:raw(stderr)};
 write(join(output,id+".json"),JSON.stringify(observation,null,2)+"\n");assert.equal(code,0,"Rust parser refused "+id);progress("rust-parser",{id,code});return observation;
}
/** 🔒️ Seals exact current-origin source model pairs and controls for separately owned native execution. */
async function sourceModel(){
 mkdirSync(output,{recursive:true});assert.ok(!existsSync(receiptPath),"Fresh receipt must not overwrite an existing authority");progress("started",{proposalPath,receiptPath});
 write(join(output,"started.json"),JSON.stringify({at:new Date().toISOString(),proposalPath,driver:base,sourceWritesOutsideTicket:false,nativeExecuted:false}));
 const proposalBody=raw(proposalPath),proposal=JSON.parse(proposalBody);assert.equal(proposal.schemaVersion,1);assert.equal(proposal.rows.length,5);assert.equal(proposal.authorities.length,16);assert.equal(proposal.sourceWrites,0);assert.equal(proposal.compilerRun,false);assert.equal(proposal.runtimeVerified,false);assert.equal(proposal.publicationReady,false);bind(proposalPath,proposalBody);
 bind(join(base,"📜️script.ts"));bind(guiPath);
 for(const name of ["project.json","nx.json","package.json"])bind(join(base,"🧪️nx",name));
 const jsonc=require("jsonc-parser"),supplied=JSON.parse(raw(guiPath));assert.equal(supplied.length,1);
 const registrations=[];
 for(const path of [join(root,".vscode/launch.json"),join(root,".vscode/🧩️launch.seed.jsonc")]){
  const body=raw(path),errors:any[]=[],document=jsonc.parse(body,errors);assert.deepEqual(errors,[]);const rows=document.configurations.filter((row:any)=>row.name===supplied[0].name);assert.equal(rows.length,1,"Exact GUI registration missing: "+path);assert.deepEqual(rows[0],supplied[0]);write(join(output,registrations.length?".seed-before.jsonc":".launch-before.jsonc"),body);registrations.push({path,ownedRow:rows[0],sourceHash:sha(body)});
 }
 const pairs=[];
 for(const row of proposal.rows){
  check();assert.equal(isAbsolute(row.path),false);const sourceRelative=relative(root,resolve(root,row.path));assert.ok(sourceRelative&&!sourceRelative.startsWith("..")&&!isAbsolute(sourceRelative));assert.equal(current(join(root,row.path)),row.before,"Current preimage mismatch: "+row.path);assert.equal(row.before===null?null:sha(row.before),row.beforeHash);assert.equal(sha(row.after),row.afterHash);
  pairs.push({path:row.path,before:row.before,after:row.after,beforeHash:row.beforeHash,afterHash:row.afterHash,forward:{start:0,delete:row.before,insert:row.after},inverse:{start:0,delete:row.after,insert:row.before}});
 }
 for(const authority of proposal.authorities){assert.equal(sha(authority.source),authority.hash);bind(isAbsolute(authority.path)?authority.path:join(root,authority.path),authority.source);}
 progress("full-source-guards",{pairs:pairs.length,authorities:proposal.authorities.length});
 const test=pairs.find(row=>row.path.includes("world/🧪️tests/🔬️unit/"))!;assert.ok(test);
 const marker="/// 🔒️ Live commands retain admitted start targets",index=test.after.indexOf(marker);assert.ok(index>0);
 assert.equal(test.before!.split("        selected,").length-1,2);
 const expected=test.before!.replaceAll("        selected,","        selected: Some(selected),\n        captured: None,").replace("grab.selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);","grab.selected = Some(Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]));")+"\n";
 assert.equal(test.after.slice(0,index),expected,"Original unit body conservation with only declared constructor shape edits");
 const fixturePair=pairs.find(row=>row.path.includes("/🧫️fixtures/🎯️captured-gesture-authority/"))!,schemaPair=pairs.find(row=>row.path.includes("/🧬️schema/🎯️captured-gesture-authority/"))!;
 const law=JSON.parse(fixturePair.after),lawSchema=JSON.parse(schemaPair.after),captureSchemaAuthority=proposal.authorities.find((row:any)=>row.path.endsWith("/🧬️schema/🔣️capture-authority.json")),captureAuthority=proposal.authorities.find((row:any)=>row.path.endsWith("/🧪️capture-law-bindings.json"));
 assert.ok(captureSchemaAuthority&&captureAuthority);const captureSchema=JSON.parse(captureSchemaAuthority.source),capture=JSON.parse(captureAuthority.source);
 const copy=<T>(value:T):T=>structuredClone(value);
 const changedCase=copy(law);changedCase.cases[0].scenario="unowned";
 const wrongBoolean=copy(law);wrongBoolean.cases[0].accepted="true";
 const outcomes=[
 ...schemaCases(lawSchema,[{id:"neutral-owner-law",value:law,expected:true},{id:"law-extra-field",value:{...law,unknown:0},expected:false},{id:"law-missing-case",value:{...law,cases:law.cases.slice(1)},expected:false},{id:"law-unknown-scenario",value:changedCase,expected:false},{id:"law-wrong-acceptance-type",value:wrongBoolean,expected:false},{id:"law-wrong-turn-control",value:{...law,maximumTurns:0},expected:false}]),
 ...schemaCases(captureSchema,[{id:"original-captured-fixture-binding",value:capture,expected:true},{id:"capture-extra-field",value:{...capture,unknown:0},expected:false},{id:"capture-item-plus-one",value:{...capture,targets:Array(246).fill("x")},expected:false},{id:"capture-byte-control-plus-one",value:{...capture,maximumBytes:16385},expected:false},{id:"capture-invalid-owner",value:{...capture,surfaceId:"x".repeat(257)},expected:false},{id:"capture-wrong-target-type",value:{...capture,targets:[0]},expected:false}])
 ];
 const original=JSON.parse(proposal.authorities.find((row:any)=>row.path.includes("/🧫️fixtures/🎯️component-selection-merges/")).source),expectedTargets=original.gumball.ids.map((id:number)=>original.targets[id]);assert.deepEqual(capture.targets,expectedTargets);assert.equal(capture.mode,original.gumball.mode);assert.ok(capture.targets.reduce((sum:number,id:string)=>sum+Buffer.byteLength(id),0)<=16384);
 progress("closed-neutral-oracle",{outcomes:outcomes.length});
 const rustfmt=Bun.which("rustfmt");assert.ok(rustfmt,"Current rustfmt parser is required");const syntax=[];
 for(const [index,row]of pairs.entries())if(row.path.endsWith(".rs")){if(row.before!==null)syntax.push(await rustSyntax("rust-"+index+"-before",row.before,rustfmt));syntax.push(await rustSyntax("rust-"+index+"-after",row.after,rustfmt));}
 bind(realpathSync(require.resolve("ajv/dist/2020.js")));bind(require.resolve("ajv/package.json"));bind(require.resolve("jsonc-parser/package.json"));
 for(const [path,hash]of bindings){assert.equal(sha(raw(path)),hash,"Post binding advanced: "+path);}
 for(const row of pairs)assert.equal(current(join(root,row.path)),row.before,"Post preimage advanced: "+row.path);
 const receipt={schemaVersion:1,at:new Date().toISOString(),ready:true,pairs,bindings:[...bindings].map(([path,sha256])=>({path,sha256})),proposal:{path:proposalPath,sha256:sha(proposalBody),source:proposalBody},driver:{path:join(base,"📜️script.ts"),sha256:sha(raw(join(base,"📜️script.ts")))},registrations,controls:{originalUnitBodyConserved:true,intendedConstructorChanges:3,closedNeutralOracle:outcomes,rustParser:syntax,syntaxDiagnostics:0,maximumTargetBytes:16384,captureBytesPerFuel:1},sourceWritesOutsideTicket:false,nativeExecuted:false,liveIdentity:false,atomicity:false,publicationReady:false};
 const fd=openSync(within(receiptPath),"wx");try{writeSync(fd,JSON.stringify(receipt,null,2)+"\n");fsyncSync(fd);}finally{closeSync(fd);}
 progress("sealed",{receiptPath,pairs:pairs.length,bindings:bindings.size,syntax:syntax.length,sourceWritesOutsideTicket:false,nativeExecuted:false});
}
const [command,epoch]=process.argv.slice(2);assert.equal(command,"source-model");assert.equal(epoch,"5");await sourceModel();
