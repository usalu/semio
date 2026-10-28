// [asm-g] graph-free timing of the constant tests + policy discovery of a delta generation (production testCaseProjects / policy createNodesV2 inside the observation capture), wall + RSS + captured rows.
import{execFileSync}from"node:child_process";import{existsSync,mkdirSync,rmSync}from"node:fs";import{readFileSync}from"node:fs";import{join}from"node:path";
const ROOT="/Users/ueli/Documents/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",TEST=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs",E=process.env;
process.chdir(ROOT);
const mb=n=>Math.round(n/1048576),t0=performance.now();let peak=0;
const sample=()=>{const r=process.memoryUsage().rss;if(r>peak)peak=r;if(r>Number(E.RSS_CAP_MB??"3000")*1048576){console.error("[DEBUG] ABORT rss "+mb(r));process.exit(3);}};setInterval(sample,100).unref();
const row=(name,extra={})=>{sample();console.error("[DEBUG] "+JSON.stringify({name,t:Math.round(performance.now()-t0),rss:mb(process.memoryUsage().rss),peak:mb(peak),...extra}));};
Bun.plugin({name:"asm",setup(b){b.onLoad({filter:/modules\/[^/]*test\/[^/]*\.mjs$/},async args=>{if(args.path!==TEST)return undefined;const t=await Bun.file(args.path).text();return{contents:t+"\nexport const __asm={testCaseProjects,withTestInferenceWriterV1,taxonomy};\n",loader:"js"};});}});
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const tests=await import(TEST),policies=await import(B+"🕸️graph/📥️inference/🛡️policy/🟨️.mjs");
const obs=await import(B+"👁️observation/🟨️.mjs"),revision=await import(B+"📥️inputs/🔏️revision/🟨️.mjs"),storage=await import(B+"📥️inputs/💾️storage/🟨️.mjs");
const admission=await import(B+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
const all=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
row("loaded",{tracked:all.length});
const STORE=E.STORE??"/private/tmp/asm-g/store";if(!E.KEEP)rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
const context={workspaceRoot:ROOT,registry};
const result=await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{
 const cap=await obs.captureObservedSourceInputsV1(async()=>{
  let t=performance.now();
  const features=all.filter(f=>f.endsWith(".feature")),scripts=all.filter(f=>f.endsWith("📜️script.ts"));
  const testContributions=await tests.__asm.testCaseProjects(features,{},context);row("tests",{ms:Math.round(performance.now()-t),features:features.length,owners:testContributions.length});t=performance.now();
  const owners=Object.assign({},...testContributions.map(([,c])=>c.projects??{}));
  const policy=await policies.default.createNodesV2[1](scripts,{},{workspaceRoot:ROOT,projects:owners});row("policies",{ms:Math.round(performance.now()-t),scripts:scripts.length,owners:policy.length});
  return{tests:testContributions.length,policy:policy.length};
 },undefined,false);
 row("captured",{rows:cap.observations.length,value:cap.value});
 const kinds={};for(const r of cap.observations)kinds[r.kind]=(kinds[r.kind]??0)+1;row("kinds",kinds);
 return cap.observations.length;
})));
row("done",{result});
console.log(JSON.stringify({wallMs:Math.round(performance.now()-t0),peakRssMB:mb(peak)}));
