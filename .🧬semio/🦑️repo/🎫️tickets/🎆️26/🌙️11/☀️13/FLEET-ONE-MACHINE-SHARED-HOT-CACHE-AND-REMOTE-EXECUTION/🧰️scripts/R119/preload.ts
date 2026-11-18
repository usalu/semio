import{readFileSync}from"node:fs";import{apply,all}from"./patches.mjs";
Bun.plugin({name:"r119",setup(b:any){b.onLoad({filter:/🟨️\.mjs$/},(args:any)=>{const text=readFileSync(args.path,"utf8");return{contents:apply(args.path,text,all),loader:"js"};});}});
