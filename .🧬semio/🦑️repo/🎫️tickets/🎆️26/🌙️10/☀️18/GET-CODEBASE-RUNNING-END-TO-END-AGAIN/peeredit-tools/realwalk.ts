/** 🧪 Real-code replay on a mirror with walk-views applied: a product-shaped command (owner-command around the print TypeScript package script) resolved through generic-target (scheduling view) and closure-of (whole walk); a scheduling-only edit and a keyed edit are applied in the mirror, each held fact is asked again with the real deriveClosureFactV1, then everything is undone. usage (inside the mirror seal): bun realwalk.ts <mirror-root> */
import {mkdirSync,readFileSync,rmSync,writeFileSync,statSync,chmodSync} from "node:fs";
import {dirname,join} from "node:path";
const root=process.argv[2]!,L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const inference=await import(L+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import(L+"/👁️observation/🟨️.mjs") as any,scriptInputs=await import(L+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/📥️owner-command/📜️script.ts",pkg="🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript";
const target={options:{cwd:".",command:`repo "${owner}" owner-command --cwd "${pkg}" -- repo "${root}/${pkg}/📜️script.ts" test`,ownerArguments:true}};
const t0=performance.now();
const first=await observation.observeInferenceSnapshotV1(async()=>{const scripts=scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
 const own=scriptInputs.closureHandleOfV1(await api.genericTargetCommandInputs(target,pkg)),whole=scriptInputs.closureHandleOfV1(await api.closureOf([join(root,owner),join(root,pkg,"📜️script.ts")],[]));
 return{own:await api.resolve([[own.key,own.recipe]]),whole:await api.resolve([[whole.key,whole.recipe]])};});
const walks=(r:any)=>r.derived.filter((f:any)=>JSON.parse(f.row.path).walk===1).map((f:any)=>f.row);
const own=walks(first.own),whole=walks(first.whole);
console.log("[DEBUG] resolve ms",Math.round(performance.now()-t0),"own walks",own.map((r:any)=>JSON.parse(r.path).view??"whole"),"whole walks",whole.map((r:any)=>JSON.parse(r.path).view??"whole"),"roster",[...first.whole.values.values()][0].length);
const ask=async(rows:any[])=>{const out=[];for(const row of rows){const t=performance.now();out.push({fact:(await observation.observeInferenceSnapshotV1(()=>scriptInputs.deriveClosureFactV1(row,scriptInputs.createScriptInputCache({workspaceRoot:root})))).fact,ms:Math.round(performance.now()-t)});}return out;};
const all=[...own,...whole],base=await ask(all);
console.log("[DEBUG] baseline holds",base.every((a,i)=>a.fact===all[i].fact),"ms",base.map(a=>a.ms));
const edit=(file:string,line:string,newFile:string)=>{const p=join(root,file),mode=statSync(p).mode&0o777,text=readFileSync(p,"utf8");mkdirSync(dirname(join(root,newFile)),{recursive:true});writeFileSync(join(root,newFile),"export const peereditProbeV1 = 1;\n");writeFileSync(p,line+text);chmodSync(p,mode);return()=>{writeFileSync(p,text);chmodSync(p,mode);rmSync(dirname(join(root,newFile)),{recursive:true});};};
for(const[name,file,line,newFile]of[["scheduling-only (leases port imports a new leases module)","🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts",'import "./📍️peeredit-probe/🟦️.ts";\n',"🧰️framework/🔨️modules/🏃️process/🔒️leases/📍️peeredit-probe/🟦️.ts"],["keyed (repository-tooling provider imports a new library module)","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts",'import "./📍️peeredit-probe/🟦️.ts";\n',"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/📍️peeredit-probe/🟦️.ts"]] as const){
 const undo=edit(file,line,newFile);const again=await ask(all);undo();
 console.log("[DEBUG]",name,"changed:",all.map((r:any,i:number)=>(JSON.parse(r.path).view??"whole")+"="+(again[i].fact!==base[i].fact)).join(" "),"ms",again.map(a=>a.ms));}
const back=await ask(all);console.log("[DEBUG] after undo holds",back.every((a,i)=>a.fact===base[i].fact));
