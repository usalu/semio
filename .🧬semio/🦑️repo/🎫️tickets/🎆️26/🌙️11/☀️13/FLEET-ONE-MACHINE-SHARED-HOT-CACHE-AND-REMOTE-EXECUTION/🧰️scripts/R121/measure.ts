import{readFileSync,writeFileSync,mkdirSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";import{createHash}from"node:crypto";
import{load,R,B}from"./common.ts";
const[waveDir,mode,countArg,listFile]=process.argv.slice(2),count=Number(countArg||50);
const m=await load(waveDir),{SI,AS,OB,RL,units,env,recordOf,pn}=m;
const dir=join(tmpdir(),"r121-"+process.pid);mkdirSync(dir,{recursive:true});const log=RL.createRowLogV1({path:join(dir,"rows.log")});
const stat=(a:number[])=>{const s=[...a].sort((x,y)=>x-y);return a.length?{n:a.length,mean:+(a.reduce((x,y)=>x+y,0)/a.length).toFixed(1),median:+s[Math.floor(s.length/2)].toFixed(1),p90:+s[Math.floor(s.length*0.9)].toFixed(1),max:+s[s.length-1].toFixed(1)}:null;};
const keys=(l:any)=>[...l.keys()].sort();
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};
const accepted:string[]=[],cpus:number[]=[],rss:number[]=[];let misses=0,checked=0,bad=0;const t0=performance.now();
if(mode==="before"||mode==="check"){
 await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{
  for(const u of units){
   if(accepted.length>=count)break;const v=recordOf(u.json.root);if(!v||v.unavailable||!v.closureRecipes)continue;
   const c0=cpu();let r:any;
   try{r=await(process.env.R121_DERIVE==="1"?(f:any)=>f():SI.runClosureTakeV1)(()=>AS.resolveClosureRefsV1(v,R,env.scripts,"hash",log));}catch(e:any){misses++;continue;}
   const c1=cpu();accepted.push(u.json.root);cpus.push(c1-c0);rss.push(Math.round(process.memoryUsage().rss/1048576));
   if(mode==="check"){
    const p=await SI.runClosureTakeV1(()=>AS.resolveClosureRefsPlanV1(v,R,env.scripts,"hash"));
    if(p===undefined){bad++;console.log("NOPLAN",u.json.root);continue;}
    const a=AS.adoptClosureRefsV1(p.plan,R,env.scripts,log);checked++;
    const same=JSON.stringify(a.rows===undefined?[]:keys(a.rows))===JSON.stringify(keys(r.rows))&&JSON.stringify(p.project)===JSON.stringify(r.project)&&a.derived.length===r.derived.length&&r.derived.every((d:any,i:number)=>JSON.stringify(d.row)===JSON.stringify(a.derived.find((x:any)=>x.row.path===d.row.path&&x.row.fact===d.row.fact)?.row)&&JSON.stringify(keys(d.support))===JSON.stringify(keys(a.derived.find((x:any)=>x.row.path===d.row.path&&x.row.fact===d.row.fact).support)));
    if(!same){bad++;console.log("DIFF",u.json.root,keys(a.rows).length,keys(r.rows).length,a.derived.length,r.derived.length);}
   }
   SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});
  }
 }));
 if(listFile)writeFileSync(listFile,JSON.stringify(accepted));
 console.log("RESULT",JSON.stringify({mode,accepted:accepted.length,misses,checked,bad,cpuMsPerOwner:stat(cpus),rssMbMax:Math.max(0,...rss),maxRssMb:Math.round(process.resourceUsage().maxRSS/1048576),wallSec:Math.round((performance.now()-t0)/1000)}));
}
if(mode==="prep"){
 SI.bindClosureStoreV1(R,(await import("./common.ts")).store,true);let done=0;const t1=performance.now();
 await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{for(const u of units){if(done>=count)break;const v=recordOf(u.json.root);if(!v||v.unavailable||!v.closureRecipes)continue;try{await AS.resolveClosureRefsV1(v,R,env.scripts,"hash",log);done++;accepted.push(u.json.root);}catch(e:any){console.log("PREPFAIL",u.json.root,String(e?.code??e?.message).slice(0,80));}SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});}}));
 writeFileSync(listFile,JSON.stringify(accepted));console.log("RESULT",JSON.stringify({mode,prepared:done,wallSec:Math.round((performance.now()-t1)/1000)}));
}
if(mode==="after"){
 const owners:string[]=JSON.parse(readFileSync(listFile,"utf8")),vdir=join(dir,"verdicts");mkdirSync(vdir,{recursive:true});
 const children=Array.from({length:4},()=>{const p=Bun.spawn([process.execPath,"--preload",process.env.R121_PRELOAD!,new URL("./child.ts",import.meta.url).pathname,waveDir],{stdin:"pipe",stdout:"pipe",stderr:"inherit"}),waiting:((l:string)=>void)[]=[],reader=p.stdout.getReader(),dec=new TextDecoder();let buf="",ready:()=>void;const up=new Promise<void>(r=>ready=r);
  (async()=>{for(;;){const{value,done}=await reader.read();if(done)return;buf+=dec.decode(value);for(let c=buf.indexOf("\n");c>=0;c=buf.indexOf("\n")){const l=buf.slice(0,c);buf=buf.slice(c+1);if(l==="READY")ready();else waiting.shift()?.(l);}}})();
  return{p,up,ask:(req:any)=>new Promise<any>(res=>{waiting.push(l=>res(JSON.parse(l)));p.stdin.write(JSON.stringify(req)+"\n");p.stdin.flush();})};});
 await Promise.all(children.map(c=>c.up));const scripts=env.scripts;let next=0,adoptedN=0,none=0;const childCpu:number[]=[],pids=new Set<number>(),adoptCpu:number[]=[];
 Bun.gc(true);const rss0=Math.round(process.memoryUsage().rss/1048576),c0=cpu(),w0=performance.now();let peak=rss0;
 await Promise.all(children.map(async c=>{while(next<owners.length){const root=owners[next++];const said=await c.ask({root,dir:vdir});pids.add(said.pid);if(!said.refs){none++;continue;}childCpu.push(said.cpu);
   const a0=cpu();const got=m.pn.adoptOwnerRefsV1(said.refs,said.refs.headerDigest,R,scripts,log);adoptCpu.push(cpu()-a0);if(got){adoptedN++;void got.rows.length;}else none++;peak=Math.max(peak,Math.round(process.memoryUsage().rss/1048576));}}));
 const wall=(performance.now()-w0)/1000,total=cpu()-c0;
 console.log("RESULT",JSON.stringify({mode,owners:owners.length,adopted:adoptedN,none,children:pids.size,parentCpuMsPerOwner:+(total/Math.max(1,owners.length)).toFixed(1),adoptCpu:stat(adoptCpu),childCpuMsPerOwner:stat(childCpu),parentRssStartMb:rss0,parentRssPeakMb:peak,parentMaxRssMb:Math.round(process.resourceUsage().maxRSS/1048576),wallSec:+wall.toFixed(1),ownersPerMinute:Math.round(adoptedN/wall*60)}));
 for(const c of children){c.p.stdin.end();c.p.kill();}
}
rmSync(dir,{recursive:true,force:true});process.exit(0);
