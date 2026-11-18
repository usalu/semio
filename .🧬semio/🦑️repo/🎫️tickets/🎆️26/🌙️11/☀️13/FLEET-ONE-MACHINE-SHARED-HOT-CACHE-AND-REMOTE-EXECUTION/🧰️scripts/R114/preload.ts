import{readFileSync}from"node:fs";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",N=B+"🕸️graph/📥️inference/📦️project-nodes/",F=N+"🏊️project-farm/",S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R114/";
const map=new Map([[B+"👁️observation/🟨️.mjs",S+"obs.stage.mjs"],[N+"🟨️.mjs",S+"stage/project-nodes.mjs"],[F+"🟨️.mjs",S+"stage/farm.mjs"],[F+"👷️service/🟨️.mjs",S+"stage/service.mjs"]]);
Bun.plugin({name:"r114",setup(b:any){b.onLoad({filter:/🟨️\.mjs$/},(args:any)=>{const staged=map.get(args.path);return{contents:readFileSync(staged??args.path,"utf8"),loader:"js"};});}});
