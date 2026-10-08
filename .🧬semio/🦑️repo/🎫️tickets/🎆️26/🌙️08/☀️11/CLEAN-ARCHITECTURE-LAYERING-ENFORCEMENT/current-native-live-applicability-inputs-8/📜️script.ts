import assert from "node:assert/strict";
import {isDeepStrictEqual} from "node:util";
import {createHash} from "node:crypto";
import {createReadStream} from "node:fs";
import {readFile,writeFile,lstat,realpath,readdir,mkdir,unlink,symlink} from "node:fs/promises";
import {createRequire} from "node:module";
import {resolve,relative,isAbsolute,join,dirname,win32} from "node:path";
type Entry={path:string;kind:"file"|"directory";sha256:string|null};
type Binding={path:string;sha256:string};
type UnitInputV1={snapshot:string;specs:{package:string;sources:string[]}[];units:{path:string;sources:string[]}[];required:string[]};
type Pair={path:string;before:string|null;after:string;beforeHash:string|null;afterHash:string;forward:{start:0;delete:string|null;insert:string};inverse:{start:0;delete:string;insert:string|null}};
export type LiveApplicabilityContextV1={signal?:AbortSignal;progress?:(event:{phase:string;done:number;total:number})=>void};
export type LiveApplicabilityProofV1={version:1;ready:true;root:string;ticket:string;models:Pair[];rootInputs:{path:string;beforeHash:string|null;afterHash:string|null;model:boolean;reasons:string[]}[];capturedBindings:Binding[];memberships:{path:string;scope:"root"|"immutable";mode:"files"|"tree";before:Entry[];after:Entry[]}[];owners:any[];sourceWritesOutsideTicket:false;nativeExecuted:false;publicationReady:false;scope:string};
const schemaScope="Current joint33 selected live applicability with negative runtime controls; no atomicity claim";
const sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
const inside=(base:string,path:string)=>{const r=relative(resolve(base),resolve(path));return !r||!r.startsWith(".."+"/")&&!r.startsWith(".."+String.fromCharCode(92))&&r!==".."&&!isAbsolute(r);};
const relativePath=(path:string)=>assert.ok(path&&!isAbsolute(path)&&!win32.isAbsolute(path)&&!path.split(/[\\/]/u).some(p=>p===".."||p==="."||!p),"Exact relative authority required");
const tick=async(context:LiveApplicabilityContextV1,phase:string,done:number,total:number)=>{context.signal?.throwIfAborted();if(done===total||done%64===0){context.progress?.({phase,done,total});await new Promise<void>(r=>setImmediate(r));context.signal?.throwIfAborted();}};
const absent=async(path:string)=>{try{await lstat(path);return false;}catch(e){if((e as NodeJS.ErrnoException).code==="ENOENT")return true;throw e;}};
const regular=async(path:string)=>{const s=await lstat(path);assert.ok(s.isFile()&&!s.isSymbolicLink(),"Regular file required: "+path);assert.equal(resolve(await realpath(path)),resolve(path),"Linked ancestor refused: "+path);return s;};
const hashPath=async(path:string,context:LiveApplicabilityContextV1={})=>{context.signal?.throwIfAborted();const a=await regular(path),h=createHash("sha256");for await(const chunk of createReadStream(path,{highWaterMark:1048576})){context.signal?.throwIfAborted();h.update(chunk);}const b=await regular(path);assert.equal(a.size,b.size);assert.equal(a.mtimeMs,b.mtimeMs);assert.equal(a.ino,b.ino);assert.equal(a.dev,b.dev);return h.digest("hex");};
const text=async(path:string)=>{const s=await regular(path);assert.ok(s.size<=268435456,"Bounded text authority required");const b=await readFile(path),value=b.toString("utf8");assert.ok(b.equals(Buffer.from(value)),"Exact UTF8 required");return value;};
const json=async(path:string)=>JSON.parse(await text(path));
const writeTicket=async(ticket:string,path:string,value:any)=>{assert.ok(inside(ticket,path)&&resolve(path)!==resolve(ticket));await mkdir(dirname(path),{recursive:true});await writeFile(path,typeof value==="string"?value:JSON.stringify(value,null,2)+"\n",{flag:"wx"});};

/** 🧬️ Checks the closed JSON contract independently from the CLI-only AJV oracle. */
function ownSchema(schema:any,value:any):boolean{
 if(schema.anyOf&&!schema.anyOf.some((s:any)=>ownSchema(s,value)))return false;
 if("const" in schema&&!isDeepStrictEqual(schema.const,value))return false;
 if(schema.enum&&!schema.enum.some((v:any)=>isDeepStrictEqual(v,value)))return false;
 const kinds=Array.isArray(schema.type)?schema.type:[schema.type],kind=value===null?"null":Array.isArray(value)?"array":typeof value;
 if(schema.type&&!kinds.includes(kind))return false;
 if(kind==="object"){if(schema.required?.some((k:string)=>!Object.hasOwn(value,k)))return false;for(const [k,v]of Object.entries(value)){if(!schema.properties?.[k]){if(schema.additionalProperties===false)return false;}else if(!ownSchema(schema.properties[k],v))return false;}}
 if(kind==="array"){if(schema.minItems!==undefined&&value.length<schema.minItems||schema.maxItems!==undefined&&value.length>schema.maxItems)return false;if(schema.items&&value.some((v:any)=>!ownSchema(schema.items,v)))return false;}
 if(kind==="string"){if(schema.minLength!==undefined&&[...value].length<schema.minLength)return false;if(schema.pattern&&!new RegExp(schema.pattern,"u").test(value))return false;}
 return true;
}

