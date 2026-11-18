import{readFileSync}from"node:fs";import{deserialize}from"node:v8";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const[dir,countArg,skipArg]=process.argv.slice(2),count=Number(countArg||20),skip=Number(skipArg||0);
if(process.env.R114_PRELOAD)await import(process.env.R114_PRELOAD);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const OB:any=await import(B+"👁️observation/🟨️.mjs");
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
SI.bindClosureStoreV1?.(R,store,true);SI.closureApiV1?.(env.scripts,R);
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};
const out:any={owners:0,verdicts:{},cpuMs:0,wallMs:0,readMs:0,per:[]};let n=0,s=0;
const t0=performance.now(),c0=cpu();
for(const u of units){if(s++<skip)continue;if(n>=count)break;
 const job=pn.ownerJobV1(env,head.globals??head,u),key=pn.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs),cachePath=dataDir+"/np/"+job.cacheKey+".json";
 const c=cpu(),t=performance.now();let v:any;
 try{v=await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(()=>OB.verifyStoredProjectRecordV1({cachePath,key})));}catch(e:any){v={verdict:"error:"+String(e?.message).slice(0,60)};}
 SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});
 out.verdicts[v.verdict]=(out.verdicts[v.verdict]||0)+1;out.per.push([Math.round(cpu()-c),Math.round(performance.now()-t),v.rows??0]);n++;
}
out.owners=n;out.cpuMs=Math.round(cpu()-c0);out.wallMs=Math.round(performance.now()-t0);out.cpuPerOwner=Math.round(out.cpuMs/Math.max(1,n));
console.log("VERIFY",JSON.stringify({...out,per:undefined}));console.log("PER",JSON.stringify(out.per.slice(0,40)));process.exit(0);
