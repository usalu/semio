import {afterAll,test,expect} from "bun:test";
import Ajv from "ajv";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {checkStoreReceiptFileV1,declaredStoreReceiptV1,encodeStoreReceiptV1,isStoreRepairSelectionV1,issueStoreReceiptV1,settleStoreReceiptV1,observeStoreContentV1,parseStoreReceiptV1,recordStoreContentV1,requireStoreReceiptV1,storeReceiptOutputV1,storeRepairNxArgumentsV1,storeTreeDigestV1,verifyStoreReceiptV1,type StoreReceiptEntryV1} from "../🟨️.mjs";
import {checkStoreReceiptTasksV1,withStoreReceiptsV1,type StoreReceiptTaskV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Receipt"}),workspaceRoot=resolve(import.meta.dir,"../../../../../../../.."),directories:string[]=[];
/** 🧷️ The fixture rows read as the types the implementation returns: an entry is a file or a directory, a receipt file is in one closed state. */
const entries=fixture.entries as readonly StoreReceiptEntryV1[];
type ReceiptFileRow=Omit<(typeof fixture.receiptFiles)[number],"expected">&{readonly expected:Awaited<ReturnType<typeof checkStoreReceiptFileV1>>["state"]};
const policy=schema.definitions.Policy.const,receiptName=(target:string)=>policy.directory+"/"+target+policy.extension;

/** 🌳️ Materializes the fixture's store tree in a fresh workspace. */
function workspace():string{
 const root=mkdtempSync(join(tmpdir(),"store-receipt-"));directories.push(root);
 for(const row of fixture.tree){const path=join(root,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"hex" in row?Buffer.from(row.hex as string,"hex"):row.text as string);}
 return root;
}
type Mutation={op:string;path:string;text?:string;target?:string};
function mutate(root:string,mutations:readonly Mutation[]):void{
 for(const row of mutations){
  const path=join(root,row.path);
  if(row.op==="remove")rmSync(path,{recursive:true,force:true});
  else if(row.op==="write"){mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.text!);}
  else if(row.op==="mkdir")mkdirSync(path,{recursive:true});
  else if(row.op==="symlink")symlinkSync(row.target!,path);
  else throw Error("Unknown fixture mutation: "+row.op);
 }
}
const receipt=(target="build")=>({version:1 as const,project:"fixture/project",target,entries:fixture.entries as never,links:[] as never});
const sha256=(input:string|Uint8Array)=>{
 for(const [command,args] of [["shasum",["-a","256"]],["sha256sum",[]]] as const){const result=spawnSync(command,[...args],{input,encoding:"utf8"});if(result.status===0)return result.stdout.split(/\s/u)[0]!;}
 return undefined;
};

afterAll(()=>{for(const directory of directories)rmSync(directory,{recursive:true,force:true});});

test("the fixture's digests match an independent SHA-256 implementation",()=>{
 const independent=sha256("abc");
 if(independent===undefined)return;
 expect(independent).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
 for(const row of fixture.tree.filter(row=>"text" in row&&row.path==="store/bin/proctor"))expect(sha256(row.text as string)).toBe(fixture.entries[0]!.sha256);
 expect(sha256(fixture.treeManifest)).toBe(fixture.entries[1]!.sha256);
});

test("observing the fixture tree yields the language-agnostic expected entries",async()=>{
 const root=workspace();
 for(const entry of entries)expect(await observeStoreContentV1(root,join(root,entry.path))).toEqual(entry);
 expect(await storeTreeDigestV1(join(root,"store/site"))).toEqual({sha256:fixture.entries[1]!.sha256,bytes:fixture.entries[1]!.bytes,files:fixture.entries[1]!.files});
 expect(await storeTreeDigestV1(join(root,"store/site"),false)).toMatchObject({bytes:51,files:4});
});

test("the receipt schema and the runtime guard accept and refuse the same documents",()=>{
 expect(validate(receipt())).toBe(true);
 expect(parseStoreReceiptV1(encodeStoreReceiptV1(receipt()))).toEqual(receipt());
 for(const row of fixture.malformed){
  const accepted=(()=>{try{requireStoreReceiptV1(row.receipt);return true;}catch{return false;}})();
  expect(validate(row.receipt)&&accepted===true||!validate(row.receipt)&&accepted===false,row.name).toBe(true);
  expect(accepted,row.name).toBe(false);
 }
});

test("a receipt encodes canonically whatever order its entries arrive in",()=>{
 const shuffled={...receipt(),entries:[...fixture.entries].reverse() as never};
 expect(encodeStoreReceiptV1(shuffled)).toBe(encodeStoreReceiptV1(receipt()));
 expect(encodeStoreReceiptV1(receipt()).endsWith("}\n")).toBe(true);
});

