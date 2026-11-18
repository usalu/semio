import{existsSync,readFileSync,realpathSync,mkdtempSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";import{join}from"node:path";import{deserialize}from"node:v8";import{brotliDecompressSync}from"node:zlib";import*as JSC from"bun:jsc";
const R="/Users/ueli/Documents/semio",B=(process.env.R107_CODE??R)+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const[dir,countArg]=process.argv.slice(2),count=Number(countArg||100);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");const{projectStampPathsV1}=await import(B+"🕸️graph/📥️inference/🔖️stamps/🟨️.mjs") as any;
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const{resolveClosureRefsV1}=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs") as any;
const OB:any=await import(B+"👁️observation/🟨️.mjs");const{inferenceInventoryReferencesHoldV1}=await import(B+"👁️observation/📥️reuse/🟨️.mjs") as any;
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
const recordOfKey=(key:string)=>{try{const text=brotliDecompressSync(readFileSync(join(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(text.slice(text.indexOf("\n")+1));}catch{return undefined;}};
const stored=(r:any)=>existsSync(store+"/e/"+createHash("sha256").update(R+"\0"+SI.closureRecipeKeyV1(r)).digest("hex")+".json.br");
const tJobs=performance.now(),cJobs=process.cpuUsage();const all=process.env.R107_ALL==="1",jobs:any[]=[];let skipped=0;
const jobs2:any[]=[];for(const u of units){const v=recordOfKey(ownerRecordKeyV1(u.json.root));if(!v)continue;jobs2.push({u,v});if(jobs2.length>=count)break;}
const t0=performance.now(),mode=process.env.R117_MODE??"resolved",keep:any[]=[];let json=0;
const{sharedSourceFilesV1:SF}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs") as any;
const strBytes=(v:any,depth=0):number=>{if(typeof v==="string")return v.length;if(depth>3||v===null||typeof v!=="object")return 8;let t=0;if(v instanceof Map){for(const[k,x]of v){t+=strBytes(k,depth+1)+strBytes(x,depth+1);}return t;}if(v instanceof Set){for(const x of v)t+=strBytes(x,depth+1);return t;}if(Array.isArray(v)){for(const x of v)t+=strBytes(x,depth+1);return t;}for(const k of Object.keys(v))t+=k.length+strBytes(v[k],depth+1);return t;};
const caches=()=>{const sc:any=env.scripts,out:any={};for(const k of Object.keys(sc)){const v=sc[k];if(v instanceof Map||v instanceof Set)out[k]=[v.size,Math.round(strBytes(v)/1048576)];}out.sharedBytes=Math.round((sc.sharedBytes??0)/1048576);out.sharedSourceFiles=[SF.size,Math.round(strBytes(SF)/1048576)];return out;};
const snap=(n:number)=>{Bun.gc(true);const h=(JSC as any).heapStats(),m=process.memoryUsage();console.log("CACHES",n,JSON.stringify(caches()));console.log("AT",n,JSON.stringify({heap:Math.round(h.heapSize/1048576),extra:Math.round(h.extraMemorySize/1048576),objects:h.objectCount,rss:Math.round(m.rss/1048576),json:Math.round(json/1048576),sec:Math.round((performance.now()-t0)/1000),top:Object.entries(h.objectTypeCounts).sort((a:any,b:any)=>b[1]-a[1]).slice(0,5)}));};
snap(0);let n=0;
await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{for(const{u,v}of jobs2){
 let project=v;
 if(mode==="resolved"){try{const r=await resolveClosureRefsV1(v,R,env.scripts,"hash",undefined);project=r;}catch{}}
 SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});keep.push(project);try{json+=JSON.stringify(project).length;}catch{}
 n++;if(n%3===0||n===jobs2.length)snap(n);
}}));
console.log("RESULT",JSON.stringify({mode,owners:n,jsonMB:Math.round(json/1048576)}));process.exit(0);
