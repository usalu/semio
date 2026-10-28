// [asm-g] graph-free: full tests discovery once (baseline), then ONE call per test owner in its own observation capture; law: per-owner result equals the baseline entry; reports ms + rows per owner and the shared-state cost.
import{execFileSync}from"node:child_process";import{existsSync,mkdirSync,rmSync,readFileSync,writeFileSync}from"node:fs";import{join}from"node:path";import{createHash}from"node:crypto";
const ROOT="/Users/ueli/Documents/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",TEST=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs",E=process.env;
process.chdir(ROOT);
const mb=n=>Math.round(n/1048576),t0=performance.now();let peak=0;
const sample=()=>{const r=process.memoryUsage().rss;if(r>peak)peak=r;if(r>Number(E.RSS_CAP_MB??"2500")*1048576){console.error("[DEBUG] ABORT rss "+mb(r));process.exit(3);}};setInterval(sample,100).unref();
const row=(name,extra={})=>{sample();console.error("[DEBUG] "+JSON.stringify({name,t:Math.round(performance.now()-t0),rss:mb(process.memoryUsage().rss),peak:mb(peak),...extra}));};
Bun.plugin({name:"asm",setup(b){b.onLoad({filter:/modules\/[^/]*test\/[^/]*\.mjs$/},async args=>{if(args.path!==TEST)return undefined;const t=await Bun.file(args.path).text();return{contents:t+"\nexport const __asm={testCaseProjects,testShared};\n",loader:"js"};});}});
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const tests=await import(TEST),obs=await import(B+"👁️observation/🟨️.mjs"),revision=await import(B+"📥️inputs/🔏️revision/🟨️.mjs"),storage=await import(B+"📥️inputs/💾️storage/🟨️.mjs"),admission=await import(B+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
const all=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
const features=all.filter(f=>f.endsWith(".feature"));row("loaded",{features:features.length});
const STORE=E.STORE??"/private/tmp/asm-g/store2";rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
const context={workspaceRoot:ROOT,registry};
const canon=v=>JSON.stringify(v,(k,x)=>x&&typeof x==="object"&&!Array.isArray(x)?Object.fromEntries(Object.keys(x).sort().map(key=>[key,x[key]])):x);
let SHARED;const out=await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{
 let t=performance.now();
 const SOLO=(E.SOLO??'').split(';').filter(Boolean);
 const sharedCap=E.SHARED==='1'?await obs.captureObservedSourceInputsV1(()=>tests.__asm.testShared(context),undefined,false):undefined;if(sharedCap){row('shared',{ms:Math.round(performance.now()-t),rows:sharedCap.observations.length});t=performance.now();}
 const full=SOLO.length?{value:[],observations:[]}:await obs.captureObservedSourceInputsV1(()=>tests.__asm.testCaseProjects(features,{},context),undefined,false);
 const fullMs=Math.round(performance.now()-t);row("full",{ms:fullMs,owners:full.value.length,rows:full.observations.length});
 const byRoot=SOLO.length?new Map(SOLO.map(r=>[r,{file:'',c:{},p:{}}])):new Map(full.value.map(([file,c])=>{const p=Object.values(c.projects)[0];return[p.root,{file,c,p}];}));
 const stats=[];let mismatches=0,done=0;const limit=Number(E.LIMIT??"40");
 for(const[root,entry]of byRoot){
  if(done>=limit)break;done++;
  const own=features.filter(f=>f.startsWith(root+"/"));
  t=performance.now();
  const one=await obs.captureObservedSourceInputsV1(()=>tests.__asm.testCaseProjects(own,{},sharedCap?{...context,testShared:sharedCap.value}:context),undefined,false);
  const ms=performance.now()-t;
  const same=SOLO.length?true:one.value.length===1&&canon(one.value[0])===canon([entry.file,entry.c]);
  if(!same){mismatches++;console.error("[DEBUG] MISMATCH "+root+" one="+one.value.length);}
  const kinds={};for(const r of one.observations)kinds[r.kind]=(kinds[r.kind]??0)+1;
  const sig=createHash('sha256').update([...new Set(one.observations.map(r=>r.kind+'\0'+r.path+'\0'+JSON.stringify(r.fact)))].sort().join('\n')).digest('hex').slice(0,16);stats.push({root,sig,uniq:new Set(one.observations.map(r=>r.kind+r.path)).size,features:own.length,ms:Math.round(ms),rows:one.observations.length,kinds});
 }
 const cpuU=process.cpuUsage();return{fullMs,stats,mismatches,cpuMs:Math.round((cpuU.user+cpuU.system)/1000)};
})));
row("done",{mismatches:out.mismatches,perOwnerMsSum:out.stats.reduce((a,s)=>a+s.ms,0),n:out.stats.length});
for(const s of out.stats.slice(0,12))console.error("[DEBUG] "+JSON.stringify(s));
writeFileSync((E.OUT??"/private/tmp/asm-g/per-owner.json"),JSON.stringify(out));
