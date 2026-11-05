import fs from "node:fs/promises";
import {syncBuiltinESMExports} from "node:module";
import {relative,resolve,dirname,join} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.cwd(),library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",uri=p=>pathToFileURL(resolve(root,p)).href;
const {repositoryCommandKernelV1}=await import(uri(library+"🏗️build/⌨️command/🧭️selection/🟦️.ts"));
const {captureNxAuthorizerSourcesV1,readNxAuthorizerSourceV1}=await import(uri(library+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"));
const {closureLiteralSitesV1}=await import(uri(library+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts"));
const {runtimeResolveV1}=await import(uri("🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts"));
const entry=relative(root,repositoryCommandKernelV1(["run","@semio-tech/mit-bestand-demonstrator:canonical-architecture"]).entry).replaceAll("\\","/"),rows=[],original=fs.open,signal=AbortSignal.timeout(90000);
fs.open=async function(...args){const handle=await original.apply(this,args);try{const stat=await handle.stat();if(stat.isFile())rows.push({path:relative(root,String(args[0])).replaceAll("\\","/"),size:stat.size});return handle;}catch(error){await handle.close();throw error;}};
syncBuiltinESMExports();
let failure=null,result=null;
try{await readNxAuthorizerSourceV1(root,entry,signal);if(rows.length!==1||rows[0].path!==entry)throw Error("Original fs open observer did not observe the actual Source read");rows.length=0;result=await captureNxAuthorizerSourcesV1(root,entry,signal,{installed:true});}
catch(error){failure={message:error.message,lineage:error.lineage??null};}
finally{fs.open=original;syncBuiltinESMExports();}
const unique=[...new Map(rows.map(row=>[row.path,row])).values()],names=new Set(unique.map(row=>row.path)),edges=[];
for(const row of unique){if(!/\.[cm]?[jt]sx?$/.test(row.path))continue;const source=await fs.readFile(join(root,row.path),"utf8");for(const site of closureLiteralSitesV1(row.path,source)){let target;if(site.specifier.startsWith("."))target=relative(root,resolve(dirname(join(root,row.path)),site.specifier)).replaceAll("\\","/");else if(!site.specifier.startsWith("node:")){try{target=relative(root,await runtimeResolveV1(site.specifier,dirname(join(root,row.path)))).replaceAll("\\","/");}catch{}}if(target&&names.has(target))edges.push({from:row.path,to:target,specifier:site.specifier});}}
const output={version:1,engine:process.versions.bun?"bun":"node",observedAt:new Date().toISOString(),entry,failure,successful:result!==null,totalBytes:unique.reduce((n,row)=>n+row.size,0),rows:unique,edges};
await fs.writeFile(join(import.meta.dirname,"🗑️generated/1104-source-census-"+(process.versions.bun?"bun":"node")+".json"),JSON.stringify(output,null,2)+"\n");
console.log(JSON.stringify({entry,failure,successful:result!==null,count:unique.length,totalBytes:output.totalBytes,last:unique.slice(-8),largest:unique.slice().sort((a,b)=>b.size-a.size).slice(0,15)},null,2));
