import{copyFileSync,rmSync,renameSync,readFileSync}from"node:fs";import{dirname}from"node:path";import*as P from"./patches.mjs";
const LIST=process.env.R120_LIST??"all",all=P[LIST];const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120/",BK=SC+"backup-"+LIST+"/";
for(const k of new Set(all.map(a=>a.file))){copyFileSync(BK+k,R+k+".r120tmp");renameSync(R+k+".r120tmp",R+k);}
for(const k of (()=>{try{return readFileSync(SC+"staged-"+LIST+".list","utf8");}catch{return"";}})().split("\n").filter(Boolean)){rmSync(R+k,{force:true});try{rmSync(dirname(R+k));}catch{}}
console.log("restored");
