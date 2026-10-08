import {assertLiveApplicabilityV1,depfileSourcesV1} from "../current-native-live-applicability-inputs-5/📜️script.ts";
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
const schemaScope="Fresh12 selected live applicability for joint27 publication with negative runtime controls; no atomicity claim";
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
 const rows=await json(join(base,"gui.json")),row=rows.find((r:any)=>r.name==="📥️native-live-applicability-builder-"+command+"-6🧪️"),inputs=await json(join(base,"inputs.json"));assert.ok(row);
 for(const [k,v]of Object.entries(row.env))assert.equal(process.env[k],String(v).replaceAll("${workspaceFolder}",inputs.root));
 const req=createRequire(join(inputs.root,"package.json")),parser=req("json5"),observations=[];
 for(const p of [join(inputs.root,".vscode/launch.json"),join(inputs.root,".vscode/🧩️launch.seed.jsonc")]){const body=await text(p),rows=parser.parse(body).configurations.filter((r:any)=>r.name===row.name);assert.deepEqual(rows,[row]);observations.push({path:p,sha256:sha(body),row});}
 return observations;
}

async function lawFixture(ticket:string,caseRoot:string,phase:string){
 const root=join(caseRoot,"root"),sealed=join(caseRoot,"sealed"),external=join(caseRoot,"external"),models:Pair[]=[];
 for(let i=0;i<26;i++){const path="model/m"+i+".rs",before=i===25?null:"before-"+i,after="after-"+i;models.push({path,before,after,beforeHash:before===null?null:sha(before),afterHash:sha(after),forward:{start:0,delete:before,insert:after},inverse:{start:0,delete:after,insert:before}});if(phase==="after"||before!==null)await writeTicket(ticket,join(root,path),phase==="after"?after:before);}
 await writeTicket(ticket,join(root,"selected.rs"),"selected");await writeTicket(ticket,join(root,"watch/member.svg"),"asset");await writeTicket(ticket,join(sealed,"helper.ts"),"sealed helper");await writeTicket(ticket,join(sealed,"generated.rs"),"generated");await writeTicket(ticket,join(external,"index.js"),"external");
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
   case "negative-file":case "negative-file-after":await writeTicket(ticket,join(proof.root,"bunfig.toml"),"introduced");break;
   case "negative-directory":case "negative-directory-after":await mkdir(join(proof.root,"bunfig.toml"));break;
   case "negative-symlink":case "negative-symlink-after":await symlink(join(proof.root,"selected.rs"),join(proof.root,"bunfig.toml"));break;
   case "negative-model-witness":await writeTicket(ticket,proof.owners[0].joins.find((j:any)=>j.path==="bunfig.toml").modelPath,"introduced");break;
   case "negative-prepared-witness":await writeTicket(ticket,proof.owners[0].joins.find((j:any)=>j.path==="bunfig.toml").preparedPath,"introduced");break;
   case "unrelated-root":await writeTicket(ticket,join(proof.root,"unrelated.rs"),"peer");break;
   case "selected-root":await overwrite(join(proof.root,"selected.rs"),"drift");break;
   case "model-after-before":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].after);break;
   case "model-before-after":await overwrite(join(proof.root,proof.models[0].path),proof.models[0].before!);break;
   case "new-model-created-before":await writeTicket(ticket,join(proof.root,proof.models[25].path),proof.models[25].after);break;
   case "new-model-missing-after":await unlink(join(proof.root,proof.models[25].path));break;
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
   case "model-absence-kind":await mkdir(join(proof.root,proof.models[25].path),{recursive:true});break;
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


