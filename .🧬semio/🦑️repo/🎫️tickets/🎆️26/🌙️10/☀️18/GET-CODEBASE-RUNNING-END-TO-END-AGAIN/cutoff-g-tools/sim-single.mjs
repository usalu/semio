// single-file edits: for every file edited since HEAD, revert only that file (overlay) and measure, on a sample of owners, how many resolved rosters / resolved projects change, against the number of owners the current fact index would stale
import{setOverlay,headContent,changedSinceHead}from"./overlay.mjs";
import{readFileSync,readdirSync,mkdtempSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs");const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const asm=await import(L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs");const pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
const staging=realpathSync(mkdtempSync(tmpdir()+"/cutoff-sim-"));
const [,, factsPath, every="160", maxFiles="40"]=process.argv;
const idx=G.unpackFactIndexV1(readFileSync(factsPath),1<<29);
const holders=new Map();for(const c of idx.classes)for(const r of c.r){const row=idx.rows[r];if(row.kind==="analysis")holders.set(row.path,(holders.get(row.path)??0)+c.o.length);}
const D=R+".cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const records=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n)).filter((_,i)=>i%Number(every)===0).flatMap(n=>{try{const t=brotliDecompressSync(readFileSync(D+n)).toString("utf8");const v=JSON.parse(t.slice(t.indexOf("\n")+1));return v.closureRecipes?[v]:[];}catch{return[];}});
const digest=v=>createHash("sha256").update(JSON.stringify(v)).digest("hex");
async function resolveAll(){
 si.resetClosureEntriesV1();obs.releaseInferenceMemoryV1();
 const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:true}));
 const out=[];
 for(const value of records){
  const rows=Object.keys(value.closureRecipes).map(key=>({kind:"closure",path:JSON.stringify([WS,key,[]]),fact:"x"}));
  try{await pin.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{await obs.settleClosureRowsV1(rows,value);
   const api=si.closureApiV1(env.scripts,WS);const r=await api.resolve(Object.entries(value.closureRecipes));
   const {project}=await asm.resolveClosureRefsV1(value,WS,env.scripts,"hash");
   out.push({values:Object.fromEntries([...r.values].map(([k,v])=>[k,digest(v)])),project:digest(project)});})));}catch(e){out.push({error:String(e.message).slice(0,80)});}
 }
 return out;
}
const live=await resolveAll();
const changed=changedSinceHead().filter(f=>/\.(ts|tsx|mjs)$/.test(f)&&holders.has(R+f)).slice(0,Number(maxFiles));
console.log("sample owners",records.length,"candidate files",changed.length);
const lines=[];
for(const f of changed){
 setOverlay(new Map([[R+f,headContent(f)]]));
 const rows=idx.rows.filter(r=>r.kind==="analysis"&&r.path===R+f);
 const stale=!(await obs.observationsHold(rows,8,new Set(),true));
 let rosterOwners=0,projectOwners=0,errors=0;
 if(stale){const old=await resolveAll();old.forEach((o,i)=>{const n=live[i];if(o.error||n.error){errors++;return;}if(Object.keys(o.values).some(k=>o.values[k]!==n.values[k]))rosterOwners++;if(o.project!==n.project)projectOwners++;});}
 const line={file:f.slice(-90),holders:holders.get(R+f),analysisChanged:stale,sample:records.length,rosterChanged:stale?rosterOwners:0,projectChanged:stale?projectOwners:0,errors};
 lines.push(line);console.log(JSON.stringify(line));
}
setOverlay(new Map());
