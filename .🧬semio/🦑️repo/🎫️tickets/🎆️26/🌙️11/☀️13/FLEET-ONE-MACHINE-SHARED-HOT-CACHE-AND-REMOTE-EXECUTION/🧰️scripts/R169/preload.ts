import{plugin}from"bun";
import{readFileSync,readdirSync,statSync}from"node:fs";
import{resolve,dirname,join}from"node:path";
const M=process.env.R169_MIRROR!,LIVE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const files=new Set<string>();
const walk=(dir:string)=>{for(const name of readdirSync(dir)){const p=join(dir,name);if(statSync(p).isDirectory())walk(p);else if(!p.includes("🧪️tests"))files.add(LIVE+p.slice(M.length));}};
walk(M);
const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/g,"\\$&");
plugin({name:"r169",setup(b){
 b.onResolve({filter:/^[./]/},a=>{
  const importer=(a.importer||((a as any).resolveDir?(a as any).resolveDir+"/x":"")).replace(/^file:\/\//,"");if(!importer)return;
  void 0;const unused="",abs=a.path.startsWith("/")?a.path:resolve(dirname(importer),a.path);
  if(files.has(abs))return{path:abs};
 });
 b.onLoad({filter:new RegExp("^("+[...files].map(escape).join("|")+")$")},a=>{const text=readFileSync(M+a.path.slice(LIVE.length),"utf8");return a.path.endsWith(".json")?{exports:{default:JSON.parse(text),...JSON.parse(text)},loader:"object"}:{contents:text,loader:"ts"};});
}});