async function registered(base:string,command:string){
 const rows=await json(join(base,"gui.json")),row=rows.find((r:any)=>r.name==="📥️native-live-applicability-"+command+"-8🧪️"),inputs=await json(join(base,"inputs.json"));assert.ok(row);
 for(const [k,v]of Object.entries(row.env))assert.equal(process.env[k],String(v).replaceAll("${workspaceFolder}",inputs.root));
 const req=createRequire(join(inputs.root,"package.json")),parser=req("json5"),observations=[];
 for(const p of [join(inputs.root,".vscode/launch.json"),join(inputs.root,".vscode/🧩️launch.seed.jsonc")]){const body=await text(p),rows=parser.parse(body).configurations.filter((r:any)=>r.name===row.name);assert.deepEqual(rows,[row]);observations.push({path:p,sha256:sha(body),row});}
 return observations;
}

async function lawFixture(ticket:string,caseRoot:string,phase:string){
 const root=join(caseRoot,"root"),sealed=join(caseRoot,"sealed"),external=join(caseRoot,"external"),models:Pair[]=[];
 for(let i=0;i<33;i++){const path="model/m"+i+".rs",before=i===31?null:"before-"+i,after="after-"+i;models.push({path,before,after,beforeHash:before===null?null:sha(before),afterHash:sha(after),forward:{start:0,delete:before,insert:after},inverse:{start:0,delete:after,insert:before}});if(phase==="after"||before!==null)await writeTicket(ticket,join(root,path),phase==="after"?after:before);}
 await writeTicket(ticket,join(root,"selected.rs"),"selected");await writeTicket(ticket,join(root,"watch/member.svg"),"asset");await writeTicket(ticket,join(sealed,"helper.ts"),"sealed helper");await writeTicket(ticket,join(sealed,"generated.rs"),"generated");await writeTicket(ticket,join(external,"index.js"),"external");
 models.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
 const rootInputs=[{path:"bunfig.toml",beforeHash:null,afterHash:null,model:false,reasons:["negative-runtime-control"]},...models.map(m=>({path:m.path,beforeHash:m.beforeHash,afterHash:m.afterHash,model:true,reasons:["model"]})),{path:"selected.rs",beforeHash:sha("selected"),afterHash:sha("selected"),model:false,reasons:["compiler"]},{path:"watch/member.svg",beforeHash:sha("asset"),afterHash:sha("asset"),model:false,reasons:["watch"]}];
 const bindings:Binding[]=[{path:join(sealed,"helper.ts"),sha256:sha("sealed helper")},{path:join(sealed,"generated.rs"),sha256:sha("generated")},{path:join(external,"index.js"),sha256:sha("external")}],owners=[];
 for(const id of ["ui","board","product"]){
  const planPath=join(sealed,id+"-plan.json"),binaryMetadataPath=join(sealed,id+"-binary.json"),snapshotRoot=join(sealed,id,"prepared"),joins=[];
  await writeTicket(ticket,binaryMetadataPath,id+"binary");bindings.push({path:binaryMetadataPath,sha256:sha(id+"binary")});
  for(const input of rootInputs){const m=models.find(m=>m.path===input.path),before=m?m.before:input.path==="bunfig.toml"?null:input.path==="selected.rs"?"selected":"asset",after=m?m.after:before,initialPath=join(sealed,id,"initial",input.path),modelPath=join(sealed,id,"model",input.path),preparedPath=join(snapshotRoot,input.path);if(before!==null){await writeTicket(ticket,initialPath,before);bindings.push({path:initialPath,sha256:sha(before)});}if(after!==null){await writeTicket(ticket,modelPath,after);await writeTicket(ticket,preparedPath,after);bindings.push({path:modelPath,sha256:sha(after)},{path:preparedPath,sha256:sha(after)});}joins.push({path:input.path,initialPath,initialHash:input.beforeHash,modelPath,modelHash:after===null?null:sha(after),preparedPath,preparedHash:after===null?null:sha(after)});}
  const plan={snapshot:snapshotRoot,initialChecks:joins.filter(r=>r.initialHash!==null).map(r=>({path:r.path,expected:r.initialHash})),modelChecks:joins.filter(r=>r.modelHash!==null).map(r=>({path:r.path,expected:r.modelHash})),checks:joins.filter(r=>r.preparedHash!==null).map(r=>({path:r.path,expected:r.preparedHash})),modelDerivation:{pairs:models}};
  const planBody=JSON.stringify(plan);await writeTicket(ticket,planPath,planBody);bindings.push({path:planPath,sha256:sha(planBody)});
  const depfilePath=join(sealed,id+"-unit.d"),dependencyPaths=[join(snapshotRoot,"selected.rs"),join(sealed,"generated.rs")],depfileBody=join(sealed,id+"-binary")+": "+dependencyPaths.map(p=>p.replaceAll(" ","\\ ")).join(" ")+"\n";await writeTicket(ticket,depfilePath,depfileBody);bindings.push({path:depfilePath,sha256:sha(depfileBody)});
  owners.push({id,planPath,planHash:sha(planBody),binaryMetadataPath,binaryMetadataHash:sha(id+"binary"),snapshotRoot,selectedPackages:["fixture-"+id],executedPackages:["fixture-"+id],uncompiledPackages:[],depfiles:[{path:depfilePath,sha256:sha(depfileBody),package:"fixture-"+id}],dependencies:[{path:join(sealed,"generated.rs"),sha256:sha("generated"),kind:"generated",rootPath:null},{path:join(snapshotRoot,"selected.rs"),sha256:sha("selected"),kind:"root",rootPath:"selected.rs"}],joins});
 }
 const before=[{path:"member.svg",kind:"file" as const,sha256:sha("asset")}],immutable=[{path:"index.js",kind:"file" as const,sha256:sha("external")}];
 return{version:1 as const,ready:true as const,root,ticket,models,rootInputs,capturedBindings:bindings,memberships:[{path:"watch",scope:"root" as const,mode:"tree" as const,before,after:before},{path:external,scope:"immutable" as const,mode:"tree" as const,before:immutable,after:immutable}],owners,sourceWritesOutsideTicket:false as const,nativeExecuted:false as const,publicationReady:false as const,scope:schemaScope};
}
async function observe(proof:LiveApplicabilityProofV1){
 const nullableHash=async(p:string)=>await absent(p)?null:await hashPath(p),root=[],bindings=[],membership=[];
 for(const r of proof.rootInputs)root.push({path:r.path,sha256:await nullableHash(join(proof.root,r.path))});
 for(const b of proof.capturedBindings)bindings.push({path:b.path,sha256:await nullableHash(b.path)});
 for(const m of proof.memberships)membership.push({path:m.path,entries:await members(m.scope==="root"?join(proof.root,m.path):m.path,m.mode)});
 const witnesses=[];for(const owner of proof.owners)for(const r of owner.joins)for(const [path,expected] of [[r.initialPath,r.initialHash],[r.modelPath,r.modelHash],[r.preparedPath,r.preparedHash]])witnesses.push({path,sha256:await nullableHash(path)});return{root,bindings,membership,witnesses};
}

