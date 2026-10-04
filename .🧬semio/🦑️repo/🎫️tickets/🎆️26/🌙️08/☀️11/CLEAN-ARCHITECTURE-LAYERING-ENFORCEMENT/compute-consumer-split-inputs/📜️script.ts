#!/usr/bin/env bun
/** 📐️ Authors guarded compute deletion and consumer ownership source cohorts. */
import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {join,dirname,resolve,relative} from "node:path";
import * as ts from "typescript";
const ticket=resolve(import.meta.dir,".."),root=resolve(ticket,"../../../../../../.."),compute="🧰️framework/🔨️modules/◻️2d/🧮️compute",hash=(value:string|null)=>value===null?null:new Bun.CryptoHasher("sha256").update(value).digest("hex");
process.chdir(root);
type Row={path:string;before:string|null;beforeHash:string|null;authored:string;authoredHash:string;inverse:{before:string;after:string|null}};
const read=(path:string)=>existsSync(path)?readFileSync(path,"utf8"):null,pair=(path:string,authored:string):Row=>{const before=read(path);return {path,before,beforeHash:hash(before),authored,authoredHash:hash(authored)!,inverse:{before:authored,after:before}};};
const splice=(source:string,before:string,after:string)=>{if(source.split(before).length!==2)throw Error("Nonunique owned splice");return source.replace(before,after);},json=(value:unknown)=>JSON.stringify(value,null,2)+"\n";
function addTarget(path:string,name:string,target:unknown){const source=read(path)!,parsed=JSON.parse(source);if(parsed.targets[name])throw Error("Duplicate target "+name);const at=source.indexOf('"targets": {')+'"targets": {'.length;return pair(path,source.slice(0,at)+'\n    '+JSON.stringify(name)+": "+JSON.stringify(target,null,2).split("\n").join("\n    ")+","+source.slice(at));}
function addPackage(path:string,name:string,project:string){const source=read(path)!,at=source.indexOf('"scripts": {')+'"scripts": {'.length;if(JSON.parse(source).scripts[name])throw Error("Duplicate script");return pair(path,source.slice(0,at)+'\n    '+JSON.stringify(name)+": "+JSON.stringify("nx run "+project+":"+name)+","+source.slice(at));}
const rootImport=(path:string,target:string)=>{const result=relative(dirname(path),target).split("\\").join("/");return result.startsWith(".")?result:"./"+result;};
const save=(name:string,rows:Row[])=>{mkdirSync(import.meta.dir,{recursive:true});writeFileSync(join(import.meta.dir,name+".json"),JSON.stringify({at:new Date().toISOString(),sourceWrites:false,rows}));console.log(JSON.stringify({stage:name,rows:rows.length}));};
async function prepareBoundary(){
 const rows:Row[]=[],testPath=compute+"/🧪️tests/📍️ownership/🧭️direction/🟦️.ts",fixturePath=compute+"/🧫️fixtures/📍️ownership/🧭️direction/🔣️.json",schemaPath=compute+"/🧬️schema/📍️ownership/🧭️direction/🔣️.json";
 const descriptor={version:1,keys:["version","source","unit","retainedUnitSha256","family","laws","cacheBehavior"],neutralPrefix:compute,absent:["🧰️framework/🛍️products","✏️s","🌎️hub"],copied:[compute+"/🧪️tests/📍️ownership/🟦️.ts",compute+"/🧫️fixtures/📍️ownership/🔣️.json",compute+"/🧬️schema/📍️ownership/🔣️.json",compute+"/🦀️.rs",compute+"/🧪️tests/🔬️unit/🦀️.rs","🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs","🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml"]};
 const schema={$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/framework/2d/compute/ownership/direction.json",type:"object",additionalProperties:false,required:Object.keys(descriptor),properties:Object.fromEntries(Object.entries(descriptor).map(([name,value])=>[name,{const:value}]))};
 rows.push(pair(fixturePath,json(descriptor)),pair(schemaPath,json(schema)));
 const source=[
 '/** 🧭️ Proves the actual compute witness executes without higher owners. */',
 'import {expect,test} from "bun:test";',
 'import Ajv from "ajv";',
 'import * as ts from "typescript";',
 'import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,existsSync} from "node:fs";',
 'import {resolve,join,dirname} from "node:path";',
 'import {spawnSync} from "node:child_process";',
 'import law from '+JSON.stringify(rootImport(testPath,fixturePath))+';',
 'import lawSchema from '+JSON.stringify(rootImport(testPath,schemaPath))+';',
 'const root=resolve(import.meta.dir,'+JSON.stringify(relative(dirname(testPath),".")||".")+'),read=(path:string)=>readFileSync(join(root,path),"utf8"),owner='+JSON.stringify(compute)+';',
 'test("closed neutral compute ownership contains only portable fields",()=>{',
 ' const admit=new Ajv({strict:true}).compile(lawSchema);expect(admit(law)).toBe(true);expect(admit({...law,extra:true})).toBe(false);',
 ' const fixture=JSON.parse(read(owner+"/🧫️fixtures/📍️ownership/🔣️.json")),schema=JSON.parse(read(owner+"/🧬️schema/📍️ownership/🔣️.json"));',
 ' expect(Object.keys(fixture).sort()).toEqual([...law.keys].sort());expect(Object.keys(schema.properties).sort()).toEqual([...law.keys].sort());expect([...schema.required].sort()).toEqual([...law.keys].sort());',
 ' for(const path of [fixture.source,fixture.unit])expect(path.startsWith(law.neutralPrefix+"/")).toBe(true);',
 '});',
 'test("neutral compute witness and task inputs do not depend on higher owners",()=>{',
 ' const path=owner+"/🧪️tests/📍️ownership/🟦️.ts",source=ts.createSourceFile(path,read(path),ts.ScriptTarget.Latest,true);',
 ' const literals:string[]=[];const visit=(node:ts.Node)=>{if(ts.isStringLiteral(node))literals.push(node.text);ts.forEachChild(node,visit);};visit(source);',
 ' for(const value of literals)for(const higher of law.absent)expect(value.includes(higher),value).toBe(false);',
 ' const project=JSON.parse(read(owner+"/📋️project.json"));for(const input of project.targets["test-ownership"].inputs)if(typeof input==="string")for(const higher of law.absent)expect(input.includes(higher),input).toBe(false);',
 '});',
 'test("deleting all higher owners leaves the actual neutral compute witness executable",()=>{',
 ' const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned output required");mkdirSync(output,{recursive:true});const sandbox=mkdtempSync(join(output,"compute-neutral-deletion-"));',
 ' for(const path of law.copied){const destination=join(sandbox,path);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,read(path));}',
 ' for(const path of law.absent)expect(existsSync(join(sandbox,path))).toBe(false);',
 ' const node=spawnSync("node",["--eval","const fs=require(\'node:fs\');const data=JSON.parse(fs.readFileSync(0,\'utf8\'));process.stdout.write(JSON.stringify(data.absent.map(p=>fs.existsSync(data.root+\'/\'+p))));"],{encoding:"utf8",timeout:30000,input:JSON.stringify({root:sandbox,absent:law.absent})});expect(node.status,node.stderr).toBe(0);expect(JSON.parse(node.stdout)).toEqual(law.absent.map(()=>false));',
 ' const child=spawnSync(process.execPath,["test",join(sandbox,law.copied[0])],{cwd:sandbox,encoding:"utf8",timeout:30000,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:join(sandbox,"output")}});',
 ' writeFileSync(join(output,"compute-neutral-deletion-runtime.json"),JSON.stringify({status:child.status,signal:child.signal,stdout:child.stdout,stderr:child.stderr,sandbox,absent:law.absent}));expect(child.status,child.stderr).toBe(0);expect(child.stderr).toContain("3 pass");',
 '});'
 ].join("\n")+"\n";
 rows.push(pair(testPath,source));
 const scriptPath=compute+"/📜️script.ts",script=read(scriptPath)!;
 rows.push(pair(scriptPath,splice(splice(script,'!["schema", "ownership"].includes(segments[0]!)','!["schema", "ownership", "neutral-ownership"].includes(segments[0]!)'),'"🧪️tests/📍️ownership/🟦️.ts"','segments[0] === "ownership" ? "🧪️tests/📍️ownership/🟦️.ts" : "🧪️tests/📍️ownership/🧭️direction/🟦️.ts"')));
 rows.push(addTarget(compute+"/📋️project.json","test-neutral-ownership",{executor:"nx:run-commands",cache:false,dependsOn:[],outputs:[],inputs:["sharedGlobals","{projectRoot}/**/*","{workspaceRoot}/🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/**/*"],options:{cwd:compute,command:"bun ./📜️script.ts test neutral-ownership"}}));
 rows.push(addPackage(compute+"/package.json","test-neutral-ownership","@semio-tech/2d-compute"));
 save("boundary-test-first-ready-1",rows);
}
async function mount(name:string){
 const rows:Row[]=JSON.parse(readFileSync(join(import.meta.dir,name+".json"),"utf8")).rows;
 const gui=rows.find(row=>row.path===".vscode/🧩️launch.seed.jsonc");
 if(gui&&read(gui.path)!==gui.before){const current=read(gui.path)!,needle='"name": "🧪️test◻️2d🧮️compute📍️ownership"',start=gui.before!.lastIndexOf("\n    {",gui.before!.indexOf(needle))+1,next=gui.before!.indexOf("\n    {",start+1),anchor=gui.before!.slice(start,next),delta=gui.authored.length-gui.before!.length,addition=gui.authored.slice(start,start+delta);if(current.split(anchor).length!==2||current.includes('"name": "◻️2d compute neutral deletion"'))throw Error("Changed owned GUI anchor");Object.assign(gui,pair(gui.path,current.replace(anchor,addition+anchor)));}
 const gaps=rows.filter(r=>read(r.path)!==r.before).map(r=>r.path);if(gaps.length)throw Error("Owned guard gaps "+JSON.stringify(gaps));
 writeFileSync(join(import.meta.dir,name+"-publication-before.json"),JSON.stringify({at:new Date().toISOString(),rows,inverse:rows.map(r=>({path:r.path,...r.inverse}))}));
 for(const row of rows){mkdirSync(dirname(row.path),{recursive:true});writeFileSync(row.path,row.authored);}
 const after=rows.map(r=>({path:r.path,source:read(r.path),sha256:hash(read(r.path)),inverse:r.inverse}));if(after.some((r,i)=>r.source!==rows[i].authored))throw Error("Publication mismatch");
 writeFileSync(join(import.meta.dir,name+"-mounted.json"),JSON.stringify({at:new Date().toISOString(),rows:after}));console.log(JSON.stringify({mounted:rows.length,gaps:0}));
}
async function prepareConsumers(){
 const rows:Row[]=[],helper=compute+"/🧪️testing/📍️consumer/🟦️.ts",runner=compute+"/🧪️testing/📍️consumer/🏃️execution/🟦️.ts",host=compute+"/🧪️testing/📍️consumer/🏃️host/🟦️.ts",genericSchema=compute+"/🧬️schema/📍️consumer/🔣️.json",validator="🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
 const oldFixturePath=compute+"/🧫️fixtures/📍️ownership/🔣️.json",oldSchemaPath=compute+"/🧬️schema/📍️ownership/🔣️.json",oldTestPath=compute+"/🧪️tests/📍️ownership/🟦️.ts",oldFixture=JSON.parse(read(oldFixturePath)!),oldSchema=JSON.parse(read(oldSchemaPath)!),family=oldFixture.family as string[];
 const schema={$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/framework/2d/compute/consumer.json",type:"object",additionalProperties:false,required:["version","id","source","manifest","bindings"],properties:{version:{const:1},id:{type:"string",minLength:1},source:{type:"string",minLength:1},manifest:{type:"string",minLength:1},bindings:{type:"array",uniqueItems:true,minItems:1,items:{enum:family}}}};
 rows.push(pair(genericSchema,json(schema)));
 rows.push(pair(helper,[
 '/** 🔗️ Defines the compute family owned by a consumer witness. */',
 'export type ComputeFamilyName='+family.map(x=>JSON.stringify(x)).join("|")+";",
 '/** 📍️ A caller-owned source and direct manifest binding. */',
 'export interface ComputeConsumerDescriptor{version:1;id:string;source:string;manifest:string;bindings:readonly ComputeFamilyName[];}',
 '/** 📖️ Reads only the caller-declared source coordinates. */',
 'export interface ComputeConsumerAccess{read(path:string):string;dependencies(manifest:string):readonly string[];}',
 'import schema from '+JSON.stringify(rootImport(helper,genericSchema))+';',
 'import {validateJsonSchemaSubset} from '+JSON.stringify(rootImport(helper,validator))+';',
 '/** ✅️ Verifies direct canonical bindings without discovering any product owner. */',
 'export function verifyComputeConsumer(descriptor:ComputeConsumerDescriptor,access:ComputeConsumerAccess):{directDependency:true;bindings:readonly ComputeFamilyName[]}{',
 ' if(validateJsonSchemaSubset(schema,descriptor).length)throw Error("Invalid compute consumer descriptor");',
 ' const source=access.read(descriptor.source);',
 ' for(const name of descriptor.bindings)if(!source.includes("semio_framework_2d::compute::"+name))throw Error("Missing compute binding "+name);',
 ' if(/\\b(?:store|semio_framework_os_kernel)::Engine(?:Handles|Cache|Key|Handle|Fault|Rep)?\\b|\\bKernelEngineHandle\\b/.test(source))throw Error("Foreign compute facade");',
 ' if(!access.dependencies(access.read(descriptor.manifest)).includes("semio-framework-2d"))throw Error("Missing direct compute dependency");',
 ' return {directDependency:true,bindings:[...descriptor.bindings]};',
 '}',""
 ].join("\n")));
 const bundle="🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts",execution="🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts",budget="🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
 rows.push(pair(runner,[
 '/** 🧪️ Runs an explicitly declared compute source law with strict types and the normal budget. */',
 'import {resolve} from "node:path";',
 'import {BundleScript} from '+JSON.stringify(rootImport(runner,bundle))+';',
 'import {runBudgetedTestCommand} from '+JSON.stringify(rootImport(runner,execution))+';',
 'import {testLevelBudgetMs} from '+JSON.stringify(rootImport(runner,budget))+';',
 'export abstract class ComputeOwnershipTestScript extends BundleScript{',
 ' abstract readonly source:string;',
 ' async run(segments:string[]):Promise<void>{',
 '  if(segments.length)throw Error("Compute ownership law accepts no arguments");if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Caller-owned test output required");',
 '  const source=resolve(this.root,this.source),options={cwd:this.repoRoot,env:process.env,budgetMs:testLevelBudgetMs(),throwOnFailure:true};',
 '  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],options);',
 '  await runBudgetedTestCommand(process.execPath,["test",source],options);',
 ' }',
 '}',""
 ].join("\n")));
 rows.push(pair(host,[
 '/** 🧫️ Registers the complete caller-owned compute witness with independent test oracles. */',
 'import {expect,test} from "bun:test";',
 'import Ajv from "ajv";',
 'import * as TOML from "@iarna/toml";',
 'import {verifyComputeConsumer,type ComputeConsumerDescriptor} from '+JSON.stringify(rootImport(host,helper))+';',
 'import {validateJsonSchemaSubset,requireRecord,type UnknownRecord} from '+JSON.stringify(rootImport(host,validator))+';',
 'const dependencies=(source:string):string[]=>{const parsed=requireRecord(Bun.TOML.parse(source),"manifest");return Object.keys(parsed.dependencies===undefined?{}:requireRecord(parsed.dependencies,"dependencies"));};',
 'export function registerComputeConsumerTests(descriptor:ComputeConsumerDescriptor,schema:UnknownRecord,read:(path:string)=>string):void{',
 ' test("closed compute consumer descriptor agrees with independent schema admission",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(descriptor)).toBe(true);expect(validateJsonSchemaSubset(schema,descriptor)).toEqual([]);for(const hostile of [{...descriptor,extra:true},{...descriptor,bindings:[]},{...descriptor,bindings:["ForeignEngine"]}]){expect(validate(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}});',
 ' test("actual consumer directly binds the canonical compute family",()=>{',
 '  const source=read(descriptor.source);for(const name of descriptor.bindings)expect(source).toContain("semio_framework_2d::compute::"+name);expect(source).not.toMatch(/\\b(?:store|semio_framework_os_kernel)::Engine(?:Handles|Cache|Key|Handle|Fault|Rep)?\\b|\\bKernelEngineHandle\\b/);',
 '  const manifest=read(descriptor.manifest),native=dependencies(manifest),oracle=TOML.parse(manifest);expect(native).toEqual(Object.keys(oracle.dependencies??{}));expect(Object.keys(oracle.dependencies??{}),descriptor.id).toContain("semio-framework-2d");',
 '  expect(verifyComputeConsumer(descriptor,{read,dependencies})).toEqual({directDependency:true,bindings:descriptor.bindings});',
 ' });',
 ' test("a declared but broken owner refuses instead of skipping",()=>{for(const missing of [descriptor.source,descriptor.manifest])expect(()=>verifyComputeConsumer(descriptor,{read:path=>{if(path===missing)throw Error("Missing owned source");return read(path);},dependencies})).toThrow("Missing owned source");});',
 '}',""
 ].join("\n")));
 const consumerRows=oldFixture.consumers as {id:string;source:string;manifest:string;bindings:string[]}[];
 const projects:{name:string;target:string}[]=[];
 for(const original of consumerRows){
  const packageRoot=dirname(original.manifest),ownerRoot=resolve(packageRoot,"../..").slice(root.length+1),witnessRoot=ownerRoot+"/🏛️ownership/🧮️compute",fixturePath=witnessRoot+"/🧫️fixtures/🔣️.json",schemaPath=witnessRoot+"/🧬️schema/🔣️.json",testPath=witnessRoot+"/🧪️tests/🟦️.ts",descriptor={version:1,...original};
  rows.push(pair(fixturePath,json(descriptor)),pair(schemaPath,json({...schema,$id:"https://json.schemas.assets.semio-tech.com/compute-consumer/"+original.id+".json",properties:Object.fromEntries(Object.entries(descriptor).map(([name,value])=>[name,{...schema.properties[name as keyof typeof schema.properties],const:value}]))})));
  rows.push(pair(testPath,[
   '/** 🧩️ Executes this owner\'s complete canonical compute witness. */',
   'import {readFileSync} from "node:fs";','import {resolve,join} from "node:path";',
   'import {registerComputeConsumerTests} from '+JSON.stringify(rootImport(testPath,host))+';',
   'import {type ComputeConsumerDescriptor} from '+JSON.stringify(rootImport(testPath,helper))+';',
   'import descriptor from '+JSON.stringify(rootImport(testPath,fixturePath))+';',
   'import schema from '+JSON.stringify(rootImport(testPath,schemaPath))+';',
   'const root=resolve(import.meta.dir,'+JSON.stringify(relative(dirname(testPath),".")||".")+');',
   'registerComputeConsumerTests(descriptor as ComputeConsumerDescriptor,schema,path=>readFileSync(join(root,path),"utf8"));',""
  ].join("\n")));
  const scriptPath=packageRoot+"/📜️script.ts",projectPath=packageRoot+"/📋️project.json",packagePath=packageRoot+"/package.json",command="test-compute-consumer";
  const script=read(scriptPath),declaration='/** 🎯️ Verifies the owner-declared canonical compute dependency. */\nclass ComputeConsumerScript extends ComputeOwnershipTestScript{readonly source='+JSON.stringify(rootImport(scriptPath,testPath))+';}\n\n',importLine='import {ComputeOwnershipTestScript} from '+JSON.stringify(rootImport(scriptPath,runner))+';\n';
  let authored:string,name:string;
  if(script===null){name="@semio-tech/framework-os-flow";authored='#!/usr/bin/env bun\n'+importLine+'import {ScriptRouter} from '+JSON.stringify(rootImport(scriptPath,bundle))+';\nimport {runScriptMain} from '+JSON.stringify(rootImport(scriptPath,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts"))+';\n'+declaration+'await runScriptMain(new ScriptRouter(import.meta.dir).register('+JSON.stringify(command)+',ComputeConsumerScript),{defaultCommand:'+JSON.stringify(command)+'});\n';}
  else{
   name=JSON.parse(read(projectPath)!).name;const at=script.indexOf("\n")+1;authored=script.slice(0,at)+importLine+script.slice(at);
   if(original.id==="puzzle-brush-cache"){authored=splice(authored,"await runArtifactRustPackageMain(",declaration+"await runArtifactRustPackageMain(");authored=splice(authored,"commands:{","commands:{"+JSON.stringify(command)+":ComputeConsumerScript,");}
   else{const marker=original.id==="host-component-fixture"?"await runScriptMain(":"const router =";authored=splice(authored,marker,declaration+marker);authored=splice(authored,"new ScriptRouter(import.meta.dir)","new ScriptRouter(import.meta.dir).register("+JSON.stringify(command)+",ComputeConsumerScript)");}
  }
  rows.push(pair(scriptPath,authored));
  const target={executor:"nx:run-commands",cache:false,dependsOn:[],outputs:[],inputs:["sharedGlobals","{workspaceRoot}/"+witnessRoot+"/**/*","{workspaceRoot}/"+original.source,"{workspaceRoot}/"+original.manifest,"{workspaceRoot}/"+compute+"/🧪️testing/📍️consumer/**/*","{workspaceRoot}/"+genericSchema],options:{cwd:packageRoot,command:"bun ./📜️script.ts "+command}};
  if(read(projectPath)===null)rows.push(pair(projectPath,json({name,$schema:rootImport(projectPath,"node_modules/nx/schemas/project-schema.json"),targets:{[command]:target}})));else rows.push(addTarget(projectPath,command,target));
  if(read(packagePath)===null)rows.push(pair(packagePath,json({name,private:true,type:"module",nx:{includedScripts:[]},scripts:{[command]:"nx run "+name+":"+command}})));else rows.push(addPackage(packagePath,command,name));
  projects.push({name,target:command});
 }
 const neutralFixture={...oldFixture},neutralSchema={...oldSchema,required:[...oldSchema.required],properties:{...oldSchema.properties}};
 for(const name of ["removed","consumers","constructorPolicy"]){delete neutralFixture[name];delete neutralSchema.properties[name];neutralSchema.required=neutralSchema.required.filter((x:string)=>x!==name);}
 rows.push(pair(oldFixturePath,json(neutralFixture)),pair(oldSchemaPath,json(neutralSchema)));
 let neutral=read(oldTestPath)!;
 neutral=neutral.replace(/;removed:string/,"").replace(/;consumers:\{[\s\S]*?constructorPolicy:[\s\S]*?}};/,"};");
 neutral=splice(neutral,' expect(validator({...corpus,constructorPolicy:{...corpus.constructorPolicy,cases:corpus.constructorPolicy.cases.map(row=>row.id==="foreign-owner"?{...row,path:corpus.constructorPolicy.owners[0]+"/🦀️.rs",allowed:true}:row)}})).toBe(false);\n',"");
 neutral=splice(neutral," expect(existsSync(join(root,corpus.removed))).toBe(false);\n","");
 neutral=splice(neutral,"digest('hex'),removed:fs.existsSync(process.argv[2])}));","digest('hex')}));");
 neutral=splice(neutral,",join(root,corpus.removed)]","]");
 neutral=splice(neutral,"{digest:corpus.retainedUnitSha256,removed:false}","{digest:corpus.retainedUnitSha256}");
 const kernel=' const kernel=read("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs");\n expect(kernel).not.toMatch(/pub mod os_engine|pub use crate::os_engine/);\n';neutral=splice(neutral,kernel,"");
 const consumerStart=neutral.indexOf('test("closed higher consumer witnesses directly bind the owned compute family"');if(consumerStart<0)throw Error("Missing original consumer law");neutral=neutral.slice(0,consumerStart).trimEnd()+"\n";
 rows.push(pair(oldTestPath,neutral));
 const projectPath=compute+"/📋️project.json",projectSource=read(projectPath)!,project=JSON.parse(projectSource),oldInputs=project.targets["test-ownership"].inputs as string[];
 project.targets["test-ownership"].inputs=oldInputs.filter(path=>!path.includes("🛍️products")&&!path.includes("✏️s")&&!path.includes("🌎️hub")&&path!=="{workspaceRoot}/📜️script.ts");
 rows.push(pair(projectPath,splice(projectSource,JSON.stringify(oldInputs,null,2).split("\n").join("\n      "),JSON.stringify(project.targets["test-ownership"].inputs,null,2).split("\n").join("\n      "))));
 const kernelRoot="🧰️framework/🛍️products/💻️os/🏛️ownership/🧮️compute",kernelFixture={version:1,source:"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs",removed:oldFixture.removed},kernelTest=kernelRoot+"/🧪️tests/🟦️.ts",kernelProject="@semio-tech/framework-os-compute-ownership";
 rows.push(pair(kernelRoot+"/🧫️fixtures/🔣️.json",json(kernelFixture)),pair(kernelRoot+"/🧬️schema/🔣️.json",json({$schema:"http://json-schema.org/draft-07/schema#",type:"object",additionalProperties:false,required:Object.keys(kernelFixture),properties:Object.fromEntries(Object.entries(kernelFixture).map(([k,v])=>[k,{const:v}]))})));
 rows.push(pair(kernelTest,[
 '/** 🏛️ Preserves the OS-owned facade retirement assertion. */','import {expect,test} from "bun:test";','import Ajv from "ajv";','import {readFileSync,existsSync} from "node:fs";','import {join,resolve} from "node:path";','import fixture from "../🧫️fixtures/🔣️.json";','import schema from "../🧬️schema/🔣️.json";',
 'const root=resolve(import.meta.dir,'+JSON.stringify(relative(dirname(kernelTest),".")||".")+');',
 'test("closed OS compute ownership fixture is exact",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:true})).toBe(false);});',
 'test("OS consumes the neutral compute owner without a generic facade",()=>{const source=readFileSync(join(root,fixture.source),"utf8");expect(source).not.toMatch(/pub mod os_engine|pub use crate::os_engine/);expect(existsSync(join(root,fixture.removed))).toBe(false);});',""
 ].join("\n")));
 const kernelScript=kernelRoot+"/📜️script.ts";
 rows.push(pair(kernelScript,'#!/usr/bin/env bun\nimport {ComputeOwnershipTestScript} from '+JSON.stringify(rootImport(kernelScript,runner))+';\nimport {ScriptRouter} from '+JSON.stringify(rootImport(kernelScript,bundle))+';\nimport {runScriptMain} from '+JSON.stringify(rootImport(kernelScript,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts"))+';\n/** 🏛️ Runs the declared OS compute ownership laws. */\nclass Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}\nawait runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test));\n'));
 rows.push(pair(kernelRoot+"/📋️project.json",json({name:kernelProject,$schema:rootImport(kernelRoot+"/📋️project.json","node_modules/nx/schemas/project-schema.json"),targets:{test:{executor:"nx:run-commands",cache:false,dependsOn:[],outputs:[],inputs:["sharedGlobals","{projectRoot}/**/*","{workspaceRoot}/"+kernelFixture.source,"{workspaceRoot}/"+compute+"/🧪️testing/📍️consumer/**/*"],options:{cwd:kernelRoot,command:"bun ./📜️script.ts test"}}}})));
 rows.push(pair(kernelRoot+"/package.json",json({name:kernelProject,private:true,type:"module",nx:{includedScripts:[]},scripts:{test:"nx run "+kernelProject+":test"}})));projects.push({name:kernelProject,target:"test"});
 const launchPath=".vscode/🧩️launch.seed.jsonc",launch=read(launchPath)!,needle='"name": "🧪️test◻️2d🧮️compute📍️ownership"',at=launch.lastIndexOf("\n    {",launch.indexOf(needle))+1,workspace="$"+"{workspaceFolder}",entries=[{name:"◻️2d compute neutral deletion",project:"@semio-tech/2d-compute",target:"test-neutral-ownership"},...projects.map(p=>({name:p.name+" compute ownership",project:p.name,target:p.target}))].map((row,i)=>({name:row.name,type:"node-terminal",request:"launch",command:"bun nx run "+row.project+":"+row.target+" --skip-nx-cache",cwd:workspace,env:{SEMIO_TEST_ARTIFACT_DIR:workspace+"/"+ticket.slice(root.length+1)+"/🗑️generated/goal-compute-ownership"},presentation:{group:"9_gates",order:900.057866+i/1000000}}));
 if(at<=0)throw Error("Missing GUI anchor");const additions=entries.map(e=>JSON.stringify(e,null,2).split("\n").map(l=>"    "+l).join("\n")+",\n").join("");rows.push(pair(launchPath,launch.slice(0,at)+additions+launch.slice(at)));
 const parseGaps=rows.filter(r=>r.path.endsWith(".ts")).flatMap(r=>ts.createSourceFile(r.path,r.authored,ts.ScriptTarget.Latest,true).parseDiagnostics.map(d=>({path:r.path,message:ts.flattenDiagnosticMessageText(d.messageText,"\n")})));
 if(parseGaps.length)throw Error("Parse gaps "+JSON.stringify(parseGaps));
 writeFileSync(join(import.meta.dir,"consumer-owner-ready-1.json"),JSON.stringify({at:new Date().toISOString(),sourceWrites:false,rows,projects,originalConsumers:consumerRows,originalNativeLaws:oldFixture.laws,parseGaps}));
 console.log(JSON.stringify({rows:rows.length,projects:projects.length,originalConsumers:consumerRows.length,originalNativeLaws:oldFixture.laws.length,parseGaps:0}));
}
async function strictConsumers(){
 const j=JSON.parse(readFileSync(join(import.meta.dir,"consumer-owner-ready-1.json"),"utf8")),overlay=new Map<string,string>(j.rows.map((r:Row)=>[resolve(r.path),r.authored])),options:ts.CompilerOptions={noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,resolveJsonModule:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun"]},host=ts.createCompilerHost(options),oldRead=host.readFile.bind(host),oldExists=host.fileExists.bind(host),oldDirectory=host.directoryExists!.bind(host);
 host.readFile=p=>overlay.get(resolve(p))??oldRead(p);host.fileExists=p=>overlay.has(resolve(p))||oldExists(p);host.directoryExists=p=>[...overlay.keys()].some(f=>f.startsWith(resolve(p)+"/"))||oldDirectory(p);
 const roots=j.rows.filter((r:Row)=>r.path.endsWith(".ts")&&(!r.path.endsWith("📜️script.ts")||r.before===null)).map((r:Row)=>resolve(r.path)),diagnostics=ts.getPreEmitDiagnostics(ts.createProgram(roots,options,host)).map(d=>({path:d.file?.fileName,start:d.start,message:ts.flattenDiagnosticMessageText(d.messageText,"\n")}));
 const output=join(ticket,"🗑️generated/goal-root/compute-consumers-strict-1.json");mkdirSync(dirname(output),{recursive:true});writeFileSync(output,JSON.stringify({at:new Date().toISOString(),roots,diagnostics}));
 console.log(JSON.stringify({roots:roots.length,diagnostics:diagnostics.length}));if(diagnostics.length)throw Error("Strict consumer diagnostics");
}
try{const command=process.argv[2];if(command==="prepare-boundary")await prepareBoundary();else if(command==="mount-boundary")await mount("boundary-test-first-ready-1");else if(command==="prepare-consumers")await prepareConsumers();else if(command==="strict-consumers")await strictConsumers();else if(command==="mount-consumers")await mount("consumer-owner-ready-1");else throw Error("Unknown owned command");}catch(error){console.error(error);process.exit(1);}
