/** 🔌 Contract: a command closure read through a view names each script provider port it defines, and each target-independent tool command it runs (the cargo driver, the lock owner), by one reference to the providers project's group of that port and view or tool, never by their files; the providers project holds every such shared closure as one group (designation, schema and the transitive walks without the view's prefixes). So a scheduling-only edit inside a provider or tool changes no held fact at all, a keyed provider or tool edit changes only that shared closure's groups (one project), an edit of the command's own closure or of the ports it defines changes the command; and an action that consumes a provider still keys on exactly the provider files through the reference (Bazel expansion, Nx projection, the slice). The real walk, `resolve`, `deriveClosureFactV1`, the Bazel input expansion, the slice closure and the Nx projection run under the real observation layer on a tiny workspace; an independent Python model states every roster, group, reference and verdict, and esbuild's bundle graph states the full closure. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️.json" with {type:"json"};
import policySchema from "../🧬️schema/🔣️.json" with {type:"json"};

setDefaultTimeout(240000);
type Model={product:{own:string[];ports:string[];native:string[]};plain:{own:string[];ports:string[]};groups:Record<string,string[]>;tools:Record<string,string[]>;whole:string[];srcs:string[]};
const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🟨️.mjs") as any,closures=await import("../🟨️.mjs") as any;
const bazel=await import("../../../../../🔧️tools/🏗️bazel/📜️script.ts") as any,load=await import("../../../../🕰️generations/🔌️load/🟨️.mjs") as any,nx=await import("../../../../../../🟨️.mjs") as any;
const scheduling=join(import.meta.dir,"../../../../../📥️source/🔏️closure/🗓️scheduling/🔣️.json"),designationSchema=join(import.meta.dir,"../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json");
const python=requireTestToolV1("python"),roots:string[]=[],P=corpus.providersProject;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const workspace=(prefix:string)=>{const root=realpathSync(mkdtempSync(join(tmpdir(),prefix)));roots.push(root);write(root,corpus.files);mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const oracle=():{states:Model[];verdicts:Record<string,boolean>[]}=>{const run=spawnSync(python,["-I",join(import.meta.dir,"🐍️oracle","🐍️.py"),join(import.meta.dir,"🔣️.json"),scheduling],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const cache=(root:string,late:boolean)=>scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:late,scriptProviderDesignation:join(root,corpus.designation)});
const target=(cwd:string)=>({options:{cwd,command:"repo ./📜️script.ts build"}});
const files=(list:readonly unknown[])=>list.filter((item):item is string=>typeof item==="string"&&item.startsWith("{workspaceRoot}/")).map(item=>item.slice(16)).sort();
const references=(list:readonly unknown[])=>list.filter(item=>closures.sharedReferenceOfV1(item));
const referenceOf=(port:string,view:string)=>({input:`scriptProvider:${port}:${view}`,projects:[P]}),toolOf=(name:string)=>({input:`toolClosure:${name}:native`,projects:[P]});
const fixtureFiles=(list:readonly unknown[],root:string)=>files(list).filter(path=>existsSync(join(root,path)));
const holderOf=(name:string)=>name.startsWith(policy.group.prefix)?name.slice(policy.group.prefix.length):"tool:"+name.slice(policy.tools.prefix.length).split(":")[0];
const infer=(root:string)=>observation.observeInferenceSnapshotV1(async()=>{
 const api=scriptInputs.closureApiV1(cache(root,true),root),handle=async(list:unknown)=>scriptInputs.closureHandleOfV1(await list),handles:Record<string,any>={
  product:await handle(api.genericTargetCommandInputs(target(corpus.products.product.cwd),corpus.products.product.cwd)),
  productNative:await handle(api.nativeTargetCommandInputs(target(corpus.products.product.cwd),await api.nativeCommandInputs(),corpus.products.product.cwd)),
  plain:await handle(api.genericTargetCommandInputs(target(corpus.products.plain.cwd),corpus.products.plain.cwd)),
  whole:await handle(api.closureOf([join(root,corpus.products.product.entry)],[],"test-discovery")),
 };
 for(const[name,list]of await api.sharedClosureGroups("lib/providers","🔣️.json"))handles[holderOf(name)]=scriptInputs.closureHandleOfV1(list);
 const resolved:Record<string,{value:unknown[];facts:Map<string,string>;rows:Map<string,any>}>={};
 for(const[name,row]of Object.entries(handles)){const result=await api.resolve([[row.key,row.recipe]]),walks=result.derived.filter((fact:any)=>JSON.parse(fact.row.path).walk===1);resolved[name]={value:result.values.get(row.key),facts:new Map(walks.map((fact:any)=>[fact.row.path,fact.row.fact])),rows:new Map(walks.map((fact:any)=>[fact.row.path,fact.row]))};}
 return resolved;
});
const ask=(row:any,root:string)=>observation.observeInferenceSnapshotV1(()=>scriptInputs.deriveClosureFactV1(row,cache(root,false)));
const esbuildRoster=async(root:string,entries:readonly string[])=>{const roster=new Set<string>();for(const entry of entries){const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});for(const path of Object.keys(result.metafile.inputs))roster.add(path.replace(/^\.\//u,""));}return[...roster].sort();};
const providerModules=(root:string,ports:readonly string[])=>{const declared=JSON.parse(readFileSync(join(root,corpus.designation),"utf8")).providers as Record<string,string>,pending=[...ports],seen=new Set(pending),modules:string[]=[];while(pending.length){const module=join(dirname(corpus.designation),declared[pending.shift()!]!).replaceAll("\\","/");modules.push(module);for(const match of readFileSync(join(root,module),"utf8").matchAll(/(?<!function\s)\bdefineScriptProviderV1\s*\(\s*["']([a-z0-9-]+)["']/gu))if(!seen.has(match[1]!)&&Object.hasOwn(declared,match[1]!)){seen.add(match[1]!);pending.push(match[1]!);}}return modules;};
const graphOf=(resolved:Record<string,{value:unknown[]}>)=>({version:1,dependencies:[],namedInputs:{},projects:{
 app:{name:"app",root:corpus.products.product.cwd,namedInputs:{commandSources0:resolved.product!.value,commandSources1:resolved.productNative!.value},targets:{build:{inputs:["commandSources0"]},native:{inputs:["commandSources1"]}}},
 other:{name:"other",root:"other",namedInputs:{},targets:{}},
 [P]:{name:P,root:dirname(corpus.designation),namedInputs:Object.fromEntries(Object.entries(resolved).filter(([name])=>name.includes(":")).map(([name,row])=>[name.startsWith("tool:")?closures.toolGroupNameV1(name.slice(5)):policy.group.prefix+name,row.value])),targets:{}},
}});
const srcsOf=(graph:any,root:string,group="commandSources0")=>[...new Set((bazel.bazelTaskInputsV1(graph,{id:"app:"+group,project:"app",definition:{inputs:[group]}}) as unknown[]).filter((item):item is string=>typeof item==="string"))].filter(path=>existsSync(join(root,path))).sort();
const nativeSrcsOf=(state:Model)=>[...new Set([...state.product.own,...state.product.ports.flatMap(port=>state.groups[port+":native"]!),...state.tools["cargo-command"]!])].sort();
const keyOf=(root:string,srcs:readonly string[])=>{const hash=createHash("sha256");for(const path of srcs)hash.update(path).update("\0").update(readFileSync(join(root,path))).update("\0");return hash.digest("hex");};

test("the shared closure policy and the corpus are closed under their schemas, every authored verdict is the independent model's, the corpus names the policy's tool commands, and group names and references read back",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true});
 expect(ajv.validate(policySchema,policy)).toBe(true);expect(validateJsonSchemaSubset(policySchema,policy)).toEqual([]);expect(ajv.validate(corpusSchema,corpus)).toBe(true);
 expect(validateJsonSchemaSubset(policySchema,{...policy,views:{scheduling:policy.views.scheduling}}).length).toBeGreaterThan(0);
 expect(oracle().verdicts).toEqual(corpus.mutations.map(mutation=>mutation.changed));
 expect(closures.providerGroupNameV1("repository-tooling","native")).toBe("scriptProvider:repository-tooling:native");
 expect(closures.sharedGroupOfV1("scriptProvider:repository-tooling:native")).toEqual({fn:"script-provider",port:"repository-tooling",view:"native"});
 expect(closures.sharedGroupOfV1("toolClosure:cargo-command:native")).toEqual({fn:"tool-closure",name:"cargo-command",view:"native"});
 for(const name of ["scriptProvider:x:whole","scriptProvider:X:native","commandSources0","scriptProvider:x:native:y","toolClosure:cargo-command:scheduling","toolClosure:unknown:native"])expect(closures.sharedGroupOfV1(name)).toBeUndefined();
 expect(()=>closures.providerGroupNameV1("x","everything")).toThrow();expect(()=>closures.toolGroupNameV1("unknown")).toThrow();
 expect(closures.sharedReferenceOfV1(referenceOf("x","scheduling"))).toEqual({project:P,recipe:{fn:"script-provider",port:"x",view:"scheduling"}});
 expect(closures.sharedReferenceOfV1({input:"nativeSources",projects:["a"]})).toBeUndefined();
 expect(closures.joinWithProviderViewV1([["{workspaceRoot}/a",referenceOf("x","scheduling"),toolOf("cargo-command")],["{workspaceRoot}/a",referenceOf("x","native"),{env:"E"},toolOf("cargo-command")]],"native")).toEqual(["{workspaceRoot}/a",{env:"E"},referenceOf("x","native"),toolOf("cargo-command")]);
 for(const[name,tool]of Object.entries(policy.tools.closures))expect({entries:tool.entries.map(entry=>join(dirname(corpus.designation),entry).replaceAll("\\","/")),resources:(tool.resources as readonly string[]).map(entry=>join(dirname(corpus.designation),entry).replaceAll("\\","/"))},name).toEqual((corpus.tools as Record<string,{entries:string[];resources:string[]}>)[name]!);
 expect(closures.expandClosureValueV1({closure:["{workspaceRoot}/a",referenceOf("x","scheduling")],fallback:false},["{workspaceRoot}/b"])).toEqual({closure:["{workspaceRoot}/a","{workspaceRoot}/b"],fallback:false});
});

test("a viewed command closure names its providers by reference, the providers project holds one group per port and view, and every held fact follows exactly what it names across the edits",async()=>{
 const root=workspace("pc-law-"),model=oracle();
 let held=await infer(root);
 for(let index=0;index<=corpus.mutations.length;index++){
  const mutation=index?corpus.mutations[index-1]!:undefined,label=mutation?.name??"initial",state=model.states[index]!;
  if(mutation){
   write(root,mutation.edit);
   const verdict:Record<string,boolean>={},fresh=await infer(root);
   for(const holder of corpus.holders){
    const before=held[holder]!,after=fresh[holder]!;let changed=[...before.facts.keys()].sort().join("\n")!==[...after.facts.keys()].sort().join("\n");
    for(const[path,fact]of before.facts){const again=(await ask(before.rows.get(path),root)).fact;if(again!==fact)changed=true;if(after.facts.has(path))expect(after.facts.get(path),`${label}: ${holder}: the held fact is what a fresh inference holds`).toBe(again);}
    verdict[holder]=changed;
   }
   expect(verdict,label).toEqual(mutation.changed);
   held=fresh;
  }
  for(const holder of corpus.holders)expect(held[holder]!.facts.size,`${label}: ${holder} holds its walks by derived facts`).toBeGreaterThan(0);
  expect(files(held.product!.value),label).toEqual(state.product.own);expect(references(held.product!.value),label).toEqual(state.product.ports.map(port=>referenceOf(port,"scheduling")));
  expect(fixtureFiles(held.productNative!.value,root),label).toEqual(state.product.own);expect(references(held.productNative!.value),label).toEqual([...state.product.ports.map(port=>referenceOf(port,"native")),toolOf("cargo-command")].sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:1));
  expect(files(held.plain!.value),label).toEqual(state.plain.own);expect(references(held.plain!.value),label).toEqual([]);
  for(const[name,group]of Object.entries(state.groups)){expect(files(held[name]!.value),`${label}: ${name}`).toEqual(group);expect(references(held[name]!.value),`${label}: ${name}`).toEqual([]);expect(held[name]!.value.some((item:unknown)=>typeof item==="object"&&item!==null&&"externalDependencies" in item),`${label}: ${name} packages`).toBe(name.endsWith(":scheduling"));}
  for(const[name,group]of Object.entries(state.tools)){expect(files(held["tool:"+name]!.value),`${label}: tool ${name}`).toEqual(group);expect(references(held["tool:"+name]!.value),`${label}: tool ${name}`).toEqual([]);}
  expect(files(held.whole!.value),label).toEqual(state.whole);
  expect(files(held.whole!.value),label).toEqual([...new Set([...await esbuildRoster(root,[corpus.products.product.entry,...providerModules(root,state.product.ports)]),...(state.product.ports.length?[corpus.designation,corpus.schemaCopy]:[])])].sort());
 }
});

test("an action keys on exactly the provider files through the reference: Bazel expands it, the slice admits the providers project, Nx gets it lifted into the target; a scheduling-only edit keeps the key, a keyed provider edit moves it",async()=>{
 const root=workspace("pc-key-"),model=oracle();
 let resolved=await infer(root),graph=graphOf(resolved),srcs=srcsOf(graph,root),key=keyOf(root,srcs),native=srcsOf(graph,root,"commandSources1"),nativeKey=keyOf(root,native);
 expect(srcs).toEqual(model.states[0]!.srcs);expect(native).toEqual(nativeSrcsOf(model.states[0]!));
 const owners={app:graph.projects.app.root,other:"other",[P]:graph.projects[P]!.root};
 expect([...await load.sliceClosureV1({owners,fragment:async(name:string)=>load.sliceViewV1({edges:[],node:(graph.projects as any)[name]}),selection:{kind:"tasks",tasks:[{project:"app",target:"build"}]}})].sort()).toEqual(["app",P].sort());
 const lifted=nx.liftNxNamedInputReferencesV1(graph.projects) as any;
 expect(lifted.app.targets.build.inputs).toEqual(["commandSources0",...model.states[0]!.product.ports.map(port=>referenceOf(port,"scheduling"))]);
 expect(lifted.app.namedInputs.commandSources0.some((item:unknown)=>closures.sharedReferenceOfV1(item))).toBe(false);
 const shared=nx.shareNxProjectInputsV1(lifted) as any;
 for(const port of model.states[0]!.product.ports)expect(Object.hasOwn(shared[P].namedInputs,`scriptProvider:${port}:scheduling`)).toBe(true);
 expect(()=>nx.liftNxNamedInputReferencesV1({...graph.projects,dependent:{name:"dependent",root:"d",namedInputs:{},targets:{build:{inputs:["^commandSources0"]}}}})).toThrow();
 for(const[index,mutation]of corpus.mutations.entries()){
  write(root,mutation.edit);
  const next=await infer(root),nextGraph=graphOf(next),nextSrcs=srcsOf(nextGraph,root),nextKey=keyOf(root,nextSrcs),state=model.states[index+1]!,nextNative=srcsOf(nextGraph,root,"commandSources1"),nextNativeKey=keyOf(root,nextNative);
  expect(nextSrcs,mutation.name).toEqual(state.srcs);expect(nextNative,mutation.name).toEqual(nativeSrcsOf(state));
  const nativeConsumed=Object.keys(mutation.edit).some(path=>native.includes(path))||nextNative.join("\n")!==native.join("\n");
  expect(nextNativeKey!==nativeKey,`${mutation.name}: the native action re-executes exactly when a file it consumes changed`).toBe(nativeConsumed);
  if(mutation.kind==="scheduling-only")expect(nextNativeKey,`${mutation.name}: 0 native re-executions`).toBe(nativeKey);
  if(mutation.kind==="keyed-tool"){expect(nextNativeKey,`${mutation.name}: the Rust action re-executes`).not.toBe(nativeKey);expect(nextKey,`${mutation.name}: the TypeScript action does not`).toBe(key);}
  const consumed=Object.keys(mutation.edit).some(path=>srcs.includes(path))||nextSrcs.join("\n")!==srcs.join("\n");
  expect(nextKey!==key,`${mutation.name}: the action re-executes exactly when a file it consumes changed`).toBe(consumed);
  if(mutation.kind==="scheduling-only")expect(nextKey,`${mutation.name}: 0 re-executions`).toBe(key);
  if(mutation.kind==="keyed-provider")expect(nextKey,`${mutation.name}: the consuming action re-executes`).not.toBe(key);
  resolved=next;graph=nextGraph;srcs=nextSrcs;key=nextKey;native=nextNative;nativeKey=nextNativeKey;
 }
});

test("the full closure of a command composed from bounded walks equals the walk that followed ports, and queries over a closure answer over the expanded closure in both modes",async()=>{
 const root=workspace("pc-query-"),model=oracle(),state=model.states[0]!;
 const direct=await observation.observeInferenceSnapshotV1(async()=>(await scriptInputs.relativeScriptInputs([join(root,corpus.products.product.entry)],root,cache(root,false))) as string[]);
 expect(files(direct)).toEqual(state.whole);
 const asked=async(late:boolean)=>observation.observeInferenceSnapshotV1(async()=>{const api=scriptInputs.closureApiV1(cache(root,late),root),list=await api.genericTargetCommandInputs(target(corpus.products.product.cwd),corpus.products.product.cwd);return{provider:await api.some(list,["lib/run.ts"]),scheduling:await api.some(list,["🔒️leases/lease.ts"]),own:await api.some(list,["app/own.ts"]),absent:await api.some(list,["plain/"])};});
 for(const late of [false,true])expect(await asked(late),`late ${late}`).toEqual({provider:true,scheduling:false,own:true,absent:false});
});

test("a designation directory that is no providers project, a provider closure that imports a generated output and a closure without a declared role are refused",async()=>{
 const root=workspace("pc-refuse-");
 rmSync(join(root,"lib/providers/📋️project.json"));
 await expect(observation.observeInferenceSnapshotV1(async()=>scriptInputs.closureApiV1(cache(root,false),root).genericTargetCommandInputs(target(corpus.products.product.cwd),corpus.products.product.cwd))).rejects.toMatchObject({key:"build.closure.provider-project-required"});
 write(root,{"lib/providers/📋️project.json":corpus.files["lib/providers/📋️project.json"],"lib/cargo/provider.ts":"import { generated } from \"../gen.ts\";\nexport const provider = generated;\n","lib/gen.ts":"export const generated = 1;\n"});
 const generating=()=>{const scripts=cache(root,false);scripts.generatedOutput=(specifier:string)=>specifier.endsWith("/lib/gen.ts")?{target:"fixture:generate",output:"lib/gen.ts"}:undefined;return scripts;};
 await expect(observation.observeInferenceSnapshotV1(async()=>scriptInputs.closureApiV1(generating(),root).sharedClosureGroups("lib/providers","🔣️.json"))).rejects.toMatchObject({key:"build.closure.provider-generated-import"});
 await expect(observation.observeInferenceSnapshotV1(async()=>scriptInputs.closureApiV1(cache(root,true),root).closureOf([join(root,corpus.products.plain.entry)],[]))).rejects.toMatchObject({key:"build.closure.walk-views-required"});
 await expect(observation.observeInferenceSnapshotV1(async()=>scriptInputs.closureApiV1(cache(root,true),root).sharedClosureGroups("plain","🔣️.json"))).rejects.toMatchObject({key:"build.closure.provider-designation-differs"});
 await expect(observation.observeInferenceSnapshotV1(async()=>scriptInputs.closureApiV1(cache(root,true),root).toolClosure("unknown"))).rejects.toMatchObject({key:"build.closure.tool-unknown"});
});