async function laws(base:string,ticket:string,out:string,context:LiveApplicabilityContextV1){
 const fixtures=await json(join(base,"fixtures.json")),schema=await json(join(base,"schema.json")),proofSchema=await json(join(base,"proof-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile(schema),validateProof=ajv.compile(proofSchema);assert.ok(ownSchema(schema,fixtures)&&validate(fixtures));const outcomes=[];
 for(const [i,c]of fixtures.cases.entries()){const phase=c.mutation.includes("after")&&!c.mutation.includes("after-before")?"after":"before",proof=await lawFixture(ticket,join(out,c.id),phase),original=structuredClone(proof);let chosen=phase;
  const overwrite=async(p:string,v:string)=>writeFile(p,v);
  switch(c.mutation){
   case "model-noncanonical-order":{proof.models.reverse();for(const owner of proof.owners){const plan=await json(owner.planPath);plan.modelDerivation.pairs.reverse();await overwrite(owner.planPath,JSON.stringify(plan,null,2)+"\n");owner.planHash=sha(await text(owner.planPath));proof.capturedBindings.find(b=>b.path===owner.planPath)!.sha256=owner.planHash;}break;}
   case "negative-file":case "negative-file-after":await writeTicket(ticket,join(proof.root,"bunfig.toml"),"introduced");break;
   case "negative-directory":case "negative-directory-after":await mkdir(join(proof.root,"bunfig.toml"));break;
   case "negative-symlink":case "negative-symlink-after":await symlink(join(proof.root,"selected.rs"),join(proof.root,"bunfig.toml"));break;
   case "negative-model-witness":await writeTicket(ticket,proof.owners[0].joins.find((j:any)=>j.path==="bunfig.toml").modelPath,"introduced");break;
   case "negative-prepared-witness":await writeTicket(ticket,proof.owners[0].joins.find((j:any)=>j.path==="bunfig.toml").preparedPath,"introduced");break;
   case "unrelated-root":await writeTicket(ticket,join(proof.root,"unrelated.rs"),"peer");break;
   case "selected-root":await overwrite(join(proof.root,"selected.rs"),"drift");break;
   case "model-after-before":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].after);break;
   case "model-before-after":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].before!);break;
   case "new-model-created-before":await writeTicket(ticket,join(proof.root,proof.models.find(m=>m.before===null)!.path),proof.models.find(m=>m.before===null)!.after);break;
   case "new-model-missing-after":await unlink(join(proof.root,proof.models.find(m=>m.before===null)!.path));break;
   case "immutable-change":case "helper-change":await overwrite(proof.capturedBindings[0].path,"drift");break;
   case "immutable-missing":await unlink(proof.capturedBindings[0].path);break;
   case "watch-add":await writeTicket(ticket,join(proof.root,"watch/new.svg"),"added");break;
   case "watch-remove":await unlink(join(proof.root,"watch/member.svg"));break;
   case "watch-change":await overwrite(join(proof.root,"watch/member.svg"),"changed");break;
   case "watch-kind":await unlink(join(proof.root,"watch/member.svg"));await mkdir(join(proof.root,"watch/member.svg"));break;
   case "generated-change":await overwrite(proof.capturedBindings[1].path,"drift");break;
   case "external-change":await overwrite(proof.capturedBindings[2].path,"drift");break;
   case "external-add":await writeTicket(ticket,join(dirname(proof.capturedBindings[2].path),"new.js"),"added");break;
   case "witness-change":await overwrite(proof.owners[0].joins[0].preparedPath,"drift");break;
   case "missing-source":await unlink(join(proof.root,"selected.rs"));break;
   case "unknown-phase":chosen="foreign";break;
   case "extra-proof":(proof as any).foreign=true;break;
   case "missing-model":proof.models.pop();break;
   case "pair-hash":proof.models[0].afterHash="0".repeat(64);break;
   case "pair-forward":proof.models[0].forward.insert="foreign";break;
   case "pair-inverse":proof.models[0].inverse.insert="foreign";break;
   case "model-absence-kind":await mkdir(join(proof.root,proof.models.find(m=>m.before===null)!.path),{recursive:true});break;
   case "owner-duplicate":proof.owners[1]=structuredClone(proof.owners[0]);break;
   case "owner-missing":proof.owners.pop();break;
   case "join-initial-hash":proof.owners[0].joins[0].initialHash="0".repeat(64);break;
   case "join-model-hash":proof.owners[0].joins[0].modelHash="0".repeat(64);break;
   case "join-prepared-hash":proof.owners[0].joins[0].preparedHash="0".repeat(64);break;
   case "join-missing":proof.owners[0].joins.pop();break;
   case "plan-change":await overwrite(proof.owners[0].planPath,"drift");break;
   case "dependency-hash":proof.owners[0].dependencies[0].sha256="0".repeat(64);break;
   case "dependency-missing":await unlink(proof.owners[0].dependencies[0].path);break;
   case "dependency-root-map":proof.owners[0].dependencies[1].rootPath="foreign.rs";break;
   case "missing-compiled-dep":proof.owners[0].dependencies.pop();break;
   case "depfile-change":await overwrite(proof.owners[0].depfiles[0].path,"foreign: foreign.rs\n");break;
   case "depfile-unlisted":proof.owners[0].depfiles=[];break;
   case "declared-executed-overlap":proof.owners[0].uncompiledPackages=[...proof.owners[0].executedPackages];break;
   case "watch-empty-directory":await mkdir(join(proof.root,"watch","new"));break;
   case "external-empty-directory":await mkdir(join(dirname(proof.capturedBindings[2].path),"new"));break;
   case "root-input-duplicate":proof.rootInputs.push(structuredClone(proof.rootInputs[0]));break;
   case "model-relative-traversal":proof.models[0].path="../foreign.rs";break;
   case "nonmodel-after-change":proof.rootInputs.find(r=>!r.model)!.afterHash="0".repeat(64);break;
   case "immutable-root-injection":proof.capturedBindings.push({path:join(proof.root,"selected.rs"),sha256:sha("selected")});break;
   case "binding-symlink":{const path=proof.capturedBindings[0].path;await unlink(path);await symlink(proof.capturedBindings[1].path,path);break;}
  }
  const localController=new AbortController(),localContext={...context,signal:localController.signal,progress:(event:any)=>{context.progress?.(event);if(c.mutation==="cancel-during"&&event.phase==="bindings")localController.abort();}};if(c.mutation==="cancel-before")localController.abort();
  let own=false,oracle=false,ownError=null,oracleError=null;
  try{await assertLiveApplicabilityV1(proof,chosen as any,localContext);own=true;}catch(e){ownError=String(e);}
  try{const expected={root:original.rootInputs.map(r=>({path:r.path,sha256:phase==="after"?r.afterHash:r.beforeHash})),bindings:original.capturedBindings,membership:original.memberships.map(m=>({path:m.path,entries:phase==="after"?m.after:m.before})),witnesses:original.owners.flatMap(owner=>owner.joins.flatMap((r:any)=>[{path:r.initialPath,sha256:r.initialHash},{path:r.modelPath,sha256:r.modelHash},{path:r.preparedPath,sha256:r.preparedHash}]))},observation=await observe(original),contract=ajv.compile({type:"object",additionalProperties:false,required:["phase","proof","observed","cancelled"],properties:{phase:{const:phase},proof:{const:original},observed:{const:expected},cancelled:{const:false}}});oracle=Boolean(validateProof(proof)&&contract({phase:chosen,proof,observed:observation,cancelled:localController.signal.aborted}));}catch(e){oracleError=String(e);}
  outcomes.push({id:c.id,mutation:c.mutation,expected:c.accepted,own,oracle,agrees:own===c.accepted&&oracle===c.accepted,ownError,oracleError});await tick(context,"laws",i+1,fixtures.cases.length);
 }
 const parserOutcomes=[],parserOracle=ajv.compile({type:"string",pattern:"^(?![\\s\\S]*\\\\\\r?\\n)[^\\r\\n]+: [^\\s$#\\r\\n][^$#\\r\\n]*\\r?\\n(?:[^\\r\\n]*\\r?\\n)*$"});
 for(const c of fixtures.parserCases){let accepted=false,paths:string[]=[];try{paths=depfileSourcesV1(c.source);accepted=true;}catch{}const oracle=Boolean(parserOracle(c.source));parserOutcomes.push({...c,own:accepted,oracle,paths,agrees:accepted===c.accepted&&oracle===c.accepted&&(!accepted||isDeepStrictEqual(paths,c.paths))});}
 const schemaOutcomes=[];for(const [id,value,expected]of [["normative",fixtures,true],["extra",{...fixtures,foreign:0},false],["missing",{...fixtures,cases:fixtures.cases.slice(1)},false],["unknown",{...fixtures,cases:fixtures.cases.map((c:any,i:number)=>i?c:{...c,mutation:"foreign"})},false]] as const)schemaOutcomes.push({id,expected,own:ownSchema(schema,value),oracle:Boolean(validate(value))});
 const errors=[...outcomes,...parserOutcomes].filter(c=>!c.agrees).concat(schemaOutcomes.filter(c=>c.own!==c.expected||c.oracle!==c.expected) as any),result={at:new Date().toISOString(),ready:errors.length===0,outcomes,parserOutcomes,schemaOutcomes,errors,sourceWritesOutsideTicket:false,nativeExecuted:false};await writeTicket(ticket,join(out,"controls.json"),result);assert.equal(errors.length,0,"Finite applicability controls failed");return result;
}


