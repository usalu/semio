import{readFileSync}from"node:fs";import{all,apply}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/";for(const k of new Set(all.map(a=>a.file))){const o=readFileSync(R+k,"utf8"),n=apply(k,o,all);console.log(k.slice(-44),o.length,"->",n.length);}
