import{readFileSync,copyFileSync,rmSync,existsSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/",BK=SC+"backup/";
for(const live of JSON.parse(readFileSync(SC+"landed.json","utf8"))){if(existsSync(BK+live))copyFileSync(BK+live,R+live);else rmSync(R+live,{force:true});console.log("restored",live.slice(-40));}
