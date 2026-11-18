import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R117/stage/";
const B=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",H=R+"🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/",N=B+"🕸️graph/📥️inference/📦️project-nodes/",F=N+"🏊️project-farm/";
const map=new Map<string,[string,string]>(JSON.parse(readFileSync(SC+"map.json","utf8")).map(([k,v,l]:[string,string,string])=>[R+k,[SC+v,l]]));
Bun.plugin({name:"r117",setup(b:any){b.onLoad({filter:/(🟨️\.mjs|🟦️\.ts)$/},(args:any)=>{const m=map.get(args.path),loader=args.path.endsWith(".json")?"json":args.path.endsWith(".ts")?"ts":"js";return{contents:readFileSync(m?m[0]:args.path,"utf8"),loader:m?m[1]:loader};});}});
