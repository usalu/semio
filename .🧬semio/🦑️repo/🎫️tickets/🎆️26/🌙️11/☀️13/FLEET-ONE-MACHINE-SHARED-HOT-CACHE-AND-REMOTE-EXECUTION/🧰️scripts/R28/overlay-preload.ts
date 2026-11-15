import{plugin}from"bun";import{existsSync,readFileSync,readdirSync,statSync}from"node:fs";import{dirname,resolve}from"node:path";
const OVERLAY=process.env.R5_OVERLAY!,LIVE=process.env.R5_LIVE!;
const served=new Map<string,string>();
const walk=(rel:string)=>{for(const entry of readdirSync(resolve(OVERLAY,rel),{withFileTypes:true})){const next=rel===""?entry.name:rel+"/"+entry.name;if(entry.isDirectory())walk(next);else served.set(LIVE+"/"+next,OVERLAY+"/"+next);}};walk("");
const liveOf=(path:string)=>LIVE+path.slice(OVERLAY.length);
const bare=/((?:from|import\s*\(|import)\s*)(["'])((?:@[a-z0-9-]+\/[a-z0-9._\/-]+)|(?:[a-z][a-z0-9._-]*(?:\/[a-z0-9._\/-]+)?))\2/giu;
plugin({name:"r28-overlay",setup(build){
 build.onResolve({filter:/^\.\.?\//u},args=>{
  const importer=args.importer;if(!importer)return undefined;
  const inOverlay=importer.startsWith(OVERLAY+"/"),target=resolve(dirname(importer),args.path),served_=served.get(inOverlay?liveOf(target):target);
  if(served_!==undefined)return{path:served_};
  if(!inOverlay)return undefined;
  if(existsSync(target)&&statSync(target).isFile())return{path:target};
  const live=liveOf(target);if(existsSync(live)&&statSync(live).isFile())return{path:live};
  return undefined;
 });
 build.onLoad({filter:/\.ts$/u},args=>{
  if(!args.path.startsWith(OVERLAY+"/"))return{contents:readFileSync(args.path,"utf8"),loader:"ts"};
  const from=dirname(liveOf(args.path));let base=from;while(!existsSync(base))base=dirname(base);
  const text=readFileSync(args.path,"utf8").replace(bare,(whole,head,quote,spec)=>{if(spec.startsWith("node:")||spec.startsWith("bun:")||["fs","path","os","crypto","child_process"].includes(spec))return whole;try{return head+quote+Bun.resolveSync(spec,base)+quote;}catch{return whole;}});
  return{contents:text,loader:"ts"};
 });
}});