/** 🗂️ Observes one explicitly bounded directory membership with regular physical custody. */
async function members(path:string,mode:"files"|"tree",context:LiveApplicabilityContextV1={}):Promise<Entry[]>{
 const entries:Entry[]=[];let done=0;
 const walk=async(path:string,rel:string)=>{context.signal?.throwIfAborted();const s=await lstat(path);assert.ok(!s.isSymbolicLink(),"Linked membership refused: "+path);assert.equal(resolve(await realpath(path)),resolve(path));if(s.isDirectory()){if(mode==="tree"&&rel)entries.push({path:rel,kind:"directory",sha256:null});for(const n of (await readdir(path)).sort())await walk(join(path,n),rel?rel+"/"+n:n);}else{assert.ok(s.isFile());entries.push({path:rel,kind:"file",sha256:await hashPath(path,context)});}await tick(context,"membership",++done,Number.MAX_SAFE_INTEGER);};
 const s=await lstat(path);assert.ok(s.isDirectory()&&!s.isSymbolicLink());await walk(path,"");return entries.sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
}

/** 🦀️ Separates declared provider packages from observed compiled unit sources. */
function selectUnitsV1(input:UnitInputV1):any{
 assert.ok(isAbsolute(input.snapshot)&&resolve(input.snapshot)===input.snapshot);assert.ok(input.required.length);const specs=new Map<string,string[]>(),units=new Map<string,any>(),executed=new Set<string>();
 for(const row of input.specs){assert.ok(row.package&&!specs.has(row.package)&&row.sources.length);for(const path of row.sources)assert.ok(isAbsolute(path)&&resolve(path)===path&&inside(input.snapshot,path));assert.equal(new Set(row.sources).size,row.sources.length);specs.set(row.package,row.sources);}
 for(const row of input.units){assert.ok(isAbsolute(row.path)&&resolve(row.path)===row.path&&!units.has(row.path)&&row.sources.length);assert.equal(new Set(row.sources).size,row.sources.length);for(const path of row.sources)assert.ok(isAbsolute(path)&&resolve(path)===path);units.set(row.path,row);}
 const selectedUnits=[];for(const row of units.values()){const packages=[...specs].filter(([,sources])=>sources.some(path=>row.sources.includes(path))).map(([name])=>name);assert.ok(packages.length<=1,"Ambiguous provider unit");assert.ok(packages.length||!row.sources.some((path:string)=>inside(input.snapshot,path)),"Compiled Root unit outside declared provider graph");if(packages.length){executed.add(packages[0]);selectedUnits.push({path:row.path,package:packages[0],sources:row.sources});}}
 for(const name of input.required)assert.ok(specs.has(name)&&executed.has(name),"Required owning package has no actual unit: "+name);return{selected:[...specs.keys()].sort(),executed:[...executed].sort(),uncompiled:[...specs.keys()].filter(name=>!executed.has(name)).sort(),units:selectedUnits.sort((a,b)=>a.path<b.path?-1:1)};
}