/** 📜️ Decodes the admitted first-line compiler depfile grammar and refuses unsupported forms. */
export function depfileSourcesV1(body:string):string[]{
 assert.ok(!body.includes("\\\n")&&!body.includes("\\\r\n"),"Continuation is outside the admitted grammar");
 const line=body.split(/\r?\n/u).find(line=>line.includes(": "));assert.ok(line,"Dependency rule required");const separator=line.indexOf(": ");assert.ok(separator>0);
 const tail=line.slice(separator+2);assert.ok(tail&&!/[$#\u0000-\u001f]/u.test(tail),"Unsupported depfile atom");
 const paths:string[]=[],parts:string[]=[];
 for(let i=0;i<tail.length;i++){const c=tail[i];if(c===" "){if(parts.length){paths.push(parts.join(""));parts.length=0;}}else if(c===String.fromCharCode(92)&&tail[i+1]===" "){parts.push(" ");i++;}else if(c===String.fromCharCode(92)&&tail[i+1]===String.fromCharCode(92)){parts.push(c);i++;}else parts.push(c);}
if(parts.length)paths.push(parts.join(""));assert.ok(paths.length);return paths;
}

/** 🗂️ Observes one explicitly bounded directory membership with regular physical custody. */
async function members(path:string,mode:"files"|"tree",context:LiveApplicabilityContextV1={}):Promise<Entry[]>{
 const entries:Entry[]=[];let done=0;
 const walk=async(path:string,rel:string)=>{context.signal?.throwIfAborted();const s=await lstat(path);assert.ok(!s.isSymbolicLink(),"Linked membership refused: "+path);assert.equal(resolve(await realpath(path)),resolve(path));if(s.isDirectory()){if(mode==="tree"&&rel)entries.push({path:rel,kind:"directory",sha256:null});for(const n of (await readdir(path)).sort())await walk(join(path,n),rel?rel+"/"+n:n);}else{assert.ok(s.isFile());entries.push({path:rel,kind:"file",sha256:await hashPath(path,context)});}await tick(context,"membership",++done,Number.MAX_SAFE_INTEGER);};
 const s=await lstat(path);assert.ok(s.isDirectory()&&!s.isSymbolicLink());await walk(path,"");return entries.sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
}

/** 🔒️ Performs only proof-driven reads; imported callers supply cancellation and progress. */
export async function assertLiveApplicabilityV1(proof:LiveApplicabilityProofV1,phase:"before"|"after",context:LiveApplicabilityContextV1={}):Promise<{rootInputs:number;bindings:number;memberships:number}>{
 context.signal?.throwIfAborted();assert.ok(ownSchema(await json(join(import.meta.dir,"proof-schema.json")),proof),"Closed applicability proof required");assert.ok(phase==="before"||phase==="after");
 const unique=<T>(rows:T[],key:(row:T)=>string)=>{const map=new Map<string,T>();for(const row of rows){const k=key(row);assert.ok(!map.has(k),"Duplicate authority: "+k);map.set(k,row);}return map;};
 const absolute=(path:string)=>{assert.ok(isAbsolute(path)&&resolve(path)===path,"Canonical absolute authority required");return path;};
 const directory=async(path:string)=>{absolute(path);const stat=await lstat(path);assert.ok(stat.isDirectory()&&!stat.isSymbolicLink());assert.equal(await realpath(path),path);};
 const missing=async(path:string)=>{assert.ok(await absent(path),"Absent authority became present: "+path);let parent=dirname(path);while(await absent(parent)){const next=dirname(parent);assert.notEqual(next,parent);parent=next;}await directory(parent);};
 await directory(proof.root);await directory(proof.ticket);
 assert.deepEqual(proof.models,[...proof.models].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),"Canonical UTF8 model order required");
 const models=unique(proof.models,m=>m.path),inputs=unique(proof.rootInputs,r=>r.path),bindings=unique(proof.capturedBindings,b=>absolute(b.path)),owners=unique(proof.owners,o=>o.id);
 assert.deepEqual([...owners.keys()].sort(),["board","product","ui"]);
 for(const m of models.values()){relativePath(m.path);assert.equal(m.beforeHash,m.before===null?null:sha(m.before));assert.equal(m.afterHash,sha(m.after));assert.deepEqual(m.forward,{start:0,delete:m.before,insert:m.after});assert.deepEqual(m.inverse,{start:0,delete:m.after,insert:m.before});const r=inputs.get(m.path);assert.ok(r?.model);assert.equal(r.beforeHash,m.beforeHash);assert.equal(r.afterHash,m.afterHash);}
 for(const r of inputs.values()){relativePath(r.path);assert.ok(r.reasons.length);assert.equal(r.model,models.has(r.path));if(!r.model){assert.equal(r.beforeHash,r.afterHash);}}
 const immutableRoots=proof.memberships.filter(m=>m.scope==="immutable").map(m=>absolute(m.path));
 for(const b of bindings.values()){assert.ok(!inputs.has(relative(proof.root,b.path)),"Selected Root cannot be immutable binding");assert.ok(inside(proof.ticket,b.path)||immutableRoots.some(root=>inside(root,b.path)),"Binding outside explicit immutable custody: "+b.path);assert.ok(!inside(proof.root,b.path)||inside(proof.ticket,b.path)||immutableRoots.some(root=>inside(root,b.path)),"Mutable Root injection refused");}
 const bound=(path:string,expected:string)=>{absolute(path);assert.equal(bindings.get(path)?.sha256,expected,"Missing exact immutable binding: "+path);};
 let done=0;await tick(context,"bindings",0,bindings.size);for(const b of bindings.values()){assert.equal(await hashPath(b.path,context),b.sha256,"Immutable body drift: "+b.path);await tick(context,"bindings",++done,bindings.size);}
 const core=(m:any)=>({path:m.path,before:m.before,after:m.after,beforeHash:m.beforeHash,afterHash:m.afterHash,forward:m.forward,inverse:m.inverse});
 for(const owner of owners.values()){
  bound(owner.planPath,owner.planHash);bound(owner.binaryMetadataPath,owner.binaryMetadataHash);assert.ok(inside(proof.ticket,owner.planPath)&&inside(proof.ticket,owner.binaryMetadataPath));absolute(owner.snapshotRoot);assert.ok(inside(proof.ticket,owner.snapshotRoot));
  const plan=await json(owner.planPath);assert.equal(plan.snapshot,owner.snapshotRoot);assert.deepEqual(plan.modelDerivation.pairs.map(core).sort((a:any,b:any)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),proof.models);
  const initial=unique<any>(plan.initialChecks,r=>r.path),modeled=unique<any>(plan.modelChecks,r=>r.path),prepared=unique<any>(plan.checks,r=>r.path),joins=unique<any>(owner.joins,r=>r.path);
  assert.deepEqual([...joins.keys()].sort(),[...inputs.keys()].sort());
  for(const r of joins.values()){
   const input=inputs.get(r.path)!;relativePath(r.path);for(const path of [r.initialPath,r.modelPath,r.preparedPath]){absolute(path);assert.ok(inside(proof.ticket,path));}assert.equal(r.preparedPath,join(owner.snapshotRoot,r.path));
   if(plan.initialSnapshot)assert.equal(r.initialPath,join(plan.initialSnapshot,r.path));if(plan.modelSnapshot)assert.equal(r.modelPath,join(plan.modelSnapshot,r.path));
   assert.equal(r.initialHash,input.beforeHash);assert.equal(r.modelHash,input.afterHash);assert.equal(initial.get(r.path)?.expected??null,r.initialHash);assert.equal(modeled.get(r.path)?.expected??null,r.modelHash);assert.equal(prepared.get(r.path)?.expected??null,r.preparedHash);
   if(r.initialHash===null)await missing(r.initialPath);else bound(r.initialPath,r.initialHash);if(r.modelHash===null)await missing(r.modelPath);else bound(r.modelPath,r.modelHash);if(r.preparedHash===null)await missing(r.preparedPath);else bound(r.preparedPath,r.preparedHash);await tick(context,"joins",++done,Number.MAX_SAFE_INTEGER);
  }
  const selected=unique<string>(owner.selectedPackages,p=>p),executed=unique<string>(owner.executedPackages,p=>p),uncompiled=unique<string>(owner.uncompiledPackages,p=>p);assert.ok(executed.size);for(const p of executed.keys())assert.ok(selected.has(p)&&!uncompiled.has(p));for(const p of uncompiled.keys())assert.ok(selected.has(p)&&!executed.has(p));assert.equal(selected.size,executed.size+uncompiled.size);
  const deps=unique<any>(owner.dependencies,d=>absolute(d.path)),files=unique<any>(owner.depfiles,d=>absolute(d.path)),compiled=new Set<string>(),seen=new Set<string>();assert.ok(files.size);
  for(const d of deps.values()){bound(d.path,d.sha256);if(d.kind==="root"){relativePath(d.rootPath);assert.equal(d.path,join(owner.snapshotRoot,d.rootPath));assert.equal(joins.get(d.rootPath)?.preparedHash,d.sha256);}else{assert.equal(d.rootPath,null);if(d.kind==="generated")assert.ok(inside(proof.ticket,d.path));else assert.ok(immutableRoots.some(root=>inside(root,d.path)));}}
  for(const file of files.values()){bound(file.path,file.sha256);assert.ok(inside(proof.ticket,file.path)&&executed.has(file.package));seen.add(file.package);for(const path of depfileSourcesV1(await text(file.path))){const resolved=isAbsolute(path)?resolve(path):resolve(owner.snapshotRoot,path);assert.ok(deps.has(resolved),"Unsealed compiler dependency: "+resolved);compiled.add(resolved);}await tick(context,"depfiles",++done,Number.MAX_SAFE_INTEGER);}
  assert.deepEqual([...seen].sort(),[...executed.keys()].sort());assert.deepEqual([...compiled].sort(),[...deps.keys()].sort());
 }
 done=0;for(const r of inputs.values()){const path=join(proof.root,r.path),expected=phase==="before"?r.beforeHash:r.afterHash;if(expected===null)await missing(path);else assert.equal(await hashPath(path,context),expected,"Selected Root body drift: "+r.path);await tick(context,"root",++done,inputs.size);}
 const membershipKeys=new Set<string>();for(const m of proof.memberships){const key=m.scope+":"+m.path;assert.ok(!membershipKeys.has(key));membershipKeys.add(key);if(m.scope==="root")relativePath(m.path);else{absolute(m.path);assert.deepEqual(m.before,m.after);}
  for(const [rows,which] of [[m.before,"before"],[m.after,"after"]] as const){unique(rows,r=>r.path);assert.deepEqual(rows,[...rows].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0));for(const r of rows){relativePath(r.path);if(r.kind==="directory")assert.equal(r.sha256,null);else{assert.notEqual(r.sha256,null);if(m.scope==="root"){const input=inputs.get(join(m.path,r.path));assert.ok(input,"Watched file lacks selected input");assert.equal(r.sha256,which==="before"?input.beforeHash:input.afterHash);}}}}
  const path=m.scope==="root"?join(proof.root,m.path):m.path;assert.deepEqual(await members(path,m.mode,context),phase==="before"?m.before:m.after,"Membership drift: "+path);
 }
 done=0;for(const b of bindings.values()){assert.equal(await hashPath(b.path,context),b.sha256,"Immutable post-read drift: "+b.path);await tick(context,"bindings-post",++done,bindings.size);}context.signal?.throwIfAborted();
 return{rootInputs:inputs.size,bindings:bindings.size,memberships:proof.memberships.length};
}

