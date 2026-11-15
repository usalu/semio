import {plugin} from "bun";
import {existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const LIVE=process.env.P2_LIVE!,MIRROR=process.env.P2_MIRROR!;
const toMirror=(path:string)=>path.startsWith(LIVE+"/")?join(MIRROR,path.slice(LIVE.length+1)):undefined;
const toLive=(path:string)=>path.startsWith(MIRROR+"/")?join(LIVE,path.slice(MIRROR.length+1)):undefined;
plugin({name:"p2-overlay",setup(build){
 build.onResolve({filter:/^\.\.?\//u},args=>{
  const importer=args.importer;if(!importer)return undefined;
  const target=resolve(dirname(importer),args.path);if(process.env.P2_DEBUG)console.error("[rel]",importer.slice(-60),args.path);
  const live=toLive(target)??target,mirror=toMirror(live);
  if(mirror&&existsSync(mirror))return{path:mirror};
  if(importer.startsWith(MIRROR+"/")&&existsSync(live))return{path:live};
  return undefined;
 });
 build.onResolve({filter:/.*/},args=>{if(args.path.startsWith(".")||args.path.startsWith("/"))return undefined;console.error("[bare]",args.path);
  const importer=args.importer;if(!importer||!importer.startsWith(MIRROR+"/")||args.path.startsWith("node:")||args.path==="bun"||args.path.startsWith("bun:"))return undefined;
  try{return{path:Bun.resolveSync(args.path,dirname(toLive(importer)!))};}catch{return undefined;}
 });
}});
