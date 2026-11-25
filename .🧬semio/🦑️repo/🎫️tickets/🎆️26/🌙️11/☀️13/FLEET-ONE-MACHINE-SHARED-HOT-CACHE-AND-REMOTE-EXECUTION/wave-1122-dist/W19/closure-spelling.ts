import {readdirSync} from "node:fs";
import {basename,dirname,relative} from "node:path";
const sidecar=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts");
const watch=sidecar.remoteExecutionSidecarWatchV1("agent"),closure=watch.closure();
const listings=new Map<string,Set<string>>(),exact=(path:string):boolean=>{let dir=dirname(path),name=basename(path);let l=listings.get(dir);if(l===undefined){l=new Set(readdirSync(dir));listings.set(dir,l);}return l.has(name)&&(dir==="/"||exact(dir)||dir.length<=1);};
let bad=0;
for(const path of closure.files.keys()){try{if(!exact(path)){bad++;console.log("[DEBUG] not exact on disk:",relative("/Users/ueli/Documents/semio",path));}}catch(e){console.log("[DEBUG] err",path,String(e));}}
console.log("[DEBUG] files",closure.files.size,"inexact",bad,"digest",closure.digest.slice(0,16));