/** 🦀️ Separates declared provider packages from observed compiled unit sources. */
function selectUnitsV1(input:UnitInputV1):any{
 assert.ok(isAbsolute(input.snapshot)&&resolve(input.snapshot)===input.snapshot);assert.ok(input.required.length);const specs=new Map<string,string[]>(),units=new Map<string,any>(),executed=new Set<string>();
 for(const row of input.specs){assert.ok(row.package&&!specs.has(row.package)&&row.sources.length);for(const path of row.sources)assert.ok(isAbsolute(path)&&resolve(path)===path&&inside(input.snapshot,path));assert.equal(new Set(row.sources).size,row.sources.length);specs.set(row.package,row.sources);}
 for(const row of input.units){assert.ok(isAbsolute(row.path)&&resolve(row.path)===row.path&&!units.has(row.path)&&row.sources.length);assert.equal(new Set(row.sources).size,row.sources.length);for(const path of row.sources)assert.ok(isAbsolute(path)&&resolve(path)===path);units.set(row.path,row);}
 const selectedUnits=[];for(const row of units.values()){const packages=[...specs].filter(([,sources])=>sources.some(path=>row.sources.includes(path))).map(([name])=>name);assert.ok(packages.length<=1,"Ambiguous provider unit");if(packages.length){executed.add(packages[0]);selectedUnits.push({path:row.path,package:packages[0],sources:row.sources});}}
 for(const name of input.required)assert.ok(specs.has(name)&&executed.has(name),"Required owning package has no actual unit: "+name);return{selected:[...specs.keys()].sort(),executed:[...executed].sort(),uncompiled:[...specs.keys()].filter(name=>!executed.has(name)).sort(),units:selectedUnits.sort((a,b)=>a.path<b.path?-1:1)};
}

