import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
const ROOT="/Users/ueli/Documents/semio/",STAGE=process.env.R139_STAGE??"";
const staged=new Map<string,string>();
const walk=(directory:string)=>{for(const name of readdirSync(directory)){const path=join(directory,name);if(statSync(path).isDirectory())walk(path);else staged.set(ROOT+path.slice(STAGE.length+1),path);}};
if(STAGE&&existsSync(STAGE))walk(STAGE);
if(staged.size)Bun.plugin({name:"r139-stage",setup(build:any){
 const filter=new RegExp("^(?:"+[...staged.keys()].map(path=>path.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|")+")$","u");
 build.onLoad({filter},(args:any)=>{
  const text=readFileSync(staged.get(args.path)!,"utf8");
  if(args.path.endsWith(".json")){const doc=JSON.parse(text);return{exports:{...doc,default:doc},loader:"object"};}
  return{contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
}});
