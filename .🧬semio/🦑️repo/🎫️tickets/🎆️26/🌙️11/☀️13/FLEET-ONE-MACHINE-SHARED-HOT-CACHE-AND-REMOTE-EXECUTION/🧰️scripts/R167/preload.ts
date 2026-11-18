import{plugin}from"bun";
import{readFileSync}from"node:fs";
const map:Record<string,string>=JSON.parse(process.env.R167_MAP??"{}"),escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\\/]/g,"\\$&");
plugin({name:"r167-stage",setup(build){
 build.onLoad({filter:/📖️source\/🟦️\.ts$/},args=>({contents:readFileSync(args.path,"utf8").replace('.map(path=>readFileSync(path,"utf8"))','.map(path=>readFileSync(process.env.R167_LOAD&&path.endsWith("📥️load/🟦️.ts")?process.env.R167_LOAD:path,"utf8"))'),loader:"ts"}));
 for(const[live,staged]of Object.entries(map))build.onLoad({filter:new RegExp(escape(live)+"$")},()=>(live.endsWith(".json")?{exports:{default:JSON.parse(readFileSync(staged,"utf8"))},loader:"object"}:{contents:readFileSync(staged,"utf8"),loader:"ts"}));
}});
