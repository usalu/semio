import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,mkdirSync,existsSync,lstatSync,openSync,writeSync,fsyncSync,closeSync} from "node:fs";
import {createRequire} from "node:module";
import {resolve,join,relative,isAbsolute,dirname} from "node:path";
const base=import.meta.dir,ticket=resolve(base,"../.."),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
const [command,epoch]=process.argv.slice(2);assert.equal(command,"source-controls");assert.ok(["red-1","green-2"].includes(epoch));
const output=join(ticket,"🗑️generated/current-engine-layout",epoch),receiptPath=join(output,"admission.json"),proposalPath=join(base,"🧩️renderer-slot-full-pairs-1.json"),guiPath=join(base,"gui-source-controls-1.json");
let cancelled=false;for(const signal of ["SIGINT","SIGTERM"] as const)process.on(signal,()=>{cancelled=true;});
const check=()=>assert.equal(cancelled,false,"Source controls cancelled");
const progress=(phase:string,details:object={})=>console.log("[DEBUG] "+JSON.stringify({phase,...details}));
const raw=(path:string)=>{check();const stat=lstatSync(path);assert.ok(stat.isFile()&&!stat.isSymbolicLink(),"Regular authority required: "+path);return readFileSync(path);};
const text=(path:string)=>{const body=raw(path),source=body.toString("utf8");assert.ok(body.equals(Buffer.from(source)),"Exact UTF-8 authority required");return source;};
const write=(path:string,body:string)=>{check();const rel=relative(ticket,path);assert.ok(rel&&!rel.startsWith("..")&&!isAbsolute(rel));mkdirSync(dirname(path),{recursive:true});const fd=openSync(path,"wx");try{writeSync(fd,body);fsyncSync(fd);}finally{closeSync(fd);}};
type Schema={type?:string;const?:unknown;enum?:unknown[];required?:string[];additionalProperties?:boolean;properties?:Record<string,Schema>;items?:Schema;minItems?:number;maxItems?:number;minLength?:number;maxLength?:number};
/** 🧬️ Interprets the closed source-control law independently of its installed schema oracle. */
function ownSchema(schema:Schema,value:any):boolean{
 if("const" in schema&&!Bun.deepEquals(schema.const,value))return false;
 if(schema.enum&&!schema.enum.some(item=>Bun.deepEquals(item,value)))return false;
 if(schema.type==="object"){if(value===null||typeof value!=="object"||Array.isArray(value))return false;if(schema.required?.some(key=>!Object.hasOwn(value,key)))return false;for(const [key,item]of Object.entries(value)){const child=schema.properties?.[key];if(!child){if(schema.additionalProperties===false)return false;}else if(!ownSchema(child,item))return false;}}
 if(schema.type==="array"){if(!Array.isArray(value)||schema.minItems!==undefined&&value.length<schema.minItems||schema.maxItems!==undefined&&value.length>schema.maxItems)return false;if(schema.items&&value.some(item=>!ownSchema(schema.items!,item)))return false;}
 if(schema.type==="string"){if(typeof value!=="string")return false;const length=[...value].length;if(schema.minLength!==undefined&&length<schema.minLength||schema.maxLength!==undefined&&length>schema.maxLength)return false;}
 if(schema.type==="boolean"&&typeof value!=="boolean")return false;
 return true;
}
/** 🧪️ Loads independent test-only parser and schema implementations behind one isolated oracle boundary. */
function oracles(schema:Schema){const JSON5=require("json5"),Ajv=require("ajv/dist/2020.js").default;return{parse:(source:string)=>JSON5.parse(source),schema:new Ajv({strict:true,allErrors:true}).compile(schema)};}
/** 🔒️ Admits exactly one measured descriptor while conserving current source, ownership and the original whole law. */
function admit(candidate:any,context:any,parse:(source:string)=>any):boolean{
 try{
  assert.equal(candidate.version,1);assert.equal(candidate.ready,false);assert.equal(candidate.sourceWritesOutsideTicket,false);assert.equal(candidate.nativeExecuted,false);assert.equal(candidate.publicationReady,false);assert.equal(candidate.pairs.length,1);
  const pair=candidate.pairs[0];assert.equal(pair.path,context.path);assert.equal(pair.before,context.before);assert.equal(pair.beforeHash,context.beforeHash);assert.equal(sha(pair.before),pair.beforeHash);assert.equal(sha(pair.after),pair.afterHash);
  assert.deepEqual(pair.forward,{start:0,delete:pair.before,insert:pair.after});assert.deepEqual(pair.inverse,{start:0,delete:pair.after,insert:pair.before});assert.deepEqual(candidate.bindings,context.bindings);
  assert.equal(sha(context.testBody),context.testHash);assert.ok(context.testBody.includes("fn engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack()"));assert.ok(context.testBody.includes("semio_framework_async::assert_fixed_slot_tables("));
  const original=parse(pair.before),after=parse(pair.after),expected=structuredClone(original),rows=expected.tables.filter((row:any)=>row.owner==="engine_canvas::EngineSurfaceRegistry");
  assert.equal(rows.length,1);assert.equal(rows[0].elementSizeBytes,86544);assert.equal(rows[0].capacity,256);assert.equal(rows[0].ownerSizeBytes,32);assert.equal(original.boundedThreadStackBytes,1048576);assert.equal(original.workerPoolStackBytes,2097152);assert.equal(original.conversionThresholdBytes,65536);
  rows[0].elementSizeBytes=90264;assert.deepEqual(after,expected);return true;
 }catch{return false;}
}
const controlsBody=text(join(base,"🧫️fixtures/🧪️source-controls.json")),schemaBody=text(join(base,"🧬️schema/🧪️source-controls.json")),controls=JSON.parse(controlsBody),schema=JSON.parse(schemaBody),reference=oracles(schema);
assert.ok(ownSchema(schema,controls)&&reference.schema(controls));assert.deepEqual(controls,reference.parse(controlsBody));
const proposalBody=text(proposalPath),proposal=JSON.parse(proposalBody);assert.deepEqual(proposal,reference.parse(proposalBody));assert.equal(proposal.ready,false);assert.equal(proposal.pairs.length,1);assert.equal(proposal.publicationReady,false);
assert.ok(!existsSync(receiptPath),"Fresh authority cannot overwrite an existing receipt");mkdirSync(output,{recursive:true});
const bindings=new Map<string,string>(),authorities:any[]=[];
const bind=(path:string,expected?:string,retain=false)=>{const body=raw(path),hash=sha(body);if(expected)assert.equal(hash,expected,"Current full authority advanced: "+path);bindings.set(path,hash);if(retain)authorities.push({path,sha256:hash,encoding:"base64",source:body.toString("base64")});return{path,sha256:hash};};
bind(proposalPath,sha(proposalBody));bind(import.meta.path,undefined,true);bind(guiPath);
for(const path of ["🧫️fixtures/🧪️source-controls.json","🧬️schema/🧪️source-controls.json","🧪️nx/project.json","🧪️nx/nx.json","🧪️nx/package.json"])bind(join(base,path));
for(const row of proposal.bindings)bind(row.path,row.sha256,row.path.endsWith(".rs")||row.path.endsWith("Cargo.toml"));
const pair=proposal.pairs[0],sourcePath=resolve(root,pair.path);assert.equal(isAbsolute(pair.path),false);assert.ok(!relative(root,sourcePath).startsWith(".."));assert.equal(text(sourcePath),pair.before);assert.equal(sha(pair.before),pair.beforeHash);assert.equal(sha(pair.after),pair.afterHash);
const testPath=proposal.bindings.find((row:any)=>row.path.includes("/🧪️tests/🧩️wgpu-engine-surfaces/")).path,testBody=text(testPath),testHash=sha(testBody);
const context={path:pair.path,before:pair.before,beforeHash:pair.beforeHash,after:pair.after,afterHash:pair.afterHash,bindings:proposal.bindings,testBody,testHash};
const guiRows=JSON.parse(text(guiPath)),owned=guiRows.find((row:any)=>row.name==="📥️engine-slot-source-controls-"+epoch+"🧪️");assert.ok(owned);
for(const [key,value]of Object.entries(owned.env))assert.equal(process.env[key],String(value).replaceAll("${workspaceFolder}",root),"Exact registered environment: "+key);
const registrations=[];
for(const path of [join(root,".vscode/launch.json"),join(root,".vscode/🧩️launch.seed.jsonc")]){const body=text(path),document=reference.parse(body),rows=document.configurations.filter((row:any)=>row.name===owned.name);assert.equal(rows.length,1);assert.deepEqual(rows[0],owned);write(join(output,registrations.length?"seed-before.jsonc":"launch-before.jsonc"),body);registrations.push({path,sourceHash:sha(body),ownedRow:rows[0]});}
progress("full-source-guards",{bindings:bindings.size,sourceWritesOutsideTicket:false});
write(join(output,"started.json"),JSON.stringify({at:new Date().toISOString(),proposalPath,driverHash:sha(text(import.meta.path)),sourceWritesOutsideTicket:false,nativeExecuted:false})+"\n");
const outcomes=[];
for(const row of controls.cases){
 check();const candidate=structuredClone(proposal),mutatedContext=structuredClone(context),endpoint=JSON.parse(candidate.pairs[0].after),target=endpoint.tables.find((item:any)=>item.owner==="engine_canvas::EngineSurfaceRegistry");
 switch(row.mutation){
  case "stack":endpoint.boundedThreadStackBytes*=2;break;
  case "worker":endpoint.workerPoolStackBytes*=2;break;
  case "threshold":endpoint.conversionThresholdBytes=1;break;
  case "capacity":target.capacity++;break;
  case "owner-bytes":target.ownerSizeBytes++;break;
  case "owner-name":target.owner="foreign";break;
  case "other-row":endpoint.tables[0].elementSizeBytes++;break;
  case "element":target.elementSizeBytes++;break;
  case "extra-field":endpoint.foreign=true;break;
  case "missing-row":endpoint.tables.pop();break;
  case "before-hash":candidate.pairs[0].beforeHash="0".repeat(64);break;
  case "after-hash":candidate.pairs[0].afterHash="0".repeat(64);break;
  case "inverse":candidate.pairs[0].inverse.insert="foreign";break;
  case "path":candidate.pairs[0].path="foreign.json";break;
  case "binding":candidate.bindings[0].sha256="0".repeat(64);break;
  case "missing-binding":candidate.bindings.pop();break;
  case "assertion":mutatedContext.testBody=testBody.replace("assert_fixed_slot_tables(","foreign_guard(");assert.notEqual(mutatedContext.testBody,testBody);break;
 }
 if(["stack","worker","threshold","capacity","owner-bytes","owner-name","other-row","element","extra-field","missing-row"].includes(row.mutation)){candidate.pairs[0].after=JSON.stringify(endpoint,null,2)+"\n";candidate.pairs[0].afterHash=sha(candidate.pairs[0].after);candidate.pairs[0].forward.insert=candidate.pairs[0].after;candidate.pairs[0].inverse.delete=candidate.pairs[0].after;}
 const own=admit(candidate,mutatedContext,JSON.parse),oracle=admit(candidate,mutatedContext,reference.parse);outcomes.push({...row,own,oracle,agrees:own===row.accepted&&oracle===row.accepted});
}
const schemaOutcomes=[];
for(const [id,value,expected]of [["schema-normative",controls,true],["schema-extra",{...controls,foreign:0},false],["schema-incomplete",{...controls,cases:controls.cases.slice(1)},false],["schema-unknown",{...controls,cases:controls.cases.map((row:any,index:number)=>index?row:{...row,mutation:"foreign"})},false],["schema-wrong-type",{...controls,cases:controls.cases.map((row:any,index:number)=>index?row:{...row,accepted:"true"})},false]] as const){const own=ownSchema(schema,value),oracle=Boolean(reference.schema(value));schemaOutcomes.push({id,own,oracle,expected,agrees:own===expected&&oracle===expected});}
const errors=[...outcomes,...schemaOutcomes].filter(row=>!row.agrees);
for(const [path,hash]of bindings){check();assert.equal(sha(raw(path)),hash,"Post current authority advanced: "+path);}
assert.equal(text(sourcePath),pair.before);assert.equal(text(testPath),testBody);
const result={epoch,ready:errors.length===0,outcomes,schemaOutcomes,errors,registrations,sourceWritesOutsideTicket:false,nativeExecuted:false,liveIdentity:false,atomicity:false,publicationReady:false};write(join(output,"controls.json"),JSON.stringify(result,null,2)+"\n");
progress("closed-source-controls",{cases:outcomes.length,schemaCases:schemaOutcomes.length,errors:errors.length});
if(errors.length)process.exit(1);
bind(require.resolve("json5/package.json"));bind(require.resolve("ajv/package.json"));bind(join(output,"controls.json"));
const receipt={...result,at:new Date().toISOString(),ready:true,pairs:proposal.pairs,bindings:[...bindings].map(([path,sha256])=>({path,sha256})),authorities,proposal:{path:proposalPath,sha256:sha(proposalBody),source:proposalBody},controls:{normativeAdversarial:outcomes,closedSchema:schemaOutcomes,originalAssertionBodyConserved:true,ownParser:"JSON.parse",thirdPartyParser:"JSON5",thirdPartySchema:"AJV"},scope:"Test-only current Renderer descriptor source admission for held native verification; original whole bounded constructor still required"};
write(receiptPath,JSON.stringify(receipt,null,2)+"\n");progress("sealed",{receiptPath,bindings:bindings.size,pairs:1});
