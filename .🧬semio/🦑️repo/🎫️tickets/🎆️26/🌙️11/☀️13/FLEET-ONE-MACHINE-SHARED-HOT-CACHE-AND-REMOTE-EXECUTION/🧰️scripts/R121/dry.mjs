import{readFileSync}from"node:fs";import{all,apply}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/";let n=0;for(const f of new Set(all.map(a=>a.file))){const out=apply(f,readFileSync(R+f,"utf8"),all);n++;console.log(f.slice(-40),out.length);}console.log("dry ok",n);
