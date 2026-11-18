import{copyFileSync,existsSync,rmSync,rmdirSync}from"node:fs";import{all}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R112/backup/",D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⏳️stretch-verdicts/";
for(const p of new Set(all.map(item=>item.file)))if(existsSync(BK+p)){copyFileSync(BK+p,R+p);console.log("restored",p.slice(-30));}
rmSync(R+D+"🟦️.ts",{force:true});try{rmdirSync(R+D);}catch{}
