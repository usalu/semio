import {readFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",B=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");
await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs");
const storage=await import(B+"📥️inputs/💾️storage/🟨️.mjs"),rev=await import(B+"📥️inputs/🔏️revision/🟨️.mjs"),{mkdtempSync}=await import("node:fs"),{tmpdir}=await import("node:os");
const staging=mkdtempSync(join(tmpdir(),"prof-staging-"));
const nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const reg=JSON.parse(readFileSync(W+"/📋️build-owners.json","utf8"));
const projectsByRoot=reg.owners.flatMap(o=>[[o.root,o.name],...(o.nativeRoots??[]).map(r=>[r,o.name])]);
const contracts=nodes.projectGeneratorContractsV1(W);
const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:W,registry:null,contracts,projectsByRoot,componentRoots:[],commandInputs:null,lateClosures:false}));
env.projectsByRoot=new class extends Map{get(k){return super.get(k)??"dummy:"+k;}}(env.projectsByRoot);
const names=process.env.OWNERS?process.env.OWNERS.split(","):reg.owners.filter(x=>new RegExp(process.env.PATTERN??"^@semio-tech\\/stdio-.*-rs$").test(x.name)).filter((_,i)=>i%Number(process.env.STEP??1)===0).slice(0,Number(process.env.COUNT??14)).map(x=>x.name);
const si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
let total=0;
for(const name of names){
  const o=reg.owners.find(x=>x.name===name),json={...JSON.parse(readFileSync(join(W,o.manifest),"utf8")),root:o.root};
  const t=performance.now(),c=process.cpuUsage();
  const r=await rev.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published(){}},()=>obs.observeInferenceSnapshotV1(()=>nodes.inferProjectBodyV1(env,{name,root:o.root,json,native:true,pipeline:false,selectedInputs:[]},undefined))));
  const u=process.cpuUsage(c);
  total+=performance.now()-t;if(process.env.RELEASE==="1")si.releaseOwnerStateV1(env.scripts,{shrink:false});
  const m=process.memoryUsage();console.log("[DEBUG]",name,Math.round(m.rss/1048576),"MB rss",Math.round(m.heapUsed/1048576),"MB heap",Math.round(m.external/1048576),"MB ext",Math.round(performance.now()-t),"ms wall",Math.round((u.user+u.system)/1000),"ms cpu",Object.keys(r.targets??{}).length,"targets",r.unavailable?"UNAVAILABLE":"");
}
console.log("[DEBUG] total",Math.round(total),"ms for",names.length);
