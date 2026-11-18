import{existsSync}from"node:fs";import{join}from"node:path";
import{load,R,B}from"./common.ts";
const[waveDir,mode,countArg]=process.argv.slice(2),count=Number(countArg||12);
const m=await load(waveDir),{units,env,OB}=m;
const OD:any=await import(B+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"),PN:any=m.pn;
const natives=units.filter((u:any)=>existsSync(join(R,u.json.metadata?.nativeRoot??u.json.root,"Cargo.toml"))).slice(0,count);
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};const per:number[]=[],childCpu:number[]=[],replay:number[]=[];let bytes=0,layerBytes=0;const seen=new Set<string>(),vdir=require("node:fs").mkdtempSync(join(require("node:os").tmpdir(),"r121au-"));let rowsN=0;const t0=performance.now();
await OB.observeInferenceSnapshotV1(async()=>{for(const u of natives){const nativeRoot=u.json.metadata?.nativeRoot??u.json.root,c=cpu();
 if(mode==="inline"){OD.projectAssemblyAuthorityV1(R,nativeRoot,env.projectsByRoot,undefined,env.nativeCache,env.nativeClosures);}
 else{const r=await PN.authorityOfOwnerV1(env,{nativeRoot,pipeline:false,directory:vdir});rowsN+=r.rows.length;childCpu.push(cpu()-c);const c2=cpu();const text=JSON.stringify({digest:r.digest,rows:r.rows,layerDigests:r.layerDigests});const back=JSON.parse(text);const fresh:any[]=[];for(const n of new Set<string>(back.layerDigests)){if(seen.has(n))continue;seen.add(n);const rows=PN.readAuthorityLayerV1(vdir,n);fresh.push(...rows);layerBytes+=JSON.stringify(rows).length;}OB.retainObservedRowsV1([...back.rows,...fresh]);replay.push(cpu()-c2);bytes+=text.length;}
 per.push(cpu()-c);}});
const s=[...per].sort((a,b)=>a-b);console.log("RESULT",JSON.stringify({mode,owners:natives.length,cpuMsTotal:Math.round(per.reduce((a,b)=>a+b,0)),first:Math.round(per[0]),median:Math.round(s[Math.floor(s.length/2)]),max:Math.round(s[s.length-1]),rows:rowsN,bytes,layerBytes,distinctLayers:seen.size,childMsTotal:Math.round(childCpu.reduce((a,b)=>a+b,0)),parentReplayMsTotal:+replay.reduce((a,b)=>a+b,0).toFixed(1),maxRssMb:Math.round(process.resourceUsage().maxRSS/1048576),wallSec:+((performance.now()-t0)/1000).toFixed(1)}));process.exit(0);
