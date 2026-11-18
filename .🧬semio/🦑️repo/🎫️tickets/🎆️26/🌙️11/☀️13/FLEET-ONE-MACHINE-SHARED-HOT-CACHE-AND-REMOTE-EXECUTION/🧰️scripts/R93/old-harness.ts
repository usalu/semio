import{mkdirSync,rmSync}from"node:fs";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",GS=B+"🪪️graph-state/";
const scratch=process.env.R93_SCRATCH!,mode=process.env.R93_MODE||"old",digests=process.argv.slice(2);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const EX:any=await import(GS+"🛰️exchange/🟦️.ts");const NX:any=await import(GS+"🧶️nx/🟦️.ts");const PO:any=await import(GS+"📦️project-records/🧳️portable/🟦️.ts");
const RF:any=await import(B+"🌐️remote-cache/🛰️fleet/🟦️.ts");
const local=scratch+"/local-"+process.pid;rmSync(local,{recursive:true,force:true});mkdirSync(local+"/ce",{recursive:true});
SI.bindClosureStoreV1(R,local+"/ce",true);
const mb=(n:number)=>Math.round(n/1048576);const t0=performance.now();
const mark=(label:string)=>{const m=process.memoryUsage();console.log(JSON.stringify({label,sec:Math.round((performance.now()-t0)/100)/10,rss:mb(m.rss),heapUsed:mb(m.heapUsed),external:mb(m.external)}));};
const settings=NX.graphFleetSettingsV1(R,process.env);const client=EX.graphFleetClientV1(settings,{fetch:RF.remoteCacheFetchV1(process.env,{root:R})});
const host=PO.hostOfRootV1(R);
const SA=B+"🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs";
const providers=async()=>{const loaded:any=await import(SA);const own=SI.scriptProviderIdsV1();return[...Object.values(loaded.prepareSourceAnalysesV1()).map((p:any)=>p.id),...own];};
let closures=0;
for(const digest of digests){
 const state={digest,frame:{kind:"project-inference"}};
 mark("start "+digest.slice(0,8));
 if(mode==="meta"||mode==="old"){
 const raw=await client.getResult(EX.graphEntryKeyV1(state));console.log("raw",typeof raw==="string"?raw:"bytes "+raw.length,"usable",client.usable());
 const held=new Map<string,string>();
 const heldOf=async(entry:any)=>{const named=/^closure-([0-9a-f]{64})\.br$/u.exec(entry.path);if(named===null)return false;const l=await SI.closureTravelV1.held(R,entry.hash);if(l===undefined)return false;held.set(named[1]!,l);return true;};
 const found=await EX.fetchGraphEntryV1(client,state,heldOf);
 if(found===undefined||found==="fault"){console.log("ENTRY",digest.slice(0,8),String(found));continue;}
 const docs=found.documents as any[];
 console.log(JSON.stringify({digest:digest.slice(0,8),docs:docs.length,closureDocs:docs.filter(d=>d.name.startsWith("closure-")).length,compressedMB:Math.round(found.note.documents.reduce((s:number,d:any)=>s+d.size,0)/1048576),decompressedMB:mb(docs.reduce((s:number,d:any)=>s+d.bytes.length,0)),largestMB:mb(Math.max(...docs.map(d=>d.bytes.length)))}));
 mark("fetched+decompressed");
 if(mode==="meta")continue;
 const sources=new Map<string,string>(found.note.documents.flatMap((row:any)=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[]));
 const record=docs.find(r=>r.name==="record");const neutralRec=PO.unpackRecordV1(JSON.parse(Buffer.from(record.bytes).toString("utf8")));
 const context={providers:await providers(),generation:PO.dependencyGenerationV1(R)};const taken=PO.localRecordV1(neutralRec,host,context);
 mark("record localised");
 // verbatim old takeClosures
 const documents=docs;const keys=taken.keys;
 const neutral=documents.filter((row:any)=>row.name.startsWith("closure-")).flatMap((row:any)=>{try{return[{recipeKey:row.name.slice(8),bundle:PO.unpackBundleV1(JSON.parse(Buffer.from(row.bytes).toString("utf8")))}];}catch{return[];}});
 mark("old: all bundles parsed+unpacked "+neutral.length);
 const all=new Map([...keys,...held,...PO.bundleKeysV1(neutral,host,"local")]),bundles=neutral.flatMap((entry:any)=>{const moved=PO.localBundleV1(entry,host,all,context),source=sources.get("closure-"+entry.recipeKey);return moved===undefined?[]:[source===undefined?moved:{...moved,source}];});
 mark("old: all bundles localised "+bundles.length);
 if(bundles.length>0)closures+=await SI.closureTravelV1.import(R,bundles);
 mark("old: imported total "+closures);
 }
}
mark("end");process.exit(0);
