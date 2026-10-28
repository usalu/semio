import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";process.chdir(R);
const WS=R.slice(0,-1);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");
const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
const D=R+".cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n));
const every=Number(process.argv[2]??40),limit=Number(process.argv[3]??60);
const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:WS,registry,projectsByRoot:[],componentRoots:[],commandInputs:null,lateClosures:true}));
const api=si.closureApiV1(env.scripts,WS);
let done=0,entries=0,totalMs=0;const stats=[];const rosters=new Map();let firstMs=0;
for(const name of names.filter((_,i)=>i%every===0)){
 if(done>=limit)break;let value;try{const text=brotliDecompressSync(readFileSync(D+name)).toString("utf8");value=JSON.parse(text.slice(text.indexOf("\n")+1));}catch{continue;}
 if(!value.closureRecipes)continue;done++;
 const t=performance.now();
 const rows=Object.keys(value.closureRecipes).map(key=>({kind:"closure",path:JSON.stringify([WS,key,[]]),fact:"x"}));
 await obs.observeInferenceSnapshotV1(async()=>{await obs.settleClosureRowsV1(rows,value);const r=await api.resolve(Object.entries(value.closureRecipes));
  for(const[key,v]of r.values){const d=createHash("sha256").update(JSON.stringify(v)).digest("hex");rosters.set(d,(rosters.get(d)??0)+1);entries++;}
  stats.push({root:value.root.slice(-50),recipes:Object.keys(value.closureRecipes).length,rows:r.rows.length,ms:Math.round(performance.now()-t)});});
 totalMs+=performance.now()-t;
}
console.log(JSON.stringify({projects:done,entries,distinctRosters:rosters.size,totalMs:Math.round(totalMs)}));
for(const s of stats.slice(0,6))console.log(JSON.stringify(s));
stats.sort((a,b)=>b.ms-a.ms);console.log("slowest",JSON.stringify(stats.slice(0,3)));
const rs=[...rosters.values()].sort((a,b)=>b-a);console.log("roster use counts top",rs.slice(0,8).join(","));
