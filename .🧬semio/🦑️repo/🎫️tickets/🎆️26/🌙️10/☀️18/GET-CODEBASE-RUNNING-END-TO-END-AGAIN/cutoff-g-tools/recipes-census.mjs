import{readFileSync,readdirSync,statSync,writeFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n));
const keys=new Map();let used=0,records=0,bytes=0;const perRecord=[];const out={};
for(const name of names){let text;try{text=brotliDecompressSync(readFileSync(D+name)).toString("utf8");}catch{continue;}
 const nl=text.indexOf("\n");let value;try{value=JSON.parse(text.slice(nl+1));}catch{continue;}
 records++;const r=value.closureRecipes;if(!r)continue;const k=Object.keys(r);perRecord.push(k.length);used+=k.length;
 for(const[key,recipe]of Object.entries(r)){if(!keys.has(key)){const s=JSON.stringify(recipe);keys.set(key,s.length);bytes+=s.length;}}
 out[value.root]={mtime:statSync(D+name).mtimeMs,keys:k};}
perRecord.sort((a,b)=>a-b);
console.log(JSON.stringify({files:names.length,records,withRecipes:perRecord.length,distinctRecipes:keys.size,distinctRecipeBytes:bytes,usesTotal:used,perRecordMedian:perRecord[perRecord.length>>1],perRecordMax:perRecord.at(-1)}));
writeFileSync(process.argv[2],JSON.stringify(out));
