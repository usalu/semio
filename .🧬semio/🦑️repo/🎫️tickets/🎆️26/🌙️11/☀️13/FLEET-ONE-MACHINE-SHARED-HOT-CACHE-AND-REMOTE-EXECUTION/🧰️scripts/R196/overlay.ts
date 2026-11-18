import{plugin}from"bun";
import{existsSync}from"node:fs";
import{dirname,resolve}from"node:path";
const MIRROR=process.env.R196_MIRROR!,LIVE=process.env.R196_LIVE!;
const logical=(path:string):string=>path.startsWith(MIRROR+"/")?LIVE+path.slice(MIRROR.length):path;
const mirrored=(path:string):string=>MIRROR+path.slice(LIVE.length);
plugin({name:"r196-overlay",setup(b){
 b.onResolve({filter:/^\.{1,2}\//},args=>{
  const target=resolve(dirname(logical(args.importer)),args.path);
  if(!target.startsWith(LIVE+"/"))return undefined;
  const staged=mirrored(target);
  if(existsSync(staged))return{path:staged};
  if(existsSync(target))return{path:target};
  return undefined;
 });
}});
