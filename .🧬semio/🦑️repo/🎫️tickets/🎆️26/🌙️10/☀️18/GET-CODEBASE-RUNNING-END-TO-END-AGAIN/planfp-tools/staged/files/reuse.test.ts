/** ♻️ Contract and oracle: the witness of a real planner run holds exactly while the plan structure (targets, actions, declared files) is unchanged, and the reused plan refreshed by its publisher is byte-identical to a complete plan. For a corpus of edits (relevant, content-only and unrelated, in the working tree, in the git index and in the ignore rules) the law is `holds => reuse + refresh == the complete plan`; edits of the file set or the declarations break it, content-only edits keep it and still move the copies and fingerprints. */
import {expect,test}from"bun:test";
import {createHash}from"node:crypto";
import {chmodSync,cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join,relative}from"node:path";
import corpus from"../../🎬️scopes/🧪️tests/🧱️workspace/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1}from"../../../../🔭️witness/🟨️.mjs";
import {bazelPlanJobFreeV1,bazelPlanKeyV1}from"../🟦️.ts";
import {bazelIgnoreSourcesDigestV1}from"../../📜️script.ts";
import {changeJournalDirectoryV1}from"../../../../🏡️resident/📜️journal/🟦️.ts";

process.env.SEMIO_LANGUAGE="en";
const repo=process.cwd(),child=join(import.meta.dirname,"🧫️fixtures/🟦️.ts"),signal=new AbortController().signal;
const git=(root:string,...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","core.fsmonitor=false","-c","commit.gpgsign=false",...args],{cwd:root,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(args.join(" ")+": "+run.stderr.toString());return run.stdout.toString();};
const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
function checkout(){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"reuse-"))),root=join(base,"checkout");mkdirSync(root);
 for(const[path,text]of Object.entries(corpus.files))write(root,path,text);
 write(root,"other/x.txt","x\n");write(root,"docs/readme.md","r\n");write(root,".gitignore","*.log\n");write(root,"gen/src/skip.log","log\n");
 git(root,"init","-q");git(root,"add","-A");git(root,"commit","-q","-m","fixture");
 return{base,root};
}
function plan(base:string,root:string,workspace:string,witnessed:boolean,mode:"defer"|"capture"|"refresh"="capture"){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
 const env={...process.env,SEMIO_LANGUAGE:"en",...(witnessed?worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace,writable:[tmpdir()],racyMilliseconds:0}):{})};
 const run=Bun.spawnSync([process.execPath,...(witnessed?["--preload",join(base,"preload.mjs")]:[]),child,root,workspace,"{}",repo,mode],{cwd:repo,env,stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString()+run.stdout.toString());
 return witnessed?readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8")):undefined;
}
const ignored=new Set(["runtime.json","snapshot.json","planner-input.json","planner-output.json"]);
function digest(workspace:string):string{
 const hash=createHash("sha256"),walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1)){
  const path=join(directory,entry.name),name=relative(workspace,path);
  if(!name.includes("/")&&(ignored.has(entry.name)||entry.name.endsWith(".tmp")||entry.name.startsWith("bazel-")))continue;
  if(entry.isDirectory())walk(path);else hash.update(name+"\0"+(lstatSync(path).mode&0o111)+"\0"+readFileSync(path)+"\0");
 }};
 walk(workspace);return hash.digest("hex");
}
const ports=(root:string)=>({git:async(subject:string,expected:string)=>{const specs=JSON.parse(subject) as string[];let listing:string;try{listing=git(root,"ls-files","-c","-z",...(specs.length?["--",...specs]:[]));}catch{return expected==="!failed";}return worldWitnessGitDigestV1(listing)===expected;},ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,signal))===expected});

const relevant:Record<string,(root:string)=>void>={
 "new input file":root=>write(root,"gen/src/new.txt","n\n"),
 "removed input file":root=>rmSync(join(root,"gen/src/a.txt")),
 "new Bazel boundary file":root=>write(root,"gen/src/BUILD","\n"),
 "input replaced by a link":root=>{rmSync(join(root,"course/src/b.txt"));symlinkSync("../../gen/src/a.txt",join(root,"course/src/b.txt"));},
 "new directory with a file":root=>write(root,"gen/src/sub/deeper.txt","d\n"),
 "ignore rule removed":root=>write(root,".gitignore",""),
 "ignored file is force-added":root=>{git(root,"add","-f","gen/src/skip.log");},
 "untracked input file":root=>write(root,"course/src/untracked.txt","u\n"),
};
const content:Record<string,(root:string)=>void>={
 "content of an input":root=>write(root,"gen/src/a.txt","changed\n"),
 "executable bit":root=>chmodSync(join(root,"gen/src/a.txt"),0o755),
 "script of the task":root=>write(root,"course/script.ts","// edited\n"),
};
const conservative:Record<string,(root:string)=>void>={
 "ignored file next to an input":root=>write(root,"course/debug.log","noise\n"),
 "ignored file hidden by the rule":root=>write(root,"gen/src/other.log","noise\n"),
};
const unrelated:Record<string,(root:string)=>void>={
 "file of another project":root=>write(root,"other/x.txt","changed\n"),
 "new file of another project":root=>write(root,"other/y.txt","y\n"),
 "documentation":root=>write(root,"docs/readme.md","changed\n"),
 "commit of unrelated work":root=>{write(root,"other/x.txt","z\n");git(root,"add","other/x.txt");git(root,"commit","-q","-m","other");},
 "infrastructure":root=>write(root,".cache/b/noise.bin","n\n"),
};