test("verification reports exactly the language-agnostic findings",async()=>{
 for(const row of fixture.verification){
  if(process.platform==="win32"&&row.mutations.some(mutation=>mutation.op==="symlink"))continue;
  const root=workspace();mutate(root,row.mutations as Mutation[]);
  expect(await verifyStoreReceiptV1(root,receipt(),row.mode as "stat"|"digest"),row.name).toEqual(row.expected as never);
 }
});

const linkedReceipt=()=>({...receipt(),entries:[fixture.entries[1]!] as never,links:fixture.links.map(({path,target})=>({path,target})) as never});

test("links bound to published content are part of the receipt",()=>{
 const linked=linkedReceipt();
 expect(validate(linked)).toBe(true);
 expect(parseStoreReceiptV1(encodeStoreReceiptV1(linked))).toEqual(linked);
});

test.skipIf(process.platform==="win32")("links bound to published content are verified with the receipt (POSIX hosts)",async()=>{
 const linked=linkedReceipt();
 for(const row of fixture.linkVerification){
  const root=workspace();
  for(const link of fixture.links){mkdirSync(dirname(join(root,link.path)),{recursive:true});symlinkSync(link.symlink,join(root,link.path));}
  mutate(root,row.mutations as Mutation[]);
  expect(await verifyStoreReceiptV1(root,linked),row.name).toEqual(row.expected as never);
 }
});

test("receipt files that are missing or unreadable are told apart from invalid content",async()=>{
 const root=workspace(),file=join(root,receiptName("build"));
 for(const row of fixture.receiptFiles as readonly ReceiptFileRow[]){
  rmSync(file,{force:true});if(row.content!==null){mkdirSync(dirname(file),{recursive:true});writeFileSync(file,row.content);}
  expect((await checkStoreReceiptFileV1(root,file)).state,row.name).toBe(row.expected);
 }
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,encodeStoreReceiptV1(receipt()));
 expect((await checkStoreReceiptFileV1(root,file)).state).toBe("valid");
});

test("a task declares a receipt only through its own target's receipt output",()=>{
 expect(storeReceiptOutputV1("build")).toBe("{projectRoot}/"+receiptName("build"));
 for(const row of fixture.declarations){
  const actual=declaredStoreReceiptV1({workspaceRoot:"/workspace",projectRoot:row.projectRoot,target:row.target,outputs:row.outputs});
  expect(actual===undefined?null:relative("/workspace",actual).split("\\").join("/"),row.name).toBe(row.expected);
 }
 expect(()=>declaredStoreReceiptV1({workspaceRoot:"/workspace",projectRoot:"../outside",target:"build",outputs:[storeReceiptOutputV1("build")]})).toThrow();
 expect(()=>storeReceiptOutputV1("a/b")).toThrow();
});

test("issuing a receipt names a private pending file and leaves the declared receipt alone",()=>{
 const root=workspace(),outputs=[storeReceiptOutputV1("build")],file=join(root,"proj",receiptName("build"));
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"finished by another run");
 const undeclared=issueStoreReceiptV1({workspaceRoot:root,projectRoot:"proj",target:"build",outputs:[]});
 expect(Object.hasOwn(undeclared,policy.environment)&&undeclared[policy.environment]===undefined,"a task without a receipt unsets the parent's").toBe(true);
 const first=issueStoreReceiptV1({workspaceRoot:root,projectRoot:"proj",target:"build",outputs})[policy.environment]!,second=issueStoreReceiptV1({workspaceRoot:root,projectRoot:"proj",target:"build",outputs})[policy.environment]!;
 expect(first.startsWith(file+".")&&first!==file&&first!==second,"each run gets its own pending file below the receipt directory").toBe(true);
 expect(first.length,"within the finite path bound").toBeLessThanOrEqual(256);
 expect(readFileSync(file,"utf8"),"a concurrent start of the same task never deletes the receipt a finishing run wrote").toBe("finished by another run");
});

