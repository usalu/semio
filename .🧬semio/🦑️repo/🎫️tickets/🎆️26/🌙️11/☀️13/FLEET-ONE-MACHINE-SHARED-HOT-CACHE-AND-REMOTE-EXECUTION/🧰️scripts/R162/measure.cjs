const fs=require("node:fs"),path=require("node:path"),os=require("node:os");
const MOD=process.env.MOD, DIRS=+(process.env.DIRS||20000), RECHECKS=+(process.env.RECHECKS||3);
const calls={};const wrap=n=>{const o=fs[n];fs[n]=function(...a){calls[n]=(calls[n]||0)+1;return o.apply(this,a);};};
const root=fs.mkdtempSync(path.join(process.env.SCR,"tree-"));
fs.writeFileSync(path.join(root,"package.json"),JSON.stringify({name:"r",semio:{workspace:{schemaVersion:1,members:["pkgs/**"],owners:["*/package.json"]}}}));
let n=0;const fan=Math.ceil(Math.cbrt(DIRS));
fs.mkdirSync(path.join(root,"pkgs"));
for(let a=0;a<fan;a++)for(let b=0;b<fan;b++){const d=path.join(root,"pkgs","a"+a,"b"+b);fs.mkdirSync(d,{recursive:true});for(let c=0;c<fan;c++){fs.mkdirSync(path.join(d,"c"+c));n++;}if((a+b)%5===0)fs.writeFileSync(path.join(d,"package.json"),JSON.stringify({name:"p"+a+b}));}
["lstatSync","opendirSync","fstatSync","openSync","readSync","closeSync","readdirSync"].forEach(wrap);
const {createBunWorkspaceOwnershipV1}=require(MOD);
const policy=process.env.POLICY;
const own=createBunWorkspaceOwnershipV1({ownershipPath:policy,ownershipSource:fs.readFileSync(policy,"utf8")});
const snap=()=>JSON.stringify(calls);const reset=()=>{for(const k in calls)calls[k]=0;};
(async()=>{ await new Promise(d=>setTimeout(d,2300));
 await own.publishBunWorkspaceOwnership(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}});
 let adv=0;const ctl={signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{adv++;}};
 reset();const t0=performance.now();let t1,t2;
 await own.withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},ctl,async c=>{t1=performance.now();console.log("derive ms",(t1-t0).toFixed(0),"calls",snap(),"advances",adv);reset();adv=0;
  for(let i=0;i<RECHECKS;i++){const s=performance.now();await c.recheck();console.log(" recheck",i,(performance.now()-s).toFixed(0),"ms",snap(),"advances",adv);reset();adv=0;}
  t2=performance.now();});
 console.log("end ms",(performance.now()-t2).toFixed(0),snap(),"dirs",n);
 const k=await (async()=>{const s=performance.now();reset();let a2=0;await own.withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{...ctl,advance:async()=>{a2++}},async c=>{});console.log("second command (same process, seeded cache) ms",(performance.now()-s).toFixed(0),snap());})();
 fs.rmSync(root,{recursive:true,force:true});
})();
