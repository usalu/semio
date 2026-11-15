import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import * as esbuild from "esbuild";
import corpus from "../🔣️.json" with {type:"json"};

setDefaultTimeout(240000);
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🟨️.mjs") as any,closures=await import("../../🟨️.mjs") as any;
const designationSchema=join(import.meta.dir,"../../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const workspace=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"tc-law-")));roots.push(root);for(const[path,text]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const cache=(root:string,late:boolean)=>scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:late,scriptProviderDesignation:join(root,corpus.designation)});
const driver="⚡️caching/🦀️cargo/📜️script.ts",native=(extra:Record<string,unknown>={})=>({options:{cwd:".",command:'repo "'+driver+'" native cargo build'},...extra});
const files=(list:readonly unknown[])=>list.filter((item):item is string=>typeof item==="string"&&item.startsWith("{workspaceRoot}/")).map(item=>item.slice(16)).sort();
const toolRef=(name:string)=>({input:`toolClosure:${name}:native`,projects:[corpus.providersProject]});
const infer=<T>(body:()=>Promise<T>)=>observation.observeInferenceSnapshotV1(body) as Promise<T>;
const groupsOf=(root:string)=>infer(async()=>{const api=scriptInputs.closureApiV1(cache(root,false),root);return new Map<string,unknown[]>(await api.sharedClosureGroups(dirname(corpus.designation),"🔣️.json"));});

test("a command that runs a declared tool script names the tool and holds none of its files, and the files it keys on are never fewer than the walk of the script",async()=>{
 const root=workspace(),input=native();
 const list=await infer(()=>scriptInputs.nativeTargetCommandInputs(input,root,undefined,cache(root,false),"."));
 expect(list.filter((item:unknown)=>closures.sharedReferenceOfV1(item))).toContainEqual(toolRef("cargo-command"));
 expect(files(list).filter(path=>path.startsWith("lib/")||path===driver)).toEqual([]);
 const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(list,root,cache(root,false)));
 const walked=await infer(()=>scriptInputs.relativeScriptInputs([join(root,driver)],root,cache(root,false)));
 const keyed=new Set(files(expanded)),missing=files(scriptInputs.withoutSchedulingV1(walked)).filter(path=>!keyed.has(path));
 expect(missing).toEqual([]);
 const built=await esbuild.build({entryPoints:[join(root,driver)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
 for(const input of files(scriptInputs.withoutSchedulingV1(Object.keys(built.metafile!.inputs).map(input=>"{workspaceRoot}/"+relative(root,resolve(root,input))))))expect(keyed.has(input),input).toBe(true);
});

test("a file added to the tool closure changes the group of the tool and no command input list that names it",async()=>{
 const root=workspace(),before=await infer(()=>scriptInputs.nativeTargetCommandInputs(native(),root,undefined,cache(root,false),".")),groups=await groupsOf(root);
 writeFileSync(join(root,"lib/added.ts"),"export const added = 1;\n");
 writeFileSync(join(root,"lib/util.ts"),'import "./added.ts";\n'+(await Bun.file(join(root,"lib/util.ts")).text()));
 const after=await infer(()=>scriptInputs.nativeTargetCommandInputs(native(),root,undefined,cache(root,false),".")),moved=await groupsOf(root);
 expect(JSON.stringify(after)).toBe(JSON.stringify(before));
 const grew=files(moved.get("toolClosure:cargo-command:native")!).filter(path=>!files(groups.get("toolClosure:cargo-command:native")!).includes(path));
 expect(grew).toEqual(["lib/added.ts"]);
});

test("a command with module globs or runtime entries keeps walking its own entries and still keys on the tool files, and a script no tool declares is walked",async()=>{
 const root=workspace();
 for(const extra of [{metadata:{semio:{closure:{modules:["{projectRoot}/lib/**/*.ts"]}}}},{metadata:{semio:{runtimeSourceEntries:["{projectRoot}/lib/run.ts"]}}}]){
  const list=await infer(()=>scriptInputs.nativeTargetCommandInputs(native(extra),root,undefined,cache(root,false),"."));
  const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(list,root,cache(root,false)));
  expect(files(expanded)).toContain("lib/util.ts");
  expect(files(expanded)).toContain("lib/run.ts");
 }
 const plain=await infer(()=>scriptInputs.nativeTargetCommandInputs({options:{cwd:"plain",command:"repo ./📜️script.ts build"}},root,undefined,cache(root,false),"plain"));
 expect(files(plain)).toContain("plain/📜️script.ts");
});

test("a late command closure reads back the same input list as the direct inference",async()=>{
 const root=workspace(),direct=await infer(()=>scriptInputs.nativeTargetCommandInputs(native(),root,undefined,cache(root,false),"."));
 const late=await infer(async()=>{const api=scriptInputs.closureApiV1(cache(root,true),root),handle=scriptInputs.closureHandleOfV1(await api.nativeTargetCommandInputs(native(),await api.nativeCommandInputs(),"."));const result=await api.resolve([[handle.key,handle.recipe]]);return result.values.get(handle.key);});
 expect(files(late)).toEqual(files(direct));
 expect(late.filter((item:unknown)=>closures.sharedReferenceOfV1(item))).toEqual(direct.filter((item:unknown)=>closures.sharedReferenceOfV1(item)));
});

test("a script that reaches a declared tool keeps only its own files, names the tool, and does not change when the tool closure grows",async()=>{
 const root=workspace();
 mkdirSync(join(root,"app2"),{recursive:true});writeFileSync(join(root,"app2/📜️script.ts"),'import "../'+driver+'";\nexport const own = 1;\n');
 const target={options:{cwd:"app2",command:"repo ./📜️script.ts build"}},ask=()=>infer(()=>scriptInputs.nativeTargetCommandInputs(target,root,undefined,cache(root,false),"app2"));
 const before=await ask();
 expect(files(before).filter(path=>path.startsWith("app2/")||path.startsWith("lib/"))).toEqual(["app2/📜️script.ts"]);
 expect(before.filter((item:unknown)=>closures.sharedReferenceOfV1(item))).toContainEqual(toolRef("cargo-command"));
 const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(before,root,cache(root,false)));
 expect(files(expanded)).toContain("lib/util.ts");
 writeFileSync(join(root,"lib/added2.ts"),"export const added2 = 2;\n");
 writeFileSync(join(root,"lib/util.ts"),'import "./added2.ts";\n'+(await Bun.file(join(root,"lib/util.ts")).text()));
 expect(JSON.stringify(await ask())).toBe(JSON.stringify(before));
});

test("a new file that nothing imports, in or beside the tool tree, changes no input list and no group, and an edit of a body changes no list",async()=>{
 const root=workspace(),ask=()=>infer(()=>scriptInputs.nativeTargetCommandInputs(native(),root,undefined,cache(root,false),".")),before=JSON.stringify(await ask()),groups=await groupsOf(root);
 mkdirSync(join(root,"lib/tests"),{recursive:true});writeFileSync(join(root,"lib/tests/unreferenced.ts"),"export const unreferenced = 1;\n");writeFileSync(join(root,"⚡️caching/🦀️cargo/unreferenced.ts"),"export const alone = 1;\n");
 expect(JSON.stringify(await ask())).toBe(before);
 const moved=await groupsOf(root);for(const[name,list]of groups)expect(JSON.stringify(moved.get(name)),name).toBe(JSON.stringify(list));
 writeFileSync(join(root,"lib/run.ts"),(await Bun.file(join(root,"lib/run.ts")).text())+"// body edit\n");
 expect(JSON.stringify(await ask())).toBe(before);
});
