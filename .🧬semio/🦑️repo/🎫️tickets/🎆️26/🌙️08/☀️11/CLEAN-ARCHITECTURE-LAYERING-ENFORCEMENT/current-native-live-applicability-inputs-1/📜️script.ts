import assert from "node:assert/strict";
import {isDeepStrictEqual} from "node:util";
import {createHash} from "node:crypto";
import {createReadStream} from "node:fs";
import {readFile,writeFile,lstat,realpath,readdir,mkdir,unlink,symlink} from "node:fs/promises";
import {createRequire} from "node:module";
import {resolve,relative,isAbsolute,join,dirname,win32} from "node:path";
type Entry={path:string;kind:"file"|"directory";sha256:string|null};
type Binding={path:string;sha256:string};
type Pair={path:string;before:string|null;after:string;beforeHash:string|null;afterHash:string;forward:{start:0;delete:string|null;insert:string};inverse:{start:0;delete:string;insert:string|null}};
export type LiveApplicabilityContextV1={signal?:AbortSignal;progress?:(event:{phase:string;done:number;total:number})=>void};
export type LiveApplicabilityProofV1={version:1;ready:true;root:string;ticket:string;models:Pair[];rootInputs:{path:string;beforeHash:string|null;afterHash:string;model:boolean;reasons:string[]}[];capturedBindings:Binding[];memberships:{path:string;scope:"root"|"immutable";mode:"files"|"tree";before:Entry[];after:Entry[]}[];owners:any[];sourceWritesOutsideTicket:false;nativeExecuted:false;publicationReady:false;scope:string};
const schemaScope="Fresh9 selected live applicability for joint21 publication; no atomicity claim";
const sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
const inside=(base:string,path:string)=>{const r=relative(resolve(base),resolve(path));return !r||!r.startsWith(".."+"/")&&!r.startsWith(".."+String.fromCharCode(92))&&r!==".."&&!isAbsolute(r);};
const relativePath=(path:string)=>assert.ok(path&&!isAbsolute(path)&&!win32.isAbsolute(path)&&!path.split(/[\\/]/u).some(p=>p===".."||p==="."||!p),"Exact relative authority required");
const tick=async(context:LiveApplicabilityContextV1,phase:string,done:number,total:number)=>{context.signal?.throwIfAborted();if(done===total||done%64===0){context.progress?.({phase,done,total});await new Promise<void>(r=>setImmediate(r));context.signal?.throwIfAborted();}};
const absent=async(path:string)=>{try{await lstat(path);return false;}catch(e){if((e as NodeJS.ErrnoException).code==="ENOENT")return true;throw e;}};
const regular=async(path:string)=>{const s=await lstat(path);assert.ok(s.isFile()&&!s.isSymbolicLink(),"Regular file required: "+path);assert.equal(resolve(await realpath(path)),resolve(path),"Linked ancestor refused: "+path);return s;};
const hashPath=async(path:string,context:LiveApplicabilityContextV1={})=>{context.signal?.throwIfAborted();const a=await regular(path),h=createHash("sha256");for await(const chunk of createReadStream(path,{highWaterMark:1048576})){context.signal?.throwIfAborted();h.update(chunk);}const b=await regular(path);assert.equal(a.size,b.size);assert.equal(a.mtimeMs,b.mtimeMs);assert.equal(a.ino,b.ino);assert.equal(a.dev,b.dev);return h.digest("hex");};
const text=async(path:string)=>{const s=await regular(path);assert.ok(s.size<=33554432,"Bounded text authority required");const b=await readFile(path),value=b.toString("utf8");assert.ok(b.equals(Buffer.from(value)),"Exact UTF8 required");return value;};
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
 const rows=await json(join(base,"gui.json")),row=rows.find((r:any)=>r.name==="📥️native-live-applicability-"+command+"-1🧪️"),inputs=await json(join(base,"inputs.json"));assert.ok(row);
 for(const [k,v]of Object.entries(row.env))assert.equal(process.env[k],String(v).replaceAll("${workspaceFolder}",inputs.root));
 const req=createRequire(join(inputs.root,"package.json")),parser=req("json5"),observations=[];
 for(const p of [join(inputs.root,".vscode/launch.json"),join(inputs.root,".vscode/🧩️launch.seed.jsonc")]){const body=await text(p),rows=parser.parse(body).configurations.filter((r:any)=>r.name===row.name);assert.deepEqual(rows,[row]);observations.push({path:p,sha256:sha(body),row});}
 return observations;
}

