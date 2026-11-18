import{unlinkSync,mkdirSync,writeFileSync,utimesSync,readdirSync,rmSync,mkdtempSync,lstatSync,readFileSync}from"node:fs";import{join,dirname}from"node:path";import{tmpdir}from"node:os";import{spawnSync}from"node:child_process";
const WORK=process.argv[2];
const src=readFileSync(WORK+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts","utf8");
const a=src.indexOf("export function optionalEntryOfV1"),b=src.indexOf("/** 🗜️ Rewrites the member index");
const ts=new Bun.Transpiler({loader:"ts"}).transformSync(src.slice(a,b).replace(/export /g,""));
const prelude=`import{unlinkSync,readdirSync,lstatSync}from"node:fs";import{join}from"node:path";const absent=e=>e?.code==="ENOENT";const M={groupProtectedIdleMilliseconds:600000};const graphStorageSchema={definitions:{OptionalQuota:{definitions:{DefaultPolicy:{const:{lowWaterBytes:536870912}}}}}};
const memberGroupProtectedV1=(n,now,p)=>now-n<p;const compactGroupIndexV1=()=>{};const turn=async()=>{};
function optionalLeavesOfV1(root,group){const dir=root+"/inference/"+group,leaves=[],pending=[""];while(pending.length){const at=pending.pop();for(const e of readdirSync(join(dir,at),{withFileTypes:true})){const name=at?at+"/"+e.name:e.name;if(e.isDirectory()){pending.push(name);continue;}const path=join(dir,name),r=lstatSync(path);leaves.push({name,path,bytes:r.size,mtimeMs:r.mtimeMs,rank:/(?:^|\\/)ob\\//u.test(name)?2:/\\.pack(?:\\.[a-f0-9]{8}\\.tmp)?$/u.test(name)?1:0});}}return{leaves:leaves.sort((l,r)=>l.rank-r.rank||(l.name<r.name?-1:l.name>r.name?1:0)),newest:0};}
`;
writeFileSync(join(tmpdir(),"lru60-mod.mjs"),prelude+ts+"\nexport{retireBuildGraphOptionalEntriesLruV1,optionalEntryOfV1,optionalAcceleratorV1};");const mod=await import(join(tmpdir(),"lru60-mod.mjs"));
const HOUR=3600000,hex=(seed:number,length:number)=>Array.from({length},(_,i)=>"0123456789abcdef"[(seed*7+i*13+(seed>>3))%16]).join("");
const record=(i:number)=>"np/"+hex(i,24)+".json",pack=(i:number,w:string)=>record(i)+".records/m"+i+"-"+i+"-"+hex(i+3,16)+"."+w;
const leaf=(index:number,kind:number)=>kind===0?"np/"+hex(index,24)+".json":kind===1?"np/ob/"+hex(index+101,64)+".br":kind===2?"mr/k"+index+"-"+index+"-"+hex(index+7,16)+".idx":"mr/k"+index+"-"+index+"-"+hex(index+7,16)+".pack";
const now=Math.floor(Date.now()/1000)*1000;let seed=29;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
const control={cancelled:()=>false,progress:()=>{}};const cases:any[]=[],actual:any[]=[];
const walk=(root:string,at="")=>{const out:string[]=[];for(const e of readdirSync(join(root,at),{withFileTypes:true})){const n=at?at+"/"+e.name:e.name;if(e.isDirectory())out.push(...walk(root,n));else out.push(n);}return out;};
for(let index=0;index<120;index++){
 const group="selections/"+hex(index+500,64),names=new Set<string>(),rows:any[]=[];
 for(let u=0;u<1+random(7);u++){const kind=random(4),base=index*32+u,age=random(3)?(1+random(40))*HOUR:random(4)*60000,members=kind===0?[record(base),pack(base,"pack"),pack(base,"idx")]:kind===1?[leaf(base,1)]:kind===2?[leaf(base,2),leaf(base,3)]:[record(base)];
  for(const name of members){if(names.has(name))continue;names.add(name);rows.push({name,bytes:1+random(900),ageMs:age+random(3)*1000});}}
 const targetBytes=random(4000),otherBytes=random(1500),root=mkdtempSync(join(tmpdir(),"lru60-")),base=join(root,"inference",...group.split("/"));
 for(const r of rows){const p=join(base,r.name);mkdirSync(dirname(p),{recursive:true});writeFileSync(p,Buffer.alloc(r.bytes,1));const at=(now-r.ageMs)/1000;utimesSync(p,at,at);}
 const report=await mod.retireBuildGraphOptionalEntriesLruV1(root,group,control,{targetBytes,otherBytes,now});
 const left=new Set(walk(base)),unit=(n:string)=>mod.optionalEntryOfV1(n),lost=rows.filter(r=>unit(r.name)!==undefined&&!left.has(r.name)),keys=(l:any[])=>[...new Set(l.map(r=>unit(r.name)!))].sort();
 cases.push({targetBytes,otherBytes,now,protectedMs:600000,leaves:rows.map(r=>({name:r.name,bytes:r.bytes,newest:now-r.ageMs}))});
 actual.push({shed:keys(lost.filter(r=>mod.optionalAcceleratorV1(r.name))),retired:keys(lost.filter(r=>!mod.optionalAcceleratorV1(r.name))),remainingBytes:report.remainingBytes});rmSync(root,{recursive:true,force:true});
}
const f=join(tmpdir(),"lru60-cases.json");writeFileSync(f,JSON.stringify({cases}));
const run=spawnSync("python3",["-I",WORK+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/♻️lru/🐍️.py",f,"entries"],{encoding:"utf8",maxBuffer:1<<26});
if(run.status!==0)throw Error(run.stderr);
const want=JSON.parse(run.stdout);let bad=0;for(let i=0;i<120;i++){if(JSON.stringify(want[i])!==JSON.stringify(actual[i])){bad++;if(bad<4)console.log("DIFF",i,JSON.stringify(cases[i]).slice(0,300),JSON.stringify(want[i]),JSON.stringify(actual[i]));}}
console.log("cases 120, differing",bad,"shed non-empty",actual.filter(a=>a.shed.length).length,"retired non-empty",actual.filter(a=>a.retired.length).length);
