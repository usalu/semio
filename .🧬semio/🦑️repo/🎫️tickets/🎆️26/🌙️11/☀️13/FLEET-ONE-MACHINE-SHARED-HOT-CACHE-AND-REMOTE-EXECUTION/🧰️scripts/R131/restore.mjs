import{copyFileSync,renameSync,rmSync}from"node:fs";import{all}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R131/backup/";
for(const k of new Set(all.map(a=>a.file))){copyFileSync(BK+k,R+k+".r131dtmp");renameSync(R+k+".r131dtmp",R+k);}console.log("restored");
{const{news}=await import(import.meta.dir+"/news.mjs");for(const[,to]of news){try{rmSync(R+to);}catch{}}console.log("removed new tests");}
