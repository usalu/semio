import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const ce=process.argv[2],count=Number(process.argv[3]??6);
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs]).sort((a,b)=>b[1]-a[1]).slice(0,400);
const picked=[];for(const[n]of names){const e=JSON.parse(brotliDecompressSync(readFileSync(join(ce,"e",n))).toString("utf8"));if(e.recipe&&["tool-closure","script-provider","target-sources","native-target"].includes(e.recipe.fn)&&!picked.some(p=>p.fn===e.recipe.fn&&p.n>=2))picked.push({fn:e.recipe.fn,recipe:e.recipe,n:picked.filter(p=>p.fn===e.recipe.fn).length+1});if(picked.length>=count)break;}
const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:true});si.closureApiV1(scripts,root);
for(const p of picked){const t=performance.now();try{const w=await si.deriveClosureRecipeV1(root,p.recipe,scripts);console.log(JSON.stringify({fn:p.fn,ms:Math.round(performance.now()-t),files:Array.isArray(w.value)?w.value.length:Object.keys(w.value??{}).length,rows:w.rows.length,layers:w.layers.length}));}catch(e){console.log(JSON.stringify({fn:p.fn,error:String(e.message).slice(0,200)}));}}
