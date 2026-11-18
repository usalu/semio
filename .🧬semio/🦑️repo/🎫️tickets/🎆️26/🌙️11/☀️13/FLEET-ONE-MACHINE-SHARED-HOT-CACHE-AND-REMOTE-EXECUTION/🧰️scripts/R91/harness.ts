import{existsSync,readFileSync,realpathSync,mkdtempSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";import{join}from"node:path";import{deserialize}from"node:v8";import{brotliDecompressSync}from"node:zlib";import{heapStats}from"bun:jsc";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const dir=process.env.R91_WAVE!,names=process.argv.slice(2);
console.log("start");const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;console.log("bootstrapped");
const pn:any=await import(PN+"🟨️.mjs");const{ownerJobV1}=pn;
console.log("pn loaded");const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const PA:any=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const OB:any=await import(B+"👁️observation/🟨️.mjs");
console.log("modules loaded");const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
const{withBuildGraphStagingV1}=await import(B+"📥️inputs/💾️storage/🟨️.mjs") as any;import*as JSC from"bun:jsc";const stagingRoot=process.env.R91_STAGING!;
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
const recordOfKey=(key:string)=>{try{const text=brotliDecompressSync(readFileSync(join(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(text.slice(text.indexOf("\n")+1));}catch{return undefined;}};
const mb=(n:number)=>Math.round(n/1048576);
const t0=performance.now();
const mark=(label:string)=>{const m=process.memoryUsage();console.log(JSON.stringify({label,sec:Math.round((performance.now()-t0)/100)/10,rss:mb(m.rss),heapUsed:mb(m.heapUsed),external:mb(m.external),cpuS:Math.round((process.cpuUsage().user+process.cpuUsage().system)/1e5)/10}));};
const gc=process.env.R91_GC==="1";const{createRowLogV1}=await import(B+"🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs") as any;const rowLog=process.env.R91_ROWLOG==="1"?createRowLogV1({path:stagingRoot+"/x-"+process.pid+".rows"}):undefined;
await withBuildGraphStagingV1({root:stagingRoot,published:()=>{}},()=>SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{
 let nx=0;const conc=Number(process.env.R91_CONC||1);await Promise.all(Array.from({length:conc},async()=>{while(nx<names.length){const name=names[nx++];
  let u=units.find((x:any)=>x.name===name);if(!u&&process.env.R91_PATHS){const hit=process.env.R91_PATHS.split(";").map(x=>x.split("=")).find(x=>x[0]===name);if(hit){const p=hit[1];const j=JSON.parse(readFileSync(R+"/"+p,"utf8"));const root=p.replace(/\/[^/]*$/u,"");u={name,kind:"project",json:{...j,root}};}}if(!u){console.log("no unit",name);continue;}
  const value=recordOfKey(ownerRecordKeyV1(u.json.root));if(!value){console.log("no record",name);continue;}
  const recipes=Object.keys(value.closureRecipes??{});
  console.log(JSON.stringify({owner:name,recipes:recipes.length,targets:Object.keys(value.targets??{}).length,namedInputs:Object.keys(value.namedInputs??{}).length,symbolic:(value.symbolicGroups??[]).length,recordKB:Math.round(JSON.stringify(value).length/1024)}));
  mark("before "+name);
  const paths=recipes.map(k=>({kind:"closure",path:JSON.stringify([R,k,["digest"]])}));
  const a=performance.now();if(process.env.R91_TAKE!=="1")await OB.settleClosureRowsV1(paths,value);const b=performance.now();mark("settled "+name+" "+Math.round(b-a)+"ms");
  const c0=process.cpuUsage();
  let res:any;
  if(process.env.R91_TAKE==="1"){
   const t=performance.now();let code="ok";
   try{res=await SI.runClosureTakeV1(()=>PA.resolveClosureRefsV1(value,R,env.scripts,"hash",rowLog));}catch(e:any){code=String(e?.code??e?.message).slice(0,60);}
   mark("TAKE "+name+" -> "+code+" "+Math.round(performance.now()-t)+"ms");
  }else{
  {const api=SI.closureApiV1(env.scripts,R);const t=performance.now();const {values}=await api.resolve(Object.entries(value.closureRecipes),rowLog);let items=0;for(const v of values.values())items+=v.length;mark("A resolve (closure entries) "+Math.round(performance.now()-t)+"ms lists "+values.size+" items "+items);}
  if(process.env.R91_PROFILE==="1")(JSC as any).startSamplingProfiler();
  const tb=performance.now();
  res=await PA.resolveClosureRefsV1(value,R,env.scripts,"hash",rowLog);
  mark("B resolveClosureRefsV1 "+Math.round(performance.now()-tb)+"ms");
  if(process.env.R91_PROFILE==="1"){const tr=(JSC as any).samplingProfilerStackTraces();const top=new Map<string,number>();for(const t of tr.traces){const f=String(t.split("\n")[0]).trim().slice(0,160);top.set(f,(top.get(f)||0)+1);}console.log("PROFILE",JSON.stringify([...top].sort((a,b)=>b[1]-a[1]).slice(0,14)));}
  }
  const c1=process.cpuUsage(c0);mark("resolved "+name+" cpu "+Math.round((c1.user+c1.system)/1e5)/10+"s rows "+(res?.rows?.length??0)+" json "+(res?JSON.stringify(res.project).length:0));
  SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});if(gc)Bun.gc(true);mark("released "+name);
 }}));
})));
process.exit(0);
