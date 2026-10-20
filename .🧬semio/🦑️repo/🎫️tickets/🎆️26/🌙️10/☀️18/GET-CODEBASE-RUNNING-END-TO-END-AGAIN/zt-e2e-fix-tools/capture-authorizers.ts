/** 📥️ Runs the tools-setup source captures (nx authorizer runtime, nx authorizer compiler, cargo compiler) read-only against a root; succeeds without node_modules only when no bare specifier remains. */
import {join,resolve} from "node:path";
const root=resolve(process.argv[2]!),L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",T0=`${L}/⚡️caching/🚀️bootstrap/🛠️tools`;
const {captureNxAuthorizerSourcesV1}=await import(join(root,T0,"🔏️authorizers/🟦️.ts"));
const entries={runtime:`${T0}/🔏️authorizers/📦️runtime/🟦️.ts`,compiler:`${T0}/🔏️authorizers/📜️script.ts`,cargo:`${L}/⚡️caching/🦀️cargo/📦️units/🏗️compiler/📥️entry/📜️script.ts`};
for(const [name,entry] of Object.entries(entries)){
  try{const source=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal);console.log(JSON.stringify({name,ok:true,files:source.manifest.sources.length,resolutions:source.manifest.resolutions.length,digest:source.digest.slice(0,16)}));}
  catch(error){console.log(JSON.stringify({name,ok:false,error:String((error as Error).message).slice(0,200)}));}
}
