import{readFileSync}from"node:fs";import{all}from"./patches.mjs";
const ROOT="/Users/ueli/Documents/semio",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120/backup/";
const files=[...new Set(all.map(item=>item.file))],escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),only=new RegExp("(?:"+files.map(file=>escape(ROOT+"/"+file)).join("|")+")$","u");
Bun.plugin({name:"r120-base",setup(b:any){
 b.onLoad({filter:only},(args:any)=>{
  const text=readFileSync(BK+args.path.slice(ROOT.length+1),"utf8");
  return args.path.endsWith(".json")?{exports:{default:JSON.parse(text),...JSON.parse(text)},loader:"object"}:{contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
}});