test("the granted job count is no part of the key: requests and envelopes that differ only in --parallel give one key, any other difference another",()=>{
 const part=(jobs:number,task="@s/a:build")=>({request:bazelPlanJobFreeV1({tasks:[task],args:[],parallel:jobs,bail:false}),capabilities:bazelPlanJobFreeV1({arguments:["run",task,"--parallel="+jobs],launches:[["repo","run",task,"--parallel",String(jobs)]],selection:[task,"--parallel="+jobs]})});
 expect(bazelPlanKeyV1(part(6))).toBe(bazelPlanKeyV1(part(2)));
 expect(bazelPlanKeyV1(part(6))).not.toBe(bazelPlanKeyV1(part(6,"@s/b:build")));
 expect(bazelPlanJobFreeV1({selection:["--parallel","x","--parallelism=3"],tasks:[{parallel:false}]})).toEqual({selection:["--parallel","x","--parallelism=3"],tasks:[{parallel:false}]});
});

test("a fresh plan in the same workspace and in an empty one are identical, so the workspace is no input of the plan",()=>{
 const{base,root}=checkout();
 try{plan(base,root,join(base,"ws"),false);const first=digest(join(base,"ws"));plan(base,root,join(base,"ws"),false);expect(digest(join(base,"ws"))).toBe(first);rmSync(join(base,"ws"),{recursive:true});plan(base,root,join(base,"ws"),false);expect(digest(join(base,"ws"))).toBe(first);}
 finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("the witness of a real planner run is complete enough to be sealed and holds on the untouched world",async()=>{
 const{base,root}=checkout();
 try{
  const witness=plan(base,root,join(base,"ws"),true)!;
  expect(witness.tainted,witness.tainted.join(";")).toEqual([]);expect(witness.unstable).toEqual([]);expect(witness.racy).toBeNull();
  expect(witness.observations.some(row=>row[0]==="g")).toBe(true);expect(witness.observations.length).toBeGreaterThan(20);
  expect(await verifyWorldWitnessV1(witness,{signal,ports:ports(root)})).toMatchObject({holds:true});
  expect(bazelPlanKeyV1({a:1,b:[2,{c:3,d:4}]})).toBe(bazelPlanKeyV1({b:[2,{d:4,c:3}],a:1}));
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("oracle: whenever the witness holds after an edit the reused plan refreshed by its publisher equals the complete plan; relevant edits break it, content-only and unrelated edits do not",async()=>{
 const results:string[]=[];
 for(const[group,edits]of[["relevant",relevant],["content",content],["unrelated",unrelated],["conservative",conservative]]as const)for(const[name,edit]of Object.entries(edits)){
  const{base,root}=checkout(),workspace=join(base,"ws");
  try{
   const witness=plan(base,root,workspace,true,"defer")!;plan(base,root,workspace,false,"refresh");const before=digest(workspace);
   expect(witness.tainted,name).toEqual([]);
   expect(witness.observations.some(row=>row[0]==="S"||row[0]==="f"&&row[1].startsWith(root+"/gen/")),"the planner read source bytes: "+name).toBe(false);
   edit(root);
   const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(root)}),holds=verdict.holds;
   let reused:string|undefined;if(holds){plan(base,root,workspace,false,"refresh");reused=digest(workspace);}
   rmSync(workspace,{recursive:true,force:true});plan(base,root,workspace,false,"capture");const after=digest(workspace);
   if(holds)expect(reused,"unsound reuse after: "+name).toBe(after);
   if(group==="relevant"){expect(holds,"missed: "+name).toBe(false);expect(after,"relevant but no change: "+name).not.toBe(before);}
   else if(group==="content"){expect(holds,"needless replan after a content edit: "+name+" "+JSON.stringify(verdict)).toBe(true);expect(after,"the refresh must publish the new bytes: "+name).not.toBe(before);}
   else if(group==="unrelated")expect(holds,"needless replan after: "+name+" "+JSON.stringify(verdict)).toBe(true);
   results.push(`${group}: ${name}: holds=${holds} same=${after===before}`);
  }finally{rmSync(base,{recursive:true,force:true});}
 }
 console.log(results.join("\n"));
},600000);

test("the decision cycle of the adapter: first plan, record, reuse, edit, plan again, verify mode and the switches",async()=>{
 const api=await import(join(import.meta.dirname,"../../📜️script.ts")),library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1}=await import(routing);
 const{recordBazelPlanV1}=await import("../🟦️.ts");
 const{base,root}=checkout(),workspace=join(base,"ws"),graph=corpus.graph as never,request=corpus.request as never,planned=api.planBazelTasksV1(graph,request),emitted=api.bazelActionsV1(graph,planned,false,{hermetic:true});
 const implementation=(file:string)=>({version:1 as const,snapshot:join(workspace,file),runner:join(workspace,"📜️script.ts"),sha256:"a".repeat(64),loaderSha256:"b".repeat(64)});
 const previous=process.env.SEMIO_REPO_PLAN_REUSE,journal=process.env.SEMIO_REPO_CHANGES;process.env.SEMIO_REPO_CHANGES="off";
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:plan-reuse-glue",maximumElapsedMilliseconds:300000},createNativeBuildCallerCapabilitiesV1(root,["run",corpus.request.tasks[0]],"bazel",{}),Date.now()),async(original:never)=>{
   const phase=createIssuedResourceControlV1(original,"bazel:plan-reuse-glue",0),prepare=(identity:string|undefined="id-1")=>api.prepareBazelPlanReuseV1(root,graph,request,planned,original,phase,workspace,[],emitted,identity);
   const first=await prepare();expect(first.result).toBeUndefined();expect(first.record).toBeDefined();
   const planOnce=async()=>{
    const record=(await prepare()).record!;writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
    const run=Bun.spawnSync([process.execPath,"--preload",record.preload,child,root,workspace,"{}",repo],{cwd:repo,env:{...process.env,...worldWitnessEnvironmentV1({out:record.out,root,workspace,writable:[tmpdir()],racyMilliseconds:0})},stdout:"pipe",stderr:"pipe"});
    if(run.exitCode!==0)throw Error(run.stderr.toString());
    return recordBazelPlanV1({workspace,key:record.key,implementation:implementation("implementation.json"),adapter:implementation("implementation.json"),scopes:{},ignore:record.ignore,signal,nowMs:Date.now()});
   };
   expect(await planOnce()).toMatchObject({recorded:true});
   const reused=await prepare();expect(reused.result).toBeDefined();expect(reused.result!.workspace).toBe(workspace);expect(reused.result!.plan.selected).toEqual(planned.selected);expect(reused.result!.implementation).toEqual(implementation("implementation.json"));
   process.env.SEMIO_REPO_PLAN_REUSE="verify";const proven=await prepare();expect(proven.result).toBeUndefined();expect(proven.record?.proven?.length).toBeGreaterThan(0);delete process.env.SEMIO_REPO_PLAN_REUSE;
   expect(await planOnce()).toMatchObject({recorded:true});
   expect((await prepare("id-2")).result).toBeUndefined();
   expect(await planOnce()).toMatchObject({recorded:true});
   process.env.SEMIO_REPO_PLAN_REUSE="off";expect(await prepare()).toEqual({});delete process.env.SEMIO_REPO_PLAN_REUSE;
   expect((await prepare()).result).toBeUndefined();
   expect(await planOnce()).toMatchObject({recorded:true});
   write(root,"gen/src/a.txt","edited\n");
   expect((await prepare()).result,"a content edit keeps the plan").toBeDefined();
   write(root,"gen/src/added.txt","added\n");
   const after=await prepare();expect(after.result,"a new file in a declared glob replans").toBeUndefined();expect(after.record).toBeDefined();
   expect(await planOnce()).toMatchObject({recorded:true});expect((await prepare()).result).toBeDefined();
   const declared=JSON.parse(JSON.stringify(graph)),project=Object.values(declared.projects as Record<string,{targets:Record<string,{inputs?:unknown[]}>}>).find(row=>Object.values(row.targets).some(target=>Array.isArray(target.inputs)))!,target=Object.values(project.targets).find(row=>Array.isArray(row.inputs))!;target.inputs=[...target.inputs!,"{projectRoot}/declared-later/**/*"];
   expect((await api.prepareBazelPlanReuseV1(root,declared,request,api.planBazelTasksV1(declared,request),original,phase,workspace,[],emitted,"id-1")).result,"changed input declarations replan").toBeUndefined();
  });
 expect(existsSync(changeJournalDirectoryV1(root))).toBe(false);
 }finally{if(previous===undefined)delete process.env.SEMIO_REPO_PLAN_REUSE;else process.env.SEMIO_REPO_PLAN_REUSE=previous;if(journal===undefined)delete process.env.SEMIO_REPO_CHANGES;else process.env.SEMIO_REPO_CHANGES=journal;rmSync(base,{recursive:true,force:true});}
},300000);
