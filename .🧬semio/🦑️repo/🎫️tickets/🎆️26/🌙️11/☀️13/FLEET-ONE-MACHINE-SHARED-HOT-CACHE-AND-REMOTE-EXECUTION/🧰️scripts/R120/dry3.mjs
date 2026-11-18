import{readFileSync}from"node:fs";import{second,apply}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/";for(const k of new Set(second.map(a=>a.file))){const o=readFileSync(R+k,"utf8"),n=apply(k,o,second);console.log(k.slice(-44),o.length,"->",n.length);new Bun.Transpiler({loader:"js"}).transformSync(n);}
