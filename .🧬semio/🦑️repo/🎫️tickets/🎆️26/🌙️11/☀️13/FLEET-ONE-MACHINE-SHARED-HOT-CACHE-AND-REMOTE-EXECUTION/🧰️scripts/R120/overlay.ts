import{readFileSync,existsSync}from"node:fs";import{dirname,resolve}from"node:path";import*as P from"./patches.mjs";const all=(P as any)[process.env.R120_LIST??"all"],apply=P.apply;
const ROOT="/Users/ueli/Documents/semio",STAGE=process.env.R120_STAGE??"/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120/stage";
const files=process.env.R120_BASE==="1"?["/nonexistent"]:[...new Set(all.map(item=>item.file))],escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),only=new RegExp("(?:"+files.map(file=>escape(ROOT+"/"+file)).join("|")+")$","u");
Bun.plugin({name:"r120",setup(b:any){
 b.onLoad({filter:only},(args:any)=>{
  const text=apply(args.path.slice(ROOT.length+1),readFileSync(args.path,"utf8"),all);
  return args.path.endsWith(".json")?{exports:{default:JSON.parse(text),...JSON.parse(text)},loader:"object"}:{contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
 b.onResolve({filter:/^\.{1,2}\//},(args:any)=>{
  if(!args.importer.startsWith(STAGE+"/"))return undefined;
  const live=resolve(ROOT,dirname(args.importer.slice(STAGE.length+1)),args.path);
  return existsSync(live)?{path:live}:undefined;
 });
}});
