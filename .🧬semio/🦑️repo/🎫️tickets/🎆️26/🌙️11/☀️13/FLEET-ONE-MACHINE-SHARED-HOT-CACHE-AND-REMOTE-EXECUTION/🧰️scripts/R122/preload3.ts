import{readFileSync}from"node:fs";import{apply,all}from"./patches3.mjs";
const files=new Set(all.map(a=>a.file));
Bun.plugin({name:"r122",setup(b:any){b.onLoad({filter:/(🟨️\.mjs|🟨️\.d\.mts|🟦️\.ts)$/},(args:any)=>{const text=readFileSync(args.path,"utf8"),key=[...files].find(f=>args.path.endsWith(f));return{contents:key===undefined?text:apply(key,text,all),loader:/\.(ts|mts)$/.test(args.path)?"ts":"js"};});}});
