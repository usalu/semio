import{readFileSync}from"node:fs";import{execFileSync}from"node:child_process";
const dir=process.argv[2];
const old=await import(dir+"/old-imports.tmp.mjs"),cur=await import(dir+"/🟨️.mjs");
const files=execFileSync("git",["ls-files","-z","--","*.ts","*.tsx","*.mjs","*.d.ts","*.d.mts","*.json"],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<28}).toString().split("\0").filter(Boolean);
const pick=files.filter((_,i)=>i%12===0).slice(0,3500);
let diff=0,n=0,total=0;
const names=["commandSourceImports","runtimeModuleSourceImports","moduleSourceImports","productionModuleSourceImports"];
for(const f of pick){let text;try{text=readFileSync("/Users/ueli/Documents/semio/"+f,"utf8");}catch{continue;}if(text.length>400000)continue;n++;
 for(const name of names){let a,b;try{a=JSON.stringify(old[name](f,text));}catch(e){a="ERR"+e.message.slice(0,40);}try{b=JSON.stringify(cur[name](f,text));}catch(e){b="ERR"+e.message.slice(0,40);}total++;if(a!==b){diff++;if(diff<5)console.log("DIFF",name,f,a.slice(0,100),b.slice(0,100));}}}
console.log("files",n,"comparisons",total,"different",diff,"stats",JSON.stringify(cur.commandSourceParseStats()),JSON.stringify(old.commandSourceParseStats()));
