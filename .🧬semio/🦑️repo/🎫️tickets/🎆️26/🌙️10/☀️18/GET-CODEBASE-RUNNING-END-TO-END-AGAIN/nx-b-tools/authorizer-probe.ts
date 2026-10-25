import {createRequire} from "node:module";
const fsp=createRequire(import.meta.url)("node:fs/promises"),original=fsp.lstat,log:string[]=[];
fsp.lstat=async function(path:string,...rest:unknown[]){log.push(String(path));return original.call(this,path,...rest);};
const root=process.argv[2]!,entry=process.argv[3]!;
const{captureNxAuthorizerSourcesV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts");
try{const result=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal);console.log("ok",result.manifest.sources.length);}
catch(error){console.log("FAIL",String((error as Error).message).slice(0,300));console.log(log.slice(-14).join("\n"));}
