import{plugin}from"bun";
import{existsSync}from"node:fs";
import{resolve,dirname}from"node:path";
const MIR=process.env.R229_MIRROR!,LIVE=process.env.R229_LIVE!;
plugin({name:"r229",setup(b){b.onResolve({filter:/.*/},a=>{
 if(process.env.R229_DEBUG)console.error("[DEBUG] resolve",a.path,a.importer);const importer=(a.importer??"").replace(/^file:\/\//,"");
 if(!importer.startsWith(MIR))return;
 const live=LIVE+importer.slice(MIR.length);
 if(a.path.startsWith(".")){const target=resolve(dirname(live),a.path),staged=MIR+target.slice(LIVE.length);return{path:existsSync(staged)?staged:target};}
 if(a.path.startsWith("/")){const staged=a.path.startsWith(LIVE)?MIR+a.path.slice(LIVE.length):"";return{path:staged&&existsSync(staged)?staged:a.path};}
 let from=dirname(live);while(!existsSync(from)&&from!==dirname(from))from=dirname(from);try{return{path:Bun.resolveSync(a.path,from)};}catch{return;}
});}});
