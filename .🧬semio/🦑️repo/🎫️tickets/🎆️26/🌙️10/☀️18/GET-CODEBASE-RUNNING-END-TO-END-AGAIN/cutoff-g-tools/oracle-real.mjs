// Oracle on recorded real edits: the tree is frozen (every file of the fact index is served from a snapshot), an edit set is the revert of some files to HEAD (OLD state), the index of a sample of owners is built in the OLD state in two shapes (legacy raw rows, derived facts), the NEW state is the frozen tree; both plans run against NEW and are compared with full recomputation (the resolved project of every owner, OLD against NEW).
import{setOverlay,headContent,changedSinceHead}from"./overlay.mjs";
import{readFileSync,readdirSync,mkdtempSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs");const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const asm=await import(L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");const C=await import(L+"🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs");const pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
const staging=realpathSync(mkdtempSync(tmpdir()+"/cutoff-oracle-"));
const [,, factsPath, every="160", setsPath]=process.argv;
const idx=G.unpackFactIndexV1(readFileSync(factsPath),1<<29);
const freeze=new Map();for(const r of idx.rows)if((r.kind==="file"||r.kind==="analysis"||r.kind==="typescriptText")&&!freeze.has(r.path)&&!/\/(?:👁️observation|📥️inputs|🧭️invocation)\//u.test(r.path)){try{freeze.set(r.path,readFileSync(r.path));}catch{}}
console.log("frozen files",freeze.size);
const D=R+".cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const records=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n)).filter((_,i)=>i%Number(every)===0||process.env.ORACLE_ALL).flatMap(n=>{try{const t=brotliDecompressSync(readFileSync(D+n)).toString("utf8");const v=JSON.parse(t.slice(t.indexOf("\n")+1));return v.closureRecipes&&(!process.env.ORACLE_ROOTS||new RegExp(process.env.ORACLE_ROOTS,"u").test(v.root))?[v]:[];}catch{return[];}});
if(process.env.ORACLE_LIMIT)records.splice(Number(process.env.ORACLE_LIMIT));console.log("sample",records.length,records.slice(0,3).map(r=>r.root.slice(-50)));
const digest=v=>createHash("sha256").update(JSON.stringify(v)).digest("hex");
async function resolveAll(){
 si.resetClosureEntriesV1();obs.releaseInferenceMemoryV1();
 const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:true}));
 const out=new Map();
 for(const value of records){
  const rows=Object.keys(value.closureRecipes).map(key=>({kind:"closure",path:JSON.stringify([WS,key,[]]),fact:"x"}));
  try{await pin.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{await obs.settleClosureRowsV1(rows,value);
   const {project,rows:own,derived}=await asm.resolveClosureRefsV1(value,WS,env.scripts,"hash");
   out.set(value.root,{project:digest(project),own,derived});})));}catch(e){out.set(value.root,{error:String(e.message).slice(0,80)});}
 }
 return out;
}
const hold=async rows=>{const plain=rows.filter(r=>r.kind!=="closure"&&r.kind!=="field"&&r.kind!=="digest"&&r.kind!=="derived");return plain.length===0||obs.observationsHold(plain,64,new Set(),true);};
async function plan(index,{cutoff}){
 const sel=G.selectFactClassesV1(index);const ids=cutoff?C.verifiableRowIdsV1(index,sel.ids):sel.ids;
 const{stale}=await G.staleFactRowsV1(index,ids,hold,{});
 if(!cutoff)return{dirty:G.staleConsumersV1(index,sel.classes,stale),stale:stale.size,questions:0};
 const{direct,suspects}=C.splitStaleConsumersV1(index,sel.classes,stale);
 if(process.env.ORACLE_DEBUG){for(const id of stale){const r=index.rows[id];const own=[];for(const at of sel.classes){const c=index.classes[at];if(c.r.includes(id))for(const o of c.o)if(!C.isDerivedOwnerV1(index.owners[o]))own.push(index.owners[o]);}console.log("DEBUG stale",r.kind,r.path.replace(R,"").slice(-90),"direct holders",own.length);}}
 si.resetClosureEntriesV1();
 const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:false}));
 const settled=await C.settleSuspectsV1({index,suspects,derive:(row)=>obs.observeInferenceSnapshotV1(()=>si.deriveClosureFactV1(row,env.scripts))});
 const dirty=new Set([...direct].filter(n=>!C.isDerivedOwnerV1(n)));for(const set of C.holdersOfDerivedV1(index,settled.changed.keys()).values())for(const n of set)dirty.add(n);
 return{dirty,stale:stale.size,questions:suspects.size,held:settled.held.size,changed:settled.changed.size,failed:settled.failed};
}
const sets=JSON.parse(readFileSync(setsPath,"utf8"));
for(const set of sets){
 const old=new Map(freeze),neu=new Map(freeze);
 for(const f of set.files??[]){const b=headContent(f);if(b)old.set(R+f,b);}
 for(const[f,text]of Object.entries(set.append??{}))neu.set(R+f,Buffer.concat([freeze.get(R+f)??readFileSync(R+f),Buffer.from(text)]));
 for(const[f,text]of Object.entries(set.replace??{}))neu.set(R+f,Buffer.from(text));
 setOverlay(old);const before=await resolveAll();
 setOverlay(neu);const after=await resolveAll();
 const legacy=new Map(),cut=new Map();
 for(const[root,row]of before){if(row.error)continue;const support=row.derived.flatMap(d=>d.support);legacy.set(root,[...new Set([...row.own,...row.derived.map(d=>d.row),...support])]);cut.set(root,[...row.own,...row.derived.map(d=>d.row)]);for(const d of row.derived)cut.set(C.derivedOwnerNameV1(d.row),[d.row,...d.support]);}
 const legacyIndex=G.createFactIndexV1(legacy),cutIndex=G.createFactIndexV1(cut);
 const L1=await plan(legacyIndex,{cutoff:false}),C1=await plan(cutIndex,{cutoff:true});
 const changed=new Set([...before].filter(([root,b])=>after.get(root)&&!b.error&&!after.get(root).error&&b.project!==after.get(root).project).map(([root])=>root));
 const unsound=[...changed].filter(root=>!C1.dirty.has(root)),waste=[...C1.dirty].filter(root=>!changed.has(root));
 console.log(JSON.stringify({label:set.label,edited:(set.files??[]).length+Object.keys(set.append??{}).length+Object.keys(set.replace??{}).length,owners:before.size,legacyDirty:L1.dirty.size,cutoffDirty:C1.dirty.size,trulyChanged:changed.size,unsound:unsound.length,wasteful:waste.length,questions:C1.questions,held:C1.held,changedQuestions:C1.changed,failed:C1.failed,staleRows:L1.stale}));
}
setOverlay(new Map());