async function lawFixture(ticket:string,caseRoot:string,phase:string){
 const root=join(caseRoot,"root"),sealed=join(caseRoot,"sealed"),external=join(caseRoot,"external"),models:Pair[]=[];
 for(let i=0;i<21;i++){const path="model/m"+i+".rs",before=i===20?null:"before-"+i,after="after-"+i;models.push({path,before,after,beforeHash:before===null?null:sha(before),afterHash:sha(after),forward:{start:0,delete:before,insert:after},inverse:{start:0,delete:after,insert:before}});if(phase==="after"||before!==null)await writeTicket(ticket,join(root,path),phase==="after"?after:before);}
 await writeTicket(ticket,join(root,"selected.rs"),"selected");await writeTicket(ticket,join(root,"watch/member.svg"),"asset");await writeTicket(ticket,join(sealed,"helper.ts"),"sealed helper");await writeTicket(ticket,join(sealed,"generated.rs"),"generated");await writeTicket(ticket,join(external,"index.js"),"external");
 const rootInputs=[...models.map(m=>({path:m.path,beforeHash:m.beforeHash,afterHash:m.afterHash,model:true,reasons:["model"]})),{path:"selected.rs",beforeHash:sha("selected"),afterHash:sha("selected"),model:false,reasons:["compiler"]},{path:"watch/member.svg",beforeHash:sha("asset"),afterHash:sha("asset"),model:false,reasons:["watch"]}];
 const bindings:Binding[]=[{path:join(sealed,"helper.ts"),sha256:sha("sealed helper")},{path:join(sealed,"generated.rs"),sha256:sha("generated")},{path:join(external,"index.js"),sha256:sha("external")}],owners=[];
 for(const id of ["ui","board","product"]){const planPath=join(sealed,id+"-plan.json"),binaryMetadataPath=join(sealed,id+"-binary.json");await writeTicket(ticket,planPath,id+"plan");await writeTicket(ticket,binaryMetadataPath,id+"binary");bindings.push({path:planPath,sha256:sha(id+"plan")},{path:binaryMetadataPath,sha256:sha(id+"binary")});const joins=[];for(const input of rootInputs){const m=models.find(m=>m.path===input.path),before=m?m.before:input.path==="selected.rs"?"selected":"asset",after=m?m.after:before,initialPath=join(sealed,id,"initial",input.path),modelPath=join(sealed,id,"model",input.path),preparedPath=join(sealed,id,"prepared",input.path);if(before!==null){await writeTicket(ticket,initialPath,before);bindings.push({path:initialPath,sha256:sha(before)});}await writeTicket(ticket,modelPath,after);await writeTicket(ticket,preparedPath,after);bindings.push({path:modelPath,sha256:sha(after)},{path:preparedPath,sha256:sha(after)});joins.push({path:input.path,initialPath,initialHash:input.beforeHash,modelPath,modelHash:sha(after),preparedPath,preparedHash:sha(after)});}
 owners.push({id,planPath,planHash:sha(id+"plan"),binaryMetadataPath,binaryMetadataHash:sha(id+"binary"),selectedPackages:["fixture-"+id],executedPackages:["fixture-"+id],uncompiledPackages:[],depfiles:[],dependencies:[{path:join(sealed,"generated.rs"),sha256:sha("generated"),kind:"generated",rootPath:null},{path:join(root,"selected.rs"),sha256:sha("selected"),kind:"root",rootPath:"selected.rs"}],joins});}
 const before=[{path:"member.svg",kind:"file" as const,sha256:sha("asset")}],immutable=[{path:"index.js",kind:"file" as const,sha256:sha("external")}];
 return{version:1 as const,ready:true as const,root,ticket,models,rootInputs,capturedBindings:bindings,memberships:[{path:"watch",scope:"root" as const,mode:"tree" as const,before,after:before},{path:external,scope:"immutable" as const,mode:"files" as const,before:immutable,after:immutable}],owners,sourceWritesOutsideTicket:false as const,nativeExecuted:false as const,publicationReady:false as const,scope:schemaScope};
}

async function observe(proof:LiveApplicabilityProofV1){
 const nullableHash=async(p:string)=>await absent(p)?null:await hashPath(p),root=[],bindings=[],membership=[];
 for(const r of proof.rootInputs)root.push({path:r.path,sha256:await nullableHash(join(proof.root,r.path))});
 for(const b of proof.capturedBindings)bindings.push({path:b.path,sha256:await nullableHash(b.path)});
 for(const m of proof.memberships)membership.push({path:m.path,entries:await members(m.scope==="root"?join(proof.root,m.path):m.path,m.mode)});
 return{root,bindings,membership};
}

