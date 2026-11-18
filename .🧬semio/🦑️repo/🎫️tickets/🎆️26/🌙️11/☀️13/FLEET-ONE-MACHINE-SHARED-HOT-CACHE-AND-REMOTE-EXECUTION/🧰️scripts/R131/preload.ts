import{readFileSync}from"node:fs";import{apply,all}from"./patches.mjs";
const files=new Set(all.map(a=>a.file));
Bun.plugin({name:"r131",setup(b:any){b.onLoad({filter:/(🟨️\.mjs|🟨️\.d\.mts|🟦️\.ts)$/},(args:any)=>{const text=readFileSync(args.path,"utf8"),key=[...files].find(f=>args.path.endsWith(f));return{contents:key===undefined?text:apply(key,text,all),loader:/\.(ts|mts)$/.test(args.path)?"ts":"js"};});
b.onLoad({filter:/read-parent\/🧬️schema\/🔣️\.json$/},(args:any)=>{const text=readFileSync(args.path,"utf8"),key=[...files].find(f=>args.path.endsWith(f));const value=JSON.parse(apply(key as string,text,all));return{exports:{...value,default:value},loader:"object"};});}});
{const fs=require("node:fs"),original=fs.readFileSync;
fs.readFileSync=function(path:any,...rest:any[]){const result=original.call(this,path,...rest);if(typeof path!=="string"&&!(path instanceof URL))return result;const text=String(path instanceof URL?path.pathname:path),key=[...files].find(f=>text.endsWith(f));if(key===undefined||typeof result!=="string")return result;return apply(key,result,all);};}
