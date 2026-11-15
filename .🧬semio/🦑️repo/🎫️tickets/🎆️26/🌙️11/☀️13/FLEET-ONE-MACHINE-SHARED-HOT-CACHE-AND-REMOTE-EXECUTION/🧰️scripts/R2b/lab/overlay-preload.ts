console.error("[r2b-overlay] preload loaded", process.env.R2b_OVERLAY);
import{plugin}from"bun";import{existsSync,statSync}from"node:fs";import{dirname,resolve}from"node:path";
const OVERLAY=process.env.R2b_OVERLAY!,LIVE=process.env.R2b_LIVE!;
plugin({name:"r2b-overlay",setup(build){
 build.onResolve({filter:/^\.\.?\//u},args=>{
  const importer=args.importer;if(!importer||!importer.startsWith(OVERLAY+"/"))return undefined;
  const target=resolve(dirname(importer),args.path);
  if(existsSync(target)&&statSync(target).isFile())return{path:target};
  const live=LIVE+target.slice(OVERLAY.length);
  if(existsSync(live))return{path:live};
  return undefined;
 });
 build.onResolve({filter:/^[^./]/u},args=>{
  if(args.path.startsWith("bun:")||args.path.startsWith("node:"))return undefined;
  const importer=args.importer;if(!importer||!importer.startsWith(OVERLAY+"/"))return undefined;
  let from=dirname(LIVE+importer.slice(OVERLAY.length));while(!existsSync(from)&&from.length>LIVE.length)from=dirname(from);
  try{return{path:Bun.resolveSync(args.path,from)};}catch(error){console.error("[r2b-overlay] "+args.path+" "+String(error));return undefined;}
 });
}});
