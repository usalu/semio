import{readFileSync,mkdirSync,rmSync}from"node:fs";import{deserialize}from"node:v8";import{join}from"node:path";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/",GS=B+"🪪️graph-state/";
const dir=process.env.R91_WAVE!,scratch=process.env.R93_SCRATCH!,mode=process.env.R93_MODE||"seed",names=process.argv.slice(2);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const OB:any=await import(B+"👁️observation/🟨️.mjs");
const PR:any=await import(GS+"📦️project-records/🟦️.ts");const EX:any=await import(GS+"🛰️exchange/🟦️.ts");const NX:any=await import(GS+"🧶️nx/🟦️.ts");const GP:any=await import(GS+"🟦️.ts");
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const store=scratch+"/local-"+process.pid+"/ce";rmSync(scratch+"/local-"+process.pid,{recursive:true,force:true});mkdirSync(store,{recursive:true});
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
SI.bindClosureStoreV1(R,store,true);
const mb=(n:number)=>Math.round(n/1048576);const t0=performance.now();
const mark=(label:string)=>{const m=process.memoryUsage();console.log(JSON.stringify({label,sec:Math.round((performance.now()-t0)/100)/10,rss:mb(m.rss),heapUsed:mb(m.heapUsed),external:mb(m.external)}));};
const paths=Object.fromEntries((process.env.R93_PATHS||"").split(";").map(x=>x.split("=")));
const port=PR.createProjectFleetPortV1({root:R,environment:process.env});
if(!port){console.log("NO FLEET PORT");process.exit(2);}
let imported=0;
for(const name of names){
 const p=paths[name];const j=JSON.parse(readFileSync(R+"/"+p,"utf8"));const root=p.replace(/\/[^/]*$/u,"");
 const job=pn.ownerJobV1(env,head.globals??head,{name,json:{...j,root}});
 const keyText=pn.ownerRecordKeyTextV1(pn.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs));
 mark("start "+name);
 if(mode==="meta"){
  const state=PR.projectRecordStateV1({root:R,fleet:job.root,keyText,platform:GP.graphPlatformV1()});
  const settings=NX.graphFleetSettingsV1(R,process.env);const client=EX.graphFleetClientV1(settings,{fetch:(await import(B+"🌐️remote-cache/🛰️fleet/🟦️.ts")).remoteCacheFetchV1(process.env,{root:R})});
  console.log("settings",JSON.stringify({...settings,token:settings?.token?"<set>":null}),"usable",client.usable());const raw=await client.getResult(EX.graphEntryKeyV1(state));console.log("raw",typeof raw==="string"?raw:Object.keys(raw||{}));const found=await EX.fetchGraphEntryV1(client,state);
  if(found===undefined||found==="fault"){console.log("ENTRY",name,String(found));continue;}
  const docs=found.documents;const comp=found.note.documents;
  console.log(JSON.stringify({name,docs:docs.length,compressedMB:mb(comp.reduce((s:number,d:any)=>s+d.size,0)),decompressedMB:mb(docs.reduce((s:number,d:any)=>s+d.bytes.length,0)),closureDocs:docs.filter((d:any)=>d.name.startsWith("closure-")).length,largestKB:Math.round(Math.max(...docs.map((d:any)=>d.bytes.length))/1024)}));
  mark("meta fetched");continue;
 }
 const info={cachePath:scratch+"/np/"+name+".json",fleet:job.root,keyText,exportRecord:()=>undefined,import:async()=>{imported++;return true;},closures:SI.closureTravelV1};
 const t=performance.now();
 const out=await PR.runTakeLookV1(async(look:any)=>{const g=await port.gate(info);return{seeded:g.seeded,outcome:look.outcome};});
 mark("SEED "+name+" "+JSON.stringify(out)+" "+Math.round(performance.now()-t)+"ms stats "+JSON.stringify(port.stats()));
}
mark("end");process.exit(0);