async function closureLaws(base:string,ticket:string,out:string){
 const fixture=await json(join(base,"closure-fixtures.json")),schema=await json(join(base,"closure-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true}),validate=ajv.compile(schema);assert.ok(ownSchema(schema,fixture)&&validate(fixture));const outcomes=[];
 const expected={selected:["a","b","c"],executed:["a","b"],uncompiled:["c"],units:[{path:"/fixture/target/a.d",package:"a",sources:["/fixture/snapshot/a.rs","/fixture/target/generated.rs"]},{path:"/fixture/target/b.d",package:"b",sources:["/fixture/snapshot/b.rs"]}]};
 for(const c of fixture.cases){const input=structuredClone(fixture.base);switch(c.mutation){case "unknown-root-unit":input.units.push({path:"/fixture/target/foreign.d",sources:["/fixture/snapshot/foreign.rs"]});break;case "missing-optional":input.units=input.units.filter((u:any)=>!u.path.endsWith("b.d"));break;case "required-uncompiled":input.units=input.units.filter((u:any)=>!u.path.endsWith("a.d"));break;case "duplicate-spec":input.specs.push(input.specs[0]);break;case "duplicate-unit":input.units.push(input.units[0]);break;case "ambiguous-unit":input.units[0].sources.push(input.specs[1].sources[0]);break;case "foreign-selected-source":input.specs[0].sources=["/foreign/a.rs"];break;case "empty-required":input.required=[];break;case "unknown-required":input.required=["foreign"];break;case "empty-unit":input.units[0].sources=[];break;case "escaped-selected-root":input.specs[0].sources=["/fixture/snapshot/../foreign.rs"];break;}
  const wanted=c.mutation==="missing-optional"?{...expected,executed:["a"],uncompiled:["b","c"],units:expected.units.slice(0,1)}:expected;let own=false,result=null,error=null;try{result=selectUnitsV1(input);own=true;}catch(e){error=String(e);}const authorized=structuredClone(fixture.base);if(c.mutation==="missing-optional")authorized.units=authorized.units.filter((u:any)=>!u.path.endsWith("b.d"));const oracle=Boolean(ajv.compile({type:"object",additionalProperties:false,required:["input","result"],properties:{input:{const:authorized},result:{type:"object",const:wanted}}})({input,result}));outcomes.push({...c,own,oracle,result,error,agrees:own===c.accepted&&oracle===c.accepted});
 }
 const result={ready:outcomes.every((c:any)=>c.agrees),outcomes,sourceWritesOutsideTicket:false,nativeExecuted:false};await writeTicket(ticket,join(out,"closure-controls.json"),result);assert.ok(result.ready,"Closed selected-unit controls failed");return result;
}

/** 🧭️ Resolves the closed literal rerun-watch law against the owning package manifest. */
function watchPathsV4(snapshot:string,manifest:string,body:string):string[]{
 assert.ok(isAbsolute(snapshot)&&resolve(snapshot)===snapshot&&isAbsolute(manifest)&&resolve(manifest)===manifest&&inside(snapshot,manifest));const paths=new Set<string>();for(const line of body.split(/\r?\n/u)){const match=/^cargo(?:::|:)rerun-if-changed=(.*)$/u.exec(line);if(!match)continue;assert.ok(match[1]&&!/[\u0000-\u001f]/u.test(match[1]));const path=isAbsolute(match[1])?resolve(match[1]):resolve(dirname(manifest),match[1]);assert.ok(inside(snapshot,path)&&path!==snapshot,"Watch outside selected captured source refused");paths.add(path);}return [...paths].sort();
}
async function watchLaws(base:string,ticket:string,out:string){
 const fixture=await json(join(base,"watch-fixtures.json")),schema=await json(join(base,"watch-schema.json")),inputs=await json(join(base,"inputs.json")),req=createRequire(join(inputs.root,"package.json")),Ajv=req("ajv/dist/2020.js").default,ajv=new Ajv({strict:true}),validate=ajv.compile(schema);assert.ok(ownSchema(schema,fixture)&&validate(fixture));const outcomes=[];
 for(const c of fixture.cases){let own=false,result=null,error=null;try{result=watchPathsV4(fixture.snapshot,fixture.manifest,c.body);own=true;}catch(e){error=String(e);}const oracle=Boolean(ajv.compile({type:"array",items:{type:"string"},const:c.paths})(result));outcomes.push({...c,own,oracle,result,error,agrees:own===c.accepted&&oracle===c.accepted});}
 const result={ready:outcomes.every((c:any)=>c.agrees),outcomes,sourceWritesOutsideTicket:false,nativeExecuted:false};await writeTicket(ticket,join(out,"watch-controls.json"),result);assert.ok(result.ready,"Closed build-watch controls failed");return result;
}

async function filesUnder(path:string,context:LiveApplicabilityContextV1):Promise<string[]>{
 const paths:string[]=[];let done=0;const walk=async(path:string)=>{const stat=await lstat(path);assert.ok(!stat.isSymbolicLink());if(stat.isDirectory()){assert.equal(await realpath(path),resolve(path));for(const name of (await readdir(path)).sort())await walk(join(path,name));}else{assert.ok(stat.isFile());paths.push(path);}await tick(context,"discover",++done,Number.MAX_SAFE_INTEGER);};await walk(path);return paths;
}

async function capture(base:string,ticket:string,out:string,context:LiveApplicabilityContextV1,helperBindings:Binding[]){
 const inputs=await json(join(base,"inputs.json"));assert.equal(await hashPath(inputs.guardPath,context),inputs.guardHash);assert.equal(await hashPath(inputs.guardReceipt,context),inputs.guardReceiptHash);const bound=new Map<string,string>(),rootReasons=new Map<string,Set<string>>(),externalRoots=new Set<string>(),watchRoots=new Set<string>(),ownerStates:any[]=[];
 const binding=async(path:string,expected?:string)=>{const hash=await hashPath(path,context);if(expected!==undefined)assert.equal(hash,expected,"Captured authority drift: "+path);if(bound.has(path))assert.equal(bound.get(path),hash);bound.set(path,hash);return hash;};
 const select=(path:string,reason:string)=>{relativePath(path);if(!rootReasons.has(path))rootReasons.set(path,new Set());rootReasons.get(path)!.add(reason);};

 const packageRoot=async(path:string)=>{let parent=dirname(path);while(await absent(join(parent,"package.json"))&&await absent(join(parent,"Cargo.toml"))){const next=dirname(parent);assert.notEqual(next,parent,"External package owner missing: "+path);parent=next;}externalRoots.add(parent);};
 let models:Pair[]|null=null;
 for(const id of inputs.owners){
  const ownerRoot=join(inputs.epochRoot,id),planPath=join(ownerRoot,"plan.json"),planHash=await binding(planPath),plan=await json(planPath);assert.equal(plan.sourceRoot,inputs.root);assert.equal(plan.authorityTicketRoot,ticket);
  for(const file of ["terminal.json",inputs.dispatcherFilename]){const path=join(ownerRoot,file),hash=await binding(path),terminal=await json(path);assert.equal(file==="terminal.json"?terminal.owningExitCode:terminal.exitCode,0,"Fresh original owning success required");assert.equal(terminal.refusal,null);assert.equal(terminal.postRefusal,null);assert.equal(terminal.cancelled,false);if(file==="terminal.json")assert.equal(terminal.sourcePostUnavailable,false);for(const b of terminal.bindings??[])if(inside(ticket,b.path))await binding(b.path,b.sha256);assert.equal(await hashPath(path,context),hash);}
  const metadataPath=join(ownerRoot,"metadata.json"),metadataHash=await binding(metadataPath),metadata=await json(metadataPath);assert.equal(metadata.exitCode,0);assert.equal(metadata.refusal,null);const cargoMetadata=JSON.parse(metadata.phases.at(-1).stdout),runtimePath=join(ownerRoot,inputs.runtimeFilename),runtimeHash=await binding(runtimePath),runtime=await json(runtimePath);assert.equal(runtime.ready,true);assert.deepEqual(runtime.gaps,[]);assert.equal(runtime.importFailure,null);assert.equal(runtime.planHash,planHash);const releasePath=join(ownerRoot,"independent-release.json");await binding(releasePath);const release=await json(releasePath);assert.equal(release.ready,true);
  const core=(m:any)=>({path:m.path,before:m.before,after:m.after,beforeHash:m.beforeHash,afterHash:m.afterHash,forward:m.forward,inverse:m.inverse});const current=plan.modelDerivation.pairs.map(core);assert.equal(current.length,27);if(models)assert.deepEqual(current,models);else models=current;for(const m of current)select(m.path,"joint-model");
  const prepared=new Map<string,any>(plan.checks.map((r:any)=>[r.path,r]));for(const path of inputs.rootControls)select(path,"owning-runtime-control");for(const d of [...plan.providers,...plan.declarations.filter((d:any)=>d.selected)])select(d.path,"selected-provider-declaration");select(plan.ownerWitness.scriptRelative,"owning-TypeScript");for(const r of runtime.loaded){if(inside(plan.snapshot,r.path))select(relative(plan.snapshot,r.path),"actual-TypeScript-runtime");else{await binding(r.path,r.sha256);await packageRoot(r.path);}}
  for(const p of runtime.packages)externalRoots.add(p.base);for(const b of runtime.packageBodies){await binding(b.path,b.sha256);await packageRoot(b.path);}for(const b of plan.bindings){if(inside(ticket,b.path))await binding(b.path,b.sha256);else{await binding(b.path,b.sha256);await packageRoot(b.path);}}
  const rootPackages=cargoMetadata.packages.filter((p:any)=>p.source===null&&inside(plan.snapshot,p.manifest_path)),providerNames=new Set(plan.providers.map((p:any)=>p.package)),specs=rootPackages.filter((p:any)=>providerNames.has(p.name)).map((p:any)=>({package:p.name,sources:p.targets.map((t:any)=>t.src_path)}));assert.equal(specs.length,providerNames.size);for(const spec of specs)for(const path of spec.sources)select(relative(plan.snapshot,path),"declared-provider-target (execution separately observed)");
  const artifactFiles=await filesUnder(join(ownerRoot,"target"),context),units=[];for(const path of artifactFiles.filter(p=>p.endsWith(".d"))){const body=await text(path),sources=depfileSourcesV1(body).map(p=>isAbsolute(p)?resolve(p):resolve(plan.snapshot,p));units.push({path,sources});}
  const selected=selectUnitsV1({snapshot:plan.snapshot,specs,units,required:[plan.route.package??plan.providers.find((p:any)=>p.path===plan.route.manifest)?.package]});const deps=new Map<string,any>(),depfiles=[];
  for(const unit of selected.units){externalRoots.add(dirname(unit.path));const sha256=await binding(unit.path);depfiles.push({path:unit.path,sha256,package:unit.package});for(const path of unit.sources){const hash=await binding(path);let kind="generated",rootPath=null;if(inside(plan.snapshot,path)){kind="root";rootPath=relative(plan.snapshot,path);select(rootPath,"actual-Rust-depfile");assert.equal(prepared.get(rootPath)?.expected,hash);}else if(!inside(ticket,path)){kind="external";await packageRoot(path);}deps.set(path,{path,sha256:hash,kind,rootPath});}}
  const packageByName=new Map(rootPackages.map((p:any)=>[p.name,p]));for(const path of artifactFiles.filter(p=>p.endsWith(join("run","stdout"))||p.endsWith("/output"))){const body=await text(path),packageName=selected.executed.find((name:string)=>relative(join(ownerRoot,"target/debug/build"),path).startsWith(name+"/")||relative(join(ownerRoot,"target/debug/build"),path).startsWith(name+"-"));if(!packageName)continue;externalRoots.add(dirname(path));await binding(path);for(const source of watchPathsV4(plan.snapshot,(packageByName.get(packageName) as any).manifest_path,body)){const rel=relative(plan.snapshot,source),stat=await lstat(source);if(stat.isDirectory())watchRoots.add(rel);else select(rel,"actual-Rust-build-watch");}}
  const binariesFiles=await filesUnder(join(ownerRoot,"artifacts"),context).catch((e:any)=>{if(e.code==="ENOENT")return [];throw e;}),binaryMetadataCandidates=binariesFiles.filter(p=>p.endsWith("/binaries-metadata.json"));assert.ok(binaryMetadataCandidates.length<=1);
  let binaryMetadataPath:string,binaryMetadataHash:string;if(binaryMetadataCandidates.length){binaryMetadataPath=binaryMetadataCandidates[0];externalRoots.add(dirname(binaryMetadataPath));binaryMetadataHash=await binding(binaryMetadataPath);const binaries=await json(binaryMetadataPath);assert.equal(binaries["rust-build-meta"]["target-directory"],join(ownerRoot,"target"));for(const row of Object.values<any>(binaries["rust-binaries"])){assert.ok(selected.executed.includes(row["package-id"]?.split("#").at(-1)?.split("@")[0]));await binding(row["binary-path"]);}for(const [pkg,rows] of Object.entries<any>(binaries["rust-build-meta"]["non-test-binaries"])){if(selected.executed.includes(pkg.split("#").at(-1)!.split("@")[0]))for(const row of rows)await binding(resolve(join(ownerRoot,"target"),row.path));}}else{binaryMetadataPath=join(out,id+"-compiled-source-index.json");await writeTicket(ticket,binaryMetadataPath,{version:1,derived:true,scope:"Actual fresh Cargo owning terminal plus selected physical depfiles; no invented nextest metadata",planHash,metadataPath,metadataHash,selected});binaryMetadataHash=await binding(binaryMetadataPath);}
  ownerStates.push({id,ownerRoot,planPath,planHash,plan,runtimePath,runtimeHash,metadataPath,metadataHash,binaryMetadataPath,binaryMetadataHash,snapshotRoot:plan.snapshot,selectedPackages:selected.selected,executedPackages:selected.executed,uncompiledPackages:selected.uncompiled,depfiles,dependencies:[...deps.values()].sort((a,b)=>a.path<b.path?-1:1)});
 }
 for(const state of ownerStates){let grew=true;while(grew){grew=false;for(const include of state.plan.declaredIncludes){if(include.exists&&rootReasons.has(include.from)&&!rootReasons.has(include.path)){select(include.path,"explicit-selected-literal-include (not inferred macro execution)");grew=true;}}}}
 externalRoots.add(base);externalRoots.add(dirname(inputs.guardPath));
 const memberships:any[]=[];assert.ok(watchRoots.size>0,"Actual selected build-watch membership is required");const canonical=ownerStates[0];for(const rel of [...watchRoots].sort()){const before=await members(join(canonical.plan.initialSnapshot,rel),"tree",context),after=await members(join(canonical.plan.modelSnapshot,rel),"tree",context);for(const r of [...before,...after])if(r.kind==="file")select(join(rel,r.path),"watched-directory-member");for(const owner of ownerStates){assert.deepEqual(await members(join(owner.plan.initialSnapshot,rel),"tree",context),before);assert.deepEqual(await members(join(owner.plan.modelSnapshot,rel),"tree",context),after);}memberships.push({path:rel,scope:"root",mode:"tree",before,after});}
 for(const path of [...externalRoots].sort()){const entries=await members(path,"tree",context);for(const row of entries)if(row.kind==="file")await binding(join(path,row.path),row.sha256!);memberships.push({path,scope:"immutable",mode:"tree",before:entries,after:entries});}
 const modelMap=new Map(models!.map(m=>[m.path,m])),rootInputs:any[]=[];for(const path of [...rootReasons.keys()].sort()){const m=modelMap.get(path),initialRows=new Map<string,any>(canonical.plan.initialChecks.map((r:any)=>[r.path,r])),beforeHash=initialRows.get(path)?.expected??null,afterHash=m?.afterHash??beforeHash;if(!m&&beforeHash===null)for(const owner of ownerStates)assert.ok(await absent(join(owner.plan.initialSnapshot,path))&&await absent(join(owner.plan.modelSnapshot,path))&&await absent(join(owner.plan.snapshot,path)));rootInputs.push({path,beforeHash,afterHash,model:Boolean(m),reasons:[...rootReasons.get(path)!].sort()});}
 const owners=[];for(const state of ownerStates){const initial=new Map<string,any>(state.plan.initialChecks.map((r:any)=>[r.path,r])),model=new Map<string,any>(state.plan.modelChecks.map((r:any)=>[r.path,r])),prepared=new Map<string,any>(state.plan.checks.map((r:any)=>[r.path,r])),joins=[];for(const r of rootInputs){const initialHash=initial.get(r.path)?.expected??null,modelHash=model.get(r.path)?.expected??null,preparedHash=prepared.get(r.path)?.expected??null;assert.equal(initialHash,r.beforeHash);assert.equal(modelHash,r.afterHash);const initialPath=join(state.plan.initialSnapshot,r.path),modelPath=join(state.plan.modelSnapshot,r.path),preparedPath=join(state.plan.snapshot,r.path);if(initialHash!==null)await binding(initialPath,initialHash);if(modelHash!==null)await binding(modelPath,modelHash);if(preparedHash!==null)await binding(preparedPath,preparedHash);joins.push({path:r.path,initialPath,initialHash,modelPath,modelHash,preparedPath,preparedHash});}owners.push({id:state.id,planPath:state.planPath,planHash:state.planHash,binaryMetadataPath:state.binaryMetadataPath,binaryMetadataHash:state.binaryMetadataHash,snapshotRoot:state.snapshotRoot,selectedPackages:state.selectedPackages,executedPackages:state.executedPackages,uncompiledPackages:state.uncompiledPackages,depfiles:state.depfiles,dependencies:state.dependencies,joins});}
 for(const b of helperBindings)await binding(b.path,b.sha256);const lawReceipt=join(ticket,"🗑️generated/current-native-live-applicability-5/laws-green/receipt.json");await binding(lawReceipt);const law=await json(lawReceipt);assert.equal(law.ready,true);for(const b of law.bindings)await binding(b.path,b.sha256);assert.equal(law.closure.ready,true);assert.equal(await hashPath(inputs.guardPath,context),inputs.guardHash);const builderLawPath=join(ticket,"🗑️generated/current-native-live-applicability-builder-6/laws-green/receipt.json");await binding(builderLawPath);const builderLaw=await json(builderLawPath);assert.equal(builderLaw.ready,true);assert.equal(builderLaw.watch.ready,true);for(const b of builderLaw.bindings)await binding(b.path,b.sha256);
 const proof:LiveApplicabilityProofV1={version:1,ready:true,root:inputs.root,ticket,models:models!,rootInputs,capturedBindings:[...bound].map(([path,sha256])=>({path,sha256})).sort((a,b)=>a.path<b.path?-1:1),memberships,owners,sourceWritesOutsideTicket:false,nativeExecuted:false,publicationReady:false,scope:schemaScope};const result=await assertLiveApplicabilityV1(proof,"before",context);await writeTicket(ticket,inputs.proofOutput,proof);await writeTicket(ticket,join(out,"receipt.json"),{at:new Date().toISOString(),ready:true,proofPath:inputs.proofOutput,proofHash:await hashPath(inputs.proofOutput,context),result,sourceWritesOutsideTicket:false,nativeExecuted:false,publicationReady:false});return result;
}

async function main(){
 const base=import.meta.dir,ticket=resolve(base,".."),[command]=process.argv.slice(2);assert.ok(["laws-red","laws-green","capture"].includes(command));assert.equal(process.argv.slice(2).length,1);const registrations=await registered(base,command),controller=new AbortController(),context:LiveApplicabilityContextV1={signal:controller.signal,progress:event=>console.log("[DEBUG] "+JSON.stringify(event))};for(const signal of ["SIGINT","SIGTERM"] as const)process.on(signal,()=>controller.abort());const out=join(ticket,"🗑️generated/current-native-live-applicability-builder-6",command);await mkdir(out,{recursive:true});const bindings:Binding[]=[];for(const file of ["📜️script.ts","schema.json","proof-schema.json","fixtures.json","inputs.json","gui.json","project.json","nx.json","package.json","closure-fixtures.json","closure-schema.json","watch-fixtures.json","watch-schema.json"]){const path=join(base,file);bindings.push({path,sha256:await hashPath(path,context)});}const inputs=await json(join(base,"inputs.json"));for(const [path,sha256]of [[inputs.guardPath,inputs.guardHash],[inputs.guardReceipt,inputs.guardReceiptHash]]){assert.equal(await hashPath(path,context),sha256);bindings.push({path,sha256});}await writeTicket(ticket,join(out,"started.json"),{at:new Date().toISOString(),command,bindings,registrations,sourceWritesOutsideTicket:false});
 if(command.startsWith("laws")){const result={ready:true,closure:await closureLaws(base,ticket,out),watch:await watchLaws(base,ticket,out)};for(const b of bindings)assert.equal(await hashPath(b.path,context),b.sha256);await writeTicket(ticket,join(out,"receipt.json"),{...result,bindings,registrations,scope:"Actual SAME normative/adversarial applicability source controls; no native execution or Root source writes"});console.log("[DEBUG] "+JSON.stringify({ready:true,receipt:join(out,"receipt.json")}));return;}
 console.log("[DEBUG] "+JSON.stringify(await capture(base,ticket,out,context,bindings)));
}
if(import.meta.main)await main();
