// [gram-g] the `build:modules` stretch alone: observedSourceAnalysisAsyncV1(module imports) over every project .ts/.js/.mjs file at a chosen concurrency, rss/heap sampled. env: CONC=32 LIMIT=0 STORE=/private/tmp/gram-g/mod
import{execFileSync}from"node:child_process";import{mkdirSync,rmSync,existsSync}from"node:fs";import{join}from"node:path";import{heapStats}from"bun:jsc";
const ROOT="/Users/ueli/Documents/semio",L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",E=process.env,CONC=Number(E.CONC??"32"),LIMIT=Number(E.LIMIT??"0"),STORE=E.STORE??"/private/tmp/gram-g/mod";
rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
const mb=n=>Math.round(n/1048576);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs"),storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs"),revision=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");
let files=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>/\.[cm]?[jt]sx?$/.test(f)&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
if(LIMIT)files=files.slice(0,LIMIT);
console.log("files",files.length);
let peak=0,done=0;const t0=performance.now();
const sample=tag=>{const m=process.memoryUsage(),h=heapStats();peak=Math.max(peak,m.rss);console.log(JSON.stringify({tag,t:Math.round((performance.now()-t0)/1000),done,rss:mb(m.rss),heap:mb(h.heapSize),extra:mb(h.extraMemorySize),objs:h.objectCount}));};
const timer=setInterval(()=>sample("tick"),3000);
await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{
 const provider=inference.prepareSourceAnalysesV1?.().module;let cursor=0;
 await Promise.all(Array.from({length:CONC},async()=>{for(;;){const i=cursor++;if(i>=files.length)return;try{await obs.observedSourceAnalysisAsyncV1(provider,join(ROOT,files[i]));}catch(e){if(e.code!=="ENOENT")throw e;}done++;}}));
})));
clearInterval(timer);sample("end");Bun.gc(true);sample("afterGC");console.log("peak rss",mb(peak));
process.exit(0);
