import{plugin}from"bun";
import{readFileSync}from"node:fs";
const all:Record<string,string>=JSON.parse(readFileSync(process.env.R193_MAP!,"utf8")),filter=new RegExp("^(?:"+Object.keys(all).map(k=>k.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|")+")$");
process.env.BUN_OPTIONS="--preload "+import.meta.path;
plugin({name:"r193",setup(b){
 b.onLoad({filter},a=>{
  const text=readFileSync(all[a.path]!,"utf8");
  return a.path.endsWith(".json")?{exports:{default:JSON.parse(text)},loader:"object"}:{contents:text,loader:a.path.endsWith(".ts")?"ts":"js"};
 });
}});
