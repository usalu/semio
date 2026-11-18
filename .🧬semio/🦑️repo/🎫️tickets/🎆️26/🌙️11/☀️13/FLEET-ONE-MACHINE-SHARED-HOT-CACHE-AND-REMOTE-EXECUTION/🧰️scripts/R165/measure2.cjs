const fs=require("node:fs"),path=require("node:path"),os=require("node:os");
const MOD=process.env.MOD,POLICY=process.env.POLICY,ROOT=process.env.TREE,RECHECKS=+(process.env.RECHECKS||2),SITES=process.env.SITES==="1";
const calls={},sites={};
const wrap=n=>{const o=fs[n];fs[n]=function(...a){calls[n]=(calls[n]||0)+1;if(SITES&&n==="lstatSync"){const s=(new Error().stack.split("\n")[2]||"").trim().replace(/\(.*[\\/]/,"(");sites[s]=(sites[s]||0)+1;}return o.apply(this,a);};};
["lstatSync","opendirSync","readdirSync","openSync","readSync"].forEach(wrap);
const {createBunWorkspaceOwnershipV1}=require(MOD);
const own=createBunWorkspaceOwnershipV1({ownershipPath:POLICY,ownershipSource:fs.readFileSync(POLICY,"utf8")});
const reset=()=>{for(const k in calls)calls[k]=0;for(const k in sites)delete sites[k];};
const top=()=>SITES?Object.entries(sites).sort((a,b)=>b[1]-a[1]).slice(0,8).map(([k,v])=>"    "+v+" "+k).join("\n"):"";
(async()=>{
 const sel=process.env.SELECT?JSON.parse(process.env.SELECT):{kind:"all"};
 await own.publishBunWorkspaceOwnership(ROOT,sel,{signal:new AbortController().signal,advance:async()=>{}});await new Promise(d=>setTimeout(d,2500));
 let adv=0;const ctl={signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{adv++;}};
 for(let round=0;round<2;round++){
 reset();adv=0;const t0=performance.now();let t1,blocked=0;
 const timer=setInterval(()=>{},1);
 await own.withCurrentBunWorkspaceOwnershipV1(ROOT,sel,ctl,async c=>{t1=performance.now();
  console.log(`[round ${round}] derive ${(t1-t0).toFixed(0)} ms lstat ${calls.lstatSync} readdir ${calls.opendirSync||0} paths ${c.observed().paths.length} adv ${adv}`);if(SITES)console.log(top());
  for(let i=0;i<RECHECKS;i++){reset();adv=0;const s=performance.now();await c.recheck();console.log(`   recheck ${i} ${(performance.now()-s).toFixed(0)} ms lstat ${calls.lstatSync} adv ${adv}`);}
 });clearInterval(timer);
 }
})();