test("settling moves the pending receipt over the declared one atomically; failure and an empty task keep only what is current",async()=>{
 const root=workspace(),outputs=[storeReceiptOutputV1("build")],file=join(root,"proj",receiptName("build")),declaration={workspaceRoot:root,projectRoot:"proj",target:"build",outputs};
 mkdirSync(dirname(file),{recursive:true});
 const issued=issueStoreReceiptV1(declaration),other=issueStoreReceiptV1(declaration);
 writeFileSync(issued[policy.environment]!,"mine");writeFileSync(other[policy.environment]!,"theirs");
 await settleStoreReceiptV1(declaration,issued,{succeeded:true,startedAtMs:Date.now()});
 expect(readFileSync(file,"utf8")).toBe("mine");expect(existsSync(issued[policy.environment]!)).toBe(false);
 const late=issueStoreReceiptV1(declaration);await settleStoreReceiptV1(declaration,late,{succeeded:true,startedAtMs:Date.now()+60_000});
 expect(existsSync(file),"a task that recorded nothing drops a receipt older than its own start").toBe(false);
 writeFileSync(file,"fresh");const empty=issueStoreReceiptV1(declaration);await settleStoreReceiptV1(declaration,empty,{succeeded:true,startedAtMs:Date.now()-60_000});
 expect(readFileSync(file,"utf8"),"but keeps one a concurrent run finished after it started").toBe("fresh");
 writeFileSync(other[policy.environment]!,"theirs");await settleStoreReceiptV1(declaration,other,{succeeded:false,startedAtMs:Date.now()});
 expect(existsSync(other[policy.environment]!),"a failed task leaves no pending file").toBe(false);expect(readFileSync(file,"utf8")).toBe("fresh");
 await settleStoreReceiptV1({...declaration,outputs:[]},{[policy.environment]:undefined},{succeeded:true,startedAtMs:0});
});

test("two overlapping runs of one task each keep a complete receipt: the later settle wins and no start deletes a finished receipt",async()=>{
 const root=workspace(),outputs=[storeReceiptOutputV1("build")],file=join(root,"proj",receiptName("build")),declaration={workspaceRoot:root,projectRoot:"proj",target:"build",outputs};
 mkdirSync(join(root,"store"),{recursive:true});writeFileSync(join(root,"store","a"),"alpha");
 const environment=(issued:Record<string,string|undefined>)=>({...issued,SEMIO_TASK_PROJECT:"fixture/project",SEMIO_TASK_TARGET:"build"});
 const first=issueStoreReceiptV1(declaration);await recordStoreContentV1({environment:environment(first),workspaceRoot:root,paths:[join(root,"store/a")]});
 const second=issueStoreReceiptV1(declaration);
 await settleStoreReceiptV1(declaration,first,{succeeded:true,startedAtMs:0});
 expect(existsSync(file),"the run that finished first still captures its receipt although the second run has started").toBe(true);
 expect(parseStoreReceiptV1(readFileSync(file,"utf8")).entries.map(entry=>entry.path)).toEqual(["store/a"]);
 await recordStoreContentV1({environment:environment(second),workspaceRoot:root,paths:[join(root,"store/a")]});
 await settleStoreReceiptV1(declaration,second,{succeeded:true,startedAtMs:0});
 expect((await checkStoreReceiptFileV1(root,file)).state).toBe("valid");
});

test("recording merges published content by path and a task without a receipt records nothing",async()=>{
 const root=workspace(),file=join(root,"proj",receiptName("build")),environment={[policy.environment]:file,SEMIO_TASK_PROJECT:"fixture/project",SEMIO_TASK_TARGET:"build"};
 await recordStoreContentV1({environment:{},workspaceRoot:root,paths:[join(root,"store/site")]});
 expect(existsSync(file)).toBe(false);
 await Promise.all([recordStoreContentV1({environment,workspaceRoot:root,paths:[join(root,"store/site")]}),recordStoreContentV1({environment,workspaceRoot:root,paths:[join(root,"store/bin/proctor")]})]);
 expect(parseStoreReceiptV1(readFileSync(file,"utf8"))).toEqual({version:1,project:"fixture/project",target:"build",entries,links:[]});
 await recordStoreContentV1({environment,workspaceRoot:root,links:fixture.links.map(({path,target})=>({path,target}))});
 expect(parseStoreReceiptV1(readFileSync(file,"utf8")).links.map(link=>link.path)).toEqual(["proj/dist","proj/📤️dist/prototype"]);
 writeFileSync(join(root,"store/bin/proctor"),"#!/bin/sh\n");
 await recordStoreContentV1({environment,workspaceRoot:root,paths:[join(root,"store/bin/proctor")]});
 const merged=parseStoreReceiptV1(readFileSync(file,"utf8"));
 expect(merged.entries.map(entry=>entry.path)).toEqual(["store/bin/proctor","store/site"]);
 expect(merged.entries[0]!.bytes).toBe(10);
 await expect(recordStoreContentV1({environment,workspaceRoot:root,paths:[resolve(root,"..","outside")]})).rejects.toThrow();
 await expect(recordStoreContentV1({environment:{[policy.environment]:file},workspaceRoot:root,paths:[join(root,"store/site")]})).rejects.toThrow();
});

test("the repair launch is exactly one task run uncached without its dependencies",()=>{
 for(const row of fixture.repairSelections)expect(isStoreRepairSelectionV1(row.selection),row.selection.join(" ")).toBe(row.expected);
 expect(isStoreRepairSelectionV1(storeRepairNxArgumentsV1("@teaching/proctor:build"))).toBe(true);
 expect(isStoreRepairSelectionV1(storeRepairNxArgumentsV1("@teaching/proctor:build:release",["--flag"]))).toBe(true);
 expect(()=>storeRepairNxArgumentsV1("proctor")).toThrow();
});