async function laws(base:string,ticket:string,out:string,context:LiveApplicabilityContextV1){
 const fixtures=await json(join(base,"fixtures.json")),schema=await json(join(base,"schema.json")),proofSchema=await json(join(base,"proof-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile(schema),validateProof=ajv.compile(proofSchema);assert.ok(ownSchema(schema,fixtures)&&validate(fixtures));const outcomes=[];
 for(const [i,c]of fixtures.cases.entries()){const phase=c.mutation.includes("after")&&!c.mutation.includes("after-before")?"after":"before",proof=await lawFixture(ticket,join(out,c.id),phase),original=structuredClone(proof);let chosen=phase;
  const overwrite=async(p:string,v:string)=>writeFile(p,v);
  switch(c.mutation){
   case "unrelated-root":await writeTicket(ticket,join(proof.root,"unrelated.rs"),"peer");break;
   case "selected-root":await overwrite(join(proof.root,"selected.rs"),"drift");break;
   case "model-after-before":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].after);break;
   case "model-before-after":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].before!);break;
   case "new-model-created-before":await writeTicket(ticket,join(proof.root,proof.models[20].path),proof.models[20].after);break;
   case "new-model-missing-after":await unlink(join(proof.root,proof.models[20].path));break;
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
   case "model-absence-kind":await mkdir(join(proof.root,proof.models[20].path),{recursive:true});break;
   case "binding-symlink":{const path=proof.capturedBindings[0].path;await unlink(path);await symlink(proof.capturedBindings[1].path,path);break;}
  }
  let own=false,oracle=false,ownError=null,oracleError=null;
  try{await assertLiveApplicabilityV1(proof,chosen as any,context);own=true;}catch(e){ownError=String(e);}
  try{const expected={root:original.rootInputs.map(r=>({path:r.path,sha256:phase==="after"?r.afterHash:r.beforeHash})),bindings:original.capturedBindings,membership:original.memberships.map(m=>({path:m.path,entries:phase==="after"?m.after:m.before}))},observation=await observe(original),contract=ajv.compile({type:"object",additionalProperties:false,required:["phase","proof","observed"],properties:{phase:{const:phase},proof:{const:original},observed:{const:expected}}});oracle=Boolean(validateProof(proof)&&contract({phase:chosen,proof,observed:observation}));}catch(e){oracleError=String(e);}
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
 const s=await lstat(path);assert.ok(s.isDirectory()&&!s.isSymbolicLink());await walk(path,"");return entries.sort((a,b)=>a.path.localeCompare(b.path));
}

/** 🔒️ Performs only proof-driven reads; imported callers supply cancellation and progress. */
export async function assertLiveApplicabilityV1(proof:LiveApplicabilityProofV1,phase:"before"|"after",context:LiveApplicabilityContextV1={}):Promise<{rootInputs:number;bindings:number;memberships:number}>{
 assert.ok(ownSchema(await json(join(import.meta.dir,"proof-schema.json")),proof),"Closed applicability proof required");assert.ok(phase==="before"||phase==="after");
 return{rootInputs:proof.rootInputs.length,bindings:proof.capturedBindings.length,memberships:proof.memberships.length};
}

async function main(){
 const base=import.meta.dir,ticket=resolve(base,".."),[command]=process.argv.slice(2);assert.ok(["laws-red","laws-green","capture"].includes(command));assert.equal(process.argv.slice(2).length,1);const registrations=await registered(base,command),controller=new AbortController(),context:LiveApplicabilityContextV1={signal:controller.signal,progress:event=>console.log("[DEBUG] "+JSON.stringify(event))};for(const signal of ["SIGINT","SIGTERM"] as const)process.on(signal,()=>controller.abort());const out=join(ticket,"🗑️generated/current-native-live-applicability-1",command);await mkdir(out,{recursive:true});const bindings:Binding[]=[];for(const file of ["📜️script.ts","schema.json","proof-schema.json","fixtures.json","inputs.json","gui.json","project.json","nx.json","package.json"]){const path=join(base,file);bindings.push({path,sha256:await hashPath(path,context)});}await writeTicket(ticket,join(out,"started.json"),{at:new Date().toISOString(),command,bindings,registrations,sourceWritesOutsideTicket:false});
 if(command.startsWith("laws")){const result=await laws(base,ticket,out,context);for(const b of bindings)assert.equal(await hashPath(b.path,context),b.sha256);await writeTicket(ticket,join(out,"receipt.json"),{...result,bindings,registrations,scope:"Actual SAME normative/adversarial applicability source controls; no native execution or Root source writes"});console.log("[DEBUG] "+JSON.stringify({ready:true,receipt:join(out,"receipt.json")}));return;}
 throw Error("Fresh capture implementation is awaiting immutable all-three original terminal success");
}
if(import.meta.main)await main();
