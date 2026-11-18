import{readFileSync,copyFileSync,existsSync,rmSync,rmdirSync}from"node:fs";import{LIVE,SC,BK}from"./base.mjs";
const base=JSON.parse(readFileSync(SC+"base.json","utf8"));
for(const r of Object.keys(base)){if(base[r]===null){rmSync(LIVE+r,{force:true});}else if(existsSync(BK+r)){copyFileSync(BK+r,LIVE+r);console.log("restored",r);}}
for(const d of["🚪️exits","🧮️lifecycle","🧪️tests/🚪️exits","🧪️tests/🧮️lifecycle"])try{rmdirSync(LIVE+d);}catch{}
