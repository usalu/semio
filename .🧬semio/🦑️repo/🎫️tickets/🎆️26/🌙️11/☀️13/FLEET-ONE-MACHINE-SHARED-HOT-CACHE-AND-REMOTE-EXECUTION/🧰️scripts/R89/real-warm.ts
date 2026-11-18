import{existsSync,mkdirSync,readFileSync,realpathSync}from"node:fs";import{createHash}from"node:crypto";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";import{deserialize}from"node:v8";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const[dir,role,K,count,partDir,warmModule]=[process.argv[2],Number(process.argv[3]),Number(process.argv[4]),Number(process.argv[5]),process.argv[6],process.argv[7]];
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const OB:any=await import(B+"👁️observation/🟨️.mjs");
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;const W:any=await import(warmModule);
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=process.env.WARM_STORE||dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
const recordOfKey=(key:string)=>{try{const t=brotliDecompressSync(readFileSync(join(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(t.slice(t.indexOf("\n")+1));}catch{return undefined;}};
const stored=(r:any)=>existsSync(dataDir+"/ce/e/"+createHash("sha256").update(R+"\0"+SI.closureRecipeKeyV1(r)).digest("hex")+".json.br");
const jobs:any[]=[];let seen=0;
for(const u of units){const v=recordOfKey(ownerRecordKeyV1(u.json.root));if(!v)continue;const rec=Object.values(v.closureRecipes??{}) as any[];if(!rec.length||!rec.every(stored))continue;if((seen++)%K!==role)continue;jobs.push({job:pn.ownerJobV1(env,head.globals??head,u),unit:u});if(jobs.length>=count)break;}
const ports={keyText:(each:any)=>pn.ownerRecordKeyTextV1(pn.projectRecordKeyV1(each.head,each.root,each.json,undefined,each.selectedInputs)),read:(p:string)=>W.readStoredRecordV1(p,R),settle:(o:any,v:any)=>OB.settleClosureRowsV1(o,v)};
SI.bindClosureStoreV1(R,store,true);SI.closureApiV1(env.scripts,R);mkdirSync(partDir,{recursive:true});const outcomes:Record<string,number>={};const t0=performance.now(),c0=process.cpuUsage();let n=0;
for(const{job}of jobs){const done=await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(()=>W.warmOwnerV1({...job,warmKey:W.warmDigestV1(job.name)},{directory:partDir,cachePath:join(dataDir,"np",job.cacheKey+".json"),ports})));outcomes[done.outcome]=(outcomes[done.outcome]||0)+1;SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});n++;}
console.log("COUNTS",JSON.stringify(SI.closureEntryCountsV1?.()),JSON.stringify(SI.layerVerdictCountsV1?.()),JSON.stringify(OB.observationLookupClockV1?.()));const sec=(performance.now()-t0)/1000,u=process.cpuUsage(c0);
console.log("RESULT",JSON.stringify({role,K,owners:n,seconds:Math.round(sec*100)/100,perOwnerMs:Math.round(sec*1000/Math.max(1,n)),cpuSeconds:Math.round((u.user+u.system)/1e4)/100,outcomes,rssMB:Math.round(process.memoryUsage().rss/1048576)}));process.exit(0);
