import{existsSync,readFileSync}from"node:fs";import{join}from"node:path";import{deserialize}from"node:v8";import{brotliDecompressSync}from"node:zlib";
export const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
export const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
export const load=async(waveDir:string)=>{
 const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
 const pn:any=await import(PN+"🟨️.mjs"),SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),AS:any=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"),OB:any=await import(B+"👁️observation/🟨️.mjs"),RL:any=await import(B+"🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs");
 const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
 const units=JSON.parse(readFileSync(waveDir+"/units.json","utf8")),session=deserialize(readFileSync(waveDir+"/session.bin"));
 const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
 const recordPath=(root:string)=>join(dataDir,"np",ownerRecordKeyV1(root)+".json");
 const recordOf=(root:string)=>{try{const text=brotliDecompressSync(readFileSync(recordPath(root))).toString("utf8");return JSON.parse(text.slice(text.indexOf("\n")+1));}catch{return undefined;}};
 return{pn,SI,AS,OB,RL,units,env,recordPath,recordOf,existsSync};
};
