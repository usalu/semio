/** 🧪 Real-code replay of the provider-closure batches on a mirror: product-shaped closures (the print TypeScript package test behind the owner-command driver through generic-target, the native cargo command fallback of every Rust target, the test discovery) and every group of the real providers project (provider and tool closures) are resolved; a scheduling-only edit (the leases port imports a new leases module), a keyed provider edit (the repository-tooling provider imports a new library module) and a keyed tool edit (the cargo driver imports a new library module) are applied in the mirror, each held derived fact is asked again with the real deriveClosureFactV1, then undone byte-exactly. Prints, per holder, its file count, references and whether its facts change. usage (inside the mirror seal): bun realreplay.ts <mirror-root> */
import {chmodSync,mkdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
const root=process.argv[2]!,L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import(L+"/🏗️build/👁️observation/🟨️.mjs") as any,si=await import(L+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/📥️owner-command/📜️script.ts",pkg="🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript";
const target={options:{cwd:".",command:`repo "${owner}" owner-command --cwd "${pkg}" -- repo "${root}/${pkg}/📜️script.ts" test`,ownerArguments:true}};
const t0=performance.now();
const resolved=await observation.observeInferenceSnapshotV1(async()=>{
 const api=si.closureApiV1(si.createScriptInputCache({workspaceRoot:root,lateClosures:true}),root),handles:Record<string,any>={
  "print-test (generic-target)":si.closureHandleOfV1(await api.genericTargetCommandInputs(target,pkg)),
  "cargo fallback (native-command)":si.closureHandleOfV1(await api.nativeCommandInputs()),
  "print-test whole (test-discovery)":si.closureHandleOfV1(await api.closureOf([join(root,owner),join(root,pkg,"📜️script.ts")],[],"test-discovery")),
 };
 for(const[name,list]of await api.sharedClosureGroups("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers","🔣️.json"))handles[name]=si.closureHandleOfV1(list);
 const out:Record<string,{value:any;rows:any[]}>={};
 for(const[name,h]of Object.entries(handles)){const r=await api.resolve([[h.key,h.recipe]]);out[name]={value:r.values.get(h.key),rows:r.derived.filter((d:any)=>JSON.parse(d.row.path).walk===1).map((d:any)=>d.row)};}
 return out;
});
const list=(value:any)=>Array.isArray(value)?value:value?.closure??[];
console.log("[DEBUG] resolve ms",Math.round(performance.now()-t0));
for(const[name,row]of Object.entries(resolved))console.log("[DEBUG] holder",JSON.stringify({name,files:list(row.value).filter((x:any)=>typeof x==="string").length,references:list(row.value).filter((x:any)=>typeof x?.input==="string").map((x:any)=>x.input),facts:row.rows.length}));
const ask=async(rows:any[])=>{const out:string[]=[];for(const row of rows)out.push((await observation.observeInferenceSnapshotV1(()=>si.deriveClosureFactV1(row,si.createScriptInputCache({workspaceRoot:root})))).fact);return out;};
const base:Record<string,string[]>={};for(const[name,row]of Object.entries(resolved))base[name]=await ask(row.rows);
console.log("[DEBUG] baseline holds",Object.entries(resolved).every(([name,row])=>row.rows.every((r:any,i:number)=>r.fact===base[name]![i])));
const edit=(file:string,line:string,newFile:string)=>{const p=join(root,file),mode=statSync(p).mode&0o777,text=readFileSync(p,"utf8");mkdirSync(dirname(join(root,newFile)),{recursive:true});writeFileSync(join(root,newFile),"export const peeredit2ProbeV1 = 1;\n");const at=text.startsWith("#!")?text.indexOf("\n")+1:0;writeFileSync(p,text.slice(0,at)+line+text.slice(at));chmodSync(p,mode);return()=>{writeFileSync(p,text);chmodSync(p,mode);rmSync(dirname(join(root,newFile)),{recursive:true});};};
for(const[label,file,line,newFile]of[["keyed tool (the cargo driver imports a new library module)","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts",'import "./📍️peeredit2-probe/🟦️.ts";\n',"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📍️peeredit2-probe/🟦️.ts"],["scheduling-only (leases port imports a new leases module)","🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts",'import "./📍️peeredit2-probe/🟦️.ts";\n',"🧰️framework/🔨️modules/🏃️process/🔒️leases/📍️peeredit2-probe/🟦️.ts"],["keyed provider (repository-tooling provider imports a new library module)","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts",'import "./📍️peeredit2-probe/🟦️.ts";\n',"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/📍️peeredit2-probe/🟦️.ts"]] as const){
 const undo=edit(file,line,newFile),changed:string[]=[],held:string[]=[];const t=performance.now();
 try{for(const[name,row]of Object.entries(resolved)){const again=await ask(row.rows);(again.some((fact,i)=>fact!==base[name]![i])?changed:held).push(name);}}finally{undo();}
 console.log("[DEBUG] edit",JSON.stringify({label,ms:Math.round(performance.now()-t),held:held.length,changedProviderGroups:changed.filter(name=>name.startsWith("scriptProvider:")).length,changed:changed.filter(name=>!name.startsWith("scriptProvider:"))}));
}
const back:boolean[]=[];for(const[name,row]of Object.entries(resolved))back.push((await ask(row.rows)).every((fact,i)=>fact===base[name]![i]));
console.log("[DEBUG] after undo holds",back.every(Boolean),"rss MB",Math.round(process.memoryUsage().rss/2**20));
