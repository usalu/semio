import{readFileSync}from"node:fs";import*as P from"./patches.mjs";
const t=readFileSync("/Users/ueli/Documents/semio/"+P.LANE,"utf8");
const src=readFileSync(import.meta.dir+"/patches.mjs","utf8");const m=/const oldIntake=`([\s\S]*?)`;/.exec(src)[1];
let n=0;while(n<m.length&&t.includes(m.slice(0,n+1)))n++;console.log("matched prefix",n,"of",m.length,JSON.stringify(m.slice(n-20,n+40)));
