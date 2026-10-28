// for sampled owners: resolve the closure of the record (a) with the files edited in a window reverted to HEAD, (b) on the live tree; compare roster values and the resolved (compacted) project
import{setOverlay,headContent,changedSinceHead}from"./overlay.mjs";
import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const asm=await import(L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs");const pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");import{mkdtempSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";
const staging=realpathSync(mkdtempSync(tmpdir()+"/cutoff-sim-"));
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
const D=R+".cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const every=Number(process.argv[2]??60),windowsH=(process.argv[3]??"0.5,2,8").split(",").map(Number);
const records=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n)).filter((_,i)=>i%every===0).flatMap(n=>{try{const t=brotliDecompressSync(readFileSync(D+n)).toString("utf8");const v=JSON.parse(t.slice(t.indexOf("\n")+1));return v.closureRecipes?[v]:[];}catch{return[];}});
console.log("sampled owners",records.length);globalThis.quiet=true;
const changed=changedSinceHead().filter(f=>/\.(ts|tsx|mjs|cjs|js|json)$/.test(f));
const mtimes=new Map(changed.map(f=>{try{return[f,statSync(R+f).mtimeMs];}catch{return[f,0];}}));
const digest=v=>createHash("sha256").update(JSON.stringify(v)).digest("hex");
async function resolveAll(){
 si.resetClosureEntriesV1();obs.releaseInferenceMemoryV1();
 const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:true}));
 const out=[];const t0=Date.now();
 for(const value of records){
  const rows=Object.keys(value.closureRecipes).map(key=>({kind:"closure",path:JSON.stringify([WS,key,[]]),fact:"x"}));
  try{await pin.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{await obs.settleClosureRowsV1(rows,value);
   const api=si.closureApiV1(env.scripts,WS);const r=await api.resolve(Object.entries(value.closureRecipes));
   const {project}=await asm.resolveClosureRefsV1(value,WS,env.scripts,"hash");
   out.push({root:value.root,values:Object.fromEntries([...r.values].map(([k,v])=>[k,digest(v)])),project:digest(project)});})));}catch(e){if(out.filter(x=>x.error).length<2)console.log("ERR",String(e.stack).slice(0,600));out.push({root:value.root,error:String(e.message).slice(0,80)});}
 }
 console.log("  resolved",out.length,"in",Date.now()-t0,"ms");return out;
}
const neu=await resolveAll();
for(const h of windowsH){
 const cutoff=Date.now()-h*3600e3,W=changed.filter(f=>(mtimes.get(f)??0)>=cutoff);
 const m=new Map();for(const f of W){const b=headContent(f);if(b)m.set(R+f,b);}setOverlay(m);
 const old=await resolveAll();setOverlay(new Map());
 let anyValue=0,anyProject=0,errors=0;
 old.forEach((o,i)=>{const n=neu[i];if(o.error||n.error){errors++;return;}
  if(Object.keys(o.values).some(k=>o.values[k]!==n.values[k]))anyValue++;if(o.project!==n.project)anyProject++;});
 console.log(JSON.stringify({windowHours:h,editedFiles:m.size,owners:old.length,errors,ownersWithChangedRosterValue:anyValue,ownersWithChangedResolvedProject:anyProject}));
}
