import{plugin}from"bun";
import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/",DP=L+"🚀️bootstrap/📦️dependencies/",PUBD=L+"💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/";
const ts:Record<string,string>={[DP+"💾️storage/🟦️.ts"]:"s1",[DP+"💾️storage/🧹️retirement/🏷️roles/🟦️.ts"]:"s2",[PUBD+"🟦️.ts"]:"s3"},json:Record<string,string>={[PUBD+"⚙️setup/♻️retirement/🧬️schema/🔣️.json"]:"s4",[L+"📖️texts/🔣️.json"]:"s5"};
const kind=process.env.R69_KIND??"new",dir=import.meta.dir+"/stage/";
plugin({name:"r69",setup(build){
 build.onLoad({filter:/🟦️\.ts$/u},args=>{const id=ts[args.path];return{contents:readFileSync(id?dir+id+"."+kind:args.path,"utf8"),loader:"ts"};});
 const exact=Object.keys(json).map(p=>p.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|");build.onLoad({filter:new RegExp("^(?:"+exact+")$","u")},args=>{const value=JSON.parse(readFileSync(dir+json[args.path]+"."+kind,"utf8"));return{exports:{default:value,...value},loader:"object"};});
}});
