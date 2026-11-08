import {mkdirSync,readdirSync,statSync,rmSync,writeFileSync} from "node:fs";
import {basename,join} from "node:path";
import {transpilePluginComponentModulesAsync} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import {optimizePluginCoreModuleAsync,pluginCoreModuleFilesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts";
const repoRoot="/Users/ueli/Documents/semio",wasm=process.argv[2]!,out=process.argv[3]!,base=(process.argv[4]??basename(wasm,".wasm")).replace(/\.wasm$/,""),ctx={repoRoot,preview2VendorDir:join(out,"vendor"),environment:{...process.env,SEMIO_SCRIPT_PROVIDERS:JSON.stringify({version:1,providers:{"repository-tooling":"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts"}})}};
const mb=(path:string)=>(statSync(path).size/1048576).toFixed(1)+" MiB";
rmSync(out,{recursive:true,force:true});mkdirSync(out,{recursive:true});writeFileSync(join(out,"base.txt"),base);
let started=performance.now();const transpiled=join(out,"transpiled");mkdirSync(transpiled);
await transpilePluginComponentModulesAsync(wasm,transpiled,base,ctx);
console.log("STAGE transpile OK",((performance.now()-started)/1000).toFixed(1)+"s",readdirSync(transpiled).filter(f=>f.endsWith(".wasm")).map(f=>f+" "+mb(join(transpiled,f))).join(", "));
started=performance.now();const optimized=join(out,"optimized");mkdirSync(optimized);
for(const file of pluginCoreModuleFilesV1(transpiled,base)){const target=join(optimized,file);await optimizePluginCoreModuleAsync(join(transpiled,file),target,ctx);console.log("  optimized",file,mb(join(transpiled,file)),"->",mb(target));}
console.log("STAGE optimize OK",((performance.now()-started)/1000).toFixed(1)+"s");