test("a cache hit whose receipt names missing store content is a miss that re-executes",async()=>{
 for(const row of fixture.scenarios){
  const root=workspace(),file=join(root,receiptName("build")),task:StoreReceiptTaskV1={id:"fixture/project:build",receipt:file};
  const restore=()=>{mkdirSync(dirname(file),{recursive:true});writeFileSync(file,encodeStoreReceiptV1(receipt()));};
  if(row.receipt==="valid")restore();
  if(row.receipt==="store-missing"){restore();rmSync(join(root,"store/site"),{recursive:true,force:true});}
  if(row.receipt==="malformed"){mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"{");}
  let runs=0,repairs=0,outcome="ok",message="";
  try{
   await withStoreReceiptsV1({root,tasks:[task],report:()=>{},
    run:async()=>{runs+=1;if((runs===1?row.firstRun:row.secondRun)==="fail")throw Error("run failed");},
    repair:async()=>{repairs+=1;if(row.repairWrites==="valid"){for(const tree of fixture.tree){const path=join(root,tree.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"hex" in tree?Buffer.from(tree.hex as string,"hex"):tree.text as string);}restore();}}});
  }catch(error){outcome="error";message=String((error as Error).message);}
  expect({runs,repairs,outcome},row.name).toEqual({runs:row.expected.runs,repairs:row.expected.repairs,outcome:row.expected.outcome});
  if("message" in row.expected)expect(message,row.name).toContain(row.expected.message!);
 }
});

test("without a declared receipt the selected run is the whole story",async()=>{
 let runs=0;
 await withStoreReceiptsV1({root:workspaceRoot,tasks:[],run:async()=>{runs+=1;},repair:async()=>{throw Error("never");}});
 expect(runs).toBe(1);
 await expect(withStoreReceiptsV1({root:workspaceRoot,tasks:[],run:async()=>{throw Error("run failed");},repair:async()=>{}})).rejects.toThrow("run failed");
 expect((await checkStoreReceiptTasksV1(workspaceRoot,[])).size).toBe(0);
});

test("every declared receipt output belongs to its own target and to a handler that publishes through a recording publisher",()=>{
 const publishers=/runArtifactRustPackageMain|runArtifactTypeScriptPackageMain|buildAdmittedWasmWebV1|buildRepositoryWasmWebV1|buildAdmittedCargoArtifactsV1|buildRepositoryCargoArtifacts|buildViteArtifact|runViteBuild|withRepositoryArtifactDestinationV1|stageRepositoryArtifacts|publishPlayPageCohortV1|buildReleasePipelineV1|stageAdmittedArtifactTreesV1/u,listed=spawnSync("git",["ls-files","-z","--","*📋️project.json"],{cwd:workspaceRoot,encoding:"utf8",maxBuffer:1<<28});
 expect(listed.status).toBe(0);
 let declared=0;
 for(const file of listed.stdout.split("\0").filter(name=>name&&!name.startsWith(".🧬semio/"))){
  const path=join(workspaceRoot,file);if(!existsSync(path))continue;
  let project:{targets?:Record<string,{outputs?:string[];options?:{cwd?:string;command?:string};metadata?:{semio?:{rustBuildIntent?:unknown}}}>};
  try{project=JSON.parse(readFileSync(path,"utf8"));}catch{continue;}
  for(const [name,target] of Object.entries(project.targets??{})){
   const receipts=(target.outputs??[]).filter(output=>output.includes("/"+policy.directory+"/"));
   if(!receipts.length)continue;
   declared+=1;
   expect(receipts,file+" "+name).toEqual([storeReceiptOutputV1(name)]);
   expect(target.metadata?.semio?.rustBuildIntent,file+" "+name+" is replaced by per-unit compile tasks").toBeUndefined();
   const script=/^repo "?([^\s"]*📜️script\.ts)/u.exec(target.options?.command??"")?.[1];
   expect(script,file+" "+name).toBeDefined();
   const source=resolve(workspaceRoot,(target.options?.cwd??dirname(file)).replace("{projectRoot}",dirname(file)),script!);
   const own=readModuleTreeSource(source,{tests:true}),imported=[...own.matchAll(/(?:from\s*|import\s*\(\s*)["'](\.{1,2}\/[^"']+\.ts)["']/gu)].flatMap(match=>{const target=resolve(dirname(source),match[1]!);return existsSync(target)?[readModuleTreeSource(target,{tests:true})]:[];});
   expect(publishers.test([own,...imported].join("\n")),file+" "+name+" publishes through "+source+" or a module it imports").toBe(true);
  }
 }
 expect(declared).toBeGreaterThan(0);
});
