import{mkdtempSync,mkdirSync,writeFileSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const o=await import(L+"🏗️build/👁️observation/🟨️.mjs");
const scratch=mkdtempSync(join(realpathSync(tmpdir()),"bp-")),lib=join(scratch,"library"),np=join(scratch,"np");mkdirSync(lib,{recursive:true});
await o.observeInference({cachePath:join(scratch,"init.json"),key:"i"},()=>null);
const closure=Array.from({length:400},(_,i)=>{const p=join(lib,"m"+i+".ts");writeFileSync(p,"export const v"+i+"="+i+";\n");return p;});
await Bun.sleep(2200);
await o.observeInference({cachePath:join(np,"a.json"),key:["blocks","a"]},()=>{let t=0;for(const p of closure)t+=o.observedFs.readFileSync(p,"utf8").length;return{t};});
const h=JSON.parse(o.inferenceHeaderLine(join(np,"a.json")));console.log(h.blocks.length,h.observations.length);
