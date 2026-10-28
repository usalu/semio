#!/usr/bin/env python3
"""🧊 [optcache] anchored edits: the optional-cache suite runs against a private repository store (support: privateGraphWorkspaceV1) instead of the checkout's. usage: edit_optcache.py ROOT [--law-only] (edits the two files under ROOT in place; every anchor must occur exactly once, exit 2 otherwise)."""
import sys, os
root = sys.argv[1]
law_only = "--law-only" in sys.argv
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/"
SUP = G + "🧰️support/🟦️.ts"
SUITE = G + "🧊️optional-cache/🟦️.ts"

def edit(rel, pairs):
    p = os.path.join(root, rel)
    s = open(p, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            sys.stderr.write("anchor count %d != 1 in %s: %r\n" % (n, rel, old[:120]))
            sys.exit(2)
        s = s.replace(old, new)
    open(p, "w", encoding="utf-8").write(s)

support_head = [
 ('import {mkdtemp,rm} from "node:fs/promises";', 'import {mkdir,mkdtemp,readdir,rm} from "node:fs/promises";'),
 ('import {readFileSync} from "node:fs";', 'import {existsSync,readFileSync,realpathSync,writeFileSync} from "node:fs";'),
 ('import {dirname,join,resolve} from "node:path";', 'import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";'),
 ('import {spyOn,beforeAll} from "bun:test";', 'import {spyOn,beforeAll,afterAll} from "bun:test";'),
]

support_tail = '''
/** 🔒️ Whether one directory lies strictly below another. */
const strictlyInsideV1=(parent:string,child:string)=>{const distance=relative(parent,child);return distance!==""&&distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);};
const checkoutDirectoryV1=process.cwd();let pinsV1=0;
/** 🧭️ The directory a spawned toolchain child starts in (its bunfig and preload live there); it is never a store root. */
export const testToolchainDirectoryV1=()=>checkoutDirectoryV1;
/** 📌️ Pins the working directory to a private repository for the body: the graph maintenance custody derives its repository from `process.cwd()`, so a private store is only seen by it while the process stands in it; nested pins share one change and the original directory returns with the last. */
async function withWorkingDirectoryV1<T>(root:string,body:()=>Promise<T>):Promise<T>{if(pinsV1++===0)process.chdir(root);try{return await body();}finally{if(--pinsV1===0)process.chdir(checkoutDirectoryV1);}}
/** 🧫️ A private repository whose store, caches and leases are its own: invocations issued for it, scratch directories inside it. */
export interface PrivateGraphWorkspaceV1{readonly root:string;invoke<T>(body:(invocation:any)=>Promise<T>,options?:{readonly owner?:string;readonly maximumElapsedMilliseconds?:number}):Promise<T>;scratch(prefix:string):Promise<string>;requireIsolated():Promise<string>;close():Promise<void>}
/** 📍️ The scratch parents a private repository may live below, shortest first: an explicit parent, the checkout's own test scratch and the artifact directory; the deepest store path must fit 256 units, so the first parent that leaves room wins. */
function privateGraphParentsV1(checkout:string|null):string[]{
 const explicit=process.env.SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT,artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 return[...(explicit?[explicit]:[]),...(checkout!==null?[join(checkout,".cache","test-sandbox")]:[]),...(artifacts?[artifacts]:[])].map(path=>{try{return realpathSync(resolve(path));}catch{return resolve(path);}});
}
const privateGraphDeepestV1=(root:string)=>(root+"/.cache/b/g/m/inference/contexts/"+"0".repeat(64)+"/inference/ob/"+"0".repeat(64)+".br.9999999.99.tmp").length;
/** 🚫️ Refuses every workspace that is not a fresh private repository below the test artifact directory: the checkout itself or an ancestor of it, a path outside the artifacts, a store or lease directory that leaves it, or a writer lease that reached the checkout's lease store. */
export async function requirePrivateGraphWorkspaceV1(root:string):Promise<string>{
 const {repositoryCheckoutRootV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧭️root/🟦️.ts"),{repositoryLeaseDirectoryV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),leases=await import("@semio-tech/process/leases"),{repositoryBuildGraphDataDirectoryV1}=await import("../../../📥️inputs/💾️storage/🟨️.mjs");
 const actual=realpathSync(resolve(root)),checkout=repositoryCheckoutRootV1(checkoutDirectoryV1),parents=privateGraphParentsV1(checkout);
 if(!parents.some(parent=>strictlyInsideV1(parent,actual))||checkout!==null&&(actual===checkout||strictlyInsideV1(actual,checkout)))throw Error("Private graph workspace required: "+actual+" is not below a private scratch parent ("+parents.join(", ")+") or reaches the checkout");
 for(const path of[repositoryLeaseDirectoryV1(actual),repositoryBuildGraphDataDirectoryV1(actual,"inference")])if(!strictlyInsideV1(actual,path))throw Error("Private graph workspace required: "+path+" leaves "+actual);
 if(checkout!==null){let live:string|undefined;try{live=repositoryLeaseDirectoryV1(checkout);}catch{live=undefined;}if(live!==undefined&&existsSync(leases.resourceLeasePathV1(live,"repository-graph-facts:"+actual)))throw Error("Private graph workspace required: its writer lease reached the checkout's lease store");}
 return actual;
}
/** 🔎️ Every place a source reads the checkout's store: a `process.cwd()` call or a literal naming a generated store directory. @see https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API */
export function liveStoreReferencesV1(source:string):string[]{
 const ts=require("typescript"),found:string[]=[],file=ts.createSourceFile("source.ts",source,ts.ScriptTarget.Latest,true),live=/\\.cache\\/b(?:\\/|$)|⚡️cache|\\.🧬semio\\/🦑️repo/u;
 const visit=(node:any)=>{if(ts.isCallExpression(node)&&node.expression.getText()==="process.cwd")found.push(file.getLineAndCharacterOfPosition(node.getStart()).line+1+": process.cwd()");if((ts.isStringLiteral(node)||ts.isNoSubstitutionTemplateLiteral(node)||ts.isTemplateHead(node)||ts.isTemplateMiddle(node)||ts.isTemplateTail(node))&&live.test(node.text))found.push(file.getLineAndCharacterOfPosition(node.getStart()).line+1+": "+node.text);ts.forEachChild(node,visit);};
 visit(file);return found;
}
/** 🪦️ Retires one private partition image with the genuine protocol: its allocations are recovered and retired, its content removed, then the backing. */
async function retirePrivatePartitionV1(options:any):Promise<void>{
 const platform=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts"),contract=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"),storage=await import("@semio-tech/process/artifacts/native-build/storage"),image=await platform.openNativePlatformStorageV1(options);let primary:{error:unknown}|undefined;
 try{const held=await image.withExclusiveMaintenance({version:1,wait:false},async(port:any)=>{
  const policy={version:1 as const,root:image.root,leaseDirectory:options.leaseDirectory,capacityBytes:options.maximumPhysicalBytes,maximumWorkspaceBytes:options.filesystemBytes,maximumArtifactBytes:options.filesystemBytes},state=await storage.observeNativeStorageStateV1({policy,control:options.control,port});let cleared=false;
  const retiring=storage.nativeStorageRetirementPortV1(port,async()=>{if(cleared)return;for(const name of await readdir(image.root)){if(name==="storage-events.jsonl"||contract.nativeImageServiceEntryV1(name))continue;await rm(platform.requireNativeStoragePathV1(image.root,join(image.root,name)),{recursive:true,force:true});}cleared=true;});
  for(const row of state.allocations){const receipt=await storage.recoverNativeStorageReservationV1({policy,control:options.control,port:retiring,id:row.id});await receipt.retire();}
 });if(!held.acquired)throw Error("Private graph partition still has image readers");}catch(error){primary={error};throw error;}finally{try{await image.close();}catch(error){if(!primary)throw error;}}
 if(!await platform.retireNativePlatformStorageV1(options,{version:1,wait:false}))throw Error("Private graph partition image retirement refused");
}
/** 🎟️ Issues invocations for one private repository: the working directory is pinned to it and the capabilities name it as the workspace. */
const privateInvokeV1=(root:string)=><T,>(body:(invocation:any)=>Promise<T>,options?:{readonly owner?:string;readonly maximumElapsedMilliseconds?:number}):Promise<T>=>withWorkingDirectoryV1(root,()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:options?.owner??"graph-fixture-publisher",maximumElapsedMilliseconds:options?.maximumElapsedMilliseconds??20000},createNativeBuildCallerCapabilitiesV1(root,["show","projects"],"nx",process.env),Date.now()),body));
/** 🪦️ Retires every partition image of one private repository (the lease image last) and removes it. */
async function retirePrivateRootV1(root:string):Promise<void>{
 await privateInvokeV1(root)(async original=>{
  const {repositoryCacheBudgetV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),{repositoryLeaseDirectoryV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),budget=repositoryCacheBudgetV1(root),control=createIssuedResourceControlV1(original,"graph:private-retire",0);
  const retire=async(partition:any,options:object)=>{if(!existsSync(join(partition.directory,"image.json")))return;await retirePrivatePartitionV1({...partition,maximumPhysicalBytes:partition.capacityBytes,control,...options});};
  for(const[id,partition]of Object.entries(budget.partitions)){if(id==="leases"||id==="rust")continue;await retire(partition,{leaseDirectory:repositoryLeaseDirectoryV1(root)});}
  await retire(budget.partitions.leases,{maximumLeaseDatabaseBytes:16384,leaseDirectory:budget.controlDirectory});
 },{owner:"graph-fixture-retirement",maximumElapsedMilliseconds:120000});
 await rm(root,{recursive:true,force:true});
}
/** 🧹️ Retires the private repositories a killed test process left behind: those whose recorded owner process is gone. */
async function reapStalePrivateRootsV1(parent:string):Promise<void>{
 let names:string[];try{names=await readdir(parent);}catch{return;}
 for(const name of names){
  if(!/^g[A-Za-z0-9]{6}$/u.test(name))continue;const root=join(parent,name);let owner:number;try{owner=Number(readFileSync(join(root,".owner"),"utf8"));}catch{continue;}
  if(!Number.isSafeInteger(owner)||owner<=0||owner===process.pid)continue;try{process.kill(owner,0);continue;}catch(error){if((error as NodeJS.ErrnoException).code!=="ESRCH")continue;}
  try{await retirePrivateRootV1(realpathSync(root));}catch(error){process.stderr.write("[script] stale private graph workspace "+root+" kept: "+String((error as Error).message)+"\\n");}
 }
}
/** 🏗️ Creates the private repository below a scratch parent and provisions its lease and graph images by entering a writer once. */
async function openPrivateGraphWorkspaceV1():Promise<PrivateGraphWorkspaceV1>{
 const adopted=process.env.SEMIO_PRIVATE_GRAPH_WORKSPACE;let root:string,owned:boolean;
 if(adopted){root=realpathSync(resolve(adopted));owned=false;process.chdir(root);}
 else{const {repositoryCheckoutRootV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧭️root/🟦️.ts"),parents=privateGraphParentsV1(repositoryCheckoutRootV1(checkoutDirectoryV1)),parent=parents.find(path=>privateGraphDeepestV1(join(path,"g000000"))<=256);if(parent===undefined)throw Error("Private graph workspace needs a shorter scratch parent (SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT): the deepest store path exceeds 256 units below "+parents.join(", "));await mkdir(parent,{recursive:true});await reapStalePrivateRootsV1(parent);root=realpathSync(await mkdtemp(join(parent,"g")));writeFileSync(join(root,".owner"),String(process.pid));owned=true;}
 const invoke=privateInvokeV1(root);
 await requirePrivateGraphWorkspaceV1(root);
 const close=async()=>{if(owned)await retirePrivateRootV1(root);};
 try{if(owned){const api=await import("../../../📥️inputs/💾️storage/🟨️.mjs");await invoke(original=>api.withRepositoryBuildGraphFactStoreV1(root,original,createIssuedResourceControlV1(original,"graph:private-open",0),async()=>{}),{owner:"graph-fixture-provisioning",maximumElapsedMilliseconds:150000});}}
 catch(error){try{await close();}catch{}throw error;}
 return Object.freeze({root,invoke,scratch:async(prefix:string)=>{await mkdir(join(root,".tmp"),{recursive:true});return realpathSync(await mkdtemp(join(root,".tmp",prefix)));},requireIsolated:()=>requirePrivateGraphWorkspaceV1(root),close});
}
let privateWorkspaceV1:Promise<PrivateGraphWorkspaceV1>|undefined;
/** 🧫️ The one private repository of this test process, created on first use and retired after the last test; a spawned child adopts its parent's through `SEMIO_PRIVATE_GRAPH_WORKSPACE` and stands in it. */
export const privateGraphWorkspaceV1=():Promise<PrivateGraphWorkspaceV1>=>privateWorkspaceV1??=openPrivateGraphWorkspaceV1();
afterAll(async()=>{if(privateWorkspaceV1!==undefined){const workspace=await privateWorkspaceV1.catch(()=>undefined);await workspace?.close();}},170000);
'''

edit(SUP, support_head)
p = os.path.join(root, SUP)
s = open(p, encoding="utf-8").read()
if not s.endswith("\n"): s += "\n"
open(p, "w", encoding="utf-8").write(s + support_tail.lstrip("\n"))

suite_pairs = [
 ('import {expect,test,spyOn} from "bun:test";', 'import {beforeAll,expect,test,spyOn} from "bun:test";'),
 ('import {graph,withGraphTestInvocationV1,CommandSourceControlV1} from "../🧰️support/🟦️.ts";', 'import {graph,CommandSourceControlV1,privateGraphWorkspaceV1,requirePrivateGraphWorkspaceV1,liveStoreReferencesV1,testToolchainDirectoryV1} from "../🧰️support/🟦️.ts";'),
 ('const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;\n', 'const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;\nbeforeAll(async()=>{await privateGraphWorkspaceV1();},170000);\n'),
 # test 1
 ('const row=fixture.optionalReaderAdmission,namespace=repositoryBuildGraphDataDirectoryV1(process.cwd(),"inference"),readdir=rawPromises.readdir;',
  'const ws=await privateGraphWorkspaceV1(),root=ws.root,row=fixture.optionalReaderAdmission,namespace=repositoryBuildGraphDataDirectoryV1(root,"inference"),readdir=rawPromises.readdir;'),
 ('try{await withGraphTestInvocationV1(original=>withRepositoryBuildGraphArtifactV1(process.cwd(),graph(),original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,actualPhase)=>{expect(createHash',
  'try{await ws.invoke(original=>withRepositoryBuildGraphArtifactV1(root,graph(),original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,actualPhase)=>{expect(createHash'),
 # test 2
 ('{workspaceRoot:process.cwd()},Date.now()),async original=>{const issued=create(', '{workspaceRoot:(await privateGraphWorkspaceV1()).root},Date.now()),async original=>{const issued=create('),
 # test 4
 ('filesystem=await import("node:fs"),root=process.cwd(),source=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"qf-")),file=join(source,"input.json")',
  'filesystem=await import("node:fs"),ws=await privateGraphWorkspaceV1(),root=ws.root,source=await ws.scratch("qf-"),file=join(source,"input.json")'),
 ('await withGraphTestInvocationV1(async original=>api.withRepositoryBuildGraphFactStoreV1(root,original,createIssuedResourceControlV1(original,"graph:fixture-writer",0),async(store:any,phaseControl)=>',
  'await ws.invoke(async original=>api.withRepositoryBuildGraphFactStoreV1(root,original,createIssuedResourceControlV1(original,"graph:fixture-writer",0),async(store:any,phaseControl)=>'),
 # test 6
 ('const row=fixture.graphCacheQuota,api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),filesystem=await import("node:fs"),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"qh-")),file=',
  'const row=fixture.graphCacheQuota,api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),filesystem=await import("node:fs"),ws=await privateGraphWorkspaceV1(),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"qh-")),file='),
 ('await expect(withGraphTestInvocationV1(async original=>api.withRepositoryBuildGraphFactStoreV1(process.cwd(),original,', 'await expect(ws.invoke(async original=>api.withRepositoryBuildGraphFactStoreV1(ws.root,original,'),
 # contender
 ('const row=fixture.graphCacheQuota,root=process.cwd(),api=require("../../../📥️inputs/💾️storage/🟨️.mjs"),leases=', 'const row=fixture.graphCacheQuota,root=(await privateGraphWorkspaceV1()).root,api=require("../../../📥️inputs/💾️storage/🟨️.mjs"),leases='),
 # writers serialize
 ('test("optional graph writers serialize distinct originals and reuse only the exact active invocation",()=>withBuildTestInvocationV1(async incoming=>{\n const row=fixture.graphCacheQuota,api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),root=process.cwd(),',
  'test("optional graph writers serialize distinct originals and reuse only the exact active invocation",async()=>{const ws=await privateGraphWorkspaceV1();await ws.invoke(async incoming=>{\n const row=fixture.graphCacheQuota,api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),root=ws.root,'),
 ('["test",import.meta.path,"--test-name-pattern","^optional graph contender isolates accepted cancellation in its actual receiver$"],root,{control:createIssuedResourceControlV1(incoming,"graph:contender-child",20000),env:routing.scriptInvocationEnvironment(incoming,{...process.env,SEMIO_GRAPH_WRITER_CONTENDER:"1"}),',
  '["test",import.meta.path,"--test-name-pattern","^optional graph contender isolates accepted cancellation in its actual receiver$"],testToolchainDirectoryV1(),{control:createIssuedResourceControlV1(incoming,"graph:contender-child",20000),env:routing.scriptInvocationEnvironment(incoming,{...process.env,SEMIO_GRAPH_WRITER_CONTENDER:"1",SEMIO_PRIVATE_GRAPH_WORKSPACE:root}),'),
 ('expect(freshEntered).toBe(true);\n}),45000);', 'expect(freshEntered).toBe(true);\n},{maximumElapsedMilliseconds:60000});},45000);'),
 # nested
 ('const row=fixture.nestedOptionalMaintenance,root=process.cwd(),api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),filesystem=await import("node:fs"),leases=await import("@semio-tech/process/leases"),source=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"nm-")),identity=',
  'const row=fixture.nestedOptionalMaintenance,ws=await privateGraphWorkspaceV1(),root=ws.root,api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),filesystem=await import("node:fs"),leases=await import("@semio-tech/process/leases"),source=await ws.scratch("nm-"),identity='),
 ('  await withGraphTestInvocationV1(original=>api.withRepositoryBuildInferenceDatasetV1(root,original,createIssuedResourceControlV1(original,"graph:fixture-dataset",0),dataset,async(current:any,phaseControl)=>{const path=',
  '  await ws.invoke(original=>api.withRepositoryBuildInferenceDatasetV1(root,original,createIssuedResourceControlV1(original,"graph:fixture-dataset",0),dataset,async(current:any,phaseControl)=>{const path='),
 ('  await withGraphTestInvocationV1(original=>withRepositoryBuildGraphArtifactV1(root,graph(),original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,artifactPhase)=>{',
  '  await ws.invoke(original=>withRepositoryBuildGraphArtifactV1(root,graph(),original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,artifactPhase)=>{'),
 ('if(seeded)await withGraphTestInvocationV1(original=>api.withRepositoryBuildInferenceDatasetV1(root,original,', 'if(seeded)await ws.invoke(original=>api.withRepositoryBuildInferenceDatasetV1(root,original,'),
 # nested seed
 (',bytes=Math.max(1,row.minimumPressureBytes-api.buildGraphWriterOptionalQuotaFactsV1(root).usedBytes);pressurePath=path;',
  ';const seed=join(current.root,"np",(identity.slice(0,24)==="1".repeat(24)?"2":"1").repeat(24)+".json");expect(api.withBuildGraphOptionalPublicationV1(seed,{peakBytes:1,retainedDeltaBytes:1},()=>{filesystem.mkdirSync(dirname(seed),{recursive:true});filesystem.writeFileSync(seed,"x",{flag:"wx"});})).toBe(true);const bytes=Math.max(1,row.minimumPressureBytes-api.buildGraphWriterOptionalQuotaFactsV1(root).usedBytes);pressurePath=path;'),
 # observation publication
 ('test("actual observation publication reads its persisted full-digest block under the same genuine writer",async()=>{\n const api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),observation=await import("../../../👁️observation/🟨️.mjs"),source=await(async()=>{const root=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);if(join(root,"ob-xxxxxx","value").length>256)throw Error("Bounded observation fixture path required");return mkdtemp(join(root,"ob-"));})(),file=',
  'test("actual observation publication reads its persisted full-digest block under the same genuine writer",async()=>{\n const api=await import("../../../📥️inputs/💾️storage/🟨️.mjs"),observation=await import("../../../👁️observation/🟨️.mjs"),ws=await privateGraphWorkspaceV1(),root=ws.root,source=await ws.scratch("ob-"),file='),
 ('try{await withGraphTestInvocationV1(original=>api.withRepositoryBuildInferenceDatasetV1(process.cwd(),original,createIssuedResourceControlV1(original,"graph:block-fixture",0),{version:1,identity,sourceRoot:relative(process.cwd(),source).replaceAll("\\\\","/")},',
  'try{await ws.invoke(original=>api.withRepositoryBuildInferenceDatasetV1(root,original,createIssuedResourceControlV1(original,"graph:block-fixture",0),{version:1,identity,sourceRoot:relative(root,source).replaceAll("\\\\","/")},'),
 ('relative(repositoryBuildGraphDataDirectoryV1(process.cwd(),"inference"),block)', 'relative(repositoryBuildGraphDataDirectoryV1(root,"inference"),block)'),
]
edit(SUITE, suite_pairs[:3] if law_only else suite_pairs)

s = open(os.path.join(root, SUITE), encoding="utf-8").read()
laws = '''
test("optional graph cache suite touches no live repository store, cache or lease root",async()=>{
 const ws=await privateGraphWorkspaceV1(),own=await readFile(fileURLToPath(import.meta.url),"utf8"),store=[".cache","/b/l"].join(""),offender=["const root=process.","cwd();const live=join(root,'"+store+"');"].join("");
 expect(liveStoreReferencesV1(own)).toEqual([]);
 expect(liveStoreReferencesV1(offender)).toHaveLength(2);
 expect(liveStoreReferencesV1("const root=resolve(directory);")).toEqual([]);
 expect(await ws.requireIsolated()).toBe(ws.root);
 await expect(requirePrivateGraphWorkspaceV1(testToolchainDirectoryV1())).rejects.toThrow("Private graph workspace required");
 await expect(requirePrivateGraphWorkspaceV1(resolve(ws.root,".."))).rejects.toThrow("Private graph workspace required");
});
'''
open(os.path.join(root, SUITE), "w", encoding="utf-8").write(s.rstrip("\n") + "\n" + laws)
print("edited", root)
