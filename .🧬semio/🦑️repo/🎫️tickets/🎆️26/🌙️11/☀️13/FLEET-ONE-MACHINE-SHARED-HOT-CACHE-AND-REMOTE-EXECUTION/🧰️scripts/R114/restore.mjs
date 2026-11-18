import{copyFileSync,existsSync,rmSync,readdirSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R114/backup/";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",N=B+"🕸️graph/📥️inference/📦️project-nodes/",F=N+"🏊️project-farm/";
for(const k of[B+"👁️observation/🟨️.mjs",N+"🟨️.mjs",F+"🟨️.mjs",F+"🟨️.d.mts",F+"👷️service/🟨️.mjs",B+"👁️observation/🟨️.d.mts"])if(existsSync(BK+k)){copyFileSync(BK+k,R+k);console.log("restored",k.slice(-30));}
const T=R+B+"👁️observation/🧪️tests/";for(const d of["🤝️adopted-verdict","🧒️children"]){const full=T+d;if(!existsSync(full)||!full.startsWith("/Users/ueli/Documents/semio/🧰️framework/"))continue;rmSync(full,{recursive:true,force:true});console.log("removed",d);}
