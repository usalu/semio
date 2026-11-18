import{copyFileSync,renameSync}from"node:fs";import{patchedFiles}from"./files.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R121/backup/";
for(const k of patchedFiles){copyFileSync(BK+k,R+k+".r121tmp");renameSync(R+k+".r121tmp",R+k);}console.log("restored");
