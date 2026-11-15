import{plugin}from"bun";import{mock}from"bun:test";import fs from"node:fs";import{existsSync,readFileSync,statSync}from"node:fs";import{dirname,resolve}from"node:path";
const OVERLAY=process.env.R5_OVERLAY!,LIVE=process.env.R5_LIVE!;
const redirect=new Map<string,string>();
const walk=(base:string,rel:string)=>{for(const entry of fs.readdirSync(resolve(base,rel),{withFileTypes:true})){const next=rel===""?entry.name:rel+"/"+entry.name;if(entry.isDirectory())walk(base,next);else redirect.set(LIVE+"/"+next,OVERLAY+"/"+next);}};
walk(OVERLAY,"");
const original=fs.readFileSync,wrapped=function(path:any,...rest:any[]){if(typeof path==="string"){const to=redirect.get(path);if(to!==undefined&&existsSync(to))return original.call(fs,to,...rest);}return original.call(fs,path,...rest);};
const patched={...fs,readFileSync:wrapped};
mock.module("node:fs",()=>({...patched,default:patched}));mock.module("fs",()=>({...patched,default:patched}));
plugin({name:"r5-overlay",setup(build){
 build.onResolve({filter:/^\.\.?\//u},args=>{
  const importer=args.importer;if(!importer)return undefined;
  const base=importer.startsWith(OVERLAY+"/")?OVERLAY:importer.startsWith(LIVE+"/")?LIVE:undefined;if(base===undefined)return undefined;
  const target=resolve(dirname(importer),args.path),live=base===OVERLAY?LIVE+target.slice(OVERLAY.length):target;
  const over=redirect.get(live);
  if(over!==undefined&&existsSync(over))return{path:over};
  if(base===OVERLAY){if(existsSync(target)&&statSync(target).isFile())return{path:target};if(existsSync(live))return{path:live};}
  return undefined;
 });
 build.onResolve({filter:/^[^./]/u},args=>{
  if(args.path.startsWith("bun:")||args.path.startsWith("node:"))return undefined;
  const importer=args.importer;if(!importer||!importer.startsWith(OVERLAY+"/"))return undefined;
  let from=dirname(LIVE+importer.slice(OVERLAY.length));while(!existsSync(from)&&from.length>LIVE.length)from=dirname(from);
  try{return{path:Bun.resolveSync(args.path,from)};}catch{return undefined;}
 });
}});
