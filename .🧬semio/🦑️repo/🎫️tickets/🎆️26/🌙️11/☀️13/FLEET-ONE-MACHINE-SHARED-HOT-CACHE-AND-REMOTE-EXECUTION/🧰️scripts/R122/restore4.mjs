import{copyFileSync,renameSync}from"node:fs";import{all}from"./patches4.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/backup4/";
for(const k of new Set(all.map(a=>a.file))){copyFileSync(BK+k,R+k+".r122dtmp");renameSync(R+k+".r122dtmp",R+k);}console.log("restored");
