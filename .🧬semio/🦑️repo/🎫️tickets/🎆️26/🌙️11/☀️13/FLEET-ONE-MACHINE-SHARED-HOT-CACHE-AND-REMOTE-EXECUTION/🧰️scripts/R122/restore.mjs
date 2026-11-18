import{copyFileSync,renameSync}from"node:fs";import{patchedFiles}from"./files.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/backup/";
for(const k of patchedFiles){copyFileSync(BK+k,R+k+".r122tmp");renameSync(R+k+".r122tmp",R+k);}console.log("restored");
