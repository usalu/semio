import{readFileSync}from"node:fs";import{apply,all}from"./patches2.mjs";
const files=new Set(all.map(a=>a.file));
Bun.plugin({name:"r121b",setup(b:any){b.onLoad({filter:/🟨️\.mjs$/},(args:any)=>{const text=readFileSync(args.path,"utf8");const key=[...files].find(f=>args.path.endsWith(f));return{contents:key===undefined?text:apply(key,text,all),loader:"js"};});}});
