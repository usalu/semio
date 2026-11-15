/** 💥 Contract: every shared file list that many project nodes used to embed is a declared tool of the providers project that the nodes name by reference, so a file added to such a list moves one group of one project and no node. The declarations are closed (entries and resources exist, no entry is declared twice), the native toolchain files are exactly the build policy's, a native command names the toolchain tool and holds none of its files, and a script of the live tree that loads framework hubs holds only its own files and names the hubs, with the keyed files never fewer than the walk. The full-owner numbers come from `🧰️scripts/R29/harness/all.ts`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import policy from "../../🔣️.json" with {type:"json"};
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

setDefaultTimeout(240000);
const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,"../../../../../../../../../../../..");
const library=join(checkout,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const providers=join(library,"🏃️process/🔌️providers");
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const scripts=await import("../../../🟨️.mjs") as any,observation=await import("../../../../../../👁️observation/🟨️.mjs") as any;
const buildPolicy=JSON.parse(readFileSync(join(library,"⚡️caching/🔣️policy.json"),"utf8")) as {toolchains:{javascript:{files:string[]};cargo:{files:string[]}}};
const cache=()=>inference.createScriptInputCache({workspaceRoot:checkout});
const files=(list:readonly unknown[])=>list.filter((item):item is string=>typeof item==="string"&&item.startsWith("{workspaceRoot}/")).map(item=>item.slice(16)).sort();
const references=(list:readonly unknown[])=>list.filter((item:any)=>item!==null&&typeof item==="object"&&typeof item.input==="string"&&item.projects!==undefined).map((item:any)=>item.input as string);
const infer=<T>(body:()=>Promise<T>)=>observation.observeInferenceSnapshotV1(body) as Promise<T>;

test("the declared tools are closed under their schema, every entry and resource is a real file inside the workspace, no entry is declared twice, and the native toolchain tool names exactly the toolchain files of the build policy",()=>{
 expect(validateJsonSchemaSubset(schema,policy)).toEqual([]);
 const seen=new Map<string,string>();
 for(const[name,tool]of Object.entries(policy.tools.closures)){
  for(const path of [...tool.entries,...tool.resources]){
   const absolute=resolve(providers,path);
   expect(existsSync(absolute),`${name}: ${path}`).toBe(true);
   expect(relative(checkout,absolute).startsWith(".."),`${name}: ${path} lies inside the workspace`).toBe(false);
  }
  for(const entry of tool.entries){expect(seen.get(entry),`${entry} is declared by ${name} and ${seen.get(entry)}`).toBeUndefined();seen.set(entry,name);}
 }
 const toolchain=policy.tools.closures["native-toolchain"] as {entries:string[];resources:string[];view:string};
 expect(toolchain.entries).toEqual([]);expect(toolchain.view).toBe("native");
 const named=toolchain.resources.map(path=>relative(checkout,resolve(providers,path)).split("\\").join("/")).sort();
 const declared=[...buildPolicy.toolchains.javascript.files.filter(file=>!["package.json","bun.lock","bun.lockb"].includes(file)),...buildPolicy.toolchains.cargo.files].sort();
 expect(named).toEqual(declared);
});

test("a native command list names the toolchain tool and holds none of the toolchain files, and the tool group holds every one of them",async()=>{
 const{list,groups}=await infer(async()=>{
  const api=scripts.closureApiV1(cache(),checkout);
  return{list:await scripts.nativeCommandInputs(checkout,cache()) as unknown[],groups:new Map<string,unknown[]>(await api.sharedClosureGroups(relative(checkout,providers),"🔣️.json"))};
 });
 expect(references(list)).toContain("toolClosure:native-toolchain:native");
 const toolchain=[...buildPolicy.toolchains.javascript.files.filter(file=>!["package.json","bun.lock","bun.lockb"].includes(file)),...buildPolicy.toolchains.cargo.files];
 for(const file of toolchain){expect(files(list),file).not.toContain(file);expect(files(groups.get("toolClosure:native-toolchain:native")!),file).toContain(file);}
});

test("a Rust package test script of the live tree keeps only its own files, names the framework hubs it loads, and keys on at least the files its walk reaches",async()=>{
 const root="🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",target={options:{cwd:root,command:"repo ./📜️script.ts test"}};
 const{list,expanded,walked}=await infer(async()=>{
  const own=await scripts.targetScriptSources(target,checkout,cache(),root,undefined,"queries") as {closure:unknown[]};
  const full=await scripts.relativeScriptInputs([join(checkout,root,"📜️script.ts")],checkout,cache()) as string[];
  return{list:own.closure,expanded:await scripts.expandedCommandInputsV1(own.closure,checkout,cache()) as unknown[],walked:scripts.withoutSchedulingV1(full) as string[]};
 });
 expect(references(list)).toContain("toolClosure:package-router:scheduling");
 expect(files(list).length).toBeLessThan(25);
 const keyed=new Set(files(expanded));
 expect(files(walked).filter(path=>!keyed.has(path))).toEqual([]);
});
