import{readdirSync,readFileSync,statSync,existsSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{deserialize}from"node:v8";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const dir=process.argv[2],N=Number(process.argv[3]||300);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const OB:any=await import(B+"👁️observation/🟨️.mjs");
const session=deserialize(readFileSync(dir+"/session.bin"));
const sel=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",ce=sel+"/ce";
const SA:any=await import(B+"🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs");const cur=SA.prepareSourceAnalysesV1();const curIds=new Map<string,string>();for(const p of Object.values(cur) as any[])curIds.set(p.id,p.revision);for(const id of SI.scriptProviderIdsV1())curIds.set(id,"?");
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:ce});
const{createClosureStoreV1}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs");
const store=createClosureStoreV1({directory:ce,readOnly:true});
const fresh=OB.observationRowFreshV1;
const names=readdirSync(ce+"/e").filter(n=>n.endsWith(".json.br"));
const step2=Math.max(1,Math.floor(names.length/(N*4)));
const reasonOf=(row:any):string=>{
 const k=row.kind,f=row.fact;
 if(k==="file"){if(f?.error!==undefined)return"file:error-fact";let st:any;try{st=statSync(row.path);}catch{return"file:missing";}if(st.size!==f.size)return"file:content-changed(size)";const stampOk=st.mtimeMs===f.mtimeMs&&st.ctimeMs===f.ctimeMs&&st.ino===f.ino;return stampOk?"file:?":"file:content-changed(sha)";}
 if(k==="analysis"){const p=f?.provider;if(typeof p==="string"&&p.includes("⟦"))return"analysis:unlocalised-token";const prov=(OB.sourceAnalysisProvidersV1??OB.registeredSourceAnalysesV1)?.();if(!curIds.has(p))return"analysis:provider-id-unregistered";if(curIds.get(p)!=="?"&&curIds.get(p)!==f.revision)return"analysis:revision-differs";let st:any;try{st=statSync(row.path);}catch{return"analysis:source-missing";}if(st.size!==f.source?.size)return"analysis:source-changed";return"analysis:nested-or-contract";}
 return k+":fact";
};
const portable=(row:any)=>{const t=JSON.stringify(row);const out:string[]=[];if(t.includes(R))out.push("absolute-root");if(/"(mtimeMs|ctimeMs|ino)":[1-9]/.test(t))out.push("stamps");if(t.includes("⟦"))out.push("token");return out;};
const idRoots:Record<string,number>={};const why:Record<string,number>={},portCount:Record<string,number>={},cost:Record<string,number>={};let n=0,stale=0,lost=0,fine=0;const examples:any[]=[];
const t0=performance.now();
for(let i=0;i<names.length&&n<N;i+=step2){
 let id:string;try{id=JSON.parse(brotliDecompressSync(readFileSync(ce+"/e/"+names[i])).toString("utf8")).id;}catch{continue;}
 const idRoot=id.split("\0")[0];idRoots[idRoot]=(idRoots[idRoot]??0)+1;if(idRoot!==R){continue;}const e=await store.load(id);if(!e){lost++;continue;}n++;
 let bad:any;const seen=new Set<string>();
 const check=(row:any)=>{const key=JSON.stringify(row);if(seen.has(key))return true;seen.add(key);for(const p of portable(row))portCount[p]=(portCount[p]??0)+1;return fresh(row);};
 for(const row of e.rows)if(!check(row)&&!bad)bad=row;
 let layerLost=false;
 for(const d of e.digests){try{for(const b of store.layerBatchesSync(d,4096))for(const row of b)if(!check(row)&&!bad)bad=row;}catch{layerLost=true;}}
 const weight=Math.max(1,e.digests.length);
 if(bad){stale++;const r=reasonOf(bad);why[r]=(why[r]??0)+1;cost[r]=(cost[r]??0)+weight;if(examples.length<25)examples.push({r,kind:bad.kind,path:String(bad.path).slice(0,160),fact:JSON.stringify(bad.fact).slice(0,220)});}
 else if(layerLost){stale++;why["layer-lost"]=(why["layer-lost"]??0)+1;}
 else fine++;
}
console.log(JSON.stringify({sampled:n,idRoots,lostOrForeign:lost,fine,stale,why,costWeightLayers:cost,rowsWithPortabilityDefect:portCount,seconds:Math.round((performance.now()-t0)/1000),examples},null,1));
process.exit(0);