async function closureLaws(base:string,ticket:string,out:string){
 const fixture=await json(join(base,"closure-fixtures.json")),schema=await json(join(base,"closure-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true}),validate=ajv.compile(schema);assert.ok(ownSchema(schema,fixture)&&validate(fixture));const outcomes=[];
 const expected={selected:["a","b","c"],executed:["a","b"],uncompiled:["c"],units:[{path:"/fixture/target/a.d",package:"a",sources:["/fixture/snapshot/a.rs","/fixture/target/generated.rs"]},{path:"/fixture/target/b.d",package:"b",sources:["/fixture/snapshot/b.rs"]}]};
 for(const c of fixture.cases){const input=structuredClone(fixture.base);switch(c.mutation){case "missing-optional":input.units=input.units.filter((u:any)=>!u.path.endsWith("b.d"));break;case "required-uncompiled":input.units=input.units.filter((u:any)=>!u.path.endsWith("a.d"));break;case "duplicate-spec":input.specs.push(input.specs[0]);break;case "duplicate-unit":input.units.push(input.units[0]);break;case "ambiguous-unit":input.units[0].sources.push(input.specs[1].sources[0]);break;case "foreign-selected-source":input.specs[0].sources=["/foreign/a.rs"];break;case "empty-required":input.required=[];break;case "unknown-required":input.required=["foreign"];break;case "empty-unit":input.units[0].sources=[];break;case "escaped-selected-root":input.specs[0].sources=["/fixture/snapshot/../foreign.rs"];break;}
  const wanted=c.mutation==="missing-optional"?{...expected,executed:["a"],uncompiled:["b","c"],units:expected.units.slice(0,1)}:expected;let own=false,result=null,error=null;try{result=selectUnitsV1(input);own=true;}catch(e){error=String(e);}const oracle=Boolean(ajv.compile({const:wanted})(result));outcomes.push({...c,own,oracle,result,error,agrees:own===c.accepted&&oracle===c.accepted});
 }
 const result={ready:outcomes.every((c:any)=>c.agrees),outcomes,sourceWritesOutsideTicket:false,nativeExecuted:false};await writeTicket(ticket,join(out,"closure-controls.json"),result);assert.ok(result.ready,"Closed selected-unit controls failed");return result;
}

async function orderLaws(base:string,ticket:string,out:string){const fixture=await json(join(base,"order-fixtures.json")),schema=await json(join(base,"order-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true});assert.ok(ownSchema(schema,fixture)&&ajv.compile(schema)(fixture));const outcomes=[];for(const c of fixture.cases){let accepted=false,paths=null;try{assert.equal(new Set(c.input).size,c.input.length);for(const path of c.input)assert.equal(Buffer.from(path).toString("utf8"),path);paths=[...c.input].sort((a:string,b:string)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));accepted=true;}catch{}const evidence=Boolean(ajv.compile({const:{paths:c.paths,accepted:c.accepted}})({paths,accepted})),oracle=c.accepted&&evidence;outcomes.push({...c,own:accepted,oracle,paths,evidence,agrees:accepted===c.accepted&&oracle===c.accepted&&evidence});}const result={ready:outcomes.every(r=>r.agrees),outcomes,sourceWritesOutsideTicket:false,nativeExecuted:false};await writeTicket(ticket,join(out,"order-controls.json"),result);assert.ok(result.ready);return result;}

async function main(){
 const base=import.meta.dir,ticket=resolve(base,".."),[command]=process.argv.slice(2);assert.ok(["laws-red","laws-green"].includes(command));assert.equal(process.argv.slice(2).length,1);const registrations=await registered(base,command),controller=new AbortController(),context:LiveApplicabilityContextV1={signal:controller.signal,progress:event=>console.log("[DEBUG] "+JSON.stringify(event))};for(const signal of ["SIGINT","SIGTERM"] as const)process.on(signal,()=>controller.abort());const out=join(ticket,"🗑️generated/current-native-live-applicability-8",command);await mkdir(out,{recursive:true});const bindings:Binding[]=[];for(const file of ["📜️script.ts","schema.json","proof-schema.json","fixtures.json","inputs.json","gui.json","project.json","nx.json","package.json","closure-fixtures.json","closure-schema.json","order-fixtures.json","order-schema.json"]){const path=join(base,file);bindings.push({path,sha256:await hashPath(path,context)});}await writeTicket(ticket,join(out,"started.json"),{at:new Date().toISOString(),command,bindings,registrations,sourceWritesOutsideTicket:false});
 if(command.startsWith("laws")){const result={...await laws(base,ticket,out,context),closure:await closureLaws(base,ticket,out),ordering:await orderLaws(base,ticket,out)};for(const b of bindings)assert.equal(await hashPath(b.path,context),b.sha256);await writeTicket(ticket,join(out,"receipt.json"),{...result,bindings,registrations,scope:"Actual SAME normative/adversarial applicability source controls; no native execution or Root source writes"});console.log("[DEBUG] "+JSON.stringify({ready:true,receipt:join(out,"receipt.json")}));return;}
 throw new Error("Only finite method laws are executable here");
}
if(import.meta.main)await main();
