import {existsSync,readFileSync,readdirSync,mkdtempSync,statSync,rmSync}from"node:fs";
import {join}from"node:path";
import {tmpdir}from"node:os";import {realpathSync}from"node:fs";
import {createHash}from"node:crypto";
const ROOT=process.cwd(),L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const [rootsArg,out]=process.argv.slice(2),roots=(rootsArg.startsWith("@list:")?readFileSync(rootsArg.slice(6),"utf8").trim().split("\n"):rootsArg.split("|")).filter(Boolean),faithful=process.env.INFER_F_FAITHFUL==="1";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");
await inference.libraryBootstrap;
const service=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs");
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
const base=mkdtempSync(join(realpathSync(tmpdir()),"semio-graph-farm-"));
let projectsByRoot=[],componentRoots=[],commandInputs=null;
if(faithful){
 const {execFileSync}=await import("node:child_process");
 const files=execFileSync("git",["ls-files","-z","--","*📋️project.json","*Cargo.toml"],{cwd:ROOT,maxBuffer:1<<28}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio"));
 for(const f of files.filter(f=>f.endsWith("📋️project.json"))){try{const j=JSON.parse(readFileSync(join(ROOT,f),"utf8")),r=f.includes("/")?f.slice(0,f.lastIndexOf("/")):".";projectsByRoot.push([r,j.name]);}catch{}}
 {const nm0=await import(L+"🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs"),have=new Set(projectsByRoot.map(([r])=>r));
  for(const f of files.filter(f=>f.endsWith("Cargo.toml"))){const r=f.includes("/")?f.slice(0,f.lastIndexOf("/")):".";if(have.has(r))continue;try{const m=nm0.readToml(join(ROOT,f));if(m.package?.name){projectsByRoot.push([r,m.package.name]);have.add(r);}}catch{}}}
 const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),ct=await import(L+"🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs");
 componentRoots=ct.componentCrateRoots(files,ROOT);
 const scripts=si.createScriptInputCache({workspaceRoot:ROOT,registry,lateClosures:true});
 const ci=await inference.default.createNodesV2 && await (await import(L+"👁️observation/🟨️.mjs")).observeInferenceSnapshotV1(()=>si.closureApiV1(scripts,ROOT).nativeCommandInputs());
 const handle=si.closureHandleOfV1(ci);commandInputs=handle?{closureRecipe:handle.recipe}:ci;
}
const pn=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");let peak=0;const t0=performance.now();
const obs=await import(L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const session={version:2,contracts:pn.projectGeneratorContractsV1(ROOT),workspaceRoot:ROOT,registry,projectsByRoot,componentRoots,commandInputs,lateClosures:true,stagingBase:base};
const O=await import(L+"👁️observation/🟨️.mjs");
const env=await O.observeInferenceSnapshotV1(()=>pn.createProjectInferenceEnvironmentV1(session));
const root=process.argv[2].split("|")[0];
const nm0=await import(L+"🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs");
const name=nm0.readToml(join(ROOT,root,"Cargo.toml")).package.name;
const ct=await import(L+"🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs");
const handle=si.closureHandleFromRecipeV1&&commandInputs?.closureRecipe?si.closureHandleFromRecipeV1(commandInputs.closureRecipe):commandInputs;
const targets=await O.observeInferenceSnapshotV1(()=>ct.cargoTargets(root,ROOT,handle,env.scripts));
const json={name,root,projectType:"library",tags:["language:cargo","discovery:manifest"],targets};
const t=()=>Math.round(performance.now()-t0)/1000;
const timer=setInterval(()=>console.error("[DEBUG] t="+t()+" rss="+Math.round(process.memoryUsage().rss/1048576)),5000);
const pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs"),stg=await import(L+"📥️inputs/💾️storage/🟨️.mjs");
await pin.withImplementationPinV1(()=>stg.withBuildGraphStagingV1({root:join(base,"w0-g1"),published:()=>{}},()=>O.observeInferenceSnapshotV1(async()=>{
 const project=await pn.inferProjectBodyV1(env,{name,root,json,native:true,pipeline:false,selectedInputs:[]},undefined);
 console.error("[DEBUG] body done t="+t(),"closureRecipes="+Object.keys(project.closureRecipes??{}).length);
 const r=await obs.resolveClosureRefsV1(project,ROOT,env.scripts);
 console.error("[DEBUG] resolve done t="+t(),"rows="+r.rows.length,"derived="+r.derived.length,"bytes="+JSON.stringify(r.project).length);
})));
clearInterval(timer);process.exit(0);
