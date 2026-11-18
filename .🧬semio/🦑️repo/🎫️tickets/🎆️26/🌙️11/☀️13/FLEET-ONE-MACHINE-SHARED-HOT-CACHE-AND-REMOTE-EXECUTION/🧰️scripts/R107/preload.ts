import{readFileSync}from"node:fs";import{apply,all}from"./patches.mjs";
const names=(process.env.R107_PATCH??"").split(",").filter(Boolean);
Bun.plugin({name:"r107",setup(b:any){b.onLoad({filter:/🟨️\.mjs$/},(args:any)=>{const text=readFileSync(args.path,"utf8");return{contents:apply(args.path,text,all.filter((_:any,i:number)=>names.includes(String(i)))),loader:"js"};});}});
