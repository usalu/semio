import{copyFileSync,existsSync,rmSync,rmdirSync}from"node:fs";import{all}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R111/backup/",D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧯️old-record-settle/";
for(const p of all)if(existsSync(BK+p.file)){copyFileSync(BK+p.file,R+p.file);console.log("restored",p.file.slice(-30));}
rmSync(R+D+"🟦️.ts",{force:true});try{rmdirSync(R+D);}catch{}
