import{readFileSync,mkdirSync}from"node:fs";import{join}from"node:path";import{deserialize}from"node:v8";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const E=process.env,wave=E.R97_WAVE,name=E.R97_OWNER,store=E.R97_STORE,out=E.R97_OUT;
const svc=await import(PN+"🏊️project-farm/👷️service/🟨️.mjs");
const units=JSON.parse(readFileSync(wave+"/units.json","utf8")),head=JSON.parse(readFileSync(wave+"/head.json","utf8")),session=deserialize(readFileSync(wave+"/session.bin"));
const unit=units.find(u=>u.name===name);if(!unit)throw Error("no unit "+name);
const mb=n=>Math.round(n/1048576);
console.log(JSON.stringify({t:"start",rss:mb(process.memoryUsage().rss)}));
const stagingBase=E.R97_STAGING;mkdirSync(stagingBase,{recursive:true});
await svc.open({...session,workspaceRoot:R,closureStore:undefined,stagingBase,authority:undefined},{slot:0,generation:1});
console.log(JSON.stringify({t:"opened",rss:mb(process.memoryUsage().rss)}));
const pn=await import(PN+"🟨️.mjs");const env=svc.sessionEnvironmentV1();
const m0=()=>{const m=process.memoryUsage();return {rss:mb(m.rss),heap:mb(m.heapUsed),ext:mb(m.external)}};
const ta=performance.now();const owner=pn.ownerJobV1(env,head.globals,unit);console.log(JSON.stringify({t:"ownerJob",ms:Math.round(performance.now()-ta),...m0(),nativeCache:env.nativeCache.size,nativeClosures:env.nativeClosures.size}));
if(E.R97_ONLY==="job"&&false){const JSC0=await import("bun:jsc");console.log(JSON.stringify({t:"types",...Object.fromEntries(Object.entries(JSC0.heapStats().objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,8))}));process.exit(0);}
const cachePath=E.R97_DATA+"/np/"+owner.cacheKey+".json";
const W=await import(PN+"🔱️warm/🟨️.mjs");const {statSync}=await import("node:fs");const {brotliDecompressSync}=await import("node:zlib");
console.log(JSON.stringify({t:"recfile",bytes:statSync(cachePath).size,text:brotliDecompressSync(readFileSync(cachePath)).length,...m0()}));
const rec=W.readStoredRecordV1(cachePath,R);console.log(JSON.stringify({t:"read",...m0()}));console.log(JSON.stringify({t:"record",recipes:Object.keys(rec?.value?.closureRecipes??{}).length,inputs:Object.keys(rec?.value?.closureInputs??{}).length}));
if(E.R97_ONLY==="read"){console.log(JSON.stringify({t:"vkeys",...Object.fromEntries(Object.entries(rec.value).map(([k,v])=>[k,JSON.stringify(v).length]))}));process.exit(0);}
const job={name,json:unit.json,globals:head.globals,warmKey:"r97"+Date.now(),warm:true,directory:E.R97_PARTS,closureStore:store,cachePath};
const OB=await import(B+"👁️observation/🟨️.mjs");const SI=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const JSC=await import("bun:jsc");
const t0=performance.now();let last="";if(E.R97_PROF==="1")JSC.startSamplingProfiler();
const prof=()=>{const tr=JSC.samplingProfilerStackTraces();const items=tr.traces??[];console.log("PROFKEYS "+JSON.stringify(Object.keys(tr))+" "+typeof items[0]+" "+JSON.stringify(items[0]).slice(0,300));const top=new Map(),inc=new Map();for(const t of items){const ls=(typeof t==="string"?t.split("\n"):(t.frames??t.stack??[]).map(f=>f.name?f.name+"@"+String(f.sourceURL||f.url||"").slice(-40)+":"+(f.lineNumber??""):JSON.stringify(f).slice(0,80))).map(x=>String(x).trim()).filter(Boolean);const f=ls[0]||"";top.set(f,(top.get(f)||0)+1);for(const l of new Set(ls.slice(0,40)))inc.set(l,(inc.get(l)||0)+1);}console.log("PROFTOP "+JSON.stringify([...top].sort((a,b)=>b[1]-a[1]).slice(0,14)));console.log("PROFINC "+JSON.stringify([...inc].sort((a,b)=>b[1]-a[1]).slice(0,40)));};
const tick=setInterval(()=>{const m=process.memoryUsage(),h=JSC.heapStats();const c=SI.closureEntryCountsV1();const line={s:Math.round((performance.now()-t0)/1000),rss:mb(m.rss),heap:mb(h.heapSize),extra:mb(h.extraMemorySize),ext:mb(m.external),ab:mb(m.arrayBuffers),objs:h.objectCount,c};const k=JSON.stringify([line.s,line.rss]);if(E.R97_PROF==="1"&&line.s>=Number(E.R97_PROFAT||8)&&!globalThis.__p){globalThis.__p=1;prof();}
if(E.R97_TICK==="1"){console.log("TICK "+JSON.stringify(line));if(line.s%4===0){const ty=Object.entries(h.objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,7);console.log("TYPES "+line.s+" "+JSON.stringify(ty)+" "+JSON.stringify(OB.inferenceCacheSizesV1())+" "+JSON.stringify(SI.layerVerdictCountsV1()));}}},1000);
const r=await svc.run(job);
console.log(JSON.stringify({t:"done",ms:Math.round(performance.now()-t0),reply:r,rss:mb(process.memoryUsage().rss),maxrss:mb(process.resourceUsage().maxRSS*1024)}));
clearInterval(tick);process.exit(0);
