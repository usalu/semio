import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,mkdirSync,existsSync,lstatSync,openSync,writeSync,fsyncSync,closeSync} from "node:fs";
import {createRequire} from "node:module";
import {resolve,join,relative,isAbsolute,dirname} from "node:path";
const base=import.meta.dir,ticket=resolve(base,"../.."),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
const [command,epoch]=process.argv.slice(2);assert.equal(command,"source-controls");assert.ok(["green-1"].includes(epoch));
const output=join(ticket,"🗑️generated/current-engine-layout-12",epoch),receiptPath=join(output,"admission.json"),proposalPath=join(base,"🧩️renderer-slot-full-pairs-12.json"),guiPath=join(base,"gui-source-controls-12.json");
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
  assert.equal(candidate.version,12);assert.equal(candidate.ready,false);assert.equal(candidate.sourceWritesOutsideTicket,false);assert.equal(candidate.nativeExecuted,false);assert.equal(candidate.publicationReady,false);assert.equal(candidate.pairs.length,1);
  const pair=candidate.pairs[0];assert.equal(pair.path,context.path);assert.equal(pair.before,context.before);assert.equal(pair.beforeHash,context.beforeHash);assert.equal(sha(pair.before),pair.beforeHash);assert.equal(sha(pair.after),pair.afterHash);
  assert.deepEqual(pair.forward,{start:0,delete:pair.before,insert:pair.after});assert.deepEqual(pair.inverse,{start:0,delete:pair.after,insert:pair.before});assert.deepEqual(candidate.bindings,context.bindings);assert.deepEqual(candidate.coupledWorld,context.coupledWorld);assert.deepEqual(candidate.storeTransition,context.storeTransition);assert.deepEqual(candidate.retainedCloneTransition,context.retainedCloneTransition);assert.deepEqual(candidate.currentAuthorityTransitions,context.currentAuthorityTransitions);assert.deepEqual(candidate.currentNativeMeasurement,context.currentNativeMeasurement);
  for(const law of context.laws){assert.equal(sha(law.body),law.sha256);assert.ok(law.body.includes("fn "+law.name+"()"));assert.ok(law.body.includes("semio_framework_async::assert_fixed_slot_tables("));}
  const original=parse(pair.before),after=parse(pair.after),expected=structuredClone(original);
  for(const measurement of context.measurements){const rows=expected.tables.filter((row:any)=>row.owner===measurement.owner);assert.equal(rows.length,1);assert.equal(rows[0].elementSizeBytes,measurement.beforeBytes);assert.equal(rows[0].capacity,measurement.capacity);assert.equal(rows[0].ownerSizeBytes,measurement.ownerBytes);rows[0].elementSizeBytes=measurement.afterBytes;}
  assert.equal(original.boundedThreadStackBytes,1048576);assert.equal(original.workerPoolStackBytes,2097152);assert.equal(original.conversionThresholdBytes,65536);assert.deepEqual(candidate.measurements,context.measurements);assert.deepEqual(after,expected);return true;
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
const lawSpecs=[["/🧪️tests/🧩️wgpu-engine-surfaces/","engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack"],["/🧪️tests/🔬️wgpu-admitted-surface-map/","admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack"],["/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/","kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack"]];
const laws=lawSpecs.map(([marker,name])=>{const path=proposal.bindings.find((row:any)=>row.path.includes(marker)).path,body=text(path);assert.ok(body.includes("fn "+name+"()"));return{path,name,body,sha256:sha(body)};});
const coupledWorld=proposal.coupledWorld,worldReceipt=JSON.parse(text(coupledWorld.receiptPath)),worldPair=worldReceipt.pairs.find((row:any)=>row.path===coupledWorld.path);
assert.equal(sha(text(coupledWorld.receiptPath)),coupledWorld.receiptHash);assert.equal(worldPair.beforeHash,coupledWorld.beforeHash);assert.equal(worldPair.afterHash,coupledWorld.afterHash);assert.equal(text(resolve(root,coupledWorld.path)),worldPair.before);assert.equal(JSON.parse(text(coupledWorld.declaredModelPath)).body,worldPair.after);assert.equal(sha(worldPair.after),worldPair.afterHash);
const measuredWorld=text(coupledWorld.measuredCapturedModelPath);assert.equal(sha(measuredWorld),coupledWorld.measuredCapturedModelHash);const measuredReceipt=JSON.parse(text(coupledWorld.measuredReceiptPath));assert.equal(sha(text(coupledWorld.measuredReceiptPath)),coupledWorld.measuredReceiptHash);assert.equal(measuredReceipt.pairs.find((row:any)=>row.path===coupledWorld.path).after,measuredWorld);let composed=measuredWorld;assert.equal(coupledWorld.methodOnlyRecipe.append,"");assert.equal(coupledWorld.methodOnlyRecipe.replacements.length,1);for(const r of coupledWorld.methodOnlyRecipe.replacements){assert.equal(composed.split(r.before).length-1,1);assert.ok(r.before.includes("then_some(a.add(b).scale(0.5))"));assert.ok(r.after.includes("ray_segment_closest_pair(self.origin, self.direction, a, b)"));composed=composed.replace(r.before,r.after);}assert.equal(composed,worldPair.after);
for(const transition of [proposal.storeTransition,proposal.retainedCloneTransition,...proposal.currentAuthorityTransitions]){assert.equal(transition.physicalLayoutAccepted,false);assert.equal(sha(text(transition.capturedBeforePath)),transition.beforeHash);let current=text(transition.capturedBeforePath);assert.ok(transition.statementReplacements.length>0);for(const r of transition.statementReplacements){assert.equal(current.split(r.before).length-1,1);current=current.replace(r.before,()=>r.after);}assert.equal(sha(transition.before),sha(text(transition.capturedBeforePath)));assert.equal(sha(transition.after),sha(current));assert.equal(sha(current),sha(text(transition.path)));assert.equal(sha(current),transition.afterHash);}

