import "./plug.ts";
import{mkdirSync,mkdtempSync,realpathSync,writeFileSync}from"node:fs";import{tmpdir}from"node:os";import{dirname,join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const fixture=(await import(B+"🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🔣️.json")).default as any;
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import(B+"👁️observation/🟨️.mjs") as any,scriptInputs=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const root=realpathSync(mkdtempSync(join(tmpdir(),"dbg-tr-")));for(const[path,text]of Object.entries({...fixture.files,"tools/📜️script.ts":'import "./entry.mjs";\nexport {};\n'}) as [string,string][]){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
const target={options:{cwd:".",command:"repo ./tools/📜️script.ts run"}};
const resolved=await observation.observeInferenceSnapshotV1(async()=>{const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.genericTargetCommandInputs(target,"."),key=scriptInputs.closureHandleOfV1(list).key;return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);});
const row=resolved.rows.find((r:any)=>r.kind==="global"&&JSON.parse(r.path)[0]==="closure-graph");
console.log("[DEBUG] row",row.path.slice(0,300),row.fact);
writeFileSync(join(root,"tools/m5.mjs"),'import "./m6.mjs";\nexport const m5 = 6;\n');
const out=await observation.observeInferenceSnapshotV1(async()=>{try{await observation.prepareGlobalFactsV1([row]);}catch(e){console.log("[DEBUG] prepare threw",e);}return observation.observationRowFreshV1(row);});
console.log("[DEBUG] holds",out);
const[, ws, seg]=JSON.parse(row.path);
const direct=await observation.observeInferenceSnapshotV1(async()=>{const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:true});scriptInputs.closureApiV1(scripts,root);try{return await scriptInputs.closureGraphAnswerV1(scripts,root,{roots:seg.roots.map((n:string)=>join(root,n.slice(16))),requiredSources:seg.requiredSources,resources:[],selectors:seg.selectors.map(([n,w]:[string,string[]])=>[join(root,n.slice(16)),w]),stops:seg.stops.map(([n,t]:[string,string])=>[join(root,n.slice(16)),t]),view:seg.view});}catch(e){return String(e?.stack??e).slice(0,1500);}});
console.log("[DEBUG] direct",direct);
process.exit(0);
