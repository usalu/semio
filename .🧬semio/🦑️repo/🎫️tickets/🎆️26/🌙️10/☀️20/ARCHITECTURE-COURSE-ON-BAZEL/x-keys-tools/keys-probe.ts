/** 🔑️ Generates the Bazel workspace of a course-shaped fixture under different caller envelopes, cwds and ambient variables and reports every generated file digest, so action-key stability is decided by byte equality of every Bazel action input (usage: bun keys-probe.ts <outJson>). */
import {mkdirSync,mkdtempSync,readdirSync,readFileSync,lstatSync,rmSync,writeFileSync,symlinkSync,realpathSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join,relative,resolve} from "node:path";

const repo=process.cwd(),library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),process_=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const api=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(process_);

const graph={version:1,projects:{gen:{name:"gen",root:"gen",targets:{generate:{execution:{kind:"command",command:"bun ./script.ts generate"},outputs:["{projectRoot}/out"],inputs:["{projectRoot}/src/**/*"],cache:true}}},course:{name:"course",root:"course",namedInputs:{default:["{projectRoot}/**/*","!{projectRoot}/dist/**"]},targets:{test:{execution:{kind:"command",command:"bun ./script.ts test"},inputs:["default"],cache:true,outputs:[]},typecheck:{execution:{kind:"command",command:"bun ./script.ts typecheck"},dependsOn:["gen:generate"],inputs:["default",{dependentTasksOutputFiles:"**/*",transitive:false}],cache:true,outputs:[]}}}},dependencies:[{source:"course",target:"gen",type:"static"}]} as any;
const request={tasks:["course:test","course:typecheck"],args:[]} as any;

const digest=(directory:string,base=directory):Record<string,string>=>{const out:Record<string,string>={};for(const name of readdirSync(directory).sort()){const path=join(directory,name),facts=lstatSync(path);if(facts.isSymbolicLink()){out[relative(base,path)]="link";continue;}if(facts.isDirectory())Object.assign(out,digest(path,base));else out[relative(base,path)]=createHash("sha256").update(readFileSync(path)).digest("hex");}return out;};

async function generate(label:string,options:{request?:any;maximumElapsedMilliseconds:number;cwd:string;ambient:Record<string,string>;mutate?:(root:string)=>void}){
 const base=mkdtempSync(join(tmpdir(),"xk-")),root=join(base,"checkout"),workspace=join(base,"ws");mkdirSync(root);mkdirSync(workspace);
 writeFileSync(join(root,"package.json"),'{"name":"fx","workspaces":["*"]}\n');writeFileSync(join(root,"project.json"),"{}\n");
 for(const [path,text] of Object.entries({"gen/src/a.txt":"a\n","gen/script.ts":"//\n","course/src/b.txt":"b\n","course/script.ts":"//\n","course/README.md":"r\n"})){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);}
 options.mutate?.(root);
 const implementation={version:1,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"a".repeat(64),loaderSha256:"b".repeat(64)};writeFileSync(implementation.snapshot,"{}");writeFileSync(implementation.runner,"//");
 const previousCwd=process.cwd(),previousEnv={...process.env};Object.assign(process.env,options.ambient);process.chdir(options.cwd);
 try{
  const asked=options.request??request,capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","course:test"],"bazel",{}),plan=api.planBazelTasksV1(graph,asked);
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:keys-probe",maximumElapsedMilliseconds:options.maximumElapsedMilliseconds},capabilities,Date.now()),async original=>{
   process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
   await api.generateBazelWorkspaceV1(root,graph,asked,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:keys-probe",0),implementation,()=>{});
  });
  const files=digest(workspace),envelope=readFileSync(join(workspace,"runtime.json"),"utf8").includes("SEMIO_SCRIPT_PROCESS_INVOCATION");
  return {label,base,workspace,files:Object.fromEntries(Object.entries(files).map(([k,v])=>[k.replaceAll(base,"<base>"),v])),runtimeCarriesEnvelope:envelope,labels:Object.fromEntries(Object.values(plan.tasks).map((t:any)=>[t.id,t.label]))};
 }finally{process.chdir(previousCwd);process.env={...previousEnv};}
}

const out=process.argv[2]??"keys-probe.json",cwdA=realpathSync(tmpdir());
const execroot=mkdtempSync(join(tmpdir(),"xk-exec-"));
const a=await generate("A deadline 30s",{maximumElapsedMilliseconds:30000,cwd:repo,ambient:{}}),
 b=await generate("B deadline 90min, other cwd, ambient noise",{maximumElapsedMilliseconds:5400000,cwd:cwdA,ambient:{SEMIO_NOISE:String(Math.random()),TMPDIR:"/tmp/other",HOME:"/tmp/other-home"}}),
 c=await generate("C source edit gen/src/a.txt",{maximumElapsedMilliseconds:30000,cwd:repo,ambient:{},mutate:root=>writeFileSync(join(root,"gen/src/a.txt"),"a changed\n")}),
 e=await generate("E other selection (typecheck only, args)",{request:{tasks:["course:typecheck"],args:["--quiet"]},maximumElapsedMilliseconds:30000,cwd:repo,ambient:{}}),
 d=await generate("D source edit course/src/b.txt",{maximumElapsedMilliseconds:30000,cwd:repo,ambient:{},mutate:root=>writeFileSync(join(root,"course/src/b.txt"),"b changed\n")});
const diff=(x:any,y:any)=>[...new Set([...Object.keys(x.files),...Object.keys(y.files)])].sort().filter(k=>x.files[k]!==y.files[k]).map(k=>k.replace(/^.*\/(?=[^/]+$)/u,""));
const shared=Object.keys(a.labels).filter(id=>e.labels[id]),sameAcrossSelections=Object.fromEntries(shared.map(id=>[id,[".json",".input"].map(x=>a.files[a.labels[id]+x]===e.files[e.labels[id]+x]||(Object.keys(a.files).find(k=>k.endsWith(a.labels[id]+x))===undefined?"n/a":false))])),report={sameAcrossSelections,a:a.label,b:b.label,runtimeCarriesEnvelope:{a:a.runtimeCarriesEnvelope,b:b.runtimeCarriesEnvelope},filesCompared:Object.keys(a.files).length,diffAB:diff(a,b),diffAC:diff(a,c),diffAD:diff(a,d),labels:a.labels,fileDigestsA:a.files};
writeFileSync(out,JSON.stringify(report,null,1));console.log(JSON.stringify({diffAB:report.diffAB,diffAC:report.diffAC,diffAD:report.diffAD,filesCompared:report.filesCompared,runtimeCarriesEnvelope:report.runtimeCarriesEnvelope},null,1));
for(const row of [a,b,c,d,e])rmSync(row.base,{recursive:true,force:true});rmSync(execroot,{recursive:true,force:true});