const store=proposal.storeTransition;assert.deepEqual(store.declarationChanges,[{owner:"EditReplay",field:"operation_progress",type:"bool"}]);const storeBlock=(body:string)=>{const start=body.indexOf("pub struct EditReplay<P, Mutation>");assert.ok(start>=0);const end=body.indexOf("\n}",start);assert.ok(end>start);return body.slice(start,end);};assert.equal(storeBlock(store.before).includes("operation_progress: bool"),false);assert.equal(storeBlock(store.after).includes("operation_progress: bool"),true);
const declared=proposal.retainedCloneTransition;assert.deepEqual(declared.declarationChanges,[{owner:"RetainedCloneGrant",field:"maximum_release_bytes",type:"usize"},{owner:"RetainedCloneProgress",field:"released_bytes",type:"usize"}]);for(const field of declared.declarationChanges){const block=(body:string)=>{const start=body.indexOf("pub struct "+field.owner+" {");assert.ok(start>=0);const end=body.indexOf("\n}",start);assert.ok(end>start);return body.slice(start,end);};assert.equal(block(text(declared.capturedBeforePath)).includes("pub "+field.field+": "+field.type),false);assert.equal(block(text(declared.path)).includes("pub "+field.field+": "+field.type),true);}for(const transition of proposal.currentAuthorityTransitions)assert.deepEqual(transition.declarationChanges,[]);
const diagnostic=join(ticket,"🗑️generated/current-product-layout-diagnostic/one"),cases=[1,2,3].map(index=>JSON.parse(text(join(diagnostic,"case-"+index+".json")))),diagnosticLog=text(join(diagnostic,"gui-run.log"));
for(const [index,name,exitCode]of [[0,"engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack",1],[1,"kernel_runtime::kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack",0],[2,"scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack",1]] as const){assert.deepEqual(cases[index].args,["--exact",name,"--nocapture"]);assert.equal(cases[index].exitCode,exitCode);}
assert.ok(diagnosticLog.includes('element_bytes: 90264, owner_bytes: 32'));assert.ok(diagnosticLog.includes('element_bytes: 28440, owner_bytes: 3136'));
const currentDiagnostic=join(ticket,"🗑️generated/current-product-layout-diagnostic/two"),native=proposal.currentNativeMeasurement,frames=JSON.parse(text(native.path)).frames;assert.equal(sha(text(native.path)),native.sha256);assert.equal(native.physicalLayoutAccepted,false);assert.deepEqual(frames.slice(1).map((r:any)=>r.exitCode),[1,1,0]);const currentLog=text(join(currentDiagnostic,"gui-run.log"));assert.ok(currentLog.includes('element_bytes: 90224, owner_bytes: 32'));assert.ok(currentLog.includes('element_bytes: 25672, owner_bytes: 32'));assert.deepEqual([native.engineElementBytes,native.kernelElementBytes,native.sceneElementBytes],[90224,25672,28440]);
const context={currentNativeMeasurement:proposal.currentNativeMeasurement,currentAuthorityTransitions:proposal.currentAuthorityTransitions,retainedCloneTransition:proposal.retainedCloneTransition,storeTransition:proposal.storeTransition,path:pair.path,before:pair.before,beforeHash:pair.beforeHash,after:pair.after,afterHash:pair.afterHash,bindings:proposal.bindings,laws,coupledWorld,measurements:proposal.measurements};
const guiRows=JSON.parse(text(guiPath)),owned=guiRows.find((row:any)=>row.name==="📥️renderer-slot-successor-12-controls-"+epoch+"🧪️");assert.ok(owned);
for(const [key,value]of Object.entries(owned.env))assert.equal(process.env[key],String(value).replaceAll("${workspaceFolder}",root),"Exact registered environment: "+key);
const registrations=[];
for(const path of [join(root,".vscode/launch.json"),join(root,".vscode/🧩️launch.seed.jsonc")]){const body=text(path),document=reference.parse(body),rows=document.configurations.filter((row:any)=>row.name===owned.name);assert.equal(rows.length,1);assert.deepEqual(rows[0],owned);write(join(output,registrations.length?"seed-before.jsonc":"launch-before.jsonc"),body);registrations.push({path,sourceHash:sha(body),ownedRow:rows[0]});}
progress("full-source-guards",{bindings:bindings.size,sourceWritesOutsideTicket:false});
write(join(output,"started.json"),JSON.stringify({at:new Date().toISOString(),proposalPath,driverHash:sha(text(import.meta.path)),sourceWritesOutsideTicket:false,nativeExecuted:false})+"\n");
const outcomes=[];
for(const row of controls.cases){
 check();const candidate=structuredClone(proposal),mutatedContext=structuredClone(context),endpoint=JSON.parse(candidate.pairs[0].after),target=endpoint.tables.find((item:any)=>item.owner==="engine_canvas::EngineSurfaceRegistry"),scene=endpoint.tables.find((item:any)=>item.owner==="scenes::AdmittedSurfaceMap<World3dState>");
 switch(row.mutation){
  case "stack":endpoint.boundedThreadStackBytes*=2;break;
  case "worker":endpoint.workerPoolStackBytes*=2;break;
  case "threshold":endpoint.conversionThresholdBytes=1;break;
  case "capacity":target.capacity++;break;
  case "owner-bytes":target.ownerSizeBytes++;break;
  case "owner-name":target.owner="foreign";break;
  case "other-row":endpoint.tables[2].elementSizeBytes++;break;
  case "element":target.elementSizeBytes++;break;
  case "scene-element":scene.elementSizeBytes++;break;
  case "scene-capacity":scene.capacity++;break;
  case "scene-owner-bytes":scene.ownerSizeBytes++;break;
  case "scene-owner-name":scene.owner="foreign";break;
  case "missing-scene-correction":scene.elementSizeBytes=28400;break;
  case "missing-engine-correction":target.elementSizeBytes=86544;break;
  case "extra-field":endpoint.foreign=true;break;
  case "missing-row":endpoint.tables.pop();break;
  case "before-hash":candidate.pairs[0].beforeHash="0".repeat(64);break;
  case "after-hash":candidate.pairs[0].afterHash="0".repeat(64);break;
  case "inverse":candidate.pairs[0].inverse.insert="foreign";break;
  case "path":candidate.pairs[0].path="foreign.json";break;
  case "binding":candidate.bindings[0].sha256="0".repeat(64);break;
  case "missing-binding":candidate.bindings.pop();break;
  case "assertion":case "scene-assertion":case "kernel-assertion":{const index=row.mutation==="assertion"?0:row.mutation==="scene-assertion"?1:2;mutatedContext.laws[index].body=laws[index].body.replace("assert_fixed_slot_tables(","foreign_guard(");assert.notEqual(mutatedContext.laws[index].body,laws[index].body);break;}
  case "world-model":candidate.coupledWorld.afterHash="0".repeat(64);break;
  case "world-recipe":candidate.coupledWorld.methodOnlyRecipe.replacements[0].after="foreign";break;
  case "kernel-element":endpoint.tables.find((r:any)=>r.owner==="kernel_runtime::RetainedSurfaceRegistry").elementSizeBytes++;break;
  case "kernel-capacity":endpoint.tables.find((r:any)=>r.owner==="kernel_runtime::RetainedSurfaceRegistry").capacity++;break;
  case "kernel-owner":endpoint.tables.find((r:any)=>r.owner==="kernel_runtime::RetainedSurfaceRegistry").ownerSizeBytes++;break;
  case "missing-kernel-correction":endpoint.tables.find((r:any)=>r.owner==="kernel_runtime::RetainedSurfaceRegistry").elementSizeBytes=24488;break;
  case "store-declaration":candidate.storeTransition.declarationChanges=[];break;
  case "current-measurement":candidate.currentNativeMeasurement.engineElementBytes++;break;
  case "measurement-physical-acceptance":candidate.currentNativeMeasurement.physicalLayoutAccepted=true;break;
  case "missing-current-measurement":delete candidate.currentNativeMeasurement;break;
  case "authority-transitions":candidate.currentAuthorityTransitions[0].afterHash="0".repeat(64);break;
  case "authority-layout-acceptance":candidate.currentAuthorityTransitions[0].physicalLayoutAccepted=true;break;
  case "missing-authority-transitions":delete candidate.currentAuthorityTransitions;break;
  case "declared-field-delta":candidate.retainedCloneTransition.declarationChanges=[];break;
  case "retained-clone-transition":candidate.retainedCloneTransition.afterHash="0".repeat(64);break;
  case "retained-clone-layout-acceptance":candidate.retainedCloneTransition.physicalLayoutAccepted=true;break;
  case "missing-retained-clone-transition":delete candidate.retainedCloneTransition;break;
  case "store-transition":candidate.storeTransition.afterHash="0".repeat(64);break;
  case "store-layout-acceptance":candidate.storeTransition.physicalLayoutAccepted=true;break;
 }
 if(["kernel-element","kernel-capacity","kernel-owner","missing-kernel-correction","stack","worker","threshold","capacity","owner-bytes","owner-name","other-row","element","scene-element","scene-capacity","scene-owner-bytes","scene-owner-name","missing-scene-correction","missing-engine-correction","extra-field","missing-row"].includes(row.mutation)){candidate.pairs[0].after=JSON.stringify(endpoint,null,2)+"\n";candidate.pairs[0].afterHash=sha(candidate.pairs[0].after);candidate.pairs[0].forward.insert=candidate.pairs[0].after;candidate.pairs[0].inverse.delete=candidate.pairs[0].after;}
 const own=admit(candidate,mutatedContext,JSON.parse),oracle=admit(candidate,mutatedContext,reference.parse);outcomes.push({...row,own,oracle,agrees:own===row.accepted&&oracle===row.accepted});
}
const schemaOutcomes=[];
for(const [id,value,expected]of [["schema-normative",controls,true],["schema-extra",{...controls,foreign:0},false],["schema-incomplete",{...controls,cases:controls.cases.slice(1)},false],["schema-unknown",{...controls,cases:controls.cases.map((row:any,index:number)=>index?row:{...row,mutation:"foreign"})},false],["schema-wrong-type",{...controls,cases:controls.cases.map((row:any,index:number)=>index?row:{...row,accepted:"true"})},false]] as const){const own=ownSchema(schema,value),oracle=Boolean(reference.schema(value));schemaOutcomes.push({id,own,oracle,expected,agrees:own===expected&&oracle===expected});}
const errors=[...outcomes,...schemaOutcomes].filter(row=>!row.agrees);
for(const [path,hash]of bindings){check();assert.equal(sha(raw(path)),hash,"Post current authority advanced: "+path);}
assert.equal(text(sourcePath),pair.before);for(const law of laws)assert.equal(text(law.path),law.body);assert.equal(text(resolve(root,coupledWorld.path)),worldPair.before);assert.equal(JSON.parse(text(coupledWorld.declaredModelPath)).body,worldPair.after);
const result={epoch,ready:errors.length===0,outcomes,schemaOutcomes,errors,registrations,sourceWritesOutsideTicket:false,nativeExecuted:false,liveIdentity:false,atomicity:false,publicationReady:false};write(join(output,"controls.json"),JSON.stringify(result,null,2)+"\n");
progress("closed-source-controls",{cases:outcomes.length,schemaCases:schemaOutcomes.length,errors:errors.length});
if(errors.length)process.exit(1);
bind(require.resolve("json5/package.json"));bind(require.resolve("ajv/package.json"));bind(join(output,"controls.json"));
const receipt={...result,at:new Date().toISOString(),ready:true,pairs:proposal.pairs,bindings:[...bindings].map(([path,sha256])=>({path,sha256})),authorities,proposal:{path:proposalPath,sha256:sha(proposalBody),source:proposalBody},controls:{normativeAdversarial:outcomes,closedSchema:schemaOutcomes,originalAssertionBodyConserved:true,ownParser:"JSON.parse",thirdPartyParser:"JSON5",thirdPartySchema:"AJV"},scope:"Coupled test-only current Renderer three-descriptor source admission for joint held native verification; all three fresh original wholes still required"};
write(receiptPath,JSON.stringify(receipt,null,2)+"\n");progress("sealed",{receiptPath,bindings:bindings.size,pairs:1});
