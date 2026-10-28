import{readFileSync,readdirSync,mkdtempSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";import{brotliDecompressSync}from"node:zlib";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const asm=await import(L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs");const pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
import{setOverlay,headContent}from"./overlay.mjs";
if(process.env.WHERE_REVERT){const f=process.env.WHERE_REVERT;setOverlay(new Map([[R+f,headContent(f)]]));}
const staging=realpathSync(mkdtempSync(tmpdir()+"/where-"));
const D=R+".cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n));
const needle=process.argv[2];
const t=brotliDecompressSync(readFileSync(D+names[Number(process.argv[3]??0)])).toString("utf8");const value=JSON.parse(t.slice(t.indexOf("\n")+1));
const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:true}));
const rows=Object.keys(value.closureRecipes).map(key=>({kind:"closure",path:JSON.stringify([WS,key,[]]),fact:"x"}));
await pin.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published:()=>{}},()=>obs.observeInferenceSnapshotV1(async()=>{await obs.settleClosureRowsV1(rows,value);
 const {rows:own,derived}=await asm.resolveClosureRefsV1(value,WS,env.scripts,"hash");
 console.log(value.root.slice(-60),"own rows",own.length,"derived",derived.length,derived.map(d=>d.support.length));
 const api=si.closureApiV1(env.scripts,WS);for(const[key,recipe]of Object.entries(value.closureRecipes)){const r=await api.resolve([[key,recipe]]);const n=r.rows.filter(x=>x.path.includes(needle)).length;if(n)console.log("RECIPE",recipe.fn,"own rows",r.rows.length,"needle rows",n,"derived",r.derived.length,JSON.stringify(recipe).slice(0,200));}
 {const sup=new Set(derived.flatMap(d=>d.support));const inSup=own.filter(r=>sup.has(r)).length;const keys=new Set(derived.flatMap(d=>d.support).map(r=>r.kind+"\0"+r.path));const byKey=own.filter(r=>keys.has(r.kind+"\0"+r.path)).length;console.log("OVERLAP own rows also in support (identity)",inSup,"(kind+path)",byKey,"of",own.length,"support total",sup.size);const dirs={};for(const r of own){const k=r.path.replace(WS+"/","").split("/").slice(0,3).join("/").slice(0,60);dirs[k]=(dirs[k]??0)+1;}console.log(JSON.stringify(Object.entries(dirs).sort((a,b)=>b[1]-a[1]).slice(0,6)));}
 const hit=own.filter(r=>r.path.includes(needle));console.log("own rows touching needle",hit.map(r=>r.kind).join(","));
 for(const d of derived)console.log("support has needle",d.support.filter(r=>r.path.includes(needle)).length);
 const kinds={};for(const r of own)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log(JSON.stringify(kinds));
 console.log(own.filter(r=>r.kind==="analysis").slice(0,5).map(r=>r.path.slice(-80)));
})));
